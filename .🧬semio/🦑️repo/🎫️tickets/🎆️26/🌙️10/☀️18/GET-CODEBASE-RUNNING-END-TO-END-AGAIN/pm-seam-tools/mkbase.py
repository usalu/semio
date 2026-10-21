#!/usr/bin/env python3
"""🧱️ Cuts the neutral schema bases (install schema, generation schema) from the live originals by replacing every manager-specific value with a token or a `$manager` expression, proves materialize(base, bun) == original, and writes base + original fixtures into the mirror. usage: PM_MIRROR=R2 mkbase.py R1"""
import copy, json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import paths, pm_materialize as pm
r1 = sys.argv[1]
D = paths.DEPENDENCIES
bun = json.loads(paths.seam_read(paths.MANAGERS + "/🍞️bun/🔣️.json"))
original = {name: json.load(open(os.path.join(r1, D, rel), encoding="utf-8")) for name, rel in
            {"install": "💾️storage/🧬️schema/📥️install.json", "generation": "🧾️generation/🧬️schema/🔣️.json"}.items()}
ROLE = {"bun-cache": "{{recipe.cache}}", "bun-global": "{{recipe.global}}", "bun-bin": "{{recipe.bin}}", "bun-home": "{{recipe.home}}", "bun-config": "{{recipe.config}}",
        "bun.lock": "{{lockFile}}", "bunfig.toml": "{{scopeConfig}}", "j": "{{generationDirectory}}"}
def roles(node):
    if isinstance(node, list):
        return [roles(x) for x in node]
    if isinstance(node, str):
        return ROLE.get(node, node)
    return node
def build_install(doc):
    doc = copy.deepcopy(doc)
    s = doc["$defs"]
    assert "Bun" in doc["description"]
    doc["description"] = doc["description"].replace("Bun", "{{label}}")
    ex = s["SourceOwnerExportsProtocol"]["const"]
    ts = ex["profiles"]["typescript-bundler-import"]
    ex["profiles"] = {"$manager": "resolution.conditions", "extend": {"typescript-bundler-import": ts}}
    assert "bun" in ex["arrays"]
    ex["arrays"] = ex["arrays"].replace("bun", "{{id}}")
    case = s["SourceOwnerExportsCase"]["properties"]
    case["profile"]["enum"] = {"$manager": "resolution.profiles", "extend": ["typescript-bundler-import"]}
    case["oracle"]["enum"] = {"$manager": "resolutionOracles"}
    s["ProducerRecipeProtocol"]["const"] = {"$manager": "recipe"}
    s["ProducerDirectoryRoleCase"]["properties"]["name"]["enum"] = roles(s["ProducerDirectoryRoleCase"]["properties"]["name"]["enum"])
    refusal = s["DelayedCustody"]["properties"]["result"]["properties"]["refusal"]
    assert "Bun" in refusal["const"]
    refusal["const"] = refusal["const"].replace("Bun", "{{label}}")
    s["SelectedPlanProtocol"]["const"]["directoryRoles"] = roles(s["SelectedPlanProtocol"]["const"]["directoryRoles"])
    s["SelectedRetirementProtocol"]["const"]["fileRoles"] = roles(s["SelectedRetirementProtocol"]["const"]["fileRoles"])
    stage = s["ProducerStageProtocol"]["const"]
    stage["rootRole"] = roles(stage["rootRole"])
    stage["fileRoles"] = roles(stage["fileRoles"])
    pattern = s["ProducerStageFile"]["properties"]["path"]["pattern"]
    assert "bun\\.lock" in pattern and "bunfig\\.toml" in pattern
    s["ProducerStageFile"]["properties"]["path"]["pattern"] = pattern.replace("bun\\.lock", "{{lockFile|re}}").replace("bunfig\\.toml", "{{scopeConfig|re}}")
    s["SourceOwnerLockProtocol"]["const"]["mutableStagePath"] = "{{lockFile}}"
    s["ManagerProtocol"] = {"const": {"$manager": "managerProtocol"}}
    return doc
def build_generation(doc):
    doc = copy.deepcopy(doc)
    c = doc["$defs"]["Protocol"]["const"]
    p = c["producer"]
    for key in ("scopeFiles", "ambientHomeFiles", "ambientConfigFiles", "environmentPrefixes", "environmentIgnored"):
        p[key] = {"$manager": "producerInputs." + key}
    p["ambientConfigFallback"] = "{{producerInputs.ambientConfigFallback}}"
    p["storeRoleFiles"] = {"$manager": "storeRoleFiles"}
    covers = c["tiers"][1]
    assert covers["covers"].startswith("bun version")
    covers["covers"] = covers["covers"].replace("bun version", "{{id}} version", 1)
    covers = c["tiers"][2]
    assert covers["covers"].startswith("npmrc and bunfig files")
    covers["covers"] = covers["covers"].replace("npmrc and bunfig files", "{{producerFileLabel}} files", 1)
    runtime = doc["$defs"]["Runtime"]
    runtime["required"] = ["manager" if x == "bun" else x for x in runtime["required"]]
    runtime["properties"] = {("manager" if k == "bun" else k): v for k, v in runtime["properties"].items()}
    return doc
bases = {"install": build_install(original["install"]), "generation": build_generation(original["generation"])}
view = pm.schema_view(bun)
for name, base in bases.items():
    result = pm.materialize(base, view)
    expected = copy.deepcopy(original[name])
    if name == "generation":
        expected["$defs"]["Runtime"]["required"] = ["manager" if x == "bun" else x for x in expected["$defs"]["Runtime"]["required"]]
        expected["$defs"]["Runtime"]["properties"] = {("manager" if k == "bun" else k): v for k, v in expected["$defs"]["Runtime"]["properties"].items()}
    if name == "install":
        expected["$defs"]["ManagerProtocol"] = result["$defs"]["ManagerProtocol"]
    assert json.dumps(expected, sort_keys=True) == json.dumps(result, sort_keys=True), name + " base does not materialize to the original"
def write(rel, value):
    paths.seam_write(rel, json.dumps(value, ensure_ascii=False, indent=2) + "\n")
write(D + "/💾️storage/🧬️schema/📥️install.json", bases["install"])
write(D + "/🧾️generation/🧬️schema/🔣️.json", bases["generation"])
write(paths.MANAGERS + "/🧪️tests/🧫️fixtures/📥️install.original.json", original["install"])
write(paths.MANAGERS + "/🧪️tests/🧫️fixtures/🧾️generation.original.json", original["generation"])
print("bases proven against the originals")
