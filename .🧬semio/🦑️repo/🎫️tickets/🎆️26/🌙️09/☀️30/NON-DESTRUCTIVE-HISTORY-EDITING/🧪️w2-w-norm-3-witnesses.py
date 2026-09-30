"""🧪️ W2-W-norm-3: authors the payload-only wire witnesses of every still-unwitnessed EN 1992, EN 1993, EN 1997 and EN 1999 leaf.

`python3 🧪️w2-w-norm-3-witnesses.py <dump dir>` writes `…/🧫️fixtures/🧬️mutations/<leaf>/🧾️wire-witness/🦠️mutation/🔣️.json` — the
aggregate wire (`{"<Variant>": payload}` for the externally tagged EN 1992/1993 aggregates, `{"mutation": "<camelKind>", …}`
for the internally tagged EN 1997/1999 ones) of a domain-meaningful op built from a Rust-encoded example document, with
every number spelled as its Rust type emits it (`f64` always with a fraction, integers without), which is what the crates'
`assert_wire_witness` law compares byte-for-value. Then it writes each crate's witness law test listing its witnesses.
"""
import copy
import json
import os
import re
import sys

ARTIFACTS = "✏️s/🔌️plugins/📕️norm/🗿️artifacts"


def subset(artifact):
    return f"{ARTIFACTS}/{artifact}/🏅️standards/🔖️1/🪆️subsets/✳️any"


def pascal(kind):
    return "".join(part.capitalize() for part in kind.split("-"))


def camel(kind):
    text = pascal(kind)
    return text[0].lower() + text[1:]


def load(dump, artifact, example):
    return json.load(open(os.path.join(dump, artifact, example + ".json"), encoding="utf-8"))


def first(document, key):
    items = document[key]
    assert items, f"example carries no {key}"
    return copy.deepcopy(items[0])


def copy_with(record, identity, **fields):
    record = copy.deepcopy(record)
    record.update({"id": identity, **fields})
    return record


def en1993(dump):
    base = load(dump, "en1993", "🔩️high-strength-connection")
    backup = load(dump, "en1993", "✅️heb240-compliant")
    fixtures = f"{subset('🔩️en1993')}/🧫️fixtures/🧬️mutations"

    def record(collection, update_dir, field):
        for document in (base, backup):
            if document[collection]:
                return copy.deepcopy(document[collection][0])
        path = next(os.path.join(root, name) for root, _, names in os.walk(f"{fixtures}/{update_dir}") for name in names if root.endswith("🦠️mutation"))
        return copy.deepcopy(next(iter(json.load(open(path, encoding="utf-8")).values()))[field])

    families = [
        ("bridge-fatigue", "bridgeFatigue", "bridgeFatigueItem", "🌉️update-bridge-inputs"),
        ("cold-formed-member", "coldFormedMembers", "coldFormedMember", "🥶️update-cold-formed-inputs"),
        ("crane-runway", "craneRunways", "craneRunway", "🏗️update-crane-inputs"),
        ("fatigue-detail", "fatigueDetails", "fatigueDetail", "🔁️update-fatigue-inputs"),
        ("fire-exposure", "fireExposures", "fireExposure", "🔥️update-fire-inputs"),
        ("joint", "joints", "joint", "🔩️update-bolt-inputs"),
        ("load-case", "loadCases", "loadCase", "⬜️update-hss-inputs"),
        ("material", "materials", "material", "✨️update-stainless-inputs"),
        ("member", "members", "member", "📊️update-member-properties"),
        ("member-action", "memberActions", "memberAction", "🧲️update-weld-inputs"),
        ("pile", "piles", "pile", "🪵️update-pile-inputs"),
        ("plated-panel", "platedPanels", "platedPanel", "🧱️update-plated-inputs"),
        ("section", "sections", "section", "↕️update-through-thickness-inputs"),
        ("silo-shell", "siloShells", "siloShell", "🛢️update-silo-shell-inputs"),
        ("tension-component", "tensionComponents", "tensionComponent", "🪢️update-tension-component-inputs"),
        ("tower-leg", "towerLegs", "towerLeg", "🗼️update-tower-inputs"),
    ]
    witnesses = {}
    for entity, collection, field, update_dir in families:
        item = record(collection, update_dir, field)
        if "id" in item:
            item["id"] = f"{item['id']}-2"
        witnesses[f"➕️insert-{entity}"] = ("insert-" + entity, {"index": len(base[collection]), field: item})
        witnesses[f"➖️remove-{entity}"] = ("remove-" + entity, {"index": 0})
    return "🔩️en1993", "external", witnesses


