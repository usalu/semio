#!/usr/bin/env python3
"""Split grouped 🎮️commands noun folders into one semantically named folder per command."""
from __future__ import annotations

import json
import os
import re
import shutil
import sys
from collections import defaultdict
from pathlib import Path

REPO = Path(__file__).resolve()
while REPO.name != "semio" and REPO != REPO.parent:
    # walk up until workspace; ticket lives at .🦑️repo/🎫️tickets/...
    if (REPO / "✏️s").is_dir() and (REPO / "📜️script.ts").is_file():
        break
    REPO = REPO.parent
if not (REPO / "✏️s").is_dir():
    REPO = Path("/Users/ueli/Documents/semio")

FILTER = os.environ.get("FILTER", "").strip()
DRY = os.environ.get("DRY", "").strip() in {"1", "true", "yes"}
SKIP_POLICY = os.environ.get("SKIP_POLICY", "").strip() in {"1", "true", "yes"}
MAPPING_OUT = os.environ.get("MAPPING_OUT", "").strip()

COMMANDS_MARK = "🎮️commands"
COMPONENT = "🦀️component.rs"
GLUE = "📦️glue.rs"

MOD_RE = re.compile(r"^(pub(?:\s*\(\s*crate\s*\))?\s+mod\s+(\w+)\s*\{)", re.M)
FN_RE = re.compile(
    r"^((?:pub(?:\s*\(\s*crate\s*\))?\s+)?(?:async\s+)?fn\s+(\w+)\s*[\(<])",
    re.M,
)
PUB_FN_RE = re.compile(
    r"^((?:pub(?:\s*\(\s*crate\s*\))\s+)(?:async\s+)?fn\s+(\w+)\s*[\(<])",
    re.M,
)
USE_RE = re.compile(r"^use\s+.+?;\s*$", re.M)
PATH_MOD_RE = re.compile(
    r'(^[ \t]*)#\[path\s*=\s*"([^"]*' + re.escape(COMMANDS_MARK) + r'/[^"]+/' + re.escape(COMPONENT) + r')"\]\s*\n[ \t]*pub mod (\w+);',
    re.M,
)
HELPER_PREFIXES = (
    "error_",
    "fixture_",
    "preset_",
    "default_",
    "loaded_",
    "parse_",
    "apply_",
    "text_select_operations",
)


def split_emoji_slug(name: str) -> tuple[str, str]:
    m = re.match(r"^([^A-Za-z0-9]+)(.+)$", name)
    if not m:
        return "", name
    return m.group(1), m.group(2)


def snake_to_kebab(name: str) -> str:
    return name.replace("_", "-")


def kebab_to_snake(name: str) -> str:
    return name.replace("-", "_")


def find_command_files() -> list[Path]:
    out: list[Path] = []
    for root, dirs, files in os.walk(REPO / "✏️s"):
        if "target" in Path(root).parts:
            dirs.clear()
            continue
        if COMPONENT not in files:
            continue
        path = Path(root) / COMPONENT
        rel = path.relative_to(REPO).as_posix()
        if f"/{COMMANDS_MARK}/" not in f"/{rel}":
            continue
        after = rel.split(f"/{COMMANDS_MARK}/", 1)[1]
        if after != f"{path.parent.name}/{COMPONENT}":
            continue
        if FILTER and FILTER not in rel:
            continue
        out.append(path)
    return sorted(out)


def match_braces(text: str, brace_idx: int) -> int:
    """Return index of closing brace matching text[brace_idx] == '{'."""
    depth = 0
    i = brace_idx
    n = len(text)
    in_str = None
    escape = False
    in_line = False
    in_block = False
    while i < n:
        ch = text[i]
        nxt = text[i + 1] if i + 1 < n else ""
        if in_line:
            if ch == "\n":
                in_line = False
            i += 1
            continue
        if in_block:
            if ch == "*" and nxt == "/":
                in_block = False
                i += 2
                continue
            i += 1
            continue
        if in_str:
            if escape:
                escape = False
            elif ch == "\\":
                escape = True
            elif ch == in_str:
                in_str = None
            i += 1
            continue
        if ch == "/" and nxt == "/":
            in_line = True
            i += 2
            continue
        if ch == "/" and nxt == "*":
            in_block = True
            i += 2
            continue
        if ch in ("'", '"'):
            in_str = ch
            i += 1
            continue
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return i
        i += 1
    raise ValueError("unbalanced braces")


