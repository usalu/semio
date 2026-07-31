#!/usr/bin/env python3
"""Catch-all: fix ANY quoted string ending in bare 📦.rs/📦.ts/📦.tsx (regardless of
whether it's a same-dir bare ref, a "./..." relative ref, or a repo-root-relative
absolute-style ref passed to resolve()) -- these are all first-round leftovers that
various category-specific regexes in the other fixer scripts missed. Runs AFTER the
physical rename; resolves each candidate both file-relative and repo-root-relative,
uses whichever resolves to the correct NEW entry filename on disk."""
import os
import re
import glob

RENAME = {"📦.rs": "📦lib.rs", "📦.ts": "📦index.ts", "📦.tsx": "📦index.tsx"}
EXCLUDE_DIRS = {"node_modules", "target", ".git", ".repo", ".nx", "pkg"}
PATTERN = re.compile(r'"([^"]*(?:📦\.rs|📦\.ts|📦\.tsx))"')

stats = {"files_changed": 0, "strings_changed": 0}


def resolve_and_fix(fpath, ref):
    base = os.path.basename(ref)
    if base not in RENAME:
        return None
    new_base = RENAME[base]
    prefix = ref[: -len(base)]
    new_ref = prefix + new_base
    # try file-relative first
    file_dir = os.path.dirname(fpath)
    candidate_rel = os.path.normpath(os.path.join(file_dir, new_ref))
    if os.path.exists(candidate_rel):
        return new_ref
    # try repo-root-relative (strip leading ./ if present)
    core = new_ref[2:] if new_ref.startswith("./") else new_ref
    if os.path.exists(core):
        return new_ref
    return None


def process(fpath):
    try:
        content = open(fpath, encoding="utf-8").read()
    except Exception:
        return
    changed = {"v": False}

    def repl(m):
        ref = m.group(1)
        if "${" in ref:
            return m.group(0)
        fixed = resolve_and_fix(fpath, ref)
        if fixed is None:
            return m.group(0)
        changed["v"] = True
        stats["strings_changed"] += 1
        return f'"{fixed}"'

    new_content = PATTERN.sub(repl, content)
    if changed["v"]:
        open(fpath, "w", encoding="utf-8").write(new_content)
        stats["files_changed"] += 1


def main():
    files = ["script.ts", "vitest.config.ts"]
    for root in ("🧰framework", "✏️s", "🌎hub", "♻️mit-bestand"):
        files += glob.glob(f"{root}/**/*.ts", recursive=True) + glob.glob(f"{root}/**/*.tsx", recursive=True)
    for f in files:
        if any(d in f for d in EXCLUDE_DIRS):
            continue
        if not os.path.exists(f):
            continue
        process(f)
    print(stats)


if __name__ == "__main__":
    main()
