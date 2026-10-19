//! 🩹️ Fem2d play app command — `patch-element`: one-field edit of an element (`kind`, `start`, `end`, `materialId`, `sectionId`) → `ChangeElement`.

use semio_s_artifact_fem_2d_mesh::standards::v1::subsets::any::schema::mutations::change_element::ChangeElement;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dElementPatch;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem2dSnapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️PatchElement
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patch-element")]
pub struct PatchElement {
    pub id: String,
    pub field: String,
    pub value: String,
}

/// 🩹️ Re-points one element reference, or swaps `Bar`↔`Beam` while keeping the element's own id and
/// every other field — `FemElement`'s two variants carry the identical five fields, so a `kind` edit
/// is a variant swap and nothing else. Emitted as one `ChangeElement`.
pub fn handle(payload: &PatchElement, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let row = doc.snapshot.elements.iter().find(|row| element_id(row) == payload.id).ok_or_else(|| Fault::from("fem2d.patch.element-missing"))?;
    let mut patch = Fem2dElementPatch::default();
    let value = payload.value.trim();
    match payload.field.as_str() {
        "kind" if matches!(value, "bar" | "beam") => patch.kind = Some(value.into()),
        "kind" => return Err(Fault::from("fem2d.patch.element-value")),
        "start" => patch.start = Some(value.into()),
        "end" => patch.end = Some(value.into()),
        "materialId" => patch.material_id = Some(value.into()),
        "sectionId" => patch.section_id = Some(value.into()),

        _ => return Err(Fault::from("fem2d.patch.element-field")),
    }
    let candidate = patch.candidate(row).map_err(|_| Fault::from("fem2d.patch.element-field"))?;
    if candidate == *row { return Ok(Emit::default()); }
    Ok(Emit::mutations(vec![Fem2dMutation::ChangeElement(ChangeElement { id: payload.id.clone(), patch })]))
}
//#endregion 🔖️PatchElement

//#region 🧪️Tests
//#endregion 🧪️Tests
