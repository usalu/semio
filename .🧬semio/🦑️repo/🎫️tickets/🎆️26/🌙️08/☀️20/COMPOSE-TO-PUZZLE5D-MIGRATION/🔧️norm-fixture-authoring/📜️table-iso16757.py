#!/usr/bin/env python3
"""📦️ iso16757 — the 21 handcrafted fixture cases.

Unlike the three flat scalar norm trees, this is a multi-collection catalogue document, so its
vocabulary spans `change`/`update`/`replace`/`create`/`delete`/`rename`/`add`/`remove`. Each case's
`after` is produced by replaying, BY HAND, exactly what that leaf's own `🔺️diff/🦀️component.rs`
does — and the committed diff is then the single top-level container that oracle wrote, verbatim.

Serde shapes verified in the sources, not assumed:
  · `Iso16757Mutation` carries **no `#[serde(...)]` attribute** → EXTERNALLY tagged, and the payload
    structs carry no `rename_all` either → snake_case payload keys:
    `{"RenameProduct": {"id": "…", "new_name": "…"}}`.
  · `Iso16757Snapshot` IS `#[serde(rename_all = "camelCase")]`, but only at the top level — every
    nested type (`Catalogue`, `Names`, `Subject`, `SelectionRequest`, `ScriptLimits`, …) carries no
    rename, so `product_groups` / `short_name` / `max_steps` stay snake_case inside.
  · `CatalogueValue` and `PartNumberRule` are internally tagged on `kind` with camelCase VARIANTS
    (`Decimal` → `"decimal"`, `Script` → `"script"`); serde's `rename_all` on an enum renames
    variants only, so a struct-variant's own fields (`function_id`) stay snake_case.
  · `ExchangeProcess`, `SubjectKind`, `PropertyKind`, `ConstraintOperator`, `EditionProfile` carry no
    serde rename → bare Rust variant names on the wire.
  · `Iso16757Diff` is `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`,
    so all ten keys are emitted, `null` for the nine each mutation leaves alone.
"""

import copy
import importlib.util
import os
import textwrap

_here = os.path.dirname(__file__)


def _load(name, filename):
    spec = importlib.util.spec_from_file_location(name, os.path.join(_here, filename))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


common = _load("emit_common", "\U0001f4dc️emit-common.py")
wiring = _load("wire", "\U0001f4dc️wire.py")

ARTIFACT = "iso16757"
ARTIFACT_DIR = "\U0001f4d3️iso16757"
SNAPSHOT_TY = "Iso16757Snapshot"
MUTATION_TY = "Iso16757Mutation"
DIFF_TY = "Iso16757Diff"
ISO = "crate::artifacts::iso16757"


def names(text, short=None, alternatives=None):
    return {"preferred": {"locale": "en", "text": text}, "short_name": short, "alternatives": alternatives or []}


BASE = {
    "catalogue": {
        "id": "cat.fixture",
        "metadata": {
            "names": names("Fixture Radiator Catalogue", "Fixture", [{"locale": "de", "text": "Musterkatalog Heizkörper"}]),
            "lifecycle": {"revision": "1", "status": "published", "valid_from": None, "valid_to": None},
            "edition_profile": "FullPublished",
        },
        "manufacturer": {"id": "mfg.fixture", "names": names("Fixture Heating Works")},
        "dictionary": {"id": "bsdd.fixture", "version": "2026-01"},
        "product_groups": [
            {"id": "group.radiators", "names": names("Radiators"), "dictionary_subject_id": "subject.radiator"}
        ],
        "product_classes": [
            {
                "id": "class.panel-radiator",
                "group_id": "group.radiators",
                "parent_id": None,
                "names": names("Panel radiator"),
                "required_property_ids": ["prop.height"],
                "optional_property_ids": [],
            }
        ],
        "product_series": [
            {
                "id": "series.pr",
                "class_id": "class.panel-radiator",
                "names": names("PR series"),
                "shared_property_values": {},
                "geometry_id": None,
            }
        ],
        "products": [
            {
                "id": "product.pr600",
                "series_id": "series.pr",
                "names": names("PR-600"),
                "parameter_domains": [],
                "variants": [],
                "static_properties": [],
            }
        ],
        "product_indexes": [],
        "property_definitions": [
            {
                "id": "prop.height",
                "names": names("Height"),
                "data_type": "decimal",
                "unit": {"symbol": "mm", "dimension": {"length": 1, "mass": 0, "time": 0, "temperature": 0}, "si_factor": 0.001},
                "cardinality": {"min": 1, "max": 1},
                "kind": "Static",
                "dictionary_property_id": "prop.height",
            }
        ],
        "accessories": {},
        "compositions": {},
        "descriptive_objects": [],
        "extensions": {"fields": {}},
    },
    "dictionary": {
        "reference": {"id": "bsdd.fixture", "version": "2026-01"},
        "subjects": [
            {
                "id": "subject.radiator",
                "kind": "ProductClass",
                "names": names("Radiator"),
                "definition": {"locale": "en", "text": "Space heating emitter"},
                "parent_id": None,
            }
        ],
        "relationships": [],
        "properties": [],
        "controlled_lists": [],
        "meta_subjects": [],
    },
    "geometry": {
        "objects": {},
        "primitive_registry": [{"id": "box", "parameters": ["width", "height", "depth"]}],
    },
    "selection": {
        "class_id": "class.panel-radiator",
        "constraints": [
            {"property_id": "prop.height", "operator": "Equal", "value": {"kind": "decimal", "value": 600.0}},
            {"property_id": "prop.length", "operator": "GreaterThan", "value": {"kind": "decimal", "value": 1000.0}},
        ],
        "series_id": "series.pr",
    },
    "partNumberRule": {"kind": "literal", "value": "PR-600"},
    "partNumberInputs": {
        "height": {"kind": "decimal", "value": 600.0},
        "length": {"kind": "decimal", "value": 1200.0},
    },
    "scriptLimits": {"max_steps": 10000, "max_recursion": 64, "timeout_ms": 50},
    "exchangeProcess": "ProvideCatalogue",
}

DIFF_NULLS = {
    "artifact": None,
    "catalogue": None,
    "dictionary": None,
    "geometry": None,
    "selection": None,
    "partNumberRule": None,
    "partNumberInputs": None,
    "scriptLimits": None,
    "exchangeProcess": None,
    "selectedCheckIndex": None,
}

NEW_SUBJECT = {
    "id": "subject.towel-radiator",
    "kind": "ProductSpecialization",
    "names": names("Towel radiator"),
    "definition": {"locale": "en", "text": "Space heating emitter doubling as a towel rail"},
    "parent_id": "subject.radiator",
}

NEW_PROPERTY_DEFINITION = {
    "id": "prop.length",
    "names": names("Length"),
    "data_type": "decimal",
    "unit": {"symbol": "mm", "dimension": {"length": 1, "mass": 0, "time": 0, "temperature": 0}, "si_factor": 0.001},
    "cardinality": {"min": 0, "max": 1},
    "kind": "Selection",
    "dictionary_property_id": None,
}

NEW_PRODUCT_GROUP = {"id": "group.towel-radiators", "names": names("Towel radiators"), "dictionary_subject_id": None}

NEW_PRODUCT = {
    "id": "product.pr900",
    "series_id": "series.pr",
    "names": names("PR-900"),
    "parameter_domains": [],
    "variants": [],
    "static_properties": [],
}

NEW_CONSTRAINT = {"property_id": "prop.width", "operator": "LessThan", "value": {"kind": "decimal", "value": 800.0}}


def doc(emoji, text):
    body = textwrap.wrap(text, width=106)
    return "\n".join(["/// %s %s" % (emoji, body[0])] + ["/// " + line for line in body[1:]])


def indent(lines):
    return "\n".join("    " + line for line in lines)


