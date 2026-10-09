//! 🧰️ Shared checks of the bundled examples: strict decode, schema validation by the third-party jsonschema library, text and pack
//! round trips, referential integrity, inference sanity, and the replay of a whole example through the mutation API.

use crate::mutations::{apply_model_mutation, ModelMutation, KINDS};
use crate::standards::v1::subsets::any::io::binary::snapshot as pack;
use crate::standards::v1::subsets::any::io::text::snapshot::{decode_model_snapshot_json, parse_dsl, print_dsl};
use crate::{ModelInference, ModelSnapshot, OpeningKind, TopConstraint};
use crate::standards::v1::subsets::any::schema::inferences::{diagnostics::Severity, spaces::SpaceStatus};
use protocol::Inference;
use std::path::{Path, PathBuf};

const VALIDATE: &str = concat!(
    "import json,sys,jsonschema\n",
    "from referencing import Registry,Resource\n",
    "load=lambda p:json.load(open(p,encoding='utf-8'))\n",
    "snap,art,inst=load(sys.argv[1]),load(sys.argv[2]),load(sys.argv[3])\n",
    "reg=Registry().with_resources([(art['$id'],Resource.from_contents(art)),(snap['$id'],Resource.from_contents(snap))])\n",
    "errs=sorted(jsonschema.Draft7Validator(snap,registry=reg).iter_errors(inst),key=lambda e:[str(x) for x in e.path])\n",
    "for e in errs[:10]:print('/'.join(str(x) for x in e.path),e.message[:160])\n",
    "sys.exit(1 if errs else 0)\n",
);