def en1992(dump):
    base = load(dump, "en1992", "🛢️liquid-retaining-fem-anchor")
    member = first(base, "members")
    layer = member["longitudinal"][0]
    action = member["actions"][0]
    anchor = first(base, "anchors")
    concrete = first(base, "concreteGrades")
    steel = first(base, "reinforcementGrades")
    witnesses = {
        "#️⃣change-bar-layer-count": ("change-bar-layer-count", {"memberId": member["id"], "layerId": layer["id"], "newCount": layer["count"] + 2}),
        "↔️change-member-width": ("change-member-width", {"memberId": member["id"], "newValue": round(member["width"] * 1.2, 4)}),
        "↕️change-member-height": ("change-member-height", {"memberId": member["id"], "newValue": round(member["height"] * 1.2, 4)}),
        "↘️change-action-v-ed": ("change-action-v-ed", {"memberId": member["id"], "actionId": action["id"], "newValue": round(action["vK"] * 1.25 or 50000.0, 1)}),
        "⚓️insert-anchor": ("insert-anchor", {"index": len(base["anchors"]), "anchor": copy_with(anchor, anchor["id"] + "-2", c1=round(anchor["c1"] * 1.5, 4))}),
        "➕️insert-member": ("insert-member", {"index": len(base["members"]), "member": copy_with(member, member["id"] + "-2", labelEn=member["labelEn"] + " (copy)", labelDe=member["labelDe"] + " (Kopie)")}),
        "➖️remove-member": ("remove-member", {"memberId": member["id"]}),
        "⤴️change-action-mk": ("change-action-mk", {"memberId": member["id"], "actionId": action["id"], "newValue": round(action["mK"] * 1.25 or 80000.0, 1)}),
        "⭕change-bar-layer-diameter": ("change-bar-layer-diameter", {"memberId": member["id"], "layerId": layer["id"], "newDiameter": 0.02 if layer["diameter"] != 0.02 else 0.025}),
        "🌉️change-member-span": ("change-member-span", {"memberId": member["id"], "newValue": round(member["span"] * 1.1 or 6.0, 4)}),
        "🌍️change-annex": ("change-annex", {"newAnnex": "En" if base["annex"] != "En" else "De"}),
        "🌦change-member-exposure": ("change-member-exposure", {"memberId": member["id"], "newExposure": "Xd1" if member["exposure"] != "Xd1" else "Xc3"}),
        "🏋️change-action-n-ed": ("change-action-n-ed", {"memberId": member["id"], "actionId": action["id"], "newValue": round(action["nK"] * 1.25 or 120000.0, 1)}),
        "🏷️change-title": ("change-title", {"newTitle": "Liquid-retaining tank wall with post-installed anchors"}),
        "📅️change-design-working-life": ("change-design-working-life", {"newYears": 100.0}),
        "📍change-anchor-h-ef": ("change-anchor-h-ef", {"anchorId": anchor["id"], "newValue": round(anchor["hEf"] * 1.25, 4)}),
        "📏️change-delta-c-dev": ("change-delta-c-dev", {"newDeltaCDev": 0.015}),
        "📐️change-member-effective-depth": ("change-member-effective-depth", {"memberId": member["id"], "newValue": round(member["effectiveDepth"] * 1.1, 4)}),
        "🔀️reorder-members": ("reorder-members", {"fromIndex": len(base["members"]) - 1, "toIndex": 0}),
        "🔥change-member-axis-distance": ("change-member-axis-distance", {"memberId": member["id"], "newAxisDistance": 0.045}),
        "🔥️change-member-fire-rating": ("change-member-fire-rating", {"memberId": member["id"], "newRating": "R120"}),
        "🔩change-reinforcement-f-yk": ("change-reinforcement-f-yk", {"gradeId": steel["id"], "newFYk": 550000000.0 if steel["fYk"] != 550000000.0 else 500000000.0}),
        "🗑️remove-anchor": ("remove-anchor", {"anchorId": anchor["id"]}),
        "🛡️change-member-cover": ("change-member-cover", {"memberId": member["id"], "newValue": round(member["cover"] + 0.01, 4)}),
        "🧪change-cement-type": ("change-cement-type", {"newCementType": "S" if base["cementType"] != "S" else "R"}),
        "🧱change-concrete-f-ck": ("change-concrete-f-ck", {"gradeId": concrete["id"], "newFCk": 45000000.0 if concrete["fCk"] != 45000000.0 else 40000000.0}),
        "🧷change-anchor-a-s": ("change-anchor-as", {"anchorId": anchor["id"], "newValue": round(anchor["aS"] * 1.5, 9)}),
        "🪢change-member-stirrup-spacing": ("change-member-stirrup-spacing", {"memberId": member["id"], "newSpacing": 0.15}),
    }
    return "🏛️en1992", "external", witnesses


