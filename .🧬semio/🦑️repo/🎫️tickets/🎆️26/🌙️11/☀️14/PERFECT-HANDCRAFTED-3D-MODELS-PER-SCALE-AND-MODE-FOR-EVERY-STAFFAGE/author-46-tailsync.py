"""Usage: python3 author-46-tailsync.py <slug> <asset> — sets the plan ellipses of the tail shapes (cx, rx) to the x extent of the elevation shape of the same id (after a rig change of the tail)."""
import glob, os, re, subprocess, sys
slug, asset = sys.argv[1:3]
bun = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
for sc, pairs in (("50", ("tail-root", "tail-curl")), ("100", ("tail",))):
    out = subprocess.run([bun, "author-25-probe.ts", asset, "--scale", sc, "--bbox"], capture_output=True, text=True).stdout
    for sid in pairs:
        m = re.search(r"elevation\s+%s\s+\S+\s+x (\S+) (\S+)" % sid, out)
        x0, x1 = float(m.group(1)), float(m.group(2))
        subprocess.run(["python3", "author-46-ellipse.py", slug, sc, "plan", sid, "cx=%.4f" % ((x0 + x1) / 2), "rx=%.4f" % ((x1 - x0) / 2), "--apply"], capture_output=True)
        print(sc, sid, x0, x1)
