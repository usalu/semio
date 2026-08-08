//! ➕️ Lowpoly play app command — appends a new primitive mesh object and makes it active. A lone
//! command in its group, so (per TEMPLATE.md §5.7's `module_inception` rule) the payload lives directly
//! at this file's top level rather than in a same-named inner `pub mod`.

use crate::apps::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::apps::lowpoly::session::LowpolyScratch;
use crate::apps::lowpoly::view::{build_doc, primitive_kind};
use crate::artifacts::lowpoly::op::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolyProjection;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️AddPrimitive
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-primitive")]
pub struct AddPrimitive {
    pub kind: Option<String>,
}

pub fn handle(payload: &AddPrimitive, doc: &DocumentView<'_, LowpolyProjection>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
    let projection = doc.projection;
    let kind = primitive_kind(payload.kind.as_deref().unwrap_or("box")).to_string();
    let Some(mut build) = build_doc(projection, cfg.projection) else { return Ok(Emit::default()) };
    let Ok(new_id) = build.add_primitive(&kind) else { return Ok(Emit::default()) };
    if build.sync_meshes_to_projection().is_err() {
        return Ok(Emit::default());
    }
    let Some(new_object) = build.projection().objects.iter().find(|object| object.id == new_id).cloned() else {
        return Ok(Emit::default());
    };
    let index = projection.objects.len();
    Ok(Emit {
        document_mutations: vec![LowpolyMutation::ObjectsAdd { index, item: new_object }],
        config_mutations: vec![
            LowpolyConfigMutation::SetActiveObject { object_id: new_id },
            LowpolyConfigMutation::SetSelectionTargets { mesh: true, vertex: false, edge: false, face: false },
            LowpolyConfigMutation::SetSelection { mode: "mesh".into(), ids: Vec::new() },
            LowpolyConfigMutation::SetSelectionKeys { keys: Vec::new() },
        ],
        ..Default::default()
    })
}
//#endregion 🔖️AddPrimitive

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::lowpoly::testkit::{app, dispatch};
    use crate::apps::lowpoly::LowpolyCommand;

    #[test]
    fn add_primitive_emits_objects_add_operation() {
        let mut a = app();
        dispatch(&mut a, LowpolyCommand::AddPrimitive(AddPrimitive { kind: Some("box".into()) }));
        let projection = a.projection().expect("projection");
        assert_eq!(projection.objects.len(), 2);
        assert!(projection.objects.iter().any(|object| object.name == "box"));
    }

    #[test]
    fn add_primitive_supports_every_known_kind() {
        let mut a = app();
        for kind in ["plane", "cylinder", "cone", "ico_sphere"] {
            dispatch(&mut a, LowpolyCommand::AddPrimitive(AddPrimitive { kind: Some(kind.into()) }));
        }
        assert_eq!(a.projection().expect("projection").objects.len(), 5);
    }
}
//#endregion 🧪️Tests
