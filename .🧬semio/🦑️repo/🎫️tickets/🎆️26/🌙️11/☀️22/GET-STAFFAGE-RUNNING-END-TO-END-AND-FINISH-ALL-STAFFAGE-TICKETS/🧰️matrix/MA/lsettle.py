"""⚖️ Keeps of the loft ops of an asset those the silhouette guard and the validator accept: `BATCH=MA source ../MR/env.sh; python3 lsettle.py <id>…` builds the trial copy `$GEN/loft/<id>.json` from the base (`LBASE` or the backup) and `loft/<id>.json` (then `loft/<id>.lofts.json` by `../MX-2/loft.ts` when it exists), measures it per scale (`../MR/measure.ts --scales`), and while a scale has a failing guard check or a validation issue other than `volumes-feature`, takes the ops of one bone group of that scale out (trunk first, then neck, head, tail, then the limbs from the body outwards; ORDER="a b" names groups to try first) and keeps the removal when the count of failures of that scale drops, or at an equal count the failing distances shrink by more than a millimetre. The full list stays in `loft/<id>.all.json`; the ops taken out are printed. Nothing of the catalog is written."""
import json, os, pathlib, re, shutil, subprocess, sys
W = pathlib.Path(__file__).resolve().parent
B, S, T, M, GEN = (os.environ[k] for k in ("B", "S", "T", "M", "GEN"))
SIDE = {"n", "f", "near", "far", "l", "r", "hn", "hf", "fn", "ff"}
FIRST = ["pelvis", "lumbar", "thorax", "trunk", "chest", "neck", "head", "jaw", "tail"]


def group(bone: str) -> str:
    return "-".join(t for t in bone.split("-") if t not in SIDE and not t.isdigit())


def dump(path, ops):
    pathlib.Path(path).write_text("[\n" + ",\n".join(" " + json.dumps(op, ensure_ascii=False) for op in ops) + "\n]\n")


def build(a, ops):
    copy = f"{GEN}/loft/{a}.json"
    shutil.copyfile(f"{os.environ.get('LBASE', GEN + '/backup')}/{a}.json", copy)
    dump(f"{GEN}/loft/{a}.settle.ops", ops)
    out = subprocess.run([B, f"{W}/loft.ts", copy, "--ops", f"{GEN}/loft/{a}.settle.ops", "--write"], cwd=T, capture_output=True, text=True).stdout
    if (W / "loft" / f"{a}.lofts.json").exists():
        subprocess.run([B, f"{M}/../MX-2/loft.ts", copy, "--ops", str(W / "loft" / f"{a}.lofts.json"), "--write"], cwd=f"{S}/📦️packages/🟦️typescript", capture_output=True, text=True)
    return out


def measure(a, scale):
    out = subprocess.run([B, f"{M}/measure.ts", f"{GEN}/loft/{a}.json", "--scales", scale], cwd=f"{S}/📦️packages/🟦️typescript", capture_output=True, text=True).stdout
    line = next((l for l in out.split("\n") if f" 1:{scale} bbox" in l), "")
    issues = [l for l in out.split("\n") if l.startswith("   ") and "volumes-feature" not in l and (f".{scale}." in l)]
    total = re.search(r"validate: (\d+) issues (.*)", out)
    other = sum(int(n) for code, n in re.findall(r"([\w-]+)×(\d+)", total.group(2) if total else "") if code != "volumes-feature")
    flagged = sum(float(mm) for mm in re.findall(r"/([\d.]+)mm!", line)) + sum(1000 * (0.92 - float(iou)) for iou in re.findall(r" ([\d.]+)!/", line))
    return line.count("!") + (len(issues) if issues else 0) + flagged / 1e4, other, line.strip(), issues


for a in sys.argv[1:]:
    path = W / "loft" / f"{a}.json"
    if not (W / "loft" / f"{a}.all.json").exists():
        shutil.copyfile(path, W / "loft" / f"{a}.all.json")
    ops = json.load(open(path))
    scales = sorted({op["scale"] for op in ops if "scale" in op}, key=int)
    out = []
    for scale in scales:
        build(a, ops)
        bad, other, line, issues = measure(a, scale)
        print(f"{a} 1:{scale} start {bad:.4f} failing, {other} other issues in all scales | {line[:150]}", flush=True)
        if bad < 1e-9:
            continue
        groups = []
        for op in ops:
            if op.get("scale") == scale and op.get("bone") and group(op["bone"]) not in groups:
                groups.append(group(op["bone"]))
        order = os.environ.get("ORDER", "").split() + FIRST
        groups.sort(key=lambda g: order.index(g) if g in order else len(order))
        for g in groups[: int(os.environ.get("TRIES", "8"))]:
            trial = [op for op in ops if not (op.get("scale") == scale and op.get("bone") and group(op["bone"]) == g)]
            build(a, trial)
            now, _, line, _ = measure(a, scale)
            print(f"{a} 1:{scale} without {g}: {now:.4f} failing | {line[:150]}", flush=True)
            if now < bad - 1e-4:
                out += [op for op in ops if op not in trial]
                ops, bad = trial, now
            if bad < 1e-9:
                break
    dump(path, ops)
    print(build(a, ops).strip().split("\n")[-2:])
    print(f"{a}: {len(out)} ops taken out: " + ", ".join(sorted({f"1:{op['scale']} {group(op['bone'])}" for op in out})), flush=True)