def extract_mods(text: str) -> list[dict]:
    mods = []
    for m in MOD_RE.finditer(text):
        name = m.group(2)
        if name == "tests":
            continue
        brace = text.find("{", m.start())
        end = match_braces(text, brace)
        # include trailing newline
        stop = end + 1
        if stop < len(text) and text[stop] == "\n":
            stop += 1
        mods.append(
            {
                "name": name,
                "start": m.start(),
                "end": stop,
                "header": m.group(1),
                "body": text[brace + 1 : end],
            }
        )
    return mods


def extract_tests(text: str) -> str | None:
    m = re.search(r"^//#region 🧪️Tests\n", text, re.M)
    if not m:
        cfg = re.search(r"^#\[cfg\(test\)\]\s*\nmod tests \{", text, re.M)
        if not cfg:
            return None
        brace = text.find("{", cfg.start())
        end = match_braces(text, brace)
        return text[cfg.start() : end + 1]
    endm = re.search(r"^//#endregion 🧪️Tests\s*\n?", text[m.start() :], re.M)
    if not endm:
        return text[m.start() :]
    return text[m.start() : m.start() + endm.end()]


def strip_tests_and_mods(text: str, mods: list[dict], tests: str | None) -> str:
    spans = [(m["start"], m["end"]) for m in mods]
    if tests:
        idx = text.find(tests)
        if idx >= 0:
            spans.append((idx, idx + len(tests)))
    spans.sort()
    parts = []
    cursor = 0
    for a, b in spans:
        parts.append(text[cursor:a])
        cursor = b
    parts.append(text[cursor:])
    leftover = "".join(parts)
    leftover = re.sub(r"\n{3,}", "\n\n", leftover).strip() + "\n"
    return leftover


def dedent(body: str) -> str:
    lines = body.split("\n")
    # drop leading/trailing empty
    while lines and lines[0].strip() == "":
        lines.pop(0)
    while lines and lines[-1].strip() == "":
        lines.pop()
    if not lines:
        return ""
    indents = []
    for line in lines:
        if line.strip() == "":
            continue
        indents.append(len(line) - len(line.lstrip(" ")))
    pad = min(indents) if indents else 0
    out = [line[pad:] if len(line) >= pad else line for line in lines]
    return "\n".join(out) + "\n"


def drop_super_use(body: str) -> str:
    lines = []
    for line in body.split("\n"):
        if re.match(r"^use super::", line.strip()):
            continue
        lines.append(line)
    text = "\n".join(lines)
    return re.sub(r"^\n+", "", text)


def file_level_uses(text: str, mods: list[dict], tests: str | None) -> str:
    leftover = strip_tests_and_mods(text, mods, tests)
    uses = []
    for line in leftover.split("\n"):
        if line.startswith("use "):
            uses.append(line)
    return "\n".join(uses)


def file_level_helpers(text: str, mods: list[dict], tests: str | None) -> str:
    leftover = strip_tests_and_mods(text, mods, tests)
    # drop module doc, uses, attributes that are file-level rustc
    lines = leftover.split("\n")
    kept = []
    skipping_doc = True
    for line in lines:
        if skipping_doc and (line.startswith("//!") or line.strip() == ""):
            continue
        skipping_doc = False
        if line.startswith("use "):
            continue
        kept.append(line)
    helper = "\n".join(kept).strip()
    return (helper + "\n") if helper else ""


def module_doc(emoji: str, kebab: str, original_doc: str) -> str:
    first = ""
    for line in original_doc.split("\n"):
        if line.startswith("//!"):
            first = line[3:].strip()
            break
    if first:
        return f"//! {emoji} {first.split('—', 1)[0].strip()} command — `{kebab}`.\n"
    return f"//! {emoji} `{kebab}` command.\n"


def original_doc(text: str) -> str:
    lines = []
    for line in text.split("\n"):
        if line.startswith("//!"):
            lines.append(line)
        elif lines:
            break
    return "\n".join(lines)


