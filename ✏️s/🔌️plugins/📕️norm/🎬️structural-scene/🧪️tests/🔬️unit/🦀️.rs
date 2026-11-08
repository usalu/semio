use crate::*;
use value::{FromValue, ToValue};

fn copy(s: &str) -> LocalizedCopy {
    LocalizedCopy::new(s, format!("{s}-de"))
}

fn ibeam() -> SectionShape {
    SectionShape::parametric(ProfileKind::IShape, &[("h", 0.24), ("b", 0.24), ("tw", 0.01), ("tf", 0.017)])
}

fn member(id: &str) -> SceneEntity {
    SceneEntity {
        entity: EntityRef::new(EntityKind::Member, id, format!("members[id={id}]")),
        geometry: Geometry::Member(MemberGeometry {
            axis: Path3::Polyline(Polyline3 { points: vec![Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0)] }),
            frame: Frame::WORLD,
            section: ibeam(),
            section_taper: None,
            start_offset: Vec2::ZERO,
            end_offset: Vec2::ZERO,
            start_release: EndRelease::Rigid,
            end_release: EndRelease::Pinned,
            stations: vec![0.0, 0.5, 1.0],
        }),
        appearance: Appearance::role(MaterialRole::Steel),
        utilisation: Some(UtilisationBinding { check_id: "c1".into(), ratio: 0.64, status: CheckStatus::Pass, per_station: Some(vec![StationValue { at: 0.0, value: 0.1 }, StationValue { at: 1.0, value: 0.64 }]), stale: false }),
        label: Some(copy("B1")),
        accessible_name: copy("Member B1"),
        accessible_detail: copy("I-shape, 4.00 m"),
        layer: "members".into(),
        pick: PickPolicy::Whole,
        handles: vec!["member-b1/length".into()],
        content_hash: String::new(),
    }
    .sealed()
}

fn load() -> SceneEntity {
    let case = LoadCaseRef { id: "g".into(), kind: LoadKind::Permanent, psi: None };
    SceneEntity {
        entity: EntityRef::new(EntityKind::Load, "l1", "loads[id=l1]"),
        geometry: Geometry::Load(LoadGlyph::Line {
            path: Path3::Polyline(Polyline3 { points: vec![Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0)] }),
            direction: Vec3::new(0.0, 0.0, -1.0),
            profile: LoadProfile::Uniform { value: Quantity::new(QuantityKind::ForcePerLength, 12_500.0) },
            case,
        }),
        appearance: Appearance::role(MaterialRole::Load { kind: LoadKind::Permanent }),
        utilisation: None,
        label: None,
        accessible_name: copy("Dead load"),
        accessible_detail: copy("12.5 kN/m"),
        layer: "loads".into(),
        pick: PickPolicy::Whole,
        handles: vec![],
        content_hash: String::new(),
    }
    .sealed()
}

pub(crate) fn scene() -> StructuralScene {
    StructuralScene {
        schema: SCENE_SCHEMA_ID.into(),
        revision: SceneRevision { document_seq: 3, report_seq: 2 },
        bounds: Aabb::new(Vec3::new(-0.12, -0.12, -0.12), Vec3::new(4.0, 0.12, 0.12)),
        entities: vec![member("b1"), load()],
        annotations: vec![
            Annotation::Dimension { id: "d1".into(), from: Vec3::ZERO, to: Vec3::new(4.0, 0.0, 0.0), offset: Vec3::new(0.0, 0.0, -0.5), value: Quantity::new(QuantityKind::Length, 4.0), binding: None },
            Annotation::Clause { id: "k1".into(), anchor: EntityRef::new(EntityKind::Member, "b1", "members[id=b1]"), clause: ClauseId { family: "X".into(), part: "1".into(), section: "6.2".into() } },
        ],
        legends: vec![Legend { id: "u".into(), title: copy("Utilisation"), scale: LegendScale::Utilisation { map: ColourMapRef { id: "util".into(), min: 0.0, max: 1.0 } } }],
        layers: vec![Layer { id: "members".into(), name: copy("Members"), visible: true, locked: false }, Layer { id: "loads".into(), name: copy("Loads"), visible: true, locked: false }],
        tracks: Vec::new(),
        owners: Vec::new(),
        description: copy("A 4 m beam under a line load."),
    }
}

