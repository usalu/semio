//! 📏️ Grades elevation samples covered by one authored GIS region.

use crate::mutations::{GisTerrainMutation, ReplaceElevationSource};
use crate::schema::snapshot::ElevationSource;
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
    pub height: i16,
}

impl GradeTerrainRegion {
    pub fn from_action(args: &DslValue) -> Result<Self, Fault> {
        let id = args.get("id").and_then(DslValue::as_str).filter(|id| !id.trim().is_empty() && id.len() <= 128)
            .ok_or_else(|| Fault::from("gis-terrain-grading-region-id-required"))?;
        let height = args.get("height").and_then(DslValue::as_f64)
            .filter(|height| height.fract() == 0.0 && (f64::from(i16::MIN)..=f64::from(i16::MAX)).contains(height))
            .ok_or_else(|| Fault::from("gis-terrain-grading-height-invalid"))?;
        Ok(Self { id: id.into(), height: height as i16 })
    }
}

/// 🎛️ Stages the GIS feature ID and the target height.
pub fn action_arguments() -> Vec<ActionArgDef> {
    vec![
        ActionArgDef::text("id", LocalizedLabel::native("Region ID", "Regions-ID")).required(),
        ActionArgDef::with_schema("height", LocalizedLabel::native("Target Height in Meters", "Zielhöhe in Metern"), ArgSchema::number(Some(f64::from(i16::MIN)), Some(f64::from(i16::MAX)), Some(1.0), true)).default_value(&100_i16),
    ]
}

fn apply(source: &ElevationSource, region: &TerrainRegionData, height: i16) -> Result<ElevationSource, Fault> {
    source.validate().map_err(|error| Fault::from(error.to_string()))?;
    if region.rings.iter().map(Vec::len).sum::<usize>() > MAX_REGION_VERTICES {
        return Err(Fault::from("gis-terrain-grading-region-exceeds-512-vertices"));
    }
    let mut rings = region.rings.iter().map(|ring| ring.iter().map(|&[x, y]| Point::new(x, y)).collect::<Vec<_>>());
    let outer = rings.next().ok_or_else(|| Fault::from("gis-terrain-grading-region-empty"))?;
    let bounds = world_box_from_points(&outer).ok_or_else(|| Fault::from("gis-terrain-grading-region-empty"))?;
    let holes = rings.collect::<Vec<_>>();
    let [west, south, east, north] = source.bounds;
    let size = (source.rows.len() - 1) as f64;
    let mut result = source.clone();
    for (row_index, row) in result.rows.iter_mut().enumerate() {
        let latitude = south + (north - south) * (row_index as f64 / size);
        for (column, sample) in row.iter_mut().enumerate() {
            let point = Point::new(west + (east - west) * (column as f64 / size), latitude);
            if world_box_contains_point(bounds, point) && polygon_covers_point(&outer, point)
                && !holes.iter().any(|hole| polygon_covers_point(hole, point)) {
                *sample = height;
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
    let next = apply(source, region, payload.height)?;
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
        let initial_height = fixture["initialHeight"].as_i64().unwrap() as i16;
        let source = ElevationSource {
            bounds: serde_json::from_value(fixture["bounds"].clone()).unwrap(),
            rows: vec![vec![initial_height; 33]; 33],
        };
        let region = TerrainRegionData {
            id: fixture["command"]["id"].as_str().unwrap().into(),
            name: "Construction site".into(),
            rings: serde_json::from_value(fixture["rings"].clone()).unwrap(),
        };
        let result = apply(&source, &region, fixture["command"]["height"].as_i64().unwrap() as i16).unwrap();
        assert_eq!(result.rows.iter().flatten().filter(|sample| **sample != initial_height).count(), fixture["graded"].as_u64().unwrap() as usize);
        for sample in fixture["samples"].as_array().unwrap() {
            let [column, row, expected] = serde_json::from_value::<[i32; 3]>(sample.clone()).unwrap();
            assert_eq!(i32::from(result.rows[row as usize][column as usize]), expected);
        }
        assert_eq!(source.rows[4][4], initial_height);
    }

    #[semio_framework_async_macros::async_test]
    async fn grading_the_demo_region_keeps_the_map_and_is_undoable() {
        use crate::editor::gis3d::{Gis3dCommand, unit_tests::context};

        let mut app = context::app().await;
        let before = app.snapshot().unwrap();
        let command = Gis3dCommand::GradeTerrainRegion(GradeTerrainRegion { id: "g_sart_tilman".into(), height: 300 });
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
