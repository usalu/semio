#!/usr/bin/env python3
"""Fix references to bare 📦.rs/📦.ts/📦.tsx (first-round entry names) now that
they've been physically renamed to 📦lib.rs/📦index.ts/📦index.tsx. Runs AFTER the
physical rename, operating on the current (already-renamed-for-directories) tree.
Only rewrites a match when the OLD bare-📦 form no longer exists but the NEW form
does at the resolved location, so it's safe to run repeatedly / is idempotent."""
import os
import re
import glob
import json

RENAME = {"📦.rs": "📦lib.rs", "📦.ts": "📦index.ts", "📦.tsx": "📦index.tsx"}
SCOPE_ROOTS = ("🧰framework/", "✏️s/", "🌎hub/", "♻️mit-bestand/")
EXCLUDE_DIRS = {"node_modules", "target", ".git", ".repo", ".nx", "pkg"}

stats = {"files_changed": 0, "strings_changed": 0}


def try_fix(base_dir, ref):
    if "${" in ref:
        return None
    query = ""
    core = ref
    if "?" in ref:
        core, query = ref.split("?", 1)
        query = "?" + query
    base = os.path.basename(core)
    if base not in RENAME:
        return None
    old_abs = os.path.normpath(os.path.join(base_dir, core)).replace(os.sep, "/")
    if not old_abs.startswith(SCOPE_ROOTS) and not any(f"/{r}" in ("/" + old_abs) for r in SCOPE_ROOTS):
        pass
    new_core = core[: -len(base)] + RENAME[base]
    new_abs = os.path.normpath(os.path.join(base_dir, new_core)).replace(os.sep, "/")
    if os.path.exists(new_abs):
        return new_core + query
    return None


def process(fpath, pattern, group, base_dir=None):
    try:
        content = open(fpath, encoding="utf-8").read()
    except Exception:
        return
    bd = base_dir if base_dir is not None else os.path.dirname(fpath)
    changed = {"v": False}

    def repl(m):
        ref = m.group(group)
        fixed = try_fix(bd, ref)
        if fixed is None:
            return m.group(0)
        changed["v"] = True
        stats["strings_changed"] += 1
        start, end = m.span(group)
        return m.group(0)[: start - m.start()] + fixed + m.group(0)[end - m.start() :]

    new_content = pattern.sub(repl, content)
    if changed["v"]:
        open(fpath, "w", encoding="utf-8").write(new_content)
        stats["files_changed"] += 1


CARGO_PATH_RE = re.compile(r'path\s*=\s*"([^"]+)"')
TS_IMPORT_RE = re.compile(r'((?:from\s+|import\()\s*")(\.\.?/[^"]+)(")')
RS_INCLUDE_RE = re.compile(r'(include_(?:str|bytes)!\(\s*")([^"]+)(")')
RS_PATH_ATTR_RE = re.compile(r'(#\[path\s*=\s*")([^"]+)(")')


def main():
    for f in glob.glob("**/Cargo.toml", recursive=True):
        if any(d in f for d in EXCLUDE_DIRS) or "compose/" in f:
            continue
        process(f, CARGO_PATH_RE, 1)
    for root in ("🧰framework", "✏️s", "🌎hub", "♻️mit-bestand"):
        for f in glob.glob(f"{root}/**/*.ts", recursive=True) + glob.glob(f"{root}/**/*.tsx", recursive=True):
            if any(d in f for d in EXCLUDE_DIRS):
                continue
            process(f, TS_IMPORT_RE, 2)
        for f in glob.glob(f"{root}/**/*.rs", recursive=True):
            if any(d in f for d in EXCLUDE_DIRS):
                continue
            process(f, RS_INCLUDE_RE, 2)
            process(f, RS_PATH_ATTR_RE, 2)
    # package.json exports
    for f in glob.glob("**/package.json", recursive=True):
        if any(d in f for d in EXCLUDE_DIRS) or "compose/" in f:
            continue
        try:
            d = json.loads(open(f, encoding="utf-8").read())
        except Exception:
            continue
        if "exports" not in d or not isinstance(d["exports"], dict):
            continue
        bd = os.path.dirname(f)
        changed = False
        for k, v in list(d["exports"].items()):
            if isinstance(v, str):
                fixed = try_fix(bd, v)
                if fixed:
                    d["exports"][k] = fixed
                    changed = True
        if changed:
            open(f, "w", encoding="utf-8").write(json.dumps(d, ensure_ascii=False, indent=2) + "\n")
            stats["files_changed"] += 1
            stats["strings_changed"] += 1
    print(json.dumps(stats, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