fn rules(r: Result<(), Vec<Violation>>) -> Vec<String> {
    r.err().unwrap_or_default().into_iter().map(|v| format!("{}:{}", v.path, v.rule)).collect()
}

#[test]
fn valid_scene_validates_and_round_trips_through_the_value_codec() {
    let scene = scene();
    assert_eq!(scene.validate(), Ok(()));
    let back = StructuralScene::from_value(scene.to_value()).expect("decode");
    assert_eq!(back, scene);
    assert_eq!(canonical_json(&back), canonical_json(&scene));
}

#[test]
fn canonical_json_is_key_sorted_compact_and_matches_the_third_party_parser() {
    let text = canonical_json(&scene());
    let parsed: serde_json::Value = serde_json::from_str(&text).expect("valid json");
    assert_eq!(serde_json::to_string(&parsed).expect("reserialise"), text);
    assert!(text.starts_with("{\"annotations\":["), "{}", &text[..40]);
    assert_eq!(canonical_json(&Vec3::new(1.0, -0.0, 0.5)), "{\"x\":1.0,\"y\":0.0,\"z\":0.5}");
    assert_eq!(canonical_json(&LocalizedCopy::new("a\"\n", "ü")), "{\"de\":\"ü\",\"en\":\"a\\\"\\n\"}");
}

#[test]
fn variant_wire_shape_is_kind_and_data_with_camel_case() {
    let json: serde_json::Value = serde_json::from_str(&canonical_json(&Geometry::Group { members: vec![] })).expect("json");
    assert_eq!(json["kind"], "group");
    let fixed: serde_json::Value = serde_json::from_str(&canonical_json(&SupportKind::Fixed)).expect("json");
    assert_eq!(fixed["kind"], "fixed");
    let scene: serde_json::Value = serde_json::from_str(&canonical_json(&scene())).expect("json");
    assert_eq!(scene["entities"][0]["accessibleName"]["en"], "Member B1");
    assert_eq!(scene["entities"][0]["geometry"]["kind"], "member");
    assert_eq!(scene["entities"][0]["geometry"]["data"]["startRelease"], "rigid");
    assert_eq!(scene["entities"][1]["geometry"]["data"]["profile"]["kind"], "uniform");
}

#[test]
fn golden_scene_matches_committed_bytes() {
    crate::projection_kit::test_support::assert_golden(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧫️fixtures/🏛️scene/🔣️.json"), &canonical_json(&scene()));
}

#[test]
fn section_outline_areas_match_closed_forms() {
    let ishape = (2.0 * 0.24 * 0.017) + (0.24 - 2.0 * 0.017) * 0.01;
    assert!((ibeam().area().unwrap() - ishape).abs() < 1e-12);
    let rect = SectionShape::parametric(ProfileKind::Rect, &[("h", 0.5), ("b", 0.3)]);
    assert!((rect.area().unwrap() - 0.15).abs() < 1e-12);
    let rhs = SectionShape::parametric(ProfileKind::Rhs, &[("h", 0.2), ("b", 0.1), ("t", 0.01)]);
    assert!((rhs.area().unwrap() - (0.2 * 0.1 - 0.18 * 0.08)).abs() < 1e-12);
    let circle = SectionShape::parametric(ProfileKind::Circle, &[("d", 1.0)]);
    let n = 48.0_f64;
    assert!((circle.area().unwrap() - n / 2.0 * 0.25 * (std::f64::consts::TAU / n).sin()).abs() < 1e-12);
    let angle = SectionShape::parametric(ProfileKind::Angle, &[("h", 0.1), ("b", 0.1), ("t", 0.01)]);
    assert!((angle.area().unwrap() - (0.1 * 0.01 * 2.0 - 0.01 * 0.01)).abs() < 1e-12);
    let tee = SectionShape::parametric(ProfileKind::Tee, &[("h", 0.2), ("b", 0.2), ("tw", 0.01), ("tf", 0.02)]);
    assert!((tee.area().unwrap() - (0.2 * 0.02 + 0.18 * 0.01)).abs() < 1e-12);
    let channel = SectionShape::parametric(ProfileKind::Channel, &[("h", 0.2), ("b", 0.1), ("tw", 0.01), ("tf", 0.02)]);
    assert!((channel.area().unwrap() - (2.0 * 0.1 * 0.02 + 0.16 * 0.01)).abs() < 1e-12);
}

#[test]
fn every_profile_outline_is_centred_and_valid() {
    let cases = [
        (ProfileKind::Cee, vec![("h", 0.2), ("b", 0.07), ("t", 0.002), ("c", 0.02)]),
        (ProfileKind::Zed, vec![("h", 0.2), ("b", 0.07), ("t", 0.002), ("c", 0.02)]),
        (ProfileKind::Chs, vec![("d", 0.1), ("t", 0.005)]),
    ];
    for (profile, params) in cases {
        let shape = SectionShape::parametric(profile, &params);
        let regions = shape.outline().expect("outline");
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].validate(), Ok(()));
        let xs: Vec<f64> = regions[0].outer.points.iter().map(|p| p.x).collect();
        let (lo, hi) = xs.iter().fold((f64::MAX, f64::MIN), |a, &x| (a.0.min(x), a.1.max(x)));
        assert!((lo + hi).abs() < 1e-12);
    }
    let cee = SectionShape::parametric(ProfileKind::Cee, &[("h", 0.2), ("b", 0.07), ("t", 0.002), ("c", 0.02)]);
    let expected = 0.2 * 0.002 + 2.0 * (0.07 - 0.002) * 0.002 + 2.0 * (0.02 - 0.002) * 0.002;
    assert!((cee.area().unwrap() - expected).abs() < 1e-12);
}

