"""🧭️ Lists the tsc roots of slice acd: every TS file of the app-channel module tree, its SDK wrapper and the known external importers."""
import subprocess, os
R = "/Users/ueli/Documents/semio"
dirs = ["🧰️framework/🔨️modules/🧾️app-channel", "🧰️framework/📦️packages/🟦️typescript/🧾️app-channel", "🧰️framework/🔨️modules/📇️directory/🧬️schema"]
extra = [l.strip() for l in open(os.path.join(os.path.dirname(__file__), "extra-roots.txt"), encoding="utf8") if l.strip()]
out = set()
for d in dirs:
    r = subprocess.run(["git", "-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "ls-files", "--cached", "--others", "--exclude-standard", "--", d], cwd=R, capture_output=True, text=True)
    for p in r.stdout.splitlines():
        if p.endswith((".ts", ".tsx")):
            out.add(R + "/" + p)
for p in extra:
    if os.path.exists(R + "/" + p):
        out.add(R + "/" + p)
for p in sorted(out):
    if os.path.exists(p):
        print(p)
