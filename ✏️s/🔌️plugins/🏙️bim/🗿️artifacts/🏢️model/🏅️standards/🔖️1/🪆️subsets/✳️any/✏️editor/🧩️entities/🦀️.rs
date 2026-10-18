//! 🧩️ The BIM entity table: every entity kind of the snapshot with its tree placement, its authored parameter rows and the mutations that create, rename
//! and delete it. The outliner, the properties panel, the library, the id mint and the delete/rename/set/create commands all read this table, so wiring a new
//! mutation kind is one row (or one `write` on a field row) here and nowhere else.

use crate::editor::bim::terminology::BimLabels;
use crate::standards::v1::subsets::any::schema::{snapshot::{Axis, ModelSnapshot, OpeningKind, TopConstraint}, inferences::ModelInference, mutations::ModelMutation};
use semio_framework_plugin::plugin_app_close_prelude::InputKind;
use semio_framework_ui_locale::LabelText;

pub use semio_s_artifact_bim_model_viewer::editor::bim::entities::row_types;
pub use row_types::*;

//#region 🔖️Helpers
use crate::standards::v1::subsets::any::schema::{diff::Assigned, snapshot::{Baluster, DoorLeaves, EndJoin, Infill, Layer, LayerFunction, LocationLine, MaterialCategory, Phase, Point2, Profile, Rgb, RiserKind, RoofShape, SpaceBoundary, StairFlight, StairStringer, StringerKind, Swing, Vertex}};
use semio_framework_plugin::DslValue;
use semio_framework_value::{FromValue, ToValue};
use std::collections::BTreeMap;

pub use semio_s_artifact_bim_model_authored::editor::bim::entities::field_codecs;
pub use field_codecs::*;

macro_rules! set {
    ($module:ident :: $leaf:ident, $id:expr, $field:literal, $value:expr) => {
        crate::editor::bim::entities::partial::<crate::standards::v1::subsets::any::schema::mutations::$module::$leaf, _>($id, $field, $value).map(crate::standards::v1::subsets::any::schema::mutations::ModelMutation::$leaf)
    };
}

macro_rules! delete {
    ($module:ident :: $leaf:ident) => {
        Some(|id| crate::standards::v1::subsets::any::schema::mutations::ModelMutation::$leaf(crate::standards::v1::subsets::any::schema::mutations::$module::$leaf { id: id.into() }))
    };
}

macro_rules! renaming {
    ($module:ident :: $leaf:ident) => {
        Some(|_, id, name| set!($module::$leaf, id, "name", &name.to_string()))
    };
}

pub fn rename_element(_: &ModelSnapshot, id: &str, name: &str) -> Option<ModelMutation> {
    Some(crate::standards::v1::subsets::any::schema::mutations::ModelMutation::RenameElement(crate::standards::v1::subsets::any::schema::mutations::rename_element::RenameElement { id: id.into(), name: name.into() }))
}

macro_rules! field {
    ($key:literal, $label:ident, $input:ident, |$s:pat_param, $id:pat_param| $read:expr) => {
        crate::editor::bim::entities::FieldRow { key: $key, label: |labels| labels.$label, input: semio_framework_plugin::plugin_app_close_prelude::InputKind::$input, read: |$s, $id| $read, write: None, choices: None }
    };
    ($key:literal, $label:ident, $input:ident, |$s:pat_param, $id:pat_param| $read:expr, rename) => {
        crate::editor::bim::entities::FieldRow { key: $key, label: |labels| labels.$label, input: semio_framework_plugin::plugin_app_close_prelude::InputKind::$input, read: |$s, $id| $read, write: Some(crate::editor::bim::entities::rename_element), choices: None }
    };
    ($key:literal, $label:ident, $input:ident, |$s:pat_param, $id:pat_param| $read:expr, $parse:ident => $module:ident :: $leaf:ident) => {
        crate::editor::bim::entities::FieldRow { key: $key, label: |labels| labels.$label, input: semio_framework_plugin::plugin_app_close_prelude::InputKind::$input, read: |$s, $id| $read, write: Some(|_, id, value| set!($module::$leaf, id, $key, &$parse(value)?)), choices: None }
    };
    ($key:literal, $label:ident, $input:ident, |$s:pat_param, $id:pat_param| $read:expr, |$ws:pat_param, $wid:pat_param, $value:pat_param| $write:expr) => {
        crate::editor::bim::entities::FieldRow { key: $key, label: |labels| labels.$label, input: semio_framework_plugin::plugin_app_close_prelude::InputKind::$input, read: |$s, $id| $read, write: Some(|$ws, $wid, $value| $write), choices: None }
    };
    ($key:literal, $label:ident, $input:ident, |$s:pat_param, $id:pat_param| $read:expr, choices: $choices:path, $parse:ident => $module:ident :: $leaf:ident) => {
        crate::editor::bim::entities::FieldRow { key: $key, label: |labels| labels.$label, input: semio_framework_plugin::plugin_app_close_prelude::InputKind::$input, read: |$s, $id| $read, write: Some(|_, id, value| set!($module::$leaf, id, $key, &$parse(value)?)), choices: Some($choices) }
    };
    ($key:literal, $label:ident, $input:ident, |$s:pat_param, $id:pat_param| $read:expr, choices: $choices:path, write: $write:path) => {
        crate::editor::bim::entities::FieldRow { key: $key, label: |labels| labels.$label, input: semio_framework_plugin::plugin_app_close_prelude::InputKind::$input, read: |$s, $id| $read, write: Some($write), choices: Some($choices) }
    };
}

