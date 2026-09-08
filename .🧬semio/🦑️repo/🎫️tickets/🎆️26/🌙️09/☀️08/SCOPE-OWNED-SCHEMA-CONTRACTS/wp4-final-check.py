import json, os, re
DRAFT07 = "http://json-schema.org/draft-07/schema#"
touched = [
 "✏️s/🔌️plugins/🌊️flow/🧬️schema", "✏️s/🔌️plugins/🌊️flow/🎬️action-cohort/🧬️schema",
 "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
 "✏️s/🔌️plugins/🌍️gis/🧬️schema", "✏️s/🔌️plugins/🌿️vcs/🧬️schema", "✏️s/🔌️plugins/🗄️stdio/🧬️schema",
 "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️schema", "✏️s/🔌️plugins/🪐️space/🧬️schema",
 "✏️s/🔌️plugins/💡️reasoning/🧬️schema", "✏️s/🔌️plugins/🧱️block/🧬️schema", "✏️s/🔌️plugins/🖍️draw/🧬️schema",
 "✏️s/🔌️plugins/💠️lowpoly/🧬️schema", "✏️s/🔌️plugins/📕️norm/🧬️schema",
 "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
 "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
 "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema",
 "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
 "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
 "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema",
 "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
 "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
 "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
 "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
 "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
 "✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
 "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
 "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
 "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
 "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
 "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
 # 🧬️ WP4b additions
 "✏️s/🔌️plugins/🖨️raster/🧬️schema", "✏️s/🔌️plugins/🧩️puzzle/🧬️schema", "✏️s/🔌️plugins/➗️mathematical/🧬️schema",
 "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
 "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema",
 "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
]
fw = json.load(open("🧰️framework/🔨️modules/🖱️ui/🧬️schema/🔣️.json", encoding="utf-8"))
FW_ID = fw["$id"]
problems = 0
for scope in touched:
    p = os.path.join(scope, "🔣️.json")
    if not os.path.exists(p):
        print("MISSING", p); problems += 1; continue
    d = json.load(open(p, encoding="utf-8"))
    label = scope.replace("✏️s/🔌️plugins/", "")
    if d.get("$schema") != DRAFT07:
        print("DIALECT", label, d.get("$schema")); problems += 1
    if not str(d.get("$id", "")).startswith("https://semio.tech/schema/"):
        print("ID", label, d.get("$id")); problems += 1
    defs = d.get("$defs", {})
    for ref in set(re.findall(r'"\$ref":\s*"([^"]+)"', json.dumps(d))):
        if ref.startswith("#/$defs/"):
            if ref.split("/")[-1] not in defs: print("DANGLING", label, ref); problems += 1
        elif ref.startswith("#/definitions/"):
            if ref.split("/")[-1] not in d.get("definitions", {}): print("DANGLING", label, ref); problems += 1
        elif ref.startswith(FW_ID + "#/$defs/"):
            if ref.split("/")[-1] not in fw["$defs"]: print("DANGLING-FW", label, ref); problems += 1
        else:
            print("FOREIGN", label, ref); problems += 1
print("modules checked:", len(touched), "| problems:", problems)
