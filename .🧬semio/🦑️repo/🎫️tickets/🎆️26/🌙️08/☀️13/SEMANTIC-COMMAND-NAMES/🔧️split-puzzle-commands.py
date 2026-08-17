#!/usr/bin/env python3
"""Split 🧩️puzzle 🎮️commands noun buckets into one verb-noun folder per command."""
from __future__ import annotations

import json
import os
import re
import sys
from pathlib import Path

REPO = Path("/Users/ueli/Documents/semio")
PUZZLE = REPO / "✏️s/🔌️plugins/🧩️puzzle"
TICKET = Path(__file__).resolve().parent
GLUE = PUZZLE / "📦️packages/🦀️rust/📦️glue.rs"
COMPONENT = "🦀️component.rs"
COMMANDS = "🎮️commands"
DRY = os.environ.get("DRY", "").strip() in {"1", "true", "yes"}

FN_RE = re.compile(r"^((?:pub(?:\s*\(\s*crate\s*\))?\s+)?(?:async\s+)?fn\s+(\w+))", re.M)
CONST_RE = re.compile(r"^((?:pub(?:\s*\(\s*crate\s*\))?\s+)?const\s+(\w+))", re.M)
USE_RE = re.compile(r"^use\s+.+?;\s*$", re.M)


def split_emoji_slug(name: str) -> tuple[str, str]:
    m = re.match(r"^([^A-Za-z0-9]+)(.+)$", name)
    if not m:
        return "", name
    return m.group(1), m.group(2)


def snake_to_kebab(name: str) -> str:
    return name.replace("_", "-")


def match_braces(text: str, brace_idx: int) -> int:
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


def match_depth0_semi(text: str, start: int) -> int:
    depth_brace = depth_brack = depth_paren = 0
    i = start
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
            depth_brace += 1
        elif ch == "}":
            depth_brace -= 1
        elif ch == "[":
            depth_brack += 1
        elif ch == "]":
            depth_brack -= 1
        elif ch == "(":
            depth_paren += 1
        elif ch == ")":
            depth_paren -= 1
        elif ch == ";" and depth_brace == 0 and depth_brack == 0 and depth_paren == 0:
            return i
        i += 1
    raise ValueError("no semicolon")


def walk_back_trivia(text: str, start: int) -> int:
    line_start = text.rfind("\n", 0, start) + 1
    prelude = text[:line_start].rstrip("\n")
    while True:
        prev_nl = prelude.rfind("\n")
        line = prelude[prev_nl + 1 :] if prev_nl >= 0 else prelude
        stripped = line.strip()
        if stripped.startswith("//#region") or stripped.startswith("//#endregion"):
            break
        if (
            stripped.startswith("//")
            or stripped.startswith("#[")
            or stripped.startswith("///")
            or stripped == ""
        ):
            start = (prev_nl + 1) if prev_nl >= 0 else 0
            prelude = text[:start].rstrip("\n")
            if prev_nl < 0:
                break
            continue
        break
    return start


def extract_tests(text: str) -> tuple[int, int, str] | None:
    m = re.search(r"^//#region 🧪️Tests\n", text, re.M)
    if m:
        endm = re.search(r"^//#endregion 🧪️Tests\s*\n?", text[m.start() :], re.M)
        if not endm:
            return m.start(), len(text), text[m.start() :]
        end = m.start() + endm.end()
        return m.start(), end, text[m.start() : end]
    cfg = re.search(r"^#\[cfg\(test\)\]\s*\nmod tests \{", text, re.M)
    if not cfg:
        return None
    brace = text.find("{", cfg.start())
    end = match_braces(text, brace)
    stop = end + 1
    if stop < len(text) and text[stop] == "\n":
        stop += 1
    start = walk_back_trivia(text, cfg.start())
    return start, stop, text[start:stop]


def first_param_header(text: str, name_start: int, name: str) -> str:
    i = name_start + len(name)
    n = len(text)
    while i < n and text[i] in " \t":
        i += 1
    if i < n and text[i] == "<":
        depth = 1
        i += 1
        while i < n and depth:
            if text[i] == "<":
                depth += 1
            elif text[i] == ">":
                depth -= 1
            i += 1
    while i < n and text[i] in " \t\n":
        i += 1
    if i >= n or text[i] != "(":
        return ""
    depth = 1
    start = i + 1
    i += 1
    while i < n and depth:
        ch = text[i]
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
            if depth == 0:
                return text[start:i]
        i += 1
    return text[start:]


