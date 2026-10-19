//! ➕️ Create rows of the BIM entity table: the create mutation of every entity kind. See [`super`].

use semio_s_artifact_bim_model_authored::editor::bim::entities::field_codecs::{first, number, point};
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{Axis, DoorLeaves, Layer, LayerFunction, LocationLine, MaterialCategory, OpeningKind, Phase, Point2, Profile, Rgb, RoofShape, SpaceBoundary, StairFlight, Swing, TopConstraint, Vertex};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Create
pub type Created = Result<ModelMutation, &'static str>;

pub fn container(map: &BTreeMap<String, impl Sized>, parent: &str, missing: &'static str) -> Result<String, &'static str> {
    map.contains_key(parent).then(|| parent.to_string()).ok_or(missing)
}

pub fn rectangle(width: f64, depth: f64) -> Vec<Vertex> {
    [(0.0, 0.0), (width, 0.0), (width, depth), (0.0, depth)].into_iter().map(|(x, y)| Vertex { point: Point2 { x, y }, bulge: 0.0 }).collect()
}

pub fn along(row: f64) -> (Point2, Point2) {
    (Point2 { x: 0.0, y: row }, Point2 { x: 4.0, y: row })
}

pub fn create_site(_: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    Ok(ModelMutation::CreateSite(semio_s_artifact_bim_model_mut_any_124::standards::v1::subsets::any::schema::mutations::create_site::CreateSite { id: id.into(), site: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Site { name: name.into(), latitude: 0.0, longitude: 0.0, elevation: 0.0, true_north: 0.0, boundary: Vec::new() } }))
}

pub fn create_building(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let site = snapshot.sites.contains_key(parent).then(|| parent.to_string()).or_else(|| first(&snapshot.sites)).ok_or("bim.create.site-missing")?;
    Ok(ModelMutation::CreateBuilding(semio_s_artifact_bim_model_mut_any_125::standards::v1::subsets::any::schema::mutations::create_building::CreateBuilding { id: id.into(), building: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Building { site, name: name.into(), origin: Point2 { x: 0.0, y: 0.0 }, rotation: 0.0, elevation: 0.0 } }))
}

pub fn create_storey(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let building = snapshot.buildings.contains_key(parent).then(|| parent.to_string()).or_else(|| first(&snapshot.buildings)).ok_or("bim.create.building-missing")?;
    let levels: Vec<&semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Storey> = snapshot.storeys.values().filter(|storey| storey.building == building).collect();
    let level = levels.iter().map(|storey| storey.level).max().map_or(0, |highest| highest + 1);
    let height = levels.iter().max_by_key(|storey| storey.level).map_or(3.0, |top| top.height);
    Ok(ModelMutation::CreateStorey(semio_s_artifact_bim_model_mut_any_126::standards::v1::subsets::any::schema::mutations::create_storey::CreateStorey { id: id.into(), storey: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Storey { building, name: name.into(), level, height, cut_height: None } }))
}

pub fn create_grid(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let building = container(&snapshot.buildings, parent, "bim.create.building-missing")?;
    let column = snapshot.grids.values().filter(|grid| grid.building == building).count() as f64;
    let (start, end) = (Point2 { x: column * 3.0, y: 0.0 }, Point2 { x: column * 3.0, y: 10.0 });
    Ok(ModelMutation::CreateGridLine(semio_s_artifact_bim_model_mut_any_132::standards::v1::subsets::any::schema::mutations::create_grid_line::CreateGridLine { id: id.into(), grid_line: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::GridLine { building, label: name.into(), start, end } }))
}

pub fn create_wall(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    let wall_type = first(&snapshot.wall_types).ok_or("bim.create.wall-type-missing")?;
    let row = snapshot.walls.values().filter(|wall| wall.storey == storey).count() as f64;
    let (start, end) = along(row);
    Ok(ModelMutation::CreateWall(semio_s_artifact_bim_model_mut_any_173::standards::v1::subsets::any::schema::mutations::create_wall::CreateWall {
        id: id.into(),
        wall: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Wall { storey, wall_type, axis: Axis::Line { start, end }, location: LocationLine::Center, base_offset: 0.0, top: TopConstraint::StoreyTop { offset: 0.0 }, phase: Phase::New, start_join: None, end_join: None, base_slab: None, name: name.into() },
    }))
}

pub fn create_curtain_wall(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    let curtain_wall_type = first(&snapshot.curtain_wall_types).ok_or("bim.create.curtain-wall-type-missing")?;
    let (start, end) = along(snapshot.curtain_walls.values().filter(|wall| wall.storey == storey).count() as f64);
    Ok(ModelMutation::CreateCurtainWall(semio_s_artifact_bim_model_mut_any_172::standards::v1::subsets::any::schema::mutations::create_curtain_wall::CreateCurtainWall {
        id: id.into(),
        curtain_wall: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::CurtainWall { storey, curtain_wall_type, axis: Axis::Line { start, end }, base_offset: 0.0, top: TopConstraint::StoreyTop { offset: 0.0 }, u_grid: None, v_grid: None, phase: Phase::New, name: name.into() },
    }))
}

pub fn create_column(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    let column_type = first(&snapshot.column_types).ok_or("bim.create.column-type-missing")?;
    let position = Point2 { x: snapshot.columns.values().filter(|column| column.storey == storey).count() as f64 * 3.0, y: 0.0 };
    Ok(ModelMutation::CreateColumn(semio_s_artifact_bim_model_mut_any_159::standards::v1::subsets::any::schema::mutations::create_column::CreateColumn {
        id: id.into(),
        column: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Column { storey, column_type, position, rotation: 0.0, tilt: None, base_offset: 0.0, top: TopConstraint::StoreyTop { offset: 0.0 }, phase: Phase::New, name: name.into() },
    }))
}

pub fn create_beam(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    let beam_type = first(&snapshot.beam_types).ok_or("bim.create.beam-type-missing")?;
    let (start, end) = along(snapshot.beams.values().filter(|beam| beam.storey == storey).count() as f64);
    Ok(ModelMutation::CreateBeam(semio_s_artifact_bim_model_mut_any_160::standards::v1::subsets::any::schema::mutations::create_beam::CreateBeam { id: id.into(), beam: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Beam { storey, beam_type, axis: Axis::Line { start, end }, top_offset: 0.0, end_top_offset: None, phase: Phase::New, name: name.into() } }))
}

pub fn create_slab(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    let slab_type = first(&snapshot.slab_types).ok_or("bim.create.slab-type-missing")?;
    Ok(ModelMutation::CreateSlab(semio_s_artifact_bim_model_mut_any_129::standards::v1::subsets::any::schema::mutations::create_slab::CreateSlab {
        id: id.into(),
        slab: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Slab { storey, slab_type, boundary: rectangle(4.0, 4.0), holes: Vec::new(), offset: 0.0, slope: None, phase: Phase::New, name: name.into() },
    }))
}

pub fn create_roof(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    let roof_type = first(&snapshot.roof_types).ok_or("bim.create.roof-type-missing")?;
    Ok(ModelMutation::CreateRoof(semio_s_artifact_bim_model_mut_any_130::standards::v1::subsets::any::schema::mutations::create_roof::CreateRoof {
        id: id.into(),
        roof: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Roof { storey, roof_type, footprint: rectangle(4.0, 4.0), shape: RoofShape::Flat, overhang: 0.3, base_offset: 0.0, phase: Phase::New, name: name.into() },
    }))
}

pub fn create_opening(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let host = snapshot.walls.contains_key(parent).then(|| parent.to_string()).or_else(|| snapshot.curtain_walls.contains_key(parent).then(|| parent.to_string())).ok_or("bim.create.wall-missing")?;
    let kind = match (first(&snapshot.window_types), first(&snapshot.door_types)) {
        (Some(window_type), _) => OpeningKind::Window { window_type },
        (None, Some(door_type)) => OpeningKind::Door { door_type },
        (None, None) => OpeningKind::Void { width: 0.9, height: 2.1 },
    };
    Ok(ModelMutation::CreateOpening(semio_s_artifact_bim_model_mut_any_175::standards::v1::subsets::any::schema::mutations::create_opening::CreateOpening {
        id: id.into(),
        opening: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Opening { host, kind, offset: 1.0, sill_override: None, width: None, height: None, flip_hand: false, flip_facing: false, reveal_depth: None, reveal_material: None, name: name.into() },
    }))
}

pub fn create_stair(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    Ok(ModelMutation::CreateStair(semio_s_artifact_bim_model_mut_any_131::standards::v1::subsets::any::schema::mutations::create_stair::CreateStair {
        id: id.into(),
        stair: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Stair {
            storey,
            start: Point2 { x: 0.0, y: 0.0 },
            direction: 0.0,
            width: 1.2,
            flight: StairFlight::Straight,
            top: TopConstraint::StoreyTop { offset: 0.0 },
            max_riser: 0.18,
            min_tread: 0.27,
            stringer: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::STANDARD_STRINGER,
            nosing: 0.0,
            tread_thickness: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::STANDARD_TREAD_THICKNESS,
            riser: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::STANDARD_RISER,
            landing_depth: 1.2,
            phase: Phase::New,
            name: name.into(),
        },
    }))
}

pub fn create_railing(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    let material = first(&snapshot.materials).ok_or("bim.create.material-missing")?;
    let (start, end) = along(snapshot.railings.values().filter(|railing| railing.storey == storey).count() as f64);
    Ok(ModelMutation::CreateRailing(semio_s_artifact_bim_model_mut_any_127::standards::v1::subsets::any::schema::mutations::create_railing::CreateRailing {
        id: id.into(),
        railing: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Railing { storey, path: vec![start, end], height: 1.0, post_spacing: 1.2, profile: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::standard_rail_profile(), post_profile: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::standard_post_profile(), baluster: None, infill: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::STANDARD_INFILL, material, base_offset: 0.0, host: None, phase: Phase::New, name: name.into() },
    }))
}

pub fn create_space(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    let number = (snapshot.spaces.values().filter(|space| space.storey == storey).count() + 1).to_string();
    Ok(ModelMutation::CreateSpace(semio_s_artifact_bim_model_mut_any_128::standards::v1::subsets::any::schema::mutations::create_space::CreateSpace {
        id: id.into(),
        space: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Space { storey, number, name: name.into(), boundary: SpaceBoundary::Bounded { seed: Point2 { x: 1.0, y: 1.0 } }, usage: String::new(), phase: Phase::New, zone: None, floor_finish: None, wall_finish: None, ceiling_finish: None },
    }))
}

pub fn create_material(_: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    Ok(ModelMutation::CreateMaterial(semio_s_artifact_bim_model_mut_any_74::standards::v1::subsets::any::schema::mutations::create_material::CreateMaterial {
        id: id.into(),
        material: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Material { name: name.into(), category: MaterialCategory::Other, color: Rgb { r: 0.7, g: 0.7, b: 0.7 }, density: 1000.0, conductivity: 1.0, specific_heat: 1000.0 },
    }))
}

pub fn layers(snapshot: &ModelSnapshot) -> Result<Vec<Layer>, &'static str> {
    let material = first(&snapshot.materials).ok_or("bim.create.material-missing")?;
    Ok(vec![Layer { material, thickness: 0.2, function: LayerFunction::Structure }])
}

