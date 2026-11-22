"""🧾️ Collects the measured proof lines of the people into proof.json (read by report.py and the finisher's report): validator count and check CLI lines from 🗑️generated/FIN-A/[DEBUG]-validate-<slug>.log and [DEBUG]-check-<slug>.log (measure.sh), silhouette guard and checkVolumes from 🗑️generated/FIN-A/gate/<slug>.log (gate.sh).
python3 build_proof.py"""
import json
import os
import re
import time

T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE"
GEN = T + "/🗑️generated/FIN-A"
HERE = T + "/🧰️scripts/FIN-A"
FEATURE = {"50": 7.5, "100": 15, "200": 30, "500": 75}
proof = {}
for slug in json.load(open(HERE + "/distinctions.json", encoding="utf-8")):
    v, c, g = (f"{GEN}/[DEBUG]-validate-{slug}.log", f"{GEN}/[DEBUG]-check-{slug}.log", f"{GEN}/gate/{slug}.log")
    check = open(c, encoding="utf-8").read().splitlines()
    scales = {m.group(1): m.group(2) for line in check for m in [re.match(r"\s+1:(\d+) authored, (.*)", line)] if m}
    rules, sil, worst_iou, haus, bbox = {}, [], 1.0, 0, 0
    for line in open(g, encoding="utf-8").read().splitlines():
        m = re.match(r"\s+(volumes-[a-z-]+) ", line)
        if m:
            rules[m.group(1)] = rules.get(m.group(1), 0) + 1
        m = re.match(r"\S+ 1:(\d+) bbox (\S+) (\S+) (\S+)\s+elevation ([\d.]+)/(\d+)mm!?\s+front ([\d.]+)/(\d+)mm!?\s+plan ([\d.]+)/(\d+)mm", line)
        if m:
            s = m.group(1)
            bbox += sum(1 for k in (2, 3, 4) if m.group(k).endswith("!"))
            ious = [float(m.group(k)) for k in (5, 7, 9)]
            dist = [int(m.group(k)) for k in (6, 8, 10)]
            worst_iou = min(worst_iou, *ious)
            haus += sum(1 for d in dist if d > 2 * FEATURE[s])
            sil.append(f"1:{s} IoU {'/'.join(f'{i:.3f}' for i in ious)}, distance {'/'.join(str(d) for d in dist)} mm")
    low = sum(1 for s in sil for i in re.findall(r"0\.\d{3}", s.split(",")[0]) if float(i) < 0.92)
    gate = f"bounding box {'within one feature at every scale' if bbox == 0 else f'{bbox} axis values off by more than one feature'}; IoU at least {worst_iou:.3f} ({'all 12 views at or above 0.92' if low == 0 else f'{low} of 12 views below 0.92'}); largest boundary distance above two features in {haus} of 12 views; checkVolumes {sum(rules.values())} findings" + (" (" + ", ".join(f"{n} {r}" for r, n in sorted(rules.items())) + ")" if rules else "") + ". Side/front/top per scale: " + "; ".join(sil)
    proof[slug] = {"at": time.strftime("%Y-%m-%d %H:%M", time.localtime(os.path.getmtime(c))), "validate": int(open(v, encoding="utf-8").readline().split("\t")[1]), "check": check[0].strip(), "scales": scales, "gate": gate,
                   "numbers": {"bbox": bbox, "lowIou": low, "worstIou": worst_iou, "hausdorff": haus, "volumeFindings": sum(rules.values()), "rules": rules}}
json.dump(proof, open(HERE + "/proof.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
for slug, p in proof.items():
    print(slug, p["validate"], p["check"], p["numbers"])
