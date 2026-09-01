import re, os, subprocess, sys, json

ROOT = "/Users/ueli/Documents/semio"
SHA = "bb06c41f73f0122fbed315b7487428b976f99921"

def git_show(path):
    r = subprocess.run(["git", "show", f"{SHA}:{path}"], cwd=ROOT, capture_output=True)
    if r.returncode != 0:
        return None
    return r.stdout.decode("utf-8")

def find_matching_brace(text, open_idx):
    assert text[open_idx] == '{'
    depth = 0
    i = open_idx
    n = len(text)
    in_line_comment = in_block_comment = in_string = in_char = False
    while i < n:
        c = text[i]
        if in_line_comment:
            if c == '\n':
                in_line_comment = False
            i += 1; continue
        if in_block_comment:
            if c == '*' and i+1 < n and text[i+1] == '/':
                in_block_comment = False; i += 2; continue
            i += 1; continue
        if in_string:
            if c == '\\':
                i += 2; continue
            if c == '"':
                in_string = False
            i += 1; continue
        if in_char:
            if c == '\\':
                i += 2; continue
            if c == "'":
                in_char = False
            i += 1; continue
        if c == '/' and i+1 < n and text[i+1] == '/':
            in_line_comment = True; i += 2; continue
        if c == '/' and i+1 < n and text[i+1] == '*':
            in_block_comment = True; i += 2; continue
        if c == '"':
            in_string = True; i += 1; continue
        if c == "'":
            j = i+1
            is_char_lit = False
            if j < n:
                if text[j] == '\\':
                    k = j
                    while k < n and text[k] != "'" and k < j+10:
                        k += 1
                    if k < n and text[k] == "'":
                        is_char_lit = True; i = k+1; continue
                else:
                    if j+1 < n and text[j+1] == "'":
                        is_char_lit = True; i = j+2; continue
            if not is_char_lit:
                i += 1; continue
        if c == '{':
            depth += 1; i += 1; continue
        if c == '}':
            depth -= 1; i += 1
            if depth == 0:
                return i - 1
            continue
        i += 1
    return -1

def find_matching_paren(text, open_idx):
    depth = 0; i = open_idx; n = len(text)
    while i < n:
        c = text[i]
        if c == '(':
            depth += 1
        elif c == ')':
            depth -= 1
            if depth == 0:
                return i
        i += 1
    return -1

def find_top_level_fn(text, name):
    pat = re.compile(r'^pub (?:async )?fn ' + re.escape(name) + r'\(', re.M)
    m = pat.search(text)
    if not m:
        return None
    sig_start = m.start()
    paren_idx = text.index('(', m.end()-1)
    close_paren = find_matching_paren(text, paren_idx)
    brace_idx = text.index('{', close_paren)
    close_brace = find_matching_brace(text, brace_idx)
    return sig_start, brace_idx, close_brace

def widen_with_leading_comment_and_region(text, sig_start):
    lines = text[:sig_start].split('\n')
    idx = len(lines) - 1
    consumed = []
    i = idx - 1
    region_line = None
    while i >= 0:
        line = lines[i]
        stripped = line.strip()
        if stripped.startswith('///') or stripped.startswith('//!') or (stripped.startswith('//') and not stripped.startswith('//#')):
            consumed.append(i); i -= 1; continue
        elif stripped.startswith('//#region'):
            region_line = i; i -= 1; break
        elif stripped == '':
            break
        else:
            break
    new_start_line = min(consumed) if consumed else idx
    def line_offset(line_no):
        return len('\n'.join(lines[:line_no]))
    widened_start = line_offset(new_start_line)
    region_start_offset = line_offset(region_line) if region_line is not None else None
    return widened_start, region_start_offset

def widen_with_trailing_region(text, close_brace_idx):
    rest = text[close_brace_idx+1:]
    m = re.match(r'\n(?:[ \t]*\n)*[ \t]*(//#endregion[^\n]*)\n', rest)
    if m:
        return close_brace_idx + 1 + m.end(), m.group(1)
    return close_brace_idx + 1, None

def extract_and_strip_fn(text, fn_name):
    found = find_top_level_fn(text, fn_name)
    if not found:
        return None, text
    sig_start, brace_idx, close_brace = found
    widened_start, region_start_offset = widen_with_leading_comment_and_region(text, sig_start)
    real_start = region_start_offset if region_start_offset is not None else widened_start
    end_after_brace, endregion_name = widen_with_trailing_region(text, close_brace)
    real_end = end_after_brace if endregion_name is not None else close_brace + 1
    extracted = text[widened_start:close_brace+1]
    new_text = text[:real_start] + text[real_end:]
    return extracted.strip('\n'), new_text