def is_command_fn(header: str) -> bool:
    first = header.split(",")[0] if header else ""
    return "ActionCtx" in first


def extract_fns(text: str) -> list[dict]:
    out = []
    for m in FN_RE.finditer(text):
        name = m.group(2)
        rest = text[m.end() :]
        brace_rel = rest.find("{")
        semi_rel = rest.find(";")
        if brace_rel < 0 or (semi_rel >= 0 and semi_rel < brace_rel):
            continue
        abs_brace = m.end() + brace_rel
        end = match_braces(text, abs_brace)
        stop = end + 1
        if stop < len(text) and text[stop] == "\n":
            stop += 1
        start = walk_back_trivia(text, m.start())
        header = first_param_header(text, m.start() + m.group(1).rfind(name), name)
        out.append(
            {
                "kind": "fn",
                "name": name,
                "start": start,
                "end": stop,
                "block": text[start:stop],
                "is_command": m.group(1).lstrip().startswith("pub") and is_command_fn(header),
            }
        )
    return out


def extract_consts(text: str) -> list[dict]:
    out = []
    for m in CONST_RE.finditer(text):
        name = m.group(2)
        end = match_depth0_semi(text, m.end())
        stop = end + 1
        if stop < len(text) and text[stop] == "\n":
            stop += 1
        start = walk_back_trivia(text, m.start())
        out.append({"kind": "const", "name": name, "start": start, "end": stop, "block": text[start:stop], "is_command": False})
    return out


def original_doc(text: str) -> str:
    lines = []
    for line in text.split("\n"):
        if line.startswith("//!"):
            lines.append(line)
        elif lines:
            break
    return "\n".join(lines)


def file_uses(text: str) -> str:
    return "\n".join(line for line in text.split("\n") if line.startswith("use "))


def needed_helpers(helpers: list[dict], body: str, tests: str) -> list[dict]:
    blob = body + "\n" + tests
    chosen: list[dict] = []
    remaining = list(helpers)
    changed = True
    while changed:
        changed = False
        keep = []
        for h in remaining:
            if h["name"] in blob or any(h["name"] in c["block"] for c in chosen):
                chosen.append(h)
                changed = True
            else:
                keep.append(h)
        remaining = keep
        blob = body + "\n" + tests + "\n" + "\n".join(c["block"] for c in chosen)
    return chosen


def compose_file(*, emoji: str, kebab: str, uses: str, helpers: list[dict], body: str, tests: str) -> str:
    chunks = [f"//! {emoji} `{kebab}` command.\n"]
    if uses.strip():
        chunks.append("\n" + uses.rstrip() + "\n")
    chunks.append("\n")
    helper_text = "\n".join(h["block"].rstrip() for h in helpers)
    if helper_text.strip():
        chunks.append(helper_text.rstrip() + "\n\n")
    chunks.append(body.rstrip() + "\n")
    if tests.strip():
        chunks.append("\n" + tests.rstrip() + "\n")
    out = "".join(chunks)
    out = re.sub(r"\n{3,}", "\n\n", out)
    if not out.endswith("\n"):
        out += "\n"
    return out


def command_files() -> list[Path]:
    out = []
    for app in (PUZZLE / "🎛️apps").iterdir():
        cmd = app / COMMANDS
        if not cmd.is_dir():
            continue
        for folder in cmd.iterdir():
            f = folder / COMPONENT
            if f.is_file():
                out.append(f)
    return sorted(out)


def command_mod_name(fn_name: str, folder_slug: str) -> tuple[str, str]:
    """Return (snake_mod, kebab)."""
    if fn_name == "apply" and folder_slug == "sun":
        return "apply_sun", "apply-sun"
    return fn_name, snake_to_kebab(fn_name)


