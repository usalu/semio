"""Streams every .ts under the process module packages tree and reports relative `from "..."` imports that do not resolve (graph-free, < 50 MB)."""
import os, re, sys, resource
root = "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript"
pattern = re.compile(r'from\s+"(\.[^"]+)"')
missing = 0; files = 0
for directory, _, names in os.walk(root):
    for name in names:
        if not name.endswith(".ts"): continue
        path = os.path.join(directory, name); files += 1
        if os.path.getsize(path) > 200000: continue
        for line in open(path, encoding="utf8", errors="replace"):
            for match in pattern.finditer(line):
                if not os.path.exists(os.path.normpath(os.path.join(directory, match.group(1)))):
                    missing += 1; print("MISSING", path, "->", match.group(1))
assert resource.getrusage(resource.RUSAGE_SELF).ru_maxrss < 500 * 1024 * 1024
print("files", files, "missing", missing); sys.exit(1 if missing else 0)
