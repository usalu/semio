"""Reviewer 3 pass: runs areaaudit.py, snapshots areaM2, areaBasis and skinFraction of every region of the five categories (🗑️generated/areareview3/snapshot.json) and lists what changed since the previous pass
(changed value, new region, new volumes in the asset file or in volumes/), then appends the pass to areareview3-passes.json. Usage: python3 areareview3-pass.py [--init]"""
import glob, json, os, subprocess, sys, time
T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *[".."] * 7))
ROOT = os.path.join(REPO, "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
CATS = ["vegetation", "animals", "humans", "appliances", "furniture"]
au = subprocess.run(["python3", os.path.join(T, "areaaudit.py")], capture_output=True, text=True, cwd=T).stdout.strip().split("\n")[-1]
rows = int(au.split("areas=")[1].split()[0]) if "areas=" in au else 0
snap, now = {}, {}
sp = os.path.join(T, "🗑️generated", "areareview3", "snapshot.json")
if os.path.exists(sp): snap = json.load(open(sp))
for c in CATS:
    for p in sorted(glob.glob(f"{ROOT}/*{c}/*/🔣️.json")):
        slug = p.split("/")[-2]
        a = json.load(open(p))["asset"]
        vol = bool(a.get("volumes")) or os.path.exists(next(iter(glob.glob(os.path.join(T, "volumes", "*", slug + ".json"))), "/nonexistent"))
        now[slug] = {"_volumes": vol, "_mtime": int(os.path.getmtime(p))}
        for r in a["thermal"]["regions"]:
            now[slug][r["id"]] = [r["areaM2"], r.get("areaBasis"), (r.get("body") or {}).get("skinFraction")]
new = []
for slug, regs in now.items():
    old = snap.get(slug)
    if old is None:
        new.append(f"{slug}: asset new in the snapshot"); continue
    if regs["_volumes"] and not old["_volumes"]:
        new.append(f"{slug}: volumes appeared")
    for rid, v in regs.items():
        if rid.startswith("_"): continue
        if rid not in old: new.append(f"{slug}/{rid}: region new {v}")
        elif old[rid] != v: new.append(f"{slug}/{rid}: {old[rid]} -> {v}")
init = "--init" in sys.argv or not snap
if not init:
    pp = os.path.join(T, "areareview3-passes.json")
    ps = json.load(open(pp)) if os.path.exists(pp) else []
    ps.append({"time": time.strftime("%Y-%m-%d %H:%M"), "rows": rows, "new": len(new), "decided": 0, "note": "; ".join(new[:6]) + (" ..." if len(new) > 6 else "") if new else "nothing new"})
    json.dump(ps, open(pp, "w"), ensure_ascii=False)
json.dump(now, open(sp, "w"), ensure_ascii=False)
print("audit", au, "| new:", len(new))
for x in new: print(" ", x)
