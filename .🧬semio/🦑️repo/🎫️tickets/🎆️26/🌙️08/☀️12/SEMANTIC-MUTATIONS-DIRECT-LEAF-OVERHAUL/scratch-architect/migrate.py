#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Wave C architect facet migration: split 72 noun-keyed triad dirs into 266 one-per-variant
triad dirs, rewrite glue.rs mounts, rewrite the dispatch enum's use-paths. Idempotent-ish: safe
to re-run (regenerates new dirs, then deletes old ones at the end)."""
import os, re, sys, json

ROOT = "/Users/ueli/Documents/semio"
MUT_DIR = f"{ROOT}/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
GLUE = f"{ROOT}/✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/📦️glue.rs"

VERB_EMOJI = {
    "create": "🌱", "delete": "🗑️", "rename": "✏️", "replace": "🔁",
    "connect": "🔗", "disconnect": "✂️",
}
VERB_PREFIX = {
    "Create": "create", "Delete": "delete", "Rename": "rename", "Replace": "replace",
    "Connect": "connect", "Disconnect": "disconnect",
}

# old triad dirs to migrate (dir name incl. emoji) -> entity emoji to reuse for all its verb-splits
ENTITY_EMOJI_OVERRIDE = {
    "🗺️set-adjacency": "🧲",
    "🧹clear-adjacency": "🧲",
    "🧵traces": "🧵",
    "🏷️update-meta": "🏷️",
    "📁update-project": "📁",
    "🏛️update-governance": "🏛️",
}

SKIP_DIRS = {"🔀adjacencies", "🖼️set-snapshot", "📝️text", "💾️binary"}

def strip_emoji_prefix(dirname):
    """Return (emoji_prefix, rest) splitting leading non-ascii-letter run from the kebab slug."""
    i = 0
    while i < len(dirname) and not (dirname[i].isascii() and dirname[i].isalpha()):
        i += 1
    return dirname[:i], dirname[i:]

def find_region_blocks(text):
    """Return list of (name, inner_text_without_markers) in source order."""
    out = []
    for m in re.finditer(r"//#region 🔖️(\w+)\n", text):
        name = m.group(1)
        end_marker = f"//#endregion 🔖️{name}"
        end_idx = text.index(end_marker, m.end())
        inner = text[m.end():end_idx]
        out.append((name, inner.rstrip("\n")))
    return out

def find_doc_and_fn(text, fn_name):
    """Brace-match extraction of `pub fn {fn_name}(...) -> ... { ... }` plus its immediately
    preceding contiguous /// doc comment lines."""
    pat = re.compile(r"pub fn " + re.escape(fn_name) + r"\(")
    m = pat.search(text)
    if not m:
        return None
    sig_start = m.start()
    # doc comment: walk backwards over contiguous /// lines
    lines_before = text[:sig_start].split("\n")
    doc_lines = []
    j = len(lines_before) - 1
    # last element is '' (partial line before sig_start) typically; walk from end
    # find index of line start
    idx = sig_start
    # collect contiguous /// lines directly above
    # locate start-of-line for sig
    line_start = text.rfind("\n", 0, sig_start) + 1
    cursor = line_start
    doc_block = []
    while True:
        prev_nl = text.rfind("\n", 0, cursor - 1)
        line = text[prev_nl + 1:cursor - 1] if cursor > 0 else ""
        if line.strip().startswith("///"):
            doc_block.insert(0, line)
            cursor = prev_nl + 1
        else:
            break
    doc_comment = "\n".join(doc_block)
    # find first '{' after signature (skip past return type)
    brace_start = text.index("{", m.end())
    depth = 1
    k = brace_start + 1
    while depth > 0:
        if text[k] == "{":
            depth += 1
        elif text[k] == "}":
            depth -= 1
        k += 1
    fn_full = text[line_start:k]
    return doc_comment, fn_full

USE_LINE_RE = re.compile(r"^use\s+([^;]+);[ \t]*$", re.MULTILINE)

def parse_use_lines(text):
    """Return list of (full_line_text, [symbol_names], base_path_or_None, is_brace)."""
    out = []
    for m in USE_LINE_RE.finditer(text):
        body = m.group(1).strip()
        full = m.group(0)
        syms = []
        base = None
        is_brace = False
        if body.endswith("}") and "{" in body:
            is_brace = True
            base, rest = body.split("{", 1)
            inner = rest.rstrip("}")
            for part in inner.split(","):
                part = part.strip()
                if not part:
                    continue
                syms.append(part.split(" as ")[-1].strip())
        else:
            syms.append(body.split("::")[-1].split(" as ")[-1].strip())
        out.append((full, syms, base, is_brace))
    return out

def filter_uses(use_lines, body_text, always_keep_prefixes=()):
    """Per-symbol filtering: brace imports are rebuilt with only the referenced symbols kept, so
    a multi-symbol `use` line doesn't drag in an unused sibling (avoids `unused import` warnings
    when only one struct out of a formerly-shared header import survives the triad split)."""
    kept = []
    for full, syms, base, is_brace in use_lines:
        if any(full.startswith(p) for p in always_keep_prefixes):
            kept.append(full)
            continue
        used = [s for s in syms if re.search(r"\b" + re.escape(s) + r"\b", body_text)]
        if not used:
            continue
        if is_brace and len(used) < len(syms):
            kept.append(f"use {base}{{{', '.join(used)}}};")
        else:
            kept.append(full)
    return kept

