import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib_spec import *
F = ROOT + "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📚️document-checks/🗃️catalog-integrity/🗃️product-and-index-checks/🦀️.rs"
c1 = child("🪪", "records", "Product identity and record checks: the identity, the configuration id, the part 7 record and the native part 1 records of one catalogue product.")
c2 = child("🔗", "links", "Product link and geometry checks: the dangling accessory and component links, the link quantities and the geometry reference of one catalogue product.")
M = "semio_s_artifact_norm_vdi3805_model::"
ctx = {"document": "&Vdi3805Snapshot", "out": "&mut Vec<CheckResult>", "product_ids": "&BTreeSet<String>", "product": "&" + M + "CatalogueProduct"}
types = {"article": "&str", "id_ok": "bool"}
modes = {"id_ok": "val"}
spec = {"file": F, "function": "check_products", "block_line": 74, "context": ctx, "types": types, "modes": modes, "keep": [75], "phases": [
    ph("check_product_identity", "🪪 Checks the product id against the identity record 100 fields.", 76, 113, c1),
    ph("check_product_configuration", "⚙ Checks that the configuration id is cfg dot the article number.", 116, 142, c1),
    ph("check_product_record_700", "🧾 Checks the part 7 record of the product.", 144, 144, c1),
    ph("check_native_records", "📚 Checks that every native record echoes its family code and the identity fields.", 185, 185, c1),
    ph("check_dangling_links", "🔗 Checks that every accessory and component link points at a declared product.", 241, 315, c2),
    ph("check_link_quantities", "🔢 Checks the accessory and component quantities and the required accessories.", 318, 378, c2),
    ph("check_product_geometry", "📐 Checks the geometry bounding box, connections and parameters when a geometry reference is set.", 406, 406, c2),
]}
dump(spec, sys.argv[1])