def en1997(dump):
    base = load(dump, "en1997", "🎬️demo")
    footing, layer, pile = first(base, "footings"), first(base, "layers"), first(base, "piles")
    wall = first(base, "retainingWalls")
    slope = first(base, "slopes")
    witnesses = {
        "↔️change-footing-width": ("change-footing-width", {"id": footing["id"], "newWidth": round(footing["width"] + 0.5, 4)}),
        "⛰️change-slope-angle": ("change-slope-angle", {"id": slope["id"], "newAngleDeg": 30.0}),
        "➕insert-footing": ("insert-footing", {"index": len(base["footings"]), "footing": copy_with(footing, footing["id"] + "-2")}),
        "➕️insert-layer": ("insert-layer", {"index": len(base["layers"]), "layer": copy_with(layer, layer["id"] + "-2", depthTop=layer["depthBottom"], depthBottom=round(layer["depthBottom"] + 3.0, 4))}),
        "➖remove-footing": ("remove-footing", {"index": 0}),
        "➖️remove-layer": ("remove-layer", {"index": 0}),
        "⬇️change-footing-embedment": ("change-footing-embedment", {"id": footing["id"], "newEmbedment": round(footing["embedment"] + 0.3, 4)}),
        "🌀️change-layer-oedometric-modulus": ("change-layer-oedometric-modulus", {"id": layer["id"], "newOedometricModulus": round(layer["oedometricModulus"] * 1.5, 1)}),
        "🌍️change-annex": ("change-annex", {"newAnnex": "En" if base["annex"] != "En" else "De"}),
        "💧change-groundwater-level": ("change-groundwater-level", {"newGroundwaterLevel": -2.5}),
        "📅️change-design-situation": ("change-design-situation", {"newDesignSituation": "bsT" if base["designSituation"] != "bsT" else "bsP"}),
        "📏️change-pile-length": ("change-pile-length", {"id": pile["id"], "newLength": round(pile["length"] + 2.0, 4)}),
        "📐️change-layer-phi-prime": ("change-layer-phi-prime", {"id": layer["id"], "newPhiPrimeDeg": 32.5}),
        "📤remove-pile": ("remove-pile", {"index": 0}),
        "📥insert-pile": ("insert-pile", {"index": len(base["piles"]), "pile": copy_with(pile, pile["id"] + "-2")}),
        "🔎️change-investigation-depth": ("change-investigation-depth", {"newInvestigationDepth": 25.0}),
        "🔢change-pile-count": ("change-pile-count", {"id": pile["id"], "newCount": pile["count"] + 2}),
        "🗂️change-geotechnical-category": ("change-geotechnical-category", {"newGeotechnicalCategory": 3 if base["geotechnicalCategory"] != 3 else 2}),
        "🧭️change-design-approach": ("change-design-approach", {"newDesignApproach": "da3" if base["designApproach"] != "da3" else "da2"}),
        "🧱change-wall-base-width": ("change-wall-base-width", {"id": wall["id"], "newBaseWidth": round(wall["baseWidth"] + 0.4, 4)}),
    }
    return "🌍️en1997", "internal", witnesses