def render(case):
    kind, name = case["kind"], case["case"]
    label = "%s/%s" % (kind, name)
    after = copy.deepcopy(BASE)
    case["mutate"](after)
    diff = dict(DIFF_NULLS)
    diff[case["diff_key"]] = copy.deepcopy(after[case["diff_key"]])

    tests = [
        """%s
#[semio_framework_async_macros::async_test]
async fn %s() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("%s applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "%s: the applied state differs from the committed after-snapshot");
%s
}""" % (doc("▶️", case["applies_doc"]), case["applies_fn"], kind, label, indent(case["applies_extra"])),
        """%s
#[semio_framework_async_macros::async_test]
async fn %s() {
    let base = before();
    let forward = <%s as protocol::Mutation<%s>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward %s applies");
    let inverse = <%s as protocol::Mutation<%s>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "%s: %s");
    for step in &inverse {
        let undo = <%s as protocol::Mutation<%s>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the %s inverse step applies");
    }
    assert_eq!(snapshot, base, "%s: %s");
}""" % (
            doc("↩️", case["inverse_doc"]), case["inverse_fn"],
            MUTATION_TY, SNAPSHOT_TY, kind, MUTATION_TY, SNAPSHOT_TY,
            label, case["inverse_shape"],
            MUTATION_TY, SNAPSHOT_TY, kind,
            label, case["inverse_restores"],
        ),
        """%s
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: %s = serde_json::from_str(text).expect("the committed catalogue snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed catalogue snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed catalogue snapshot reparses");
        assert_eq!(reencoded, original, "%s: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the %s payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the %s payload reparses");
    assert_eq!(reencoded, original, "%s: the committed %s JSON is not canonical");
}""" % (
            doc("\U0001f523️", "Both committed snapshots and the committed `%s` payload are already canonical: decode → encode is a fixed point. The committed payload is spelled %s." % (kind, case["wire_prose"])),
            SNAPSHOT_TY, label, kind, kind, label, kind,
        ),
        """%s
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "%s: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "%s: %s");
    assert!(produced.messages().is_empty(), "%s: an accepted %s emits no diagnostics at all");
}""" % (doc("\U0001f3af️", case["outcome_doc"]), label, label, case["guard_prose"], label, kind),
        """%s
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced %s diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "%s: the produced diff differs from the committed \U0001f53a️diff/\U0001f523️component.json");
}""" % (
            doc("\U0001f53a️", "The sparse delta `%s` produces is exactly the committed diff — the load-bearing assertion of this fixture: `Iso16757Diff` is a per-CONTAINER delta, so this pins that only `%s` is rewritten and the other eight containers stay `null`." % (kind, case["diff_key"])),
            kind, label,
        ),
        """%s
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: %s = serde_json::from_str(DIFF).expect("the committed %s diff decodes");
%s
%s
    assert!(decoded.artifact.is_none(), "%s: a container-scoped mutation must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "%s: the committed diff JSON is not canonical");
}""" % (
            doc("\U0001f523️", "The committed diff decodes to `Iso16757Diff`, re-encodes unchanged, and carries %s" % case["diff_prose"]),
            DIFF_TY, kind,
            indent(case["diff_extra"]),
            indent(["assert!(decoded.%s.is_none(), \"%s: %s writes `%s` and must leave `%s` untouched\");" % (other, label, kind, case["diff_key"], other) for other in case["diff_none_fields"]]),
            label, label,
        ),
        """%s
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: %s = serde_json::from_str(DIFF).expect("the committed %s diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "%s: the committed diff did not carry before to after");
}""" % (
            doc("🩹", "The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete description of the %s, not a summary of it." % case["change_noun"]),
            DIFF_TY, kind, label,
        ),
    ]

    rust = common.render_test(
        artifact=ARTIFACT,
        types=(SNAPSHOT_TY, MUTATION_TY, DIFF_TY),
        kind=kind,
        case=name,
        header_note=[
            "`Iso16757Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).",
            "`%s` never writes it, so it stays `None` and rides the JSON round trip as a plain `null`;" % kind,
            "the nested states `None` and `Some(None)` are NOT distinguishable in this file's committed diff,",
            "and nothing here asserts that they are.",
        ],
        tests=tests,
    )
    common.emit_case(
        ARTIFACT_DIR,
        common.resolve_leaf_dir(ARTIFACT_DIR, kind),
        name,
        before=BASE,
        after=after,
        mutation=case["mutation"],
        diff=diff,
        outcome={"status": "applied"},
        rust=rust,
    )


# ---------------------------------------------------------------- per-case mutate helpers


def _set_pn_input(snapshot):
    snapshot["partNumberInputs"]["height"] = {"kind": "decimal", "value": 750.0}


def _drop_pn_input(snapshot):
    del snapshot["partNumberInputs"]["length"]


def _rename_catalogue(snapshot):
    snapshot["catalogue"]["metadata"]["names"]["preferred"]["text"] = "Fixture Radiator Catalogue 2026"


def _rename_manufacturer(snapshot):
    snapshot["catalogue"]["manufacturer"]["names"]["preferred"]["text"] = "Fixture Heating Works AG"


def _selection_class(snapshot):
    snapshot["selection"]["class_id"] = "class.towel-radiator"


def _selection_series(snapshot):
    snapshot["selection"]["series_id"] = "series.pr-plus"


def _add_constraint(snapshot):
    snapshot["selection"]["constraints"].append(copy.deepcopy(NEW_CONSTRAINT))


def _remove_constraint(snapshot):
    del snapshot["selection"]["constraints"][1]


def _create_subject(snapshot):
    snapshot["dictionary"]["subjects"].append(copy.deepcopy(NEW_SUBJECT))


def _delete_subject(snapshot):
    snapshot["dictionary"]["subjects"] = []


def _script_limits(snapshot):
    snapshot["scriptLimits"] = {"max_steps": 20000, "max_recursion": 128, "timeout_ms": 250}


def _delete_product(snapshot):
    snapshot["catalogue"]["products"] = []


def _delete_product_group(snapshot):
    snapshot["catalogue"]["product_groups"] = []


def _delete_property_definition(snapshot):
    snapshot["catalogue"]["property_definitions"] = []


def _create_property_definition(snapshot):
    snapshot["catalogue"]["property_definitions"].append(copy.deepcopy(NEW_PROPERTY_DEFINITION))


def _create_product_group(snapshot):
    snapshot["catalogue"]["product_groups"].append(copy.deepcopy(NEW_PRODUCT_GROUP))


def _create_product(snapshot):
    snapshot["catalogue"]["products"].append(copy.deepcopy(NEW_PRODUCT))


def _replace_rule(snapshot):
    snapshot["partNumberRule"] = {"kind": "script", "function_id": "partno", "source": "height"}


def _exchange_process(snapshot):
    snapshot["exchangeProcess"] = "DetermineProduct"


def _rename_product_group(snapshot):
    snapshot["catalogue"]["product_groups"][0]["names"]["preferred"]["text"] = "Panel radiators"


def _rename_product(snapshot):
    snapshot["catalogue"]["products"][0]["names"]["preferred"]["text"] = "PR-600 Compact"


