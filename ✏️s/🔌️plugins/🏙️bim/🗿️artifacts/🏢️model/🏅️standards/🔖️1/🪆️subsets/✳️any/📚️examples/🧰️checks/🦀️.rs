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
    on_storey!(walls, curtain_walls, columns, beams, slabs, roofs, stairs, railings, spaces);
    for (id, building) in &model.buildings {
        need(format!("buildings/{id}/site"), model.sites.contains_key(&building.site));
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
        model.walls.contains_key(id) || model.curtain_walls.contains_key(id) || model.columns.contains_key(id) || model.beams.contains_key(id) || model.slabs.contains_key(id) || model.roofs.contains_key(id) || model.openings.contains_key(id) || model.stairs.contains_key(id) || model.railings.contains_key(id) || model.spaces.contains_key(id)
    };
    for id in model.properties.keys() {
        need(format!("properties/{id}"), owned(id));
    }
    for id in model.classifications.keys() {
        need(format!("classifications/{id}"), owned(id));
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
    ("create-space", "createSpace", "space", spaces),
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
            state = apply_json(&state, &format!("{{\"mutation\":\"{variant}\",\"id\":{id},\"{field}\":{record}}}"));
            applied += 1;
        }
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
    let replayed = replay(model);
    eprintln!("replay of {name}: {} mutations applied, create kinds still missing: {:?}", replayed.applied, replayed.seeded);
    assert!(replayed.applied >= model.sites.len() + model.buildings.len() + model.storeys.len() + model.walls.len(), "the foundation kinds are replayed");
    assert!(replayed.result == *model, "replaying the creates reproduces the committed example");
}