def en1999(dump):
    base = load(dump, "en1999", "🏠️aluminium-roof-purlin")
    member, connection, section = first(base, "members"), first(base, "connections"), first(base, "sections")
    action = member["actions"][0]
    element = section["elements"][0]
    material = first(base, "materials")
    tweak = lambda records, key, factor: [dict(record, **{key: round(record[key] * factor, 6)}) if index == 0 else record for index, record in enumerate(copy.deepcopy(records))]
    witnesses = {
        "⚗️change-material-designation": ("change-material-designation", {"materialId": material["id"], "newDesignation": "EN AW-6082 T6" if material["designation"] != "EN AW-6082 T6" else "EN AW-6061 T6"}),
        "❄️change-cold-formed": ("change-cold-formed", {"coldFormed": copy.deepcopy(base["coldFormed"])}),
        "➕add-member": ("add-member", {"index": len(base["members"]), "member": copy_with(member, member["id"] + "-2")}),
        "➖remove-member": ("remove-member", {"id": member["id"]}),
        "⤴️change-member-my-ed": ("change-member-my-ed", {"memberId": member["id"], "actionId": action["id"], "newMYK": round(action["mYK"] * 1.25 or 5000.0, 1)}),
        "🌍️change-annex": ("change-annex", {"newAnnex": "En" if base["annex"] != "En" else "De"}),
        "🏋️change-member-n-ed": ("change-member-n-ed", {"memberId": member["id"], "actionId": action["id"], "newNK": round(action["nK"] * 1.25 or 20000.0, 1)}),
        "🏗️change-members": ("change-members", {"members": tweak(base["members"], "length", 1.1)}),
        "📏️change-member-buckling-length": ("change-member-buckling-length", {"memberId": member["id"], "axis": "z", "newLength": round(member["bucklingLengthZ"] * 0.5 or 1.5, 4)}),
        "📐️change-sections": ("change-sections", {"sections": tweak(base["sections"], "height", 1.2)}),
        "🔄️change-fatigue-details": ("change-fatigue-details", {"fatigueDetails": copy.deepcopy(base["fatigueDetails"])}),
        "🔗change-connections": ("change-connections", {"connections": copy.deepcopy(base["connections"])}),
        "🔥️change-fire-scenarios": ("change-fire-scenarios", {"fireScenarios": copy.deepcopy(base["fireScenarios"])}),
        "🔥️change-weld-throat": ("change-weld-throat", {"connectionId": connection["id"], "newThroat": 0.005}),
        "🔩change-bolt-count": ("change-bolt-count", {"connectionId": connection["id"], "newRows": 3, "newBoltsPerRow": 2}),
        "🧱change-materials": ("change-materials", {"materials": copy.deepcopy(base["materials"]) + [{"id": "mat-6082", "designation": "EN AW-6082 T6"}]}),
        "🧱change-plate-thickness": ("change-plate-thickness", {"sectionId": section["id"], "elementId": element["id"], "newThickness": round(element["thickness"] * 1.25, 6)}),
        "🫙change-shells": ("change-shells", {"shells": copy.deepcopy(base["shells"])}),
    }
    return "🪶️en1999", "internal", witnesses


def schema_types(schema, documents):
    def resolve(node, context):
        while "$ref" in node:
            reference = node["$ref"]
            target, _, pointer = reference.partition("#")
            document = documents[target] if target else context
            context = document
            node = document
            for part in [part for part in pointer.split("/") if part]:
                node = node[part]
        return node, context
    return resolve


