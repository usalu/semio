"""🌳️ Rename 📦️glue.rs / 📦️index.ts entry files to their configurableEntryContracts kind-only
basenames (🦀️.rs / 🟦️.ts) and repoint every literal reference repo-wide.

Rename scope: ✏️s, 🧰️framework, 🌎️hub only (per task).
Reference-substitution scope: whole repo, excluding node_modules/target/.git/dist/
storybook-static/target-engines/.🧬semio/.cursor and the two frozen-evidence fixture dirs.
"""
import os, subprocess, sys

RENAME_ROOTS = ["✏️s", "🧰️framework", "🌎️hub"]
SKIP_DIRS = {"node_modules", "target", ".git", "dist", "build", "pkg", "__pycache__", "coverage",
             ".venv", "storybook-static", "target-engines", ".nx", ".🧬semio", ".cursor"}
FROZEN_PATH_SUBSTRINGS = [
    "🧫️fixtures/🧪️remaining-package-purity-authority/",
    "🧫️fixtures/🧪️cad-draw-path-projection/",
]
TEXT_EXT = {".rs", ".ts", ".tsx", ".js", ".mjs", ".cjs", ".json", ".toml", ".md", ".py", ".graphql",
            ".proto", ".semio", ".yaml", ".yml", ".ksy", ".g4", ".ebnf", ".abnf", ".spicy", ".wit", ".html"}

def walk(roots):
    for r in roots:
        for dp, dn, fn in os.walk(r):
            dn[:] = [x for x in dn if x not in SKIP_DIRS]
            for n in fn:
                p = os.path.join(dp, n)
                if any(s in p for s in FROZEN_PATH_SUBSTRINGS):
                    continue
                yield p, n

def do_rename(old, new, apply_it):
    targets = [p for p, n in walk(RENAME_ROOTS) if n == old]
    renamed = 0
    collisions = []
    for p in targets:
        dst = os.path.join(os.path.dirname(p), new)
        if os.path.exists(dst):
            collisions.append(p)
            continue
        if apply_it:
            subprocess.run(["mv", p, dst], check=True)
        renamed += 1
    print(f"  {'APPLIED' if apply_it else 'DRY-RUN'} rename {old} -> {new}: {renamed} files, {len(collisions)} collisions")
    for c in collisions:
        print(f"    !! collision, skipped: {c}")
    return renamed, collisions

def do_refs(old, new, apply_it):
    subs = files_touched = 0
    touched_files = []
    for p, n in walk(["."]):
        if os.path.splitext(n)[1] not in TEXT_EXT:
            continue
        try:
            s = open(p, encoding="utf8").read()
        except Exception:
            continue
        if old not in s:
            continue
        c = s.count(old)
        if apply_it:
            open(p, "w", encoding="utf8").write(s.replace(old, new))
        subs += c
        files_touched += 1
        touched_files.append((p, c))
    print(f"  {'APPLIED' if apply_it else 'DRY-RUN'} refs {old} -> {new}: {subs} occurrences in {files_touched} files")
    return subs, touched_files

if __name__ == "__main__":
    old, new = sys.argv[1], sys.argv[2]
    apply_it = "--apply" in sys.argv
    refs_only = "--refs-only" in sys.argv
    if not refs_only:
        do_rename(old, new, apply_it)
    subs, touched = do_refs(old, new, apply_it)
    if "--list-files" in sys.argv:
        for p, c in touched:
            print(f"    {c:4d}  {p}")