#[test]
fn composite_and_thin_walled_outlines() {
    let composite = SectionShape::Composite {
        parts: vec![
            CompositePart { shape: Box::new(ibeam()), role: MaterialRole::Steel, offset: Vec2::ZERO },
            CompositePart { shape: Box::new(SectionShape::parametric(ProfileKind::Rect, &[("h", 0.1), ("b", 0.5)])), role: MaterialRole::Concrete, offset: Vec2::new(0.0, 0.17) },
        ],
        rebar: vec![RebarLayout { bar_diameter: 0.012, positions: vec![Vec2::new(0.0, 0.2)], cover: 0.03 }],
    };
    assert_eq!(composite.validate(), Ok(()));
    assert_eq!(composite.outline().unwrap().len(), 2);
    let thin = SectionShape::ThinWalled { centreline: Polyline2 { points: vec![Vec2::ZERO, Vec2::new(0.1, 0.0), Vec2::new(0.1, 0.05)] }, thickness: vec![0.002, 0.002] };
    assert!((thin.area().unwrap() - 0.15 * 0.002).abs() < 1e-12);
}

#[test]
fn invalid_values_report_path_addressed_rules() {
    assert_eq!(rules(Vec3::new(f64::NAN, 0.0, 0.0).validate()), ["$:finite"]);
    assert_eq!(rules(Frame { origin: Vec3::ZERO, x: Vec3::X, y: Vec3::X, z: Vec3::Z }.validate()).contains(&"$:orthogonal".to_owned()), true);
    assert_eq!(rules(Frame { origin: Vec3::ZERO, x: Vec3::X, y: Vec3::Z, z: Vec3::Y }.validate()), ["$:right-handed"]);
    assert_eq!(rules(Rotation { x: 0.0, y: 0.0, z: 0.0, w: 2.0 }.validate()), ["$:unit-quaternion"]);
    assert_eq!(rules(Aabb::new(Vec3::X, Vec3::ZERO).validate()), ["$:ordered"]);
    assert_eq!(rules(Polyline3 { points: vec![Vec3::ZERO] }.validate()), ["$:at-least-2-points"]);
    assert_eq!(rules(LocalizedCopy::new("x", " ").validate()), ["$.de:non-empty"]);
    assert_eq!(rules(SectionShape::parametric(ProfileKind::IShape, &[("h", 0.2)]).validate()), ["$.params:required-params-present"]);
    assert_eq!(rules(SectionShape::parametric(ProfileKind::Rect, &[("h", -0.2), ("b", 0.1)]).validate()), ["$.params[0].value:positive"]);
    assert_eq!(rules(SectionShape::parametric(ProfileKind::IShape, &[("h", 0.2), ("b", 0.1), ("tw", 0.2), ("tf", 0.01)]).validate()), ["$.params:consistent-dimensions"]);
    assert!(SectionShape::parametric(ProfileKind::IShape, &[("h", 0.2)]).outline().is_err());
    assert_eq!(rules(CombinationFactors { psi0: 0.5, psi1: 0.7, psi2: 0.3 }.validate()), ["$:non-increasing"]);
    assert_eq!(rules(SupportKind::Spring { stiffness: vec![] }.validate()), ["$.stiffness:six-components"]);
    assert_eq!(rules(Annotation::Leader { id: "a".into(), anchor: Vec3::ZERO, text: copy("t"), offset_px: vec![1] }.validate()), ["$.offsetPx:two-components"]);
    assert_eq!(rules(PointGrid { rows: 2, columns: 2, points: vec![Vec3::ZERO] }.validate()), ["$:rows-times-columns-points"]);
}