CASES = [
    {
        "kind": "change-part-number-input",
        "case": "raises-the-height-part-number-input-to-750",
        "mutation": {"ChangePartNumberInput": {"key": "height", "new_value": {"kind": "decimal", "value": 750.0}}},
        "mutate": _set_pn_input, "diff_key": "partNumberInputs",
        "applies_fn": "raises_the_height_part_number_input_to_750",
        "applies_doc": "The oracle clones the WHOLE `part_number_inputs` map and inserts over the addressed key, so `height` moves to 750.0 while the sibling `length` entry rides through byte-identical — this is an insert-over-clone, not a map replacement.",
        "applies_extra": [
            'assert_eq!(applied.part_number_inputs.get("height"), Some(&%s::CatalogueValue::Decimal { value: 750.0 }), "change-part-number-input/raises-the-height-part-number-input-to-750: the addressed key must hold 750.0");' % ISO,
            'assert_eq!(applied.part_number_inputs.get("length"), before().part_number_inputs.get("length"), "change-part-number-input/raises-the-height-part-number-input-to-750: the untargeted `length` input must survive the clone-and-insert unchanged");',
            'assert_eq!(applied.part_number_inputs.len(), 2, "change-part-number-input/raises-the-height-part-number-input-to-750: writing over an EXISTING key must not grow the map");',
        ],
        "inverse_fn": "restoring_the_600_height_input_restores_before",
        "inverse_doc": "`change-part-number-input`'s inverse branches on whether the key already existed: `height` does, so it yields one `ChangePartNumberInput` carrying the OLD 600.0 — not the `RemovePartNumberInput` it would emit for a fresh key.",
        "inverse_shape": "the existing-key branch of the inverse yields exactly one ChangePartNumberInput back",
        "inverse_restores": "replaying the 600.0 value did not restore the before-snapshot",
        "outcome_doc": "The committed `height` input is 600.0, so the `Some(&payload.new_value)` equality guard does not match 750.0 and no `mutation.no-op` warning is raised.",
        "guard_prose": "750.0 differs from the committed 600.0, so the `base.part_number_inputs.get(key) == Some(&new_value)` guard cannot fire",
        "wire_prose": "`{\"ChangePartNumberInput\": {\"key\": …, \"new_value\": {\"kind\": \"decimal\", …}}}` — externally tagged variant, snake_case payload keys, and an internally `kind`-tagged CatalogueValue",
        "diff_prose": "the whole rewritten part-number input map and nothing else.",
        "diff_extra": [
            'let inputs = decoded.part_number_inputs.as_ref().expect("the committed change-part-number-input diff carries the input map");',
            'assert_eq!(inputs.len(), 2, "change-part-number-input/raises-the-height-part-number-input-to-750: the diff carries BOTH inputs, because this container delta is a whole-map replacement");',
            'assert_eq!(inputs.get("height"), Some(&%s::CatalogueValue::Decimal { value: 750.0 }), "change-part-number-input/raises-the-height-part-number-input-to-750: the diff must carry the new 750.0 height");' % ISO,
        ],
        "diff_none_fields": ["catalogue", "dictionary", "selection", "part_number_rule"],
        "change_noun": "part-number input change",
    },
    {
        "kind": "remove-part-number-input",
        "case": "drops-the-length-part-number-input",
        "mutation": {"RemovePartNumberInput": {"key": "length"}},
        "mutate": _drop_pn_input, "diff_key": "partNumberInputs",
        "applies_fn": "drops_the_length_part_number_input",
        "applies_doc": "The oracle clones the map and removes the addressed key, so `length` disappears and `height` survives — the diff is still the whole remaining map, which is why the `after` map is one entry shorter rather than carrying a tombstone.",
        "applies_extra": [
            'assert_eq!(applied.part_number_inputs.len(), 1, "remove-part-number-input/drops-the-length-part-number-input: exactly one input must remain");',
            'assert!(!applied.part_number_inputs.contains_key("length"), "remove-part-number-input/drops-the-length-part-number-input: the addressed key must be gone");',
            'assert_eq!(applied.part_number_inputs.get("height"), before().part_number_inputs.get("height"), "remove-part-number-input/drops-the-length-part-number-input: the untargeted `height` input must survive");',
        ],
        "inverse_fn": "reinserting_the_length_input_restores_before",
        "inverse_doc": "`remove-part-number-input`'s inverse reads the OLD value out of BASE and yields one `ChangePartNumberInput`, which re-inserts it; because `part_number_inputs` is a `BTreeMap`, re-inserting also restores the key ORDER, not just the contents.",
        "inverse_shape": "removing an existing key inverts to exactly one ChangePartNumberInput that puts it back",
        "inverse_restores": "re-inserting `length` did not restore the before-snapshot",
        "outcome_doc": "`length` IS present in the committed inputs, so the `contains_key` guard passes and the `mutation.target-missing` Error branch is not taken.",
        "guard_prose": "the committed map contains `length`, so `remove-part-number-input`'s `mutation.target-missing` error branch cannot fire",
        "wire_prose": "`{\"RemovePartNumberInput\": {\"key\": \"length\"}}` — externally tagged, snake_case payload key",
        "diff_prose": "the whole surviving part-number input map and nothing else.",
        "diff_extra": [
            'let inputs = decoded.part_number_inputs.as_ref().expect("the committed remove-part-number-input diff carries the input map");',
            'assert_eq!(inputs.len(), 1, "remove-part-number-input/drops-the-length-part-number-input: a removal is expressed as the SHORTER whole map, never as a delete marker");',
            'assert!(!inputs.contains_key("length"), "remove-part-number-input/drops-the-length-part-number-input: the removed key must not appear in the diff");',
        ],
        "diff_none_fields": ["catalogue", "dictionary", "selection", "script_limits"],
        "change_noun": "part-number input removal",
    },
    {
        "kind": "rename-catalogue",
        "case": "restamps-the-catalogue-as-the-2026-edition",
        "mutation": {"RenameCatalogue": {"new_name": "Fixture Radiator Catalogue 2026"}},
        "mutate": _rename_catalogue, "diff_key": "catalogue",
        "applies_fn": "restamps_the_catalogue_as_the_2026_edition",
        "applies_doc": "`rename-catalogue` writes `catalogue.metadata.names.preferred.text` and nothing else — the German alternative name, the short name, the lifecycle revision and the catalogue ID are all carried through the whole-catalogue clone untouched.",
        "applies_extra": [
            'assert_eq!(applied.catalogue.metadata.names.preferred.text, "Fixture Radiator Catalogue 2026", "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: the preferred name must be restamped");',
            'assert_eq!(applied.catalogue.metadata.names.alternatives, before().catalogue.metadata.names.alternatives, "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: the German alternative name is a separate locale entry and must not be rewritten");',
            'assert_eq!(applied.catalogue.id, before().catalogue.id, "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: renaming must never re-mint the catalogue identifier");',
        ],
        "inverse_fn": "renaming_back_restores_before",
        "inverse_doc": "`rename-catalogue`'s inverse reads the OLD preferred text out of BASE, so replaying it puts \"Fixture Radiator Catalogue\" back on the metadata.",
        "inverse_shape": "the inverse of one catalogue rename is exactly one rename back",
        "inverse_restores": "renaming back did not restore the before-snapshot",
        "outcome_doc": "\"Fixture Radiator Catalogue 2026\" differs from the committed \"Fixture Radiator Catalogue\", so the equality guard on `metadata.names.preferred.text` does not degrade this to a `mutation.no-op` warning.",
        "guard_prose": "the new name differs from the committed preferred text, so `rename-catalogue`'s `mutation.no-op` guard cannot fire",
        "wire_prose": "`{\"RenameCatalogue\": {\"new_name\": …}}` — externally tagged, snake_case payload key",
        "diff_prose": "the whole rewritten catalogue and nothing else.",
        "diff_extra": [
            'let catalogue = decoded.catalogue.as_ref().expect("the committed rename-catalogue diff carries the catalogue");',
            'assert_eq!(catalogue.metadata.names.preferred.text, "Fixture Radiator Catalogue 2026", "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: the diff must carry the new preferred name");',
            'assert_eq!(catalogue.products.len(), 1, "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: the catalogue delta is whole-container, so the untouched product list rides along in full");',
        ],
        "diff_none_fields": ["dictionary", "selection", "part_number_inputs", "exchange_process"],
        "change_noun": "catalogue rename",
    },
    {
        "kind": "rename-manufacturer",
        "case": "adds-the-ag-suffix-to-the-manufacturer",
        "mutation": {"RenameManufacturer": {"new_name": "Fixture Heating Works AG"}},
        "mutate": _rename_manufacturer, "diff_key": "catalogue",
        "applies_fn": "adds_the_ag_suffix_to_the_manufacturer",
        "applies_doc": "`rename-manufacturer` reaches into `catalogue.manufacturer.names.preferred.text` — a DIFFERENT `Names` from the catalogue's own metadata — so the catalogue title must be visibly unaffected. That distinction is what this case pins.",
        "applies_extra": [
            'assert_eq!(applied.catalogue.manufacturer.names.preferred.text, "Fixture Heating Works AG", "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: the manufacturer name must gain the AG suffix");',
            'assert_eq!(applied.catalogue.metadata.names.preferred.text, before().catalogue.metadata.names.preferred.text, "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: the CATALOGUE title is a different Names and must not be touched");',
            'assert_eq!(applied.catalogue.manufacturer.id, before().catalogue.manufacturer.id, "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: a rename must not re-mint the manufacturer id");',
        ],
        "inverse_fn": "dropping_the_ag_suffix_restores_before",
        "inverse_doc": "`rename-manufacturer`'s inverse reads the OLD manufacturer text out of BASE, so replaying it puts \"Fixture Heating Works\" back.",
        "inverse_shape": "the inverse of one manufacturer rename is exactly one rename back",
        "inverse_restores": "dropping the AG suffix again did not restore the before-snapshot",
        "outcome_doc": "\"Fixture Heating Works AG\" differs from the committed \"Fixture Heating Works\", so the equality guard on the MANUFACTURER's preferred text stays shut.",
        "guard_prose": "the new name differs from the committed manufacturer text, so `rename-manufacturer`'s `mutation.no-op` guard cannot fire",
        "wire_prose": "`{\"RenameManufacturer\": {\"new_name\": …}}` — externally tagged, snake_case payload key",
        "diff_prose": "the whole rewritten catalogue and nothing else.",
        "diff_extra": [
            'let catalogue = decoded.catalogue.as_ref().expect("the committed rename-manufacturer diff carries the catalogue");',
            'assert_eq!(catalogue.manufacturer.names.preferred.text, "Fixture Heating Works AG", "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: the diff must carry the new manufacturer name");',
            'assert_eq!(catalogue.metadata.names.preferred.text, "Fixture Radiator Catalogue", "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: the catalogue title must ride through the diff unchanged");',
        ],
        "diff_none_fields": ["dictionary", "selection", "script_limits", "exchange_process"],
        "change_noun": "manufacturer rename",
    },
    {
        "kind": "change-selection-class",
        "case": "retargets-the-selection-at-the-towel-radiator-class",
        "mutation": {"ChangeSelectionClass": {"new_class_id": "class.towel-radiator"}},
        "mutate": _selection_class, "diff_key": "selection",
        "applies_fn": "retargets_the_selection_at_the_towel_radiator_class",
        "applies_doc": "`change-selection-class` clones the `SelectionRequest` and writes only `class_id`. The new id does NOT exist in `catalogue.product_classes`, and the oracle has no referential guard — a selection may legitimately be retargeted before the class is created, so this must apply cleanly.",
        "applies_extra": [
            'assert_eq!(applied.selection.class_id, "class.towel-radiator", "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: the selection class must be retargeted");',
            'assert_eq!(applied.selection.constraints, before().selection.constraints, "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: both existing constraints must ride through the clone");',
            'assert_eq!(applied.catalogue.product_classes.len(), 1, "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: retargeting at an id that does not exist yet must not create a class");',
        ],
        "inverse_fn": "retargeting_at_the_panel_radiator_class_restores_before",
        "inverse_doc": "`change-selection-class`'s inverse reads the OLD `class_id` out of BASE, so replaying it points the request back at `class.panel-radiator`.",
        "inverse_shape": "the inverse of one selection-class change is exactly one change back",
        "inverse_restores": "pointing the request back at the panel-radiator class did not restore the before-snapshot",
        "outcome_doc": "\"class.towel-radiator\" differs from the committed \"class.panel-radiator\", so the equality guard stays shut. There is deliberately no existence check on the target class.",
        "guard_prose": "the new class id differs from the committed one, so `change-selection-class`'s `mutation.no-op` guard cannot fire",
        "wire_prose": "`{\"ChangeSelectionClass\": {\"new_class_id\": …}}` — externally tagged, snake_case payload key",
        "diff_prose": "the whole rewritten selection request and nothing else.",
        "diff_extra": [
            'let selection = decoded.selection.as_ref().expect("the committed change-selection-class diff carries the selection request");',
            'assert_eq!(selection.class_id, "class.towel-radiator", "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: the diff must carry the new class id");',
            'assert_eq!(selection.constraints.len(), 2, "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: the selection delta is whole-container, so both constraints ride along");',
        ],
        "diff_none_fields": ["catalogue", "dictionary", "part_number_inputs", "exchange_process"],
        "change_noun": "selection-class change",
    },
    {
        "kind": "change-selection-series",
        "case": "narrows-the-selection-to-the-pr-plus-series",
        "mutation": {"ChangeSelectionSeries": {"new_series_id": "series.pr-plus"}},
        "mutate": _selection_series, "diff_key": "selection",
        "applies_fn": "narrows_the_selection_to_the_pr_plus_series",
        "applies_doc": "`change-selection-series` writes `selection.series_id`, whose payload type is `Option<String>` — this case carries `Some`, so it swaps one series id for another rather than clearing the field.",
        "applies_extra": [
            'assert_eq!(applied.selection.series_id.as_deref(), Some("series.pr-plus"), "change-selection-series/narrows-the-selection-to-the-pr-plus-series: the selection series must be narrowed");',
            'assert_eq!(applied.selection.class_id, before().selection.class_id, "change-selection-series/narrows-the-selection-to-the-pr-plus-series: the class id is a sibling field of the same request and must not move");',
            'assert_eq!(applied.selection.constraints.len(), 2, "change-selection-series/narrows-the-selection-to-the-pr-plus-series: narrowing by series must not drop property constraints");',
        ],
        "inverse_fn": "widening_back_to_the_pr_series_restores_before",
        "inverse_doc": "`change-selection-series`'s inverse reads the OLD `Option<String>` out of BASE and replays it wholesale, so a `Some(\"series.pr\")` goes back exactly as it was — the same code path that would restore a `None`.",
        "inverse_shape": "the inverse of one selection-series change is exactly one change back",
        "inverse_restores": "widening back to `series.pr` did not restore the before-snapshot",
        "outcome_doc": "`Some(\"series.pr-plus\")` differs from the committed `Some(\"series.pr\")`, so the `Option`-level equality guard stays shut.",
        "guard_prose": "the guard compares whole `Option<String>` values and the two differ, so `change-selection-series`'s `mutation.no-op` warning cannot fire",
        "wire_prose": "`{\"ChangeSelectionSeries\": {\"new_series_id\": \"series.pr-plus\"}}` — the payload field is an `Option<String>` with no `skip_serializing_if`, so a cleared series would encode as an explicit `null` here",
        "diff_prose": "the whole rewritten selection request and nothing else.",
        "diff_extra": [
            'let selection = decoded.selection.as_ref().expect("the committed change-selection-series diff carries the selection request");',
            'assert_eq!(selection.series_id.as_deref(), Some("series.pr-plus"), "change-selection-series/narrows-the-selection-to-the-pr-plus-series: the diff must carry the new series id");',
            'assert_eq!(selection.class_id, "class.panel-radiator", "change-selection-series/narrows-the-selection-to-the-pr-plus-series: the class id rides through the whole-container delta unchanged");',
        ],
        "diff_none_fields": ["catalogue", "dictionary", "part_number_rule", "script_limits"],
        "change_noun": "selection-series change",
    },
    {
        "kind": "add-selection-constraint",
        "case": "appends-a-width-under-800-constraint",
        "mutation": {"AddSelectionConstraint": {"constraint": NEW_CONSTRAINT}},
        "mutate": _add_constraint, "diff_key": "selection",
        "applies_fn": "appends_a_width_under_800_constraint",
        "applies_doc": "`add-selection-constraint` PUSHES onto `selection.constraints`, so the new width constraint lands at index 2, after the committed height and length constraints, and the existing two keep their positions.",
        "applies_extra": [
            'assert_eq!(applied.selection.constraints.len(), 3, "add-selection-constraint/appends-a-width-under-800-constraint: the constraint list must grow by exactly one");',
            'assert_eq!(applied.selection.constraints[2].property_id, "prop.width", "add-selection-constraint/appends-a-width-under-800-constraint: the new constraint is APPENDED, so it must land at index 2");',
            'assert_eq!(applied.selection.constraints[0], before().selection.constraints[0], "add-selection-constraint/appends-a-width-under-800-constraint: the pre-existing height constraint must keep both its value and its position");',
        ],
        "inverse_fn": "removing_the_appended_constraint_restores_before",
        "inverse_doc": "`add-selection-constraint`'s inverse is a `RemoveSelectionConstraint` addressed at `base.selection.constraints.len()` — index 2 here — which is exactly where the push landed the new entry.",
        "inverse_shape": "an append inverts to exactly one RemoveSelectionConstraint at the pre-append length",
        "inverse_restores": "removing the appended constraint did not restore the before-snapshot",
        "outcome_doc": "The committed constraints are on `prop.height` and `prop.length`; a `prop.width` constraint is not `contains`-equal to either, so the `mutation.no-op` duplicate guard stays shut.",
        "guard_prose": "the new constraint is not already in the committed list, so `add-selection-constraint`'s `mutation.no-op` guard cannot fire",
        "wire_prose": "`{\"AddSelectionConstraint\": {\"constraint\": {\"property_id\": …, \"operator\": \"LessThan\", \"value\": {\"kind\": \"decimal\", …}}}}` — `ConstraintOperator` carries no serde rename, so it is the bare Rust variant name",
        "diff_prose": "the whole rewritten selection request and nothing else.",
        "diff_extra": [
            'let selection = decoded.selection.as_ref().expect("the committed add-selection-constraint diff carries the selection request");',
            'assert_eq!(selection.constraints.len(), 3, "add-selection-constraint/appends-a-width-under-800-constraint: the diff carries the whole three-entry list, not just the appended entry");',
            'assert_eq!(selection.constraints[2].property_id, "prop.width", "add-selection-constraint/appends-a-width-under-800-constraint: the appended constraint must be last in the diff too");',
        ],
        "diff_none_fields": ["catalogue", "dictionary", "part_number_inputs", "exchange_process"],
        "change_noun": "constraint append",
    },
    {
        "kind": "remove-selection-constraint",
        "case": "drops-the-trailing-length-constraint",
        "mutation": {"RemoveSelectionConstraint": {"index": 1}},
        "mutate": _remove_constraint, "diff_key": "selection",
        "applies_fn": "drops_the_trailing_length_constraint",
        "applies_doc": "`remove-selection-constraint` is INDEX-addressed, not id-addressed. This case removes index 1 — the trailing length constraint — leaving the height constraint at index 0.",
        "applies_extra": [
            'assert_eq!(applied.selection.constraints.len(), 1, "remove-selection-constraint/drops-the-trailing-length-constraint: the constraint list must shrink by exactly one");',
            'assert_eq!(applied.selection.constraints[0].property_id, "prop.height", "remove-selection-constraint/drops-the-trailing-length-constraint: the surviving constraint is the one that was at index 0");',
            'assert_eq!(applied.selection.class_id, before().selection.class_id, "remove-selection-constraint/drops-the-trailing-length-constraint: removing a constraint must not retarget the request");',
        ],
        "inverse_fn": "re_appending_the_length_constraint_restores_before",
        "inverse_doc": "`remove-selection-constraint`'s inverse is an `AddSelectionConstraint`, which PUSHES — so it restores the removed entry only because this case deliberately removes the LAST constraint. Removing an interior index would invert to a different order, and this fixture pins the boundary the inverse is exact at.",
        "inverse_shape": "removing the trailing constraint inverts to exactly one AddSelectionConstraint that pushes it back",
        "inverse_restores": "re-appending the length constraint did not restore the before-snapshot — the push-based inverse is exact only for the trailing index",
        "outcome_doc": "Index 1 is inside the committed two-entry list, so the `index >= len` bound check does not take the `mutation.target-missing` Error branch.",
        "guard_prose": "index 1 is within the committed two-entry constraint list, so `remove-selection-constraint`'s `mutation.target-missing` error cannot fire",
        "wire_prose": "`{\"RemoveSelectionConstraint\": {\"index\": 1}}` — a bare JSON integer, because the payload field is a `usize`",
        "diff_prose": "the whole surviving selection request and nothing else.",
        "diff_extra": [
            'let selection = decoded.selection.as_ref().expect("the committed remove-selection-constraint diff carries the selection request");',
            'assert_eq!(selection.constraints.len(), 1, "remove-selection-constraint/drops-the-trailing-length-constraint: a removal is expressed as the SHORTER whole list, never as an index marker");',
            'assert_eq!(selection.constraints[0].property_id, "prop.height", "remove-selection-constraint/drops-the-trailing-length-constraint: the surviving constraint must be the height one");',
        ],
        "diff_none_fields": ["catalogue", "dictionary", "part_number_inputs", "script_limits"],
        "change_noun": "constraint removal",
    },
    {
        "kind": "create-subject",
        "case": "appends-a-towel-radiator-subject-under-the-radiator-parent",
        "mutation": {"CreateSubject": {"subject": NEW_SUBJECT, "index": None}},
        "mutate": _create_subject, "diff_key": "dictionary",
        "applies_fn": "appends_a_towel_radiator_subject_under_the_radiator_parent",
        "applies_doc": "With `index: None` the oracle takes its `_ => push` arm, so the new subject is appended after `subject.radiator` and the `clamped` flag stays false. The subject declares `parent_id: Some(\"subject.radiator\")`, which the oracle stores verbatim without validating the parent.",
        "applies_extra": [
            'assert_eq!(applied.dictionary.subjects.len(), 2, "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: the subject list must grow by exactly one");',
            'assert_eq!(applied.dictionary.subjects[1].id, "subject.towel-radiator", "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: a null index appends, so the new subject must be last");',
            'assert_eq!(applied.dictionary.subjects[1].parent_id.as_deref(), Some("subject.radiator"), "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: the declared parent must be stored verbatim");',
        ],
        "inverse_fn": "deleting_the_towel_radiator_subject_restores_before",
        "inverse_doc": "`create-subject`'s inverse is a `DeleteSubject` on the created id — but only when that id is ABSENT from BASE; the fixture's id is fresh, so exactly one delete step is produced.",
        "inverse_shape": "creating a fresh subject inverts to exactly one DeleteSubject",
        "inverse_restores": "deleting the created subject did not restore the before-snapshot",
        "outcome_doc": "`subject.towel-radiator` is not among the committed subjects, so the fatal `mutation.duplicate-id` branch is not taken; and `index` is `None`, so the `mutation.clamped` warning is not raised either.",
        "guard_prose": "the id is fresh (no `mutation.duplicate-id`) and the index is null rather than out of range (no `mutation.clamped`)",
        "wire_prose": "`{\"CreateSubject\": {\"subject\": {…}, \"index\": null}}` — `index` is an `Option<usize>` with no `skip_serializing_if`, so it is present as an explicit `null`, and `SubjectKind::ProductSpecialization` is spelled with its bare Rust variant name",
        "diff_prose": "the whole rewritten dictionary and nothing else.",
        "diff_extra": [
            'let dictionary = decoded.dictionary.as_ref().expect("the committed create-subject diff carries the dictionary");',
            'assert_eq!(dictionary.subjects.len(), 2, "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: the diff carries both subjects, because the dictionary delta is whole-container");',
            'assert_eq!(dictionary.subjects[1].kind, %s::part_4::SubjectKind::ProductSpecialization, "create-subject/appends-a-towel-radiator-subject-under-the-radiator-parent: the declared subject kind must survive the diff");' % ISO,
        ],
        "diff_none_fields": ["catalogue", "selection", "part_number_inputs", "exchange_process"],
        "change_noun": "subject creation",
    },
    {
        "kind": "delete-subject",
        "case": "removes-the-radiator-subject-from-the-dictionary",
        "mutation": {"DeleteSubject": {"id": "subject.radiator"}},
        "mutate": _delete_subject, "diff_key": "dictionary",
        "applies_fn": "removes_the_radiator_subject_from_the_dictionary",
        "applies_doc": "`delete-subject` retains everything whose id differs, emptying the list here. The catalogue's `product_groups[0].dictionary_subject_id` still points at the deleted subject — the oracle performs NO cascade, and this case is the pin on that: a dangling dictionary reference is a validation concern, not a mutation one.",
        "applies_extra": [
            'assert!(applied.dictionary.subjects.is_empty(), "delete-subject/removes-the-radiator-subject-from-the-dictionary: the addressed subject must be gone");',
            'assert_eq!(applied.catalogue.product_groups[0].dictionary_subject_id.as_deref(), Some("subject.radiator"), "delete-subject/removes-the-radiator-subject-from-the-dictionary: the oracle severs no catalogue reference, so the now-dangling pointer must survive verbatim");',
            'assert_eq!(applied.dictionary.reference, before().dictionary.reference, "delete-subject/removes-the-radiator-subject-from-the-dictionary: the dictionary identity is not part of the deletion");',
        ],
        "inverse_fn": "recreating_the_radiator_subject_restores_before",
        "inverse_doc": "`delete-subject`'s inverse is a `CreateSubject` carrying the removed subject AND its recorded position (`index: Some(0)`), which is what makes the round trip order-exact rather than merely set-exact.",
        "inverse_shape": "deleting an existing subject inverts to exactly one positioned CreateSubject",
        "inverse_restores": "recreating the subject at its recorded index did not restore the before-snapshot",
        "outcome_doc": "`subject.radiator` IS among the committed subjects, so the `mutation.target-missing` Error branch is not taken.",
        "guard_prose": "the addressed subject exists in the committed dictionary, so `delete-subject`'s `mutation.target-missing` error cannot fire",
        "wire_prose": "`{\"DeleteSubject\": {\"id\": \"subject.radiator\"}}` — externally tagged, snake_case payload key",
        "diff_prose": "the whole emptied dictionary and nothing else.",
        "diff_extra": [
            'let dictionary = decoded.dictionary.as_ref().expect("the committed delete-subject diff carries the dictionary");',
            'assert!(dictionary.subjects.is_empty(), "delete-subject/removes-the-radiator-subject-from-the-dictionary: the deletion is expressed as the shorter whole subject list");',
            'assert_eq!(dictionary.reference.id, "bsdd.fixture", "delete-subject/removes-the-radiator-subject-from-the-dictionary: the dictionary reference rides through the whole-container delta");',
        ],
        "diff_none_fields": ["catalogue", "selection", "part_number_rule", "script_limits"],
        "change_noun": "subject deletion",
    },
    {
        "kind": "update-script-limits",
        "case": "doubles-the-step-budget-and-quintuples-the-timeout",
        "mutation": {"UpdateScriptLimits": {"new_max_steps": 20000, "new_max_recursion": 128, "new_timeout_ms": 250}},
        "mutate": _script_limits, "diff_key": "scriptLimits",
        "applies_fn": "doubles_the_step_budget_and_quintuples_the_timeout",
        "applies_doc": "`update-script-limits` is the tree's one `update-<facet>` verb: it rebuilds the whole `ScriptLimits` triple from three payload fields at once, because the three budgets are validated as one atomic bundle rather than as independent rows.",
        "applies_extra": [
            'assert_eq!(applied.script_limits.max_steps, 20000, "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: the step budget must double");',
            'assert_eq!(applied.script_limits.max_recursion, 128, "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: the recursion budget must double");',
            'assert_eq!(applied.script_limits.timeout_ms, 250, "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: the timeout must reach 250 ms");',
        ],
        "inverse_fn": "restoring_the_default_budgets_restores_before",
        "inverse_doc": "`update-script-limits`'s inverse reads all three OLD budgets out of BASE into one `UpdateScriptLimits`, so the atomic bundle is restored atomically.",
        "inverse_shape": "the inverse of one script-limits update is exactly one update back",
        "inverse_restores": "restoring the 10000/64/50 budgets did not restore the before-snapshot",
        "outcome_doc": "The oracle builds the candidate `ScriptLimits` first and compares the WHOLE struct; 20000/128/250 differs from the committed 10000/64/50, so `mutation.no-op` stays shut.",
        "guard_prose": "the rebuilt ScriptLimits differs from the committed 10000/64/50 triple, so the whole-struct `mutation.no-op` guard cannot fire",
        "wire_prose": "`{\"UpdateScriptLimits\": {\"new_max_steps\": …, \"new_max_recursion\": …, \"new_timeout_ms\": …}}` — three bare JSON integers (`u32`, `u32`, `u64`), snake_case",
        "diff_prose": "the rebuilt script-limits triple and nothing else.",
        "diff_extra": [
            'let limits = decoded.script_limits.as_ref().expect("the committed update-script-limits diff carries the limits");',
            'assert_eq!((limits.max_steps, limits.max_recursion, limits.timeout_ms), (20000, 128, 250), "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: the diff must carry all three new budgets together");',
        ],
        "diff_none_fields": ["catalogue", "dictionary", "selection", "part_number_rule"],
        "change_noun": "script-limits update",
    },
    {
        "kind": "delete-product",
        "case": "removes-the-pr600-product-from-the-catalogue",
        "mutation": {"DeleteProduct": {"id": "product.pr600"}},
        "mutate": _delete_product, "diff_key": "catalogue",
        "applies_fn": "removes_the_pr600_product_from_the_catalogue",
        "applies_doc": "`delete-product` retains everything whose id differs, emptying the product list. The series the product belonged to is NOT deleted with it — the oracle touches only `catalogue.products`.",
        "applies_extra": [
            'assert!(applied.catalogue.products.is_empty(), "delete-product/removes-the-pr600-product-from-the-catalogue: the addressed product must be gone");',
            'assert_eq!(applied.catalogue.product_series.len(), 1, "delete-product/removes-the-pr600-product-from-the-catalogue: deleting a product must not cascade into its series");',
            'assert_eq!(applied.catalogue.product_groups.len(), 1, "delete-product/removes-the-pr600-product-from-the-catalogue: nor into the group above it");',
        ],
        "inverse_fn": "recreating_the_pr600_product_restores_before",
        "inverse_doc": "`delete-product`'s inverse is a `CreateProduct` carrying the removed product AND its recorded position (`index: Some(0)`), so the round trip restores order as well as content.",
        "inverse_shape": "deleting an existing product inverts to exactly one positioned CreateProduct",
        "inverse_restores": "recreating the product at its recorded index did not restore the before-snapshot",
        "outcome_doc": "`product.pr600` IS in the committed catalogue, so the `mutation.target-missing` Error branch is not taken.",
        "guard_prose": "the addressed product exists in the committed catalogue, so `delete-product`'s `mutation.target-missing` error cannot fire",
        "wire_prose": "`{\"DeleteProduct\": {\"id\": \"product.pr600\"}}` — externally tagged, snake_case payload key",
        "diff_prose": "the whole rewritten catalogue and nothing else.",
        "diff_extra": [
            'let catalogue = decoded.catalogue.as_ref().expect("the committed delete-product diff carries the catalogue");',
            'assert!(catalogue.products.is_empty(), "delete-product/removes-the-pr600-product-from-the-catalogue: the deletion is expressed as the shorter whole product list");',
            'assert_eq!(catalogue.product_series.len(), 1, "delete-product/removes-the-pr600-product-from-the-catalogue: the untouched series list rides along in the whole-container delta");',
        ],
        "diff_none_fields": ["dictionary", "selection", "part_number_inputs", "script_limits"],
        "change_noun": "product deletion",
    },
    {
        "kind": "delete-product-group",
        "case": "removes-the-radiators-group-and-strands-its-class",
        "mutation": {"DeleteProductGroup": {"id": "group.radiators"}},
        "mutate": _delete_product_group, "diff_key": "catalogue",
        "applies_fn": "removes_the_radiators_group_and_strands_its_class",
        "applies_doc": "`delete-product-group` retains everything whose id differs and performs NO cascade: `class.panel-radiator` keeps pointing at the now-absent `group.radiators`. That stranded reference is deliberate and is what this case pins.",
        "applies_extra": [
            'assert!(applied.catalogue.product_groups.is_empty(), "delete-product-group/removes-the-radiators-group-and-strands-its-class: the addressed group must be gone");',
            'assert_eq!(applied.catalogue.product_classes[0].group_id, "group.radiators", "delete-product-group/removes-the-radiators-group-and-strands-its-class: the oracle severs nothing, so the class keeps its now-dangling group_id");',
            'assert_eq!(applied.catalogue.products.len(), 1, "delete-product-group/removes-the-radiators-group-and-strands-its-class: no product is deleted along with the group");',
        ],
        "inverse_fn": "recreating_the_radiators_group_restores_before",
        "inverse_doc": "`delete-product-group`'s inverse is a `CreateProductGroup` carrying the removed group AND its recorded index, so the group returns to position 0.",
        "inverse_shape": "deleting an existing group inverts to exactly one positioned CreateProductGroup",
        "inverse_restores": "recreating the group at its recorded index did not restore the before-snapshot",
        "outcome_doc": "`group.radiators` IS in the committed catalogue, so the `mutation.target-missing` Error branch is not taken — and no extra message is raised for the class left dangling.",
        "guard_prose": "the addressed group exists in the committed catalogue, so `delete-product-group`'s `mutation.target-missing` error cannot fire",
        "wire_prose": "`{\"DeleteProductGroup\": {\"id\": \"group.radiators\"}}` — externally tagged, snake_case payload key",
        "diff_prose": "the whole rewritten catalogue and nothing else.",
        "diff_extra": [
            'let catalogue = decoded.catalogue.as_ref().expect("the committed delete-product-group diff carries the catalogue");',
            'assert!(catalogue.product_groups.is_empty(), "delete-product-group/removes-the-radiators-group-and-strands-its-class: the deletion is expressed as the shorter whole group list");',
            'assert_eq!(catalogue.product_classes.len(), 1, "delete-product-group/removes-the-radiators-group-and-strands-its-class: the class list must ride through the diff untouched");',
        ],
        "diff_none_fields": ["dictionary", "selection", "part_number_rule", "exchange_process"],
        "change_noun": "product-group deletion",
    },
    {
        "kind": "delete-property-definition",
        "case": "removes-the-height-property-definition",
        "mutation": {"DeletePropertyDefinition": {"id": "prop.height"}},
        "mutate": _delete_property_definition, "diff_key": "catalogue",
        "applies_fn": "removes_the_height_property_definition",
        "applies_doc": "`delete-property-definition` empties `catalogue.property_definitions` here. Both `class.panel-radiator.required_property_ids` and the selection's height constraint still name `prop.height`; neither is cleaned up, because the oracle touches only the definition list.",
        "applies_extra": [
            'assert!(applied.catalogue.property_definitions.is_empty(), "delete-property-definition/removes-the-height-property-definition: the addressed definition must be gone");',
            'assert_eq!(applied.catalogue.product_classes[0].required_property_ids, vec!["prop.height".to_string()], "delete-property-definition/removes-the-height-property-definition: the class keeps requiring a property that no longer exists — no cascade");',
            'assert_eq!(applied.selection.constraints[0].property_id, "prop.height", "delete-property-definition/removes-the-height-property-definition: the selection constraint on the deleted property survives too");',
        ],
        "inverse_fn": "recreating_the_height_property_definition_restores_before",
        "inverse_doc": "`delete-property-definition`'s inverse is a `CreatePropertyDefinition` carrying the removed definition AND its recorded index, so it returns to position 0 with its unit and cardinality intact.",
        "inverse_shape": "deleting an existing definition inverts to exactly one positioned CreatePropertyDefinition",
        "inverse_restores": "recreating the definition at its recorded index did not restore the before-snapshot",
        "outcome_doc": "`prop.height` IS in the committed catalogue, so the `mutation.target-missing` Error branch is not taken.",
        "guard_prose": "the addressed definition exists in the committed catalogue, so `delete-property-definition`'s `mutation.target-missing` error cannot fire",
        "wire_prose": "`{\"DeletePropertyDefinition\": {\"id\": \"prop.height\"}}` — externally tagged, snake_case payload key",
        "diff_prose": "the whole rewritten catalogue and nothing else.",
        "diff_extra": [
            'let catalogue = decoded.catalogue.as_ref().expect("the committed delete-property-definition diff carries the catalogue");',
            'assert!(catalogue.property_definitions.is_empty(), "delete-property-definition/removes-the-height-property-definition: the deletion is expressed as the shorter whole definition list");',
            'assert_eq!(catalogue.product_classes[0].required_property_ids.len(), 1, "delete-property-definition/removes-the-height-property-definition: the class requirement list is untouched inside the same delta");',
        ],
        "diff_none_fields": ["dictionary", "selection", "part_number_inputs", "exchange_process"],
        "change_noun": "property-definition deletion",
    },
    {
        "kind": "create-property-definition",
        "case": "appends-a-selection-scoped-length-property",
        "mutation": {"CreatePropertyDefinition": {"property_definition": NEW_PROPERTY_DEFINITION, "index": None}},
        "mutate": _create_property_definition, "diff_key": "catalogue",
        "applies_fn": "appends_a_selection_scoped_length_property",
        "applies_doc": "With `index: None` the oracle appends, so `prop.length` lands after `prop.height`. It declares `PropertyKind::Selection` and an OPTIONAL cardinality (`min: 0`), which distinguishes it from the committed mandatory `Static` height property.",
        "applies_extra": [
            'assert_eq!(applied.catalogue.property_definitions.len(), 2, "create-property-definition/appends-a-selection-scoped-length-property: the definition list must grow by exactly one");',
            'assert_eq!(applied.catalogue.property_definitions[1].kind, %s::part_1::PropertyKind::Selection, "create-property-definition/appends-a-selection-scoped-length-property: the new definition must keep its Selection kind");' % ISO,
            'assert_eq!(applied.catalogue.property_definitions[1].cardinality.min, 0, "create-property-definition/appends-a-selection-scoped-length-property: the new definition is optional, unlike the committed mandatory height property");',
        ],
        "inverse_fn": "deleting_the_length_property_restores_before",
        "inverse_doc": "`create-property-definition`'s inverse is a `DeletePropertyDefinition` on the created id — produced only because that id is absent from BASE, which it is here.",
        "inverse_shape": "creating a fresh definition inverts to exactly one DeletePropertyDefinition",
        "inverse_restores": "deleting the created definition did not restore the before-snapshot",
        "outcome_doc": "`prop.length` is not among the committed definitions, so the fatal `mutation.duplicate-id` branch is not taken; `index` is `None`, so `mutation.clamped` is not raised either.",
        "guard_prose": "the id is fresh (no `mutation.duplicate-id`) and the index is null rather than out of range (no `mutation.clamped`)",
        "wire_prose": "`{\"CreatePropertyDefinition\": {\"property_definition\": {…}, \"index\": null}}` — the nested `unit`/`cardinality`/`kind` keep snake_case field names and bare Rust enum spellings",
        "diff_prose": "the whole rewritten catalogue and nothing else.",
        "diff_extra": [
            'let catalogue = decoded.catalogue.as_ref().expect("the committed create-property-definition diff carries the catalogue");',
            'assert_eq!(catalogue.property_definitions.len(), 2, "create-property-definition/appends-a-selection-scoped-length-property: the diff carries both definitions, because the catalogue delta is whole-container");',
            'assert_eq!(catalogue.property_definitions[1].id, "prop.length", "create-property-definition/appends-a-selection-scoped-length-property: a null index appends, so the new definition is last in the diff too");',
        ],
        "diff_none_fields": ["dictionary", "selection", "part_number_rule", "script_limits"],
        "change_noun": "property-definition creation",
    },
    {
        "kind": "create-product-group",
        "case": "appends-a-towel-radiators-group",
        "mutation": {"CreateProductGroup": {"product_group": NEW_PRODUCT_GROUP, "index": None}},
        "mutate": _create_product_group, "diff_key": "catalogue",
        "applies_fn": "appends_a_towel_radiators_group",
        "applies_doc": "With `index: None` the oracle appends, so `group.towel-radiators` lands after `group.radiators`. It carries `dictionary_subject_id: None`, i.e. a group may exist before it is mapped onto a dictionary subject.",
        "applies_extra": [
            'assert_eq!(applied.catalogue.product_groups.len(), 2, "create-product-group/appends-a-towel-radiators-group: the group list must grow by exactly one");',
            'assert_eq!(applied.catalogue.product_groups[1].id, "group.towel-radiators", "create-product-group/appends-a-towel-radiators-group: a null index appends, so the new group must be last");',
            'assert!(applied.catalogue.product_groups[1].dictionary_subject_id.is_none(), "create-product-group/appends-a-towel-radiators-group: an unmapped group must stay unmapped, not inherit a subject");',
        ],
        "inverse_fn": "deleting_the_towel_radiators_group_restores_before",
        "inverse_doc": "`create-product-group`'s inverse is a `DeleteProductGroup` on the created id — produced only because that id is absent from BASE, which it is here.",
        "inverse_shape": "creating a fresh group inverts to exactly one DeleteProductGroup",
        "inverse_restores": "deleting the created group did not restore the before-snapshot",
        "outcome_doc": "`group.towel-radiators` is not among the committed groups, so the fatal `mutation.duplicate-id` branch is not taken; `index` is `None`, so `mutation.clamped` is not raised either.",
        "guard_prose": "the id is fresh (no `mutation.duplicate-id`) and the index is null rather than out of range (no `mutation.clamped`)",
        "wire_prose": "`{\"CreateProductGroup\": {\"product_group\": {…}, \"index\": null}}` — `Names.short_name` and `dictionary_subject_id` are plain `Option`s with no skip attribute, so both appear as explicit `null`",
        "diff_prose": "the whole rewritten catalogue and nothing else.",
        "diff_extra": [
            'let catalogue = decoded.catalogue.as_ref().expect("the committed create-product-group diff carries the catalogue");',
            'assert_eq!(catalogue.product_groups.len(), 2, "create-product-group/appends-a-towel-radiators-group: the diff carries both groups, because the catalogue delta is whole-container");',
            'assert_eq!(catalogue.product_groups[0].id, "group.radiators", "create-product-group/appends-a-towel-radiators-group: the pre-existing group must keep position 0 in the diff");',
        ],
        "diff_none_fields": ["dictionary", "selection", "part_number_inputs", "exchange_process"],
        "change_noun": "product-group creation",
    },
    {
        "kind": "create-product",
        "case": "appends-a-pr900-product-to-the-existing-series",
        "mutation": {"CreateProduct": {"product": NEW_PRODUCT, "index": None}},
        "mutate": _create_product, "diff_key": "catalogue",
        "applies_fn": "appends_a_pr900_product_to_the_existing_series",
        "applies_doc": "With `index: None` the oracle appends, so `product.pr900` lands after `product.pr600`. It declares `series_id: \"series.pr\"`, an existing series — but the oracle checks only id UNIQUENESS, never referential validity, so nothing about the series is verified here.",
        "applies_extra": [
            'assert_eq!(applied.catalogue.products.len(), 2, "create-product/appends-a-pr900-product-to-the-existing-series: the product list must grow by exactly one");',
            'assert_eq!(applied.catalogue.products[1].id, "product.pr900", "create-product/appends-a-pr900-product-to-the-existing-series: a null index appends, so the new product must be last");',
            'assert_eq!(applied.catalogue.product_series.len(), 1, "create-product/appends-a-pr900-product-to-the-existing-series: joining an existing series must not duplicate that series");',
        ],
        "inverse_fn": "deleting_the_pr900_product_restores_before",
        "inverse_doc": "`create-product`'s inverse is a `DeleteProduct` on the created id — produced only because that id is absent from BASE, which it is here.",
        "inverse_shape": "creating a fresh product inverts to exactly one DeleteProduct",
        "inverse_restores": "deleting the created product did not restore the before-snapshot",
        "outcome_doc": "`product.pr900` is not among the committed products, so the fatal `mutation.duplicate-id` branch is not taken; `index` is `None`, so `mutation.clamped` is not raised either.",
        "guard_prose": "the id is fresh (no `mutation.duplicate-id`) and the index is null rather than out of range (no `mutation.clamped`)",
        "wire_prose": "`{\"CreateProduct\": {\"product\": {…}, \"index\": null}}` — the nested `series_id`/`parameter_domains`/`static_properties` keys stay snake_case, because `Product` carries no `rename_all`",
        "diff_prose": "the whole rewritten catalogue and nothing else.",
        "diff_extra": [
            'let catalogue = decoded.catalogue.as_ref().expect("the committed create-product diff carries the catalogue");',
            'assert_eq!(catalogue.products.len(), 2, "create-product/appends-a-pr900-product-to-the-existing-series: the diff carries both products, because the catalogue delta is whole-container");',
            'assert_eq!(catalogue.products[1].series_id, "series.pr", "create-product/appends-a-pr900-product-to-the-existing-series: the declared series id must survive the diff verbatim");',
        ],
        "diff_none_fields": ["dictionary", "selection", "part_number_rule", "script_limits"],
        "change_noun": "product creation",
    },
    {
        "kind": "replace-part-number-rule",
        "case": "swaps-the-literal-rule-for-a-height-driven-script",
        "mutation": {"ReplacePartNumberRule": {"new_rule": {"kind": "script", "function_id": "partno", "source": "height"}}},
        "mutate": _replace_rule, "diff_key": "partNumberRule",
        "applies_fn": "swaps_the_literal_rule_for_a_height_driven_script",
        "applies_doc": "`replace-part-number-rule` swaps the whole tagged `PartNumberRule` enum, moving it from the `Literal` variant to the `Script` variant. The script's `source` names `height`, which the committed `part_number_inputs` supplies — but the oracle does not resolve inputs, so nothing about that binding is checked here.",
        "applies_extra": [
            'assert!(matches!(applied.part_number_rule, %s::part_5::PartNumberRule::Script { .. }), "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: the rule must land on the Script variant");' % ISO,
            'assert_eq!(applied.part_number_inputs, before().part_number_inputs, "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: replacing the rule must not touch the inputs it will read");',
            'assert_eq!(applied.script_limits, before().script_limits, "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: nor the budgets the script will run under");',
        ],
        "inverse_fn": "restoring_the_literal_rule_restores_before",
        "inverse_doc": "`replace-part-number-rule`'s inverse clones the OLD rule out of BASE, so replaying it puts the `Literal { value: \"PR-600\" }` variant back.",
        "inverse_shape": "the inverse of one rule replacement is exactly one replacement back",
        "inverse_restores": "restoring the literal rule did not restore the before-snapshot",
        "outcome_doc": "The `Script` rule is not equal to the committed `Literal` rule, so the whole-enum equality guard does not raise `mutation.no-op`.",
        "guard_prose": "the Script variant differs from the committed Literal variant, so `replace-part-number-rule`'s `mutation.no-op` guard cannot fire",
        "wire_prose": "`{\"ReplacePartNumberRule\": {\"new_rule\": {\"kind\": \"script\", \"function_id\": …}}}` — `PartNumberRule` is internally tagged on `kind` with camelCase VARIANTS, but its struct-variant FIELDS keep snake_case",
        "diff_prose": "the replacement part-number rule and nothing else.",
        "diff_extra": [
            'let rule = decoded.part_number_rule.as_ref().expect("the committed replace-part-number-rule diff carries the rule");',
            'assert!(matches!(rule, %s::part_5::PartNumberRule::Script { .. }), "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: the diff must carry the Script variant");' % ISO,
        ],
        "diff_none_fields": ["catalogue", "dictionary", "selection", "part_number_inputs"],
        "change_noun": "part-number rule replacement",
    },
    {
        "kind": "change-exchange-process",
        "case": "advances-the-exchange-stage-to-determine-product",
        "mutation": {"ChangeExchangeProcess": {"new_exchange_process": "DetermineProduct"}},
        "mutate": _exchange_process, "diff_key": "exchangeProcess",
        "applies_fn": "advances_the_exchange_stage_to_determine_product",
        "applies_doc": "`change-exchange-process` writes the single ISO 16757-5 stage enum, moving the document from `ProvideCatalogue` to `DetermineProduct`. It is the only leaf in this tree whose diff container is a bare scalar rather than a whole collection.",
        "applies_extra": [
            'assert_eq!(applied.exchange_process, %s::part_5::ExchangeProcess::DetermineProduct, "change-exchange-process/advances-the-exchange-stage-to-determine-product: the stage must advance");' % ISO,
            'assert_eq!(applied.selection, before().selection, "change-exchange-process/advances-the-exchange-stage-to-determine-product: entering the determine-product stage must not pre-fill the selection request");',
            'assert_eq!(applied.catalogue, before().catalogue, "change-exchange-process/advances-the-exchange-stage-to-determine-product: nor touch the catalogue being exchanged");',
        ],
        "inverse_fn": "returning_to_the_provide_catalogue_stage_restores_before",
        "inverse_doc": "`change-exchange-process`'s inverse reads the OLD stage out of BASE by COPY (the enum is `Copy`, so there is no `.clone()` here), so replaying it returns the document to `ProvideCatalogue`.",
        "inverse_shape": "the inverse of one stage change is exactly one stage change back",
        "inverse_restores": "returning to the provide-catalogue stage did not restore the before-snapshot",
        "outcome_doc": "`DetermineProduct` differs from the committed `ProvideCatalogue`, so the equality guard stays shut; stage ORDER is not enforced, so any stage may follow any other.",
        "guard_prose": "the new stage differs from the committed one, so `change-exchange-process`'s `mutation.no-op` guard cannot fire",
        "wire_prose": "`{\"ChangeExchangeProcess\": {\"new_exchange_process\": \"DetermineProduct\"}}` — `ExchangeProcess` has `#[dsl(key = \"determineProduct\")]` for the DSL but NO serde rename, so the JSON spelling is the bare Rust variant name",
        "diff_prose": "the new exchange stage and nothing else.",
        "diff_extra": [
            'assert_eq!(decoded.exchange_process, Some(%s::part_5::ExchangeProcess::DetermineProduct), "change-exchange-process/advances-the-exchange-stage-to-determine-product: the diff must carry the new stage");' % ISO,
        ],
        "diff_none_fields": ["catalogue", "dictionary", "selection", "script_limits"],
        "change_noun": "exchange-stage change",
    },
    {
        "kind": "rename-product-group",
        "case": "renames-the-radiators-group-to-panel-radiators",
        "mutation": {"RenameProductGroup": {"id": "group.radiators", "new_name": "Panel radiators"}},
        "mutate": _rename_product_group, "diff_key": "catalogue",
        "applies_fn": "renames_the_radiators_group_to_panel_radiators",
        "applies_doc": "`rename-product-group` finds the group by id and writes only its `names.preferred.text`; the group's own id and its dictionary-subject mapping are untouched, so a rename never re-keys anything that points at it.",
        "applies_extra": [
            'assert_eq!(applied.catalogue.product_groups[0].names.preferred.text, "Panel radiators", "rename-product-group/renames-the-radiators-group-to-panel-radiators: the group name must change");',
            'assert_eq!(applied.catalogue.product_groups[0].id, "group.radiators", "rename-product-group/renames-the-radiators-group-to-panel-radiators: the id is the identity and must never follow the label");',
            'assert_eq!(applied.catalogue.product_classes[0].group_id, "group.radiators", "rename-product-group/renames-the-radiators-group-to-panel-radiators: the child class keeps pointing at the same id");',
        ],
        "inverse_fn": "renaming_the_group_back_restores_before",
        "inverse_doc": "`rename-product-group`'s inverse looks the group up in BASE and carries its OLD name, so replaying it puts \"Radiators\" back on the same id.",
        "inverse_shape": "the inverse of one group rename is exactly one rename back",
        "inverse_restores": "renaming the group back did not restore the before-snapshot",
        "outcome_doc": "The group exists, so the `mutation.target-missing` Error branch is skipped; and \"Panel radiators\" differs from the committed \"Radiators\", so the `mutation.no-op` branch is skipped too. This leaf is one of the few with BOTH guards.",
        "guard_prose": "the group exists (no `mutation.target-missing`) and the new name differs from the committed one (no `mutation.no-op`)",
        "wire_prose": "`{\"RenameProductGroup\": {\"id\": …, \"new_name\": …}}` — externally tagged, snake_case payload keys",
        "diff_prose": "the whole rewritten catalogue and nothing else.",
        "diff_extra": [
            'let catalogue = decoded.catalogue.as_ref().expect("the committed rename-product-group diff carries the catalogue");',
            'assert_eq!(catalogue.product_groups[0].names.preferred.text, "Panel radiators", "rename-product-group/renames-the-radiators-group-to-panel-radiators: the diff must carry the new group name");',
            'assert_eq!(catalogue.product_groups[0].dictionary_subject_id.as_deref(), Some("subject.radiator"), "rename-product-group/renames-the-radiators-group-to-panel-radiators: the dictionary mapping rides through the diff unchanged");',
        ],
        "diff_none_fields": ["dictionary", "selection", "part_number_inputs", "script_limits"],
        "change_noun": "product-group rename",
    },
    {
        "kind": "rename-product",
        "case": "renames-pr600-to-the-compact-variant-name",
        "mutation": {"RenameProduct": {"id": "product.pr600", "new_name": "PR-600 Compact"}},
        "mutate": _rename_product, "diff_key": "catalogue",
        "applies_fn": "renames_pr600_to_the_compact_variant_name",
        "applies_doc": "`rename-product` finds the product by id and writes only its `names.preferred.text`. The product id stays `product.pr600` and the part-number rule still literals `PR-600`, because a marketing name and a part number are different facts.",
        "applies_extra": [
            'assert_eq!(applied.catalogue.products[0].names.preferred.text, "PR-600 Compact", "rename-product/renames-pr600-to-the-compact-variant-name: the product name must change");',
            'assert_eq!(applied.catalogue.products[0].id, "product.pr600", "rename-product/renames-pr600-to-the-compact-variant-name: the id is the identity and must never follow the label");',
            'assert_eq!(applied.part_number_rule, before().part_number_rule, "rename-product/renames-pr600-to-the-compact-variant-name: the part-number rule is a different fact from the display name and must not be rewritten");',
        ],
        "inverse_fn": "renaming_the_product_back_restores_before",
        "inverse_doc": "`rename-product`'s inverse looks the product up in BASE and carries its OLD name, so replaying it puts \"PR-600\" back on the same id.",
        "inverse_shape": "the inverse of one product rename is exactly one rename back",
        "inverse_restores": "renaming the product back did not restore the before-snapshot",
        "outcome_doc": "The product exists, so the `mutation.target-missing` Error branch is skipped; and \"PR-600 Compact\" differs from the committed \"PR-600\", so the `mutation.no-op` branch is skipped too.",
        "guard_prose": "the product exists (no `mutation.target-missing`) and the new name differs from the committed one (no `mutation.no-op`)",
        "wire_prose": "`{\"RenameProduct\": {\"id\": …, \"new_name\": …}}` — externally tagged, snake_case payload keys",
        "diff_prose": "the whole rewritten catalogue and nothing else.",
        "diff_extra": [
            'let catalogue = decoded.catalogue.as_ref().expect("the committed rename-product diff carries the catalogue");',
            'assert_eq!(catalogue.products[0].names.preferred.text, "PR-600 Compact", "rename-product/renames-pr600-to-the-compact-variant-name: the diff must carry the new product name");',
            'assert_eq!(catalogue.products[0].series_id, "series.pr", "rename-product/renames-pr600-to-the-compact-variant-name: the series membership rides through the diff unchanged");',
        ],
        "diff_none_fields": ["dictionary", "selection", "part_number_rule", "exchange_process"],
        "change_noun": "product rename",
    },
]


def main():
    for case in CASES:
        render(case)
    wiring.wire(ARTIFACT_DIR, [case["kind"] for case in CASES])
    print("iso16757: %d cases" % len(CASES))


if __name__ == "__main__":
    main()
