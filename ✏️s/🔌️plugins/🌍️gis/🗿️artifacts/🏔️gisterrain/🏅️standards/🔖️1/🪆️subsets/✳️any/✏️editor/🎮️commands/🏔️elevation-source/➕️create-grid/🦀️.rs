//! ➕️ Authors a flat elevation grid at a user-specified WGS84 extent.

use crate::mutations::{GisTerrainMutation, ReplaceElevationSource};
use crate::schema::snapshot::ElevationSource;
use crate::GisTerrainSnapshot;
use semio_framework_value::DslValue;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_plugin::{ActionArgDef, ArgSchema, ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_ui_locale::LocalizedLabel;

const GRID_SIZE: usize = 33;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "create-elevation-grid")]
pub struct CreateElevationGrid {
    pub west: f64,
    pub south: f64,
    pub east: f64,
    pub north: f64,
    pub height: i16,
}

impl CreateElevationGrid {
    pub fn from_action(args: &DslValue) -> Result<Self, Fault> {
        let number = |key| args.get(key).and_then(DslValue::as_f64).filter(|value| value.is_finite())
            .ok_or_else(|| Fault::from(format!("gis-terrain-create-grid-invalid-{key}")));
        let height = number("height")?;
        if height.fract() != 0.0 || !(f64::from(i16::MIN)..=f64::from(i16::MAX)).contains(&height) {
            return Err(Fault::from("gis-terrain-create-grid-invalid-height"));
        }
        let command = Self { west: number("west")?, south: number("south")?, east: number("east")?, north: number("north")?, height: height as i16 };
        command.is_valid().then_some(command).ok_or_else(|| Fault::from("gis-terrain-create-grid-invalid-bounds"))
    }

    pub fn is_valid(&self) -> bool {
        (-180.0..self.east).contains(&self.west) && self.east <= 180.0
            && (-90.0..self.north).contains(&self.south) && self.north <= 90.0
    }

    fn source(&self) -> Result<ElevationSource, Fault> {
        if !self.is_valid() {
            return Err(Fault::from("gis-terrain-create-grid-invalid-bounds"));
        }
        Ok(ElevationSource {
            bounds: [self.west, self.south, self.east, self.north],
            rows: vec![vec![self.height; GRID_SIZE]; GRID_SIZE],
        })
    }
}

/// 🧭️ Stages the four geographic bounds and the initial height.
pub fn action_arguments() -> Vec<ActionArgDef> {
    let bound = |id, en, de, maximum: f64, default: f64| {
        ActionArgDef::with_schema(id, LocalizedLabel::native(en, de), ArgSchema::number(Some(-maximum), Some(maximum), None, false)).default_value(&default)
    };
    vec![
        bound("west", "West Longitude", "Westlicher Längengrad", 180.0, 5.55),
        bound("south", "South Latitude", "Südlicher Breitengrad", 90.0, 50.58),
        bound("east", "East Longitude", "Östlicher Längengrad", 180.0, 5.61),
        bound("north", "North Latitude", "Nördlicher Breitengrad", 90.0, 50.64),
        ActionArgDef::with_schema("height", LocalizedLabel::native("Initial Height in Meters", "Starthöhe in Metern"), ArgSchema::number(Some(f64::from(i16::MIN)), Some(f64::from(i16::MAX)), Some(1.0), true)).default_value(&100_i16),
    ]
}

/// 📦️ Records the generated samples as one reversible source replacement.
pub fn handle(payload: &CreateElevationGrid, doc: &ArtifactView<'_, GisTerrainSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisTerrainMutation, NoConfigMutation>, Fault> {
    let source = payload.source()?;
    if doc.snapshot.elevation_source.as_ref() == Some(&source) {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![GisTerrainMutation::ReplaceElevationSource(ReplaceElevationSource { new_elevation_source: Some(source) })]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::gis3d::{Gis3dPlayApp, unit_tests::context};
    use semio_framework_pack_json::{JsonMemberPolicy, from_json_str};
    use semio_plugin::ArtifactEditor as _;
    use serde_json::Value;

    #[semio_framework_async_macros::async_test]
    async fn creating_a_flat_grid_publishes_and_replays_only_the_elevation_source() {
        let fixture: Value = serde_json::from_str(include_str!("🧪️tests/🔣️.json")).unwrap();
        let args = |key: &str| from_json_str::<DslValue>(&fixture[key].to_string(), JsonMemberPolicy::Reject).unwrap();
        assert!(CreateElevationGrid::from_action(&args("invalidOrder")).is_err());
        assert!(CreateElevationGrid::from_action(&args("invalidHeight")).is_err());
        let command = Gis3dPlayApp::command_from_action("createElevationGrid", &mut Some(args("command")), &mut None).unwrap();
        let mut app = context::app().await;
        let before = app.snapshot().unwrap();
        let receipt = context::dispatch(&mut app, command).await;
        assert!(receipt.lanes.contains(&semio_plugin::app::TypedOperationResultLane::Artifact));
        let after = app.snapshot().unwrap();
        let grid = after.elevation_source.as_ref().unwrap();
        let expected = &fixture["expected"];
        assert_eq!(grid.bounds, serde_json::from_value::<[f64; 4]>(expected["bounds"].clone()).unwrap());
        assert_eq!(grid.rows.len(), expected["rows"].as_u64().unwrap() as usize);
        assert!(grid.rows.iter().all(|row| row.len() == expected["columns"].as_u64().unwrap() as usize
            && row.iter().all(|height| i64::from(*height) == expected["height"].as_i64().unwrap())));
        assert_eq!(after.imported_map, before.imported_map);
        context::history_verb(&mut app, "undo").await;
        assert_eq!(app.snapshot().unwrap(), before);
        context::history_verb(&mut app, "redo").await;
        assert_eq!(app.snapshot().unwrap(), after);
        context::close(&mut app);
    }
}
