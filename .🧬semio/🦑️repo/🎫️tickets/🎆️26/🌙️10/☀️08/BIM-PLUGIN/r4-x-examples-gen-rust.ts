#!/usr/bin/env bun
/** 🏡️ Writes the Rust and TypeScript sources of the `house` and `office` examples and the shared example checks, and mounts them in the artifact root (idempotent). The snapshot sources come from `r4-x-examples-gen.ts`. */
import { existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { artifact, child, em, subset } from "./r3-f1-paths.ts";

const EXAMPLES = em(0x1f4da) + "examples";
const ASSETS = em(0x1f5bc) + "assets";
const TESTS = em(0x1f9ea) + "tests";
const EXAMPLE_TESTS = em(0x1f9e9) + "example";
const CHECKS = em(0x1f9f0) + "checks";
const RS = em(0x1f980) + ".rs";
const TS = em(0x1f7e6) + ".ts";
const DSL = em(0x1f5e3) + ".dsl.semio";
const SNAPSHOT = em(0x1f4f8) + "snapshot.json";
const STANDARD = `${em(0x1f3c5)}standards/${em(0x1f516)}1/${em(0x1fa86)}subsets/${String.fromCodePoint(0x2733)}️any`;
const SCHEMA = em(0x1f9ec) + "schema";
const JSON_FILE = em(0x1f523) + ".json";

const write = (path: string, text: string) => {
  mkdirSync(join(path, ".."), { recursive: true });
  writeFileSync(path, text);
};

const examples = [
  { id: "house", dir: em(0x1f3e1) + "house", mod: "house", en: "Family House", de: "Einfamilienhaus", icon: "house", emoji: em(0x1f3e1) },
  { id: "office", dir: em(0x1f3e2) + "office", mod: "office", en: "Office Building", de: "Bürogebäude", icon: "building-2", emoji: em(0x1f3e2) },
];

const component = (e: (typeof examples)[number], doc: string) => `//! ${e.emoji} Example \`${e.id}\`: ${doc}

use semio_framework_plugin::ExampleSource;
use semio_framework_ui_locale::LocalizedLabel;

pub const ID: &str = "${e.id}";
pub const ASSET_DIR: &str = "${e.dir}";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("${e.en}", "${e.de}")
}
pub const ICON: &str = "${e.icon}";
pub const PRIMARY_TEXT: &str = include_str!("../../${ASSETS}/${e.dir}/${DSL}");
pub const SNAPSHOT_JSON: &str = include_str!("../../${ASSETS}/${e.dir}/${SNAPSHOT}");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
`;

const manifest = (e: (typeof examples)[number]) => `/** ${e.emoji} Example \`${e.id}\`. */
export const id = "${e.id}";
export const label = { en: "${e.en}", de: "${e.de}" } as const;
export const icon = "${e.icon}";
`;

const houseTests = `use super::{ASSET_DIR, PRIMARY_TEXT, SNAPSHOT_JSON};
use crate::examples::checks::Asset;
use crate::{Axis, ModelSnapshot, OpeningKind, SpaceBoundary};

const ASSET: Asset = Asset { dir: ASSET_DIR, text: PRIMARY_TEXT, json: SNAPSHOT_JSON };

fn walls_on(model: &ModelSnapshot, storey: &str) -> usize {
    model.walls.values().filter(|wall| wall.storey == storey).count()
}

fn near(left: f64, right: f64) -> bool {
    (left - right).abs() < 1e-9
}

#[semio_framework_async_macros::async_test]
async fn the_house_is_the_documented_building() {
    let model = ASSET.model();
    assert_eq!((model.sites.len(), model.buildings.len(), model.storeys.len(), model.grids.len()), (1, 1, 4, 6));
    let mut levels: Vec<i32> = model.storeys.values().map(|storey| storey.level).collect();
    levels.sort();
    assert_eq!(levels, vec![-1, 0, 1, 2]);
    assert_eq!((walls_on(&model, "st-basement"), walls_on(&model, "st-ground"), walls_on(&model, "st-upper"), walls_on(&model, "st-attic")), (7, 9, 10, 4));
    assert_eq!(model.walls.values().filter(|wall| matches!(wall.axis, Axis::Arc { .. })).count(), 3, "one bay wall on the basement, ground and upper storey");
    let windows = model.openings.values().filter(|opening| matches!(opening.kind, OpeningKind::Window { .. })).count();
    let doors = model.openings.values().filter(|opening| matches!(opening.kind, OpeningKind::Door { .. })).count();
    let voids = model.openings.values().filter(|opening| matches!(opening.kind, OpeningKind::Void { .. })).count();
    assert_eq!((windows, doors, voids), (25, 9, 1));
    assert_eq!((model.slabs.len(), model.roofs.len(), model.stairs.len(), model.railings.len(), model.columns.len(), model.beams.len()), (4, 2, 2, 2, 1, 1));
    assert!(model.slabs.values().any(|slab| !slab.holes.is_empty()), "a slab carries a stair hole");
    assert_eq!(model.spaces.len(), 12);
    assert!(model.spaces.values().all(|space| matches!(space.boundary, SpaceBoundary::Bounded { .. })));
    assert_eq!((model.materials.len(), model.wall_types.len(), model.slab_types.len(), model.roof_types.len()), (10, 6, 3, 2));
    assert_eq!((model.properties.len(), model.classifications.len()), (6, 6));
    let site = model.sites.values().next().expect("a site");
    assert!((46.0..48.0).contains(&site.latitude) && (7.0..8.0).contains(&site.longitude) && site.boundary.len() == 4);
}

#[semio_framework_async_macros::async_test]
async fn the_house_round_trips_and_validates() {
    ASSET.assert_text_is_the_codec_fixed_point();
    ASSET.assert_pack_round_trip();
    ASSET.assert_schema_valid();
    let model = ASSET.model();
    assert_eq!(crate::examples::checks::dangling(&model), Vec::<String>::new());
}

#[semio_framework_async_macros::async_test]
async fn the_house_infers_levels_heights_and_valid_openings() {
    let model = ASSET.model();
    let inferred = crate::examples::checks::infer(&model);
    let level = |id: &str| inferred.storey_levels[id];
    assert!(near(level("st-basement").elevation, -2.6) && near(level("st-basement").top_elevation, 0.0));
    assert!(near(level("st-ground").elevation, 0.0) && near(level("st-ground").top_elevation, 2.8));
    assert!(near(level("st-upper").elevation, 2.8) && near(level("st-upper").top_elevation, 5.5));
    assert!(near(level("st-attic").elevation, 5.5) && near(level("st-attic").top_elevation, 7.9));
    assert!(near(level("st-ground").absolute_elevation, 542.3), "site 542 m plus the 0.3 m plinth");
    let heights = |storey: &str| -> Vec<f64> { model.walls.iter().filter(|(_, wall)| wall.storey == storey).map(|(id, _)| inferred.wall_layout[id].height).collect() };
    assert!(heights("st-basement").iter().all(|height| near(*height, 2.6)));
    assert!(heights("st-ground").iter().all(|height| near(*height, 2.8)));
    assert!(heights("st-upper").iter().all(|height| near(*height, 2.7)));
    assert!(heights("st-attic").iter().all(|height| near(*height, 0.9)), "knee walls stay 0.9 m high");
    let south = &inferred.wall_layout["w-g-south"];
    assert!(near(south.thickness, 0.37) && near(south.length, 10.0) && near(south.base_z, 0.0) && near(south.top_z, 2.8));
    let bay = &inferred.wall_layout["w-g-bay"];
    assert!((bay.length - 3.477356).abs() < 1e-5, "the bay is a 106 degree arc of radius 1.875 m, got {}", bay.length);
    assert!(near(inferred.wall_layout["w-g-spine"].thickness, 0.205) && near(inferred.wall_layout["w-g-cross"].thickness, 0.205) && near(inferred.wall_layout["w-g-wc"].thickness, 0.145));
    assert_eq!(inferred.stair_runs["sr-main"].riser_count, 15);
    assert_eq!(inferred.stair_runs["sr-cellar"].riser_count, 13);
}

#[semio_framework_async_macros::async_test]
async fn a_storey_height_edit_moves_every_storey_above_it() {
    use protocol::Inference;
    let mut model = ASSET.model();
    model.storeys.get_mut("st-ground").expect("the ground storey").height = 3.0;
    let inferred = crate::ModelInference::infer(&model).expect("infers");
    assert!(near(inferred.storey_levels["st-upper"].elevation, 3.0) && near(inferred.storey_levels["st-attic"].elevation, 5.7));
    assert!(near(inferred.wall_layout["w-g-south"].height, 3.0) && near(inferred.wall_layout["w-u-south"].base_z, 3.0) && near(inferred.wall_layout["w-a-south"].base_z, 5.7));
    assert!(near(inferred.wall_layout["w-b-south"].top_z, 0.0), "the basement below does not move");
}

#[semio_framework_async_macros::async_test]
async fn the_house_quantities_and_rooms_match_the_hand_calculation() {
    let model = ASSET.model();
    let inferred = crate::examples::checks::infer(&model);
    let quantity = |id: &str| &inferred.quantities.elements[id];
    let bay = 1.875f64 * 1.875 / 2.0 * (4.0 * 0.5f64.atan() - 0.96);
    let ground = quantity("sl-g");
    assert!((ground.gross_area - (80.0 + bay)).abs() < 1e-6, "an 10 x 8 m slab plus a circular segment of radius 1.875 m, got {}", ground.gross_area);
    assert!((ground.net_area - (80.0 + bay - 0.9 * 2.69)).abs() < 1e-6, "minus the cellar stair hole");
    assert!((ground.net_volume - ground.net_area * 0.3).abs() < 1e-9, "a 30 cm floor slab");
    let south = quantity("w-g-south");
    assert!(near(south.gross_side_area, 28.0) && (south.opening_area - (1.0 * 2.15 + 1.8 * 1.4 + 1.6 * 2.25)).abs() < 1e-9, "door, picture window and patio door cut the south wall");
    assert!((south.net_side_area - (28.0 - 8.27)).abs() < 1e-9);
    let totals = &inferred.quantities.project.kinds;
    assert_eq!((totals["wall"].count, totals["window"].count, totals["door"].count, totals["void"].count, totals["slab"].count, totals["stair"].count, totals["space"].count), (30, 25, 9, 1, 4, 2, 12));
    let wc = &inferred.spaces["sp-g2"];
    assert!((wc.area - (1.7275 - 0.185) * (7.815 - 5.1025)).abs() < 1e-6, "the WC is bounded by the faces of four walls, got {}", wc.area);
    assert!(wc.clear_height > 2.0, "a room is taller than two metres, got {}", wc.clear_height);
}

#[semio_framework_async_macros::async_test]
async fn the_house_is_reachable_through_the_mutation_api() {
    crate::examples::checks::assert_replay("house", &ASSET.model());
}

#[semio_framework_async_macros::async_test]
async fn bless_the_house_text() {
    ASSET.bless();
}
`;

const officeTests = `use super::{ASSET_DIR, PRIMARY_TEXT, SNAPSHOT_JSON};
use crate::examples::checks::Asset;
use crate::{ModelSnapshot, OpeningKind};

const ASSET: Asset = Asset { dir: ASSET_DIR, text: PRIMARY_TEXT, json: SNAPSHOT_JSON };

fn walls_on(model: &ModelSnapshot, storey: &str) -> usize {
    model.walls.values().filter(|wall| wall.storey == storey).count()
}

fn near(left: f64, right: f64) -> bool {
    (left - right).abs() < 1e-9
}

#[semio_framework_async_macros::async_test]
async fn the_office_is_the_documented_building() {
    let model = ASSET.model();
    assert_eq!((model.sites.len(), model.buildings.len(), model.storeys.len(), model.grids.len()), (1, 1, 5, 10));
    assert_eq!((walls_on(&model, "st-0"), walls_on(&model, "st-1"), walls_on(&model, "st-2"), walls_on(&model, "st-3"), walls_on(&model, "st-r")), (14, 14, 14, 14, 4));
    assert_eq!((model.curtain_walls.len(), model.columns.len(), model.beams.len()), (8, 96, 152));
    assert_eq!((model.slabs.len(), model.roofs.len(), model.stairs.len(), model.spaces.len()), (4, 1, 6, 28));
    let windows = model.openings.values().filter(|opening| matches!(opening.kind, OpeningKind::Window { .. })).count();
    let doors = model.openings.values().filter(|opening| matches!(opening.kind, OpeningKind::Door { .. })).count();
    assert_eq!((windows, doors), (24, 17));
    assert_eq!(model.slabs.values().map(|slab| slab.holes.len()).sum::<usize>(), 6, "two stair shafts through each of the three upper slabs");
    let on_grid = |value: f64, axis: bool| model.grids.values().any(|grid| if axis { near(grid.start.x, value) && near(grid.end.x, value) } else { near(grid.start.y, value) && near(grid.end.y, value) });
    assert!(model.columns.values().all(|column| on_grid(column.position.x, true) && on_grid(column.position.y, false)), "every column stands on a grid intersection");
    assert!(model.beams.values().all(|beam| (on_grid(beam.start.x, true) && on_grid(beam.end.x, true)) || (on_grid(beam.start.y, false) && on_grid(beam.end.y, false))), "every beam runs along a grid line");
}

#[semio_framework_async_macros::async_test]
async fn the_office_round_trips_and_validates() {
    ASSET.assert_text_is_the_codec_fixed_point();
    ASSET.assert_pack_round_trip();
    ASSET.assert_schema_valid();
    let model = ASSET.model();
    assert_eq!(crate::examples::checks::dangling(&model), Vec::<String>::new());
}

#[semio_framework_async_macros::async_test]
async fn the_office_infers_levels_heights_and_valid_openings() {
    let model = ASSET.model();
    let inferred = crate::examples::checks::infer(&model);
    for (id, elevation, top) in [("st-0", 0.0, 4.0), ("st-1", 4.0, 7.8), ("st-2", 7.8, 11.6), ("st-3", 11.6, 15.4), ("st-r", 15.4, 16.6)] {
        let level = inferred.storey_levels[id];
        assert!(near(level.elevation, elevation) && near(level.top_elevation, top), "{id}");
        assert!(near(level.absolute_elevation, 408.0 + elevation), "{id} sits on the 408 m site");
    }
    assert!(near(inferred.wall_layout["w-0-core-w-south"].height, 4.0) && near(inferred.wall_layout["w-0-west-1"].height, 3.14), "cores run the full storey, infill panels stop under the edge beam");
    assert!(model.walls.iter().filter(|(_, wall)| wall.storey == "st-r").all(|(id, _)| near(inferred.wall_layout[id].height, 1.0)), "parapets are 1 m");
    assert_eq!(inferred.curtain_layout.len(), 8);
    assert!(near(inferred.wall_layout["w-0-core-w-south"].thickness, 0.25) && near(inferred.wall_layout["w-0-west-1"].thickness, 0.34));
    assert_eq!(inferred.stair_runs["sr-0-w"].riser_count, 20);
    assert_eq!(inferred.stair_runs["sr-1-e"].riser_count, 19);
}

#[semio_framework_async_macros::async_test]
async fn a_storey_height_edit_moves_every_storey_above_it() {
    use protocol::Inference;
    let mut model = ASSET.model();
    model.storeys.get_mut("st-1").expect("the first storey").height = 4.2;
    let inferred = crate::ModelInference::infer(&model).expect("infers");
    assert!(near(inferred.storey_levels["st-2"].elevation, 8.2) && near(inferred.storey_levels["st-r"].elevation, 15.8));
    assert!(near(inferred.wall_layout["w-1-west-1"].height, 3.34) && near(inferred.wall_layout["w-3-west-1"].base_z, 12.0) && near(inferred.wall_layout["w-0-west-1"].height, 3.14) && near(inferred.wall_layout["w-1-core-w-south"].height, 4.2));
}

#[semio_framework_async_macros::async_test]
async fn the_office_quantities_and_rooms_match_the_hand_calculation() {
    let model = ASSET.model();
    let inferred = crate::examples::checks::infer(&model);
    let quantity = |id: &str| &inferred.quantities.elements[id];
    let floor = quantity("sl-1");
    assert!((floor.gross_area - 30.6 * 18.6).abs() < 1e-9 && (floor.net_area - (30.6 * 18.6 - 2.0 * 2.7 * 4.9)).abs() < 1e-9, "a 30.6 x 18.6 m slab with two 2.7 x 4.9 m shafts");
    assert!((floor.net_volume - floor.net_area * 0.36).abs() < 1e-9, "a 36 cm floor slab");
    assert!((quantity("c-0-A1").gross_volume - 0.5 * 0.5 * 4.0).abs() < 1e-9 && (quantity("c-3-F4").gross_volume - 0.4 * 0.4 * 3.8).abs() < 1e-9);
    assert!((quantity("bm-0-x-A1").gross_volume - 6.0 * 0.4 * 0.6).abs() < 1e-9 && (quantity("bm-2-y-C2").gross_volume - 6.0 * 0.3 * 0.5).abs() < 1e-9);
    let totals = &inferred.quantities.project.kinds;
    assert_eq!((totals["wall"].count, totals["curtain-wall"].count, totals["column"].count, totals["beam"].count, totals["window"].count, totals["door"].count, totals["stair"].count, totals["space"].count), (60, 8, 96, 152, 24, 17, 6, 28));
    assert!((inferred.spaces["sp-0-1"].area - 180.0).abs() < 1e-9, "the lobby is the 30 x 6 m south band");
    assert!((inferred.spaces["sp-2-6"].area - 2.75 * 4.95).abs() < 1e-6, "a stair core is bounded by the inner faces of its four 25 cm walls, got {}", inferred.spaces["sp-2-6"].area);
}

#[semio_framework_async_macros::async_test]
async fn the_office_is_reachable_through_the_mutation_api() {
    crate::examples::checks::assert_replay("office", &ASSET.model());
}

#[semio_framework_async_macros::async_test]
async fn bless_the_office_text() {
    ASSET.bless();
}
`;

const checks = `//! 🧰️ Shared checks of the bundled examples: strict decode, schema validation by the third-party jsonschema library, text and pack
//! round trips, referential integrity, inference sanity, and the replay of a whole example through the mutation API.

use crate::mutations::{apply_model_mutation, ModelMutation, KINDS};
use crate::standards::v1::subsets::any::io::binary::snapshot as pack;
use crate::standards::v1::subsets::any::io::text::snapshot::{decode_model_snapshot_json, parse_dsl, print_dsl};
use crate::{ModelInference, ModelSnapshot, OpeningKind, TopConstraint};
use crate::standards::v1::subsets::any::schema::inferences::{diagnostics::Severity, spaces::SpaceStatus};
use protocol::Inference;
use std::path::{Path, PathBuf};

const VALIDATE: &str = concat!(
    "import json,sys,jsonschema\\n",
    "from referencing import Registry,Resource\\n",
    "load=lambda p:json.load(open(p,encoding='utf-8'))\\n",
    "snap,art,inst=load(sys.argv[1]),load(sys.argv[2]),load(sys.argv[3])\\n",
    "reg=Registry().with_resources([(art['$id'],Resource.from_contents(art)),(snap['$id'],Resource.from_contents(snap))])\\n",
    "errs=sorted(jsonschema.Draft7Validator(snap,registry=reg).iter_errors(inst),key=lambda e:[str(x) for x in e.path])\\n",
    "for e in errs[:10]:print('/'.join(str(x) for x in e.path),e.message[:160])\\n",
    "sys.exit(1 if errs else 0)\\n",
);

fn subset_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../${STANDARD}").join(relative)
}

fn venv_python() -> Option<PathBuf> {
    let mut dir = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    loop {
        for relative in [".venv/Scripts/python.exe", ".venv/bin/python"] {
            let candidate = dir.join(relative);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
        if !dir.pop() {
            return None;
        }
    }
}

/// 📦️ One bundled example: its asset directory, committed DSL text and committed JSON snapshot source.
#[derive(Clone, Copy)]
pub struct Asset {
    pub dir: &'static str,
    pub text: &'static str,
    pub json: &'static str,
}

impl Asset {
    pub fn model(&self) -> ModelSnapshot {
        decode_model_snapshot_json(self.json).expect("the committed snapshot decodes strictly")
    }

    fn file(&self, name: &str) -> PathBuf {
        subset_path("${ASSETS}").join(self.dir).join(name)
    }

    pub fn assert_text_is_the_codec_fixed_point(&self) {
        let model = self.model();
        assert_eq!(parse_dsl(self.text).expect("the committed text parses"), model, "text and json describe the same model");
        assert!(print_dsl(&model) == self.text, "the committed text is this codec's own output");
    }

    pub fn assert_pack_round_trip(&self) {
        let model = self.model();
        assert!(pack::decode(&pack::encode(&model)).expect("the pack decodes") == model, "pack round trip");
    }

    pub fn assert_schema_valid(&self) {
        let Some(python) = venv_python() else {
            eprintln!("schema validation skipped: no repository .venv with the jsonschema library");
            return;
        };
        let output = std::process::Command::new(python)
            .args(["-I", "-c", VALIDATE])
            .arg(subset_path("${SCHEMA}/${em(0x1f4f8)}snapshot/${JSON_FILE}"))
            .arg(subset_path("${SCHEMA}/${JSON_FILE}"))
            .arg(self.file("${SNAPSHOT}"))
            .output()
            .expect("python runs");
        assert!(output.status.success(), "the snapshot violates its JSON Schema:\\n{}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
    }

    pub fn bless(&self) {
        if std::env::var_os("BIM_BLESS").is_none() {
            return;
        }
        std::fs::write(self.file("${DSL}"), print_dsl(&self.model())).expect("the text is written");
    }
}

fn top_storey(top: &TopConstraint) -> Option<&str> {
    match top {
        TopConstraint::Storey { storey, .. } => Some(storey),
        _ => None,
    }
}

/// 🔗️ Every reference of the snapshot that points at nothing, as readable paths; empty when the model is closed.
pub fn dangling(model: &ModelSnapshot) -> Vec<String> {
    let mut found = Vec::new();
    let mut need = |what: String, present: bool| {
        if !present {
            found.push(what);
        }
    };
    macro_rules! on_storey {
        ($($collection:ident),*) => {
            $(for (id, row) in &model.$collection {
                need(format!("{}/{id}/storey", stringify!($collection)), model.storeys.contains_key(&row.storey));
            })*
        };
    }
    on_storey!(walls, curtain_walls, columns, beams, slabs, roofs, stairs, railings, spaces);
    for (id, building) in &model.buildings {
        need(format!("buildings/{id}/site"), model.sites.contains_key(&building.site));
    }
    for (id, space) in &model.spaces {
        need(format!("spaces/{id}/zone"), space.zone.as_ref().is_none_or(|zone| model.zones.contains_key(zone)));
        for (field, finish) in [("floor_finish", &space.floor_finish), ("wall_finish", &space.wall_finish), ("ceiling_finish", &space.ceiling_finish)] {
            need(format!("spaces/{id}/{field}"), finish.as_ref().is_none_or(|material| model.materials.contains_key(material)));
        }
    }
    for (id, scheme) in &model.area_schemes {
        for zone in &scheme.zones {
            need(format!("area_schemes/{id}/zones"), model.zones.contains_key(zone));
        }
    }
    for (id, storey) in &model.storeys {
        need(format!("storeys/{id}/building"), model.buildings.contains_key(&storey.building));
    }
    for (id, grid) in &model.grids {
        need(format!("grids/{id}/building"), model.buildings.contains_key(&grid.building));
    }
    for (id, wall) in &model.walls {
        need(format!("walls/{id}/wall_type"), model.wall_types.contains_key(&wall.wall_type));
        need(format!("walls/{id}/top"), top_storey(&wall.top).is_none_or(|storey| model.storeys.contains_key(storey)));
    }
    for (id, curtain) in &model.curtain_walls {
        need(format!("curtain_walls/{id}/top"), top_storey(&curtain.top).is_none_or(|storey| model.storeys.contains_key(storey)));
        need(format!("curtain_walls/{id}/panel_material"), model.materials.contains_key(&curtain.panel_material));
        need(format!("curtain_walls/{id}/mullion_material"), model.materials.contains_key(&curtain.mullion_material));
    }
    for (id, column) in &model.columns {
        need(format!("columns/{id}/column_type"), model.column_types.contains_key(&column.column_type));
        need(format!("columns/{id}/top"), top_storey(&column.top).is_none_or(|storey| model.storeys.contains_key(storey)));
    }
    for (id, beam) in &model.beams {
        need(format!("beams/{id}/beam_type"), model.beam_types.contains_key(&beam.beam_type));
    }
    for (id, slab) in &model.slabs {
        need(format!("slabs/{id}/slab_type"), model.slab_types.contains_key(&slab.slab_type));
    }
    for (id, roof) in &model.roofs {
        need(format!("roofs/{id}/roof_type"), model.roof_types.contains_key(&roof.roof_type));
    }
    for (id, stair) in &model.stairs {
        need(format!("stairs/{id}/top"), top_storey(&stair.top).is_none_or(|storey| model.storeys.contains_key(storey)));
    }
    for (id, railing) in &model.railings {
        need(format!("railings/{id}/material"), model.materials.contains_key(&railing.material));
    }
    for (id, opening) in &model.openings {
        need(format!("openings/{id}/host"), model.walls.contains_key(&opening.host) || model.curtain_walls.contains_key(&opening.host));
        match &opening.kind {
            OpeningKind::Window { window_type } => need(format!("openings/{id}/window_type"), model.window_types.contains_key(window_type)),
            OpeningKind::Door { door_type } => need(format!("openings/{id}/door_type"), model.door_types.contains_key(door_type)),
            OpeningKind::Void { .. } => {}
        }
    }
    macro_rules! layered {
        ($($collection:ident),*) => {
            $(for (id, kind) in &model.$collection {
                for layer in &kind.layers {
                    need(format!("{}/{id}/layer", stringify!($collection)), model.materials.contains_key(&layer.material));
                }
            })*
        };
    }
    layered!(wall_types, slab_types, roof_types);
    macro_rules! of_material {
        ($($collection:ident),*) => {
            $(for (id, kind) in &model.$collection {
                need(format!("{}/{id}/material", stringify!($collection)), model.materials.contains_key(&kind.material));
            })*
        };
    }
    of_material!(column_types, beam_types, window_types, door_types);
    let owned = |id: &String| {
        model.walls.contains_key(id) || model.curtain_walls.contains_key(id) || model.columns.contains_key(id) || model.beams.contains_key(id) || model.slabs.contains_key(id) || model.roofs.contains_key(id) || model.openings.contains_key(id) || model.stairs.contains_key(id) || model.railings.contains_key(id) || model.spaces.contains_key(id) || model.zones.contains_key(id) || model.area_schemes.contains_key(id)
    };
    for id in model.properties.keys() {
        need(format!("properties/{id}"), owned(id));
    }
    for id in model.classifications.keys() {
        need(format!("classifications/{id}"), owned(id));
    }
    let present = |id: &str| owned(&id.to_string()) || model.grids.contains_key(id);
    for (id, row) in &model.dimensions {
        need(format!("dimensions/{id}/storey"), model.storeys.contains_key(&row.storey));
        need(format!("dimensions/{id}/style"), model.annotation_styles.contains_key(&row.style));
        need(format!("dimensions/{id}/anchors"), row.elements().all(present));
    }
    for (id, row) in &model.tags {
        need(format!("tags/{id}/storey"), model.storeys.contains_key(&row.storey));
        need(format!("tags/{id}/style"), model.annotation_styles.contains_key(&row.style));
        need(format!("tags/{id}/element"), present(&row.element));
    }
    for (id, row) in &model.text_notes {
        need(format!("text_notes/{id}/storey"), model.storeys.contains_key(&row.storey));
        need(format!("text_notes/{id}/style"), model.annotation_styles.contains_key(&row.style));
    }
    for (id, row) in &model.leaders {
        need(format!("leaders/{id}/storey"), model.storeys.contains_key(&row.storey));
        need(format!("leaders/{id}/style"), model.annotation_styles.contains_key(&row.style));
        need(format!("leaders/{id}/anchor"), row.element().is_none_or(present));
    }
    for (id, schedule) in &model.schedules {
        for storey in &schedule.storeys {
            need(format!("schedules/{id}/storeys"), model.storeys.contains_key(storey));
        }
        need(format!("schedules/{id}/definition"), crate::schedule_kit::schedule_problem(schedule).is_none());
    }
    let mut levels = std::collections::BTreeSet::new();
    for (id, storey) in &model.storeys {
        need(format!("storeys/{id}/level is unique in its building"), levels.insert((storey.building.clone(), storey.level)));
    }
    let mut numbers = std::collections::BTreeSet::new();
    for (id, space) in &model.spaces {
        need(format!("spaces/{id}/number is unique on its storey"), numbers.insert((space.storey.clone(), space.number.clone())));
    }
    found
}

/// 💡️ Infers the whole model and asserts the inference is deterministic, complete and clean: one level per storey, one layout per wall
/// and curtain wall, one valid frame per opening, one compliant run per stair.
pub fn infer(model: &ModelSnapshot) -> ModelInference {
    let inferred = ModelInference::infer(model).expect("the model infers");
    assert!(inferred == ModelInference::infer(model).expect("the model infers again"), "inference is deterministic");
    assert_eq!(inferred.storey_levels.len(), model.storeys.len());
    assert_eq!(inferred.wall_layout.len(), model.walls.len());
    assert_eq!(inferred.curtain_layout.len(), model.curtain_walls.len());
    assert_eq!(inferred.opening_frames.len(), model.openings.len());
    assert_eq!(inferred.stair_runs.len(), model.stairs.len());
    for (id, layout) in &inferred.wall_layout {
        assert!(layout.height > 0.0 && layout.thickness > 0.0 && layout.length > 0.0, "wall {id} resolves to a solid extent");
    }
    for (id, frame) in &inferred.opening_frames {
        assert!(frame.valid, "opening {id} is valid: issues {:?}, overlaps {:?}", frame.issues, frame.overlaps);
    }
    for (id, run) in &inferred.stair_runs {
        assert!(run.compliance.compliant, "stair {id} is compliant: {:?}", run.compliance);
    }
    assert_eq!(inferred.spaces.len(), model.spaces.len());
    for (id, room) in &inferred.spaces {
        assert!(matches!(room.status, SpaceStatus::Inferred | SpaceStatus::Explicit) && room.area > 0.0, "space {id} is a closed room: {:?}", room.status);
    }
    let errors: Vec<_> = inferred.diagnostics.iter().filter(|diagnostic| diagnostic.severity == Severity::Error).collect();
    assert!(errors.is_empty(), "the model has no diagnostic errors: {errors:?}");
    for diagnostic in &inferred.diagnostics {
        eprintln!("diagnostic {:?} {:?} {:?}", diagnostic.severity, diagnostic.code, diagnostic.elements);
    }
    inferred
}

macro_rules! families {
    ($(($kind:literal, $variant:literal, $field:literal, $collection:ident)),* $(,)?) => {
        fn rows(model: &ModelSnapshot) -> Vec<(&'static str, &'static str, &'static str, Vec<(String, String)>)> {
            vec![$(($kind, $variant, $field, model.$collection.iter().map(|(id, row)| (semio_framework_pack_json::to_json_string(id), semio_framework_pack_json::to_json_string(row))).collect())),*]
        }

        fn seed(base: &mut ModelSnapshot, model: &ModelSnapshot, kind: &str) {
            $(if kind == $kind {
                base.$collection = model.$collection.clone();
            })*
        }
    };
}

families!(
    ("create-material", "createMaterial", "material", materials),
    ("create-wall-type", "createWallType", "wall_type", wall_types),
    ("create-slab-type", "createSlabType", "slab_type", slab_types),
    ("create-roof-type", "createRoofType", "roof_type", roof_types),
    ("create-column-type", "createColumnType", "column_type", column_types),
    ("create-beam-type", "createBeamType", "beam_type", beam_types),
    ("create-window-type", "createWindowType", "window_type", window_types),
    ("create-door-type", "createDoorType", "door_type", door_types),
    ("create-site", "createSite", "site", sites),
    ("create-building", "createBuilding", "building", buildings),
    ("create-storey", "createStorey", "storey", storeys),
    ("create-grid-line", "createGridLine", "grid_line", grids),
    ("create-wall", "createWall", "wall", walls),
    ("create-curtain-wall", "createCurtainWall", "curtain_wall", curtain_walls),
    ("create-column", "createColumn", "column", columns),
    ("create-beam", "createBeam", "beam", beams),
    ("create-slab", "createSlab", "slab", slabs),
    ("create-roof", "createRoof", "roof", roofs),
    ("create-opening", "createOpening", "opening", openings),
    ("create-stair", "createStair", "stair", stairs),
    ("create-railing", "createRailing", "railing", railings),
    ("create-zone", "createZone", "zone", zones),
    ("create-area-scheme", "createAreaScheme", "area_scheme", area_schemes),
    ("create-space", "createSpace", "space", spaces),
    ("create-schedule", "createSchedule", "schedule", schedules),
    ("create-annotation-style", "createAnnotationStyle", "annotation_style", annotation_styles),
    ("create-dimension", "createDimension", "dimension", dimensions),
    ("create-tag", "createTag", "tag", tags),
    ("create-text-note", "createTextNote", "text_note", text_notes),
    ("create-leader", "createLeader", "leader", leaders),
);

fn apply_json(state: &ModelSnapshot, json: &str) -> ModelSnapshot {
    let mutation: ModelMutation = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{json} decodes: {error}"));
    apply_model_mutation(state, &mutation).unwrap_or_else(|error| panic!("{json} applies: {error:?}"))
}

/// 🔁️ The result of replaying a snapshot as creates: the mutations applied and the create kinds that do not exist yet.
pub struct Replay {
    pub applied: usize,
    pub seeded: Vec<&'static str>,
    pub result: ModelSnapshot,
}

/// 🔁️ Builds \`model\` from an empty snapshot with one \`create-*\` mutation per record, in dependency order, through the central applier.
/// Collections whose create kind is not part of \`KINDS\` yet are seeded into the starting snapshot and reported in \`seeded\`.
pub fn replay(model: &ModelSnapshot) -> Replay {
    let mut state = ModelSnapshot { project: model.project.clone(), ..ModelSnapshot::default() };
    let mut seeded = Vec::new();
    let table = rows(model);
    for (kind, _, _, records) in &table {
        if !KINDS.contains(kind) && !records.is_empty() {
            seed(&mut state, model, kind);
            seeded.push(*kind);
        }
    }
    let (set_property, set_classification) = (KINDS.contains(&"set-element-property"), KINDS.contains(&"set-element-classification"));
    if !set_property {
        state.properties = model.properties.clone();
        seeded.push("set-element-property");
    }
    if !set_classification {
        state.classifications = model.classifications.clone();
        seeded.push("set-element-classification");
    }
    let mut applied = 0;
    for (kind, variant, field, records) in table {
        if !KINDS.contains(&kind) {
            continue;
        }
        for (id, record) in records {
            state = apply_json(&state, &format!("{{\\"mutation\\":\\"{variant}\\",\\"id\\":{id},\\"{field}\\":{record}}}"));
            applied += 1;
        }
    }
    let quoted = |text: &String| semio_framework_pack_json::to_json_string(text);
    if set_property {
        for (id, sets) in &model.properties {
            for (pset, properties) in sets {
                for (property, value) in properties {
                    state = apply_json(&state, &format!("{{\\"mutation\\":\\"setElementProperty\\",\\"id\\":{},\\"pset\\":{},\\"property\\":{},\\"value\\":{}}}", quoted(id), quoted(pset), quoted(property), semio_framework_pack_json::to_json_string(value)));
                    applied += 1;
                }
            }
        }
    }
    if set_classification {
        for (id, classification) in &model.classifications {
            state = apply_json(&state, &format!("{{\\"mutation\\":\\"setElementClassification\\",\\"id\\":{},\\"classification\\":{}}}", quoted(id), semio_framework_pack_json::to_json_string(classification)));
            applied += 1;
        }
    }
    Replay { applied, seeded, result: state }
}

/// 🔁️ Asserts the replayed snapshot equals \`model\` and reports the kinds that were seeded instead of replayed.
pub fn assert_replay(name: &str, model: &ModelSnapshot) {
    let replayed = replay(model);
    eprintln!("replay of {name}: {} mutations applied, create kinds still missing: {:?}", replayed.applied, replayed.seeded);
    assert!(replayed.applied >= model.sites.len() + model.buildings.len() + model.storeys.len() + model.walls.len(), "the foundation kinds are replayed");
    assert!(replayed.result == *model, "replaying the creates reproduces the committed example");
}
`;

const examplesDir = join(subset, EXAMPLES);
const rustFor = (e: (typeof examples)[number], doc: string, tests: string) => {
  const dir = join(examplesDir, e.dir);
  write(join(dir, RS), component(e, doc));
  write(join(dir, TS), manifest(e));
  write(join(dir, TESTS, EXAMPLE_TESTS, RS), tests);
};
rustFor(examples[0], "a detached house with basement, ground, upper and attic storeys, a bay wall, a gable roof and a U-stair.", houseTests);
rustFor(examples[1], "a four-storey office on a six metre grid with columns, beams, cores, stairs and a curtain wall.", officeTests);
write(join(examplesDir, CHECKS, RS), checks);

const root = join(artifact, RS);
const mount = (e: (typeof examples)[number]) => `    #[path = "."]
    pub mod ${e.mod} {
        #[path = "${STANDARD}/${EXAMPLES}/${e.dir}/${RS}"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "${STANDARD}/${EXAMPLES}/${e.dir}/${TESTS}/${EXAMPLE_TESTS}/${RS}"]
        mod tests;
    }
`;
const rootText = readFileSync(root, "utf8");
if (!rootText.includes("pub mod house {")) {
  const anchor = "pub mod examples {\n";
  if (rootText.split(anchor).length !== 2) throw new Error("examples mount anchor is not unique");
  const checksMount = `    #[cfg(test)]\n    #[path = "${STANDARD}/${EXAMPLES}/${CHECKS}/${RS}"]\n    pub mod checks;\n`;
  const fresh = readFileSync(root, "utf8");
  writeFileSync(root, fresh.replace(anchor, anchor + checksMount + mount(examples[0]) + mount(examples[1])));
  console.log("mounted house, office and checks in the artifact root");
} else console.log("artifact root already mounts the examples");
console.log("example sources written");
void existsSync;
void readdirSync;
void child;