#[test]
fn scene_level_rules() {
    let mut s = scene();
    s.schema = "other".into();
    s.entities.push(member("b1"));
    s.entities[0].layer = "ghost".into();
    s.entities[1].accessible_name = LocalizedCopy::new("", "x");
    s.entities[1].appearance.opacity = 2.0;
    let found = rules(s.validate());
    for expected in ["$.schema:schema-id", "$.entities:unique-entity-ids", "$.entities:layer-exists", "$.entities[1].accessibleName.en:non-empty", "$.entities[1].appearance.opacity:unit-interval"] {
        assert!(found.contains(&expected.to_owned()), "{expected} in {found:?}");
    }
}

#[test]
fn content_hash_detects_tampering_and_ignores_labels() {
    let mut m = member("b1");
    assert_eq!(m.content_hash, m.compute_hash());
    assert_eq!(m.content_hash.len(), 16);
    m.label = Some(copy("renamed"));
    assert_eq!(m.validate(), Ok(()));
    m.appearance.style = StyleState::Selected;
    assert_eq!(rules(m.validate()), ["$.contentHash:matches-content"]);
    assert_ne!(m.clone().sealed().content_hash, member("b1").content_hash);
}

#[test]
fn every_geometry_kind_validates_and_matches_its_entity_kind() {
    let plate = Geometry::Plate(PlateGeometry { frame: Frame::WORLD, outline: Polygon2::solid(vec![Vec2::ZERO, Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0)]), thickness: 0.02, offset: 0.0, stiffeners: vec![] });
    let shell = Geometry::Shell(ShellGeometry { shape: ShellShape::Revolved { origin: Vec3::ZERO, axis: Vec3::Z, generatrix: Polyline2 { points: vec![Vec2::new(2.0, 0.0), Vec2::new(2.0, 5.0)] }, sweep: std::f64::consts::TAU }, thickness: 0.01 });
    let solid = Geometry::Solid(SolidGeometry { shape: SolidShape::Box { frame: Frame::WORLD, size: Vec3::new(1.0, 1.0, 1.0) } });
    let fastener = Geometry::Fastener(FastenerGeometry { kind: FastenerKind::Bolt { class: "8.8".into(), preload: None }, axis: Segment3 { from: Vec3::ZERO, to: Vec3::new(0.0, 0.0, 0.05) }, diameter: 0.02, head: HeadShape::Hex, pattern: Some(Pattern::Grid { nx: 2, ny: 2, sx: 0.08, sy: 0.08 }), hole_clearance: Some(0.002) });
    let support = Geometry::Support(SupportGeometry { kind: SupportKind::Roller { normal: Vec3::Z }, at: Vec3::ZERO, frame: Frame::WORLD });
    let surface = Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Arc { arc: Arc { centre: Vec3::ZERO, normal: Vec3::Y, start: Vec3::X, sweep: 1.0 }, width: 1.0 } });
    let group = Geometry::Group { members: vec![EntityRef::new(EntityKind::Member, "a", "members[id=a]")] };
    for (g, k) in [(plate, EntityKind::Plate), (shell, EntityKind::Shell), (solid, EntityKind::Solid), (fastener, EntityKind::Fastener), (support, EntityKind::Support), (surface, EntityKind::Surface), (group, EntityKind::Group)] {
        assert_eq!(g.validate(), Ok(()));
        assert_eq!(g.kind(), k);
        assert_eq!(Geometry::from_value(g.to_value()).expect("decode"), g);
    }
}

