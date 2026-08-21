"""📔️ VDI 3805 — 19 hand-authored mutation fixture cases."""
import copy
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from importlib import import_module

common = import_module("📜️common")
REPO = common.REPO

ROOT = os.path.join(REPO, "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations")

BAG = {"fields": {}}

FILE = {
    "header_version": "3805",
    "manufacturer": "DEMO",
    "building_system_number": {"system_code": "420", "subsystem": "10", "sequence": 1},
    "created": "2026-07-22",
    "charset": "UTF-8",
    "record_count": 2,
    "extensions": BAG,
}

M3H = {"symbol": "m3/h", "kind": "Volume", "delta": False, "si_factor": 1.0}
PCT = {"symbol": "%", "kind": "Dimensionless", "delta": True, "si_factor": 0.01}

VALVE_50 = {
    "identity": {"manufacturer_code": "DEMO", "product_group": "HV", "article_number": "VLV-50-001"},
    "title": [{"locale": "de", "text": "Stellventil DN50"}, {"locale": "en", "text": "Control valve DN50"}],
    "sheet": 2,
    "records": [{"family": "100", "fields": ["100", "DEMO", "HV", "VLV-50-001", "2"], "extensions": BAG}],
    "configuration": {
        "id": "cfg.VLV-50-001",
        "parameters": {"dn": {"kind": "integer", "value": 50}, "kvs": {"kind": "decimal", "value": 4.5, "unit": M3H}},
        "geometry_ref": "geom.valve.50",
        "function_refs": ["curve.kvs"],
    },
    "accessories": [],
    "components": [],
    "extensions": BAG,
}

BEFORE = {
    "manufacturerFile": FILE,
    "catalog": {"file": FILE, "products": [VALVE_50], "extensions": BAG},
    "editionProfile": {"8": "Legacy"},
    "correctionAsOf": {"year": 2024, "month": 1},
    "strictMode": False,
    "index": {"entries": [{"product_id": "VLV-50-001", "sheet": 2, "tags": ["Stellventil DN50", "Control valve DN50"], "dn": 50}]},
    "geometry": {
        "geom.valve.50": {
            "id": "geom.valve.50",
            "bbox": {"min_x": 0.0, "min_y": 0.0, "min_z": 0.0, "max_x": 0.25, "max_y": 0.5, "max_z": 0.125},
            "connections": [
                {"id": "in", "medium": "water", "position": [0.0, 0.25, 0.0625], "direction": [-1.0, 0.0, 0.0], "diameter_mm": 50.0},
                {"id": "out", "medium": "water", "position": [0.25, 0.25, 0.0625], "direction": [1.0, 0.0, 0.0], "diameter_mm": 50.0},
            ],
            "parameters": {"scale": 1.0},
        }
    },
    "curves": {
        "curve.kvs": {
            "id": "curve.kvs",
            "x_unit": PCT,
            "y_unit": M3H,
            "points": [{"x": 0.0, "y": 0.0}, {"x": 100.0, "y": 4.5}],
        }
    },
    "limits": {"max_file_bytes": 16777216, "max_records": 100000, "max_field_length": 8192, "max_nesting_depth": 32},
}

DIFF_NULL = {
    "artifact": None,
    "manufacturerFile": None,
    "catalog": None,
    "editionProfile": None,
    "correctionAsOf": None,
    "strictMode": None,
    "index": None,
    "geometry": None,
    "curves": None,
    "limits": None,
    "selectedCheckIndex": None,
}

APPLIED = {"status": "applied"}

CASES = []


def case(leaf, name, summary, mutate, mutation, touched, extra_applied, extra_diff):
    after = copy.deepcopy(BEFORE)
    mutate(after)
    diff = copy.deepcopy(DIFF_NULL)
    for field in touched:
        diff[field] = copy.deepcopy(after[field])
    CASES.append(
        dict(
            leaf=leaf,
            case=name,
            summary=summary,
            after=after,
            mutation=mutation,
            diff=diff,
            extra_applied=extra_applied,
            extra_diff=extra_diff,
        )
    )


KIND = {
    "🏕️update-manufacturer-file": "update-manufacturer-file",
    "🏜️change-correction-as-of": "change-correction-as-of",
    "🦋change-strict-mode": "change-strict-mode",
    "🦈update-limits": "update-limits",
    "🐝change-edition-profile": "change-edition-profile",
    "⛰️remove-edition-profile": "remove-edition-profile",
    "🪵create-product": "create-product",
    "🐳delete-product": "delete-product",
    "🏖️rename-product": "rename-product",
    "🗻replace-product-configuration": "replace-product-configuration",
    "🦭create-geometry": "create-geometry",
    "🐬delete-geometry": "delete-geometry",
    "🏟️resize-geometry": "resize-geometry",
    "🐞add-geometry-connection": "add-geometry-connection",
    "🏔️remove-geometry-connection": "remove-geometry-connection",
    "🐌replace-geometry-parameters": "replace-geometry-parameters",
    "🏝️create-curve": "create-curve",
    "🐢delete-curve": "delete-curve",
    "🏞️replace-curve-points": "replace-curve-points",
}

