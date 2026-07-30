#!/usr/bin/env python3
"""Fix references shaped like "./🧰/..." (a repo-root-relative path with a redundant
"./" prefix) -- these evaded rewrite-refs-v2.py's regexes (TS_IMPORT_RE expects a
file-relative "./x" it resolves against the file's own dir; RESOLVE_ABS_RE expects no
"./" prefix at all). Runs AFTER the physical rename: translates the OLD (still-embedded
in the string) segment names to NEW ones via translate-path-v2.py, verifying the
resulting NEW path exists on the current (already-renamed) filesystem before rewriting."""
import os
import re
import glob
import importlib.util

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("translate_path_v2", os.path.join(HERE, "translate-path-v2.py"))
tp = importlib.util.module_from_spec(spec)
spec.loader.exec_module(tp)

EXCLUDE_DIRS = {"node_modules", "target", ".git", ".repo", ".nx", "pkg"}
PATTERN = re.compile(r'"(\./(?:🧰|✏️|🌎|♻️)/[^"]+)"')

stats = {"files_changed": 0, "strings_changed": 0}


def process(fpath):
    try:
        content = open(fpath, encoding="utf-8").read()
    except Exception:
        return
    changed = {"v": False}

    def repl(m):
        ref = m.group(1)
        old_core = ref[2:]  # strip "./"
        new_core = tp.translate(old_core)
        if new_core == old_core:
            return m.group(0)
        if not os.path.exists(new_core):
            return m.group(0)
        changed["v"] = True
        stats["strings_changed"] += 1
        return f'"./{new_core}"'

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
