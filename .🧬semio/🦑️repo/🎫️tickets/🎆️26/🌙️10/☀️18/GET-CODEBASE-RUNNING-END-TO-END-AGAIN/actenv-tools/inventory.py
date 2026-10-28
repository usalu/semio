"""🧾 Streams the environment names the repository's own code reads (not tests), per tree; prints `name count first-file` sorted. Low-RAM: one file at a time. usage: inventory.py ROOT TREE... ; RSS guard 500 MB."""
import os, re, resource, sys
READ = re.compile(r'(?:\bprocess\.env|\b[A-Za-z_]*[eE]nv(?:ironment)?|\bbaseEnv)\??(?:\.([A-Z][A-Z0-9_]{2,})\b|\["([A-Z][A-Z0-9_]{2,})"\])')
GO = re.compile(r'os\.(?:Getenv|LookupEnv)\("([A-Z][A-Z0-9_]+)"\)')
RS = re.compile(r'(?:std::)?env::var(?:_os)?\("([A-Z][A-Z0-9_]+)"\)')
SKIP = {"node_modules", ".cache", "🧪️tests", "🗑️generated", "dist", "target", ".git"}
EXT = {".ts", ".mjs", ".js", ".tsx", ".go", ".rs"}
found = {}
root = sys.argv[1]
for tree in sys.argv[2:]:
    for here, dirs, files in os.walk(os.path.join(root, tree)):
        dirs[:] = [d for d in dirs if d not in SKIP]
        for name in files:
            if os.path.splitext(name)[1] not in EXT: continue
            path = os.path.join(here, name)
            try: text = open(path, encoding="utf8", errors="ignore").read()
            except OSError: continue
            if resource.getrusage(resource.RUSAGE_SELF).ru_maxrss > 500 * 1024 * 1024: sys.exit("RSS guard")
            for rx in (READ, GO, RS):
                for m in rx.finditer(text):
                    n = next(g for g in m.groups() if g)
                    row = found.setdefault(n, [0, os.path.relpath(path, root)]); row[0] += 1
for n, (c, f) in sorted(found.items(), key=lambda kv: -kv[1][0]): print(n, c, f[-80:])
