"""🌐 Cross-workspace edge share of the corrected graph (workspace of a crate = the nested workspace listing it as a member, else the root). Run with `cwd` = the generated folder."""
import json, os, collections
g = json.load(open("crates2.json"))
ws = {}
for e, mem in g["nested_members"].items():
    for m in mem:
        ws[os.path.normpath(os.path.join(e, m))] = e
for e in g["nested"]:
    ws.setdefault(e, e)
tot = cross = 0
into = collections.Counter()
FW = "🧰️framework"
for d, c in g["crates"].items():
    for k in ("normal", "build"):
        for _, t in c["deps"][k]:
            if t in g["crates"]:
                tot += 1
                a, b = ws.get(d, "."), ws.get(t, ".")
                if a != b:
                    cross += 1
                    into[b] += 1
print("edges", tot, "cross-workspace", cross, round(100 * cross / tot, 1), "%")
print("into framework workspace", into[FW], "into root workspace", into["."], "distinct target workspaces", len(into))
print("workspaces", len(set(ws.values()) | {"."}), "crates per workspace (framework, root):", sum(1 for v in ws.values() if v == FW), sum(1 for d in g["crates"] if ws.get(d, ".") == "."))