#[test]
fn delta_validates_and_round_trips() {
    let delta = SceneDelta {
        base: SceneRevision { document_seq: 3, report_seq: 2 },
        revision: SceneRevision { document_seq: 4, report_seq: 2 },
        bounds: None,
        entities: vec![EntityPatch::Replace { entity: member("b1") }, EntityPatch::Add { index: 1, entity: member("b2") }, EntityPatch::Remove { id: "l1".into() }],
        annotations: vec![AnnotationPatch::Remove { id: "d1".into() }],
        legends: None,
        layers: None,
        tracks: None,
        owners: None,
        description: None,
    };
    assert_eq!(delta.validate(), Ok(()));
    assert!(!delta.is_empty());
    assert_eq!(SceneDelta::from_value(delta.to_value()).expect("decode"), delta);
    let mut bad = delta.clone();
    bad.entities.push(EntityPatch::Remove { id: "b1".into() });
    bad.revision = SceneRevision { document_seq: 1, report_seq: 2 };
    assert_eq!(rules(bad.validate()), ["$.revision:revision-not-before-base", "$.entities:one-patch-per-entity"]);
    assert!(SceneDelta { entities: vec![], annotations: vec![], ..delta }.is_empty() == false);
}

#[test]
fn twin_files_declare_every_rust_type() {
    let schema = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🧬️schema");
    let read = |name: &str| std::fs::read_to_string(schema.join(name)).expect(name);
    let (proto, graphql, ts) = (read("🛰️.proto"), read("🔗️.graphql"), read("🟦️.ts"));
    let mut names = Vec::new();
    for module in ["📐️geometry", "🎨️appearance", "🧱️section-shapes", "🏹️load-glyphs", "🧩️entities", "🏷️annotations", "🔺️scene-diff", "✅️validation"] {
        let source = std::fs::read_to_string(schema.join(module).join("🦀️.rs")).expect(module);
        names.extend(source.lines().filter_map(|l| l.trim().strip_prefix("pub struct ").or_else(|| l.trim().strip_prefix("pub enum "))).map(|l| l.trim_end_matches(" {").to_owned()));
    }
    assert!(names.len() >= 80, "{}", names.len());
    for name in names {
        assert!(proto.contains(&format!("message {name} {{")) || proto.contains(&format!("enum {name} {{")), "proto {name}");
        assert!(graphql.contains(&format!("type {name} {{")) || graphql.contains(&format!("enum {name} {{")) || graphql.contains(&format!("union {name} =")), "graphql {name}");
        assert!(ts.contains(&format!("interface {name} {{")) || ts.contains(&format!("type {name} =")), "ts {name}");
    }
    assert!(ts.contains("export * from \"./📈️diagram2d/🟦️.ts\"") && ts.contains("export * from \"./🕹️handles/🟦️.ts\""));
    assert!(proto.contains("import \"📈️diagram2d/🛰️.proto\"") && proto.contains("import \"🕹️handles/🛰️.proto\""));
    assert!(graphql.contains("#import 📈️diagram2d/🔗️.graphql") && graphql.contains("#import 🕹️handles/🔗️.graphql"));
}

fn bar(pattern: Option<Pattern>, kind: FastenerKind) -> Geometry {
    Geometry::Fastener(FastenerGeometry { kind, axis: Segment3 { from: Vec3::ZERO, to: Vec3::new(2.0, 0.0, 0.0) }, diameter: 0.01, head: HeadShape::None, pattern, hole_clearance: None })
}

