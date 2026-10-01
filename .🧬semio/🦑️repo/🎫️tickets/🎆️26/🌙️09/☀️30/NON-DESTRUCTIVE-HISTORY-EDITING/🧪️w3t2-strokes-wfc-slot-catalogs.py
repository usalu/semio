"""📚️ W3-T2-STROKES: registers the `drag-slots` / `set-slot-positions` leaves of `s.wfc.wfc2d` and `s.wfc.wfc3d`
in each subset's oracle catalog (kinds, vectors, manifest rows, vector count), its exhaustive-case feature tables and,
for wfc3d, its mount contract. Idempotent: a second run changes nothing.

Run from anywhere: `python3 <this file>`.
"""

import json
import os

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
WFC = os.path.join(ROOT, "✏️s", "🔌️plugins", "🀄️wfc", "🗿️artifacts")

SUBSETS = {
    "wfc2d": {
        "root": os.path.join(WFC, "◻️2d", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any"),
        "capability": "wfc2d-1-mutate",
        "feature": os.path.join("🧪️tests", "🧩️mutate-wfc2d-1", "🥒️.feature"),
        "after": "delete-rule",
        "vectors": [
            ("drag-slots", "✋️drag-slots", "DragSlots", [("drags-slots-a-and-b-together", "✋️drags-slots-a-and-b-together"), ("skips-a-slot-the-board-lacks", "⚠️skips-a-slot-the-board-lacks")]),
            ("set-slot-positions", "🎯️set-slot-positions", "SetSlotPositions", [("sets-slots-a-and-c", "🎯️sets-slots-a-and-c")]),
        ],
        "mount": None,
    },
    "wfc3d": {
        "root": os.path.join(WFC, "🧊️3d", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any"),
        "capability": "wfc3d-1-mutate",
        "feature": os.path.join("🧪️tests", "🧩️mutate-wfc3d-1", "🥒️.feature"),
        "after": "change-seed",
        "vectors": [
            ("drag-slots", "✋️drag-slots", "DragSlots", [("lifts-both-rooms-two-storeys", "✋️lifts-both-rooms-two-storeys"), ("skips-a-slot-the-corridor-lacks", "⚠️skips-a-slot-the-corridor-lacks")]),
            ("set-slot-positions", "🎯️set-slot-positions", "SetSlotPositions", [("sets-the-corridor-and-room-b", "🎯️sets-the-corridor-and-room-b")]),
        ],
        "mount": os.path.join("🧫️fixtures", "🧩️mount-contract", "🔣️.json"),
    },
}


def catalog(subset):
    path = os.path.join(subset["root"], "🔮️oracles", "🔣️.json")
    with open(path, encoding="utf-8") as handle:
        document = json.load(handle)
    mutation_catalog = document["mutationCatalogs"][0]
    manifest = document["mutationManifests"][0]
    for kind, directory, variant, scenarios in subset["vectors"]:
        if kind not in mutation_catalog["kinds"]:
            mutation_catalog["kinds"].append(kind)
        if not any(vector["mutationId"] == kind for vector in mutation_catalog["vectors"]):
            mutation_catalog["vectors"].append({"mutationId": kind, "sourceMutationDirectoryName": directory, "mutationDirectoryName": directory, "scenarios": [{"id": scenario, "directoryName": name} for scenario, name in scenarios]})
        if not any(row["id"] == kind for row in manifest["mutations"]):
            manifest["mutations"].append({
                "id": kind,
                "capability": subset["capability"],
                "payloadSchema": "🧬️.schema.json",
                "outcomes": ["applied", "no-op", "rejected"],
                "productionDispatch": {"operation": kind, "bridgeVersion": 1, "variant": variant},
                "oracleRequirements": [{"capability": subset["capability"], "qualifyingKind": "verified-native-second-implementation"}],
            })
    coverage = document["oracles"][0]["nativeSecondImplementation"]["fixtureCoverage"]
    coverage["vectors"] = sum(len(vector["scenarios"]) for vector in mutation_catalog["vectors"])
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(document, indent=2, ensure_ascii=False) + "\n")
    print(path, "kinds", len(mutation_catalog["kinds"]), "vectors", coverage["vectors"])


def feature(subset):
    path = os.path.join(subset["root"], subset["feature"])
    with open(path, encoding="utf-8") as handle:
        lines = handle.read().split("\n")
    output = []
    for line in lines:
        output.append(line)
        cells = [cell.strip() for cell in line.strip().strip("|").split("|")] if line.strip().startswith("|") else []
        if cells and cells[0] == subset["after"]:
            indent = line[: len(line) - len(line.lstrip())]
            width = len(line.split("|")[1]) - 1
            vector_width = len(line.split("|")[2]) - 1
            for kind, directory, _, scenarios in subset["vectors"]:
                for _, name in scenarios:
                    row = f"{indent}| {kind.ljust(width - 1)}| {(directory + '/' + name).ljust(vector_width - 1)}|"
                    if row.strip() not in (existing.strip() for existing in lines):
                        output.append(row)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write("\n".join(output))
    print(path, "rows", sum(1 for line in output if "drag-slots" in line or "set-slot-positions" in line))


def mount(subset):
    path = os.path.join(subset["root"], subset["mount"])
    with open(path, encoding="utf-8") as handle:
        text = handle.read()
    for kind, *_ in subset["vectors"]:
        if f'"{kind}"' not in text:
            anchor = f'    "{subset["after"]}"\n  ],'
            assert text.count(anchor) == 1, path
            text = text.replace(anchor, f'    "{subset["after"]}",\n    "{kind}"\n  ],')
            subset["after"] = kind
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)
    print(path, "mount ok")


def main():
    for subset in SUBSETS.values():
        catalog(subset)
        feature(subset)
        if subset["mount"]:
            mount(subset)


if __name__ == "__main__":
    main()