def plan() -> dict:
    mapping: dict[str, list[dict]] = {}
    ident_map: dict[tuple[str, str, str], str] = {}
    # (app_dir_name, old_mod, ident) -> new_mod
    folder_map: dict[tuple[str, str], str] = {}
    # (app_dir_name, old_folder) -> first new folder (for comments)

    for path in command_files():
        text = path.read_text(encoding="utf-8")
        app_dir = path.parts[path.parts.index("🎛️apps") + 1]
        folder = path.parent.name
        emoji, slug = split_emoji_slug(folder)
        old_mod = slug.replace("-", "_")
        fns = extract_fns(text)
        consts = extract_consts(text)
        tests_span = extract_tests(text)
        tests = tests_span[2] if tests_span else ""
        commands = [f for f in fns if f["is_command"]]
        helpers = [f for f in fns if not f["is_command"]] + consts
        uses = file_uses(text)
        existing = {p.name for p in path.parent.parent.iterdir() if p.is_dir()}
        recs = []
        used_mods: set[str] = set()
        test_owner = None
        if tests:
            for cmd in commands:
                if any(h["name"] in tests for h in helpers) and any(h["name"] in cmd["block"] for h in helpers):
                    test_owner = cmd["name"]
                    break
            if test_owner is None:
                test_owner = commands[0]["name"] if commands else None

        if not commands:
            raise SystemExit(f"no commands in {path}")

        for cmd in commands:
            new_mod, kebab = command_mod_name(cmd["name"], slug)
            new_folder = f"{emoji}{kebab}"
            if new_folder in existing and new_folder != folder:
                kebab = f"{slug}-{kebab}"
                new_mod = f"{old_mod}_{new_mod}"
                new_folder = f"{emoji}{kebab}"
            if new_mod in used_mods:
                kebab = f"{slug}-{snake_to_kebab(cmd['name'])}"
                new_mod = f"{old_mod}_{cmd['name']}"
                new_folder = f"{emoji}{kebab}"
            existing.add(new_folder)
            used_mods.add(new_mod)
            cmd_tests = tests if test_owner == cmd["name"] else ""
            chosen = needed_helpers(helpers, cmd["block"], cmd_tests)
            body = compose_file(
                emoji=emoji,
                kebab=kebab,
                uses=uses,
                helpers=chosen,
                body=cmd["block"],
                tests=cmd_tests,
            )
            new_dir = path.parent.parent / new_folder
            new_file = new_dir / COMPONENT
            rec = {
                "old_mod": old_mod,
                "old_folder": folder,
                "new_mod": new_mod,
                "kebab": kebab,
                "fn": cmd["name"],
                "new_rel": new_file.relative_to(REPO).as_posix(),
                "new_dir": str(new_dir),
                "content": body,
                "idents": [cmd["name"]] + [h["name"] for h in chosen],
                "rename_only": len(commands) == 1 and new_folder != folder and not chosen,
            }
            recs.append(rec)
            for ident in rec["idents"]:
                ident_map[(app_dir, old_mod, ident)] = new_mod
            if (app_dir, folder) not in folder_map:
                folder_map[(app_dir, folder)] = new_folder

        old_rel = path.relative_to(REPO).as_posix()
        mapping[old_rel] = recs

    return {"by_old": mapping, "ident_map": ident_map, "folder_map": folder_map}


def apply_files(planned: dict) -> None:
    written = 0
    deleted = 0
    for old_rel, recs in planned["by_old"].items():
        old_path = REPO / old_rel
        for rec in recs:
            new_dir = Path(rec["new_dir"])
            new_file = REPO / rec["new_rel"]
            if not DRY:
                new_dir.mkdir(parents=True, exist_ok=True)
                new_file.write_text(rec["content"], encoding="utf-8")
            written += 1
        same = any((REPO / r["new_rel"]) == old_path for r in recs)
        if not same and old_path.exists():
            if not DRY:
                old_path.unlink()
                try:
                    old_path.parent.rmdir()
                except OSError:
                    pass
            deleted += 1
    print(f"wrote {written} command files, removed {deleted} old files")


