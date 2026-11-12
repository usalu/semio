use super::camera::*;
use super::colour::*;
use super::config::*;
use super::description::*;
use super::lowering::*;
use super::measure::*;
use super::mesh::*;
use super::parts::*;
use super::fields::{self, FieldSelection, Site};
use super::picking::*;
use super::playback::{self, PlaybackCommand, PlaybackState};
use super::pose::*;
use super::scene::*;
use super::window::*;
use semio_framework_pack_json::Value;

const PRISMS: &str = include_str!("../🧫️fixtures/🔣️prisms.json");
const DYNAMICS: &str = include_str!("../🧫️fixtures/🎬️dynamics.json");

fn copy(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn dimensioned(profile: ProfileKind, params: &[(&str, f64)]) -> SectionShape {
    SectionShape::parametric(profile, params)
}

fn heb240() -> SectionShape {
    dimensioned(ProfileKind::IShape, &[("h", 0.24), ("b", 0.24), ("tw", 0.01), ("tf", 0.017)])
}

fn entity(kind: EntityKind, id: &str, geometry: Geometry, role: MaterialRole, util: Option<(f64, CheckStatus)>, layer: &str) -> SceneEntity {
    let mut appearance = Appearance::role(role);
    if util.is_some() { appearance.channel = ColourChannel::Utilisation; }
    SceneEntity {
        entity: EntityRef::new(kind, id, format!("{}[id={id}]", granularity(kind))),
        geometry,
        appearance,
        utilisation: util.map(|(ratio, status)| UtilisationBinding { check_id: format!("check-{id}"), ratio, status, per_station: None, stale: false }),
        label: None,
        accessible_name: copy(&format!("Entity {id}"), &format!("Objekt {id}")),
        accessible_detail: copy("detail", "Detail"),
        layer: layer.into(),
        pick: PickPolicy::Whole,
        handles: Vec::new(),
        content_hash: String::new(),
    }.sealed()
}

fn beam(id: &str, to_x: f64, util: Option<(f64, CheckStatus)>) -> SceneEntity {
    entity(EntityKind::Member, id, Geometry::Member(MemberGeometry {
        axis: Path3::Polyline(Polyline3 { points: vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(to_x, 0.0, 0.0)] }),
        frame: Frame::WORLD,
        section: heb240(),
        section_taper: None,
        start_offset: Vec2::new(0.0, 0.0),
        end_offset: Vec2::new(0.0, 0.0),
        start_release: EndRelease::Rigid,
        end_release: EndRelease::Rigid,
        stations: Vec::new(),
    }), MaterialRole::Steel, util, "members")
}

fn bolts(id: &str, nx: u32, ny: u32) -> SceneEntity {
    entity(EntityKind::Fastener, id, Geometry::Fastener(FastenerGeometry {
        kind: FastenerKind::Bolt { class: "8.8".into(), preload: None },
        axis: Segment3 { from: Vec3::new(0.0, 0.0, 0.1), to: Vec3::new(0.0, 0.0, 0.0) },
        diameter: 0.02,
        head: HeadShape::Hex,
        pattern: Some(Pattern::Grid { nx, ny, sx: 0.06, sy: 0.06 }),
        hole_clearance: None,
    }), MaterialRole::Bolt, None, "fasteners")
}

fn layers() -> Vec<Layer> {
    ["members", "fasteners", "loads", "supports"].iter().map(|id| Layer { id: (*id).into(), name: copy(id, id), visible: true, locked: false }).collect()
}

fn scene_of(entities: Vec<SceneEntity>) -> StructuralScene {
    StructuralScene {
        schema: SCENE_SCHEMA_ID.into(),
        revision: SceneRevision { document_seq: 1, report_seq: 1 },
        bounds: Aabb::new(Vec3::new(0.0, -1.0, -1.0), Vec3::new(4.0, 1.0, 1.0)),
        entities,
        annotations: Vec::new(),
        legends: Vec::new(),
        layers: layers(),
        tracks: Vec::new(),
        owners: Vec::new(),
        description: copy("A steel frame with two beams.", "Ein Stahlrahmen mit zwei Trägern."),
    }
}

fn inputs<'a>(config: &'a Viewport3dWindowConfig, selected: &'a [String], measure: &'a MeasureState) -> ViewportInputs<'a> {
    ViewportInputs { config, selected, hovered: None, measure, lang: Lang::En }
}

fn lowered(scene: &StructuralScene, config: &Viewport3dWindowConfig, selected: &[String]) -> LoweredViewport {
    lower(scene, &inputs(config, selected, &MeasureState::default()), &BasicTessellator::default())
}

fn num(v: &Value, key: &str, i: usize) -> f64 {
    v.get(key).and_then(Value::as_array).and_then(|a| a[i].as_f64()).expect("number")
}

#[test]
fn section_prisms_match_the_python_oracle_volumes() {
    let oracle: Value = semio_framework_pack_json::parse(PRISMS, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("oracle golden");
    for (name, section) in [
        ("ishape", heb240()),
        ("rhs", dimensioned(ProfileKind::Rhs, &[("h", 0.2), ("b", 0.1), ("t", 0.01)])),
        ("chs", dimensioned(ProfileKind::Chs, &[("d", 0.1), ("t", 0.005)])),
    ] {
        let volume: f64 = section.outline().unwrap().iter().map(|p| prism(p, 1).volume()).sum();
        let expected = oracle.get(name).and_then(|v| v.get("area")).and_then(Value::as_f64).expect("area");
        assert!((volume - expected).abs() < 1e-9, "{name}: {volume} vs {expected}");
    }
}

#[test]
fn triangulation_preserves_area_of_concave_polygons_with_holes() {
    let polygon = dimensioned(ProfileKind::Rhs, &[("h", 0.2), ("b", 0.1), ("t", 0.01)]).outline().unwrap().remove(0);
    let (points, triangles) = triangulate(&polygon);
    let area: f64 = triangles.iter().map(|t| { let (a, b, c) = (points[t[0]], points[t[1]], points[t[2]]); ((b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])) / 2.0 }).sum();
    assert!((area - polygon.area()).abs() < 1e-12);
    assert!(triangles.iter().all(|t| { let (a, b, c) = (points[t[0]], points[t[1]], points[t[2]]); (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]) > 0.0 }));
}

#[test]
fn basis_pose_rotates_local_axes_onto_the_requested_directions() {
    let pose = span_pose([0.0; 3], [4.0, 0.0, 0.0], [0.0, 1.0, 0.0], (1.0, 1.0));
    assert_eq!(pose.rotation.map(|c| (c * 1e9).round() / 1e9), [0.5, 0.5, 0.5, 0.5]);
    assert_eq!(pose.scale, [1.0, 1.0, 4.0]);
    let z = rotate(pose.rotation, [0.0, 0.0, 1.0]);
    assert!(len(sub(z, [1.0, 0.0, 0.0])) < 1e-12);
}

#[test]
fn beam_lowers_to_one_scoped_instance_with_the_golden_transform() {
    let scene = scene_of(vec![beam("b1", 4.0, None)]);
    let out = lowered(&scene, &Viewport3dWindowConfig::default(), &[]);
    assert_eq!(out.instances.len(), 1);
    let i = &out.instances[0];
    assert_eq!(num(i, "position", 0), 0.0);
    assert_eq!(i.get("rotation").map(|r| r.as_array().unwrap().iter().map(|c| c.as_f64().unwrap()).collect::<Vec<_>>()), Some(vec![0.5, 0.5, 0.5, 0.5]));
    assert_eq!(i.get("scale").map(|r| r.as_array().unwrap().iter().map(|c| c.as_f64().unwrap()).collect::<Vec<_>>()), Some(vec![1.0, 1.0, 4.0]));
    assert_eq!(i.get("interactionGranularityId").and_then(Value::as_str), Some("member"));
    let id = i.get("id").and_then(Value::as_str).unwrap();
    assert_eq!(id, entity_address(EntityKind::Member, "b1").unwrap());
    assert_eq!(out.meshes.len(), 1);
    let data = out.meshes[0].get("data").unwrap();
    assert!(["positions", "normals", "colors", "indices"].iter().all(|k| data.get(k).is_some()));
}

