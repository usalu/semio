#!/usr/bin/env python3
"""🔨️ F1 — merges the fixtures `🔨️f1-ifc-generate.py` produced into each IFC subset's own
`🧪️oracle/🔣️.json`. `base`/`cobie`/`cv20`/`sav` reference their ALREADY-registered `ifcopenshell`
`third-party-library` entry (shard E2). `4/any` gets one new `steputils` `third-party-library` entry
scoped to `ifc-4-any-mutate`, mirroring `step/ap214/base`'s own registration."""
import json
import os

REPO = os.getcwd()
TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION"
ROOT_2X3 = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets"
ROOT_4 = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets"

SUBSET_CONFIG = {
    "✳️base": {"path": f"{ROOT_2X3}/✳️base", "target": {"artifact": "s.stdio.ifc", "standard": "2x3", "subset": "base"}, "oracle": "ifcopenshell-ifc-2x3-base-differential", "kind": "ifcopenshell"},
    "✳️cobie": {"path": f"{ROOT_2X3}/✳️cobie", "target": {"artifact": "s.stdio.ifc", "standard": "2x3", "subset": "cobie"}, "oracle": "ifcopenshell-ifc-2x3-cobie-mutate-reader", "kind": "ifcopenshell"},
    "✳️cv20": {"path": f"{ROOT_2X3}/✳️cv20", "target": {"artifact": "s.stdio.ifc", "standard": "2x3", "subset": "cv20"}, "oracle": "ifcopenshell-ifc-2x3-cv20-mutate-reader", "kind": "ifcopenshell"},
    "✳️sav": {"path": f"{ROOT_2X3}/✳️sav", "target": {"artifact": "s.stdio.ifc", "standard": "2x3", "subset": "sav"}, "oracle": "ifcopenshell-ifc-2x3-sav-mutate-reader", "kind": "ifcopenshell"},
    "4-any": {"path": f"{ROOT_4}/✳️any", "target": {"artifact": "s.stdio.ifc", "standard": "4", "subset": "base"}, "oracle": "steputils-ifc-4-any-mutate-reader", "kind": "steputils"},
}

NEW_STEPUTILS_ORACLE = {
    "id": "steputils-ifc-4-any-mutate-reader",
    "kind": "third-party-library",
    "ecosystem": "python",
    "package": "steputils",
    "version": "0.1",
    "engine": {"family": "steputils", "implementation": "steputils Part-21/STEP reader+writer", "version": "0.1"},
    "capabilities": ["ifc-4-any-mutate"],
    "license": "MIT",
    "testOnly": True,
    "productionReachable": False,
    "networkDuringExecution": False,
    "homepage": "https://github.com/mozman/steputils",
    "comparisonProfiles": ["semantic-ifc-v1"],
    "rationale": (
        "📖️ F1 (ticket SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION). "
        "`4/any`'s own `IfcMutation` operates at the RAW Part-21 entity-graph level "
        "(InsertEntity/RemoveEntity/SetEntityName/SetEntityArg/InsertEntityArg/RemoveEntityArg over an "
        "id-keyed `entities` collection and its positional `args`) — the exact same generic model "
        "`step/ap214/base`'s own `StepMutation` uses, already discharged there by this same package "
        "(`steputils-step-ap214-base-mutate-reader`). The sibling `ifcopenshell-ifc-4-any-differential` "
        "entry registered in this file is schema-TYPED (IFC4 EXPRESS entities have fixed attribute "
        "arity in ifcopenshell's own C++ core) and cannot perform an arity-changing edit like "
        "InsertEntityArg/RemoveEntityArg at all — verified directly this session. `steputils` 0.1 "
        "(mozman, MIT) is schema-agnostic ISO 10303-21 Part-21 syntax: IFC4 is physically Part-21 text "
        "under a different EXPRESS schema, exactly as `ruststep`'s own sibling rationale in `2x3/base` "
        "already establishes for IFC2X3. Verified genuinely this session by round-tripping this "
        "subset's own real fixture (`shared://🧪️nakagin-capsule-tower/🏗️.ifc`, IFC4, 24792 real "
        "entities) through `p21.readfile`/`StepFile.save` and diffing the result. Used here as the "
        "generator for every `fixtureManifests[]` entry below this rationale registers: `before` is "
        "this fixture round-tripped unmodified, `after` is the same load with exactly one raw "
        "entity/header edit applied through steputils' own object model before saving."
    ),
}


def merge_subset(key, fragments):
    cfg = SUBSET_CONFIG[key]
    oracle_path = os.path.join(REPO, cfg["path"], "🧪️oracle", "🔣️.json")
    with open(oracle_path, "r", encoding="utf-8") as f:
        doc = json.load(f)

    if cfg["kind"] == "steputils":
        existing_ids = {o["id"] for o in doc.get("oracles", [])}
        if cfg["oracle"] not in existing_ids:
            doc.setdefault("oracles", []).append(NEW_STEPUTILS_ORACLE)

    doc.setdefault("fixtureManifests", [])
    existing_fixture_ids = {fm["id"] for fm in doc["fixtureManifests"]}

    for frag in fragments:
        if frag["id"] in existing_fixture_ids:
            continue
        entry = {
            "schema": "semio.repository-test.fixture/v2",
            "id": frag["id"],
            "class": "third-party-generated",
            "target": cfg["target"],
            "mutation": frag["mutation"],
            "outcome": "applied",
            "units": {"length": "millimetre", "angle": "radian"},
            "files": frag["files"],
            "generator": {
                "oracle": cfg["oracle"],
                "packageVersion": "0.8.4.post1" if cfg["kind"] == "ifcopenshell" else "0.1",
                "engineFamily": cfg["kind"],
                "engineVersion": "0.8.4.post1" if cfg["kind"] == "ifcopenshell" else "0.1",
                "command": f"uv run --group test python3 {TICKET}/🔨️f1-ifc-generate.py",
                "platform": "darwin-arm64",
            },
            "provenance": {
                "source": "generated",
                "license": "LGPL-3.0-or-later (ifcopenshell)" if cfg["kind"] == "ifcopenshell" else "MIT (steputils)",
                "attribution": (
                    "Generated with IfcOpenShell 0.8.4.post1 (LGPL-3.0-or-later) via a real read+write round trip against this repository's own committed real-world fixture"
                    if cfg["kind"] == "ifcopenshell"
                    else "Generated with steputils 0.1 (MIT, mozman) via a real parse+write round trip against this repository's own committed real-world fixture 🧪️nakagin-capsule-tower/🏗️.ifc"
                ),
                "security": "scanned-clean",
                "privacy": "no-personal-data",
            },
            "comparisonProfile": "semantic-ifc-v1",
            "reproducible": True,
            "family": "structural",
            "notes": frag["note"],
        }
        doc["fixtureManifests"].append(entry)

    with open(oracle_path, "w", encoding="utf-8") as f:
        json.dump(doc, f, indent=2, ensure_ascii=False)
        f.write("\n")
    print(f"merged {len(fragments)} fixture(s) into {oracle_path}")


def main():
    frag_path = os.path.join(REPO, TICKET, "🗑️generated", "f1-ifc-fragments.json")
    with open(frag_path, "r", encoding="utf-8") as f:
        fragments_by_subset = json.load(f)
    for key, fragments in fragments_by_subset.items():
        merge_subset(key, fragments)


if __name__ == "__main__":
    main()
