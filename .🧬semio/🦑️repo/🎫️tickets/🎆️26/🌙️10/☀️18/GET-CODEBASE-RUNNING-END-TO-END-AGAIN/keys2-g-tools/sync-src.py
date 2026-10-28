"""python3 sync-src.py <src-dir> <dest-dir> [--absolute <live-dir>] : copies a module authored with its LIVE relative imports.
With --absolute every relative import that leaves the module directory is rewritten to the absolute path it has in <live-dir>, so the copy can be unit-tested outside the library before the one live batch.
Without it the copy is byte-identical (the install into the live tree)."""
import os, re, shutil, sys
src, dest = os.path.abspath(sys.argv[1]), os.path.abspath(sys.argv[2])
live = os.path.abspath(sys.argv[sys.argv.index("--absolute") + 1]) if "--absolute" in sys.argv else None
SPEC = re.compile(r'((?:from|import)\s*\(?\s*)(["\'])(\.{1,2}/[^"\']+)\2')
if live and os.path.exists(dest):
    shutil.rmtree(dest)
for here, dirs, files in os.walk(src):
    dirs[:] = [d for d in dirs if d != "node_modules"]
    for name in files:
        path = os.path.join(here, name)
        rel = os.path.relpath(path, src)
        out = os.path.join(dest, rel)
        os.makedirs(os.path.dirname(out), exist_ok=True)
        if not live or not name.endswith((".ts", ".mjs")):
            shutil.copy2(path, out)
            continue
        def fix(m):
            at = os.path.normpath(os.path.join(live, os.path.dirname(rel), m.group(3)))
            return m.group(0) if at.startswith(live + os.sep) else f"{m.group(1)}{m.group(2)}{at}{m.group(2)}"
        open(out, "w", encoding="utf-8").write(SPEC.sub(fix, open(path, encoding="utf-8").read()))
print("synced", src, "->", dest)
