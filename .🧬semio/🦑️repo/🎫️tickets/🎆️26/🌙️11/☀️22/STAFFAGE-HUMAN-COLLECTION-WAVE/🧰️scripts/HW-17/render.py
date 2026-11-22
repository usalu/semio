"""🖨️ Renders preview sheets (elevation, front, plan, optional axonometric) of a human through the preview CLI and rasterises them with sharp.
python3 render.py <slug> <scale,...> [--pose p | --clip c --time t] [--view light-solid] [--axo] [--dpi 200]"""
import argparse, glob, os, subprocess
from hw17lib import *
ap = argparse.ArgumentParser()
ap.add_argument("slug"); ap.add_argument("scales")
ap.add_argument("--pose"); ap.add_argument("--clip"); ap.add_argument("--time")
ap.add_argument("--view", default="light-solid"); ap.add_argument("--axo", action="store_true"); ap.add_argument("--dpi", default="200")
o = ap.parse_args()
out = f"{GEN}/{o.slug}"
os.makedirs(out, exist_ok=True)
for sc in o.scales.split(","):
    args = [BUN, "🔨️modules/🎨️depiction/🖨️preview/🟦️.ts", "--asset", o.slug, "--scale", sc, "--views", "elevation,front,plan" + (",axonometric" if o.axo else ""), "--projection", "third", "--view", o.view, "--language", "en", "--no-png", "--out", out]
    if o.pose: args += ["--pose", o.pose]
    if o.clip: args += ["--clip", o.clip, "--time", o.time]
    r = subprocess.run(args, cwd=S, capture_output=True, text=True)
    if r.returncode != 0:
        print(r.stdout[-2000:], r.stderr[-2000:])
svgs = sorted(glob.glob(f"{out}/*.svg"), key=os.path.getmtime)[-len(o.scales.split(",")):]
r = subprocess.run([BUN, f"{SCRIPTS}/png.ts", o.dpi, *svgs], cwd=SCRIPTS, capture_output=True, text=True)
print(r.stdout, r.stderr[-1500:])
