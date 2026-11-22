//! 📉️ Derives an undoable map overlay from unexaggerated heights on the terrain mesh.

use std::collections::{BTreeMap, BTreeSet};

use crate::editor::gis3d::{Gis3dCommand, Gis3dPlayApp};
use crate::mutations::{ChangeImportedFeatures, GisTerrainMutation};
use crate::schema::imported_map::ImportedMap;
use crate::schema::snapshot::ElevationSource;
use crate::GisTerrainSnapshot;
use semio_framework_surface::terrain::TerrainGridTopology;
use semio_framework_value::{DslValue, ToValue};
use semio_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_plugin::{ActionArgDef, ArgSchema, ArtifactView, ConfigView, EditorApp, Emit, Fault, FaultCode, FaultOrigin, NoConfig, NoConfigMutation};
use semio_ui_locale::LocalizedLabel;

const MINIMUM_INTERVAL: f64 = 0.1;
const MAXIMUM_INTERVAL: f64 = 65_535.0;
const MAXIMUM_LEVELS: i64 = 64;
const MAXIMUM_SEGMENTS: usize = 4_096;
const MAXIMUM_ROUTES: usize = 512;
const ID_PREFIX: &str = "terrain-contour-";
const SOURCE_FIELD: &str = "semioTerrainContour";
const ELEVATION_FIELD: &str = "elevationMetres";
pub const ACTION: &str = "generateTerrainContours";

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "generate-terrain-contours")]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GenerateTerrainContours {
    pub interval_metres: f64,
}

impl GenerateTerrainContours {
    pub fn is_valid(&self) -> bool {
        (MINIMUM_INTERVAL..=MAXIMUM_INTERVAL).contains(&self.interval_metres)
    }

    pub fn from_action(args: &DslValue) -> Result<Self, Fault> {
        args.get("intervalMetres").and_then(DslValue::as_f64)
            .map(|interval_metres| Self { interval_metres })
            .filter(Self::is_valid)
            .ok_or_else(|| ContourFault::Interval.fault())
    }
}

/// 🎚️ Stages an interval for contour elevations above and below sea level.
pub fn action_arguments() -> Vec<ActionArgDef> {
    vec![ActionArgDef::with_schema(
        "intervalMetres",
        LocalizedLabel::native("Contour Interval (m)", "Höhenlinienabstand (m)"),
        ArgSchema::number(Some(MINIMUM_INTERVAL), Some(MAXIMUM_INTERVAL), None, false),
    ).default_value(&25.0_f64)]
}

enum ContourFault { Interval, InvalidSource, MissingSource, Density, Canceled, Command }

impl ContourFault {
    fn message(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::Interval => ("gis.terrain.contour.interval", "Enter a contour interval from 0.1 to 65,535 metres.", "Einen Höhenlinienabstand von 0,1 bis 65.535 Metern eingeben."),
            Self::InvalidSource => ("gis.terrain.contour.invalid-source", "The saved elevation grid is invalid.", "Das gespeicherte Höhenraster ist ungültig."),
            Self::MissingSource => ("gis.terrain.contour.missing-source", "Create or import an elevation grid first.", "Zuerst ein Höhenraster erstellen oder importieren."),
            Self::Density => ("gis.terrain.contour.density", "The contour interval produces too many lines. Increase the interval.", "Der Höhenlinienabstand erzeugt zu viele Linien. Den Abstand vergrößern."),
            Self::Canceled => ("gis.terrain.contour.canceled", "Contour generation was canceled.", "Die Erzeugung der Höhenlinien wurde abgebrochen."),
            Self::Command => ("gis.terrain.contour.command", "The contour tool received a different command.", "Das Höhenlinienwerkzeug hat einen anderen Befehl erhalten."),
        }
    }

    fn fault(&self) -> Fault {
        let (code, en, _) = self.message();
        Fault::new(FaultOrigin::App, FaultCode::new(code), en)
    }
}

/// 🔔️ Registers contour refusal messages in both supported locales.
pub fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
    use ContourFault::*;
    static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 6]> = std::sync::LazyLock::new(|| {
        [Interval, InvalidSource, MissingSource, Density, Canceled, Command].map(|kind| {
            let (code, en, de) = kind.message();
            (code, LocalizedLabel::native(en, de))
        })
    });
    &NOTICES[..]
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Endpoint(usize, usize);