# ── 1. update-manufacturer-file ────────────────────────────────────────────────
ACME = copy.deepcopy(FILE)
ACME["manufacturer"] = "ACME"
ACME["record_count"] = 3


def _update_manufacturer_file(s):
    s["manufacturerFile"] = copy.deepcopy(ACME)


case(
    "🏕️update-manufacturer-file",
    "renames-the-header-manufacturer-to-acme",
    "The whole `010` header facet is swapped atomically: `manufacturer` becomes `ACME` and "
    "`record_count` becomes 3. `catalog.file` is a SEPARATE copy of the header and is deliberately "
    "left alone — the diff builder writes `manufacturer_file` only.",
    _update_manufacturer_file,
    {"UpdateManufacturerFile": {"new_manufacturer_file": ACME}},
    ["manufacturerFile"],
    '''    assert_eq!(snapshot.manufacturer_file.manufacturer, "ACME", "update-manufacturer-file/renames-the-header-manufacturer-to-acme: the header manufacturer must read ACME");
    assert_eq!(snapshot.manufacturer_file.record_count, 3, "update-manufacturer-file/renames-the-header-manufacturer-to-acme: the header record_count must read 3");
    assert_eq!(snapshot.catalog.file.manufacturer, "DEMO", "update-manufacturer-file/renames-the-header-manufacturer-to-acme: catalog.file is a separate header copy and must still read DEMO");''',
    '''    assert_eq!(raised_diff.manufacturer_file.as_ref().map(|file| file.manufacturer.as_str()), Some("ACME"), "update-manufacturer-file/renames-the-header-manufacturer-to-acme: the diff must publish manufacturerFile with manufacturer ACME");
    assert!(raised_diff.catalog.is_none(), "update-manufacturer-file/renames-the-header-manufacturer-to-acme: updating the header must not republish the whole catalog");''',
)

# ── 2. change-correction-as-of ─────────────────────────────────────────────────
def _change_correction_as_of(s):
    s["correctionAsOf"] = {"year": 2025, "month": 3}


case(
    "🏜️change-correction-as-of",
    "advances-the-correction-cut-off-to-2025-03",
    "The document root's correction cut-off edition moves from 2024-01 to 2025-03. Nothing else in "
    "the document is republished — the diff carries `correctionAsOf` alone.",
    _change_correction_as_of,
    {"ChangeCorrectionAsOf": {"new_correction_as_of": {"year": 2025, "month": 3}}},
    ["correctionAsOf"],
    '''    assert_eq!(snapshot.correction_as_of.year, 2025, "change-correction-as-of/advances-the-correction-cut-off-to-2025-03: the cut-off year must be 2025");
    assert_eq!(snapshot.correction_as_of.month, 3, "change-correction-as-of/advances-the-correction-cut-off-to-2025-03: the cut-off month must be 03");''',
    '''    assert_eq!(raised_diff.correction_as_of, Some(crate::artifacts::vdi3805::EditionId { year: 2025, month: 3 }), "change-correction-as-of/advances-the-correction-cut-off-to-2025-03: the diff must publish correctionAsOf 2025-03");
    assert!(raised_diff.edition_profile.is_none(), "change-correction-as-of/advances-the-correction-cut-off-to-2025-03: the correction cut-off must not touch the per-sheet edition profile map");''',
)

# ── 3. change-strict-mode ──────────────────────────────────────────────────────
def _change_strict_mode(s):
    s["strictMode"] = True


case(
    "🦋change-strict-mode",
    "turns-strict-mode-on",
    "The document root's strict-mode flag flips `false` → `true`. This is the narrowest diff in the "
    "vocabulary: a single scalar `Option<bool>` and ten explicit nulls.",
    _change_strict_mode,
    {"ChangeStrictMode": {"new_strict_mode": True}},
    ["strictMode"],
    '''    assert!(snapshot.strict_mode, "change-strict-mode/turns-strict-mode-on: strict mode must be enabled after the mutation");
    assert!(!before().strict_mode, "change-strict-mode/turns-strict-mode-on: the committed before-snapshot must start with strict mode disabled");''',
    '''    assert_eq!(raised_diff.strict_mode, Some(true), "change-strict-mode/turns-strict-mode-on: the diff must publish strictMode = true");
    assert!(raised_diff.limits.is_none(), "change-strict-mode/turns-strict-mode-on: the strict-mode flag and the security limits are separate facets");''',
)

# ── 4. update-limits ───────────────────────────────────────────────────────────
TIGHT = {"max_file_bytes": 1048576, "max_records": 4096, "max_field_length": 512, "max_nesting_depth": 8}


def _update_limits(s):
    s["limits"] = copy.deepcopy(TIGHT)


