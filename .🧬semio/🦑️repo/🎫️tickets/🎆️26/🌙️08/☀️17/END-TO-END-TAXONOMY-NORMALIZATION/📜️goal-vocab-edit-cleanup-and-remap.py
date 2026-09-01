import json

F = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
d = json.load(open(F, encoding="utf-8"))

sd = d["semanticDirectoryKinds"]
# drop the two entries whose on-disk emoji fails the taxonomy's own
# Extended_Pictographic+VS16 canonical-emoji rule (verified against the real
# validator): "#⃣" (keycap, base codepoint not pictographic) and "🗚️" (not
# Extended_Pictographic in the Unicode data the validator uses). Neither can
# ever be a registered directory kind's own emoji, and since the actual
# on-disk directories already carry exactly these (invalid) emoji, no kind
# registration can make them match without an on-disk rename first (out of
# this slice's scope: taxonomy vocabulary only, no file moves).
for bad in ("hash", "fonts"):
    assert bad in sd
    del sd[bad]

tc = sd["test-case"]
tc["parentKindIds"] = [k for k in tc["parentKindIds"] if k not in ("hash", "fonts")]

# ---- fileKindResolutionRules: point the split extensions at their new owners ----
frr = d["fileKindResolutionRules"]
remap = {
    "physical-model-3d-3dm": "cad-source-model",
    "physical-model-3d-stp": "cad-source-model",
    "physical-model-3d-step": "cad-source-model",
    "physical-model-3d-glb": "mesh-model",
    "physical-model-3d-gltf": "mesh-model",
    "physical-model-3d-obj": "mesh-model",
    "physical-model-3d-stl": "mesh-model",
    "physical-model-3d-ply": "mesh-model",
    "physical-model-3d-las": "mesh-model",
    "physical-model-3d-dxf": "drawing-2d-model",
    "physical-model-3d-dwg": "drawing-2d-model",
    "physical-model-3d-ifc": "building-model",
}
for rule_id, new_kind in remap.items():
    assert frr[rule_id]["fileKindId"] == "model-3d", rule_id
    frr[rule_id]["fileKindId"] = new_kind

# ---- packageSourceDispositions: one entry per new tool-config fixed contract ----
psd = d["packageSourceDispositions"]
psd["vitest-config"] = {"contractKind": "fixed", "disposition": "tool-metadata", "validator": "tool-config-vitest", "authority": "Vitest", "verification": "vitest --config"}
psd["tailwind-config"] = {"contractKind": "fixed", "disposition": "tool-metadata", "validator": "tool-config-tailwind", "authority": "Tailwind CSS", "verification": "tailwindcss --config"}
psd["postcss-config"] = {"contractKind": "fixed", "disposition": "tool-metadata", "validator": "tool-config-postcss", "authority": "PostCSS", "verification": "postcss --config"}
psd["eslint-config"] = {"contractKind": "fixed", "disposition": "tool-metadata", "validator": "tool-config-eslint", "authority": "ESLint", "verification": "eslint --config"}
psd["dependency-cruiser-config"] = {"contractKind": "fixed", "disposition": "tool-metadata", "validator": "tool-config-dependency-cruiser", "authority": "dependency-cruiser", "verification": "depcruise --config"}

out = json.dumps(d, indent=2, ensure_ascii=False) + "\n"
open(F, "w", encoding="utf-8").write(out)
print("semanticDirectoryKinds now:", len(sd))
print("test-case parentKindIds count:", len(tc["parentKindIds"]))
