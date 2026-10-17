"""🧭️ Lists the tsc roots of slice c4p50: every TS file of the five module trees, their SDK wrappers and the known external importers."""
import subprocess, sys, os
R = "/Users/ueli/Documents/semio"
dirs = ["🧰️framework/🔨️modules/" + d for d in ("⏯️tool-run", "🔄️machine", "🛰️backbone", "🧾️app-channel", "📇️directory", "📞️directory-client", "🌐️browser-bundle")]
dirs += ["🧰️framework/📦️packages/🟦️typescript/" + d for d in ("⏯️tool-run", "🔄️machine", "🛰️backbone", "🧾️app-channel", "📇️directory")]
extra = [l.strip() for l in open(os.path.join(os.path.dirname(__file__), "extra-roots.txt"), encoding="utf8") if l.strip()]
out = set()
for d in dirs:
    r = subprocess.run(["git", "-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "ls-files", "--cached", "--others", "--exclude-standard", "--", d], cwd=R, capture_output=True, text=True)
    for p in r.stdout.splitlines():
        if p.endswith((".ts", ".tsx")) and "/node_modules/" not in p and "/🦀️rust/" not in p and "/🤖️generated/" not in p:
            out.add(R + "/" + p)
for p in extra:
    if os.path.exists(R + "/" + p):
        out.add(R + "/" + p)
for p in sorted(out):
    if os.path.exists(p):
        print(p)