def kebab_to_words(kebab):
    return kebab.split("-")

def main():
    entries = sorted(os.listdir(MUT_DIR))
    plan = []  # list of dicts describing each new triad dir to create
    old_dirs_to_delete = []
    for old_dir in entries:
        if old_dir in SKIP_DIRS:
            continue
        full_old = os.path.join(MUT_DIR, old_dir)
        if not os.path.isdir(full_old):
            continue
        mut_path = os.path.join(full_old, "🦠️mutation", "🦀️component.rs")
        if not os.path.exists(mut_path):
            continue
        old_dirs_to_delete.append(old_dir)
        mut_text = open(mut_path, encoding="utf-8").read()
        diff_text = open(os.path.join(full_old, "🔺️diff", "🦀️component.rs"), encoding="utf-8").read()
        inv_text = open(os.path.join(full_old, "↩️inverse", "🦀️component.rs"), encoding="utf-8").read()

        emoji_prefix, old_slug = strip_emoji_prefix(old_dir)
        entity_emoji = ENTITY_EMOJI_OVERRIDE.get(old_dir, emoji_prefix)

        blocks = find_region_blocks(mut_text)
        for struct_name, struct_body in blocks:
            verb = None
            for pfx, v in VERB_PREFIX.items():
                if struct_name.startswith(pfx):
                    verb = v
                    break
            assert verb, f"cannot infer verb for {struct_name} in {old_dir}"
            kind_m = re.search(r'kind:\s*"([a-z0-9-]+)"', struct_body)
            assert kind_m, f"no kind found for {struct_name}"
            kind_slug = kind_m.group(1)
            module_ident = kind_slug.replace("-", "_")
            new_dir_emoji = VERB_EMOJI[verb] + entity_emoji
            new_dir = f"{new_dir_emoji}{kind_slug}"

            diff_fn_old = f"diff_{verb}"
            inv_fn_old = f"inverse_{verb}"
            diff_doc, diff_fn_full = find_doc_and_fn(diff_text, diff_fn_old)
            inv_doc, inv_fn_full = find_doc_and_fn(inv_text, inv_fn_old)

            # rename fn name in extracted bodies: diff_X( -> diff(   /  inverse_X( -> inverse(
            diff_fn_new = re.sub(r"pub fn " + re.escape(diff_fn_old) + r"\(", "pub fn diff(", diff_fn_full, count=1)
            inv_fn_new = re.sub(r"pub fn " + re.escape(inv_fn_old) + r"\(", "pub fn inverse(", inv_fn_full, count=1)

            # in mutation struct's impl block, delegate calls reference super::diff::diff_X / super::inverse::inverse_X
            struct_body_new = struct_body.replace(f"super::diff::{diff_fn_old}(", "super::diff::diff(")
            struct_body_new = struct_body_new.replace(f"super::inverse::{inv_fn_old}(", "super::inverse::inverse(")

            plan.append({
                "old_dir": old_dir,
                "new_dir": new_dir,
                "module_ident": module_ident,
                "kind_slug": kind_slug,
                "struct_name": struct_name,
                "struct_body": struct_body_new,
                "diff_doc": diff_doc, "diff_fn": diff_fn_new,
                "inv_doc": inv_doc, "inv_fn": inv_fn_new,
                "verb": verb,
            })

    # Build global name -> module_ident map
    name_to_module = {p["struct_name"]: p["module_ident"] for p in plan}
    assert len(name_to_module) == len(plan), "duplicate struct names!"
    print(f"Total new triad dirs planned: {len(plan)}", file=sys.stderr)

    # Now, per old dir, gather the shared header `use` lines (everything minus the per-fn ones)
    # We already read mut_text/diff_text/inv_text per old dir above but didn't keep references;
    # re-derive per old_dir grouping to fetch the use-line pools.
    old_dir_sources = {}
    for old_dir in old_dirs_to_delete:
        full_old = os.path.join(MUT_DIR, old_dir)
        mut_text = open(os.path.join(full_old, "🦠️mutation", "🦀️component.rs"), encoding="utf-8").read()
        diff_text = open(os.path.join(full_old, "🔺️diff", "🦀️component.rs"), encoding="utf-8").read()
        inv_text = open(os.path.join(full_old, "↩️inverse", "🦀️component.rs"), encoding="utf-8").read()
        old_dir_sources[old_dir] = (mut_text, diff_text, inv_text)

    for p in plan:
        mut_text, diff_text, inv_text = old_dir_sources[p["old_dir"]]
        p["_mut_uses"] = parse_use_lines(mut_text)
        p["_diff_uses"] = parse_use_lines(diff_text)
        p["_inv_uses"] = parse_use_lines(inv_text)

    with open("/tmp/architect_migration_plan.json", "w", encoding="utf-8") as f:
        json.dump([{k: v for k, v in p.items() if not k.startswith("_")} for p in plan], f, ensure_ascii=False, indent=1)

    return plan, name_to_module

if __name__ == "__main__":
    plan, name_to_module = main()
    print("OK", len(plan))