case(
    "🦈update-limits",
    "tightens-every-untrusted-input-limit",
    "All four untrusted-input security limits are one policy and move together: 16 MiB→1 MiB, "
    "100000→4096 records, 8192→512 field bytes, depth 32→8.",
    _update_limits,
    {"UpdateLimits": {"new_limits": TIGHT}},
    ["limits"],
    '''    assert_eq!(snapshot.limits.max_file_bytes, 1048576, "update-limits/tightens-every-untrusted-input-limit: max_file_bytes must be 1 MiB");
    assert_eq!(snapshot.limits.max_records, 4096, "update-limits/tightens-every-untrusted-input-limit: max_records must be 4096");
    assert_eq!(snapshot.limits.max_field_length, 512, "update-limits/tightens-every-untrusted-input-limit: max_field_length must be 512");
    assert_eq!(snapshot.limits.max_nesting_depth, 8, "update-limits/tightens-every-untrusted-input-limit: max_nesting_depth must be 8");''',
    '''    assert_eq!(raised_diff.limits.map(|limits| limits.max_nesting_depth), Some(8), "update-limits/tightens-every-untrusted-input-limit: the diff must publish the whole limits facet with depth 8");
    assert!(raised_diff.strict_mode.is_none(), "update-limits/tightens-every-untrusted-input-limit: changing limits must not also flip strict mode");''',
)

# ── 5. change-edition-profile ──────────────────────────────────────────────────
def _change_edition_profile(s):
    s["editionProfile"]["8"] = "Current"


case(
    "🐝change-edition-profile",
    "switches-sheet-8-from-legacy-to-current",
    "The upsert rewrites sheet `8`'s existing override from `Legacy` to `Current`. Because the key "
    "already existed in `before`, the inverse is another `change-edition-profile` (back to "
    "`Legacy`), never a `remove-edition-profile`.",
    _change_edition_profile,
    {"ChangeEditionProfile": {"sheet": "8", "new_choice": "Current"}},
    ["editionProfile"],
    '''    assert_eq!(snapshot.edition_profile.get("8"), Some(&crate::artifacts::vdi3805::EditionProfileChoice::Current), "change-edition-profile/switches-sheet-8-from-legacy-to-current: sheet 8 must resolve to the Current profile");
    assert_eq!(snapshot.edition_profile.len(), 1, "change-edition-profile/switches-sheet-8-from-legacy-to-current: an upsert of an existing key must not add a second override");''',
    '''    assert_eq!(raised_diff.edition_profile.as_ref().and_then(|map| map.get("8")), Some(&crate::artifacts::vdi3805::EditionProfileChoice::Current), "change-edition-profile/switches-sheet-8-from-legacy-to-current: the diff must publish editionProfile with sheet 8 = Current");
    assert!(raised_diff.correction_as_of.is_none(), "change-edition-profile/switches-sheet-8-from-legacy-to-current: a per-sheet override must not move the document-wide correction cut-off");''',
)

# ── 6. remove-edition-profile ──────────────────────────────────────────────────
def _remove_edition_profile(s):
    del s["editionProfile"]["8"]


case(
    "⛰️remove-edition-profile",
    "clears-the-sheet-8-legacy-override",
    "Sheet `8`'s `Legacy` override is dropped, reverting that sheet to the evaluator's default. The "
    "diff republishes the whole (now empty) `editionProfile` map, and the inverse is a "
    "`change-edition-profile` back to `Legacy`.",
    _remove_edition_profile,
    {"RemoveEditionProfile": {"sheet": "8"}},
    ["editionProfile"],
    '''    assert!(!snapshot.edition_profile.contains_key("8"), "remove-edition-profile/clears-the-sheet-8-legacy-override: sheet 8 must no longer carry an override");
    assert!(snapshot.edition_profile.is_empty(), "remove-edition-profile/clears-the-sheet-8-legacy-override: sheet 8 was the only override, so the map must end up empty");''',
    '''    assert_eq!(raised_diff.edition_profile.as_ref().map(|map| map.len()), Some(0), "remove-edition-profile/clears-the-sheet-8-legacy-override: the diff must publish an empty editionProfile map, not omit the field");
    assert!(raised_diff.catalog.is_none(), "remove-edition-profile/clears-the-sheet-8-legacy-override: dropping a sheet override must not republish the catalog");''',
)

# ── 7. create-product ──────────────────────────────────────────────────────────
VALVE_80 = {
    "identity": {"manufacturer_code": "DEMO", "product_group": "HV", "article_number": "VLV-80-002"},
    "title": [{"locale": "de", "text": "Stellventil DN80"}, {"locale": "en", "text": "Control valve DN80"}],
    "sheet": 2,
    "records": [],
    "configuration": {
        "id": "cfg.VLV-80-002",
        "parameters": {"dn": {"kind": "integer", "value": 80}},
        "geometry_ref": None,
        "function_refs": [],
    },
    "accessories": [],
    "components": [],
    "extensions": BAG,
}


def _create_product(s):
    s["catalog"]["products"].append(copy.deepcopy(VALVE_80))
    s["index"]["entries"].append({"product_id": "VLV-80-002", "sheet": 2, "tags": ["Stellventil DN80", "Control valve DN80"], "dn": 80})


