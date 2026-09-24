"""🧭️ Surveys the 39 editor-layer mutation vocabularies (T11 §1) for registration: owner surface, the document
manifest's artifact/standard/subset, each leaf's kind/variant/outcomes/payload fields, the aggregate and snapshot
paths, the snapshot struct source and any committed vectors. Writes `wp-t12/generated/editor-survey.json`."""
import json, os, re, glob
root = "/Users/ueli/Documents/semio/"
paths = json.load(open(root + ".tmp-ticket/wp-t11/generated/editor-paths-0.json", encoding="utf-8"))
out = []
for v in paths:
    owner = v["owner"]
    m = re.match(r"(.*?/🪆️subsets/[^/]+)/(.*)$", owner)
    subset_dir, surface = m.group(1), m.group(2)
    doc = json.load(open(root + subset_dir + "/🔮️oracles/🔣️.json", encoding="utf-8")) if os.path.exists(root + subset_dir + "/🔮️oracles/🔣️.json") else {}
    man = (doc.get("mutationManifests") or [{}])[0]
    leaves = []
    mroot = root + owner + "/🧬️schema/🧬️mutations"
    for d in sorted(os.listdir(mroot)) if os.path.isdir(mroot) else []:
        f = os.path.join(mroot, d, "🔣️.json")
        if not os.path.exists(f): continue
        j = json.load(open(f, encoding="utf-8"))
        src = open(os.path.join(mroot, d, "🦀️.rs"), encoding="utf-8").read() if os.path.exists(os.path.join(mroot, d, "🦀️.rs")) else ""
        s = re.search(r"pub struct (\w+)\s*(\{.*?\n\}|;|\(.*?\);)", src, re.S)
        leaves.append({"dir": d, "kind": j["semanticKind"], "variant": j["aggregateVariant"], "outcomes": j["outcomeClasses"], "struct": s.group(0) if s else None})
    out.append({**v, "subsetDir": subset_dir, "surface": surface, "artifact": man.get("artifact"), "standard": man.get("standard"), "subset": man.get("subset"), "leaves": leaves})
json.dump(out, open(root + ".tmp-ticket/wp-t12/generated/editor-survey.json", "w"), ensure_ascii=False, indent=1)
for x in out: print(x["artifact"], x["standard"], x["subset"], x["surface"], [l["kind"] for l in x["leaves"]])