def rewrite_glue(planned: dict) -> None:
    text = GLUE.read_text(encoding="utf-8")
    orig = text

    def repl_block(m: re.Match) -> str:
        indent = m.group(1)
        body = m.group(2)
        lines = []
        path_re = re.compile(
            r'#\[path\s*=\s*"([^"]+)"\]\s*\n\s*pub mod (\w+);'
        )
        for pm in path_re.finditer(body):
            path, _mod = pm.group(1), pm.group(2)
            abs_target = (GLUE.parent / path).resolve()
            try:
                rel = abs_target.relative_to(REPO).as_posix()
            except ValueError:
                lines.append(pm.group(0))
                continue
            recs = planned["by_old"].get(rel)
            if not recs:
                lines.append(f'{indent}#[path = "{path}"]\n{indent}pub mod {_mod};')
                continue
            for rec in recs:
                new_rel = os.path.relpath(REPO / rec["new_rel"], GLUE.parent).replace("\\", "/")
                lines.append(f'{indent}#[path = "{new_rel}"]')
                lines.append(f'{indent}pub mod {rec["new_mod"]};')
        return f"{m.group(0).split('{', 1)[0]}{{\n" + "\n".join(lines) + "\n" + indent[:-4] + "}"

    # Replace each `pub mod commands { ... }` whose body contains 🎮️commands/
    pattern = re.compile(
        r"^([ \t]*)pub mod commands \{((?:[^{}]|\n)*?🎮️commands/(?:[^{}]|\n)*?)^\1\}",
        re.M,
    )
    # The nested braces in commands blocks are only path/mod lines — no inner `{}`.
    # But lookbehind: the block uses no nested braces. Good.

    def repl(m: re.Match) -> str:
        indent = m.group(1)
        inner_indent = indent + "    "
        body = m.group(2)
        path_re = re.compile(r'#\[path\s*=\s*"([^"]+)"\]\s*\n[ \t]*pub mod (\w+);')
        lines = []
        for pm in path_re.finditer(body):
            path, oldm = pm.group(1), pm.group(2)
            abs_target = (GLUE.parent / path).resolve()
            try:
                rel = abs_target.relative_to(REPO).as_posix()
            except ValueError:
                lines.append(f'{inner_indent}#[path = "{path}"]')
                lines.append(f"{inner_indent}pub mod {oldm};")
                continue
            recs = planned["by_old"].get(rel)
            if not recs:
                lines.append(f'{inner_indent}#[path = "{path}"]')
                lines.append(f"{inner_indent}pub mod {oldm};")
                continue
            for rec in recs:
                new_rel = os.path.relpath(REPO / rec["new_rel"], GLUE.parent).replace("\\", "/")
                lines.append(f'{inner_indent}#[path = "{new_rel}"]')
                lines.append(f"{inner_indent}pub mod {rec['new_mod']};")
        return f"{indent}pub mod commands {{\n" + "\n".join(lines) + f"\n{indent}}}"

    text = pattern.sub(repl, text)
    if text == orig:
        raise SystemExit("glue.rs commands blocks were not rewritten")
    if not DRY:
        GLUE.write_text(text, encoding="utf-8")
    print("updated glue.rs")


def app_dir_for_rust(rust_app: str) -> str:
    return {"puzzle2d": "◻2d", "puzzle3d": "🧊️3d", "puzzle5d": "🖐️5d"}[rust_app]


