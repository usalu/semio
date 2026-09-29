"""📋️ FH1 family A: every framework-raised code with its parameter sets, sites and the developer message of each raise
(the `Fault::new(origin, FaultCode::new("code"), message)` third argument) — the source for the handcrafted catalog."""
import json, re, sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1")
O = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults/"
d = json.load(open("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-fh1-census/family-A.json"))
from importlib import util
spec = util.spec_from_file_location("raise", "/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1/fh1-raise.py")
texts = {}
codes = {}
for r in d["raises"]:
    c = codes.setdefault(r["code"], {"params": set(), "sites": [], "messages": []})
    c["params"].add(",".join(sorted(r["parameters"])))
    c["sites"].append(f'{r["path"].replace("🧰️framework/🛍️products/💻️os/🔨️modules/", "")}:{r["line"]}')
    if r["path"] not in texts:
        texts[r["path"]] = open(O + r["path"]).read().split("\n")
    line = texts[r["path"]][r["line"] - 1]
    window = " ".join(t.strip() for t in texts[r["path"]][r["line"] - 1:r["line"] + 3])
    m = re.search(r'FaultCode::new\(\s*"' + re.escape(r["code"]) + r'"\s*\)\s*,\s*(.{0,200})', window)
    if m:
        c["messages"].append(m.group(1)[:200])
out = []
for code in sorted(codes):
    c = codes[code]
    out.append({"code": code, "params": sorted(c["params"]), "n": len(c["sites"]), "sites": c["sites"][:3], "messages": list(dict.fromkeys(c["messages"]))[:3]})
json.dump(out, open("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-fh1-census/a-codes.json", "w"), ensure_ascii=False, indent=1)
print(len(out), "codes;", sum(1 for o in out if o["params"] != [""]), "with parameters;", sum(1 for o in out if len(o["params"]) > 1), "with conflicting parameter sets")
