//! 🧰️ Shared helpers of the glTF export tests: the committed house model, its fixture directory and a third-party reader (the `gltf` crate) over exported bytes.

use crate::ModelSnapshot;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

/// 🏠️ The committed export fixture: one site, one building, four storeys and every element family.
pub const HOUSE: &str = include_str!("../../../../../🧫️fixtures/🏗️ifc/🏠️house/📸️snapshot/🔣️.json");

/// 📁️ The directory of the committed glTF export of the house and the table the `three` oracle measured from it.
pub const HOUSE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧊️gltf/🏠️house");

/// 🏠️ The decoded house model.
pub fn house() -> ModelSnapshot {
    from_json_str(HOUSE, JsonMemberPolicy::Reject).expect("the committed house decodes")
}

/// 📖️ A committed file of the house fixture directory.
pub fn read(name: &str) -> Vec<u8> {
    std::fs::read(format!("{HOUSE_DIR}/{name}")).unwrap_or_else(|error| panic!("{name}: {error}. Run the test with BIM_BLESS=1 to write the file, then `bun 🟦️.ts write` of the export case."))
}

/// 📖️ The document of a GLB as the `gltf` crate reads it, with its binary chunk.
pub fn third_party(bytes: &[u8]) -> (gltf::Document, Vec<u8>) {
    let parsed = gltf::Gltf::from_slice(bytes).expect("the gltf crate reads the container and validates the document");
    (parsed.document, parsed.blob.unwrap_or_default())
}

/// 🪑️ The committed components model: free and hosted components, terminals of several systems, ducts, pipes and trays on two storeys.
pub const COMPONENTS: &str = include_str!("../../../../../🧫️fixtures/🏗️ifc/🪑️components/📸️snapshot/🔣️.json");

/// 📁️ The directory of the committed glTF export of the components model and the table the `three` oracle measured from it.
pub const COMPONENTS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧊️gltf/🪑️components");

/// 🪑️ The decoded components model.
pub fn components() -> ModelSnapshot {
    from_json_str(COMPONENTS, JsonMemberPolicy::Reject).expect("the committed components model decodes")
}

/// 📖️ A committed file of the components fixture directory.
pub fn read_components(name: &str) -> Vec<u8> {
    std::fs::read(format!("{COMPONENTS_DIR}/{name}")).unwrap_or_else(|error| panic!("{name}: {error}. Run the test with BIM_BLESS=1 to write the file, then `bun 🟦️.ts write` of the export case."))
}