def canonical(value, node, context, resolve):
    node, context = resolve(node, context)
    for keyword in ("oneOf", "anyOf"):
        if keyword in node:
            wanted = {"object"} if isinstance(value, dict) else {"array"} if isinstance(value, list) else {"number", "integer"}
            candidates = []
            for branch in node[keyword]:
                resolved, branch_context = resolve(branch, context)
                declared = resolved.get("type")
                declared = set(declared) if isinstance(declared, list) else {declared} if declared else ({"object"} if "properties" in resolved else set())
                if declared & wanted:
                    candidates.append((resolved, branch_context))
            def fits(candidate):
                resolved, branch_context = candidate
                if not isinstance(value, dict):
                    return True
                properties = resolved.get("properties", {})
                consts = all(value.get(key) == prop["const"] for key, prop in properties.items() if isinstance(prop, dict) and "const" in prop and key in value)
                return consts and set(resolved.get("required", [])) <= set(value) and (resolved.get("additionalProperties", True) is not False or set(value) <= set(properties))
            chosen = next((candidate for candidate in candidates if fits(candidate)), candidates[0] if candidates else None)
            if chosen:
                node, context = chosen
    kind = node.get("type")
    kinds = set(kind) if isinstance(kind, list) else {kind}
    if isinstance(value, bool) or value is None or isinstance(value, str):
        return value
    if isinstance(value, (int, float)):
        if "integer" in kinds:
            assert float(value).is_integer(), value
            return int(value)
        if "number" in kinds:
            return float(value)
        if not kinds - {None}:
            return value
        raise AssertionError(f"number {value} under a schema node without a numeric type: {node}")
    if isinstance(value, list):
        items = node.get("items", {})
        return [canonical(item, items, context, resolve) for item in value]
    if isinstance(value, dict):
        properties = node.get("properties", {})
        if not properties and "allOf" in node:
            for part in node["allOf"]:
                properties.update(resolve(part, context)[0].get("properties", {}))
        return {key: canonical(item, properties.get(key, node.get("additionalProperties", {}) if isinstance(node.get("additionalProperties"), dict) else {}), context, resolve) for key, item in value.items()}
    raise AssertionError(value)


def documents_for(root):
    documents = {}
    for directory, _, names in os.walk(root):
        for name in names:
            if name == "🔣️.json" and "🧬️schema" in directory:
                document = json.load(open(os.path.join(directory, name), encoding="utf-8"))
                if isinstance(document, dict) and "$id" in document:
                    documents[document["$id"]] = document
    return documents


TEST = '''//! 🧾️ Every committed payload-only wire witness of this vocabulary IS the canonical Rust wire of its op
//! (`store::os_store::test_support::assert_wire_witness`), and names the leaf it witnesses.
use crate::standards::v1::subsets::any::schema::mutations::{aggregate};

const WITNESSES: &[(&str, &str)] = &[
{rows}];

#[test]
fn committed_wire_witnesses_are_the_canonical_wire() {{
    for (kind, witness) in WITNESSES {{
        let op: {aggregate} = store::os_store::test_support::assert_wire_witness(witness);
        assert_eq!(protocol::Mutation::<{snapshot}>::descriptor(&op).semantic_kind, *kind, "the witness of {{kind}} decodes to another leaf");
    }}
}}
'''


def main(dump):
    for build in (en1993, en1992, en1997, en1999):
        artifact, tagging, witnesses = build(dump)
        root = subset(artifact)
        mutations = f"{root}/🧬️schema/🧬️mutations"
        documents = documents_for(f"{root}/🧬️schema")
        resolve = schema_types(None, documents)
        number = re.search(r"en(\d+)", artifact).group(1)
        rows = []
        for leaf, (kind, payload) in witnesses.items():
            schema = json.load(open(f"{mutations}/{leaf}/🧬️schema/🔣️.json", encoding="utf-8"))
            descriptor = json.load(open(f"{mutations}/{leaf}/🔣️.json", encoding="utf-8"))
            assert descriptor["semanticKind"] == kind, (leaf, descriptor["semanticKind"], kind)
            payload = canonical(payload, schema, schema, resolve)
            wire = {descriptor["aggregateVariant"]: payload} if tagging == "external" else {"mutation": schema["properties"]["mutation"]["const"], **payload}
            path = f"{root}/🧫️fixtures/🧬️mutations/{leaf}/🧾️wire-witness/🦠️mutation/🔣️.json"
            os.makedirs(os.path.dirname(path), exist_ok=True)
            with open(path, "w", encoding="utf-8") as handle:
                handle.write(json.dumps(wire, ensure_ascii=False, indent=2) + "\n")
            rows.append(f'    ("{kind}", include_str!("../../../../🧫️fixtures/🧬️mutations/{leaf}/🧾️wire-witness/🦠️mutation/🔣️.json")),\n')
        with open(f"{mutations}/🧪️tests/🧪️wire-witness/🦀️.rs", "w", encoding="utf-8") as handle:
            handle.write(TEST.format(aggregate=f"En{number}Mutation", snapshot=f"crate::En{number}Snapshot", rows="".join(rows)))
        print(artifact, len(witnesses), "witnesses")


if __name__ == "__main__":
    main(sys.argv[1])
