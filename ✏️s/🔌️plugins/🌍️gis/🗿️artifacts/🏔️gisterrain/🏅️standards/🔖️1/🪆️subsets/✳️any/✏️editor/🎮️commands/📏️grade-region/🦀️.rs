//! 📏️ Grades elevation samples covered by one authored GIS region.

use crate::mutations::{GisTerrainMutation, ReplaceElevationSource};
use crate::schema::snapshot::ElevationSource;
use semio_s_artifact_gis_gisterrain_inferences::elevation_edit::metres_to_millimetres;
use crate::schema::terrain_descriptor::{TerrainOverlays, TerrainRegionData};
use crate::GisTerrainSnapshot;
use semio_framework_geometry_primitives::{Point, geom_sel::{polygon_covers_point, world_box_contains_point, world_box_from_points}};
use semio_framework_value::DslValue;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_plugin::{ActionArgDef, ArgSchema, ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_ui_locale::LocalizedLabel;

const MAX_REGION_VERTICES: usize = 512;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "grade-terrain-region")]
pub struct GradeTerrainRegion {
    pub id: String,
    pub height_millimetres: i32,
}

impl GradeTerrainRegion {
    pub fn from_action(args: &DslValue) -> Result<Self, Fault> {
        let id = args.get("id").and_then(DslValue::as_str).filter(|id| !id.trim().is_empty() && id.len() <= 128)
            .ok_or_else(|| Fault::from("gis-terrain-grading-region-id-required"))?;
        let height_millimetres = args.get("height").and_then(DslValue::as_f64).and_then(metres_to_millimetres)
            .ok_or_else(|| Fault::from("gis-terrain-grading-height-invalid"))?;
        Ok(Self { id: id.into(), height_millimetres })
    }
}

/// 🎛️ Stages the GIS feature ID and the target height.
pub fn action_arguments() -> Vec<ActionArgDef> {
    vec![
        ActionArgDef::text("id", LocalizedLabel::native("Region ID", "Regions-ID")).required(),
        ActionArgDef::with_schema("height", LocalizedLabel::native("Target Height in Meters", "Zielhöhe in Metern"), ArgSchema::number(Some(f64::from(i32::MIN) / 1000.0), Some(f64::from(i32::MAX) / 1000.0), Some(0.001), false)).default_value(&100.0_f64),
    ]
}

fn apply(source: &ElevationSource, region: &TerrainRegionData, height_millimetres: i32) -> Result<ElevationSource, Fault> {
    source.validate().map_err(|error| Fault::from(error.to_string()))?;
    if region.rings.iter().map(Vec::len).sum::<usize>() > MAX_REGION_VERTICES {
        return Err(Fault::from("gis-terrain-grading-region-exceeds-512-vertices"));
    }
    let mut rings = region.rings.iter().map(|ring| ring.iter().map(|&[x, y]| Point::new(x, y)).collect::<Vec<_>>());
    let outer = rings.next().ok_or_else(|| Fault::from("gis-terrain-grading-region-empty"))?;
    let bounds = world_box_from_points(&outer).ok_or_else(|| Fault::from("gis-terrain-grading-region-empty"))?;
    let holes = rings.collect::<Vec<_>>();
    let [west, south, east, north] = source.bounds;
    let width = source.rows_millimetres[0].len();
    let mask = source.missing_samples.as_deref();
    let x_segments = (width - 1) as f64;
    let y_segments = (source.rows_millimetres.len() - 1) as f64;
    let mut result = source.clone();
    for (row_index, row) in result.rows_millimetres.iter_mut().enumerate() {
        let latitude = south + (north - south) * (row_index as f64 / y_segments);
        for (column, sample) in row.iter_mut().enumerate() {
            if semio_framework_surface::terrain::missing_samples::is_missing(mask, row_index * width + column) {
                continue;
            }
            let point = Point::new(west + (east - west) * (column as f64 / x_segments), latitude);
            if world_box_contains_point(bounds, point) && polygon_covers_point(&outer, point)
                && !holes.iter().any(|hole| polygon_covers_point(hole, point)) {
                *sample = height_millimetres;
            }
        }
    }
    Ok(result)
}