case(
    "🪵create-product",
    "appends-vlv-80-002-and-its-index-entry",
    "A second catalogue product is appended (`index: null` ⇒ push at the end, no `mutation.clamped` "
    "warning), and the persisted `catalog.index` gains the matching entry built by "
    "`catalog_index_entry_for` — `dn` 80 read out of the configuration's parameter bag, `tags` from "
    "the bilingual title.",
    _create_product,
    {"CreateProduct": {"product": VALVE_80, "index": None}},
    ["catalog", "index"],
    '''    assert_eq!(snapshot.catalog.products.len(), 2, "create-product/appends-vlv-80-002-and-its-index-entry: the catalogue must hold both products");
    assert_eq!(snapshot.catalog.products[1].identity.article_number, "VLV-80-002", "create-product/appends-vlv-80-002-and-its-index-entry: a null insert index must append, not prepend");
    assert_eq!(snapshot.index.entries.iter().find(|entry| entry.product_id == "VLV-80-002").and_then(|entry| entry.dn), Some(80), "create-product/appends-vlv-80-002-and-its-index-entry: the index entry must carry dn 80 extracted from the configuration");''',
    '''    assert_eq!(raised_diff.catalog.as_ref().map(|catalog| catalog.products.len()), Some(2), "create-product/appends-vlv-80-002-and-its-index-entry: the diff must publish the catalog with two products");
    assert_eq!(raised_diff.index.as_ref().map(|index| index.entries.len()), Some(2), "create-product/appends-vlv-80-002-and-its-index-entry: catalog.index is persisted state and must be republished in lockstep");''',
)

# ── 8. delete-product ──────────────────────────────────────────────────────────
def _delete_product(s):
    s["catalog"]["products"] = []
    s["index"]["entries"] = []


case(
    "🐳delete-product",
    "removes-vlv-50-001-and-its-index-entry",
    "The only catalogue product is deleted by article number; `catalog.index` loses its matching "
    "entry in the same diff. The inverse re-creates the product at its original position 0.",
    _delete_product,
    {"DeleteProduct": {"id": "VLV-50-001"}},
    ["catalog", "index"],
    '''    assert!(snapshot.catalog.products.is_empty(), "delete-product/removes-vlv-50-001-and-its-index-entry: the catalogue must be empty afterwards");
    assert!(snapshot.index.entries.is_empty(), "delete-product/removes-vlv-50-001-and-its-index-entry: the persisted index must lose the VLV-50-001 entry too");
    assert!(snapshot.geometry.contains_key("geom.valve.50"), "delete-product/removes-vlv-50-001-and-its-index-entry: deleting a product must NOT cascade into the geometry it referenced");''',
    '''    assert_eq!(raised_diff.catalog.as_ref().map(|catalog| catalog.products.len()), Some(0), "delete-product/removes-vlv-50-001-and-its-index-entry: the diff must publish an emptied product list");
    assert!(raised_diff.geometry.is_none(), "delete-product/removes-vlv-50-001-and-its-index-entry: no geometry cascade is part of this mutation's contract");''',
)

# ── 9. rename-product ──────────────────────────────────────────────────────────
NEW_TITLE = [{"locale": "de", "text": "Regelventil DN50"}, {"locale": "en", "text": "Regulating valve DN50"}]


def _rename_product(s):
    s["catalog"]["products"][0]["title"] = copy.deepcopy(NEW_TITLE)
    s["index"]["entries"][0]["tags"] = ["Regelventil DN50", "Regulating valve DN50"]


case(
    "🏖️rename-product",
    "retitles-vlv-50-001-and-resyncs-its-index-tags",
    "The bilingual product title is replaced, and the persisted `catalog.index` entry's `tags` are "
    "rebuilt from the new title's texts in order. Only `catalog` and `index` are republished.",
    _rename_product,
    {"RenameProduct": {"id": "VLV-50-001", "new_title": NEW_TITLE}},
    ["catalog", "index"],
    '''    let title = &snapshot.catalog.products[0].title;
    assert_eq!(title.iter().find(|entry| entry.locale == "en").map(|entry| entry.text.as_str()), Some("Regulating valve DN50"), "rename-product/retitles-vlv-50-001-and-resyncs-its-index-tags: the English title must be the new one");
    assert_eq!(title.iter().find(|entry| entry.locale == "de").map(|entry| entry.text.as_str()), Some("Regelventil DN50"), "rename-product/retitles-vlv-50-001-and-resyncs-its-index-tags: the German title must be the new one");
    assert_eq!(snapshot.index.entries[0].tags, vec!["Regelventil DN50".to_string(), "Regulating valve DN50".to_string()], "rename-product/retitles-vlv-50-001-and-resyncs-its-index-tags: index tags must mirror the new title texts in order");
    assert_eq!(snapshot.index.entries[0].dn, Some(50), "rename-product/retitles-vlv-50-001-and-resyncs-its-index-tags: renaming must leave the index entry's dn untouched");''',
    '''    assert_eq!(raised_diff.index.as_ref().map(|index| index.entries[0].tags.len()), Some(2), "rename-product/retitles-vlv-50-001-and-resyncs-its-index-tags: the diff must republish the index with both new tags");
    assert!(raised_diff.manufacturer_file.is_none(), "rename-product/retitles-vlv-50-001-and-resyncs-its-index-tags: a product title is not part of the manufacturer file header");''',
)

# ── 10. replace-product-configuration ──────────────────────────────────────────
NEW_CONFIG = {
    "id": "cfg.VLV-50-001",
    "parameters": {"dn": {"kind": "integer", "value": 80}, "kvs": {"kind": "decimal", "value": 8.0, "unit": M3H}},
    "geometry_ref": "geom.valve.50",
    "function_refs": ["curve.kvs"],
}


