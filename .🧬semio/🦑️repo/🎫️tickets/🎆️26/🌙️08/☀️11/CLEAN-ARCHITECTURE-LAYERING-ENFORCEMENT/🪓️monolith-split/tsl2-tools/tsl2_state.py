import sys, json, os, collections
R = "/Users/ueli/Documents/semio"
G = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
sp = G + "/tsl2-state.json"
st = json.load(open(sp, encoding="utf8")) if os.path.exists(sp) else {}
cmd = sys.argv[1]
if cmd == "merge":
    for label in sys.argv[2:]:
        for x in json.load(open(f"{G}/tsl2-{label}.json", encoding="utf8")):
            n = collections.Counter(s for s, _ in x["rows"])
            st[x["file"]] = {"rc": x["rc"], "passed": n.get("passed", 0), "failed": n.get("failed", 0), "skipped": n.get("skipped", 0), "label": label, "key": x["key"]}
    json.dump(st, open(sp, "w", encoding="utf8"), ensure_ascii=False)
inscope = set(l for l in open(G + "/tsl2-inscope.txt", encoding="utf8").read().split("\n") if l)
rows = [(f, v) for f, v in st.items() if f in inscope]
bad = [(f, v) for f, v in rows if v["rc"] != 0]
print("files", len(rows), "green", len(rows) - len(bad), "red", len(bad), "tests passed", sum(v["passed"] for _, v in rows), "failed", sum(v["failed"] for _, v in rows), "rc-killed", sum(1 for _, v in bad if v["rc"] < 0))
if cmd == "list":
    for f, v in sorted(bad): print(v["rc"], v["failed"], v["passed"], v["label"], f.replace("🧰️framework/", ""))