fn subset_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any").join(relative)
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
        subset_path("🖼️assets").join(self.dir).join(name)
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
            .arg(subset_path("🧬️schema/📸️snapshot/🔣️.json"))
            .arg(subset_path("🧬️schema/🔣️.json"))
            .arg(self.file("📸️snapshot.json"))
            .output()
            .expect("python runs");
        assert!(output.status.success(), "the snapshot violates its JSON Schema:\n{}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
    }

    pub fn bless(&self) {
        if std::env::var_os("BIM_BLESS").is_none() {
            return;
        }
        std::fs::write(self.file("🗣️.dsl.semio"), print_dsl(&self.model())).expect("the text is written");
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
    on_storey!(walls, curtain_walls, columns, beams, slabs, ceilings, roofs, stairs, ramps, railings, spaces);
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
    for (id, ceiling) in &model.ceilings {
        need(format!("ceilings/{id}/ceiling_type"), model.ceiling_types.contains_key(&ceiling.ceiling_type));
    }
    for (id, roof) in &model.roofs {
        need(format!("roofs/{id}/roof_type"), model.roof_types.contains_key(&roof.roof_type));
    }
    for (id, stair) in &model.stairs {
        need(format!("stairs/{id}/top"), top_storey(&stair.top).is_none_or(|storey| model.storeys.contains_key(storey)));
    }
    for (id, ramp) in &model.ramps {
        need(format!("ramps/{id}/material"), model.materials.contains_key(&ramp.material));
        need(format!("ramps/{id}/top"), top_storey(&ramp.top).is_none_or(|storey| model.storeys.contains_key(storey)));
    }
    for (id, railing) in &model.railings {
        need(format!("railings/{id}/material"), model.materials.contains_key(&railing.material));
        need(format!("railings/{id}/host"), railing.host.as_ref().is_none_or(|host| model.stairs.contains_key(&host.element) || model.ramps.contains_key(&host.element) || model.slabs.contains_key(&host.element)));
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
    layered!(wall_types, slab_types, ceiling_types, roof_types);
    macro_rules! of_material {
        ($($collection:ident),*) => {
            $(for (id, kind) in &model.$collection {
                need(format!("{}/{id}/material", stringify!($collection)), model.materials.contains_key(&kind.material));
            })*
        };
    }
    of_material!(column_types, beam_types, window_types, door_types);
    let owned = |id: &String| {
        model.walls.contains_key(id) || model.curtain_walls.contains_key(id) || model.columns.contains_key(id) || model.beams.contains_key(id) || model.slabs.contains_key(id) || model.ceilings.contains_key(id) || model.roofs.contains_key(id) || model.openings.contains_key(id) || model.stairs.contains_key(id) || model.ramps.contains_key(id) || model.railings.contains_key(id) || model.spaces.contains_key(id) || model.zones.contains_key(id) || model.area_schemes.contains_key(id) || model.views.contains_key(id)
    };
    for id in model.properties.keys() {
        need(format!("properties/{id}"), owned(id));
    }
    for id in model.classifications.keys() {
        need(format!("classifications/{id}"), owned(id));
    }
    for (id, view) in &model.views {
        need(format!("views/{id}/definition"), crate::view_problem(model, id, view).is_none());
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
    assert_eq!(inferred.ramp_runs.len(), model.ramps.len());
    for (id, layout) in &inferred.wall_layout {
        assert!(layout.height > 0.0 && layout.thickness > 0.0 && layout.length > 0.0, "wall {id} resolves to a solid extent");
    }
    for (id, frame) in &inferred.opening_frames {
        assert!(frame.valid, "opening {id} is valid: issues {:?}, overlaps {:?}", frame.issues, frame.overlaps);
    }
    for (id, run) in &inferred.stair_runs {
        assert!(run.compliance.compliant, "stair {id} is compliant: {:?}", run.compliance);
    }
    for (id, run) in &inferred.ramp_runs {
        assert!(run.compliance.compliant, "ramp {id} is compliant: {:?}", run.compliance);
    }
    assert_eq!(inferred.schedules.len(), model.schedules.len(), "every authored schedule has a table");
    for (id, schedule) in &model.schedules {
        schedules_add_up(id, schedule, &inferred.schedules[id]);
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

/// 📋️ A schedule table is consistent with its definition: it has a total row exactly when a column is summed and a row was listed, and every summed column of the total row is the sum of the rows the table lists (the items
/// when itemized, else the collapsed rows of the outermost level).
fn schedules_add_up(id: &str, schedule: &crate::Schedule, table: &crate::standards::v1::subsets::any::schema::inferences::schedules::ScheduleTable) {
    use crate::standards::v1::subsets::any::schema::inferences::schedules::{RowKind, ScheduleCell};
    assert_eq!(table.keys, schedule.columns.iter().map(|column| column.key.clone()).collect::<Vec<_>>(), "schedule {id} keeps its columns");
    let total = table.rows.iter().find(|row| row.kind == RowKind::Total);
    assert_eq!(total.is_some(), table.items > 0 && schedule.columns.iter().any(|column| column.total), "schedule {id} has a total row exactly when something is summed");
    let Some(total) = total else { return };
    let listed = |row: &&crate::standards::v1::subsets::any::schema::inferences::schedules::ScheduleRow| if schedule.itemize { row.kind == RowKind::Item } else { row.kind == RowKind::Group && row.level == 0 };
    for (index, column) in schedule.columns.iter().enumerate().filter(|(_, column)| column.total) {
        let sum: f64 = table.rows.iter().filter(listed).filter_map(|row| if let ScheduleCell::Number { value } = row.cells[index] { Some(value) } else { None }).sum();
        match total.cells[index] {
            ScheduleCell::Number { value } => assert!((value - sum).abs() < 1e-6 * sum.abs().max(1.0), "schedule {id}: the total of column {index} is {value}, its rows sum to {sum}"),
            ScheduleCell::Empty => assert!(sum == 0.0, "schedule {id}: column {index} sums to {sum} but its total is empty"),
            ref other => panic!("schedule {id}: the total of column {index} is {other:?}"),
        }
    }
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
    ("create-ceiling-type", "createCeilingType", "ceiling_type", ceiling_types),
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
    ("create-ceiling", "createCeiling", "ceiling", ceilings),
    ("create-roof", "createRoof", "roof", roofs),
    ("create-opening", "createOpening", "opening", openings),
    ("create-stair", "createStair", "stair", stairs),
    ("create-ramp", "createRamp", "ramp", ramps),
    ("create-railing", "createRailing", "railing", railings),
    ("create-zone", "createZone", "zone", zones),
    ("create-area-scheme", "createAreaScheme", "area_scheme", area_schemes),
    ("create-space", "createSpace", "space", spaces),
    ("create-view", "createView", "view", views),
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

/// 🔁️ Builds `model` from an empty snapshot with one `create-*` mutation per record, in dependency order, through the central applier.
/// Collections whose create kind is not part of `KINDS` yet are seeded into the starting snapshot and reported in `seeded`.
pub fn replay(model: &ModelSnapshot) -> Replay {
    replay_derived(model, "{\"derived\":[]}")
}

/// 🪞️ [`replay`] where the records a modify mutation makes from other records (`derivations`: `{"derived": [{"produced": [ids], "mutations": [mutation payloads]}], "moved": [{"id", "from", "to"}]}`) are not
/// created but made by those mutations, applied once every create is in: a copy, a mirror or an array replays as the toolset's own mutation, with the ids it mints. A `moved` element is created on its `from` storey
/// and stood on its `to` storey by `set-element-storey`, so the replay proves the storey move as well as the committed record.
pub fn replay_derived(model: &ModelSnapshot, derivations: &str) -> Replay {
    let derivations: serde_json::Value = serde_json::from_str(derivations).expect("the derivations are JSON");
    let rows_of = |key: &str| -> Vec<serde_json::Value> { derivations["derived"].as_array().expect("derived rows").iter().flat_map(|row| row[key].as_array().expect("a list").clone()).collect() };
    let moved: Vec<(String, String, String)> = derivations["moved"].as_array().map_or_else(Vec::new, |rows| rows.iter().map(|row| (row["id"].as_str().expect("an id").to_string(), row["from"].as_str().expect("a storey").to_string(), row["to"].as_str().expect("a storey").to_string())).collect());
    let produced: Vec<String> = rows_of("produced").iter().map(|id| id.as_str().expect("an id").to_string()).collect();
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
            if produced.iter().any(|made| id.trim_matches('"') == made) {
                continue;
            }
            let record = match moved.iter().find(|(element, _, _)| id.trim_matches('"') == element) {
                Some((_, from, _)) => {
                    let mut row: serde_json::Value = serde_json::from_str(&record).expect("the record is JSON");
                    row["storey"] = serde_json::Value::String(from.clone());
                    row.to_string()
                }
                None => record,
            };
            state = apply_json(&state, &format!("{{\"mutation\":\"{variant}\",\"id\":{id},\"{field}\":{record}}}"));
            applied += 1;
        }
    }
    for (element, _, to) in &moved {
        state = apply_json(&state, &format!("{{\"mutation\":\"setElementStorey\",\"id\":\"{element}\",\"storey\":\"{to}\"}}"));
        applied += 1;
    }
    for mutation in rows_of("mutations") {
        state = apply_json(&state, &mutation.to_string());
        applied += 1;
    }
    let quoted = |text: &String| semio_framework_pack_json::to_json_string(text);
    if set_property {
        for (id, sets) in &model.properties {
            for (pset, properties) in sets {
                for (property, value) in properties {
                    state = apply_json(&state, &format!("{{\"mutation\":\"setElementProperty\",\"id\":{},\"pset\":{},\"property\":{},\"value\":{}}}", quoted(id), quoted(pset), quoted(property), semio_framework_pack_json::to_json_string(value)));
                    applied += 1;
                }
            }
        }
    }
    if set_classification {
        for (id, classification) in &model.classifications {
            state = apply_json(&state, &format!("{{\"mutation\":\"setElementClassification\",\"id\":{},\"classification\":{}}}", quoted(id), semio_framework_pack_json::to_json_string(classification)));
            applied += 1;
        }
    }
    Replay { applied, seeded, result: state }
}

/// 🔁️ Asserts the replayed snapshot equals `model` and reports the kinds that were seeded instead of replayed.
pub fn assert_replay(name: &str, model: &ModelSnapshot) {
    assert_replay_derived(name, model, "{\"derived\":[]}");
}

/// 🪞️ [`assert_replay`] for an example whose derived records are made by modify mutations.
pub fn assert_replay_derived(name: &str, model: &ModelSnapshot, derivations: &str) {
    let replayed = replay_derived(model, derivations);
    eprintln!("replay of {name}: {} mutations applied, create kinds still missing: {:?}", replayed.applied, replayed.seeded);
    assert!(replayed.applied >= model.sites.len() + model.buildings.len() + model.storeys.len() + model.walls.len(), "the foundation kinds are replayed");
    assert!(replayed.result == *model, "replaying the creates reproduces the committed example");
}

/// 🖼️ Asserts the authored views of `model` are the ones the `createView` command makes for each of its buildings (the plan of every storey, the ceiling plan of every storey with ceilings, the four elevations, the section, the camera)
/// plus the second section, the first one turned a quarter, so the examples show exactly what a person gets by asking the editor for views.
pub fn assert_views_are_the_commands(name: &str, model: &ModelSnapshot) {
    use crate::editor::bim::commands::create_view::{extents, handle, CreateView, MARGIN};
    use crate::editor::bim::unit_tests::support::{ctx, run};
    use crate::{Point2, View, ViewPlane};
    let bare = ModelSnapshot { views: Default::default(), ..model.clone() };
    let made = |kind: &str, parent: &str| -> Vec<View> {
        let mut context = ctx(&[]);
        let emit = run(&bare, |doc, cfg| handle(&CreateView { kind: kind.into(), parent: parent.into(), name: String::new() }, doc, cfg, &mut context)).unwrap_or_else(|error| panic!("{name}: {kind} of {parent}: {error:?}"));
        emit.artifact_mutations
            .iter()
            .map(|mutation| match mutation {
                ModelMutation::CreateView(payload) => payload.view.clone(),
                other => panic!("only views are created, got {other:?}"),
            })
            .collect()
    };
    let mut expected = 0;
    for building in model.buildings.keys() {
        let mut commanded: Vec<View> = Vec::new();
        for storey in model.storeys.iter().filter(|(_, storey)| &storey.building == building).map(|(id, _)| id) {
            commanded.extend(made("plan", storey));
            if model.ceilings.values().any(|ceiling| &ceiling.storey == storey) {
                commanded.extend(made("ceiling-plan", storey));
            }
        }
        if let Some(rect) = extents(model, building) {
            let section = made("section", building).remove(0);
            let middle = (rect[0] + rect[2]) / 2.0;
            let turned = View { plane: Some(ViewPlane { start: Point2 { x: middle, y: rect[1] - MARGIN }, end: Point2 { x: middle, y: rect[3] + MARGIN } }), depth: (rect[2] - rect[0]) / 2.0 + MARGIN, ..section.clone() };
            commanded.extend(made("elevations", building));
            commanded.extend([section, turned]);
            commanded.extend(made("perspective", building));
        }
        let anonymous = |view: &View| View { name: String::new(), ..view.clone() };
        for view in &commanded {
            let found = model.views.values().any(|authored| anonymous(authored) == anonymous(view));
            assert!(found, "{name}: no authored view equals what the command makes: {view:?}");
        }
        expected += commanded.len();
    }
    assert_eq!(model.views.len(), expected, "{name}: every authored view is one the command makes");
}