/// 🧾️ Produces one reversible edit preserving feature geometry and holes.
pub fn handle(payload: &GradeTerrainRegion, doc: &ArtifactView<'_, GisTerrainSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisTerrainMutation, NoConfigMutation>, Fault> {
    let source = doc.snapshot.elevation_source.as_ref().ok_or_else(|| Fault::from("gis-terrain-elevation-source-required"))?;
    let region = doc.snapshot.imported_map.as_ref().and_then(|map| map.regions.iter().find(|feature| {
        feature.get("id").and_then(DslValue::as_str) == Some(payload.id.as_str())
    })).ok_or_else(|| Fault::from("gis-terrain-grading-region-not-found"))?;
    let overlays = TerrainOverlays::from_map_features(&[], &[], std::slice::from_ref(region));
    let region = overlays.regions.first().ok_or_else(|| Fault::from("gis-terrain-grading-region-invalid"))?;
    let next = apply(source, region, payload.height_millimetres)?;
    if next == *source {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![GisTerrainMutation::ReplaceElevationSource(ReplaceElevationSource { new_elevation_source: Some(next) })]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn grades_the_region_without_altering_its_hole_or_outside_samples() {
        let fixture: Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🎮️commands/📏️grade-region/🧪️tests/🔣️.json")).unwrap();
        let initial_height = fixture["initialHeightMillimetres"].as_i64().unwrap() as i32;
        let source = ElevationSource {
            missing_samples: None,
            bounds: serde_json::from_value(fixture["bounds"].clone()).unwrap(),
            rows_millimetres: vec![vec![initial_height; fixture["columns"].as_u64().unwrap() as usize]; fixture["rows"].as_u64().unwrap() as usize],
        };
        let decode = |value: &Value| {
            let args = semio_framework_pack_json::parse(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
            GradeTerrainRegion::from_action(&semio_framework_pack_json::to_dsl_value(&args))
        };
        let command = decode(&fixture["command"]).unwrap();
        for case in fixture["heightLimits"].as_array().unwrap() {
            assert_eq!(i64::from(decode(&case["command"]).unwrap().height_millimetres), case["heightMillimetres"].as_i64().unwrap());
        }
        for invalid in fixture["invalidActions"].as_array().unwrap() {
            assert!(decode(invalid).is_err(), "{invalid}");
        }
        let region = TerrainRegionData {
            id: command.id,
            name: "Construction site".into(),
            rings: serde_json::from_value(fixture["rings"].clone()).unwrap(),
        };
        let result = apply(&source, &region, command.height_millimetres).unwrap();
        assert_eq!(result.rows_millimetres.iter().flatten().filter(|sample| **sample != initial_height).count(), fixture["graded"].as_u64().unwrap() as usize);
        for sample in fixture["samples"].as_array().unwrap() {
            let [column, row, expected] = serde_json::from_value::<[i32; 3]>(sample.clone()).unwrap();
            assert_eq!(result.rows_millimetres[row as usize][column as usize], expected);
        }
        assert_eq!(source.rows_millimetres[2][4], initial_height);
    }

    #[semio_framework_async_macros::async_test]
    async fn grading_the_demo_region_keeps_the_map_and_is_undoable() {
        use crate::editor::gis3d::{Gis3dCommand, unit_tests::context};

        let mut app = context::app().await;
        let before = app.snapshot().unwrap();
        let command = Gis3dCommand::GradeTerrainRegion(GradeTerrainRegion { id: "g_sart_tilman".into(), height_millimetres: 300_000 });
        let receipt = context::dispatch(&mut app, command).await;
        assert!(receipt.lanes.contains(&semio_plugin::app::TypedOperationResultLane::Artifact));
        let after = app.snapshot().unwrap();
        assert_ne!(after.elevation_source, before.elevation_source);
        assert_eq!(after.imported_map, before.imported_map);
        context::history_verb(&mut app, "undo").await;
        assert_eq!(app.snapshot().unwrap(), before);
        context::close(&mut app);
    }
}