def _replace_product_configuration(s):
    s["catalog"]["products"][0]["configuration"] = copy.deepcopy(NEW_CONFIG)
    s["index"]["entries"][0]["dn"] = 80


case(
    "🗻replace-product-configuration",
    "reparameterises-vlv-50-001-to-dn-80-and-resyncs-index-dn",
    "The product's whole configuration block is swapped (dn 50→80, kvs 4.5→8.0 m3/h) and "
    "`extract_dn` recomputes the persisted index entry's `dn` from the NEW parameter bag. `tags` are "
    "not touched — only `dn` is derived from the configuration.",
    _replace_product_configuration,
    {"ReplaceProductConfiguration": {"id": "VLV-50-001", "new_configuration": NEW_CONFIG}},
    ["catalog", "index"],
    '''    assert_eq!(snapshot.index.entries[0].dn, Some(80), "replace-product-configuration/reparameterises-vlv-50-001-to-dn-80-and-resyncs-index-dn: extract_dn must lift dn 80 out of the new parameter bag");
    assert_eq!(snapshot.index.entries[0].tags, before().index.entries[0].tags, "replace-product-configuration/reparameterises-vlv-50-001-to-dn-80-and-resyncs-index-dn: the title-derived tags must be left alone");
    assert_eq!(snapshot.catalog.products[0].configuration.parameters.len(), 2, "replace-product-configuration/reparameterises-vlv-50-001-to-dn-80-and-resyncs-index-dn: the swapped configuration keeps both dn and kvs parameters");''',
    '''    assert_eq!(raised_diff.index.as_ref().and_then(|index| index.entries[0].dn), Some(80), "replace-product-configuration/reparameterises-vlv-50-001-to-dn-80-and-resyncs-index-dn: the diff must republish the index with dn 80");
    assert!(raised_diff.curves.is_none(), "replace-product-configuration/reparameterises-vlv-50-001-to-dn-80-and-resyncs-index-dn: a configuration's function_refs are references, so the curve store is untouched");''',
)

# ── 11. create-geometry ────────────────────────────────────────────────────────
GEOM_80 = {
    "id": "geom.valve.80",
    "bbox": {"min_x": 0.0, "min_y": 0.0, "min_z": 0.0, "max_x": 0.5, "max_y": 0.75, "max_z": 0.25},
    "connections": [],
    "parameters": {"scale": 1.0},
}


def _create_geometry(s):
    s["geometry"]["geom.valve.80"] = copy.deepcopy(GEOM_80)


case(
    "🦭create-geometry",
    "adds-the-geom-valve-80-definition",
    "A second id-keyed parametric geometry is inserted into the `geometry` map. A duplicate id would "
    "be `mutation.duplicate-id` (fatal); `geom.valve.80` is fresh, so the whole map is republished "
    "with two entries.",
    _create_geometry,
    {"CreateGeometry": {"geometry": GEOM_80}},
    ["geometry"],
    '''    assert!(snapshot.geometry.contains_key("geom.valve.80"), "create-geometry/adds-the-geom-valve-80-definition: the new geometry id must be present");
    assert_eq!(snapshot.geometry.len(), 2, "create-geometry/adds-the-geom-valve-80-definition: the pre-existing geom.valve.50 must survive");
    assert_eq!(snapshot.geometry["geom.valve.80"].bbox.max_y, 0.75, "create-geometry/adds-the-geom-valve-80-definition: the inserted bounding box must be the payload's");''',
    '''    assert_eq!(raised_diff.geometry.as_ref().map(|map| map.len()), Some(2), "create-geometry/adds-the-geom-valve-80-definition: the diff republishes the whole geometry map, now holding two definitions");
    assert!(raised_diff.catalog.is_none(), "create-geometry/adds-the-geom-valve-80-definition: creating geometry does not touch the product catalogue that may later reference it");''',
)

# ── 12. delete-geometry ────────────────────────────────────────────────────────
def _delete_geometry(s):
    del s["geometry"]["geom.valve.50"]


case(
    "🐬delete-geometry",
    "removes-the-geom-valve-50-definition",
    "The only geometry definition is dropped from the `geometry` map. `VLV-50-001`'s "
    "`configuration.geometry_ref` still names it — this mutation deliberately does NOT cascade into "
    "the catalogue.",
    _delete_geometry,
    {"DeleteGeometry": {"id": "geom.valve.50"}},
    ["geometry"],
    '''    assert!(snapshot.geometry.is_empty(), "delete-geometry/removes-the-geom-valve-50-definition: the geometry map must end up empty");
    assert_eq!(snapshot.catalog.products[0].configuration.geometry_ref.as_deref(), Some("geom.valve.50"), "delete-geometry/removes-the-geom-valve-50-definition: the dangling geometry_ref is left in place — no cascade");''',
    '''    assert_eq!(raised_diff.geometry.as_ref().map(|map| map.len()), Some(0), "delete-geometry/removes-the-geom-valve-50-definition: the diff must publish an empty geometry map, not omit the field");
    assert!(raised_diff.index.is_none(), "delete-geometry/removes-the-geom-valve-50-definition: the catalogue index is product-keyed and is not touched by geometry deletion");''',
)

