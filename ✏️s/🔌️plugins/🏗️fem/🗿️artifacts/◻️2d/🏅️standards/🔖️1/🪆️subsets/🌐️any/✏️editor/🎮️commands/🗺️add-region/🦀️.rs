//! 🧱️ 🧱️ Fem2d play app commands command — `add-region`.

use semio_plugin::NoConfig;

use semio_plugin::NoConfigMutation;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemRegion;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use semio_plugin::Emit;
use semio_plugin::Fault;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem2dSnapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️AddNode
//#endregion 🔖️AddNode

//#region 🔖️AddBar
//#endregion 🔖️AddBar

//#region 🔖️AddBeam
//#endregion 🔖️AddBeam

//#region 🔖️AddMaterial
//#endregion 🔖️AddMaterial

//#region 🔖️AddSection
//#endregion 🔖️AddSection

//#region 🔖️AddSupport
//#endregion 🔖️AddSupport

//#region 🔖️AddRegion
//#endregion 🔖️AddRegion

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "add-region")]
pub struct AddRegion {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub material_id: String,
    pub thickness: Option<f64>,
    pub mesh_size: Option<f64>,
}

pub fn handle(payload: &AddRegion, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let refuse = |code: &'static str, message: String| Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new(code), message);
    if !doc.snapshot.materials.iter().any(|material| material.id.as_str() == payload.material_id) {
        return Err(refuse("fem2d.region.material-unknown", format!("'{}' is not a material of this document", payload.material_id)).with_param("material", payload.material_id.as_str()));
    }
    if ![payload.width, payload.height].iter().all(|extent| extent.is_finite() && *extent > 0.0) {
        return Err(refuse("fem2d.region.extent", "a region needs a positive width and height".into()));
    }
    let id = doc.operation()?.entity_id("r", "regions")?;
    let outline = vec![[payload.x, payload.y], [payload.x + payload.width, payload.y], [payload.x + payload.width, payload.y + payload.height], [payload.x, payload.y + payload.height]];
    let region = FemRegion { id, name: "Region".into(), outline, holes: Vec::new(), thickness: payload.thickness.unwrap_or(0.02), material_id: payload.material_id.clone(), mesh_size: payload.mesh_size.unwrap_or(0.25) };
    Ok(Emit::mutations(vec![Fem2dMutation::CreateRegion(semio_s_artifact_fem_2d_mesh_regions::standards::v1::subsets::any::schema::mutations::create_region::CreateRegion { region, index: None })]))
}
