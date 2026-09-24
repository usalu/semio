import json, re, os
G = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf"
S = G + "/🏅️standards/🔖️2.0/🪆️subsets"
root = open(G + "/🦀️.rs", encoding="utf8").read()
mods = dict((m, p) for p, m in re.findall(r'#\[path = "([^"]+)"\]\s*pub mod (\w+);', root))
kinds = {}
for s in ["🎬️scene", "💿️buffer", "🕸️mesh"]:
    kinds[s] = json.load(open(f"{S}/{s}/🔮️oracles/🔣️.json"))["mutationCatalogs"][0]["kinds"]
fm = {m["mutation"]: m for m in json.load(open(f"{S}/♾️any/🔮️oracles/🔣️.json"))["fixtureManifests"]}
out = {}
for s, ks in kinds.items():
    for k in ks:
        mod = k.replace("-", "_")
        path = mods.get(mod)
        src = open(G + "/" + path, encoding="utf8").read() if path else ""
        structs = re.findall(r'pub struct (\w+Payload)\s*\{(.*?)\n\}', src, re.S)
        applysig = re.search(r'pub fn apply\((.*?)\)\s*->\s*([^{]+)\{', src, re.S)
        m = fm.get(k, {})
        files = {f["role"]: f["path"] for f in m.get("files", [])}
        out[k] = {"subset": s, "module": mod, "leaf": path, "payloads": [(n, b.strip()) for n, b in structs], "apply": applysig.group(1).strip() if applysig else None, "fixture": files, "notes": m.get("notes"), "pipeline": m.get("comparisonPipeline"), "profile": m.get("comparisonProfile")}
json.dump(out, open(".tmp-ticket/wp-t4/gltf/survey.json", "w"), ensure_ascii=False, indent=1)
missing = [k for k, v in out.items() if not v["leaf"] or not v["payloads"] or not v["fixture"]]
print(len(out), "missing", missing)