def rewrite_tests_for_mod(tests: str, mod_name: str) -> str:
    if not tests:
        return ""
    text = tests
    text = re.sub(rf"use super::{mod_name}\s*;\s*\n", "", text)
    text = re.sub(rf"use super::\{{{mod_name}\}}\s*;\s*\n", "", text)
    text = re.sub(rf"\b{mod_name}::", "", text)
    return text


def needed_helpers(helper_src: str, body: str, tests: str) -> str:
    if not helper_src.strip():
        return ""
    blob = body + "\n" + tests
    # keep a helper region/fn if its name appears in body/tests
    names = re.findall(r"\b(?:fn|struct|enum|type|const|static)\s+(\w+)", helper_src)
    keep_names = [n for n in names if n in blob]
    if not keep_names:
        # if body used a free fn from leftover without declaring, copy all
        leftover_fns = re.findall(r"\bfn\s+(\w+)", helper_src)
        if any(fn in blob for fn in leftover_fns):
            return helper_src if helper_src.endswith("\n") else helper_src + "\n"
        return ""
    return helper_src if helper_src.endswith("\n") else helper_src + "\n"


def flatten_command_file(
    *,
    emoji: str,
    kebab: str,
    original_text: str,
    uses: str,
    helpers: str,
    body: str,
    tests: str,
    extra_uses: list[str] | None = None,
) -> str:
    body = drop_super_use(dedent(body))
    helpers = needed_helpers(helpers, body, tests)
    doc = module_doc(emoji, kebab, original_text)
    parts = [doc, ""]
    if uses:
        parts.append(uses.rstrip() + "\n")
    if extra_uses:
        for u in extra_uses:
            if u not in (uses or ""):
                parts.append(u.rstrip() + "\n")
    parts.append("")
    if helpers.strip():
        parts.append(helpers.rstrip() + "\n\n")
    parts.append(body.rstrip() + "\n")
    if tests.strip():
        parts.append("\n" + tests.rstrip() + "\n")
    text = "".join(parts) if False else "\n".join(
        p.rstrip("\n") for p in parts if p is not None
    )
    # rebuild more cleanly
    chunks: list[str] = [doc.rstrip() + "\n"]
    if uses.strip():
        chunks.append("\n" + uses.rstrip() + "\n")
    if extra_uses:
        extra = "\n".join(u for u in extra_uses if u not in (uses or ""))
        if extra.strip():
            chunks.append(extra.rstrip() + "\n")
    chunks.append("\n")
    if helpers.strip():
        chunks.append(helpers.rstrip() + "\n\n")
    chunks.append(body.rstrip() + "\n")
    if tests.strip():
        chunks.append("\n" + tests.rstrip() + "\n")
    out = "".join(chunks)
    out = re.sub(r"\n{3,}", "\n\n", out)
    if not out.endswith("\n"):
        out += "\n"
    return out


def assign_tests(tests: str | None, mod_names: list[str]) -> dict[str, str]:
    assigned: dict[str, str] = {n: "" for n in mod_names}
    if not tests:
        return assigned
    # Prefer the `use super::foo` target; else first mentioned mod; else first mod.
    hits = re.findall(r"use super::(\w+)", tests)
    owner = None
    for h in hits:
        if h in mod_names:
            owner = h
            break
    if owner is None:
        for n in mod_names:
            if re.search(rf"\b{n}::", tests):
                owner = n
                break
    if owner is None:
        owner = mod_names[0]
    assigned[owner] = rewrite_tests_for_mod(tests, owner)
    return assigned


def extract_pub_fns(text: str) -> list[dict]:
    fns = []
    for m in PUB_FN_RE.finditer(text):
        name = m.group(2)
        # find opening brace of the function, if any (may be `;` for signatures — skip)
        rest = text[m.end() - 1 :]
        brace = rest.find("{")
        semi = rest.find(";")
        if brace < 0 or (semi >= 0 and semi < brace):
            continue
        abs_brace = m.end() - 1 + brace
        end = match_braces(text, abs_brace)
        stop = end + 1
        if stop < len(text) and text[stop] == "\n":
            stop += 1
        # include leading doc comments / attributes immediately above
        start = m.start()
        line_start = text.rfind("\n", 0, start) + 1
        prelude = text[:line_start].rstrip("\n")
        # walk back attribute/doc lines
        while True:
            prev_nl = prelude.rfind("\n")
            line = prelude[prev_nl + 1 :] if prev_nl >= 0 else prelude
            if line.startswith("//") or line.startswith("#[") or line.startswith("///"):
                start = (prev_nl + 1) if prev_nl >= 0 else 0
                prelude = text[:start].rstrip("\n")
                continue
            break
        fns.append({"name": name, "start": start, "end": stop, "block": text[start:stop]})
    return fns


