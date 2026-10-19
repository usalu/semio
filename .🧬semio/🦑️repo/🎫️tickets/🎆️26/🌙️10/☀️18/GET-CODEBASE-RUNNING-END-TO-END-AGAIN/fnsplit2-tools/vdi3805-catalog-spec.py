import sys, glob
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib_spec import *
F = glob.glob(ROOT + "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📚️document-checks/🗃️catalog-integrity/🦀️.rs")[0]
c1 = child("📏", "limit-and-extension-checks", "Limit and extension checks: the catalog limits and the extension field bags of the catalog integrity check.")
c2 = child("🗓", "correction-date-and-strict-mode-checks", "Correction date and strict mode checks: the correction date, its chronology and the strict mode header of the catalog integrity check.")
c3 = child("🗃", "product-and-index-checks", "Product and index checks: the product ids, the dangling index entries, the per product geometry and the index to product consistency of the catalog integrity check.")
types = {"out": "Vec<CheckResult>", "product_ids": "BTreeSet<String>", "lim": "&semio_s_artifact_norm_vdi3805_model::SecurityLimits", "actual_records": "usize", "actual_field_len": "usize", "actual_bytes": "usize", "actual_depth": "usize"}
modes = {k: "val" for k in ["actual_records", "actual_field_len", "actual_bytes", "actual_depth"]}
modes["out"] = "mut"
keep = [6, 7, 8, 9, 10, 19, 29, 30, 33, 36, 39, 909, 911]
spec = {"file": F, "function": "check_catalog_integrity", "types": types, "modes": modes, "keep": keep, "phases": [
    ph("check_limits", "📏 Checks the record count, the file bytes, the field length and the nesting depth against the declared limits.", 42, 42, c1),
    ph("check_extension_fields", "🧩 Checks the extension field bags of the file, the catalog and the products.", 76, 76, c1),
    ph("check_correction_date", "🗓 Checks the correction date range and its chronology against the file creation date.", 118, 167, c2),
    ph("check_strict_mode", "🔒 Checks the charset and the header version of the strict mode.", 169, 169, c2),
    ph("check_unique_product_ids", "🆔 Checks that every product id is unique.", 219, 245, c3),
    ph("check_dangling_index_entries", "🔗 Checks that every index entry points at a declared product.", 247, 278, c3),
    ph("check_products", "📦 Checks the identity, the records and the geometry of every product.", 280, 280, c3),
    ph("check_index_entries", "📇 Checks the sheet, the nominal size and the tags of every index entry against its product.", 810, 810, c3, allow_control=[811]),
]}
dump(spec, sys.argv[1])