#[test]
fn equal_sections_share_one_mesh_and_utilisation_buckets_split_colours() {
    let scene = scene_of(vec![beam("b1", 4.0, Some((0.64, CheckStatus::Pass))), beam("b2", 6.0, Some((0.64, CheckStatus::Pass))), beam("b3", 5.0, Some((1.1, CheckStatus::Fail)))]);
    let out = lowered(&scene, &Viewport3dWindowConfig::default(), &[]);
    assert_eq!(out.instances.len(), 3);
    assert_eq!(out.meshes.len(), 2);
    let id_of = |n: usize| out.instances[n].get("meshId").and_then(Value::as_str).unwrap().to_string();
    assert_eq!(id_of(0), id_of(1));
    assert_ne!(id_of(0), id_of(2));
}

#[test]
fn repeated_fasteners_are_instanced_from_two_meshes() {
    let scene = scene_of(vec![bolts("g1", 10, 10)]);
    let out = lowered(&scene, &Viewport3dWindowConfig::default(), &[]);
    assert_eq!(out.instances.len(), 200);
    assert_eq!(out.meshes.len(), 2);
    assert!(out.instances.iter().skip(1).any(|i| i.get("interactionId").is_some()));
}

#[test]
fn device_profile_decides_lod_and_budget() {
    let scene = scene_of(vec![bolts("g1", 10, 10)]);
    let mobile = Viewport3dWindowConfig { device: DeviceClass::Mobile, ..Viewport3dWindowConfig::default() };
    assert_eq!(lowered(&scene, &mobile, &[]).instances.len(), 100);
    let flood = scene_of(vec![bolts("g1", 60, 60)]);
    let out = lowered(&flood, &mobile, &[]);
    assert_eq!(out.stats.truncated, 1);
    assert!(out.instances.is_empty());
    assert!(out.description.contains("hidden to stay within the device budget"));
    assert_eq!(lowered(&flood, &Viewport3dWindowConfig::default(), &[]).stats.truncated, 0);
}

#[test]
fn utilisation_scalar_field_and_legend_follow_the_focused_member() {
    let mut worst = beam("b2", 4.0, Some((1.12, CheckStatus::Fail)));
    worst.utilisation.as_mut().unwrap().per_station = Some(vec![StationValue { at: 0.0, value: 0.5 }, StationValue { at: 1.0, value: 1.12 }]);
    let scene = scene_of(vec![beam("b1", 4.0, Some((0.64, CheckStatus::Pass))), worst]);
    let out = lowered(&scene, &Viewport3dWindowConfig::default(), &[]);
    let field = out.scalar_field.expect("scalar field of the worst member");
    assert_eq!(field.range.max, 1.2);
    let mesh = out.meshes.iter().find(|m| m.get("id").and_then(Value::as_str) == Some(field.mesh_id.as_str())).expect("field mesh");
    let vertices = mesh.get("data").and_then(|d| d.get("positions")).and_then(Value::as_array).unwrap().len() / 3;
    assert_eq!(field.values.len(), vertices);
    let (lo, hi) = field.values.iter().flatten().fold((f64::MAX, f64::MIN), |a, v| (a.0.min(*v), a.1.max(*v)));
    assert!((lo - 0.5).abs() < 1e-9 && (hi - 1.12).abs() < 1e-9);
    let selected = vec![entity_address(EntityKind::Member, "b1").unwrap()];
    let chosen = lowered(&scene, &Viewport3dWindowConfig::default(), &selected).scalar_field.unwrap();
    assert!(chosen.values.iter().flatten().all(|v| (*v - 0.64).abs() < 1e-12));
}

#[test]
fn selection_hover_and_ghost_styles_reach_the_instances() {
    let mut ghost = beam("g", 2.0, None);
    ghost.appearance.style = StyleState::Ghost;
    let scene = scene_of(vec![beam("b1", 4.0, None), ghost]);
    let selected = vec![entity_address(EntityKind::Member, "b1").unwrap()];
    let measure = MeasureState::default();
    let hover = entity_address(EntityKind::Member, "g").unwrap();
    let config = Viewport3dWindowConfig::default();
    let out = lower(&scene, &ViewportInputs { hovered: Some(&hover), ..inputs(&config, &selected, &measure) }, &BasicTessellator::default());
    assert_eq!(out.selection_ids, selected);
    assert_eq!(out.hovered.as_deref(), Some(hover.as_str()));
    assert_eq!(out.instances[0].get("selected").and_then(Value::as_bool), Some(true));
    assert_eq!(out.instances[1].get("provisional").and_then(Value::as_bool), Some(true));
    assert_eq!(out.instances[1].get("hovered").and_then(Value::as_bool), Some(true));
    let scene3d = world3d_scene_of(&out, "norm.en1993", true);
    assert_eq!(scene3d.domain_id.as_deref(), Some("norm.en1993"));
    assert!(scene3d.selection_json.contains("b1") || scene3d.selection_json.contains(&selected[0]));
}

#[test]
fn layers_load_kinds_and_section_are_applied() {
    let load = entity(EntityKind::Load, "w1", Geometry::Load(LoadGlyph::Point { at: Vec3::new(2.0, 0.0, 0.5), direction: Vec3::new(0.0, 0.0, -1.0), magnitude: Quantity::new(QuantityKind::Force, 50_000.0), case: LoadCaseRef { id: "lc-w".into(), kind: LoadKind::Wind, psi: None } }), MaterialRole::Load { kind: LoadKind::Wind }, None, "loads");
    let scene = scene_of(vec![beam("b1", 4.0, None), load]);
    let full = lowered(&scene, &Viewport3dWindowConfig::default(), &[]);
    assert_eq!(full.instances.len(), 3);
    let hidden_layer = Viewport3dWindowConfig { hidden_layers: vec!["loads".into()], ..Viewport3dWindowConfig::default() };
    assert_eq!(lowered(&scene, &hidden_layer, &[]).instances.len(), 1);
    let hidden_kind = Viewport3dWindowConfig { hidden_load_kinds: vec![LoadKind::Wind], ..Viewport3dWindowConfig::default() };
    assert_eq!(lowered(&scene, &hidden_kind, &[]).instances.len(), 1);
    let solo = Viewport3dWindowConfig { solo_layer: Some("loads".into()), ..Viewport3dWindowConfig::default() };
    assert_eq!(lowered(&scene, &solo, &[]).instances.len(), 2);
    let cut = Viewport3dWindowConfig { section: Some(SectionPlane { origin: Vec3::new(2.0, 0.0, 0.0), normal: Vec3::new(1.0, 0.0, 0.0), cap: true }), ..Viewport3dWindowConfig::default() };
    assert!(lowered(&scene, &cut, &[]).options.section.is_some_and(|s| s.cap.is_some()));
}

#[test]
fn deformed_members_keep_a_ghost_and_scale_with_the_animation_phase() {
    let mut deformed = beam("b1", 4.0, None);
    deformed.appearance.deformation = Some(DeformationOverlay { surface: Vec::new(), motion: Motion::Harmonic, load_factor: None, kind: DeformationKind::Mode, index: 1, frequency: None, participation: 1.0, scale: 10.0, samples: vec![DisplacementSample { at: 0.0, delta: Vec3::new(0.0, 0.0, 0.0) }, DisplacementSample { at: 0.5, delta: Vec3::new(0.0, 0.0, -0.01) }, DisplacementSample { at: 1.0, delta: Vec3::new(0.0, 0.0, 0.0) }] });
    let scene = scene_of(vec![deformed]);
    let deepest = |config: &Viewport3dWindowConfig| lowered(&scene, config, &[]).instances.iter().map(|i| num(i, "position", 2)).fold(0.0, f64::min);
    let out = lowered(&scene, &Viewport3dWindowConfig::default(), &[]);
    assert_eq!(out.instances.len(), 1 + 16);
    assert_eq!(out.instances[0].get("disabled").and_then(Value::as_bool), Some(true));
    assert!((deepest(&Viewport3dWindowConfig::default()) + 0.1).abs() < 1e-6);
    assert!(deepest(&Viewport3dWindowConfig { deformation_phase: 0.0, ..Viewport3dWindowConfig::default() }).abs() < 1e-9);
    assert!((deepest(&Viewport3dWindowConfig { deformation_phase: -0.5, ..Viewport3dWindowConfig::default() })).abs() < 1e-9);
}

