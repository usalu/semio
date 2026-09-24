"""🔎️ Finds leaf descriptors and Rust/TS sites that still speak the severity outcome vocabulary outside the T10 switch's reach."""
import json, os, re, sys
root = "/Users/ueli/Documents/semio/"
table = json.load(open(root + ".tmp-ticket/wp-t10/leaf-outcomes.json", encoding="utf-8"))
tabled = {r + "/🔣️.json" for r in table}
hand = json.load(open(root + ".tmp-ticket/wp-t10/hand-descriptors.json", encoding="utf-8"))
OLD = {"info", "warning", "error", "fatal"}
leaf_foreign, rs_sites, ts_sites = [], [], []
for base in ("✏️s", "🧰️framework", "🌐️hub", "🧪️test"):
    if not os.path.isdir(root + base): continue
    for dp, ds, fs in os.walk(root + base):
        ds[:] = [d for d in ds if d not in ("node_modules", "target", "dist", "🗑️generated", ".git")]
        for f in fs:
            p = os.path.join(dp, f); rel = p[len(root):]
            if f == "🔣️.json":
                t = open(p, encoding="utf-8").read()
                if '"outcomeClasses"' not in t: continue
                for m in re.finditer(r'"outcomeClasses"\s*:\s*\[([^\]]*)\]', t):
                    vals = set(re.findall(r'"([a-z-]+)"', m.group(1)))
                    if vals & OLD and rel not in tabled: leaf_foreign.append((rel, sorted(vals)))
            elif f == "🦀️.rs":
                t = open(p, encoding="utf-8").read()
                if re.search(r"MutationOutcomeClass::(Info|Warning|Error|Fatal)\b", t) and rel not in hand: rs_sites.append(rel)
                if "protocol_outcomes" in t and "🏭️bridge" not in rel: rs_sites.append(rel + " (protocol_outcomes)")
            elif f.endswith(".ts") and "outcomeClasses" in (t := open(p, encoding="utf-8").read()):
                if re.search(r'"(info|warning|fatal)"', t): ts_sites.append(rel)
print("leaf descriptors outside the table with severity values:", len(leaf_foreign))
for r in leaf_foreign: print("  LEAF", r)
print("rust sites outside hand-descriptors:", len(rs_sites))
for r in rs_sites: print("  RS", r)
print("ts sites mentioning outcomeClasses + severity strings:", len(ts_sites))
for r in ts_sites: print("  TS", r)