impl Endpoint {
    fn at_vertex(vertex: usize) -> Self {
        Self(vertex, vertex)
    }

    fn on_edge(a: usize, b: usize) -> Self {
        Self(a.min(b), a.max(b))
    }

    fn fraction(self, level: f64, heights: &[f64]) -> f64 {
        if self.0 == self.1 { 0.0 } else { (level - heights[self.0]) / (heights[self.1] - heights[self.0]) }
    }
}

type Segment = [Endpoint; 2];

fn contour_segment(vertices: [usize; 3], level: f64, heights: &[f64]) -> Option<Segment> {
    let exact = vertices.map(|vertex| heights[vertex] == level);
    if exact.iter().all(|same| *same) { return None; }
    let mut endpoints = [None; 3];
    for index in 0..3 {
        let next = (index + 1) % 3;
        let (a, b) = (vertices[index], vertices[next]);
        endpoints[index] = if exact[index] {
            Some(Endpoint::at_vertex(a))
        } else if !exact[next] && (heights[a] < level) != (heights[b] < level) {
            Some(Endpoint::on_edge(a, b))
        } else {
            None
        };
    }
    let mut endpoints = endpoints.into_iter().flatten();
    let (a, b) = (endpoints.next()?, endpoints.next()?);
    Some([a.min(b), a.max(b)])
}

fn contour_paths(segments: BTreeSet<Segment>) -> Vec<Vec<Endpoint>> {
    let mut neighbors = BTreeMap::<Endpoint, Vec<Endpoint>>::new();
    for [a, b] in &segments {
        neighbors.entry(*a).or_default().push(*b);
        neighbors.entry(*b).or_default().push(*a);
    }
    let order = neighbors.iter().filter(|(_, adjacent)| adjacent.len() != 2)
        .chain(neighbors.iter().filter(|(_, adjacent)| adjacent.len() == 2));
    let mut unseen = segments;
    let mut paths = Vec::new();
    for (&start, adjacent) in order {
        for &next in adjacent {
            let mark = |unseen: &mut BTreeSet<Segment>, a: Endpoint, b: Endpoint| unseen.remove(&[a.min(b), a.max(b)]);
            if !mark(&mut unseen, start, next) { continue; }
            let mut path = vec![start, next];
            let (mut previous, mut current) = (start, next);
            while current != start && neighbors[&current].len() == 2 {
                let adjacent = &neighbors[&current];
                let next = if adjacent[0] == previous { adjacent[1] } else { adjacent[0] };
                if !mark(&mut unseen, current, next) { break; }
                path.push(next);
                (previous, current) = (current, next);
            }
            paths.push(path);
        }
    }
    paths
}

fn contours(source: &ElevationSource, interval: f64, canceled: impl Fn() -> bool) -> Result<Vec<DslValue>, Fault> {
    if !source.is_valid() { return Err(ContourFault::InvalidSource.fault()); }
    let (width, height) = source.dimensions().ok_or_else(|| ContourFault::InvalidSource.fault())?;
    let topology = TerrainGridTopology::new([width as u32, height as u32], source.missing_samples.as_deref());
    let heights = source.rows_millimetres.iter().rev().flatten().map(|&sample| f64::from(sample) / 1000.0).collect::<Vec<_>>();
    let mut surface_heights = topology.faces().flat_map(|face| face.vertices.map(|vertex| heights[vertex as usize]));
    let Some(first_height) = surface_heights.next() else { return Ok(Vec::new()); };
    let (minimum, maximum) = surface_heights.fold((first_height, first_height), |(low, high), height| (low.min(height), high.max(height)));
    let first = (minimum / interval).floor() as i64 + 1;
    let last = (maximum / interval).ceil() as i64 - 1;
    if last - first + 1 > MAXIMUM_LEVELS {
        return Err(ContourFault::Density.fault());
    }
    let [west, south, east, north] = source.bounds;
    let coordinate = |vertex: usize| {
        let column = (vertex % width) as f64 / (width - 1) as f64;
        let row = (vertex / width) as f64 / (height - 1) as f64;
        [west + column * (east - west), north + row * (south - north)]
    };
    let mut routes = Vec::new();
    let mut used_segments = 0;
    for index in first..=last {
        if canceled() { return Err(ContourFault::Canceled.fault()); }
        let level = index as f64 * interval;
        if !(minimum < level && level < maximum) { continue; }
        let mut segments = BTreeSet::new();
        for face in topology.faces() {
            if let Some(segment) = contour_segment(face.vertices.map(|vertex| vertex as usize), level, &heights) {
                segments.insert(segment);
            }
        }
        used_segments += segments.len();
        if used_segments > MAXIMUM_SEGMENTS {
            return Err(ContourFault::Density.fault());
        }
        for path in contour_paths(segments) {
            let [a, b] = [path[0], path[1]];
            let id = format!("{ID_PREFIX}{:016x}-{}-{}-{}-{}", level.to_bits(), a.0, a.1, b.0, b.1);
            let mut points = path.into_iter().map(|endpoint| {
                let [ax, ay] = coordinate(endpoint.0);
                let [bx, by] = coordinate(endpoint.1);
                let t = endpoint.fraction(level, &heights);
                [ax + t * (bx - ax), ay + t * (by - ay)]
            }).collect::<Vec<_>>();
            points.dedup();
            if points.len() < 2 { continue; }
            routes.push(DslValue::object([
                ("id".into(), id.to_value()),
                ("label".into(), format!("{level} m").to_value()),
                ("points".into(), points.to_value()),
                (SOURCE_FIELD.into(), true.to_value()),
                (ELEVATION_FIELD.into(), level.to_value()),
            ]));
            if routes.len() > MAXIMUM_ROUTES {
                return Err(ContourFault::Density.fault());
            }
        }
    }
    Ok(routes)
}