class Mapping:
    def __init__(self):
        # old_rel -> list of {old_mod, new_rel, new_mod, kebab}
        self.by_old: dict[str, list[dict]] = defaultdict(list)
        self.old_mod_to_new: dict[tuple[str, str], str] = {}
        # (commands_parent_rel, old_glue_mod) -> list of new glue mods + paths
        self.glue: dict[tuple[str, str], list[dict]] = defaultdict(list)

    def add(self, *, old_file: Path, old_mod: str, new_file: Path, new_mod: str, kebab: str):
        old_rel = old_file.relative_to(REPO).as_posix()
        new_rel = new_file.relative_to(REPO).as_posix()
        rec = {"old_mod": old_mod, "new_rel": new_rel, "new_mod": new_mod, "kebab": kebab, "old_rel": old_rel}
        self.by_old[old_rel].append(rec)
        parent = str(old_file.parent.parent.relative_to(REPO).as_posix())
        self.old_mod_to_new[(parent, old_mod)] = new_mod
        self.glue[(parent, old_mod)].append({"new_mod": new_mod, "new_rel": new_rel, "old_rel": old_rel})


def plan_file(path: Path, mapping: Mapping) -> None:
    text = path.read_text(encoding="utf-8")
    folder = path.parent.name
    emoji, slug = split_emoji_slug(folder)
    mods = extract_mods(text)
    tests = extract_tests(text)

    if mods:
        uses = file_level_uses(text, mods, tests)
        helpers = file_level_helpers(text, mods, tests)
        test_map = assign_tests(tests, [m["name"] for m in mods])
        used_names = {m["name"] for m in mods}
        # collision-safe folder names within this commands dir
        existing = {p.name for p in path.parent.parent.iterdir() if p.is_dir()}
        for mod in mods:
            kebab = snake_to_kebab(mod["name"])
            new_folder = f"{emoji}{kebab}"
            if new_folder in existing and new_folder != folder:
                new_folder = f"{emoji}{slug}-{kebab}"
                kebab = f"{slug}-{kebab}"
            existing.add(new_folder)
            new_dir = path.parent.parent / new_folder
            new_file = new_dir / COMPONENT
            body = flatten_command_file(
                emoji=emoji,
                kebab=kebab,
                original_text=text,
                uses=uses,
                helpers=helpers,
                body=mod["body"],
                tests=test_map.get(mod["name"], ""),
            )
            mapping.add(old_file=path, old_mod=path_mod_name(path), new_file=new_file, new_mod=mod["name"], kebab=kebab)
            rec = mapping.by_old[path.relative_to(REPO).as_posix()][-1]
            rec["content"] = body
            rec["new_dir"] = str(new_dir)
            rec["delete_old"] = True
        # glue old mod is the current glue identifier, not the folder slug necessarily.
        return

    fns = extract_pub_fns(text)
    command_fns = [f for f in fns if not is_helper_fn(f["name"])]
    if len(command_fns) >= 2:
        uses = "\n".join(line for line in text.split("\n") if line.startswith("use "))
        tests = extract_tests(text) or ""
        existing = {p.name for p in path.parent.parent.iterdir() if p.is_dir()}
        leftover_helpers = []
        helper_fns = [f for f in fns if is_helper_fn(f["name"])]
        helper_src = "\n".join(f["block"] for f in helper_fns)
        # private fns too
        private = []
        for m in FN_RE.finditer(text):
            if m.group(1).strip().startswith("pub"):
                continue
            name = m.group(2)
            rest = text[m.end() - 1 :]
            brace = rest.find("{")
            if brace < 0:
                continue
            abs_brace = m.end() - 1 + brace
            try:
                end = match_braces(text, abs_brace)
            except ValueError:
                continue
            private.append(text[m.start() : end + 1])
        helper_src = (helper_src + "\n" + "\n".join(private)).strip()
        for fn in command_fns:
            kebab = snake_to_kebab(fn["name"])
            new_folder = f"{emoji}{kebab}"
            if new_folder in existing and new_folder != folder:
                new_folder = f"{emoji}{slug}-{kebab}"
            existing.add(new_folder)
            new_dir = path.parent.parent / new_folder
            new_file = new_dir / COMPONENT
            needed = needed_helpers(helper_src + "\n", fn["block"], tests if fn is command_fns[0] else "")
            doc = module_doc(emoji, kebab, text)
            chunks = [doc.rstrip() + "\n"]
            if uses.strip():
                chunks.append("\n" + uses.rstrip() + "\n\n")
            if needed.strip():
                chunks.append(needed.rstrip() + "\n\n")
            chunks.append(fn["block"].rstrip() + "\n")
            if fn is command_fns[0] and tests.strip():
                chunks.append("\n" + tests.rstrip() + "\n")
            body = re.sub(r"\n{3,}", "\n\n", "".join(chunks))
            if not body.endswith("\n"):
                body += "\n"
            mapping.add(old_file=path, old_mod=path_mod_name(path), new_file=new_file, new_mod=fn["name"], kebab=kebab)
            rec = mapping.by_old[path.relative_to(REPO).as_posix()][-1]
            rec["content"] = body
            rec["new_dir"] = str(new_dir)
            rec["delete_old"] = True
        return

    # already 1:1 — maybe rename noun folder
    if should_rename_noun(slug, text):
        new_kebab = rename_target(slug, text)
        new_folder = f"{emoji}{new_kebab}"
        if new_folder == folder:
            return
        new_dir = path.parent.parent / new_folder
        new_file = new_dir / COMPONENT
        mapping.add(old_file=path, old_mod=path_mod_name(path), new_file=new_file, new_mod=kebab_to_snake(new_kebab), kebab=new_kebab)
        rec = mapping.by_old[path.relative_to(REPO).as_posix()][-1]
        rec["content"] = text
        rec["new_dir"] = str(new_dir)
        rec["delete_old"] = True
        rec["rename_only"] = True