# ── 13. resize-geometry ────────────────────────────────────────────────────────
BIG_BBOX = {"min_x": 0.0, "min_y": 0.0, "min_z": 0.0, "max_x": 0.5, "max_y": 1.0, "max_z": 0.25}


def _resize_geometry(s):
    s["geometry"]["geom.valve.50"]["bbox"] = copy.deepcopy(BIG_BBOX)


case(
    "🏟️resize-geometry",
    "doubles-the-geom-valve-50-bounding-box",
    "Every axis of `geom.valve.50`'s bounding box doubles (0.25/0.5/0.125 → 0.5/1.0/0.25). The "
    "builder first gates on finiteness and on `max >= min` per axis (`mutation.invariant`, fatal); "
    "this payload passes both, so only `bbox` inside the republished geometry map changes.",
    _resize_geometry,
    {"ResizeGeometry": {"id": "geom.valve.50", "new_bbox": BIG_BBOX}},
    ["geometry"],
    '''    assert_eq!(snapshot.geometry["geom.valve.50"].bbox.max_x, 0.5, "resize-geometry/doubles-the-geom-valve-50-bounding-box: max_x must be 0.5");
    assert_eq!(snapshot.geometry["geom.valve.50"].bbox.max_y, 1.0, "resize-geometry/doubles-the-geom-valve-50-bounding-box: max_y must be 1.0");
    assert_eq!(snapshot.geometry["geom.valve.50"].bbox.max_z, 0.25, "resize-geometry/doubles-the-geom-valve-50-bounding-box: max_z must be 0.25");
    assert_eq!(snapshot.geometry["geom.valve.50"].connections.len(), 2, "resize-geometry/doubles-the-geom-valve-50-bounding-box: resizing must leave both connection points in place");''',
    '''    assert_eq!(raised_diff.geometry.as_ref().map(|map| map["geom.valve.50"].bbox.max_y), Some(1.0), "resize-geometry/doubles-the-geom-valve-50-bounding-box: the diff must publish the geometry map carrying the new max_y");
    assert!(raised_diff.limits.is_none(), "resize-geometry/doubles-the-geom-valve-50-bounding-box: a bounding box is geometry, never a security limit");''',
)

# ── 14. add-geometry-connection ────────────────────────────────────────────────
DRAIN = {"id": "drain", "medium": "water", "position": [0.125, 0.0, 0.0625], "direction": [0.0, -1.0, 0.0], "diameter_mm": 12.5}


def _add_geometry_connection(s):
    s["geometry"]["geom.valve.50"]["connections"].append(copy.deepcopy(DRAIN))


case(
    "🐞add-geometry-connection",
    "attaches-the-drain-connection-to-geom-valve-50",
    "A third connection point (`drain`, DN12.5, pointing -Y) is upserted onto `geom.valve.50`. The "
    "builder retains-then-pushes by `connection.id`, so a fresh id lands at the END of the list, "
    "after `in` and `out`. Because `drain` was absent in `before`, the inverse is "
    "`remove-geometry-connection`.",
    _add_geometry_connection,
    {"AddGeometryConnection": {"id": "geom.valve.50", "connection": DRAIN}},
    ["geometry"],
    '''    let connections = &snapshot.geometry["geom.valve.50"].connections;
    assert_eq!(connections.len(), 3, "add-geometry-connection/attaches-the-drain-connection-to-geom-valve-50: the geometry must carry three connection points");
    assert_eq!(connections[2].id, "drain", "add-geometry-connection/attaches-the-drain-connection-to-geom-valve-50: a fresh connection id is appended last, after in and out");
    assert_eq!(connections[2].diameter_mm, Some(12.5), "add-geometry-connection/attaches-the-drain-connection-to-geom-valve-50: the drain's nominal diameter must be 12.5 mm");''',
    '''    assert_eq!(raised_diff.geometry.as_ref().map(|map| map["geom.valve.50"].connections.len()), Some(3), "add-geometry-connection/attaches-the-drain-connection-to-geom-valve-50: the diff must publish the geometry map with three connections");
    assert!(raised_diff.curves.is_none(), "add-geometry-connection/attaches-the-drain-connection-to-geom-valve-50: connection points are not characteristic curves");''',
)

# ── 15. remove-geometry-connection ─────────────────────────────────────────────
def _remove_geometry_connection(s):
    s["geometry"]["geom.valve.50"]["connections"] = [s["geometry"]["geom.valve.50"]["connections"][0]]


