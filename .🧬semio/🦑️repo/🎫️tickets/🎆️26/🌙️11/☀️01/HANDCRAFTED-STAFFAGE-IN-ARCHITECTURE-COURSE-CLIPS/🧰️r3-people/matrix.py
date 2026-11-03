import json
import os
import sys

sys.stdout.reconfigure(encoding="utf8")
root = sys.argv[1]
for cat in sorted(os.listdir(root)):
    if cat not in ("👤️humans", "🐕️animals"):
        continue
    for name in sorted(os.listdir(os.path.join(root, cat))):
        a = json.load(open(os.path.join(root, cat, name, "🔣️.json"), encoding="utf8"))["asset"]
        d = a["dimensions"]
        plan = {k: len(v["geometry"]["plan"]) for k, v in a["representations"].items() if v["status"] == "authored"}
        th = a["thermal"]
        em = [e["id"] for e in th["emitters"]]
        print(f"{name} | {d['heightM']}x{d['widthM']}x{d['depthM']} {d['massKg']}kg | poses {[p['id'] for p in a['poses'] if not p.get('key')]} | clips {[c['id'] for c in a['clips']]} | plan {plan} | emit {em} | noseE {th['respiration']['noseEmitter']} | anchors {[x['id'] for x in a['anchors'] if x['kind'] == 'emitter']}")
