//! 🪆️ Layout persists genuine Drawing member references while authored births retain a separate original payload.
use semio_framework_value::{ToValue,FromValue};
#[derive(Clone,Debug,PartialEq,ToValue,FromValue,semio_framework_value::RetainedClone,semio_framework_value::RetireOwned,semio_framework_value::CanonicalJsonTree,semio_framework_dsl_record_derive::DslRecord)]
#[canonical_json(owner=semio_framework_pack_json)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct LayoutDrawingMember{pub handle:store::os_store::artifact_child::ArtifactChild<super::LayoutBackgroundDrawing>}
impl semio_framework_schema_composition::ChildFieldRefs for LayoutDrawingMember{
 const MANY:bool=false;
 fn visit_child_field<'a,V:semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self,slot:&'static str,visitor:&mut V)->Result<(),V::Error>{visitor.step()?;semio_framework_schema_composition::ChildFieldRefs::visit_child_field(&self.handle,slot,visitor)}
}
#[derive(Clone,Debug,PartialEq,ToValue,FromValue,semio_framework_value::RetainedClone,semio_framework_value::RetireOwned)]
#[value(tag="kind",rename_all="camelCase",deny_unknown_fields)]
pub enum LayoutDrawingBirthSource{Document,Page{#[value(rename="pageId")]page_id:String}}
#[derive(semio_framework_value::RetireOwned)]
pub struct LayoutDrawingBirth<S:semio_framework_value::RetireOwned>{pub source:LayoutDrawingBirthSource,pub member:LayoutDrawingMember,pub initial:S}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

#[path="📖️source/🦀️.rs"]pub mod source;
pub use source::{drawing_member_at,drawing_read_demand,drawing_read_at,DrawingReadAdvance};

#[derive(semio_framework_value::RetireOwned)]
pub struct LayoutImportBundle<S:semio_framework_value::RetireOwned>{pub document:super::snapshot::LayoutSnapshot,pub drawing_births:semio_framework_value_persistent::list::PagedList<LayoutDrawingBirth<S>,{usize::MAX}>}

#[path="🌱️birth/📤️transfer/🦀️.rs"]pub mod transfer;
pub use transfer::{drawing_birth_transfer_demand,transfer_drawing_birth,DrawingBirthTransfer};