pub fn create_wall_type(snapshot: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    Ok(ModelMutation::CreateWallType(semio_s_artifact_bim_model_mut_any_75::standards::v1::subsets::any::schema::mutations::create_wall_type::CreateWallType { id: id.into(), wall_type: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::WallType { name: name.into(), layers: layers(snapshot)? } }))
}

pub fn create_slab_type(snapshot: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    Ok(ModelMutation::CreateSlabType(semio_s_artifact_bim_model_mut_any_76::standards::v1::subsets::any::schema::mutations::create_slab_type::CreateSlabType { id: id.into(), slab_type: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::SlabType { name: name.into(), layers: layers(snapshot)? } }))
}

pub fn create_roof_type(snapshot: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    Ok(ModelMutation::CreateRoofType(semio_s_artifact_bim_model_mut_any_77::standards::v1::subsets::any::schema::mutations::create_roof_type::CreateRoofType { id: id.into(), roof_type: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::RoofType { name: name.into(), layers: layers(snapshot)? } }))
}

pub fn create_column_type(snapshot: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    let material = first(&snapshot.materials).ok_or("bim.create.material-missing")?;
    Ok(ModelMutation::CreateColumnType(semio_s_artifact_bim_model_mut_any_70::standards::v1::subsets::any::schema::mutations::create_column_type::CreateColumnType {
        id: id.into(),
        column_type: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ColumnType { name: name.into(), profile: Profile::Rectangle { width: 0.3, depth: 0.3 }, material },
    }))
}

