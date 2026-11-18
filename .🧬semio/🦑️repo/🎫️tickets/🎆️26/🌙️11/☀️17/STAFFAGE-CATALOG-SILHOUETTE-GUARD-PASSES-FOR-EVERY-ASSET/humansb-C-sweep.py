"""🔁 Sweep: python3 humansb-C-sweep.py <asset> <base.json> <scale> <template-json-with-{v}> v1 v2 ... : applies base ops plus the template ops per value and prints the guard line of the scale."""
import json, subprocess, sys, os
asset, base, scale, tpl, *vals = sys.argv[1:]
H = os.path.dirname(os.path.abspath(__file__))
T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET/humansb-C-try.ts"
B = "/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
CWD = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript"
EXTRA = os.environ.get('SWEEP_EXTRA', '')
ops = json.load(open(base)) if base != "-" else []
for v in vals:
    o = ops + json.loads(open(tpl).read().replace("{v}", v))
    p = H + "/🗑️generated/humansb-C-sweep.json"
    json.dump(o, open(p, "w"))
    r = subprocess.run([B, T, asset, p + (',' + EXTRA if EXTRA else ''), scale], cwd=CWD, capture_output=True, text=True)
    print(v, r.stdout.strip().split("\n")[-1], r.stderr[-200:])
