import json, sys
R = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🪑️objects"
IDS = "retractable-bollard shopping-trolley stepladder storage-crate-stack street-lamp tealight waste-bin".split()
out = {}
for a in IDS:
    d = json.load(open(R + "\\" + a + "\🔣️.json", encoding="utf-8"))["asset"]
    sc = [k for k, v in d["representations"].items() if v["status"] == "authored"]
    whens = ["pose:" + p["id"] for p in d["poses"]]
    for c in d["clips"]:
        whens += ["clip:%s:%s" % (c["id"], round(c["durationS"] * f, 3)) for f in (0.0, 0.5, 0.999)]
    out[a] = {"scales": sc, "whens": whens}
json.dump(out, sys.stdout, ensure_ascii=False)
