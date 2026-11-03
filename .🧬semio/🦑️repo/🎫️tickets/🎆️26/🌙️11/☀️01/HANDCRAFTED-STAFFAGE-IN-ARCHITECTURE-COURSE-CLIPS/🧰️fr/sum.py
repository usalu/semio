import glob, json, os, sys

T = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
WATCH = set(open(os.path.join(T, "🧰️fr", "casts.txt"), encoding="utf8").read().split())
for path in sorted(glob.glob(os.path.join(T, "🗑️generated", "fr", "*.probe.json"))):
    scene = os.path.basename(path).split(".")[0]
    for e in json.load(open(path, encoding="utf8")):
        cast = e["cast"].split("/")[-1]
        if cast in WATCH:
            print(f"{scene:34s} {cast:46s} wall {e['wall']:6.1f}s rate_end {e['rate']:6.1f} elapsed {e['elapsed']:7.1f}/{e['dur']:.0f} ({100*e['elapsed']/e['dur']:.0f}%) loop {e['loops']} inst {e['ids']}")