pub fn create_beam_type(snapshot: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    let material = first(&snapshot.materials).ok_or("bim.create.material-missing")?;
    Ok(ModelMutation::CreateBeamType(semio_s_artifact_bim_model_mut_any_71::standards::v1::subsets::any::schema::mutations::create_beam_type::CreateBeamType {
        id: id.into(),
        beam_type: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::BeamType { name: name.into(), profile: Profile::Rectangle { width: 0.2, depth: 0.4 }, material },
    }))
}

pub fn create_window_type(snapshot: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    let material = first(&snapshot.materials).ok_or("bim.create.material-missing")?;
    Ok(ModelMutation::CreateWindowType(semio_s_artifact_bim_model_mut_any_72::standards::v1::subsets::any::schema::mutations::create_window_type::CreateWindowType {
        id: id.into(),
        window_type: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::WindowType { name: name.into(), width: 1.2, height: 1.2, sill: 0.9, frame_width: 0.06, frame_depth: 0.08, panes: 2, material, u_value: None, g_value: None, frame_fraction: None },
    }))
}

pub fn create_door_type(snapshot: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    let material = first(&snapshot.materials).ok_or("bim.create.material-missing")?;
    Ok(ModelMutation::CreateDoorType(semio_s_artifact_bim_model_mut_any_73::standards::v1::subsets::any::schema::mutations::create_door_type::CreateDoorType {
        id: id.into(),
        door_type: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::DoorType { name: name.into(), width: 0.9, height: 2.1, frame_width: 0.06, frame_depth: 0.08, leaves: DoorLeaves::Single, swing: Swing::Left, material, u_value: None },
    }))
}
//#endregion 🔖️Create
