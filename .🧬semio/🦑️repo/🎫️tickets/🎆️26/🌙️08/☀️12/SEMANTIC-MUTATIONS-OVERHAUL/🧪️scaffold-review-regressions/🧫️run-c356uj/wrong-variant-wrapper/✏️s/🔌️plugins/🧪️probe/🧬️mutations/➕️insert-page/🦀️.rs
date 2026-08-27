//! 🧬️ SCAFFOLD: authoritative direct mutation owner for `insert-page`.
//! @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM

//#region 🪪️Descriptor
pub const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "page", kind: "insert-page", record: "InsertPage" };
//#endregion 🪪️Descriptor

//#region 🧬️Mutation
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Mutation;
//#endregion 🧬️Mutation