fn generated_route(route: &DslValue) -> bool {
    matches!(route.get(SOURCE_FIELD), Some(DslValue::Bool(true)))
        && route.get("id").and_then(DslValue::as_str).is_some_and(|id| id.starts_with(ID_PREFIX))
}

fn replace_contours(current: Option<&ImportedMap>, mut routes: Vec<DslValue>) -> ImportedMap {
    let mut map = current.map(|map| ImportedMap {
        positions: map.positions.clone(),
        routes: map.routes.iter().filter(|route| !generated_route(route)).cloned().collect(),
        regions: map.regions.clone(),
        properties: map.properties.clone(),
    }).unwrap_or_default();
    let mut occupied = map.positions.iter().chain(&map.regions).chain(&map.routes)
        .filter_map(|record| record.get("id").and_then(DslValue::as_str).map(str::to_owned))
        .collect::<BTreeSet<_>>();
    for route in &mut routes {
        let DslValue::Object(members) = route else { unreachable!("contour route object") };
        let id = &mut members.iter_mut().find(|(name, _)| name == "id").expect("contour ID member").1;
        let base = id.as_str().expect("contour route ID");
        if occupied.insert(base.to_owned()) { continue; }
        let mut suffix = 1;
        loop {
            let candidate = format!("{base}-{suffix}");
            if occupied.insert(candidate.clone()) {
                *id = candidate.to_value();
                break;
            }
            suffix += 1;
        }
    }
    map.routes.extend(routes);
    map
}

/// 🗾️ Replaces the generated routes in a single reversible terrain document mutation.
pub fn handle(payload: &GenerateTerrainContours, doc: &ArtifactView<'_, GisTerrainSnapshot>, _: &ConfigView<'_, NoConfig>) -> Result<Emit<GisTerrainMutation, NoConfigMutation>, Fault> {
    generate(payload, doc.snapshot, || false)
}

fn generate(payload: &GenerateTerrainContours, snapshot: &GisTerrainSnapshot, canceled: impl Fn() -> bool) -> Result<Emit<GisTerrainMutation, NoConfigMutation>, Fault> {
    if !payload.is_valid() { return Err(ContourFault::Interval.fault()); }
    let source = snapshot.elevation_source.as_ref().ok_or_else(|| ContourFault::MissingSource.fault())?;
    let routes = contours(source, payload.interval_metres, &canceled)?;
    if canceled() { return Err(ContourFault::Canceled.fault()); }
    let current = snapshot.imported_map.as_ref();
    if routes.is_empty() && current.is_none() { return Ok(Emit::default()); }
    let map = replace_contours(current, routes);
    if current.is_some_and(|prior| prior.same(&map)) { return Ok(Emit::default()); }
    Ok(Emit::mutations(vec![GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_map: Some(map) })]))
}

