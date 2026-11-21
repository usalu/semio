"""🫁️ Apply the ticket's scale-specific thermal breath marker geometry to four animal assets."""
import json
import re
from pathlib import Path

root = Path("🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals")
designs = {
    "goat": {
        50: {"nose-pair": [0.0038, 0.004, 0.005], "mouth": [0.0038, 0.009, 0.0038]},
        100: {"nose": [0.0076, 0.008, 0.014], "mouth": [0.0076, 0.012, 0.008]},
        200: {"face": [0.0152, 0.019, 0.017]},
        500: {"face": [0.0315, 0.034, 0.0325]},
    },
    "pig": {
        50: {"nose-pair": [0.0038, 0.0065, 0.008]},
        100: {"nose": [0.0076, 0.008, 0.016]},
    },
    "sheep": {
        50: {"nose-pair": [0.0038, 0.0046, 0.0055], "mouth": [0.0038, 0.01, 0.004]},
        100: {"nose": [0.0076, 0.008, 0.014], "mouth": [0.0076, 0.013, 0.008]},
        200: {"face": [0.0152, 0.019, 0.018]},
    },
    "pigeon": {
        50: {"nose": [0.0024, 0.00195, 0.003]},
        100: {"nose": [0.0027, 0.00165, 0.0034]},
    },
}

def display(solid):
    return "\n".join("          " + line for line in json.dumps(solid, ensure_ascii=False, indent=2).splitlines())

def resized(solid, radii):
    if solid["kind"] == "ellipsoid":
        return {**solid, "radii": radii}
    assert solid["kind"] == "extrude"
    normal, first, second = {"yz": (0, 1, 2), "zx": (1, 2, 0), "xy": (2, 0, 1)}[solid["plane"]]
    width = max(abs(x) for x, _ in solid["profile"])
    height = max(abs(y) for _, y in solid["profile"])
    profile = [[round(x * radii[first] / width, 8), round(y * radii[second] / height, 8)] for x, y in solid["profile"]]
    return {**solid, "from": -radii[normal], "to": radii[normal], "profile": profile}

for animal, scales in designs.items():
    path = root / animal / "🔣️.json"
    current = path.read_text()
    parsed = json.loads(current)
    planned = json.loads(current)
    prefix, volumes = current.split('    "volumes": {\n')
    for scale, model in scales.items():
        key = str(scale)
        mark = f'      "{key}": {{\n'
        start = volumes.index(mark) + len(mark)
        following = re.search(r'^      "\d+": \{', volumes[start:], re.M)
        end = start + following.start() if following else len(volumes)
        block = volumes[start:end]
        thermal = planned["asset"]["volumes"][key]["thermal"]
        original = [solid for solid in thermal if solid["id"].startswith("th-breath-")]
        nasal_near, nasal_far, *other = original
        mouth = other[0] if other else None
        changes = {}
        if "face" in model:
            assert mouth is not None
            centre = [round((left + right) / 2, 5) for left, right in zip(nasal_near["transform"]["translation"][:2], mouth["transform"]["translation"][:2])]
            changes[nasal_near["id"]] = {
                **resized(nasal_near, model["face"]), "id": "th-breath-muzzle",
                "transform": {"translation": [*centre, 0]},
                "tags": ["emitter", "breath-nose", "breath-mouth"],
            }
            changes[nasal_far["id"]] = None
            changes[mouth["id"]] = None
        else:
            if "nose" in model:
                changes[nasal_near["id"]] = {
                    **resized(nasal_near, model["nose"]),
                    "id": "th-breath-nostril" if animal == "pigeon" else "th-breath-nose",
                    "transform": {"translation": [*nasal_near["transform"]["translation"][:2], 0]},
                }
                changes[nasal_far["id"]] = None
            else:
                for nasal in (nasal_near, nasal_far):
                    changes[nasal["id"]] = resized(nasal, model["nose-pair"])
            if mouth is not None and "mouth" in model:
                changes[mouth["id"]] = resized(mouth, model["mouth"])
        for solid in original:
            if solid["id"] not in changes:
                continue
            updated = changes[solid["id"]]
            old = display(solid)
            assert block.count(old) == 1, (animal, scale, solid["id"])
            if updated is None:
                assert old + ",\n" in block, (animal, scale, solid["id"])
                block = block.replace(old + ",\n", "", 1)
            else:
                block = block.replace(old, display(updated), 1)
        planned["asset"]["volumes"][key]["thermal"] = [changes.get(solid["id"], solid) for solid in thermal if changes.get(solid["id"], solid) is not None]
        volumes = volumes[:start] + block + volumes[end:]
    result = prefix + '    "volumes": {\n' + volumes
    assert json.loads(result) == planned
    assert parsed["asset"]["thermal"] == planned["asset"]["thermal"]
    assert parsed["asset"]["rig"] == planned["asset"]["rig"]
    assert path.read_text() == current
    path.write_text(result)