def rewrite_imports(planned: dict) -> None:
    ident_map: dict[tuple[str, str, str], str] = planned["ident_map"]
    folder_map: dict[tuple[str, str], str] = planned["folder_map"]
    by_old = planned["by_old"]

    # per app: old_mod -> list of new mods (appearance order)
    app_new_mods: dict[str, list[str]] = {"◻2d": [], "🧊️3d": [], "🖐️5d": []}
    app_old_to_news: dict[str, dict[str, list[str]]] = {"◻2d": {}, "🧊️3d": {}, "🖐️5d": {}}
    seen_mod: dict[str, set[str]] = {"◻2d": set(), "🧊️3d": set(), "🖐️5d": set()}
    for old_rel, recs in by_old.items():
        app_dir = old_rel.split("🎛️apps/")[1].split("/")[0]
        for rec in recs:
            if rec["new_mod"] not in seen_mod[app_dir]:
                app_new_mods[app_dir].append(rec["new_mod"])
                seen_mod[app_dir].add(rec["new_mod"])
            app_old_to_news[app_dir].setdefault(rec["old_mod"], []).append(rec["new_mod"])

    rust_apps = {"◻2d": "puzzle2d", "🧊️3d": "puzzle3d", "🖐️5d": "puzzle5d"}
    changed_files = 0
    for path in PUZZLE.rglob("*.rs"):
        if path.name == "📦️glue.rs":
            continue
        text = path.read_text(encoding="utf-8")
        orig = text
        # which app?
        app_dir = None
        for ad in rust_apps:
            if f"🎛️apps/{ad}/" in path.as_posix() or f"/🎛️apps/{ad}/" in ("/" + path.relative_to(PUZZLE).as_posix()):
                app_dir = ad
                break
        if app_dir is None:
            rel = path.relative_to(PUZZLE).as_posix()
            for ad in rust_apps:
                if rel.startswith(f"🎛️apps/{ad}/"):
                    app_dir = ad
                    break
        if app_dir is None:
            continue

        rust = rust_apps[app_dir]
        new_mods = app_new_mods[app_dir]
        use_pat = re.compile(rf"use crate::apps::{rust}::commands::\{{([^}}]+)\}};")

        def use_repl(m: re.Match) -> str:
            names = ", ".join(new_mods)
            return f"use crate::apps::{rust}::commands::{{{names}}};"

        text = use_pat.sub(use_repl, text)

        # selection as selection_commands → new mods already in the use list;
        # rewrite selection_commands::ident using selection's ident map
        for (ad, old_mod, ident), new_mod in ident_map.items():
            if ad != app_dir:
                continue
            text = text.replace(f"{old_mod}::{ident}", f"{new_mod}::{ident}")
            if old_mod == "selection":
                text = text.replace(f"selection_commands::{ident}", f"{new_mod}::{ident}")

        for (ad, old_folder), new_folder in folder_map.items():
            if ad != app_dir:
                continue
            text = text.replace(f"{COMMANDS}/{old_folder}", f"{COMMANDS}/{new_folder}")

        if text != orig:
            if not DRY:
                path.write_text(text, encoding="utf-8")
            changed_files += 1
    print(f"updated {changed_files} rust files for imports/paths")


def dump(planned: dict) -> None:
    serial = {}
    for old, recs in planned["by_old"].items():
        serial[old] = [{k: v for k, v in r.items() if k != "content"} for r in recs]
    dest = TICKET / "scratch-puzzle-mapping.json"
    dest.write_text(json.dumps(serial, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"dumped {dest.name} ({len(serial)} old files, {sum(len(v) for v in serial.values())} commands)")


def main() -> int:
    planned = plan()
    dump(planned)
    if DRY:
        print("DRY run")
        for old, recs in planned["by_old"].items():
            print(old)
            for r in recs:
                print("  ->", r["new_mod"], r["new_rel"])
        return 0
    apply_files(planned)
    rewrite_glue(planned)
    rewrite_imports(planned)
    dump(planned)
    leftover = list(command_files())
    nouns = []
    for p in leftover:
        _, slug = split_emoji_slug(p.parent.name)
        first = slug.split("-")[0]
        if first not in {
            "create", "delete", "insert", "remove", "add", "rename", "change", "update",
            "move", "drag", "resize", "rotate", "scale", "reorder", "edit", "replace",
            "duplicate", "connect", "disconnect", "bind", "unbind", "group", "ungroup",
            "flatten", "unflatten", "split", "merge", "extract", "inline", "clear", "fix",
            "toggle", "apply", "set", "format", "open", "load", "save", "import", "export",
            "lint", "request", "select", "hover", "evaluate", "run", "retry", "calibrate",
            "place", "commit", "input", "submit", "abort", "patch", "paint", "nudge",
            "reset", "focus", "step", "query", "fill", "snap", "ingest", "cycle",
            "cancel", "world", "translate", "zoom", "register", "retarget", "proximity",
            "engagement", "lod", "context", "relocate",
        }:
            nouns.append(p.parent.name)
    print("remaining command folders:", len(leftover))
    if nouns:
        print("remaining noun-like folders:", nouns)
    return 0


if __name__ == "__main__":
    sys.exit(main())