case(
    "🏔️remove-geometry-connection",
    "detaches-the-out-connection-from-geom-valve-50",
    "`geom.valve.50`'s outlet connection is retained-out by id, leaving only `in`. A missing geometry "
    "id or a missing connection id would both be `mutation.target-missing`; both exist here.",
    _remove_geometry_connection,
    {"RemoveGeometryConnection": {"id": "geom.valve.50", "connection_id": "out"}},
    ["geometry"],
    '''    let connections = &snapshot.geometry["geom.valve.50"].connections;
    assert_eq!(connections.len(), 1, "remove-geometry-connection/detaches-the-out-connection-from-geom-valve-50: exactly one connection point must survive");
    assert_eq!(connections[0].id, "in", "remove-geometry-connection/detaches-the-out-connection-from-geom-valve-50: the surviving connection must be the inlet");
    assert_eq!(snapshot.geometry["geom.valve.50"].bbox, before().geometry["geom.valve.50"].bbox, "remove-geometry-connection/detaches-the-out-connection-from-geom-valve-50: detaching a connection must not resize the geometry");''',
    '''    assert_eq!(raised_diff.geometry.as_ref().map(|map| map["geom.valve.50"].connections.len()), Some(1), "remove-geometry-connection/detaches-the-out-connection-from-geom-valve-50: the diff must publish the geometry map with the outlet gone");
    assert!(raised_diff.catalog.is_none(), "remove-geometry-connection/detaches-the-out-connection-from-geom-valve-50: no product in the catalogue is rewritten by detaching a connection");''',
)

# ── 16. replace-geometry-parameters ────────────────────────────────────────────
def _replace_geometry_parameters(s):
    s["geometry"]["geom.valve.50"]["parameters"] = {"clearance": 0.0625, "scale": 0.5}


case(
    "🐌replace-geometry-parameters",
    "rescales-geom-valve-50-to-half-and-adds-clearance",
    "The tuning parameter map is swapped WHOLESALE — `scale` drops 1.0 → 0.5 and a new `clearance` "
    "key appears. `bbox` itself is untouched: `evaluate_bbox` derives the scaled extent at read time.",
    _replace_geometry_parameters,
    {"ReplaceGeometryParameters": {"id": "geom.valve.50", "new_parameters": {"clearance": 0.0625, "scale": 0.5}}},
    ["geometry"],
    '''    let parameters = &snapshot.geometry["geom.valve.50"].parameters;
    assert_eq!(parameters.get("scale"), Some(&0.5), "replace-geometry-parameters/rescales-geom-valve-50-to-half-and-adds-clearance: scale must be 0.5");
    assert_eq!(parameters.get("clearance"), Some(&0.0625), "replace-geometry-parameters/rescales-geom-valve-50-to-half-and-adds-clearance: the new clearance key must be present");
    assert_eq!(snapshot.geometry["geom.valve.50"].bbox.max_x, 0.25, "replace-geometry-parameters/rescales-geom-valve-50-to-half-and-adds-clearance: the stored bbox is NOT pre-scaled by this mutation");''',
    '''    assert_eq!(raised_diff.geometry.as_ref().map(|map| map["geom.valve.50"].parameters.len()), Some(2), "replace-geometry-parameters/rescales-geom-valve-50-to-half-and-adds-clearance: the diff must publish the geometry map with both parameter keys");
    assert!(raised_diff.edition_profile.is_none(), "replace-geometry-parameters/rescales-geom-valve-50-to-half-and-adds-clearance: geometry tuning has nothing to do with sheet edition profiles");''',
)

# ── 17. create-curve ───────────────────────────────────────────────────────────
CURVE_DP = {
    "id": "curve.dp",
    "x_unit": PCT,
    "y_unit": {"symbol": "kPa", "kind": "Pressure", "delta": True, "si_factor": 1000.0},
    "points": [{"x": 0.0, "y": 0.0}, {"x": 50.0, "y": 12.5}, {"x": 100.0, "y": 25.0}],
}


def _create_curve(s):
    s["curves"]["curve.dp"] = copy.deepcopy(CURVE_DP)


case(
    "🏝️create-curve",
    "adds-the-curve-dp-pressure-drop-curve",
    "A second characteristic curve — differential pressure in kPa over valve travel in % — is "
    "inserted into the id-keyed `curves` map. A duplicate id would be `mutation.duplicate-id` "
    "(fatal); `curve.dp` is fresh.",
    _create_curve,
    {"CreateCurve": {"curve": CURVE_DP}},
    ["curves"],
    '''    assert!(snapshot.curves.contains_key("curve.dp"), "create-curve/adds-the-curve-dp-pressure-drop-curve: the new curve id must be present");
    assert_eq!(snapshot.curves.len(), 2, "create-curve/adds-the-curve-dp-pressure-drop-curve: the pre-existing curve.kvs must survive");
    assert_eq!(snapshot.curves["curve.dp"].y_unit.symbol, "kPa", "create-curve/adds-the-curve-dp-pressure-drop-curve: the ordinate unit must be kPa");
    assert_eq!(snapshot.curves["curve.dp"].points.len(), 3, "create-curve/adds-the-curve-dp-pressure-drop-curve: the curve must carry its three interpolation points");''',
    '''    assert_eq!(raised_diff.curves.as_ref().map(|map| map.len()), Some(2), "create-curve/adds-the-curve-dp-pressure-drop-curve: the diff republishes the whole curves map, now holding two curves");
    assert!(raised_diff.geometry.is_none(), "create-curve/adds-the-curve-dp-pressure-drop-curve: curves and parametric geometry are separate id spaces");''',
)

# ── 18. delete-curve ───────────────────────────────────────────────────────────
def _delete_curve(s):
    del s["curves"]["curve.kvs"]