def collapse_blank_lines(text):
    text = re.sub(r'\n{3,}', '\n\n', text)
    return text.strip('\n') + '\n'

def get_struct_name(text):
    m = re.search(r'pub struct (\w+)', text)
    return m.group(1) if m else None

def replace_method_body(text, method_name, delegate_target):
    pat = re.compile(r' {4}(?:async )?fn ' + re.escape(method_name) + r'\(&self,')
    m = pat.search(text)
    if not m:
        return text, False
    paren_idx = text.index('(', m.start())
    close_paren = find_matching_paren(text, paren_idx)
    brace_idx = text.index('{', close_paren)
    sig = text[m.start():brace_idx].rstrip()
    close_idx = find_matching_brace(text, brace_idx)
    new_body = "{\n        " + delegate_target + "\n    }"
    new_text = text[:m.start()] + sig + " " + new_body + text[close_idx+1:]
    return new_text, True

def rewrite_impl_delegate(text, struct_name):
    m = re.search(r'impl \w*MutationKind<[^\n]*?for ' + re.escape(struct_name) + r' \{', text)
    if not m:
        return text, False
    brace_idx = text.index('{', m.start())
    close_idx = find_matching_brace(text, brace_idx)
    block = text[brace_idx:close_idx+1]
    new_block, c1 = replace_method_body(block, "diff", "super::diff::diff(self, base)")
    new_block, c2 = replace_method_body(new_block, "inverse", "super::inverse::inverse(self, base)")
    if not (c1 or c2):
        return text, False
    new_text = text[:brace_idx] + new_block + text[close_idx+1:]
    return new_text, True

def prune_unused_uses(text):
    lines = text.split('\n')
    out = []
    for i, line in enumerate(lines):
        stripped = line.strip()
        if stripped.startswith('use ') and stripped.endswith(';'):
            rest_text = '\n'.join(lines[:i] + lines[i+1:])
            m = re.match(r'use\s+([\w:]+)::\{([^}]+)\};', stripped)
            if m:
                prefix, items = m.groups()
                names = [it.strip() for it in items.split(',')]
                kept = []
                for nm in names:
                    bare = nm.split(' as ')[-1].strip()
                    if re.search(r'\b' + re.escape(bare) + r'\b', rest_text):
                        kept.append(nm)
                if not kept:
                    continue
                indent = line[:len(line)-len(line.lstrip())]
                if len(kept) == 1:
                    out.append(f'{indent}use {prefix}::{kept[0]};')
                elif len(kept) == len(names):
                    out.append(line)
                else:
                    out.append(f'{indent}use {prefix}::{{{", ".join(kept)}}};')
                continue
            m2 = re.match(r'use\s+([\w:]+)::(\w+);', stripped)
            if m2:
                prefix, name = m2.groups()
                if not re.search(r'\b' + re.escape(name) + r'\b', rest_text):
                    continue
                out.append(line); continue
            out.append(line)
        else:
            out.append(line)
    return '\n'.join(out)

def build_facet_content(pinned_text, current_fn_extract, struct_name):
    if pinned_text is not None:
        # The pinned commit predates flattening: every mutation (this one AND any sibling mutation
        # referenced cross-mutation) had its struct+impl in a dedicated `🦠️mutation` facet/module.
        # That module no longer exists anywhere (struct+impl now live directly in the mutation's
        # kind-only leaf), so every `::mutation::Type` path — this mutation's own via `super::`, or
        # another mutation's via its full `mutations::<slug>::mutation::Type` path — collapses by
        # dropping the now-nonexistent `mutation::` segment.
        content = re.sub(r'::mutation::', '::', pinned_text)
        return content.rstrip('\n') + '\n', 'pinned'
    assert current_fn_extract is not None
    needs_use = False
    for mo in re.finditer(r'\b' + re.escape(struct_name) + r'\b', current_fn_extract):
        start = mo.start()
        preceding = current_fn_extract[max(0,start-8):start]
        if not preceding.endswith('super::'):
            needs_use = True; break
    header = f"use super::{struct_name};\n\n" if needs_use else ""
    content = header + current_fn_extract.rstrip('\n') + '\n'
    return content, 'derived'