#[test]
fn annotations_are_bilingual_and_follow_density() {
    let mut scene = scene_of(vec![beam("b1", 4.0, Some((1.2, CheckStatus::Fail)))]);
    scene.annotations = vec![
        Annotation::Dimension { id: "d1".into(), from: Vec3::new(0.0, 0.0, 0.0), to: Vec3::new(3.2, 0.0, 0.0), offset: Vec3::new(0.0, 0.0, -0.5), value: Quantity::new(QuantityKind::Length, 3.2), binding: None },
        Annotation::Label { id: "l1".into(), anchor: Vec3::new(1.0, 0.0, 0.0), text: copy("Beam", "Träger"), tone: Tone::Info },
    ];
    let texts = |config: &Viewport3dWindowConfig| lowered(&scene, config, &[]).annotations.map(|l| l.items.iter().map(|a| (a.id().to_string(), a.text().clone())).collect::<Vec<_>>()).unwrap_or_default();
    let normal = texts(&Viewport3dWindowConfig::default());
    let dim = normal.iter().find(|(id, _)| id == "d1").unwrap();
    assert_eq!((dim.1.en.as_str(), dim.1.de.as_str()), ("3.2 m", "3,2 m"));
    assert!(normal.iter().any(|(id, t)| id == "status:b1" && t.de.starts_with("Versagt")));
    let mm = texts(&Viewport3dWindowConfig { unit: LengthUnit::Millimetre, ..Viewport3dWindowConfig::default() });
    assert_eq!(mm.iter().find(|(id, _)| id == "d1").unwrap().1.en, "3200 mm");
    let low = texts(&Viewport3dWindowConfig { label_density: LabelDensity::Low, ..Viewport3dWindowConfig::default() });
    assert!(low.iter().all(|(id, _)| id != "l1") && low.iter().any(|(id, _)| id == "d1"));
    let off = texts(&Viewport3dWindowConfig { label_density: LabelDensity::Off, ..Viewport3dWindowConfig::default() });
    assert!(off.iter().all(|(id, _)| id.starts_with("status:")));
}

#[test]
fn camera_presets_fit_selection_and_worst_check() {
    let scene = scene_of(vec![beam("b1", 4.0, Some((0.4, CheckStatus::Pass))), {
        let mut far = beam("b2", 2.0, Some((0.9, CheckStatus::Pass)));
        if let Geometry::Member(m) = &mut far.geometry { m.axis = Path3::Polyline(Polyline3 { points: vec![Vec3::new(10.0, 0.0, 0.0), Vec3::new(12.0, 0.0, 0.0)] }); }
        far
    }]);
    let plan = preset_pose(&scene, CameraPreset::Plan);
    assert!(plan.position[2] > plan.target[2] + 1.0 && (plan.position[0] - plan.target[0]).abs() < 1e-3);
    let front = preset_pose(&scene, CameraPreset::ElevationFront);
    assert!(front.position[1] < front.target[1] && (front.position[2] - front.target[2]).abs() < 1e-9);
    assert_eq!(worst_entity(&scene), Some("b2"));
    let worst = fit_worst(&scene, CameraPreset::Iso);
    assert!((worst.target[0] - 11.0).abs() < 1e-9);
    assert!((fit_selection(&scene, &["b1".to_string()], CameraPreset::Iso).target[0] - 2.0).abs() < 1e-9);
    assert!(front.json().contains("\"fov\""));
}

#[test]
fn measure_tool_reports_distance_delta_and_angle() {
    let state = MeasureState::default().push([0.0, 0.0, 0.0]).push([3.0, 0.0, 4.0]);
    let m = state.measurement().unwrap();
    assert!((m.distance - 5.0).abs() < 1e-12 && (m.horizontal - 3.0).abs() < 1e-12 && m.angle_deg.is_none());
    let angle = state.push([3.0, 4.0, 4.0]).measurement().unwrap().angle_deg.unwrap();
    assert!((angle - 90.0).abs() < 1e-9);
    assert_eq!(state.push([1.0; 3]).push([2.0; 3]).points.len(), 1);
    let scene = scene_of(vec![beam("b1", 4.0, None)]);
    let config = Viewport3dWindowConfig::default();
    let out = lower(&scene, &inputs(&config, &[], &state), &BasicTessellator::default());
    assert!(out.annotations.unwrap().items.iter().any(|a| a.id() == "measure:distance" && a.text().de.starts_with("Abstand")));
}

#[test]
fn picks_resolve_to_entity_and_field_path() {
    let scene = scene_of(vec![beam("b1", 4.0, None)]);
    let address = entity_address(EntityKind::Member, "b1").unwrap();
    let Some(PickTarget::Entity { entity, field_path }) = resolve(&scene, &format!("{address}#3")) else { panic!("entity") };
    assert_eq!((entity.id.as_str(), field_path.as_str()), ("b1", "member[id=b1]"));
    assert!(resolve(&scene, &entity_address(EntityKind::Plate, "b1").unwrap()).is_none());
    let check = crate::protocol::wire::encode_scoped_identity(&[GRANULARITY_CHECK, "c1"]).unwrap();
    assert_eq!(resolve(&scene, &check), Some(PickTarget::Check("c1".into())));
}