def path_mod_name(path: Path) -> str:
    _, slug = split_emoji_slug(path.parent.name)
    return kebab_to_snake(slug)


APPROVED = {
    "create", "delete", "insert", "remove", "add", "rename", "change", "update", "move",
    "drag", "resize", "rotate", "scale", "reorder", "edit", "replace", "duplicate",
    "connect", "disconnect", "bind", "unbind", "group", "ungroup", "flatten", "unflatten",
    "split", "merge", "extract", "inline", "clear", "fix", "toggle", "apply", "set",
    "format", "open", "load", "save", "import", "export", "lint", "request", "select",
    "hover", "evaluate", "run", "retry", "calibrate", "place", "commit", "input",
    "submit", "abort", "patch", "paint", "nudge", "reset", "focus", "step", "query",
    "evaluate", "fill", "snap", "nudge", "ingest", "exaggeration",
}


def is_helper_fn(name: str) -> bool:
    if name.startswith(HELPER_PREFIXES):
        return True
    if name.endswith("_json") and not name.startswith(("set_", "load_", "export_", "import_")):
        return True
    if name in {"error_result_json", "run_jack_query", "fixture_dsl_for_preset", "preset_query"}:
        return True
    return False


def should_rename_noun(slug: str, text: str) -> bool:
    first = slug.split("-")[0]
    if first in APPROVED:
        return False
    if "-" in slug:
        # already compound; if first isn't a verb, still rename when a single struct/fn exists
        pass
    structs = re.findall(r"^pub struct (\w+)", text, re.M)
    mods = extract_mods(text)
    fns = [f for f in extract_pub_fns(text) if not is_helper_fn(f["name"])]
    if mods or len(fns) >= 2:
        return False
    if structs:
        kebab = snake_to_kebab(_struct_to_snake(structs[0]))
        return kebab != slug
    if len(fns) == 1:
        return snake_to_kebab(fns[0]["name"]) != slug
    return False