#[derive(Default)]
pub struct Work { started: bool }

impl ArtifactCommandWork<EditorApp<Gis3dPlayApp>> for Work {
    fn tool_id(&self) -> &'static str { ACTION }

    fn extent(&self, command: &Gis3dCommand, _: &GisTerrainSnapshot, _: &protocol::InteractionState, _: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Gis3dPlayApp>>>) -> Option<usize> {
        matches!(command, Gis3dCommand::GenerateTerrainContours(payload) if payload.is_valid()).then_some(1)
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> { Some(std::mem::size_of::<Self>()) }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Gis3dPlayApp>>, cx: &mut semio_plugin::__job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Gis3dPlayApp>>, semio_plugin::__job::OriginalTurnFailure> {
        let Gis3dCommand::GenerateTerrainContours(payload) = input.command else { return Err(ContourFault::Command.fault().into()); };
        let progress = || ArtifactCommandWorkStep::Progress {
            stage: "generate-terrain-contours",
            preview: r#"{"en":"Generating contour lines","de":"Höhenlinien werden erzeugt"}"#.as_bytes(),
        };
        if !self.started {
            self.started = true;
            return Ok(progress());
        }
        let result = generate(payload, input.snapshot, || cx.is_cancelled());
        if cx.is_cancelled() { return Ok(progress()); }
        Ok(ArtifactCommandWorkStep::Complete(result?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::gis3d::{Gis3dCommand, unit_tests::context};
    use crate::editor::gis3d::modes::view::windows::terrain::GIS3D_PLAY_BODY_COMPOSITE;

    fn fixture_source(surface: &str, bounds: [f64; 4]) -> ElevationSource {
        let rows: Vec<Vec<i32>> = (0..33).map(|row| (0..33).map(|column| match surface {
            "eastwardRamp" => column - 12,
            "isolatedHill" if (row, column) == (16, 16) => 32,
            "isolatedHill" | "flat" => 0,
            _ => panic!("unknown fixture surface {surface}"),
        }).collect()).collect();
        ElevationSource { missing_samples: None, bounds, rows_millimetres: rows.into_iter().map(|row: Vec<i32>| row.into_iter().map(|height| height * 1000).collect()).collect() }
    }

    fn points(route: &DslValue) -> Vec<[f64; 2]> {
        route.get("points").and_then(DslValue::as_array).unwrap().iter().map(|point| {
            let point = point.as_array().unwrap();
            [point[0].as_f64().unwrap(), point[1].as_f64().unwrap()]
        }).collect()
    }

    #[test]
    fn rectangular_ramp_preserves_fractional_contour_levels_and_latitude_extent() {
        let source = ElevationSource {
            missing_samples: None,
            bounds: [1.0, 1.0, 4.0, 3.0],
            rows_millimetres: vec![vec![0, 10_000, 20_000, 30_000]; 3],
        };
        let lines = contours(&source, 7.5, || false).unwrap();
        assert_eq!(lines.len(), 3);
        for (line, longitude) in lines.iter().zip([1.75, 2.5, 3.25]) {
            let coordinates = points(line);
            assert!(coordinates.iter().all(|point| (point[0] - longitude).abs() < 1e-12));
            assert!(coordinates.iter().any(|point| (point[1] - 1.0).abs() < 1e-12));
            assert!(coordinates.iter().any(|point| (point[1] - 3.0).abs() < 1e-12));
        }
    }

    fn canonical(mut path: Vec<[f64; 2]>) -> Vec<[f64; 2]> {
        let closed = path.first() == path.last();
        if !closed && path.iter().all(|point| point[0] == path[0][0]) {
            path = vec![path[0], *path.last().unwrap()];
        }
        let candidates = if closed {
            path.pop();
            let length = path.len();
            (0..length).flat_map(|start| [false, true].map(|reverse| {
                (0..=length).map(|offset| path[(start + if reverse { length - offset } else { offset }) % length]).collect::<Vec<_>>()
            })).collect::<Vec<_>>()
        } else {
            vec![path.clone(), path.into_iter().rev().collect()]
        };
        candidates.into_iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
    }

    #[test]
    fn generates_exact_triangle_contours_for_shared_scenarios() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🔣️.json")).unwrap();
        let bounds = serde_json::from_value(fixture["bounds"].clone()).unwrap();
        for scenario in fixture["scenarios"].as_array().unwrap() {
            let interval = scenario["intervalMetres"].as_f64().unwrap_or_else(|| fixture["intervalMetres"].as_f64().unwrap());
            let source = fixture_source(scenario["surface"].as_str().unwrap(), bounds);
            let routes = contours(&source, interval, || false).unwrap();
            let levels = routes.iter().map(|route| route.get(ELEVATION_FIELD).and_then(DslValue::as_f64).unwrap()).collect::<Vec<_>>();
            assert_eq!(levels, serde_json::from_value::<Vec<f64>>(scenario["levels"].clone()).unwrap(), "{}", scenario["name"]);
            let expected = serde_json::from_value::<Vec<Vec<[f64; 2]>>>(scenario["routes"].clone()).unwrap();
            assert_eq!(routes.iter().map(|route| canonical(points(route))).collect::<Vec<_>>(), expected.into_iter().map(canonical).collect::<Vec<_>>(), "{}", scenario["name"]);
        }
    }

    #[test]
    fn rejects_excessive_density_and_keeps_user_routes_with_similar_ids() {
        let source = fixture_source("eastwardRamp", [0.0, 0.0, 32.0, 32.0]);
        assert!(contours(&source, 0.1, || false).is_err());
        assert!(contours(&source, 16.0, || true).is_err());
        let generated = contours(&source, 16.0, || false).unwrap();
        let id = generated[0].get("id").unwrap().clone();
        let custom = DslValue::object([("id".into(), id.clone())]);
        let map = ImportedMap { routes: vec![custom.clone()], ..Default::default() };
        let combined = replace_contours(Some(&map), generated.clone());
        assert_eq!(combined.routes[0], custom);
        assert_ne!(combined.routes[1].get("id"), Some(&id));
        assert!(combined.same(&replace_contours(Some(&combined), generated)));
    }

    #[semio_framework_async_macros::async_test]
    async fn mounted_terrain_contours_regenerate_and_undo_as_one_overlay_mutation() {
        let mut app = context::app().await;
        let before = app.snapshot().unwrap();
        let command = |interval_metres| Gis3dCommand::GenerateTerrainContours(GenerateTerrainContours { interval_metres });
        let receipt = context::dispatch(&mut app, command(50.0)).await;
        assert!(receipt.lanes.contains(&semio_plugin::app::TypedOperationResultLane::Artifact));
        let after = app.snapshot().unwrap();
        let prior_map = before.imported_map.as_ref().unwrap();
        let map = after.imported_map.as_ref().unwrap();
        assert_eq!(map.positions, prior_map.positions);
        assert_eq!(map.regions, prior_map.regions);
        assert_eq!(map.properties, prior_map.properties);
        assert_eq!(map.routes.iter().filter(|route| !generated_route(route)).cloned().collect::<Vec<_>>(), prior_map.routes);
        assert!(map.routes.iter().any(generated_route));
        assert_eq!(after.elevation_source, before.elevation_source);
        let body = context::render(&mut app, GIS3D_PLAY_BODY_COMPOSITE).await;
        let scene = semio_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<semio_plugin::World3dScene>(&body).unwrap();
        let world: serde_json::Value = serde_json::from_str(scene.terrain_json.as_deref().unwrap()).unwrap();
        for route in map.routes.iter().filter(|route| generated_route(route)) {
            let id = route.get("id").and_then(DslValue::as_str).unwrap();
            assert!(world["overlays"]["routes"].as_array().unwrap().iter().any(|route| route["id"] == id));
        }
        let receipt = context::dispatch(&mut app, command(50.0)).await;
        assert!(!receipt.lanes.contains(&semio_plugin::app::TypedOperationResultLane::Artifact));
        assert_eq!(app.snapshot().unwrap(), after);
        context::dispatch(&mut app, command(100.0)).await;
        let regenerated = app.snapshot().unwrap();
        assert_ne!(regenerated, after);
        context::history_verb(&mut app, "undo").await;
        assert_eq!(app.snapshot().unwrap(), after);
        context::history_verb(&mut app, "undo").await;
        assert_eq!(app.snapshot().unwrap(), before);
        context::history_verb(&mut app, "redo").await;
        assert_eq!(app.snapshot().unwrap(), after);
        context::close(&mut app);
    }
}