def rel_glue_path(mutation_dir_rel, plugin):
    prefix = f"✏️s/🔌️plugins/{plugin}/"
    assert mutation_dir_rel.startswith(prefix), mutation_dir_rel
    tail = mutation_dir_rel[len(prefix):]
    return f"../../{tail}"

def process_one(leaf_rel, dry_run=True):
    """Returns a dict report for this mutation dir."""
    abs_leaf = os.path.join(ROOT, leaf_rel)
    d_rel = os.path.dirname(leaf_rel)
    d_abs = os.path.join(ROOT, d_rel)
    plugin = leaf_rel.split('/')[2]

    with open(abs_leaf, encoding='utf-8') as f:
        orig_text = f.read()

    struct_name = get_struct_name(orig_text)
    if not struct_name:
        return {"leaf": leaf_rel, "status": "ERROR", "msg": "no struct found"}

    text = orig_text
    diff_extract, text = extract_and_strip_fn(text, "diff")
    inv_extract, text = extract_and_strip_fn(text, "inverse")

    if diff_extract is None and inv_extract is None:
        return {"leaf": leaf_rel, "status": "SKIP", "msg": "no top-level diff/inverse fn found (unexpected)"}

    text, changed_impl = rewrite_impl_delegate(text, struct_name)

    text = collapse_blank_lines(text)
    text = prune_unused_uses(text)
    text = collapse_blank_lines(text)

    pinned_diff = git_show(f"{d_rel}/🔺️diff/🦀️component.rs") if diff_extract is not None else None
    pinned_inv = git_show(f"{d_rel}/↩️inverse/🦀️component.rs") if inv_extract is not None else None

    diff_content = diff_source = None
    inv_content = inv_source = None
    if diff_extract is not None:
        diff_content, diff_source = build_facet_content(pinned_diff, diff_extract, struct_name)
    if inv_extract is not None:
        inv_content, inv_source = build_facet_content(pinned_inv, inv_extract, struct_name)

    report = {
        "leaf": leaf_rel, "status": "OK", "struct": struct_name,
        "changed_impl": changed_impl,
        "diff_source": diff_source, "inv_source": inv_source,
        "diff_dir_exists": os.path.isdir(os.path.join(d_abs, "🔺️diff")),
        "inv_dir_exists": os.path.isdir(os.path.join(d_abs, "↩️inverse")),
    }

    if not dry_run:
        if diff_content is not None:
            diff_dir = os.path.join(d_abs, "🔺️diff")
            os.makedirs(diff_dir, exist_ok=True)
            with open(os.path.join(diff_dir, "🦀️.rs"), 'w', encoding='utf-8') as f:
                f.write(diff_content)
        if inv_content is not None:
            inv_dir = os.path.join(d_abs, "↩️inverse")
            os.makedirs(inv_dir, exist_ok=True)
            with open(os.path.join(inv_dir, "🦀️.rs"), 'w', encoding='utf-8') as f:
                f.write(inv_content)
        with open(abs_leaf, 'w', encoding='utf-8') as f:
            f.write(text)

    report["new_leaf_preview"] = text
    report["diff_content"] = diff_content
    report["inv_content"] = inv_content
    return report

if __name__ == "__main__":
    listfile = sys.argv[1]
    mode = sys.argv[2] if len(sys.argv) > 2 else "dry"
    with open(listfile, encoding='utf-8') as f:
        leaves = [l.strip() for l in f if l.strip()]
    results = []
    for leaf in leaves:
        try:
            r = process_one(leaf, dry_run=(mode == "dry"))
        except Exception as e:
            r = {"leaf": leaf, "status": "EXC", "msg": str(e)}
        results.append(r)
    n_ok = sum(1 for r in results if r["status"] == "OK")
    n_skip = sum(1 for r in results if r["status"] == "SKIP")
    n_err = sum(1 for r in results if r["status"] in ("ERROR", "EXC"))
    print(f"OK={n_ok} SKIP={n_skip} ERR={n_err}")
    for r in results:
        if r["status"] != "OK":
            print(r)
    # dump light report (no big content) to json
    light = [{k: v for k, v in r.items() if k not in ("new_leaf_preview","diff_content","inv_content")} for r in results]
    with open(os.path.join(os.path.dirname(listfile), f"report_{mode}.json"), 'w', encoding='utf-8') as f:
        json.dump(light, f, indent=2, ensure_ascii=False)
