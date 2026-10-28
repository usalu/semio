"""📋 [backlog] the law list of a group: every changed test file plus the `🧪️tests/🟦️.ts` suites that sit next to (or one/two levels above) a changed production file, restricted to files that exist in the mirror. usage: lawlist.py <mirror> <base> [more-changed-list...]"""
import os, subprocess, sys
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
mir, base = sys.argv[1], sys.argv[2]
home = os.path.join(os.environ.get("TMPDIR", "/tmp"), "sm")
changed = subprocess.run(["sh", C + "/backlog-tools/changed.sh", mir, base], capture_output=True, text=True).stdout.split("\n")
out = set()
for f in filter(None, changed):
    if not f.endswith((".ts", ".mjs", ".mts", ".json")) or "/node_modules/" in f or f.startswith("."): continue
    if "🧪️tests" in f.split("/"):
        if f.endswith(".ts") and (f.endswith("🟦️.ts") or "test" in os.path.basename(f)) and not f.endswith(".d.ts"): out.add(f)
        continue
    d = os.path.dirname(f)
    for _ in range(3):
        t = f"{d}/🧪️tests/🟦️.ts"
        if os.path.exists(f"{home}/{mir}/{t}"): out.add(t); break
        d = os.path.dirname(d)
print("\n".join(sorted(out)))