macro_rules! inferred {
    ($key:literal, $label:ident, |$s:pat_param, $inference:pat_param, $id:pat_param| $read:expr) => {
        crate::editor::bim::entities::InferredRow { key: $key, label: |labels| labels.$label, read: |$s, $inference, $id| $read }
    };
}

macro_rules! kind {
    ($kind:literal, $icon:literal, $library:literal, $label:ident, $group:ident, $coll:ident . $name:ident, parent: |$ps:pat_param, $pid:pat_param| $parent:expr, $($rest:tt)*) => {
        kind!(@build $kind, $icon, $library, $label, $group, $coll . $name, |$ps, $pid| $parent, $($rest)*)
    };
    (@build $kind:literal, $icon:literal, $library:literal, $label:ident, $group:ident, $coll:ident . $name:ident, |$ps:pat_param, $pid:pat_param| $parent:expr, delete: $delete:expr, rename: $rename:expr, create: $create:expr, fields: $fields:expr, inferred: $inferred:expr $(,)?) => {
        crate::editor::bim::entities::EntityKind {
            kind: $kind,
            icon: $icon,
            library: $library,
            label: |labels| labels.$label,
            group: |labels| labels.$group,
            ids: |snapshot| crate::editor::bim::entities::keys(&snapshot.$coll),
            name: |snapshot, id| snapshot.$coll.get(id).map(|row| row.$name.clone()),
            parent: |$ps, $pid| $parent,
            delete: $delete,
            rename: $rename,
            create: $create,
            fields: $fields,
            inferred: $inferred,
        }
    };
}
//#endregion 🔖️Helpers

#[path = "🖼️views/🦀️.rs"]
pub mod views;

#[path = "📋️schedules/🦀️.rs"]
pub mod schedules;

#[path = "🏘️zoning/🦀️.rs"]
pub mod zoning;

#[path = "🪧️notations/🦀️.rs"]
pub mod notations;

#[path = "📄️sheets/🦀️.rs"]
pub mod sheets;

#[path = "🤝️coordination/🦀️.rs"]
pub mod coordination;

#[path = "🔲️ceilings/🦀️.rs"]
pub mod ceilings;

pub use semio_s_artifact_bim_model_viewer::editor::bim::entities::phasing;

#[path = "🛝️ramps/🦀️.rs"]
pub mod ramps;

#[path = "🧷️wall-sweeps/🦀️.rs"]
pub mod wall_sweeps;

#[path = "🏷️psets/🦀️.rs"]
pub mod psets;

pub use semio_s_artifact_bim_model_viewer::editor::bim::entities::frame;

#[path = "🧬️families/🦀️.rs"]
pub mod families;

#[path = "🪑️components/🦀️.rs"]
pub mod components;

#[path = "🌀️mep/🦀️.rs"]
pub mod mep;

#[path = "🌡️energy/🦀️.rs"]
pub mod energy;

#[path = "🧭️options/🦀️.rs"]
pub mod options;

#[path = "🧮️field-tables/🦀️.rs"]
pub mod field_tables;
pub use field_tables::*;

pub use semio_s_artifact_bim_model_viewer::editor::bim::entities::create_rows;
pub use create_rows::*;

#[path = "🗂️entity-table/🦀️.rs"]
mod entity_table;
pub use entity_table::*;

#[path = "🔎️project-and-lookup/🦀️.rs"]
mod project_and_lookup;
pub use project_and_lookup::*;

#[path = "🧾️properties/🦀️.rs"]
pub mod properties;
pub use properties::*;


