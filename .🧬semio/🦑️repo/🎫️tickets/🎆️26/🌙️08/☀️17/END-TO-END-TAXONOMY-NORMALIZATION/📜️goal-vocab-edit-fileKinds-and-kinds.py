import json, collections

F = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
d = json.load(open(F, encoding="utf-8"))

# ---- A. fileKinds: fix svg emoji, split model-3d ----
fk = d["fileKinds"]
assert fk["svg"]["emoji"] == "🎨️"
fk["svg"]["emoji"] = "🔣️"

old_model = fk.pop("model-3d")
assert set(old_model["extensionChains"]) == {".3dm", ".glb", ".gltf", ".obj", ".ply", ".stl", ".dxf", ".dwg", ".ifc", ".las", ".stp", ".step"}
fk["cad-source-model"] = {"emoji": "📐️", "extensionChains": [".3dm", ".stp", ".step"], "role": "asset"}
fk["mesh-model"] = {"emoji": "🧊️", "extensionChains": [".glb", ".gltf", ".obj", ".stl", ".ply", ".las"], "role": "asset"}
fk["drawing-2d-model"] = {"emoji": "🖊️", "extensionChains": [".dxf", ".dwg"], "role": "asset"}
fk["building-model"] = {"emoji": "🏗️", "extensionChains": [".ifc"], "role": "asset"}

# sanity: no extension lost or duplicated across the whole fileKinds map
all_exts = collections.Counter()
for k, v in fk.items():
    for e in v["extensionChains"]:
        all_exts[e] += 1
dupes = {e: c for e, c in all_exts.items() if c > 1}
assert not dupes, dupes

# ---- B. semanticDirectoryKinds ----
sd = d["semanticDirectoryKinds"]

# widen asset-subject to accept underscore + mixed case content names (metabolism assets, icon variants)
assert sd["asset-subject"]["slugPattern"] == "^(?!assets$)[a-z0-9]+(?:-[a-z0-9]+)*$"
sd["asset-subject"]["slugPattern"] = "^(?!assets$)[A-Za-z0-9]+(?:[-_][A-Za-z0-9]+)*$"

new_words = {
    "activation": "🪪️", "assembly": "🎟️", "authority": "🪪️", "base64": "🔤️",
    "binding": "🔗️", "bindings": "🔗️", "bootstrap": "🏗️", "budget": "⏱️",
    "bytes": "🔢️", "cancellation": "🚫️", "causal-add": "➕️", "clock": "⏱️",
    "commit": "🔗️", "compare": "⚖️", "composition": "🏘️", "content": "📦️",
    "cooperative": "⏱️", "copied": "📋️", "copy": "📋️", "credit": "🎟️",
    "enqueue": "📥️", "entries": "📚️", "evidence": "🧾️", "fault": "🧯️",
    "fixed": "🗃️", "fonts": "🗚️", "framing": "📄️", "graph": "🔬️",
    "hash": "#⃣", "inbound": "📨️", "inbox": "📥️", "index": "🗂️",
    "input": "📥️", "json": "🧩️", "lifetime": "🚪️", "list": "📋️",
    "local-interaction": "🏠️", "message": "💌️",
    "mutation-leaf-contract": "🧬️", "mutation-leaf-source-contract": "🧬️",
    "nodes": "🗂️", "numeric": "🔢️", "operations": "🩹️", "ordered": "🗂️",
    "ownership": "📏️", "pack": "🧩️", "page": "📄️", "pages": "📄️",
    "payload": "📦️", "pending": "📨️", "poll": "📥️", "read-lease": "📖️",
    "reader": "📖️", "release": "🧾️", "resident": "🎟️", "response": "📨️",
    "retirement": "♻️", "return": "📤️", "set": "🧺️", "slot": "📨️",
    "source": "🏠️", "string": "🔤️", "tail": "🏁️", "transaction": "🔄️",
    "tutorial": "🎬️", "update": "🩹️", "validation": "🛡️", "value": "🧾️",
    "whole": "📄️", "wire-retirement": "🧹️", "writer": "✍️",
}
for word, emoji in new_words.items():
    assert word not in sd, word
    sd[word] = {"emoji": emoji, "slugPattern": f"^{word}$", "allowEmojiOnly": False}

# extend test-case's allowed direct-parent contexts to include every new domain-module
# directory kind, so a co-located test file (🧪️<name>) living directly inside one of
# these ordinary module directories resolves to test-case instead of colliding with
# test-fixture-member (semantic-stem-ambiguous).
tc = sd["test-case"]
assert tc["parentKindIds"] == ["tests", "mutation-test-subject"]
tc["parentKindIds"] = tc["parentKindIds"] + sorted(new_words.keys()) + ["builder"]

# ---- C. fixedFilenameContracts: tool-mandated config leaves ----
ff = d["fixedFilenameContracts"]
def add_contract(id_, pattern, authority, reason, verification):
    assert id_ not in ff, id_
    ff[id_] = {
        "pathPattern": pattern,
        "authority": authority,
        "reason": reason,
        "configurability": "unconfigurable",
        "scope": {"kind": "package-root", "ecosystemId": "🟦️typescript"},
        "verification": verification,
        "expires": None,
    }

add_contract("vitest-config", "**/vitest.config.ts", "Vitest", "Vitest configuration file discovery", "vitest --config")
add_contract("tailwind-config", "**/tailwind.config.ts", "Tailwind CSS", "Tailwind CSS configuration discovery", "tailwindcss --config")
add_contract("postcss-config", "**/postcss.config.ts", "PostCSS", "PostCSS configuration discovery", "postcss --config")
add_contract("eslint-config", "**/eslint.config.ts", "ESLint", "ESLint flat configuration discovery", "eslint --config")

if ".dependency-cruiser.cjs" not in json.dumps(ff):
    ff["dependency-cruiser-config"] = {
        "pathPattern": ".dependency-cruiser.cjs",
        "authority": "dependency-cruiser",
        "reason": "dependency-cruiser configuration discovery",
        "configurability": "unconfigurable",
        "scope": {"kind": "repository-root"},
        "verification": "depcruise --config",
        "expires": None,
    }

out = json.dumps(d, indent=2, ensure_ascii=False) + "\n"
open(F, "w", encoding="utf-8").write(out)
print("wrote", len(out), "bytes")
print("new semanticDirectoryKinds count:", len(sd))
print("new fixedFilenameContracts count:", len(ff))
print("new fileKinds count:", len(fk))