#[test]
fn cages_points_and_weld_runs_validate_and_round_trip() {
    let outline = Ring2 { points: vec![Vec2::new(-0.1, -0.2), Vec2::new(0.1, -0.2), Vec2::new(0.1, 0.2), Vec2::new(-0.1, 0.2)] };
    let cage = |zones, reference| bar(Some(Pattern::Cage { outline: outline.clone(), zones, reference }), FastenerKind::Rebar { bend: None });
    let zones = vec![LinkZone { start: 0.0, spacing: 0.1, count: 5 }, LinkZone { start: 0.8, spacing: 0.2, count: 4 }];
    let good = cage(zones.clone(), Vec3::Y);
    assert_eq!(good.validate(), Ok(()));
    assert_eq!(Geometry::from_value(good.to_value()).expect("decode"), good);
    assert_eq!(rules(cage(vec![], Vec3::Y).validate()), ["$.pattern.zones:non-empty"]);
    assert_eq!(rules(cage(zones.clone().into_iter().rev().collect(), Vec3::Y).validate()), ["$.pattern.zones:ascending-start"]);
    assert_eq!(rules(cage(zones, Vec3::ZERO).validate()), ["$.pattern.reference:non-zero-direction"]);
    let points = bar(Some(Pattern::Points { offsets: vec![Vec3::ZERO, Vec3::new(0.3, 0.1, 0.0)] }), FastenerKind::Dowel);
    assert_eq!(points.validate(), Ok(()));
    assert_eq!(rules(bar(Some(Pattern::Points { offsets: vec![] }), FastenerKind::Dowel).validate()), ["$.pattern.offsets:non-empty"]);
    let path = Path3::Polyline(Polyline3 { points: vec![Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0)] });
    let run = |pitch| bar(None, FastenerKind::WeldRun { throat: 0.004, weld: WeldKind::Fillet, path: path.clone(), pitch });
    assert_eq!(run(Some(WeldPitch { length: 0.1, gap: 0.1 })).validate(), Ok(()));
    assert_eq!(rules(run(Some(WeldPitch { length: 0.0, gap: 0.1 })).validate()), ["$.kind.pitch.length:positive"]);
    assert_eq!(Geometry::from_value(run(None).to_value()).expect("decode"), run(None));
}

#[test]
fn wall_cuts_and_laminates_validate() {
    let layer = WallLayer { thickness: 0.2, role: MaterialRole::Masonry, label: copy("Wall") };
    let wall = |openings| SolidShape::WallAssembly { path: Polyline3 { points: vec![Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0)] }, height: 2.5, layers: vec![layer.clone()], openings };
    let cut = |offset, width, sill, height| WallCut { offset, width, sill, height };
    assert_eq!(wall(vec![cut(0.5, 1.0, 0.0, 2.1), cut(2.0, 1.2, 0.9, 1.2)]).validate(), Ok(()));
    assert_eq!(rules(wall(vec![cut(3.5, 1.0, 0.0, 2.1)]).validate()), ["$.openings:inside-wall"]);
    assert_eq!(rules(wall(vec![cut(0.5, 1.0, 0.0, 2.1), cut(1.0, 1.0, 0.0, 2.1)]).validate()), ["$.openings:disjoint"]);
    let board = |grain| GrainLayer { thickness: 0.04, role: MaterialRole::Timber, grain, label: copy("Board") };
    let clt = SolidShape::Laminate { frame: Frame::WORLD, footprint: Polygon2::solid(vec![Vec2::ZERO, Vec2::new(3.0, 0.0), Vec2::new(3.0, 1.0), Vec2::new(0.0, 1.0)]), layers: vec![board(0.0), board(std::f64::consts::FRAC_PI_2), board(0.0)] };
    assert_eq!(clt.validate(), Ok(()));
    assert_eq!(SolidShape::from_value(clt.to_value()).expect("decode"), clt);
}

#[test]
fn scalar_fields_and_mode_surfaces_validate() {
    let field = |id: &str, samples| ScalarField { id: id.into(), unit: QuantityKind::Stress, samples };
    let vertex = FieldSamples::Vertex { values: vec![0.0, 1.0, 2.0] };
    let station = FieldSamples::Station { values: vec![StationValue { at: 0.0, value: 1.0 }, StationValue { at: 1.0, value: 3.0 }] };
    let channel = |active: &str| ColourChannel::Fields { active: active.into(), fields: vec![field("sigma", vertex.clone()), field("tendon", station.clone()), field("per-bolt", FieldSamples::Instance { values: vec![0.4, 0.9] })] };
    assert_eq!(channel("tendon").validate(), Ok(()));
    assert_eq!(rules(channel("missing").validate()), ["$.active:active-field-exists"]);
    let bad = ColourChannel::Fields { active: "a".into(), fields: vec![field("a", FieldSamples::Station { values: vec![StationValue { at: 0.5, value: 1.0 }, StationValue { at: 0.2, value: 1.0 }] })] };
    assert_eq!(rules(bad.validate()), ["$.fields[0].samples.values:strictly-ascending"]);
    assert_eq!(ColourChannel::from_value(channel("sigma").to_value()).expect("decode"), channel("sigma"));
    let mode = |samples, surface, alpha| DeformationOverlay { kind: DeformationKind::Mode, index: 1, frequency: Some(Quantity::new(QuantityKind::Frequency, 2.5)), participation: 1.0, scale: 1.0, samples, surface, motion: Motion::Harmonic, load_factor: alpha };
    let plane = vec![SurfaceSample { at: Vec2::new(0.1, 0.2), delta: Vec3::new(0.0, 0.0, 0.01) }];
    assert_eq!(mode(vec![], plane.clone(), Some(1.7)).validate(), Ok(()));
    assert_eq!(rules(mode(vec![], vec![], None).validate()), ["$.samples:non-empty"]);
    assert_eq!(rules(mode(vec![], plane, Some(0.0)).validate()), ["$.loadFactor:positive"]);
}