def _struct_to_snake(name: str) -> str:
    s = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", name)
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", s)
    return s.lower()


def rename_target(slug: str, text: str) -> str:
    structs = re.findall(r"^pub struct (\w+)", text, re.M)
    if structs:
        return snake_to_kebab(_struct_to_snake(structs[0]))
    fns = [f for f in extract_pub_fns(text) if not is_helper_fn(f["name"])]
    if len(fns) == 1:
        return snake_to_kebab(fns[0]["name"])
    return slug


def apply_mapping(mapping: Mapping) -> None:
    written = 0
    deleted = set()
    for old_rel, recs in mapping.by_old.items():
        for rec in recs:
            new_dir = Path(rec["new_dir"])
            new_file = REPO / rec["new_rel"]
            if not DRY:
                new_dir.mkdir(parents=True, exist_ok=True)
                new_file.write_text(rec["content"], encoding="utf-8")
            written += 1
        old_path = REPO / old_rel
        if recs and recs[0].get("delete_old") and old_path.exists():
            # don't delete if we wrote back into the same path
            same = any((REPO / r["new_rel"]) == old_path for r in recs)
            if not same:
                if not DRY:
                    old_path.unlink()
                    parent = old_path.parent
                    try:
                        parent.rmdir()
                    except OSError:
                        pass
                deleted.add(old_rel)
    print(f"wrote {written} command files, removed {len(deleted)} old files")


def rewrite_glue(mapping: Mapping) -> int:
    count = 0
    # index: old path posix -> recs
    old_to_recs = mapping.by_old
    for glue in (REPO / "✏️s").rglob(GLUE):
        if "target" in glue.parts:
            continue
        text = glue.read_text(encoding="utf-8")
        orig = text

        def repl(m: re.Match) -> str:
            indent = m.group(1)
            path = m.group(2)
            # path is relative to glue file
            glue_dir = glue.parent
            abs_target = (glue_dir / path).resolve()
            try:
                rel = abs_target.relative_to(REPO).as_posix()
            except ValueError:
                return m.group(0)
            recs = old_to_recs.get(rel)
            if not recs:
                return m.group(0)
            lines = []
            for rec in recs:
                new_abs = REPO / rec["new_rel"]
                new_rel_from_glue = os.path.relpath(new_abs, glue_dir).replace("\\", "/")
                lines.append(f'{indent}#[path = "{new_rel_from_glue}"]')
                lines.append(f'{indent}pub mod {rec["new_mod"]};')
            return "\n".join(lines)

        text = PATH_MOD_RE.sub(repl, text)
        if text != orig:
            if not DRY:
                glue.write_text(text, encoding="utf-8")
            count += 1
    print(f"updated {count} glue.rs files")
    return count