case(
    "🐢delete-curve",
    "removes-the-curve-kvs-flow-curve",
    "The only characteristic curve is dropped. `VLV-50-001`'s `configuration.function_refs` still "
    "names `curve.kvs` — as with geometry, deleting a curve does not cascade into the catalogue.",
    _delete_curve,
    {"DeleteCurve": {"id": "curve.kvs"}},
    ["curves"],
    '''    assert!(snapshot.curves.is_empty(), "delete-curve/removes-the-curve-kvs-flow-curve: the curves map must end up empty");
    assert_eq!(snapshot.catalog.products[0].configuration.function_refs, vec!["curve.kvs".to_string()], "delete-curve/removes-the-curve-kvs-flow-curve: the dangling function_ref is left in place — no cascade");''',
    '''    assert_eq!(raised_diff.curves.as_ref().map(|map| map.len()), Some(0), "delete-curve/removes-the-curve-kvs-flow-curve: the diff must publish an empty curves map, not omit the field");
    assert!(raised_diff.catalog.is_none(), "delete-curve/removes-the-curve-kvs-flow-curve: the referencing product is not rewritten");''',
)

# ── 19. replace-curve-points ───────────────────────────────────────────────────
NEW_POINTS = [{"x": 0.0, "y": 0.0}, {"x": 50.0, "y": 2.25}, {"x": 100.0, "y": 4.5}]


def _replace_curve_points(s):
    s["curves"]["curve.kvs"]["points"] = copy.deepcopy(NEW_POINTS)


case(
    "🏞️replace-curve-points",
    "resamples-curve-kvs-onto-three-points",
    "`curve.kvs`'s two-point ramp is resampled onto three points by inserting the 50 % midpoint at "
    "2.25 m3/h. Both units are left as-is; only `points` inside the republished curves map changes.",
    _replace_curve_points,
    {"ReplaceCurvePoints": {"id": "curve.kvs", "new_points": NEW_POINTS}},
    ["curves"],
    '''    let points = &snapshot.curves["curve.kvs"].points;
    assert_eq!(points.len(), 3, "replace-curve-points/resamples-curve-kvs-onto-three-points: the curve must carry three points afterwards");
    assert_eq!(points[1].x, 50.0, "replace-curve-points/resamples-curve-kvs-onto-three-points: the inserted midpoint must sit at 50 % travel");
    assert_eq!(points[1].y, 2.25, "replace-curve-points/resamples-curve-kvs-onto-three-points: the inserted midpoint must read 2.25 m3/h");
    assert_eq!(snapshot.curves["curve.kvs"].x_unit, before().curves["curve.kvs"].x_unit, "replace-curve-points/resamples-curve-kvs-onto-three-points: resampling must not rewrite the abscissa unit");''',
    '''    assert_eq!(raised_diff.curves.as_ref().map(|map| map["curve.kvs"].points.len()), Some(3), "replace-curve-points/resamples-curve-kvs-onto-three-points: the diff must publish the curves map with the three-point list");
    assert!(raised_diff.manufacturer_file.is_none(), "replace-curve-points/resamples-curve-kvs-onto-three-points: a curve resample is not a header edit");''',
)

# ── emit ──────────────────────────────────────────────────────────────────────
assert len(CASES) == 19, len(CASES)

for entry in CASES:
    kind = KIND[entry["leaf"]]
    rust = common.test_source(
        artifact="vdi3805",
        snapshot_ty="Vdi3805Snapshot",
        diff_ty="Vdi3805Diff",
        mutation_ty="Vdi3805Mutation",
        kind=kind,
        case=entry["case"],
        summary=entry["summary"],
        extra_applied=entry["extra_applied"],
        extra_diff=entry["extra_diff"],
    )
    common.emit_case(ROOT, entry["leaf"], entry["case"], BEFORE, entry["after"], entry["mutation"], entry["diff"], APPLIED, rust)

MODULE = {
    "🏕️update-manufacturer-file": "update_manufacturer_file",
    "🏜️change-correction-as-of": "change_correction_as_of",
    "🦋change-strict-mode": "change_strict_mode",
    "🦈update-limits": "update_limits",
    "🐝change-edition-profile": "change_edition_profile",
    "⛰️remove-edition-profile": "remove_edition_profile",
    "🪵create-product": "create_product",
    "🐳delete-product": "delete_product",
    "🏖️rename-product": "rename_product",
    "🗻replace-product-configuration": "replace_product_configuration",
    "🦭create-geometry": "create_geometry",
    "🐬delete-geometry": "delete_geometry",
    "🏟️resize-geometry": "resize_geometry",
    "🐞add-geometry-connection": "add_geometry_connection",
    "🏔️remove-geometry-connection": "remove_geometry_connection",
    "🐌replace-geometry-parameters": "replace_geometry_parameters",
    "🏝️create-curve": "create_curve",
    "🐢delete-curve": "delete_curve",
    "🏞️replace-curve-points": "replace_curve_points",
}

lines = []
for entry in CASES:
    leaf = entry["leaf"]
    module = "tests_{}_{}".format(MODULE[leaf], entry["case"].replace("-", "_"))
    lines.append('    #[path = "{}/🧪️tests/{}/🦀️component.rs"]'.format(leaf, entry["case"]))
    lines.append("    mod {};".format(module))
print("\n".join(lines))
