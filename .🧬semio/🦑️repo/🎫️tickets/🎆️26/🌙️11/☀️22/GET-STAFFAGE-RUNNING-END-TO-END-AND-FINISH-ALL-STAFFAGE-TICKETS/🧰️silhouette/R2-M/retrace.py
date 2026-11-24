"""🔁️ Redraws, in one view of one scale, the drawn shapes that `diff.ts` finds fuller than the body and draws the solids it finds missing, each as its solids project (`trace.ts`), only on the bones named as rigid in the views (a flat front or plan shape on a bone that poses turn is not written): python3 retrace.py <id> <scale> <view> <bone,bone…> [--min squareFeatures] [--skip id,id…]
A shape `x` is redrawn from the solids `x` or, when there is none, from the solids of the same bone whose id starts with `x` or with which `x` starts; a missing solid replaces the shape of its name or is added under the first shape of the view. The ops are merged into `ops/<id>.json`."""
import glob, json, os, re, subprocess, sys

ident, scale, view, bones = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4].split(",")
args = sys.argv[5:]
least = args[args.index("--min") + 1] if "--min" in args else "1"
skip = args[args.index("--skip") + 1].split(",") if "--skip" in args else []
here = os.path.dirname(os.path.abspath(__file__))
bun = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
ops = f"{here}/ops/{ident}.json"
if not os.path.exists(ops):
    open(ops, "w").write("[]\n")
asset = json.load(open(glob.glob(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/{ident}/🔣️.json")[0]))["asset"]
solids = {s["id"]: s["bone"] for s in asset["volumes"][scale]["body"]}
out = subprocess.run([bun, f"{here}/diff.ts", ident, scale, view, "--ops", ops, "--min", least, "--count", "200"], capture_output=True, text=True).stdout
jobs, kind = {}, ""
for line in out.splitlines():
    if line.startswith("=="):
        kind = "excess" if " excess" in line else "missing"
        continue
    drawn = re.findall(r"([\w-]+)\[([\w-]+)(?:,outline)?\]", line.split("drawn:")[1].split("| vol:")[0])
    vol = re.findall(r"([\w-]+)\[([\w-]+)(?:,slender)?\]", line.split("| vol:")[1])
    if kind == "excess" and drawn:
        sid, bone = drawn[-1]
        names = [sid] if solids.get(sid) == bone else [n for n, b in solids.items() if b == bone and (n.startswith(sid) or sid.startswith(n))]
        if bone in bones and names and sid not in skip:
            jobs[sid] = (bone, names, True)
    if kind == "missing" and vol:
        sid, bone = vol[0]
        if bone in bones and sid not in skip:
            jobs.setdefault(sid, (bone, [sid], False))
shapes = lambda: [s["id"] for s in asset["representations"][scale]["geometry"][view]] + [o["shape"]["id"] for o in json.load(open(ops)) if o["op"] == "shape-add" and o["scale"] == scale and o["view"] == view]
first = asset["representations"][scale]["geometry"][view][0]["id"]
for sid, (bone, names, replace) in jobs.items():
    has = sid in shapes()
    cmd = [bun, f"{here}/trace.ts", ident, scale, view, ",".join(names), "--id", sid, "--bone", bone, "--like", sid if has else first, "--holes"]
    cmd += ["--replace", sid] if has else ["--after", first, "--z", str(min(s.get("z", 0) for s in asset["representations"][scale]["geometry"][view]) - 0.05), "--tone", str(next(s for s in asset["volumes"][scale]["body"] if s["id"] == sid).get("tone", 0.4)), "--role", "body"]
    r = subprocess.run(cmd, capture_output=True, text=True)
    print((r.stdout.strip().splitlines() or [r.stderr.strip()[-160:]])[-1][:150])
print(f"{ident} 1:{scale} {view}: {len(jobs)} shapes redrawn")