def rewrite_imports(mapping: Mapping) -> int:
    """Rewrite commands::old_mod::inner → commands::inner across rust sources."""
    # per commands parent directory, old glue mod → list of inner new mods
    # We derive old glue mod from the glue file itself... easier: from folder slug.
    # commands::text::set_text → commands::set_text
    # Build: for each old file, old_mod_name (folder slug snake) and list of inner new_mods
    transitions: list[tuple[str, str]] = []  # (old_qual, new_qual) longest first
    glob_rewrites: list[tuple[str, str, list[str]]] = []  # old_mod, pattern, new mods

    for old_rel, recs in mapping.by_old.items():
        old_path = Path(old_rel)
        old_mod = kebab_to_snake(split_emoji_slug(old_path.parent.name)[1])
        new_mods = [r["new_mod"] for r in recs]
        if recs and recs[0].get("rename_only") and len(recs) == 1:
            new_mod = recs[0]["new_mod"]
            if old_mod != new_mod:
                transitions.append((f"commands::{old_mod}", f"commands::{new_mod}"))
            continue
        if len(new_mods) == 1 and new_mods[0] == old_mod:
            # flatten in place same name: commands::locale::set_locale → commands::set_locale
            inner = new_mods[0]
            # if folder was noun and inner differs
            pass
        for rec in recs:
            inner = rec["new_mod"]
            # commands::text::text_edit → commands::text_edit
            transitions.append((f"commands::{old_mod}::{inner}", f"commands::{inner}"))
        if old_mod not in new_mods:
            # leftover commands::text:: should not remain; glob form handled below
            glob_rewrites.append((old_mod, new_mods))

    # longest old first
    transitions.sort(key=lambda t: len(t[0]), reverse=True)

    files_changed = 0
    for path in walk_rs():
        text = path.read_text(encoding="utf-8")
        orig = text
        for old, new in transitions:
            text = text.replace(old, new)
        known_old = {kebab_to_snake(split_emoji_slug(Path(old_rel).parent.name)[1]) for old_rel in mapping.by_old}
        def glob_sub(m: re.Match) -> str:
            oldm = m.group(1)
            inner = m.group(2)
            if oldm not in known_old:
                return m.group(0)
            return f"commands::{{{inner}}}"
        text = re.sub(r"commands::([A-Za-z0-9_]+)::\{([^}]+)\}", glob_sub, text)
        # triple leftover: commands::text::set_locale::SetLocale already handled
        # comments: 🎮️commands/<oldfolder>
        for old_rel, recs in mapping.by_old.items():
            old_folder = Path(old_rel).parent.name
            if any(old_folder in (Path(r["new_rel"]).parent.name) for r in recs):
                continue
            # replace folder mention with first new folder
            new_folder = Path(recs[0]["new_rel"]).parent.name
            text = text.replace(f"{COMMANDS_MARK}/{old_folder}", f"{COMMANDS_MARK}/{new_folder}")
        if text != orig:
            if not DRY:
                path.write_text(text, encoding="utf-8")
            files_changed += 1
    print(f"updated {files_changed} rust/text files for imports/paths")
    return files_changed


def walk_rs() -> list[Path]:
    out = []
    for root, dirs, files in os.walk(REPO / "✏️s"):
        if "target" in Path(root).parts:
            dirs.clear()
            continue
        if FILTER and FILTER not in str(Path(root)):
            continue
        for f in files:
            if f.endswith((".rs", ".ts", ".md", ".graphql", ".json")):
                out.append(Path(root) / f)
    if not FILTER:
        out.append(REPO / "📜️script.ts")
    return out


def rewrite_policy_allowlist(mapping: Mapping) -> None:
    script = REPO / "📜️script.ts"
    text = script.read_text(encoding="utf-8")
    orig = text
    for old_rel, recs in mapping.by_old.items():
        if old_rel not in text:
            continue
        replacement = ",\n  ".join(json.dumps(r["new_rel"]) for r in recs)
        text = text.replace(json.dumps(old_rel), replacement)
    if text != orig and not DRY:
        script.write_text(text, encoding="utf-8")
        print("updated 📜️script.ts allowlist paths")


def dump_mapping(mapping: Mapping, dest: Path) -> None:
    serial = {}
    for old, recs in mapping.by_old.items():
        serial[old] = [{k: v for k, v in r.items() if k != "content"} for r in recs]
    dest.write_text(json.dumps(serial, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def main() -> int:
    files = find_command_files()
    print(f"command files: {len(files)} filter={FILTER!r} dry={DRY}")
    mapping = Mapping()
    for path in files:
        try:
            plan_file(path, mapping)
        except Exception as e:
            print(f"PLAN FAIL {path.relative_to(REPO)}: {e}")
            raise
    print(f"planned rewrites from {len(mapping.by_old)} old files")
    ticket = Path(__file__).resolve().parent
    mapping_dest = Path(MAPPING_OUT) if MAPPING_OUT else ticket / "scratch-mapping.json"
    dump_mapping(mapping, mapping_dest)
    if DRY:
        print("DRY run — no writes")
        for old, recs in mapping.by_old.items():
            print(old)
            for r in recs:
                print("   ->", r["new_rel"], "mod", r["new_mod"])
        return 0
    apply_mapping(mapping)
    rewrite_glue(mapping)
    rewrite_imports(mapping)
    if not SKIP_POLICY:
        rewrite_policy_allowlist(mapping)
    dump_mapping(mapping, mapping_dest)
    return 0


if __name__ == "__main__":
    sys.exit(main())