#[test]
fn plate_pick_policy_publishes_face_edge_and_vertex_targets() {
    let mut plate = entity(EntityKind::Plate, "p1", Geometry::Plate(PlateGeometry { frame: Frame::WORLD, outline: Polygon2::solid(vec![Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0)]), thickness: 0.02, offset: 0.0, stiffeners: Vec::new() }), MaterialRole::Steel, None, "members");
    plate.pick = PickPolicy::Edge;
    let out = lowered(&scene_of(vec![plate]), &Viewport3dWindowConfig::default(), &[]);
    let targets: Value = semio_framework_pack_json::parse(&out.pick_targets_json.expect("pick lane"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(targets.as_array().unwrap().len(), 4);
    assert_eq!(targets.as_array().unwrap()[0].get("kind").and_then(Value::as_str), Some("edge"));
}

#[test]
fn instance_delta_lists_changed_and_removed_ids() {
    let a = lowered(&scene_of(vec![beam("b1", 4.0, None), beam("b2", 4.0, None)]), &Viewport3dWindowConfig::default(), &[]);
    let b = lowered(&scene_of(vec![beam("b1", 5.0, None)]), &Viewport3dWindowConfig::default(), &[]);
    let delta = instances_delta(1, 2, &a.instances, &b.instances);
    assert_eq!(delta.get("count").and_then(Value::as_u64), Some(1));
    assert_eq!(delta.get("changed").and_then(Value::as_array).map(Vec::len), Some(1));
    assert_eq!(delta.get("removed").and_then(Value::as_array).map(Vec::len), Some(1));
    assert_eq!(instances_delta(2, 3, &b.instances, &b.instances).get("changed").and_then(Value::as_array).map(Vec::len), Some(0));
}

#[test]
fn descriptions_and_announcements_exist_in_both_languages() {
    let scene = scene_of(vec![beam("b1", 4.0, Some((0.64, CheckStatus::Pass)))]);
    let address = entity_address(EntityKind::Member, "b1").unwrap();
    assert_eq!(announce_selection(&scene, Lang::En, &address).as_deref(), Some("Entity b1 selected, utilisation 64 %, passes"));
    assert_eq!(announce_selection(&scene, Lang::De, &address).as_deref(), Some("Objekt b1 ausgewählt, Ausnutzung 64 %, besteht"));
    let stats = LoweringStats { entities: 1, ..LoweringStats::default() };
    assert!(describe(&scene, Lang::De, &stats, std::slice::from_ref(&address)).contains("Ausgewählt: Objekt b1."));
    assert!(describe(&scene, Lang::En, &stats, &[]).starts_with("A steel frame"));
}

#[test]
fn utilisation_ramp_is_monotone_in_the_viridis_blue_channel_and_quantised() {
    let c = |r| ramp_rgb(ColourRamp::Viridis, r, 1.2);
    assert_eq!(c(0.0), [68.0 / 255.0, 1.0 / 255.0, 84.0 / 255.0]);
    assert_eq!(c(1.2), [253.0 / 255.0, 231.0 / 255.0, 37.0 / 255.0]);
    assert_eq!(c(0.5000), c(0.5001));
    assert_eq!(rgb_hex([1.0, 0.0, 0.5]), "ff0080");
}

#[test]
fn config_roundtrips_through_the_value_codec_and_validates() {
    use semio_framework_value::{FromValue, ToValue};
    let config = Viewport3dWindowConfig { active_field: Some("settlement".into()), playback_speed: 2.0, preset: CameraPreset::ElevationLeft, hidden_load_kinds: vec![LoadKind::Snow], section: Some(SectionPlane { origin: Vec3::new(1.0, 2.0, 3.0), normal: Vec3::new(0.0, 0.0, 1.0), cap: true }), ..Viewport3dWindowConfig::default() };
    assert_eq!(Viewport3dWindowConfig::from_value(config.to_value()).unwrap(), config);
    assert!(config.validate().is_ok());
    assert!(Viewport3dWindowConfig { domain_max: 0.0, ..config }.validate().is_err());
}

#[test]
fn every_geometry_kind_lowers_to_instances() {
    let r = |id: &str| EntityRef::new(EntityKind::Member, id, "m");
    let square = || Polygon2::solid(vec![Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0)]);
    let ring3 = || Polygon3 { outer: Ring3 { points: vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 0.0)] }, holes: Vec::new() };
    let case = || LoadCaseRef { id: "lc".into(), kind: LoadKind::Variable, psi: None };
    let force = |v| Quantity::new(QuantityKind::Force, v);
    let geoms = vec![
        (EntityKind::Plate, Geometry::Plate(PlateGeometry { frame: Frame::WORLD, outline: square(), thickness: 0.02, offset: 0.0, stiffeners: vec![Stiffener { path: Polyline2 { points: vec![Vec2::new(0.0, 0.5), Vec2::new(1.0, 0.5)] }, section: SectionShape::parametric(ProfileKind::Rect, &[("h", 0.05), ("b", 0.01)]) }] })),
        (EntityKind::Shell, Geometry::Shell(ShellGeometry { shape: ShellShape::Revolved { origin: Vec3::new(0.0, 0.0, 0.0), axis: Vec3::new(0.0, 0.0, 1.0), generatrix: Polyline2 { points: vec![Vec2::new(1.0, 0.0), Vec2::new(1.0, 2.0)] }, sweep: std::f64::consts::TAU }, thickness: 0.01 })),
        (EntityKind::Solid, Geometry::Solid(SolidGeometry { shape: SolidShape::Box { frame: Frame::WORLD, size: Vec3::new(1.0, 1.0, 1.0) } })),
        (EntityKind::Solid, Geometry::Solid(SolidGeometry { shape: SolidShape::Cylinder { frame: Frame::WORLD, radius: 0.3, height: 1.0 } })),
        (EntityKind::Solid, Geometry::Solid(SolidGeometry { shape: SolidShape::Wedge { frame: Frame::WORLD, base: Vec2::new(1.0, 1.0), height: 1.0 } })),
        (EntityKind::Solid, Geometry::Solid(SolidGeometry { shape: SolidShape::StrataColumn { frame: Frame::WORLD, footprint: square(), strata: vec![Stratum { thickness: 1.0, role: MaterialRole::Soil { class: "sand".into() }, label: copy("Sand", "Sand") }, Stratum { thickness: 2.0, role: MaterialRole::Soil { class: "clay".into() }, label: copy("Clay", "Ton") }] } })),
        (EntityKind::Solid, Geometry::Solid(SolidGeometry { shape: SolidShape::FootingBody { frame: Frame::WORLD, footprint: square(), thickness: 0.5, pedestal: Some(Vec2::new(0.4, 0.4)), pedestal_height: 0.6 } })),
        (EntityKind::Solid, Geometry::Solid(SolidGeometry { shape: SolidShape::WallAssembly { path: Polyline3 { points: vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(3.0, 0.0, 0.0)] }, height: 2.5, layers: vec![WallLayer { thickness: 0.1, role: MaterialRole::Masonry, label: copy("Brick", "Ziegel") }, WallLayer { thickness: 0.2, role: MaterialRole::Concrete, label: copy("Core", "Kern") }], openings: vec![WallCut { offset: 1.0, width: 0.9, sill: 0.0, height: 2.1 }] } })),
        (EntityKind::Fastener, Geometry::Fastener(FastenerGeometry { kind: FastenerKind::Weld { throat: 0.004, weld: WeldKind::Fillet }, axis: Segment3 { from: Vec3::new(0.0, 0.0, 0.0), to: Vec3::new(0.3, 0.0, 0.0) }, diameter: 0.01, head: HeadShape::None, pattern: None, hole_clearance: None })),
        (EntityKind::Fastener, Geometry::Fastener(FastenerGeometry { kind: FastenerKind::Rebar { bend: Some(BendShape { bend_angle: 1.5, bend_radius: 0.05, leg_length: 0.2, both_ends: false }) }, axis: Segment3 { from: Vec3::new(0.0, 0.0, 0.0), to: Vec3::new(1.0, 0.0, 0.0) }, diameter: 0.016, head: HeadShape::None, pattern: Some(Pattern::Cage { outline: Ring2 { points: vec![Vec2::new(-0.1, -0.2), Vec2::new(0.1, -0.2), Vec2::new(0.1, 0.2), Vec2::new(-0.1, 0.2)] }, zones: vec![LinkZone { start: 0.0, spacing: 0.15, count: 5 }], reference: Vec3::new(0.0, 0.0, 1.0) }), hole_clearance: None })),
        (EntityKind::Support, Geometry::Support(SupportGeometry { kind: SupportKind::Roller { normal: Vec3::new(0.0, 0.0, 1.0) }, at: Vec3::new(0.0, 0.0, 0.0), frame: Frame::WORLD })),
        (EntityKind::Support, Geometry::Support(SupportGeometry { kind: SupportKind::ElasticBed { stiffness: Quantity::new(QuantityKind::Stiffness, 1.0e6), extent: ring3() }, at: Vec3::new(0.0, 0.0, 0.0), frame: Frame::WORLD })),
        (EntityKind::Load, Geometry::Load(LoadGlyph::Line { path: Path3::Polyline(Polyline3 { points: vec![Vec3::new(0.0, 0.0, 1.0), Vec3::new(4.0, 0.0, 1.0)] }), direction: Vec3::new(0.0, 0.0, -1.0), profile: LoadProfile::Linear { start: Quantity::new(QuantityKind::ForcePerLength, 5.0e3), end: Quantity::new(QuantityKind::ForcePerLength, 20.0e3) }, case: case() })),
        (EntityKind::Load, Geometry::Load(LoadGlyph::Area { polygon: ring3(), direction: Vec3::new(0.0, 0.0, -1.0), intensity: Quantity::new(QuantityKind::Pressure, 3.0e3), case: case() })),
        (EntityKind::Load, Geometry::Load(LoadGlyph::Pressure { face: r("m"), zones: vec![PressureZone { polygon: ring3(), value: Quantity::new(QuantityKind::Pressure, -800.0), label: copy("A", "A") }], case: case() })),
        (EntityKind::Load, Geometry::Load(LoadGlyph::Inertial { origin: Vec3::new(0.0, 0.0, 0.0), direction: Vec3::new(1.0, 0.0, 0.0), masses: vec![PointMass { at: Vec3::new(0.0, 0.0, 3.0), mass: Quantity::new(QuantityKind::Mass, 1.0e4) }], case: case() })),
        (EntityKind::Load, Geometry::Load(LoadGlyph::Moment { at: Vec3::new(0.0, 0.0, 0.0), axis: Vec3::new(0.0, 1.0, 0.0), magnitude: Quantity::new(QuantityKind::Moment, 1.0e4), case: case() })),
        (EntityKind::Load, Geometry::Load(LoadGlyph::Explosion { centre: Vec3::new(0.0, 0.0, 0.0), pressure: Quantity::new(QuantityKind::Pressure, 1.0e4), shape: PressureWave::Spherical, case: case() })),
        (EntityKind::Load, Geometry::Load(LoadGlyph::Impact { at: Vec3::new(0.0, 0.0, 0.0), direction: Vec3::new(1.0, 0.0, 0.0), energy_or_force: force(1.0e5), case: case() })),
        (EntityKind::Surface, Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Arc { arc: Arc { centre: Vec3::new(0.0, 0.0, 0.0), normal: Vec3::new(0.0, 1.0, 0.0), start: Vec3::new(2.0, 0.0, 0.0), sweep: 1.0 }, width: 1.0 } })),
        (EntityKind::Surface, Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: ring3() } })),
    ];
    for (kind, g) in geoms {
        let role = if matches!(kind, EntityKind::Load) { MaterialRole::Load { kind: LoadKind::Variable } } else { MaterialRole::Neutral };
        let layer = match kind { EntityKind::Load => "loads", EntityKind::Support => "supports", EntityKind::Fastener => "fasteners", _ => "members" };
        let name = format!("{kind:?}");
        let out = lowered(&scene_of(vec![entity(kind, "x", g, role, None, layer)]), &Viewport3dWindowConfig::default(), &[]);
        assert!(!out.instances.is_empty(), "{name} lowered to nothing");
        assert!(out.meshes.iter().all(|m| m.get("data").and_then(|d| d.get("indices")).and_then(Value::as_array).is_some_and(|i| !i.is_empty())), "{name} empty mesh");
    }
}

