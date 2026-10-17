"""🔧 E51 wave-7 part 2: dwg records widening and the process3d contributed catalog owner constructor."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from widen import widen
from ed import edit
D = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📤️export/🧱️records/🗜️container/🦀️.rs"
widen(D, "EncodedR2004Page", 136, "pub", ["section_id", "page_number", "start_offset", "payload", "address", "allocation_size"])
widen(D, "R2004SectionDescriptor", 195, "pub", ["name", "section_id", "semantic_size", "max_decompressed_size", "compression", "encryption"])
P = "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/"
edit(P + "🧬️schema/📸️snapshot/🦀️.rs", """    machines: Vec<WorkshopMachine>,
}
impl MachineCatalog for ContributedMachineCatalog {""", """    machines: Vec<WorkshopMachine>,
}
impl ContributedMachineCatalog {
    /// 🧩️ Seals one contributed catalog from the identity and machine list the topic contribution decoded.
    pub fn new(catalog_id: String, label: String, icon_id: String, machines: Vec<WorkshopMachine>) -> Self {
        Self { catalog_id, label, icon_id, machines }
    }
}
impl MachineCatalog for ContributedMachineCatalog {""")
edit(P + "✏️editor/🧰️kit/🦀️.rs", "catalogs.push(ContributedMachineCatalog { catalog_id: module_id, label, icon_id, machines });", "catalogs.push(ContributedMachineCatalog::new(module_id, label, icon_id, machines));")