#[test]
fn parameter_tracks_validate_and_round_trip() {
    let key = |at: f64, field: Option<&str>, layer: Option<&str>| Keyframe { at, caption: copy("k"), field: field.map(Into::into), layer: layer.map(Into::into) };
    let track = |kind, keyframes| ParameterTrack { id: "fire".into(), kind, label: copy("Fire"), unit: QuantityKind::Time, min: 0.0, max: 3600.0, step: 60.0, looped: false, keyframes };
    let good = track(TrackKind::Time, vec![key(0.0, Some("temp@0"), None), key(1800.0, Some("temp@30"), None)]);
    assert_eq!(good.validate(), Ok(()));
    assert_eq!(ParameterTrack::from_value(good.to_value()).expect("decode"), good);
    assert_eq!(track(TrackKind::Phase, vec![]).validate(), Ok(()));
    assert_eq!(rules(track(TrackKind::Mode, vec![]).validate()), ["$.keyframes:non-empty"]);
    assert_eq!(rules(track(TrackKind::Age, vec![key(7200.0, Some("a"), None)]).validate()), ["$.keyframes:ascending-within-range"]);
    assert_eq!(rules(track(TrackKind::Time, vec![key(0.0, None, None)]).validate()), ["$.keyframes[0]:field-or-layer"]);
}

/// 🎚️ Tone and scene status at the utilisation boundary always agree with the shared check status.
#[test]
fn utilisation_tone_and_status_agree_with_the_shared_limit_rule() {
    for u in [0.5, 0.85, 0.95, 1.0, 1.0 + 1.0e-12, 1.0 + 5.0e-10, 1.0 + 2.0e-9, 1.0 + 1.0e-6, 3.0] {
        let failing = norm::limit_status(u) == norm::CheckStatus::Fail;
        assert_eq!(utilisation_tone(u) == Tone::Error, failing, "tone at {u}");
        assert_eq!(utilisation_status(u, 0.9) == CheckStatus::Fail, failing, "status at {u}");
    }
    assert_eq!(utilisation_tone(1.0 + 1.0e-12), Tone::Warning);
    assert_eq!(utilisation_tone(0.5), Tone::Info);
    assert_eq!(utilisation_status(0.95, 0.9), CheckStatus::Warning);
    assert_eq!(utilisation_status(0.5, 0.9), CheckStatus::Pass);
}

#[test]
fn quantity_kinds_are_declared_in_every_twin_with_their_wire_names() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📏️quantity-kinds/🔣️.json")).expect("fixture");
    let (ts, graphql, proto) = (include_str!("../../🧬️schema/🟦️.ts"), include_str!("../../🧬️schema/🔗️.graphql"), include_str!("../../🧬️schema/🛰️.proto"));
    let kinds = [("velocity", QuantityKind::Velocity), ("ventilationRate", QuantityKind::VentilationRate), ("heatTransferCoefficient", QuantityKind::HeatTransferCoefficient), ("power", QuantityKind::Power)];
    let rows = fixture["rows"].as_array().expect("rows");
    assert_eq!(rows.len(), kinds.len());
    for (row, (wire, kind)) in rows.iter().zip(kinds) {
        assert_eq!(row["kind"], wire);
        assert_eq!(canonical_json(&kind), format!("\"{wire}\""));
        assert!(ts.contains(&format!("\"{wire}\"")), "{wire} missing in the TypeScript twin");
        assert!(graphql.contains(&format!("\n  {}\n", row["graphql"].as_str().expect("graphql"))), "{wire} missing in the GraphQL twin");
        assert!(proto.contains(&format!("{} = ", row["proto"].as_str().expect("proto"))), "{wire} missing in the proto twin");
    }
}