fn golden(key: &str) -> Vec<Value> {
    let oracle: Value = semio_framework_pack_json::parse(DYNAMICS, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("dynamics golden");
    oracle.get(key).and_then(Value::as_array).expect("golden rows").clone()
}

fn f(v: &Value) -> f64 {
    v.as_f64().expect("number")
}

fn row(v: &Value, i: usize) -> &Value {
    &v.as_array().expect("row")[i]
}

fn key(at: f64, field: Option<&str>, layer: Option<&str>) -> Keyframe {
    Keyframe { at, caption: copy(&format!("k{at}"), &format!("s{at}")), field: field.map(Into::into), layer: layer.map(Into::into) }
}

fn track(id: &str, kind: TrackKind, max: f64, step: f64, keyframes: Vec<Keyframe>) -> ParameterTrack {
    ParameterTrack { id: id.into(), kind, label: copy("Fire duration", "Branddauer"), unit: QuantityKind::Time, min: 0.0, max, step, looped: false, keyframes }
}

fn fire_track() -> ParameterTrack {
    track("fire", TrackKind::Time, 3600.0, 60.0, vec![key(0.0, Some("temp@0"), None), key(1800.0, Some("temp@30"), None), key(3600.0, Some("temp@60"), None)])
}

fn field_beam(id: &str, fields: Vec<ScalarField>, active: &str) -> SceneEntity {
    let mut e = beam(id, 4.0, None);
    e.appearance.channel = ColourChannel::Fields { active: active.into(), fields };
    e.sealed()
}

fn vertex_field(id: &str, unit: QuantityKind, values: Vec<f64>) -> ScalarField {
    ScalarField { id: id.into(), unit, samples: FieldSamples::Vertex { values } }
}

fn station_field(id: &str, unit: QuantityKind, values: &[(f64, f64)]) -> ScalarField {
    ScalarField { id: id.into(), unit, samples: FieldSamples::Station { values: values.iter().map(|(at, value)| StationValue { at: *at, value: *value }).collect() } }
}

fn mode_beam(motion: Motion) -> SceneEntity {
    let mut e = beam("b1", 4.0, None);
    e.appearance.deformation = Some(DeformationOverlay { surface: Vec::new(), motion, load_factor: Some(3.42), kind: DeformationKind::Mode, index: 1, frequency: Some(Quantity::new(QuantityKind::Frequency, 2.5)), participation: 1.0, scale: 10.0, samples: vec![DisplacementSample { at: 0.0, delta: Vec3::new(0.0, 0.0, 0.0) }, DisplacementSample { at: 0.5, delta: Vec3::new(0.0, 0.0, -0.01) }, DisplacementSample { at: 1.0, delta: Vec3::new(0.0, 0.0, 0.0) }] });
    e.sealed()
}

fn at_state(scene: &StructuralScene, tracks: &[ParameterTrack], playback: PlaybackState, config: &Viewport3dWindowConfig) -> LoweredViewport {
    let measure = MeasureState::default();
    let scene = StructuralScene { tracks: tracks.to_vec(), ..scene.clone() };
    lower_dynamic(&scene, &inputs(config, &[], &measure), &playback, &BasicTessellator::default())
}

fn meshes_colours(out: &LoweredViewport) -> Vec<Vec<f64>> {
    out.meshes.iter().filter_map(|m| m.get("data").and_then(|d| d.get("colors")).and_then(Value::as_array).map(|c| c.iter().map(f).collect())).collect()
}

#[test]
fn oscillation_phase_and_ramp_frames_match_the_python_oracle() {
    for r in golden("phase") {
        let state = PlaybackState { playing: true, clock: f(row(&r, 0)), ..PlaybackState::default() };
        assert!((playback::frame(&state, &[], 1.0).phase - f(row(&r, 1))).abs() < 1e-12);
    }
    for r in golden("ramp") {
        let state = PlaybackState { playing: true, clock: f(row(&r, 0)), ..PlaybackState::default() };
        assert!((playback::frame(&state, &[], 1.0).ramp - f(row(&r, 1))).abs() < 1e-12);
    }
}

#[test]
fn track_samples_blend_neighbouring_fields_like_the_oracle() {
    let fire = fire_track();
    for r in golden("blend") {
        let got = playback::sample(&fire, f(row(&r, 0))).fields.expect("fields");
        let want = row(&r, 1);
        assert_eq!(Some(got.0.as_str()), row(want, 0).as_str());
        assert!((got.2 - f(row(want, 2))).abs() < 1e-12);
        if got.2 > 0.0 { assert_eq!(got.1.as_deref(), row(want, 1).as_str()); }
    }
}

#[test]
fn discrete_tracks_snap_to_the_nearest_keyframe_and_manage_layers() {
    let modes = track("modes", TrackKind::Mode, 3.0, 1.0, vec![key(1.0, Some("mode-1"), Some("mode-1")), key(2.0, Some("mode-2"), Some("mode-2")), key(3.0, None, Some("mode-3"))]);
    let sample = playback::sample(&modes, 1.8);
    assert_eq!(sample.fields, Some(("mode-2".into(), None, 0.0)));
    assert_eq!((sample.layer.as_deref(), sample.managed_layers.len()), (Some("mode-2"), 3));
    assert_eq!(playback::sample(&modes, 3.0).fields, None);
}

#[test]
fn transport_reducer_seeks_steps_wraps_and_respects_reduced_motion() {
    let tracks = vec![fire_track(), track("phase", TrackKind::Phase, 1.0, 0.25, Vec::new())];
    let run = |s: &PlaybackState, c: PlaybackCommand| playback::reduce(s, &tracks, 1.0, &c);
    let mut s = run(&PlaybackState::default(), PlaybackCommand::Select(Some("fire".into())));
    s = run(&s, PlaybackCommand::Seek(1234.0));
    assert_eq!(s.value, 1260.0);
    s = run(&s, PlaybackCommand::Step(-30));
    assert_eq!(s.value, 0.0);
    s = run(&s, PlaybackCommand::Play);
    assert!(s.playing);
    s = run(&s, PlaybackCommand::Tick(4.0));
    assert!((s.value - 1800.0).abs() < 1e-9 && s.playing && (s.clock - 4.0).abs() < 1e-12);
    s = run(&s, PlaybackCommand::Tick(10.0));
    assert_eq!((s.value, s.playing), (3600.0, false));
    s = run(&run(&s, PlaybackCommand::Select(Some("phase".into()))), PlaybackCommand::Step(5));
    assert!((s.value - 0.25).abs() < 1e-12);
    assert_eq!(run(&s, PlaybackCommand::Select(Some("nope".into()))).track, None);
    let calm = PlaybackState { track: Some("fire".into()), ..PlaybackState::default() }.with_reduced_motion(true);
    let pressed = run(&run(&calm, PlaybackCommand::Play), PlaybackCommand::Toggle);
    assert!(!pressed.playing);
    assert_eq!(run(&PlaybackState { playing: true, ..calm.clone() }, PlaybackCommand::Tick(1.0)).clock, 0.0);
    assert_eq!(run(&calm, PlaybackCommand::Seek(600.0)).value, 600.0);
    assert_eq!(playback::command_of(playback::ACTION_STEP, None, Some(-1.0)), Some(PlaybackCommand::Step(-1)));
    assert_eq!(playback::command_of(playback::ACTION_SELECT_TRACK, Some(""), None), Some(PlaybackCommand::Select(None)));
}

#[test]
fn mode_shape_frames_at_fixed_clock_match_the_oracle_depth() {
    let scene = scene_of(vec![mode_beam(Motion::Harmonic)]);
    let deepest = |clock: f64| {
        let out = at_state(&scene, &[], PlaybackState { playing: true, clock, ..PlaybackState::default() }, &Viewport3dWindowConfig::default());
        out.instances.iter().map(|i| num(i, "position", 2)).fold(f64::MAX, f64::min)
    };
    for r in golden("modeDepth") {
        let expected = f(row(&r, 1));
        assert!((deepest(f(row(&r, 0))) - expected.min(0.0)).abs() < 1e-5, "clock {}", f(row(&r, 0)));
    }
    let amplified = Viewport3dWindowConfig { deformation_scale: 2.0, ..Viewport3dWindowConfig::default() };
    let peak = at_state(&scene, &[], PlaybackState { playing: true, clock: 0.5, ..PlaybackState::default() }, &amplified);
    assert!((peak.instances.iter().map(|i| num(i, "position", 2)).fold(f64::MAX, f64::min) + 0.2).abs() < 1e-5);
}

#[test]
fn reduced_motion_freezes_harmonic_modes_at_the_manual_phase() {
    let scene = scene_of(vec![mode_beam(Motion::Harmonic)]);
    let deepest = |state: PlaybackState| at_state(&scene, &[], state, &Viewport3dWindowConfig::default()).instances.iter().map(|i| num(i, "position", 2)).fold(f64::MAX, f64::min);
    let running = PlaybackState { playing: true, clock: 0.25, ..PlaybackState::default() };
    assert!((deepest(running.clone()) + 0.0707106).abs() < 1e-5);
    assert!((deepest(running.with_reduced_motion(true)) + 0.1).abs() < 1e-9);
}

#[test]
fn ramp_motion_grows_from_rest_and_static_motion_ignores_the_clock() {
    let deepest = |motion, clock| {
        let scene = scene_of(vec![mode_beam(motion)]);
        at_state(&scene, &[], PlaybackState { playing: true, clock, ..PlaybackState::default() }, &Viewport3dWindowConfig::default()).instances.iter().map(|i| num(i, "position", 2)).fold(f64::MAX, f64::min)
    };
    assert!((deepest(Motion::Ramp, 1.0) + 0.05).abs() < 1e-9 && (deepest(Motion::Ramp, 2.0) + 0.1).abs() < 1e-9);
    assert!((deepest(Motion::Static, 0.3) + 0.1).abs() < 1e-9);
}

#[test]
fn inverse_distance_surface_displacement_matches_the_oracle() {
    let samples = vec![
        SurfaceSample { at: Vec2::new(0.0, 0.0), delta: Vec3::new(0.0, 0.0, 0.0) },
        SurfaceSample { at: Vec2::new(2.0, 0.0), delta: Vec3::new(0.0, 0.0, 0.02) },
        SurfaceSample { at: Vec2::new(0.0, 2.0), delta: Vec3::new(0.0, 0.0, -0.01) },
    ];
    for r in golden("idw") {
        let at = row(&r, 0).as_array().expect("point");
        let got = surface_delta(&samples, [f(&at[0]), f(&at[1])]);
        for k in 0..3 { assert!((got[k] - f(row(row(&r, 1), k))).abs() < 1e-12); }
    }
}

#[test]
fn grid_node_order_matches_the_oracle() {
    let want: Vec<usize> = golden("gridNodes3x3").iter().map(|v| f(v) as usize).collect();
    let got: Vec<usize> = (0..want.len()).map(|v| fields::grid_node(v, 3, 3)).collect();
    assert_eq!(got, want);
}

#[test]
fn vertex_fields_blend_and_resample_like_the_oracle() {
    let oracle: Value = semio_framework_pack_json::parse(DYNAMICS, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("golden");
    let blend = oracle.get("vertexBlend").expect("blend");
    let list = |k: &str| blend.get(k).and_then(Value::as_array).unwrap().iter().map(f).collect::<Vec<_>>();
    let (from, to, want) = (list("from"), list("to"), list("values"));
    let (a, b) = (FieldSamples::Vertex { values: from }, FieldSamples::Vertex { values: to });
    let field = fields::ResolvedField { id: "x", unit: QuantityKind::Length, from: &a, to: Some(&b), weight: 0.25 };
    for (i, w) in want.iter().enumerate() { assert!((field.value(Site { vertex: i, vertices: 4, station: None, instance: 0 }) - w).abs() < 1e-12); }
    assert!((field.value(Site { vertex: 3, vertices: 7, station: None, instance: 0 }) - (1.5 + 10.0 * 0.25)).abs() < 1e-12);
}

#[test]
fn station_fields_bake_a_ramp_along_the_member_with_a_matching_legend() {
    let scene = scene_of(vec![field_beam("b1", vec![station_field("moment", QuantityKind::Moment, &[(0.0, 0.0), (1.0, 100_000.0)])], "moment")]);
    let out = at_state(&scene, &[], PlaybackState::default(), &Viewport3dWindowConfig::default());
    let colours = meshes_colours(&out);
    assert_eq!(colours.len(), 1);
    assert!(colours[0].chunks(3).any(|c| c != &colours[0][0..3]), "ramp must vary along the member");
    let field = out.scalar_field.expect("legend field");
    let (lo, hi) = field.values.iter().flatten().fold((f64::MAX, f64::MIN), |a, v| (a.0.min(*v), a.1.max(*v)));
    assert!(lo.abs() < 1e-9 && (hi - 100.0).abs() < 1e-9);
    assert_eq!((field.range.min, field.range.max, field.legend.unit.as_deref()), (0.0, 100.0, Some("kNm")));
    assert_eq!((field.legend.title.en.as_str(), field.legend.title.de.as_str()), ("Moment", "Moment"));
}

#[test]
fn the_toolbar_choice_switches_the_active_field_and_the_legend() {
    let entity = field_beam("p1", vec![vertex_field("settlement", QuantityKind::Length, vec![0.0, 0.02]), vertex_field("stress", QuantityKind::Stress, vec![0.0, 2.0e8])], "settlement");
    let scene = scene_of(vec![entity]);
    assert_eq!(fields::available(&scene).iter().map(|c| c.title.de.clone()).collect::<Vec<_>>(), vec!["Setzung", "Spannung"]);
    let legend = |active: Option<&str>| {
        let config = Viewport3dWindowConfig { active_field: active.map(Into::into), ..Viewport3dWindowConfig::default() };
        let field = at_state(&scene, &[], PlaybackState::default(), &config).scalar_field.expect("field");
        (field.legend.title.en, field.legend.unit, field.range.max)
    };
    let (title, unit, max) = legend(None);
    assert_eq!((title.as_str(), unit.as_deref()), ("Settlement", Some("m")));
    assert!((max - 0.02).abs() < 1e-12);
    let (title, unit, max) = legend(Some("stress"));
    assert_eq!((title.as_str(), unit.as_deref()), ("Stress", Some("MPa")));
    assert!((max - 200.0).abs() < 1e-9);
    assert_eq!(legend(Some("missing")).0, "Settlement");
}

#[test]
fn scrubbing_a_track_blends_fields_over_a_stable_legend_domain() {
    let temps = vec![vertex_field("temp@0", QuantityKind::Temperature, vec![20.0; 2]), vertex_field("temp@30", QuantityKind::Temperature, vec![600.0, 800.0]), vertex_field("temp@60", QuantityKind::Temperature, vec![900.0, 1000.0])];
    let scene = scene_of(vec![field_beam("b1", temps, "temp@0")]);
    let tracks = vec![fire_track()];
    let frame = |value: f64| at_state(&scene, &tracks, PlaybackState { track: Some("fire".into()), value, ..PlaybackState::default() }, &Viewport3dWindowConfig::default());
    let early = frame(0.0);
    let mid = frame(900.0);
    let late = frame(3600.0);
    let range = |o: &LoweredViewport| o.scalar_field.as_ref().map(|f| (f.range.min, f.range.max)).unwrap();
    assert_eq!(range(&early), (20.0, 1000.0));
    assert_eq!(range(&mid), range(&late));
    let first = mid.scalar_field.as_ref().unwrap().values[0].unwrap();
    assert!((first - (20.0 + (600.0 - 20.0) * 0.5)).abs() < 1e-9);
    assert_eq!(mid.readout.as_ref().map(|r| (r.en.as_str(), r.de.as_str())), Some(("Fire duration: 15 min", "Branddauer: 15 min")));
    assert!(mid.description.contains("Fire duration: 15 min"));
    assert_ne!(meshes_colours(&early), meshes_colours(&late));
}

#[test]
fn layer_keyframes_show_one_mode_layer_at_a_time() {
    let mut a = beam("m1", 4.0, None);
    a.layer = "mode-1".into();
    let mut b = beam("m2", 4.0, None);
    b.layer = "mode-2".into();
    let mut scene = scene_of(vec![a.sealed(), b.sealed()]);
    scene.layers = ["mode-1", "mode-2"].iter().map(|id| Layer { id: (*id).into(), name: copy(id, id), visible: true, locked: false }).collect();
    let modes = vec![track("modes", TrackKind::Mode, 2.0, 1.0, vec![key(1.0, None, Some("mode-1")), key(2.0, None, Some("mode-2"))])];
    let shown = |value: f64| at_state(&scene, &modes, PlaybackState { track: Some("modes".into()), value, ..PlaybackState::default() }, &Viewport3dWindowConfig::default()).instances.iter().filter_map(|i| i.get("id").and_then(Value::as_str).map(str::to_string)).collect::<Vec<_>>();
    assert_eq!((shown(1.0).len(), shown(2.0).len()), (1, 1));
    assert_ne!(shown(1.0), shown(2.0));
}

#[test]
fn instance_fields_tint_each_pattern_instance() {
    let mut pattern = bolts("bolts", 2, 1);
    pattern.appearance.channel = ColourChannel::Fields { active: "util".into(), fields: vec![ScalarField { id: "util".into(), unit: QuantityKind::Dimensionless, samples: FieldSamples::Instance { values: vec![0.1, 0.9] } }] };
    let out = at_state(&scene_of(vec![pattern.sealed()]), &[], PlaybackState::default(), &Viewport3dWindowConfig::default());
    let ids: std::collections::BTreeSet<&str> = out.instances.iter().filter_map(|i| i.get("meshId").and_then(Value::as_str)).collect();
    assert!(ids.len() >= 3, "head and shaft meshes split by instance value: {ids:?}");
}

fn mode_plate(surface: Vec<SurfaceSample>) -> SceneEntity {
    let square = Polygon2::solid(vec![Vec2::new(0.0, 0.0), Vec2::new(2.0, 0.0), Vec2::new(2.0, 2.0), Vec2::new(0.0, 2.0)]);
    let mut e = entity(EntityKind::Plate, "p1", Geometry::Plate(PlateGeometry { frame: Frame::WORLD, outline: square, thickness: 0.02, offset: 0.0, stiffeners: Vec::new() }), MaterialRole::Steel, None, "members");
    e.appearance.deformation = Some(DeformationOverlay { surface, motion: Motion::Harmonic, load_factor: Some(2.0), kind: DeformationKind::Mode, index: 2, frequency: None, participation: 1.0, scale: 5.0, samples: Vec::new() });
    e.sealed()
}

#[test]
fn plate_mode_samples_deform_a_surface_next_to_the_ghost() {
    let surface = vec![SurfaceSample { at: Vec2::new(1.0, 1.0), delta: Vec3::new(0.0, 0.0, 0.01) }];
    let tess = BasicTessellator::default();
    let scene = scene_of(vec![mode_plate(surface)]);
    let ctx = PartContext { scene: &scene, lod: Lod::FULL, glyph: 0.1, deformation_scale: 1.0, frame: playback::Frame { phase: 1.0, ramp: 1.0 } };
    let parts = tess.parts(&scene.entities[0], &ctx);
    assert_eq!((parts.len(), parts[0].ghost, parts[1].ghost), (2, true, false));
    let top = (0..parts[1].mesh.vertex_count()).map(|v| parts[1].mesh.point(v)[2]).fold(f64::MIN, f64::max);
    assert!((top - (0.01 + 0.05)).abs() < 1e-9, "single sample moves every node by delta * scale: {top}");
    let rest = PartContext { frame: playback::Frame { phase: 0.0, ramp: 1.0 }, ..ctx };
    let flat = tess.parts(&scene.entities[0], &rest);
    assert!((0..flat[1].mesh.vertex_count()).all(|v| (flat[1].mesh.point(v)[2] - 0.01).abs() < 1e-9));
    assert!(flat[1].mesh.indices.len() >= 6 * 6 * 6 * 2);
}

#[test]
fn shell_mode_samples_use_physical_surface_coordinates() {
    let grid = PointGrid { rows: 3, columns: 3, points: (0..9).map(|k| Vec3::new((k % 3) as f64, (k / 3) as f64, 0.0)).collect() };
    let mut shell = entity(EntityKind::Shell, "s1", Geometry::Shell(ShellGeometry { shape: ShellShape::Patch { grid }, thickness: 0.1 }), MaterialRole::Concrete, None, "members");
    shell.appearance.deformation = Some(DeformationOverlay { surface: vec![SurfaceSample { at: Vec2::new(1.0, 1.0), delta: Vec3::new(0.0, 0.0, 0.1) }], motion: Motion::Static, load_factor: None, kind: DeformationKind::Mode, index: 1, frequency: None, participation: 1.0, scale: 1.0, samples: Vec::new() });
    let scene = scene_of(vec![shell.sealed()]);
    let tess = BasicTessellator::default();
    let ctx = PartContext { scene: &scene, lod: Lod::FULL, glyph: 0.1, deformation_scale: 1.0, frame: playback::Frame::default() };
    let parts = tess.parts(&scene.entities[0], &ctx);
    assert_eq!((parts.len(), parts[0].ghost, parts[1].grid), (2, true, Some((3, 3))));
    let lifted = (0..parts[1].mesh.vertex_count()).map(|v| parts[1].mesh.point(v)[2]).fold(f64::MIN, f64::max);
    assert!((lifted - 0.1).abs() < 1e-12);
    let (along, across) = surface_coordinates(&(0..9).map(|k| [(k % 3) as f64 * 2.0, (k / 3) as f64, 0.0]).collect::<Vec<_>>(), 3, 3);
    assert_eq!((along, across), (vec![0.0, 2.0, 4.0], vec![0.0, 1.0, 2.0]));
}

#[test]
fn mode_labels_carry_frequency_and_alpha_cr_in_both_languages() {
    let scene = scene_of(vec![mode_beam(Motion::Harmonic)]);
    let items = lowered(&scene, &Viewport3dWindowConfig::default(), &[]).annotations.map(|l| l.items.iter().map(|a| (a.id().to_string(), a.text().clone())).collect::<Vec<_>>()).unwrap_or_default();
    let label = &items.iter().find(|(id, _)| id == "mode:b1").expect("mode label").1;
    assert_eq!(label.en, "Mode 1 · f = 2.5 Hz · αcr = 3.42 · ×10");
    assert_eq!(label.de, "Eigenform 1 · f = 2,5 Hz · αcr = 3,42 · ×10");
}

#[test]
fn field_selection_is_stable_without_a_track() {
    assert_eq!(fields::selection(Some("a"), None), FieldSelection { primary: Some("a".into()), secondary: None, weight: 0.0, domain_ids: vec!["a".into()] });
    assert_eq!(fields::selection(None, None), FieldSelection::default());
}

#[test]
fn tracks_come_from_the_scene_and_unselected_track_layers_rest_on_their_first_keyframe() {
    let mut a = beam("a", 4.0, None);
    a.layer = "mode-1".into();
    let mut b = beam("b", 4.0, None);
    b.layer = "mode-2".into();
    let mut scene = scene_of(vec![a.sealed(), b.sealed()]);
    scene.layers.extend(["mode-1", "mode-2"].map(|id| Layer { id: id.into(), name: copy(id, id), visible: true, locked: false }));
    scene.tracks = vec![track("mode", TrackKind::Mode, 2.0, 1.0, vec![key(1.0, None, Some("mode-1")), key(2.0, None, Some("mode-2"))])];
    let ids = |out: &LoweredViewport| out.instances.iter().filter_map(|i| i.get("id").and_then(Value::as_str).map(str::to_string)).collect::<Vec<_>>();
    let config = Viewport3dWindowConfig::default();
    let rest = at_state(&scene, &scene.tracks, PlaybackState::default(), &config);
    assert_eq!(ids(&rest).len(), 1);
    let second = at_state(&scene, &scene.tracks, PlaybackState { track: Some("mode".into()), value: 2.0, ..PlaybackState::default() }, &config);
    assert_eq!(ids(&second).len(), 1);
    assert_ne!(ids(&rest), ids(&second));
}

#[test]
fn hatch_ribbons_follow_their_direction_and_stay_inside_the_region() {
    let region = Polygon2::solid(vec![Vec2::new(0.0, 0.0), Vec2::new(4.0, 0.0), Vec2::new(4.0, 1.0), Vec2::new(0.0, 1.0)]);
    let along = hatch_ribbons(&region, Hatch::Horizontal, 0.0, 0.25, 0.02);
    let across = hatch_ribbons(&region, Hatch::Vertical, 0.0, 0.25, 0.02);
    assert!(along.vertex_count() > 0 && across.vertex_count() > 0);
    let extent = |m: &MeshData, axis: usize| (0..m.vertex_count()).map(|i| m.point(i)[axis]).fold((f64::MAX, f64::MIN), |(l, h), v| (l.min(v), h.max(v)));
    assert!(extent(&along, 0).1 - extent(&along, 0).0 > 3.9);
    assert!(extent(&across, 1).1 - extent(&across, 1).0 > 0.9);
    assert!(along.vertex_count() < across.vertex_count());
    for mesh in [&along, &across, &hatch_ribbons(&region, Hatch::Diagonal, 0.0, 0.25, 0.02), &hatch_ribbons(&region, Hatch::CrossDiagonal, 0.0, 0.25, 0.02), &hatch_ribbons(&region, Hatch::Dots, 0.0, 0.25, 0.02)] {
        assert!(mesh.vertex_count() > 0);
        assert!((0..mesh.vertex_count()).all(|i| { let p = mesh.point(i); p[2] == 0.0 && p[0] > -0.02 && p[0] < 4.02 && p[1] > -0.02 && p[1] < 1.02 }));
    }
    let turned = hatch_ribbons(&region, Hatch::Horizontal, std::f64::consts::FRAC_PI_2, 0.25, 0.02);
    assert_eq!(turned.vertex_count(), across.vertex_count());
}

#[test]
fn a_hatch_rule_around_a_hole_leaves_the_hole_clear() {
    let region = Polygon2 { outer: Ring2 { points: vec![Vec2::new(0.0, 0.0), Vec2::new(4.0, 0.0), Vec2::new(4.0, 4.0), Vec2::new(0.0, 4.0)] }, holes: vec![Ring2 { points: vec![Vec2::new(1.0, 1.0), Vec2::new(3.0, 1.0), Vec2::new(3.0, 3.0), Vec2::new(1.0, 3.0)] }] };
    let mesh = hatch_ribbons(&region, Hatch::Horizontal, 0.0, 0.5, 0.02);
    assert!((0..mesh.vertex_count()).all(|i| { let p = mesh.point(i); !(p[0] > 1.05 && p[0] < 2.95 && p[1] > 1.05 && p[1] < 2.95) }));
}

#[test]
fn laminate_layers_show_their_grain_and_boxes_their_hatch_in_3d() {
    let footprint = Polygon2::solid(vec![Vec2::new(0.0, 0.0), Vec2::new(3.0, 0.0), Vec2::new(3.0, 2.0), Vec2::new(0.0, 2.0)]);
    let layer = |grain: f64| GrainLayer { thickness: 0.04, role: MaterialRole::Timber, grain, label: copy("Layer", "Lage") };
    let clt = entity(EntityKind::Solid, "clt", Geometry::Solid(SolidGeometry { shape: SolidShape::Laminate { frame: Frame::WORLD, footprint, layers: vec![layer(0.0), layer(std::f64::consts::FRAC_PI_2), layer(0.0)] } }), MaterialRole::Timber, None, "members");
    let scene = scene_of(vec![clt]);
    let out = lowered(&scene, &Viewport3dWindowConfig::default(), &[]);
    let ink = |out: &LoweredViewport| out.meshes.iter().filter(|m| m.get("id").and_then(Value::as_str).is_some_and(|id| id.starts_with("hatch:"))).count();
    assert_eq!(out.stats.instances, 6);
    assert_eq!(ink(&out), 2);
    let mut hatched = entity(EntityKind::Solid, "block", Geometry::Solid(SolidGeometry { shape: SolidShape::Box { frame: Frame::WORLD, size: Vec3::new(1.0, 1.0, 1.0) } }), MaterialRole::Timber, None, "members");
    assert_eq!(lowered(&scene_of(vec![hatched.clone()]), &Viewport3dWindowConfig::default(), &[]).stats.instances, 1);
    hatched.appearance.hatch = Some(Hatch::Diagonal);
    assert_eq!(lowered(&scene_of(vec![hatched.sealed()]), &Viewport3dWindowConfig::default(), &[]).stats.instances, 2);
}
