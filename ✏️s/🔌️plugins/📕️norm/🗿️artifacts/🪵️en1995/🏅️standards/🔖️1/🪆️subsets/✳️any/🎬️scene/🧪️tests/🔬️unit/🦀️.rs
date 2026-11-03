use super::*;
use scene::projection_kit::test_support::{revision, entity, assert_handle_laws};
use model::{CutoutFace, CutoutKind, LayerOrientation, MemberRole, SupportType, TaperKind, TensionSide};
use scene::{canonical_json, Validate};
use std::collections::BTreeSet;

fn report_of(document: &En1995Snapshot) -> CheckReport {
    schema::checks::timber::evaluate_structure(document)
}

fn scene_of(document: &En1995Snapshot, placements: &[Placement]) -> (StructuralScene, CheckReport) {
    let report = report_of(document);
    let scene = project(&SceneInput { document, report: &report, placements, revision: revision() });
    (scene, report)
}

fn compliant() -> En1995Snapshot {
    En1995Snapshot::compliant_building_beam()
}

fn failing() -> En1995Snapshot {
    En1995Snapshot::noncompliant_building()
}

fn fixtures() -> Vec<En1995Snapshot> {
    vec![compliant(), failing(), En1995Snapshot::compliant_bridge(), En1995Snapshot::noncompliant_bridge()]
}

fn burning() -> En1995Snapshot {
    let mut document = compliant();
    document.members[0].fire_duration_s = 1800.0;
    document
}

fn select(id: &str, kind: scene::EntityKind, collection: &str) -> Vec<scene::EntityRef> {
    vec![scene::EntityRef::new(kind, id, format!("{collection}[id={id}]"))]
}

#[test]
fn the_scene_of_every_fixture_validates() {
    for document in fixtures().into_iter().chain([burning()]) {
        let (scene, _) = scene_of(&document, &[]);
        assert_eq!(scene.validate(), Ok(()), "violations");
        assert_eq!(scene.schema, scene::SCENE_SCHEMA_ID);
    }
}

#[test]
fn ids_are_unique_and_layers_exist() {
    for document in fixtures().into_iter().chain([burning()]) {
        let (scene, _) = scene_of(&document, &[]);
        let ids: BTreeSet<_> = scene.entities.iter().map(|e| e.entity.id.clone()).collect();
        assert_eq!(ids.len(), scene.entities.len());
        let annotation_ids: BTreeSet<_> = scene.annotations.iter().map(|a| a.id().to_string()).collect();
        assert_eq!(annotation_ids.len(), scene.annotations.len());
        let layers: BTreeSet<_> = scene.layers.iter().map(|l| l.id.as_str()).collect();
        assert!(scene.entities.iter().all(|e| layers.contains(e.layer.as_str())));
    }
}

#[test]
fn projection_is_deterministic_and_locale_free() {
    let (a, _) = scene_of(&failing(), &[]);
    let (b, _) = scene_of(&failing(), &[]);
    assert_eq!(canonical_json(&a), canonical_json(&b));
    assert!(a.entities.iter().all(|e| !e.accessible_name.en.is_empty() && !e.accessible_name.de.is_empty()));
}

#[test]
fn a_glulam_member_extrudes_its_section_in_lamellae() {
    let document = compliant();
    let (scene, _) = scene_of(&document, &[]);
    let member = entity(&scene, "beam-B1");
    let scene::Geometry::Member(geometry) = &member.geometry else { panic!("member geometry") };
    let m = &document.members[0];
    assert!((geometry.section.area().expect("area") - m.b_m * m.h_m).abs() < 1e-9);
    assert!(matches!(&geometry.section, scene::SectionShape::Composite { parts, .. } if parts.len() == 10));
    let scene::Path3::Polyline(axis) = &geometry.axis else { panic!("polyline axis") };
    assert!((axis.points[0].distance(&axis.points[1]) - m.span_m).abs() < 1e-9);
    assert_eq!(member.entity.path, "members[id=beam-B1]");
}

#[test]
fn solid_timber_is_a_plain_rectangle() {
    let (scene, _) = scene_of(&failing(), &[]);
    let scene::Geometry::Member(geometry) = &entity(&scene, "beam-B2").geometry else { panic!("member geometry") };
    assert!(matches!(&geometry.section, scene::SectionShape::Parametric { profile: scene::ProfileKind::Rect, .. }));
}

#[test]
fn the_default_layout_runs_members_left_to_right_and_columns_upright() {
    let document = failing();
    let layout = Layout::of(&document, &[]);
    let beam = layout.member("beam-B2").expect("beam");
    let column = layout.member("col-C1").expect("column");
    assert_eq!(beam.frame.origin, scene::Vec3::ZERO);
    assert!((column.frame.origin.x - (document.members[0].span_m + layout::MEMBER_GAP)).abs() < 1e-12);
    assert_eq!(beam.frame.x, scene::Vec3::X);
    assert_eq!(column.frame.x, scene::Vec3::Z);
    assert!((layout.connection("conn-C2").expect("connection").y - layout::ROW_CONNECTIONS).abs() < 1e-12);
}

#[test]
fn a_continuous_beam_is_drawn_over_two_spans_with_three_supports() {
    let mut document = compliant();
    document.members[0].support = SupportType::ContinuousTwoSpan;
    let (scene, _) = scene_of(&document, &[]);
    for tag in ["support-a", "support-b", "support-c"] {
        assert!(matches!(entity(&scene, &format!("beam-B1:{tag}")).geometry, scene::Geometry::Support(_)));
    }
    let scene::Geometry::Member(geometry) = &entity(&scene, "beam-B1").geometry else { panic!("member") };
    let scene::Path3::Polyline(axis) = &geometry.axis else { panic!("axis") };
    assert!((axis.points[0].distance(&axis.points[1]) - 2.0 * document.members[0].span_m).abs() < 1e-9);
}

#[test]
fn a_persisted_placement_overrides_the_default_and_rotates_about_z() {
    let placement = Placement { entity_id: "beam-B1".into(), origin: scene::Vec3::new(10.0, 2.0, 0.0), yaw: std::f64::consts::FRAC_PI_2 };
    let layout = Layout::of(&compliant(), &[placement]);
    let pose = layout.member("beam-B1").expect("pose");
    assert_eq!(pose.frame.origin, scene::Vec3::new(10.0, 2.0, 0.0));
    assert!(pose.frame.x.y > 0.999 && pose.frame.x.x.abs() < 1e-9);
}

#[test]
fn utilisation_colouring_follows_the_governing_check() {
    let (scene, report) = scene_of(&compliant(), &[]);
    let member = entity(&scene, "beam-B1");
    let binding = member.utilisation.as_ref().expect("binding");
    let expected = report.checks.iter().filter(|c| c.status != norm::CheckStatus::NotApplicable && c.subject.entity_id == "beam-B1" && c.utilization.is_finite()).map(|c| c.utilization).fold(0.0_f64, f64::max);
    assert!((binding.ratio - expected).abs() < 1e-12);
    assert_eq!(member.appearance.channel, scene::ColourChannel::Utilisation);
}

#[test]
fn a_failing_member_gets_error_style_hatch_and_failure_glyphs() {
    let (scene, report) = scene_of(&failing(), &[]);
    assert!(report.checks.iter().any(|c| c.subject.entity_id == "beam-B2" && c.status == norm::CheckStatus::Fail));
    let member = entity(&scene, "beam-B2");
    assert_eq!(member.appearance.style, scene::StyleState::Error);
    assert_eq!(member.appearance.hatch, Some(scene::Hatch::Diagonal));
    assert!(scene.annotations.iter().any(|a| matches!(a, scene::Annotation::FailureGlyph { id, .. } if id.starts_with("beam-B2:failure-"))));
}

#[test]
fn bearings_loads_and_restraints_project_with_the_member() {
    let (scene, _) = scene_of(&compliant(), &[]);
    assert!(matches!(entity(&scene, "beam-B1:bearing-a").geometry, scene::Geometry::Solid(_)));
    assert!(matches!(entity(&scene, "beam-B1:load-g").geometry, scene::Geometry::Load(scene::LoadGlyph::Line { .. })));
    assert!(matches!(entity(&scene, "beam-B1:restraint-1").geometry, scene::Geometry::Support(_)));
    let (scene, _) = scene_of(&failing(), &[]);
    assert!(matches!(entity(&scene, "beam-B2:load-q:point").geometry, scene::Geometry::Load(scene::LoadGlyph::Point { .. })));
    assert!(matches!(entity(&scene, "col-C1:load-g:point").geometry, scene::Geometry::Load(scene::LoadGlyph::Point { .. })));
}

#[test]
fn the_notch_is_a_void_box_of_the_notch_depth() {
    let document = failing();
    let (scene, _) = scene_of(&document, &[]);
    let scene::Geometry::Solid(solid) = &entity(&scene, "beam-B2:notch").geometry else { panic!("notch") };
    let scene::SolidShape::Box { size, .. } = &solid.shape else { panic!("box") };
    assert!((size.z - document.members[0].notch_depth_m).abs() < 1e-12);
    assert!(scene.entities.iter().all(|e| e.entity.id != "beam-B1:notch"));
}

#[test]
fn fire_shows_residual_section_char_layer_and_zero_strength_layer() {
    let document = burning();
    let (scene, _) = scene_of(&document, &[]);
    let m = &document.members[0];
    let fire = facts::fire_after(document.annex, facts::properties(m.strength_class.id()).expect("class").product, m.b_m, m.h_m, m.fire_duration_s);
    let scene::Geometry::Member(residual) = &entity(&scene, "beam-B1").geometry else { panic!("member") };
    assert!((residual.section.area().expect("area") - fire.b_fi * fire.h_fi).abs() < 1e-9);
    let area = |id: &str| match &entity(&scene, id).geometry {
        scene::Geometry::Member(g) => g.section.area().expect("area"),
        _ => panic!("member geometry"),
    };
    let (b, h, c) = (m.b_m, m.h_m, fire.d_char);
    let ahead = c - members::FRONT_THICKNESS.min(c / 2.0);
    assert!((area("beam-B1:char") - (b * h - (b - 2.0 * ahead) * (h - 2.0 * ahead))).abs() < 1e-9);
    assert!((area("beam-B1:char") + area("beam-B1:front") - (b * h - (b - 2.0 * c) * (h - 2.0 * c))).abs() < 1e-9, "char and front fill the char layer");
    assert!((area("beam-B1:zsl") - ((b - 2.0 * c) * (h - 2.0 * c) - fire.b_fi * fire.h_fi)).abs() < 1e-9);
    assert!(fire.d_ef > fire.d_char);
    assert!(scene.entities.iter().any(|e| e.entity.id == "beam-B1:char"));
    let (cold, _) = scene_of(&compliant(), &[]);
    assert!(cold.entities.iter().all(|e| !e.entity.id.ends_with(":char") && !e.entity.id.ends_with(":zsl")));
}

#[test]
fn floors_and_footbridges_carry_a_vibration_mode_with_the_kernel_frequency() {
    let document = compliant();
    let (scene, _) = scene_of(&document, &[]);
    let overlay = entity(&scene, "beam-B1").appearance.deformation.as_ref().expect("mode");
    assert_eq!(overlay.kind, scene::DeformationKind::Mode);
    let m = &document.members[0];
    let props = facts::properties(m.strength_class.id()).expect("class");
    let expected = facts::dynamics(document.annex, m, &props).expect("floor").f1(m.span_m);
    assert!((overlay.frequency.as_ref().expect("frequency").value / expected - 1.0).abs() < 2e-3, "kernel eigen-frequency against pi / (2 L^2) sqrt(EI / m)");
    assert!(expected > 0.0);
    let bridge = En1995Snapshot::compliant_bridge();
    let (scene, _) = scene_of(&bridge, &[]);
    assert!(entity(&scene, "bridge-G1").appearance.deformation.as_ref().is_some_and(|o| o.kind == scene::DeformationKind::Mode));
    assert!(bridge.members[0].role == MemberRole::Bridge);
}

#[test]
fn a_connection_projects_layers_a_fastener_grid_zones_and_a_group() {
    let document = compliant();
    let (scene, _) = scene_of(&document, &[]);
    let c = &document.connections[0];
    let g = facts::grid(c);
    let scene::Geometry::Fastener(bolts) = &entity(&scene, "conn-C1:fasteners").geometry else { panic!("fasteners") };
    assert_eq!(bolts.pattern, Some(scene::Pattern::Grid { nx: c.rows, ny: g.per_row, sx: g.a2, sy: c.spacing_m }));
    assert_eq!(bolts.diameter, c.diameter_m);
    assert!(matches!(bolts.kind, scene::FastenerKind::Bolt { .. }));
    assert_eq!(g.per_row * g.rows, c.number);
    assert!(matches!(entity(&scene, "conn-C1:layer-0").geometry, scene::Geometry::Solid(_)));
    assert!(matches!(entity(&scene, "conn-C1:zone-a1").geometry, scene::Geometry::Surface(_)));
    let scene::Geometry::Group { members } = &entity(&scene, "conn-C1").geometry else { panic!("group") };
    assert_eq!(members.len(), 2 + 1 + 4);
}

#[test]
fn nails_and_steel_plates_and_double_shear_stack_the_layers() {
    let (scene, _) = scene_of(&failing(), &[]);
    let scene::Geometry::Fastener(nails) = &entity(&scene, "conn-C2:fasteners").geometry else { panic!("fasteners") };
    assert_eq!(nails.kind, scene::FastenerKind::Nail);
    let mut document = compliant();
    document.connections[0].steel_plate = true;
    document.connections[0].steel_plate_thickness_m = 0.008;
    document.connections[0].shear_planes = 2;
    let (scene, _) = scene_of(&document, &[]);
    assert!(matches!(entity(&scene, "conn-C1:layer-1").geometry, scene::Geometry::Plate(_)));
    assert!(matches!(entity(&scene, "conn-C1:layer-2").geometry, scene::Geometry::Solid(_)));
}

#[test]
fn spacing_zones_are_coloured_by_their_own_check() {
    let (scene, report) = scene_of(&failing(), &[]);
    let a1 = report.checks.iter().find(|c| c.id == "en1995.8.spacing.a1.conn-C2").expect("a1 check");
    let zone = entity(&scene, "conn-C2:zone-a1");
    assert_eq!(zone.utilisation.as_ref().expect("binding").check_id, a1.id);
    assert_eq!(zone.appearance.style == scene::StyleState::Error, a1.status == norm::CheckStatus::Fail);
}

#[test]
fn the_governing_johansen_mode_is_the_lowest_bar() {
    for document in fixtures() {
        for c in &document.connections {
            let j = facts::johansen(c).expect("johansen");
            assert!(j.modes.iter().all(|m| m.1 >= j.modes[j.governing].1));
        }
    }
    let (scene, _) = scene_of(&compliant(), &[]);
    assert!(scene.annotations.iter().any(|a| matches!(a, scene::Annotation::Label { id, text, .. } if id == "conn-C1:mode" && text.en.contains("mode"))));
}

#[test]
fn every_advertised_handle_exists_and_writes_a_known_leaf() {
    for document in [failing(), burning(), compliant()] {
        let (scene, _) = scene_of(&document, &[]);
        let selection: Vec<_> = scene.entities.iter().map(|e| e.entity.clone()).collect();
        let all = handles(&document, &[], &selection);
        assert_handle_laws(&all);
        let ids: BTreeSet<_> = all.iter().map(|h| h.id.clone()).collect();
        for e in &scene.entities {
            assert!(e.handles.iter().all(|h| ids.contains(h)), "{} advertises unknown handle", e.entity.id);
        }
        assert!(all.iter().all(|h| h.writes.iter().all(|w| w.path.contains("[id=") && w.path.contains('.'))));
        assert!(all.iter().all(|h| h.accessible_name.en.len() > 2 && h.accessible_name.de.len() > 2));
        assert!(all.iter().all(|h| h.validate().is_ok()), "handle violations");
    }
}

#[test]
fn dragging_the_span_handle_writes_the_snapped_span_and_halves_for_two_spans() {
    let document = compliant();
    let all = handles(&document, &[], &select("beam-B1", scene::EntityKind::Member, "members"));
    assert_handle_laws(&all);
    let span = all.iter().find(|h| h.id == "beam-B1:span").expect("span handle");
    let writes = span.writes_at(5.012);
    assert_eq!(writes[0].path, "members[id=beam-B1].spanM");
    assert!((writes[0].value - 5.0).abs() < 1e-9);
    assert_eq!(span.writes_at(1000.0)[0].value, 40.0);
    let mut two = compliant();
    two.members[0].support = SupportType::ContinuousTwoSpan;
    let all = handles(&two, &[], &select("beam-B1", scene::EntityKind::Member, "members"));
    assert_handle_laws(&all);
    assert!((all.iter().find(|h| h.id == "beam-B1:span").expect("span").writes_at(10.0)[0].value - 5.0).abs() < 1e-9);
}

#[test]
fn section_handles_double_the_face_offset_and_snap_to_the_lamella() {
    let all = handles(&compliant(), &[], &select("beam-B1", scene::EntityKind::Member, "members"));
    assert_handle_laws(&all);
    let depth = all.iter().find(|h| h.id == "beam-B1:depth").expect("depth");
    assert_eq!(depth.writes_at(0.2)[0].path, "members[id=beam-B1].hM");
    assert!((depth.writes_at(0.2)[0].value - 0.40).abs() < 1e-9);
    assert!((depth.writes_at(0.213)[0].value - 0.44).abs() < 1e-9 || (depth.writes_at(0.213)[0].value - 0.40).abs() < 1e-9);
    let width = all.iter().find(|h| h.id == "beam-B1:width").expect("width");
    assert!((width.writes_at(0.1)[0].value - 0.20).abs() < 1e-9);
}

#[test]
fn notch_and_fire_handles_invert_to_the_current_values() {
    let document = failing();
    let m = &document.members[0];
    let all = handles(&document, &[], &select("beam-B2", scene::EntityKind::Member, "members"));
    assert_handle_laws(&all);
    let depth = all.iter().find(|h| h.id == "beam-B2:notch-depth").expect("notch depth");
    assert!((depth.writes_at(depth.anchor.z)[0].value - m.notch_depth_m).abs() < 5e-3);
    let distance = all.iter().find(|h| h.id == "beam-B2:notch-distance").expect("notch distance");
    assert!((distance.writes_at(distance.anchor.x)[0].value - m.notch_distance_m).abs() < 5e-3);
    let fired = burning();
    let all = handles(&fired, &[], &select("beam-B1", scene::EntityKind::Member, "members"));
    assert_handle_laws(&all);
    let time = all.iter().find(|h| h.id == "beam-B1:fire-time").expect("fire time");
    assert!((time.writes_at(time.anchor.z)[0].value - fired.members[0].fire_duration_s).abs() <= 60.0);
    assert!(handles(&compliant(), &[], &select("beam-B1", scene::EntityKind::Member, "members")).iter().all(|h| !h.id.ends_with("fire-time")));
}

#[test]
fn connection_handles_cover_spacings_distances_counts_and_thicknesses() {
    let all = handles(&compliant(), &[], &select("conn-C1", scene::EntityKind::Group, "connections"));
    assert_handle_laws(&all);
    let tags: BTreeSet<_> = all.iter().map(|h| h.id.rsplit(':').next().unwrap_or_default().to_string()).collect();
    for tag in ["spacing", "end-distance", "edge-distance", "number", "rows", "diameter", "t1", "t2", "planes"] {
        assert!(tags.contains(tag), "missing {tag}");
    }
    let rows = all.iter().find(|h| h.id.ends_with(":rows")).expect("rows");
    assert_eq!(rows.writes_at(2.6)[0].value, 3.0);
    let diameter = all.iter().find(|h| h.id.ends_with(":diameter")).expect("diameter");
    assert!((diameter.writes_at(0.006)[0].value - 0.012).abs() < 1e-9);
    let t2 = all.iter().find(|h| h.id.ends_with(":t2")).expect("t2");
    assert_eq!(t2.writes_at(0.4)[0].path, "connections[id=conn-C1].t2M");
    assert!((t2.writes_at(0.4)[0].value - 0.2).abs() < 1e-9);
}

#[test]
fn derived_scene_ids_resolve_to_their_document_entity() {
    let document = compliant();
    let derived = handles(&document, &[], &select("conn-C1:fasteners", scene::EntityKind::Fastener, "connections"));
    assert_handle_laws(&derived);
    assert!(derived.iter().any(|h| h.id == "conn-C1:spacing"));
    assert_eq!(entity_kind_of(&document, "conn-C1:zone-a1").map(|k| k.collection), Some("connections"));
    assert_eq!(entity_kind_of(&document, "beam-B1").map(|k| k.id), Some("member"));
    assert_eq!(entity_kind_of(&document, "nothing"), None);
}

#[test]
fn dependencies_name_what_a_projection_reads() {
    let document = compliant();
    assert_eq!(entity_dependencies(&document, "beam-B1"), vec!["g".to_string(), "q".to_string()]);
    assert_eq!(entity_dependencies(&document, "conn-C1:fasteners"), vec!["g".to_string(), "q".to_string()]);
}

fn all_diagrams(document: &En1995Snapshot) -> Vec<scene::Diagram2d> {
    diagrams(document, &report_of(document), None)
}

#[test]
fn every_diagram_validates_and_has_a_table_twin() {
    let mut expected: Vec<(En1995Snapshot, Vec<&str>)> = vec![
        (compliant(), vec!["kmod:beam-B1", "kc:beam-B1", "kcrit:beam-B1", "deflection:beam-B1", "frequency:beam-B1", "velocity:beam-B1", "embedment:conn-C1", "johansen:conn-C1", "failure-modes:conn-C1", "spacing:conn-C1"]),
        (failing(), vec!["kmod:beam-B2", "kc:col-C1", "kcrit:beam-B2", "deflection:beam-B2", "fire:beam-B2", "embedment:conn-C2", "johansen:conn-C2", "failure-modes:conn-C2", "spacing:conn-C2"]),
        (En1995Snapshot::compliant_bridge(), vec![]),
        (burning(), vec!["fire:beam-B1"]),
    ];
    expected[2].1 = vec!["frequency:bridge-G1"];
    for (document, ids) in expected {
        let all = all_diagrams(&document);
        let found: BTreeSet<_> = all.iter().map(|d| d.id.clone()).collect();
        for id in ids {
            assert!(found.contains(id), "missing diagram {id} in {found:?}");
        }
        for diagram in &all {
            assert_eq!(diagram.validate(), Ok(()), "diagram {}", diagram.id);
            assert!(diagram.table_twin.rows() > 0, "diagram {} has no table twin", diagram.id);
        }
    }
}

#[test]
fn the_kc_markers_sit_on_the_curve_and_drag_the_buckling_length() {
    let document = failing();
    let report = report_of(&document);
    let diagram = diagrams(&document, &report, Some("col-C1")).into_iter().find(|d| d.id == "kc:col-C1").expect("kc");
    let m = &document.members[1];
    let props = facts::properties(m.strength_class.id()).expect("class");
    let s = facts::slenderness(m, &props);
    let marker = diagram.markers.iter().find(|x| x.id == "marker-z").expect("marker z");
    assert!((marker.at.x - s.rel_z.min(3.0)).abs() < 1e-12);
    assert!((marker.at.y - schema::checks::timber::kc_factor(s.rel_z, s.beta_c)).abs() < 1e-12);
    let binding = marker.binding.as_ref().expect("binding");
    assert_eq!(binding.path, "members[id=col-C1].bucklingLengthZM");
    assert!((binding.value_at(s.rel_z) - m.buckling_length_z_m).abs() < 0.03);
}

#[test]
fn the_deflection_bars_are_the_kernel_components() {
    let document = compliant();
    let diagram = all_diagrams(&document).into_iter().find(|d| d.id == "deflection:beam-B1").expect("deflection");
    let m = &document.members[0];
    let props = facts::properties(m.strength_class.id()).expect("class");
    let w = facts::deflection(document.annex, document.reliability_class, m, &props).expect("deflection");
    let column = &diagram.table_twin.columns[1].values;
    assert!((column[0] - w.w_g * 1e3).abs() < 1e-9 && (column[1] - w.w_q * 1e3).abs() < 1e-9 && (column[3] - w.w_fin * 1e3).abs() < 1e-9);
    assert!((column[3] - (column[0] + column[1] + column[2])).abs() < 1e-9);
}

#[test]
fn the_spacing_markers_drag_the_connection_leaves_in_metres() {
    let document = failing();
    let diagram = all_diagrams(&document).into_iter().find(|d| d.id == "spacing:conn-C2").expect("spacing");
    let marker = diagram.markers.iter().find(|m| m.id == "marker-a1").expect("a1");
    let binding = marker.binding.as_ref().expect("binding");
    assert_eq!(binding.path, "connections[id=conn-C2].spacingM");
    assert!((binding.value_at(marker.at.y) - document.connections[0].spacing_m).abs() < 0.0051);
}

#[test]
fn the_fire_marker_sits_at_the_effective_depth_and_drags_the_duration() {
    let document = burning();
    let diagram = all_diagrams(&document).into_iter().find(|d| d.id == "fire:beam-B1").expect("fire");
    let marker = &diagram.markers[0];
    assert!((marker.at.x - 30.0).abs() < 1e-9);
    assert!((marker.binding.as_ref().expect("binding").value_at(30.0) - 1800.0).abs() < 1e-9);
}

#[test]
fn focus_narrows_the_diagram_set() {
    let document = failing();
    let report = report_of(&document);
    let only = diagrams(&document, &report, Some("conn-C2"));
    assert_eq!(only.iter().map(|d| d.id.as_str()).collect::<Vec<_>>(), vec!["embedment:conn-C2", "johansen:conn-C2", "spacing:conn-C2", "n-ef:conn-C2"]);
    assert!(diagrams(&document, &report, Some("nothing")).is_empty());
}

#[test]
fn golden_scene_matches_committed_bytes() {
    let (scene, _) = scene_of(&compliant(), &[]);
    let actual = canonical_json(&scene);
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧫️fixtures/🏛️scene/🔣️.json");
    if std::env::var("SEMIO_UPDATE_GOLDEN").is_ok() {
        std::fs::write(path, &actual).expect("write golden");
    }
    let expected = std::fs::read_to_string(path).expect("golden fixture");
    assert_eq!(actual.trim(), expected.trim(), "golden scene changed; rerun with SEMIO_UPDATE_GOLDEN=1 after reviewing the diff");
}

#[test]
fn the_independent_oracle_agrees() {
    let documents = [compliant(), failing(), burning()];
    let mut parts = Vec::new();
    for document in &documents {
        let (scene, _) = scene_of(document, &[]);
        let diagram_json: Vec<String> = all_diagrams(document).iter().map(canonical_json).collect();
        let members: Vec<String> = document
            .members
            .iter()
            .map(|m| {
                let props = facts::properties(m.strength_class.id()).expect("class");
                let fire = facts::fire_of(document.annex, m, &props);
                let slender = facts::slenderness(m, &props);
                format!(
                    "{{\"id\":\"{}\",\"b\":{},\"h\":{},\"fire\":{},\"relY\":{},\"relZ\":{},\"betaC\":{}}}",
                    m.id,
                    m.b_m,
                    m.h_m,
                    fire.map_or("null".to_string(), |f| format!("{{\"dChar\":{},\"dEf\":{},\"bFi\":{},\"hFi\":{}}}", f.d_char, f.d_ef, f.b_fi, f.h_fi)),
                    slender.rel_y,
                    slender.rel_z,
                    slender.beta_c
                )
            })
            .collect();
        let connections: Vec<String> = document
            .connections
            .iter()
            .filter_map(|c| {
                let j = facts::johansen(c)?;
                let g = facts::grid(c);
                Some(format!(
                    "{{\"id\":\"{}\",\"kind\":\"{}\",\"steel\":{},\"planes\":{},\"d\":{},\"t1\":{},\"t2\":{},\"ts\":{},\"fh\":{},\"my\":{},\"perRow\":{},\"rows\":{},\"a1\":{},\"a2\":{},\"modes\":[{}]}}",
                    c.id,
                    g.kind,
                    c.steel_plate,
                    c.shear_planes,
                    c.diameter_m * 1e3,
                    c.t1_m * 1e3,
                    c.t2_m * 1e3,
                    c.steel_plate_thickness_m * 1e3,
                    j.fh,
                    j.my,
                    g.per_row,
                    g.rows,
                    g.a1,
                    g.a2,
                    j.modes.iter().map(|m| format!("{{\"mode\":\"{}\",\"rk\":{}}}", m.0, m.1)).collect::<Vec<_>>().join(",")
                ))
            })
            .collect();
        parts.push(format!("{{\"scene\":{},\"diagrams\":[{}],\"members\":[{}],\"connections\":[{}]}}", canonical_json(&scene), diagram_json.join(","), members.join(","), connections.join(",")));
    }
    let input = format!("[{}]", parts.join(","));
    let script = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🔮️oracles/🐍️.py");
    let output = norm::python_oracle::run_ok(script, &[], Some(input.as_bytes()));
    let verdict = String::from_utf8_lossy(&output.stdout);
    assert!(verdict.contains("\"ok\": true"), "oracle verdict: {verdict}");
}

#[test]
fn projecting_a_thousand_members_stays_interactive() {
    let mut document = compliant();
    let template = document.members[0].clone();
    document.members = (0..1000).map(|i| model::TimberMember { id: format!("m{i}"), ..template.clone() }).collect();
    document.connections.clear();
    let report = CheckReport::default();
    let started = std::time::Instant::now();
    let scene = project(&SceneInput { document: &document, report: &report, placements: &[], revision: revision() });
    assert!(scene.entities.len() >= 1000);
    eprintln!("[latency] en1995/scene-cold-1000 p50_ms={0:.3} p95_ms={0:.3} budget_ms={1:.1}", started.elapsed().as_secs_f64() * 1e3, 2000.0);
    assert!(started.elapsed().as_millis() < 2000, "projection took {:?}", started.elapsed());
}

#[test]
fn the_mode_shapes_are_eigen_solutions_of_the_drawn_supports() {
    let (span, ei, mass) = (5.0, 4.0e6, 80.0);
    let pinned = modes::first_vertical(SupportType::SimplySupported, span, ei, mass).expect("pinned");
    let closed = std::f64::consts::PI / (2.0 * span * span) * (ei / mass).sqrt();
    assert!((pinned.frequency / closed - 1.0).abs() < 1e-3);
    assert!(pinned.shape.iter().all(|&(t, u)| (u - (std::f64::consts::PI * t).sin()).abs() < 5e-3));
    let cantilever = modes::first_vertical(SupportType::Cantilever, span, ei, mass).expect("cantilever");
    let beta = 1.875_104_068_711_961;
    assert!((cantilever.frequency / (beta * beta / (2.0 * std::f64::consts::PI * span * span) * (ei / mass).sqrt()) - 1.0).abs() < 2e-3);
    assert!(cantilever.shape.first().is_some_and(|&(_, u)| u.abs() < 1e-12) && cantilever.shape.last().is_some_and(|&(_, u)| (u.abs() - 1.0).abs() < 1e-9));
    let continuous = modes::first_vertical(SupportType::ContinuousTwoSpan, span, ei, mass).expect("continuous");
    assert!((continuous.frequency / closed - 1.0).abs() < 1e-3, "first mode of two equal spans equals the single pinned span");
    let (left, right) = (continuous.shape[4].1, continuous.shape[12].1);
    assert!(left * right < 0.0 && (left + right).abs() < 1e-6, "antisymmetric about the middle support");
}

#[test]
fn check_stations_colour_the_member_the_deformed_shape_and_the_fasteners() {
    let document = compliant();
    let (scene, report) = scene_of(&document, &[]);
    let bending = report.checks.iter().find(|c| c.id == "en1995.6.1.6.bending.beam-B1").expect("bending");
    assert_eq!(bending.stations.len(), 11);
    let member = entity(&scene, "beam-B1");
    let stations = member.utilisation.as_ref().and_then(|u| u.per_station.as_ref()).expect("envelope");
    assert!(stations.len() >= 11 && stations.iter().map(|s| s.value).fold(0.0, f64::max) >= bending.utilization - 1e-9);
    let johansen = report.checks.iter().find(|c| c.id.contains(".johansen.conn-C1")).expect("johansen");
    assert_eq!(johansen.station_field, "fastener-force");
    let scene::ColourChannel::Fields { fields, .. } = &entity(&scene, "conn-C1:fasteners").appearance.channel else { panic!("fastener field") };
    let scene::FieldSamples::Instance { values } = &fields[0].samples else { panic!("instances") };
    let c = &document.connections[0];
    assert_eq!(values.len() as u32, c.rows * facts::grid(c).per_row);
}

#[test]
fn the_fire_track_grows_the_char_layer_linearly_with_the_exposure_time() {
    let document = burning();
    let (scene, _) = scene_of(&document, &[]);
    assert_eq!(scene.validate(), Ok(()));
    let track = scene.tracks.iter().find(|t| t.id == "fire").expect("fire track");
    assert_eq!((track.kind, track.min, track.max, track.keyframes.len()), (scene::TrackKind::Time, 900.0, 7200.0, 8));
    assert!(track.keyframes.iter().all(|k| k.layer.as_ref().is_some_and(|l| scene.layers.iter().any(|x| &x.id == l && x.visible))));
    let m = &document.members[0];
    let beta = facts::fire_after(document.annex, facts::properties(m.strength_class.id()).expect("class").product, m.b_m, m.h_m, 3600.0).beta_n;
    let area = |id: &str| match &entity(&scene, id).geometry {
        scene::Geometry::Member(g) => g.section.area().expect("area"),
        _ => panic!("member geometry"),
    };
    let mut front_depth = 0.0;
    for minutes in [15.0, 30.0, 45.0, 60.0, 75.0, 90.0, 105.0, 120.0] {
        let c = beta * 60.0 * minutes;
        let expected = m.b_m * m.h_m - (m.b_m - 2.0 * c).max(1e-3) * (m.h_m - 2.0 * c).max(1e-3);
        let layer = area(&format!("beam-B1:char@{minutes}")) + area(&format!("beam-B1:front@{minutes}"));
        assert!((layer - expected).abs() < 1e-9, "{minutes} min");
        assert!(c > front_depth, "the char front advances with the exposure time");
        front_depth = c;
    }
    assert!(compliant_scene_has_no_fire_track());
}

fn compliant_scene_has_no_fire_track() -> bool {
    scene_of(&compliant(), &[]).0.tracks.iter().all(|track| track.kind != scene::TrackKind::Time)
}

#[test]
fn catalogue_cases_are_drawn_bound_and_editable() {
    let mut document = compliant();
    document.cases.push(model::CaseRow { id: "case-t45".into(), label_en: "Notched support".into(), label_de: "Ausgeklinktes Auflager".into(), entry_id: "T-45".into(), numbers: ["v_d:20000", "b:120", "h:240", "h_ef:180", "x:30", "sc:1"].iter().map(|s| { let (k, v) = s.split_once(':').unwrap(); model::CaseNumber { id: k.into(), value: v.parse().unwrap() } }).collect(), texts: vec![model::CaseText { id: "class".into(), value: "C24".into() }] });
    document.cases.push(model::CaseRow { id: "case-t71".into(), label_en: "Bolt".into(), label_de: "Bolzen".into(), entry_id: "T-71".into(), numbers: ["d:12", "t1:60", "t2:60", "f_u:400", "sc:1"].iter().map(|s| { let (k, v) = s.split_once(':').unwrap(); model::CaseNumber { id: k.into(), value: v.parse().unwrap() } }).collect(), texts: vec![model::CaseText { id: "class".into(), value: "C24".into() }, model::CaseText { id: "duration".into(), value: "medium".into() }] });
    let (scene, report) = scene_of(&document, &[]);
    assert_eq!(scene.validate(), Ok(()));
    for id in ["case-kser", "case-t45", "case-t71"] {
        assert!(scene.entities.iter().any(|e| e.entity.id == id && e.entity.collection == "cases"), "{id} group");
    }
    assert!(scene.entities.iter().any(|e| e.entity.id == "case-t45:notch"), "notch void of the notched support");
    assert!(scene.entities.iter().any(|e| e.entity.id == "case-t71:fastener-0"), "fastener of the bolt case");
    let group = scene.entities.iter().find(|e| e.entity.id == "case-t71").unwrap();
    assert!(group.utilisation.is_some(), "the case is bound to its verification");
    let handles = handles(&document, &[], &[group.entity.clone()]);
    assert_handle_laws(&handles);
    assert!(handles.iter().any(|h| h.writes.iter().any(|w| w.path == "cases[id=case-t71].numbers[id=d].value")));
    assert!(handles.iter().any(|h| h.writes.iter().any(|w| w.path == "cases[id=case-t71].numbers[id=t1].value")));
    let notched = scene.entities.iter().find(|e| e.entity.id == "case-t45").unwrap();
    assert!(handles(&document, &[], &[notched.entity.clone()]).iter().any(|h| h.writes.iter().any(|w| w.path == "cases[id=case-t45].numbers[id=h_ef].value")));
    assert!(report.checks.iter().any(|c| c.subject.entity_id == "case-t45"));
}

#[test]
fn the_n_ef_diagram_reproduces_the_effective_number_and_drags_the_fastener_count() {
    let document = compliant();
    let connection = &document.connections[0];
    let diagram = all_diagrams(&document).into_iter().find(|d| d.id == format!("n-ef:{}", connection.id)).expect("n_ef");
    assert_eq!(diagram.validate(), Ok(()));
    let g = facts::grid(connection);
    let marker = &diagram.markers[0];
    let expected = schema::checks::timber::n_ef_value(facts::fastener_kind(connection), f64::from(g.per_row), g.a1 * 1e3, connection.diameter_m * 1e3, 0.0, false);
    assert!((marker.at.y - expected).abs() < 1e-12 && marker.at.y <= f64::from(g.per_row) + 1e-12);
    let binding = marker.binding.as_ref().expect("binding");
    assert_eq!(binding.path, format!("connections[id={}].number", connection.id));
    assert!((binding.value_at(f64::from(g.per_row)) - f64::from(g.per_row * g.rows)).abs() < 1e-9);
    assert!(marker.bound_check.is_some());
}

#[test]
fn the_connection_carries_a_splitting_plane_and_a_row_shear_block() {
    let document = compliant();
    let (scene, _) = scene_of(&document, &[]);
    let connection = &document.connections[0];
    let plane = entity(&scene, &format!("{}:{}", connection.id, connections::SPLITTING_TAG));
    assert!(matches!(plane.geometry, scene::Geometry::Surface(_)));
    let block = entity(&scene, &format!("{}:{}", connection.id, connections::BLOCK_TAG));
    assert!(matches!(block.geometry, scene::Geometry::Solid(_)));
}

fn case_of(id: &str, entry: &str, numbers: &[(&str, f64)], texts: &[(&str, &str)]) -> model::CaseRow {
    model::CaseRow { id: id.into(), label_en: id.into(), label_de: id.into(), entry_id: entry.into(), numbers: numbers.iter().map(|(k, v)| model::CaseNumber { id: (*k).into(), value: *v }).collect(), texts: texts.iter().map(|(k, v)| model::CaseText { id: (*k).into(), value: (*v).into() }).collect() }
}

#[test]
fn domain_cases_get_their_real_geometry_diagrams_and_motion() {
    let mut document = compliant();
    document.cases = vec![
        case_of("tap", "T-30", &[("m_d", 60e6), ("b", 140.0), ("h", 500.0), ("alpha_deg", 6.0), ("sc", 1.0)], &[("class", "GL28h"), ("duration", "medium"), ("side", "compression")]),
        case_of("apx", "T-31", &[("m_ap", 120e6), ("b", 160.0), ("h_ap", 900.0), ("r_in", 12000.0), ("t_lam", 33.0), ("alpha_ap_deg", 10.0), ("sc", 1.0)], &[("class", "GL28h"), ("duration", "medium")]),
        case_of("clt", "T-97", &[("n_layers", 3.0), ("t1", 30.0), ("o1", 1.0), ("t2", 40.0), ("o2", 0.0), ("t3", 30.0), ("o3", 1.0), ("l", 4000.0), ("g_r", 50.0)], &[("class", "CLT100")]),
        case_of("fire", "T-101", &[("b", 200.0), ("h", 400.0), ("t_req", 60.0), ("m_fi", 60e6), ("sc", 1.0)], &[("class", "GL28h")]),
        case_of("floor", "T-63", &[("span_m", 4.5), ("ei_l", 12.5e6), ("mass_kg_m2", 150.0)], &[("class", "C24")]),
        case_of("pat", "T-71", &[("d", 12.0), ("t1", 60.0), ("t2", 60.0), ("f_u", 400.0), ("sc", 1.0), ("px1", 84.0), ("py1", 36.0), ("px2", 168.0), ("py2", 72.0), ("px3", 252.0), ("py3", 36.0)], &[("class", "C24"), ("duration", "medium")]),
    ];
    document.fasteners = with_entities().fasteners;
    document.fasteners[0].positions = ["p1:0.084:0.036", "p2:0.168:0.072", "p3:0.252:0.036"].iter().map(|s| { let f: Vec<&str> = s.split(':').collect(); model::FastenerPosition { id: f[0].into(), x_m: f[1].parse().unwrap(), y_m: f[2].parse().unwrap() } }).collect();
    let (scene, report) = scene_of(&document, &[]);
    assert_eq!(scene.validate(), Ok(()));
    let has = |id: &str| scene.entities.iter().any(|e| e.entity.id == id);
    assert!(has("tap") && has("apx") && has("apx:apex"), "tapered body, curved pitched body and apex zone");
    assert!(has("clt"), "CLT laminate with one grain layer per layup row");
    for minutes in [30, 60, 90, 120] {
        assert!(has(&format!("fire:char@{minutes}")), "char layer after {minutes} min");
    }
    assert!(scene.tracks.iter().any(|t| t.id == "fire"), "fire time track");
    let floor = scene.entities.iter().find(|e| e.entity.id == "floor").unwrap();
    assert!(matches!(&floor.appearance.deformation, Some(d) if d.motion == scene::Motion::Harmonic && d.frequency.is_some()), "animated first mode");
    let pattern = scene.entities.iter().find(|e| e.entity.id == "fastener-1:fasteners").unwrap();
    assert!(matches!(&pattern.geometry, scene::Geometry::Fastener(f) if matches!(&f.pattern, Some(scene::Pattern::Points { offsets }) if offsets.len() == 3)), "explicit fastener pattern of the typed entity");
    assert!(scene.entities.iter().any(|e| e.entity.id == "pat:fastener-0"), "the catalogue bolt case is drawn by the generic forms");
    let diagrams = diagrams(&document, &report, None);
    for id in ["case-fire:fire", "case-mode:floor", "case-layup:clt", "case-results:pat"] {
        let diagram = diagrams.iter().find(|d| d.id == id).unwrap_or_else(|| panic!("{id}"));
        assert!(!diagram.table_twin.columns.is_empty());
    }
    let handles = handles(&document, &[], &[scene.entities.iter().find(|e| e.entity.id == "clt").unwrap().entity.clone()]);
    assert_handle_laws(&handles);
    assert!(handles.iter().any(|h| h.writes.iter().any(|w| w.path == "cases[id=clt].numbers[id=t2].value")));
}

#[test]
fn clt_layers_are_hatched_along_or_across_the_span_by_their_orientation() {
    let mut document = compliant();
    document.cases = vec![case_of("clt", "T-97", &[("n_layers", 3.0), ("t1", 30.0), ("o1", 1.0), ("t2", 40.0), ("o2", 0.0), ("t3", 30.0), ("o3", 1.0), ("l", 4000.0), ("g_r", 50.0)], &[("class", "CLT100")])];
    let report = report_of(&document);
    let layup = diagrams(&document, &report, None).into_iter().find(|d| d.id == "case-layup:clt").expect("layup");
    let hatches: Vec<_> = layup.series.iter().filter_map(|s| match s { scene::Series::Polygon { hatch, .. } => Some(*hatch), _ => None }).collect();
    assert_eq!(hatches, vec![Some(scene::Hatch::Horizontal), Some(scene::Hatch::Vertical), Some(scene::Hatch::Horizontal)]);
}

fn cross(a: &scene::Vec3, b: &scene::Vec3) -> (f64, f64, f64) {
    (a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x)
}

/// 📐 Frame conventions of the structural scene, read from its tessellation: a `Prism` extrudes its base polygon (frame coordinates) along the frame `z`, a `Laminate` stacks layers along `z` over a footprint centred on the frame origin, and `Pattern::Points` offsets are `(axial, e1, e2)` with `e1 = Y` and `e2 = -X` for a `+Z` axis (`basis`: `u = unit(axis x X)`, `w = axis x u`).
#[test]
fn frame_conventions_of_prism_laminate_and_point_patterns_hold() {
    let mut document = compliant();
    document.cases = vec![
        case_of("apx", "T-31", &[("b", 160.0), ("h_ap", 900.0), ("r_in", 12000.0), ("alpha_ap_deg", 10.0)], &[("class", "GL28h")]),
        case_of("clt", "T-97", &[("n_layers", 2.0), ("t1", 30.0), ("o1", 1.0), ("t2", 40.0), ("o2", 0.0), ("l", 4000.0)], &[("class", "CLT100")]),
    ];
    document.fasteners = with_entities().fasteners;
    document.fasteners[0].positions = vec![model::FastenerPosition { id: "p1".into(), x_m: 0.084, y_m: 0.036 }, model::FastenerPosition { id: "p2".into(), x_m: 0.168, y_m: 0.072 }];
    let (scene, _) = scene_of(&document, &[]);
    let geometry = |id: &str| scene.entities.iter().find(|e| e.entity.id == id).unwrap().geometry.clone();
    let scene::Geometry::Solid(solid) = geometry("apx") else { panic!("solid") };
    let scene::SolidShape::Prism { frame, height, .. } = solid.shape else { panic!("prism") };
    assert_eq!(cross(&frame.x, &frame.y), (frame.z.x, frame.z.y, frame.z.z), "right-handed frame");
    assert_eq!((frame.z.x, frame.z.y, frame.z.z), (0.0, -1.0, 0.0), "the elevation is extruded towards -Y");
    let pose = Layout::of(&document, &[]).case("apx").cloned().expect("pose");
    assert!((frame.origin.y - pose.origin.y - height / 2.0).abs() < 1e-9, "so the prism spans Y in [-b/2, b/2] about the centre of the case from its origin at Y = b/2");
    let scene::Geometry::Solid(solid) = geometry("clt") else { panic!("solid") };
    let scene::SolidShape::Laminate { frame, footprint, layers } = solid.shape else { panic!("laminate") };
    let (min_x, max_x) = footprint.outer.points.iter().fold((f64::MAX, f64::MIN), |m, p| (m.0.min(p.x), m.1.max(p.x)));
    assert!((min_x + max_x).abs() < 1e-12, "footprint centred on the frame origin");
    assert_eq!((frame.z.x, frame.z.y, frame.z.z), (0.0, 0.0, 1.0));
    assert_eq!(layers.len(), 2);
    assert!((layers[0].grain).abs() < 1e-12 && (layers[1].grain - std::f64::consts::FRAC_PI_2).abs() < 1e-12, "longitudinal then cross layer");
    let scene::Geometry::Fastener(fastener) = geometry("fastener-1:fasteners") else { panic!("fastener") };
    let axis = (fastener.axis.to.x - fastener.axis.from.x, fastener.axis.to.y - fastener.axis.from.y, fastener.axis.to.z - fastener.axis.from.z);
    assert!(axis.0.abs() < 1e-12 && axis.1.abs() < 1e-12 && axis.2 > 0.0, "vertical fastener axis");
    let Some(scene::Pattern::Points { offsets }) = fastener.pattern else { panic!("points") };
    let (e1, e2) = (scene::Vec3::new(0.0, 1.0, 0.0), scene::Vec3::new(-1.0, 0.0, 0.0));
    assert_eq!(cross(&scene::Vec3::new(0.0, 0.0, 1.0), &e1), (e2.x, e2.y, e2.z), "e2 = axis x e1");
    let world = |o: &scene::Vec3| (o.y * e1.x + o.z * e2.x, o.y * e1.y + o.z * e2.y);
    assert_eq!(world(&offsets[0]), (0.0, 0.0));
    let second = world(&offsets[1]);
    assert!((second.0 - 0.084).abs() < 1e-12 && (second.1 - 0.036).abs() < 1e-12, "the second fastener lies +84 mm in X and +36 mm in Y of the first: {second:?}");
}

#[test]
fn beam_shape_cases_expose_angle_and_radius_handles() {
    let mut document = compliant();
    document.cases = vec![
        case_of("tap", "T-30", &[("b", 140.0), ("h", 500.0), ("alpha_deg", 6.0)], &[("class", "GL28h")]),
        case_of("apx", "T-31", &[("b", 160.0), ("h_ap", 900.0), ("r_in", 12000.0), ("alpha_ap_deg", 10.0)], &[("class", "GL28h")]),
    ];
    let (scene, _) = scene_of(&document, &[]);
    for (case, leaves) in [("tap", vec!["alpha_deg"]), ("apx", vec!["alpha_ap_deg", "r_in", "h_ap"])] {
        let group = scene.entities.iter().find(|e| e.entity.id == case).unwrap();
        let hs = handles(&document, &[], &[group.entity.clone()]);
        assert_handle_laws(&hs);
        for leaf in leaves {
            let path = format!("cases[id={case}].numbers[id={leaf}].value");
            let handle = hs.iter().find(|h| h.writes.iter().any(|w| w.path == path)).unwrap_or_else(|| panic!("{path}"));
            if leaf.ends_with("_deg") {
                let write = handle.writes.iter().find(|w| w.path == path).unwrap();
                let shifted = if leaf == "alpha_ap_deg" { 10.0_f64.to_radians() - std::f64::consts::FRAC_PI_2 } else { 10.0_f64.to_radians() };
                assert!((write.map.apply(shifted) - 10.0).abs() < 1e-9, "the angle handle writes degrees from its documented zero");
            }
        }
    }
}

#[test]
fn the_floor_mode_is_memoised_and_follows_the_span() {
    let a = entities::cached_mode(4.5, 12.5e6, 150.0).expect("mode");
    assert_eq!(a, entities::cached_mode(4.5, 12.5e6, 150.0).expect("mode"));
    let b = entities::cached_mode(6.0, 12.5e6, 150.0).expect("mode");
    assert!(b.frequency < a.frequency, "a longer span lowers f1");
}

fn angle_of(handle: &scene::Handle) -> f64 {
    let scene::HandleKind::Angular { axis, centre } = &handle.kind else { panic!("angular") };
    let length = (axis.x * axis.x + axis.y * axis.y + axis.z * axis.z).sqrt();
    let a = (axis.x / length, axis.y / length, axis.z / length);
    let perpendicular = |v: (f64, f64, f64)| { let d = v.0 * a.0 + v.1 * a.1 + v.2 * a.2; (v.0 - d * a.0, v.1 - d * a.1, v.2 - d * a.2) };
    let mut reference = perpendicular((1.0, 0.0, 0.0));
    if reference.0.hypot(reference.1).hypot(reference.2) < 1e-9 {
        reference = perpendicular((0.0, 0.0, 1.0));
    }
    let norm = reference.0.hypot(reference.1).hypot(reference.2);
    let r = (reference.0 / norm, reference.1 / norm, reference.2 / norm);
    let v = (handle.anchor.x - centre.x, handle.anchor.y - centre.y, handle.anchor.z - centre.z);
    let c = (r.1 * v.2 - r.2 * v.1, r.2 * v.0 - r.0 * v.2, r.0 * v.1 - r.1 * v.0);
    (c.0 * a.0 + c.1 * a.1 + c.2 * a.2).atan2(r.0 * v.0 + r.1 * v.1 + r.2 * v.2)
}

/// 🧭️ Law of the `Angular` handle: the angle of the anchor about the axis, measured from the reference `+X` (projected perpendicular to the axis), written through the affine map, is the document value, and lies inside the limits.
#[test]
fn angular_handles_have_the_documented_zero_direction() {
    let mut document = compliant();
    document.cases = vec![
        case_of("tap", "T-30", &[("b", 140.0), ("h", 500.0), ("alpha_deg", 6.0)], &[("class", "GL28h")]),
        case_of("apx", "T-31", &[("b", 160.0), ("h_ap", 900.0), ("r_in", 12000.0), ("alpha_ap_deg", 10.0)], &[("class", "GL28h")]),
    ];
    let (scene, _) = scene_of(&document, &[]);
    let mut seen = 0;
    for (case, leaf, expected) in [("tap", "alpha_deg", 6.0), ("apx", "alpha_ap_deg", 10.0)] {
        let group = scene.entities.iter().find(|e| e.entity.id == case).unwrap();
        for handle in handles(&document, &[], &[group.entity.clone()]).iter().filter(|h| matches!(h.kind, scene::HandleKind::Angular { .. })) {
            let write = handle.writes.iter().find(|w| w.path.ends_with(&format!("[id={leaf}].value"))).unwrap();
            let s = angle_of(handle);
            assert!((write.map.apply(s) - expected).abs() < 1e-6, "{case}: s = {s} writes {}", write.map.apply(s));
            let limits = handle.limits.as_ref().unwrap();
            assert!(s >= limits.min && s <= limits.max, "{case}: s = {s} outside the limits");
            seen += 1;
        }
    }
    assert_eq!(seen, 2);
}

fn special_document() -> En1995Snapshot {
    let mut document = compliant();
    document.cases = vec![
        case_of("jb", "T-90", &[("a1", 4800.0), ("i1", 5.12e6), ("e1", 11000.0), ("h1", 40.0), ("s1", 100.0), ("a2", 9600.0), ("i2", 5.12e6), ("e2", 11000.0), ("h2", 80.0), ("k", 2000.0), ("l", 5000.0), ("m_d", 8e6), ("q_d", 0.1)], &[]),
        case_of("sc", "T-93", &[("lef", 3000.0), ("i_tot", 40.0), ("l1", 600.0), ("i1", 14.0), ("n_parts", 2.0), ("n_c", 60000.0), ("area", 30000.0), ("sc", 1.0)], &[("class", "GL24h"), ("joint", "nailed")]),
        case_of("rp", "T-106", &[("b", 200.0), ("h", 400.0), ("t_req", 60.0), ("m_fi", 60e6), ("sc", 1.0)], &[("class", "GL28h")]),
        case_of("cf", "T-107", &[("n_layers", 5.0), ("t1", 30.0), ("o1", 1.0), ("t2", 30.0), ("o2", 0.0), ("t3", 30.0), ("o3", 1.0), ("t4", 30.0), ("o4", 0.0), ("t5", 30.0), ("o5", 1.0), ("t_req", 90.0), ("m_fi", 2000.0)], &[("class", "CLT100")]),
        case_of("ft", "T-121", &[("ds1", 10.0), ("n1", 1e6), ("ds2", 6.0), ("n2", 4e6), ("sn_a", 12.0), ("sn_b", 4.0), ("years", 50.0)], &[]),
        case_of("dk", "T-122", &[("sigma_p_0", 1.0), ("loss_relax", 0.15), ("loss_creep", 0.25), ("loss_mc", 0.1), ("bar_spacing", 800.0), ("deck_depth", 250.0), ("bar_area", 804.0), ("f_bar_d", 700.0), ("plate_a", 22500.0), ("span_m", 4.0), ("deck_width_m", 1.5)], &[]),
        case_of("cw", "T-120", &[("span_m", 30.0), ("ei_nm2", 8.5e8), ("mass_kg_m", 800.0), ("zeta", 0.03), ("crowd_density", 0.8), ("deck_width_m", 3.0)], &[("crowd_model", "setra")]),
    ];
    document
}

fn catalogue_grips() -> (En1995Snapshot, StructuralScene) {
    let mut document = special_document();
    document.cases.extend([
        case_of("df", "T-94", &[("b_i", 1200.0), ("l_dia", 4.0), ("s", 75.0), ("n_panels", 3.0), ("f_v_ed", 5000.0)], &[("kind", "floor")]),
        case_of("fs", "T-101", &[("b", 200.0), ("h", 400.0), ("t_req", 60.0), ("face_left", 1.0), ("face_right", 1.0), ("face_top", 0.0), ("face_bottom", 1.0), ("m_fi", 60e6)], &[("class", "GL28h")]),
        case_of("tr", "T-95", &[("n_braced", 4.0), ("n_d", 5000.0), ("l_m", 6.0), ("span", 6000.0), ("height", 900.0), ("bays", 6.0), ("chord_b", 100.0), ("chord_h", 200.0), ("web_b", 80.0), ("web_h", 120.0)], &[("kind", "warren"), ("product", "solid")]),
        case_of("br", "T-96", &[("n_braced", 4.0), ("n_d", 5000.0), ("l_m", 6.0), ("f_rd", 2000.0), ("k_br", 500.0), ("spacing", 800.0), ("member_b", 100.0), ("member_h", 200.0)], &[("kind", "plan-bracing")]),
    ]);
    let (scene, _) = scene_of(&document, &[]);
    (document, scene)
}

fn grip_of(document: &En1995Snapshot, scene: &StructuralScene, case: &str, tag: &str) -> scene::Handle {
    let body = scene.entities.iter().find(|e| e.entity.id == case).unwrap_or_else(|| panic!("{case} is in the scene"));
    handles(document, &[], &[body.entity.clone()]).into_iter().find(|h| h.id == format!("{case}:{tag}")).unwrap_or_else(|| panic!("{case} has no grip {tag}"))
}

fn writes_to(handle: &scene::Handle) -> Vec<&str> {
    handle.writes.iter().map(|w| w.path.rsplit("numbers[id=").next().unwrap().trim_end_matches("].value")).collect()
}

#[test]
fn the_width_handle_of_a_jointed_part_writes_area_and_inertia_consistently() {
    let (document, scene) = catalogue_grips();
    let handle = grip_of(&document, &scene, "jb", "a1");
    assert_handle_laws(std::slice::from_ref(&handle));
    assert_eq!(writes_to(&handle), ["a1", "i1"], "one drag writes the area and the inertia of part 1");
    let (a, i) = (handle.writes[0].map.apply(0.01), handle.writes[1].map.apply(0.01));
    assert!((a - 5600.0).abs() < 1e-6, "{a}: 140 x 40 mm");
    assert!((i - 140.0 * 40.0_f64.powi(3) / 12.0).abs() < 1e-3, "{i}");
    assert!((handle.writes[0].map.apply(0.0) - 4800.0).abs() < 1e-9 && (handle.writes[1].map.apply(0.0) - 120.0 * 40.0_f64.powi(3) / 12.0).abs() < 1e-3, "no drag, no change");
}

#[test]
fn every_restored_count_spacing_and_gauge_grip_binds_its_own_input() {
    let (document, scene) = catalogue_grips();
    let bound = |case: &str, tag: &str, leaf: &str| {
        let handle = grip_of(&document, &scene, case, tag);
        assert_handle_laws(std::slice::from_ref(&handle));
        assert_eq!(writes_to(&handle)[0], leaf, "{case}:{tag}");
        handle
    };
    for (case, tag, leaf) in [("jb", "s1", "s1"), ("sc", "n_parts", "n_parts"), ("sc", "area", "area"), ("df", "s", "s"), ("df", "n_panels", "n_panels"), ("ft", "n1", "n1"), ("ft", "n2", "n2"), ("ft", "years", "years"), ("dk", "bar_spacing", "bar_spacing"), ("dk", "plate_a", "plate_a"), ("cw", "crowd_density", "crowd_density"), ("rp", "t_req", "t_req"), ("cf", "t_req", "t_req"), ("cf", "n_layers", "n_layers"), ("fs", "t_req", "t_req"), ("fs", "face_left", "face_left"), ("fs", "face_right", "face_right"), ("fs", "face_top", "face_top"), ("fs", "face_bottom", "face_bottom")] {
        bound(case, tag, leaf);
    }
    for (case, tag) in [("sc", "n_parts"), ("df", "n_panels"), ("cf", "n_layers"), ("fs", "face_top")] {
        let stepper = bound(case, tag, tag);
        assert!(matches!(stepper.kind, scene::HandleKind::CountStepper) && stepper.writes.len() == 1 && stepper.writes[0].quantise == Some(scene::Quantise::Int) && stepper.writes[0].map == scene::Affine::IDENTITY, "{case}:{tag} steps an integer count");
    }
    for (case, tag, grown) in [("jb", "s1", 100.0), ("df", "s", 75.0), ("dk", "bar_spacing", 800.0)] {
        let spacing = bound(case, tag, tag);
        assert!(matches!(spacing.kind, scene::HandleKind::ScalarAlong { .. }), "{case}:{tag} is a linear grip");
        assert!(spacing.writes[0].map.apply(0.1) > grown, "{case}:{tag} widens the spacing when dragged outwards");
    }
    let density = grip_of(&document, &scene, "cw", "crowd_density");
    assert!((density.writes[0].map.apply(0.0) - 0.8).abs() < 1e-12 && density.writes[0].map.apply(1.0) > 0.8, "the gauge grip scales the density from its value");
    let years = grip_of(&document, &scene, "ft", "years");
    assert!(years.writes[0].map.apply(0.1) > 50.0, "a taller tower stands for a longer life");
    let cycles = grip_of(&document, &scene, "ft", "n1");
    assert!(cycles.writes[0].map.apply(0.1) > 1e6, "a taller tower stands for more cycles");
    let burn = grip_of(&document, &scene, "fs", "t_req");
    assert!((burn.writes[0].map.apply(0.0) - 60.0).abs() < 1e-9 && burn.writes[0].map.apply(0.01) > 60.0, "a deeper char front is a longer fire resistance time");
}

#[test]
fn the_declared_truss_and_bracing_details_make_the_catalogue_cases_editable_like_the_typed_entities() {
    let (document, scene) = catalogue_grips();
    let has = |id: &str| scene.entities.iter().any(|e| e.entity.id == id);
    assert!(has("tr:member1") && has("tr:joint1") && has("br:member1") && has("br:diagonal1"), "both cases are drawn from their detail inputs");
    for (case, tag) in [("tr", "span"), ("tr", "height"), ("tr", "bays"), ("tr", "chord_h"), ("tr", "web_h"), ("br", "n_braced"), ("br", "spacing"), ("br", "member_b"), ("br", "member_h")] {
        let handle = grip_of(&document, &scene, case, tag);
        assert_handle_laws(std::slice::from_ref(&handle));
        assert_eq!(writes_to(&handle)[0], tag, "{case}:{tag}");
    }
}

#[test]
fn annex_fire_and_bridge_cases_have_scenes_handles_and_diagrams_with_twins() {
    let document = special_document();
    let (scene, report) = scene_of(&document, &[]);
    assert_eq!(scene.validate(), Ok(()));
    let has = |id: &str| scene.entities.iter().any(|e| e.entity.id == id);
    for id in ["jb", "jb:part1", "jb:fasteners-1", "sc", "sc:part2", "sc:spacer0-0", "cf", "cf:char@60", "cf:residual", "ft", "ft:block1", "ft:limit", "dk", "dk:bar0", "dk:plate0a", "cw:crowd"] {
        assert!(has(id), "{id}");
    }
    let diagrams = diagrams(&document, &report, None);
    for id in ["case-gamma:jb", "case-column:sc", "case-fire-reduced:rp", "case-clt-fire:cf", "case-sn:ft", "case-miner:ft", "case-prestress:dk", "case-crowd:cw"] {
        let diagram = diagrams.iter().find(|d| d.id == id).unwrap_or_else(|| panic!("{id}"));
        assert_eq!(diagram.validate(), Ok(()), "{id}");
        assert!(!diagram.table_twin.columns.is_empty(), "{id}");
    }
    let paths = |case: &str| -> Vec<String> {
        let group = scene.entities.iter().find(|e| e.entity.id == case).unwrap();
        handles(&document, &[], &[group.entity.clone()]).iter().flat_map(|h| h.writes.iter().map(|w| w.path.clone())).collect()
    };
    for (case, leaves) in [("jb", vec!["h1", "h2", "l"]), ("sc", vec!["lef", "l1"]), ("ft", vec!["ds1"]), ("dk", vec!["deck_depth", "deck_width_m", "span_m"]), ("cw", vec!["deck_width_m"])] {
        let all = paths(case);
        for leaf in leaves {
            assert!(all.contains(&format!("cases[id={case}].numbers[id={leaf}].value")), "{case}.{leaf}");
        }
    }
}

#[test]
fn the_clt_char_slabs_follow_the_kernel_fall_off_depth() {
    let document = special_document();
    let (scene, _) = scene_of(&document, &[]);
    let residual = scene.entities.iter().find(|e| e.entity.id == "cf:residual").unwrap();
    let scene::Geometry::Solid(solid) = &residual.geometry else { panic!("solid") };
    let scene::SolidShape::Prism { base, .. } = &solid.shape else { panic!("prism") };
    let (depth, _) = schema::checks::timber::clt_char_depth(&[30.0; 5], 0.65, 90.0, true, 2.0, 25.0);
    let ys: Vec<f64> = base.outer.points.iter().map(|p| p.y).collect();
    let drawn = ys.iter().cloned().fold(f64::MIN, f64::max) - ys.iter().cloned().fold(f64::MAX, f64::min);
    assert!((drawn - depth / 1e3).abs() < 1e-9, "the char slab is an outline as deep as the kernel fall-off depth");
}

#[test]
fn a_member_drag_commits_one_undoable_placement_event_and_reloads_to_the_same_pose() {
    use scene::projection_kit::{placements_of, PlacementCommit, PlacementMove};
    let mut document = compliant();
    let before = placements_of(&document);
    assert!(before.is_empty());
    let moved = PlacementMove { entity_id: "beam-B1".into(), origin: scene::Vec3::new(10.0, 2.0, 0.0), yaw: 0.5 };
    let commit = PlacementCommit::of(&before, std::slice::from_ref(&moved));
    assert_eq!(commit.ops.len(), 1);
    let mut rows = before.clone();
    commit.apply(&mut rows);
    document.placements = rows.iter().map(|p| model::Placement { id: p.entity_id.clone(), x: p.origin.x, y: p.origin.y, z: p.origin.z, yaw: p.yaw }).collect();
    let reloaded = placements_of(&document);
    assert_eq!(reloaded, rows);
    assert_eq!(Layout::of(&document, &reloaded).member("beam-B1").expect("pose").frame.origin, moved.origin);
    assert!(PlacementCommit::of(&reloaded, std::slice::from_ref(&moved)).is_empty());
    let grips = handles(&document, &reloaded, &select("beam-B1", scene::EntityKind::Member, "members"));
    assert_handle_laws(&grips);
    let paths: Vec<&str> = grips.iter().filter(|h| h.id.contains(":move-")).flat_map(|h| h.writes.iter().map(|w| w.path.as_str())).collect();
    assert_eq!(paths, ["placements[id=beam-B1].x", "placements[id=beam-B1].y"]);
    let again = PlacementMove { entity_id: "beam-B1".into(), origin: scene::Vec3::new(11.0, 2.0, 0.0), yaw: 0.5 };
    let second = PlacementCommit::of(&reloaded, std::slice::from_ref(&again));
    assert_eq!(second.ops.len(), 1);
    let mut after = reloaded.clone();
    second.apply(&mut after);
    second.inverse(&reloaded).apply(&mut after);
    assert_eq!(after, reloaded);
    let mut undone = reloaded.clone();
    commit.inverse(&before).apply(&mut undone);
    assert_eq!(undone, before);
}

#[test]
fn the_t106_residual_is_the_exact_char_depth_section_of_the_kernel() {
    let mut document = compliant();
    document.cases = vec![case_of("rp", "T-106", &[("b", 200.0), ("h", 400.0), ("t_req", 60.0), ("m_fi", 60e6), ("sc", 1.0)], &[("class", "GL28h")])];
    let (scene, _) = scene_of(&document, &[]);
    assert_eq!(scene.validate(), Ok(()));
    let residual = scene.entities.iter().find(|e| e.entity.id == "rp:residual").expect("residual");
    let scene::Geometry::Solid(solid) = &residual.geometry else { panic!("solid") };
    let scene::SolidShape::Prism { base, .. } = &solid.shape else { panic!("prism") };
    let product = facts::properties("GL28h").expect("class").product;
    let beta = facts::fire_after(document.annex, product, 0.2, 0.4, 3600.0).beta_n * 6e4;
    let kernel = schema::checks::timber::reduced_properties(200.0, 400.0, [1.0, 1.0, 0.0, 1.0], beta, 60.0);
    assert!((base.area() - kernel.area / 1e6).abs() < 1e-9, "the drawn residual is the d_char,n section, not the d_ef section");
    let (xs, ys): (Vec<f64>, Vec<f64>) = base.outer.points.iter().map(|p| (p.x, p.y)).unzip();
    let extent = |v: &[f64]| v.iter().cloned().fold(f64::MIN, f64::max) - v.iter().cloned().fold(f64::MAX, f64::min);
    assert!((extent(&xs) - kernel.b / 1e3).abs() < 1e-9 && (extent(&ys) - kernel.h / 1e3).abs() < 1e-9);
    assert!(ys.iter().cloned().fold(f64::MIN, f64::max) <= 0.2 + 1e-9, "the protected top face keeps its position");
    assert!(scene.entities.iter().any(|e| e.entity.id == "rp:front@60"), "char front of the exposed faces");
}

#[test]
fn a_floor_case_with_width_draws_its_plate_mode_with_an_acceleration_heat_map() {
    let mut document = compliant();
    document.cases = vec![case_of("fl", "T-65", &[("span_m", 4.5), ("width_m", 4.0), ("ei_l", 1.2e6), ("ei_b", 6.0e4), ("mass_kg_m2", 160.0), ("zeta", 0.03)], &[("class", "C24")])];
    let (scene, _) = scene_of(&document, &[]);
    assert_eq!(scene.validate(), Ok(()));
    let mode = scene.entities.iter().find(|e| e.entity.id == "fl:mode").expect("plate mode");
    assert_eq!(mode.entity.kind, scene::EntityKind::Surface);
    let scene::ColourChannel::Fields { active, fields } = &mode.appearance.channel else { panic!("field channel") };
    assert_eq!(active, "mode-field");
    let scene::FieldSamples::Vertex { values } = &fields[0].samples else { panic!("vertex samples") };
    let overlay = mode.appearance.deformation.as_ref().expect("overlay");
    assert_eq!(overlay.motion, scene::Motion::Harmonic);
    assert_eq!(overlay.surface.len(), values.len());
    let peak = schema::checks::timber::floor_plate_peak_acceleration(4.5, 4.0, 1.2e6, 6.0e4, 160.0);
    let largest = values.iter().cloned().fold(0.0, f64::max);
    assert!(largest <= peak + 1e-12 && largest > 0.95 * peak, "the heat map peaks at the antinode");
    assert!(values.iter().cloned().fold(f64::MAX, f64::min) < 1e-9, "the supports do not accelerate");
    let hz = overlay.frequency.as_ref().expect("frequency").value;
    assert!((hz - schema::checks::timber::floor_plate_f1(4.5, 4.0, 1.2e6, 6.0e4, 160.0)).abs() < 1e-12);
    assert!(scene.legends.iter().any(|l| l.id == "mode:fl"));
    assert!(scene.entities.iter().find(|e| e.entity.id == "fl").is_some_and(|body| matches!(body.geometry, scene::Geometry::Solid(_))), "the floor body is a solid of the form beside its mode surface");
}

#[test]
fn johansen_failure_modes_are_sketched_with_the_governing_mode_highlighted() {
    let mut document = compliant();
    document.cases = vec![case_of("jt", "T-71", &[("d", 12.0), ("t1", 60.0), ("t2", 60.0), ("f_u", 400.0), ("sc", 1.0)], &[("class", "C24"), ("duration", "medium")])];
    for diagram in all_diagrams(&document).into_iter().filter(|d| d.id.starts_with("failure-modes:")) {
        assert_eq!(diagram.validate(), Ok(()), "{}", diagram.id);
        assert!(diagram.table_twin.rows() > 0);
        let governing = diagram.markers.iter().find(|m| m.id == "marker-governing").expect("governing marker");
        let letters: Vec<String> = diagram.series.iter().filter_map(|s| match s { scene::Series::Polygon { id, .. } => id.strip_prefix("cell-").map(str::to_string), _ => None }).collect();
        assert!(letters.len() >= 2);
        let strongest = diagram.table_twin.columns[1].values.iter().cloned().fold(f64::MAX, f64::min);
        let position = diagram.table_twin.columns[1].values.iter().position(|v| (*v - strongest).abs() < 1e-12).expect("lowest capacity");
        assert!((governing.at.x - (position as f64 + 0.5)).abs() < 1e-9, "the marker sits in the lowest-capacity cell {}", diagram.id);
    }
    let ids: BTreeSet<String> = all_diagrams(&document).into_iter().map(|d| d.id).collect();
    assert!(ids.contains("failure-modes:jt") && ids.contains("failure-modes:conn-C1"));
    let single = case_of("js", "T-71", &[("d", 12.0), ("t1", 60.0), ("t2", 100.0), ("f_u", 400.0), ("sc", 1.0)], &[("class", "C24"), ("duration", "medium")]);
    let mut doc = compliant();
    doc.cases = vec![single];
    let letters = all_diagrams(&doc).into_iter().find(|d| d.id == "failure-modes:js").map(|d| d.series.iter().filter(|s| matches!(s, scene::Series::Polyline { id, .. } if id.starts_with("rod-"))).count());
    assert_eq!(letters, Some(6), "modes (a) to (f) of timber-to-timber single shear");
}

fn with_entities() -> En1995Snapshot {
    use model::{Cutout, Fastener, Floor, Panel, PanelLayer, Taper};
    let mut d = compliant();
    d.tapers.push(Taper { id: "taper-1".into(), member_id: "beam-B1".into(), kind: TaperKind::DoubleTapered, taper_angle_rad: 0.1, apex_depth_m: 0.8, inner_radius_m: 15.0, lamella_thickness_m: 0.033, apex_angle_rad: 0.1, tension_side: TensionSide::Tension, rods: 0, rod_resistance_n: 0.0 });
    d.cutouts.push(Cutout { id: "cutout-1".into(), member_id: "beam-B1".into(), kind: CutoutKind::Notch, face: CutoutFace::Bottom, distance_m: 0.0, length_m: 0.15, depth_m: 0.06, centre_height_m: 0.0, slope: 0.0, rods: 0, rod_resistance_n: 0.0 });
    d.cutouts.push(Cutout { id: "cutout-2".into(), member_id: "beam-B1".into(), kind: CutoutKind::Hole, face: CutoutFace::Bottom, distance_m: 1.0, length_m: 0.05, depth_m: 0.0, centre_height_m: 0.2, slope: 0.0, rods: 0, rod_resistance_n: 0.0 });
    d.fasteners.push(Fastener { id: "fastener-1".into(), connection_id: "conn-C1".into(), member_id: "beam-B1".into(), columns: 4, lines: 2, spacing_parallel_m: 0.08, spacing_perpendicular_m: 0.04, end_distance_m: 0.1, edge_distance_m: 0.04, offset_x_m: 0.1, offset_y_m: 0.05, load_angle_rad: 0.0, positions: Vec::new() });
    d.panels.push(Panel { id: "panel-1".into(), strength_class: semio_s_artifact_norm_en1995_model::StrengthClass::C24, layers: (1..=5).map(|k| PanelLayer { id: format!("layer-{k}"), thickness_m: 0.04, orientation: if k % 2 == 1 { LayerOrientation::Longitudinal } else { LayerOrientation::Transverse }, grade: "C24".into() }).collect(), span_m: 4.0, width_m: 1.0, load_pa: 5000.0, fire_duration_s: 3600.0 });
    d.floors.push(Floor { id: "floor-1".into(), member_id: "beam-B1".into(), span_m: 4.0, width_m: 5.0, joist_spacing_m: 0.625, ei_l_n_m2_per_m: 1.0e6, ei_b_n_m2_per_m: 2.0e5, mass_kg_per_m2: 150.0, damping_ratio: 0.02, permanent_load_pa: 1000.0, imposed_load_pa: 2000.0, psi_2: 0.3 });
    d
}

#[test]
fn typed_entities_are_drawn_owned_by_their_row_with_grips_diagrams_and_the_creep_track() {
    let document = with_entities();
    let (scene, report) = scene_of(&document, &[]);
    assert_eq!(scene.validate(), Ok(()), "violations");
    let ids: Vec<&str> = scene.entities.iter().map(|e| e.entity.id.as_str()).collect();
    for row in ["taper-1", "cutout-1", "cutout-2", "fastener-1", "panel-1", "floor-1"] {
        assert!(ids.contains(&row), "{row} is drawn: {ids:?}");
    }
    assert!(scene.entities.iter().any(|e| e.entity.id == "cutout-2:hole" && matches!(&e.geometry, scene::Geometry::Solid(solid) if matches!(solid.shape, scene::SolidShape::Cylinder { .. }))), "the hole is a cylinder");
    assert!(scene.entities.iter().any(|e| e.entity.id == "taper-1:apex"), "the apex zone of the double tapered member is marked");
    assert!(scene.entities.iter().any(|e| e.entity.id == "cutout-1:notch"), "the notch void is drawn");
    assert!(scene.entities.iter().any(|e| e.entity.id == "panel-1:laminate"), "the CLT layup is a laminate");
    let creep: Vec<&scene::SceneEntity> = scene.entities.iter().filter(|e| e.entity.id.starts_with("floor-1:creep@")).collect();
    assert_eq!(creep.len(), CREEP_COUNT, "one deformed floor per creep keyframe");
    assert!(creep.iter().all(|e| matches!(&e.appearance.deformation, Some(overlay) if overlay.motion == scene::Motion::Static && !overlay.surface.is_empty())));
    assert!(scene.tracks.iter().any(|track| track.id == "creep" && track.kind == scene::TrackKind::Age && track.keyframes.len() == CREEP_COUNT), "the age track scrubs the creep layers");
    let hole_drawn = scene.entities.iter().find(|e| e.entity.id == "cutout-2:hole").map(|e| e.layer.clone());
    assert_eq!(hole_drawn.as_deref(), Some("members"));
    let grips = handles(&document, &[], &select("taper-1", scene::EntityKind::Group, "tapers"));
    assert!(grips.iter().any(|h| h.writes.iter().any(|w| w.path == "tapers[id=taper-1].apexDepthM" && w.unit == "m")), "the apex depth grip writes the entity field in metres");
    assert!(grips.iter().all(|h| h.writes.iter().all(|w| !w.path.starts_with("cases["))), "no grip writes a case input");
    let angles: Vec<_> = grips.iter().filter(|h| h.writes.iter().any(|w| w.path == "tapers[id=taper-1].apexAngleRad")).collect();
    assert!(angles.iter().all(|h| h.writes.iter().all(|w| w.unit == "rad")));
    let drawn = diagrams(&document, &report, Some("floor-1"));
    assert!(drawn.iter().any(|d| d.id == "creep-time:floor-1" && d.focus.as_ref().is_some_and(|f| f.path == "floors[id=floor-1]")), "the creep time diagram is offered for the floor");
    let marker_paths: Vec<String> = diagrams(&document, &report, Some("fastener-1")).iter().flat_map(|d| d.markers.iter().filter_map(|m| m.binding.as_ref().map(|b| b.path.clone()))).collect();
    assert!(marker_paths.iter().all(|p| !p.starts_with("cases[")), "{marker_paths:?}");
}

const CREEP_COUNT: usize = 7;

#[test]
fn a_dragged_taper_grip_commits_the_entity_field_and_reprojects_only_that_entity() {
    let document = with_entities();
    let before = entities_of(&document);
    assert!(before.iter().any(|e| e.id == "taper-1") && before.iter().any(|e| e.id == "floor-1"));
    assert_eq!(entity_dependencies(&document, "fastener-1"), ["beam-B1", "conn-C1"]);
    assert_eq!(entity_kind_of(&document, "panel-1").map(|k| k.collection), Some("panels"));
    let mut changed = document.clone();
    changed.tapers[0].apex_depth_m = 1.0;
    let (a, _) = scene_of(&document, &[]);
    let (b, _) = scene_of(&changed, &[]);
    let hash = |scene: &StructuralScene, id: &str| scene.entities.iter().find(|e| e.entity.id == id).map(|e| e.content_hash.clone());
    assert_ne!(hash(&a, "taper-1:body"), hash(&b, "taper-1:body"), "the edited entity changes");
    assert_eq!(hash(&a, "panel-1:laminate"), hash(&b, "panel-1:laminate"), "an untouched entity keeps its content hash");
}

fn grip<'a>(grips: &'a [scene::Handle], entity: &str, tag: &str) -> &'a scene::Handle {
    grips.iter().find(|h| h.id == format!("{entity}:{tag}")).unwrap_or_else(|| panic!("grip {entity}:{tag} in {:?}", grips.iter().map(|h| h.id.clone()).collect::<Vec<_>>()))
}

fn parameter_at_anchor(handle: &scene::Handle, origin: &scene::Vec3) -> f64 {
    let local = [handle.anchor.x - origin.x, handle.anchor.y - origin.y, handle.anchor.z - origin.z];
    match &handle.kind {
        scene::HandleKind::ScalarAlong { axis } => local[0] * axis.x + local[1] * axis.y + local[2] * axis.z,
        scene::HandleKind::FaceOffset { normal } => local[0] * normal.x + local[1] * normal.y + local[2] * normal.z,
        scene::HandleKind::Radial { centre } => handle.anchor.distance(centre),
        other => panic!("no parameter for {other:?}"),
    }
}

#[test]
fn the_hole_grips_drag_the_position_and_the_diameter_of_the_cutout_in_metres() {
    let document = with_entities();
    let origin = Layout::of(&document, &[]).entity("cutout-2").cloned().expect("origin");
    let grips = handles(&document, &[], &select("cutout-2", scene::EntityKind::Group, "cutouts"));
    assert_handle_laws(&grips);
    for (tag, path, current) in [("hole_x", "cutouts[id=cutout-2].distanceM", 1.0), ("hole_z", "cutouts[id=cutout-2].centreHeightM", 0.2), ("hole_d", "cutouts[id=cutout-2].lengthM", 0.05)] {
        let handle = grip(&grips, "cutout-2", tag);
        assert_eq!(handle.writes.len(), 1, "{tag} is one write");
        let write = &handle.writes[0];
        assert_eq!((write.path.as_str(), write.unit.as_str()), (path, "m"));
        assert!((write.map.apply(parameter_at_anchor(handle, &origin)) - current).abs() < 1e-9, "{tag} writes the stored value at its anchor");
    }
    let diameter = grip(&grips, "cutout-2", "hole_d");
    assert!((diameter.writes[0].map.apply(0.04) - 0.08).abs() < 1e-12, "the radial grip writes the diameter");
    let cylinder = scene_of(&document, &[]).0.entities.into_iter().find(|e| e.entity.id == "cutout-2:hole").expect("hole");
    assert_eq!(cylinder.handles.len(), 3, "the hole cylinder carries its three grips");
}

#[test]
fn the_notch_depth_grip_writes_the_depth_through_the_mirrored_effective_depth() {
    let document = with_entities();
    let origin = Layout::of(&document, &[]).entity("cutout-1").cloned().expect("origin");
    let grips = handles(&document, &[], &select("cutout-1", scene::EntityKind::Group, "cutouts"));
    let handle = grip(&grips, "cutout-1", "h_ef");
    let write = &handle.writes[0];
    assert_eq!((write.path.as_str(), write.unit.as_str()), ("cutouts[id=cutout-1].depthM", "m"));
    let depth = document.cutouts[0].depth_m;
    assert!((write.map.apply(parameter_at_anchor(handle, &origin)) - depth).abs() < 1e-9, "the anchor stands at h − depth");
    let h = document.members[0].h_m;
    assert!((write.map.apply(h - 0.1) - 0.1).abs() < 1e-9, "dragging the effective depth to h − 0.1 m writes a depth of 0.1 m");
}

#[test]
fn the_fastener_pattern_editor_drags_rows_and_columns_and_writes_the_spacings() {
    let document = with_entities();
    let origin = Layout::of(&document, &[]).entity("fastener-1").cloned().expect("origin");
    let f = document.fasteners[0].clone();
    let grips = handles(&document, &[], &select("fastener-1", scene::EntityKind::Group, "fasteners"));
    assert_handle_laws(&grips);
    for i in 1..f.columns {
        let handle = grip(&grips, "fastener-1", &format!("col{i}"));
        let write = &handle.writes[0];
        assert_eq!((write.path.as_str(), write.unit.as_str()), ("fasteners[id=fastener-1].spacingParallelM", "m"));
        assert!((write.map.apply(parameter_at_anchor(handle, &origin)) - f.spacing_parallel_m).abs() < 1e-9, "column {i} stands at a₃ + {i}·a₁");
        let s = f.end_distance_m + f64::from(i) * 0.1;
        assert!((write.map.apply(s) - 0.1).abs() < 1e-9, "dragging column {i} to a₁ = 0.1 m");
    }
    for j in 1..f.lines {
        let handle = grip(&grips, "fastener-1", &format!("row{j}"));
        let write = &handle.writes[0];
        assert_eq!(write.path, "fasteners[id=fastener-1].spacingPerpendicularM");
        assert!((write.map.apply(parameter_at_anchor(handle, &origin)) - f.spacing_perpendicular_m).abs() < 1e-9);
    }
    assert!(grips.iter().all(|h| h.id != format!("fastener-1:col{}", f.columns) && h.id != format!("fastener-1:row{}", f.lines)), "no grip beyond the pattern");
    assert_eq!(grip(&grips, "fastener-1", "n").writes[0].path, "fasteners[id=fastener-1].columns");
    assert_eq!(grip(&grips, "fastener-1", "rows").writes[0].path, "fasteners[id=fastener-1].lines");
    assert_eq!(grip(&grips, "fastener-1", "a3t").writes[0].path, "fasteners[id=fastener-1].endDistanceM");
    assert_eq!(grip(&grips, "fastener-1", "a4t").writes[0].path, "fasteners[id=fastener-1].edgeDistanceM");
    assert!(grips.iter().all(|h| h.writes.iter().all(|w| !w.path.starts_with("cases["))), "no grip writes a case input");
    let drawn = scene_of(&document, &[]).0.entities.into_iter().find(|e| e.entity.id == "fastener-1:fasteners").expect("fasteners");
    for tag in ["col1", "col3", "row1", "n", "rows", "a1", "a2"] {
        assert!(drawn.handles.iter().any(|h| h == &format!("fastener-1:{tag}")), "the fastener part carries {tag}");
    }
}

#[test]
fn every_clt_layer_has_a_thickness_grip_that_writes_its_own_layer_row() {
    let document = with_entities();
    let grips = handles(&document, &[], &select("panel-1", scene::EntityKind::Group, "panels"));
    assert_handle_laws(&grips);
    let origin = Layout::of(&document, &[]).entity("panel-1").cloned().expect("origin");
    for (index, layer) in document.panels[0].layers.iter().enumerate() {
        let handle = grip(&grips, "panel-1", &format!("t{}", index + 1));
        let write = &handle.writes[0];
        assert_eq!((write.path.clone(), write.unit.as_str()), (format!("panels[id=panel-1].layers[id={}].thicknessM", layer.id), "m"));
        let s = parameter_at_anchor(handle, &origin);
        assert!((write.map.apply(s) - layer.thickness_m).abs() < 1e-9 && (write.map.apply(s + 0.01) - layer.thickness_m - 0.01).abs() < 1e-9, "layer {} grip", index + 1);
    }
    assert!(grips.iter().all(|h| h.writes.iter().all(|w| !w.path.starts_with("cases["))), "no grip writes a case input");
}

#[test]
fn the_laminate_follows_the_layer_rows_in_thickness_and_grain_direction() {
    let mut document = with_entities();
    document.panels[0].layers[1].orientation = LayerOrientation::Longitudinal;
    document.panels[0].layers[2].thickness_m = 0.06;
    let grain = |document: &En1995Snapshot| {
        let drawn = scene_of(document, &[]).0.entities.into_iter().find(|e| e.entity.id == "panel-1:laminate").expect("laminate");
        let scene::Geometry::Solid(solid) = drawn.geometry else { panic!("solid") };
        let scene::SolidShape::Laminate { layers, .. } = solid.shape else { panic!("laminate") };
        layers.into_iter().map(|l| (l.thickness, l.grain)).collect::<Vec<_>>()
    };
    let layers = grain(&document);
    assert_eq!(layers.len(), 5);
    assert!((layers[2].0 - 0.06).abs() < 1e-9 && layers.iter().all(|l| l.1.abs() < 1e-12));
    document.panels[0].layers[1].orientation = LayerOrientation::Transverse;
    assert!((grain(&document)[1].1 - std::f64::consts::FRAC_PI_2).abs() < 1e-12, "a transverse layer lies at a right angle");
}

fn irregular(document: &mut En1995Snapshot) {
    use model::FastenerPosition;
    document.fasteners[0].positions = vec![
        FastenerPosition { id: "p1".into(), x_m: 0.10, y_m: 0.04 },
        FastenerPosition { id: "p2".into(), x_m: 0.18, y_m: 0.04 },
        FastenerPosition { id: "p3".into(), x_m: 0.30, y_m: 0.09 },
    ];
}

#[test]
fn explicit_fastener_positions_get_a_grip_per_coordinate_and_replace_the_regular_pattern() {
    let mut document = with_entities();
    irregular(&mut document);
    let grips = handles(&document, &[], &select("fastener-1", scene::EntityKind::Group, "fasteners"));
    assert_handle_laws(&grips);
    let origin = Layout::of(&document, &[]).entity("fastener-1").cloned().expect("origin");
    for (k, position) in document.fasteners[0].positions.iter().enumerate() {
        let hx = grip(&grips, "fastener-1", &format!("px{}", k + 1));
        assert_eq!(hx.writes[0].path, format!("fasteners[id=fastener-1].positions[id={}].xM", position.id));
        assert!((hx.writes[0].map.apply(parameter_at_anchor(hx, &origin)) - position.x_m).abs() < 1e-9, "the grip stands at the position");
        let hy = grip(&grips, "fastener-1", &format!("py{}", k + 1));
        assert_eq!(hy.writes[0].path, format!("fasteners[id=fastener-1].positions[id={}].yM", position.id));
    }
    assert!(grips.iter().all(|h| !h.id.ends_with(":col1") && !h.id.ends_with(":row1") && !h.id.ends_with(":a1")), "an irregular pattern has no spacing to drag");
    let (scene, _) = scene_of(&document, &[]);
    let drawn = scene.entities.iter().find(|e| e.entity.id == "fastener-1:fasteners").expect("fasteners");
    assert!(matches!(&drawn.geometry, scene::Geometry::Fastener(f) if matches!(&f.pattern, Some(scene::Pattern::Points { offsets }) if offsets.len() == 3)), "one instance per explicit position");
}

#[test]
fn the_spacing_colouring_of_an_irregular_pattern_follows_the_actual_pairwise_distances() {
    let mut document = with_entities();
    irregular(&mut document);
    document.fasteners[0].positions[1].x_m = 0.10 + 0.004;
    let tight = spacing_demands_of(&document);
    assert_eq!(tight.len(), 3);
    assert!(tight[0] > 1.0 && tight[1] > 1.0, "two fasteners 4 mm apart in one row violate a₁");
    document.fasteners[0].positions[1].x_m = 0.10 + 0.5;
    let wide = spacing_demands_of(&document);
    assert!(wide[0] < tight[0] && wide[1] < tight[1], "moving the neighbour away lowers the demand");
    let demand = |positions: &[(f64, f64)]| schema::checks::timber::pairwise_demand(0, positions, Some(0.1), Some(0.05));
    assert!(demand(&[(0.0, 0.0), (0.2, 0.0)]) < 1.0, "a neighbour 200 mm away in the row keeps a₁ = 100 mm");
    assert!(demand(&[(0.0, 0.0), (0.001, 0.0)]) > 1.0, "a neighbour 1 mm away violates both a₁ and a₂");
    assert!(demand(&[(0.0, 0.0), (0.02, 0.06)]) < 1.0, "a neighbour 60 mm across keeps a₂ = 50 mm even when staggered closely along the grain");
}

fn spacing_demands_of(document: &En1995Snapshot) -> Vec<f64> {
    let (scene, _) = scene_of(document, &[]);
    let drawn = scene.entities.iter().find(|e| e.entity.id == "fastener-1:fasteners").expect("fasteners");
    assert!(matches!(&drawn.geometry, scene::Geometry::Fastener(f) if matches!(&f.pattern, Some(scene::Pattern::Points { offsets }) if offsets.len() as u32 == if document.fasteners[0].positions.is_empty() { document.fasteners[0].columns * document.fasteners[0].lines } else { document.fasteners[0].positions.len() as u32 })), "one instance per fastener");
    let scene::ColourChannel::Fields { fields, .. } = &drawn.appearance.channel else { panic!("per-fastener colour field") };
    let scene::FieldSamples::Instance { values } = &fields[0].samples else { panic!("one value per instance") };
    values.clone()
}

#[test]
fn the_minimum_spacing_feedback_colours_each_fastener_live() {
    let mut document = with_entities();
    let count = (document.fasteners[0].columns * document.fasteners[0].lines) as usize;
    document.fasteners[0].spacing_parallel_m = 0.5;
    document.fasteners[0].spacing_perpendicular_m = 0.5;
    document.fasteners[0].end_distance_m = 0.5;
    document.fasteners[0].edge_distance_m = 0.5;
    let roomy = spacing_demands_of(&document);
    assert_eq!(roomy.len(), count);
    assert!(roomy.iter().all(|v| v.is_finite() && *v > 0.0 && *v < 1.0), "generous spacings keep every fastener below the minimum demand: {roomy:?}");
    document.fasteners[0].spacing_parallel_m = 0.005;
    let tight = spacing_demands_of(&document);
    assert!(tight.iter().all(|v| *v > 1.0), "a₁ of 5 mm violates the minimum of every fastener that has a neighbour along the grain: {tight:?}");
    assert!(tight.iter().zip(&roomy).all(|(a, b)| a >= b), "tightening never relaxes a fastener");
    let (scene, _) = scene_of(&document, &[]);
    let detail = &scene.entities.iter().find(|e| e.entity.id == "fastener-1:fasteners").expect("fasteners").accessible_detail;
    assert!(detail.en.contains("violated") && detail.de.contains("unterschritten"), "{detail:?}");
    assert!(scene.legends.iter().any(|l| l.id == "spacing:fastener-1"), "the colour key is offered");
    document.fasteners[0].spacing_parallel_m = 0.08;
    document.fasteners[0].columns = 6;
    assert_eq!(spacing_demands_of(&document).len(), 12, "adding columns adds instances");
}

#[test]
fn every_example_and_showcase_projection_obeys_the_scene_diagram_and_handle_laws() {
    let demos = [schema::checks::timber::catalogue_demo_document(), schema::checks::timber::entity_demo_document()];
    for document in fixtures().into_iter().chain([burning()]).chain(demos) {
        let (scene, report) = scene_of(&document, &[]);
        let selection: Vec<scene::EntityRef> = scene.entities.iter().map(|e| e.entity.clone()).collect();
        scene::projection_kit::test_support::assert_projection_laws(&scene, &diagrams(&document, &report, None), &handles(&document, &[], &selection));
    }
}

#[test]
fn a_document_with_two_combinations_offers_the_combination_track() {
    for document in [compliant(), failing()] {
        let (scene, _) = scene_of(&document, &[]);
        let distinct = facts::combination_captions(&document).into_iter().map(|(en, _)| en).collect::<std::collections::BTreeSet<_>>().len();
        scene::projection_kit::test_support::assert_combination_track_law(&scene, distinct);
    }
}

#[test]
fn a_stress_laminated_deck_is_drawn_with_laminations_prestress_bars_anchor_plates_and_a_wheel_patch() {
    let document = schema::checks::timber::entity_demo_document();
    let (scene, _) = scene_of(&document, &[]);
    let ids: BTreeSet<String> = scene.entities.iter().map(|e| e.entity.id.clone()).collect();
    for id in ["deck-1:laminations", "deck-1:bar0", "deck-1:bar9", "deck-1:plate0a", "deck-1:plate9b", "deck-1:wheel-patch"] {
        assert!(ids.contains(id), "{id} is drawn");
    }
    assert!(!ids.contains("deck-1:bar10"), "bars stand every 0.8 m inside the 8 m span");
    for entity in scene.entities.iter().filter(|e| e.entity.id.starts_with("deck-1:bar")) {
        assert!(matches!(&entity.geometry, scene::Geometry::Fastener(f) if (f.diameter - 0.032).abs() < 1e-12), "the bars are as thick as the bar diameter of the deck");
    }
    assert!(scene.entities.iter().any(|e| e.entity.id.starts_with("deck-1:") && !e.handles.is_empty()), "the deck has grips");
    let mut wider = document.clone();
    wider.decks[0].bar_spacing_m = 0.4;
    let (scene, _) = scene_of(&wider, &[]);
    assert!(scene.entities.iter().any(|e| e.entity.id == "deck-1:bar19"), "halving the bar spacing doubles the bars");
}

#[test]
fn a_diaphragm_a_truss_and_a_bracing_are_drawn_with_their_members_joints_and_stabilised_members() {
    let document = schema::checks::timber::entity_demo_document();
    let (scene, _) = scene_of(&document, &[]);
    let ids: BTreeSet<String> = scene.entities.iter().map(|e| e.entity.id.clone()).collect();
    for id in ["diaphragm-1:panel1", "diaphragm-1:panel2", "diaphragm-1:fastener-lines", "truss-1:member1", "truss-1:joint1", "bracing-1:member1", "bracing-1:member4", "bracing-1:force1", "bracing-1:force4", "bracing-1:diagonal1", "bracing-1:diagonal2"] {
        assert!(ids.contains(id), "{id} is drawn");
    }
    let (joints, _) = schema::checks::timber::truss_layout(model::TrussKind::Warren, 12.0, 1.5, 6);
    assert_eq!(scene.entities.iter().filter(|e| e.entity.id.starts_with("truss-1:joint")).count(), joints.len());
    for entity in ["diaphragm-1", "truss-1", "bracing-1"] {
        assert!(scene.entities.iter().any(|e| e.entity.id.starts_with(&format!("{entity}:")) && !e.handles.is_empty()), "{entity} has grips");
    }
}

#[test]
fn component_diagrams_project_the_tapers_notches_diaphragms_trusses_bracings_and_decks_and_an_edit_changes_only_its_own() {
    let mut document = schema::checks::timber::entity_demo_document();
    let all = |document: &En1995Snapshot| diagrams(document, &report_of(document), None);
    let before = all(&document);
    for id in ["case-taper:taper-2", "case-notch:cutout-1", "case-racking:diaphragm-1", "case-truss-chord:truss-1", "case-bracing:bracing-1", "case-wheel-spread:deck-1"] {
        let diagram = before.iter().find(|d| d.id == id).unwrap_or_else(|| panic!("{id}"));
        assert!(!diagram.markers.is_empty() && diagram.table_twin.columns.len() >= 2 && diagram.table_twin.columns.iter().all(|c| c.values.len() == diagram.table_twin.columns[0].values.len()), "{id}");
    }
    let series_at = |d: &scene::Diagram2d, id: &str| d.series.iter().find_map(|s| match s { scene::Series::Polyline { id: i, points, .. } if i == id => Some(points.clone()), _ => None }).unwrap_or_else(|| panic!("{id}"));
    let taper = before.iter().find(|d| d.id == "case-taper:taper-2").unwrap();
    assert!((series_at(taper, "km-alpha")[0].y - 1.0).abs() < 1e-12, "k_m,α is 1 without a taper");
    assert!(taper.markers[0].binding.as_ref().is_some_and(|b| b.path == "tapers[id=taper-2].taperAngleRad"), "the taper angle marker writes the entity field");
    let notch = before.iter().find(|d| d.id == "case-notch:cutout-1").unwrap();
    assert!((series_at(notch, "k-v").last().unwrap().y - 1.0).abs() < 1e-12, "k_v is 1 without a notch");
    let racking = before.iter().find(|d| d.id == "case-racking:diaphragm-1").unwrap();
    let panel = &racking.markers[0].at;
    assert!((panel.x - 1200.0).abs() < 1e-9 && (panel.y - 2.0 * 3000.0 * 1200.0 / 75.0).abs() < 1e-6, "ΣF_i,v,Rd = n F_f,Rd b_i c_i / s: {panel:?}");
    let bracing = before.iter().find(|d| d.id == "case-bracing:bracing-1").unwrap();
    let deflection = series_at(bracing, "deflection");
    assert!(deflection.windows(2).all(|w| w[1].y < w[0].y), "stiffer bracing deflects less");
    document.tapers[1].taper_angle_rad += 0.01;
    let after = all(&document);
    assert_eq!(before.len(), after.len());
    for (b, a) in before.iter().zip(&after) {
        assert_eq!(b.id, a.id);
        if !b.id.ends_with(":taper-2") {
            assert_eq!(b, a, "{} is untouched by the taper edit", b.id);
        }
    }
    assert_ne!(before.iter().find(|d| d.id == "case-taper:taper-2"), after.iter().find(|d| d.id == "case-taper:taper-2"));
    assert_eq!(after, all(&document.clone()), "diagrams are a pure function of the document");
}

#[test]
fn response_curves_project_columns_beams_deflections_bolts_and_claddings_and_their_markers_write_the_case_leaves() {
    let mut document = compliant();
    document.cases = vec![
        case_of("col", "T-20", &[("b", 100.0), ("h", 100.0), ("lef_y", 3000.0), ("lef_z", 4500.0), ("n_c", 10_000.0)], &[("class", "C24"), ("duration", "medium")]),
        case_of("ltb", "T-22", &[("b", 120.0), ("h", 400.0), ("lef", 9000.0), ("m_y", 30e6)], &[("class", "C24")]),
        case_of("def", "T-61", &[("b", 100.0), ("h", 200.0), ("l", 4000.0), ("load_g", 1.5), ("load_q", 2.0), ("k_def", 0.6), ("w_c", 5.0)], &[("class", "C24")]),
        case_of("emb", "T-70", &[("d", 12.0), ("rho_k1", 350.0), ("f_u", 400.0), ("alpha_deg", 30.0)], &[("kind", "bolt")]),
        case_of("nail", "T-70", &[("d", 3.0), ("rho_k1", 350.0), ("f_u", 600.0)], &[("kind", "nail")]),
        case_of("clad", "T-103", &[("h_p", 20.0), ("t_min", 60.0)], &[("class", "C24")]),
    ];
    let all = diagrams(&document, &report_of(&document), None);
    let find = |id: &str| all.iter().find(|d| d.id == id).unwrap_or_else(|| panic!("{id}"));
    let series_of = |d: &scene::Diagram2d, id: &str| d.series.iter().find_map(|s| match s { scene::Series::Polyline { id: i, points, .. } if i == id => Some(points.clone()), _ => None }).unwrap_or_else(|| panic!("{id}"));
    for id in ["case-buckling:col", "case-ltb:ltb", "case-deflection:def", "case-embedment:emb", "case-cladding:clad"] {
        let d = find(id);
        assert!(!d.markers.is_empty() && d.table_twin.columns.len() >= 2 && d.table_twin.columns.iter().all(|c| c.values.len() == d.table_twin.columns[0].values.len()), "{id}");
    }
    assert!(all.iter().all(|d| d.id != "case-embedment:nail"), "a nail has no angle dependence to draw");
    let buckling = find("case-buckling:col");
    let (y, z) = (&buckling.markers[0], &buckling.markers[1]);
    assert!((y.at.x - 1.7622).abs() < 1e-3 && (y.at.y - 0.28457).abs() < 1e-4 && (z.at.y - 0.1328).abs() < 5e-4, "{:?} {:?}", y.at, z.at);
    assert!(y.binding.as_ref().is_some_and(|b| b.path == "cases[id=col].numbers[id=lef_y].value" && (b.map.apply(y.at.x) - 3000.0).abs() < 1e-6), "the axis marker writes lef_y");
    assert!(z.binding.as_ref().is_some_and(|b| (b.map.apply(z.at.x) - 4500.0).abs() < 1e-6), "the axis marker writes lef_z");
    let kc = series_of(buckling, "k-c");
    assert!((kc[0].y - 1.0).abs() < 1e-12 && kc.windows(2).all(|w| w[1].y <= w[0].y + 1e-12), "k_c starts at 1 and never rises");
    let ltb = find("case-ltb:ltb");
    assert!((ltb.markers[0].at.x - 9000.0).abs() < 1e-9 && (ltb.markers[0].at.y - 0.79533).abs() < 1e-4 && ltb.markers[0].binding.as_ref().is_some_and(|b| b.path == "cases[id=ltb].numbers[id=lef].value"));
    assert!(series_of(ltb, "k-crit").windows(2).all(|w| w[1].y <= w[0].y + 1e-12), "a longer span buckles earlier");
    let deflection = find("case-deflection:def");
    let w_fin = series_of(deflection, "w-fin");
    let slope = (w_fin[w_fin.len() - 1].y - w_fin[0].y) / (w_fin[w_fin.len() - 1].x - w_fin[0].x);
    let marker = &deflection.markers[0].at;
    assert!((marker.x - 0.6).abs() < 1e-12 && (marker.y - (w_fin[0].y + slope * 0.6)).abs() < 1e-9, "the marker sits on the straight w_fin line");
    assert!((series_of(deflection, "w-net-fin")[0].y - (w_fin[0].y - 5.0)).abs() < 1e-9, "the camber lowers the net curve");
    let embedment = find("case-embedment:emb");
    let f0 = 0.082 * 0.88 * 350.0;
    let fh = series_of(embedment, "f-h");
    assert!((fh[0].y - f0).abs() < 1e-9 && (fh[fh.len() - 1].y - f0 / 1.53).abs() < 1e-9, "Hankinson at 0 and 90 degrees");
    assert!(embedment.markers[0].binding.as_ref().is_some_and(|b| b.path == "cases[id=emb].numbers[id=alpha_deg].value"));
    let cladding = find("case-cladding:clad");
    let protected = series_of(cladding, "protected");
    assert!(protected.iter().filter(|q| q.x <= 42.0).all(|q| q.y.abs() < 1e-12) && (cladding.markers[0].at.y - 2.0 * 0.65 * 18.0).abs() < 1e-9, "no char before t_ch, 2 beta_0 after the board");
    assert!(series_of(cladding, "bare").iter().all(|q| (q.y - 0.65 * q.x).abs() < 1e-9));
    assert!(cladding.markers[0].binding.as_ref().is_some_and(|b| b.path == "cases[id=clad].numbers[id=t_min].value"));
    let (scene, _) = scene_of(&document, &[]);
    assert_eq!(scene.validate(), Ok(()));
    assert!(scene.entities.iter().any(|e| e.entity.id.starts_with("clad")), "the protected member case is drawn");
}

#[test]
fn placements_are_a_registered_kind_whose_rows_resolve_when_no_entity_owns_the_id() {
    let mut document = compliant();
    let spec = ENTITY_KINDS.iter().find(|k| k.id == "placement").expect("placement kind");
    assert_eq!((spec.collection, spec.label_en, spec.label_de), ("placements", "Placement", "Platzierung"));
    let member = document.members[0].id.clone();
    document.placements = vec![model::Placement { id: member.clone(), x: 1.0, y: 0.0, z: 0.0, yaw: 0.0 }, model::Placement { id: "orphan".into(), x: 0.0, y: 0.0, z: 0.0, yaw: 0.0 }];
    assert_eq!(entity_kind_of(&document, &member).map(|k| k.id), Some("member"), "a placement follows the entity it moves");
    assert_eq!(entity_kind_of(&document, "orphan").map(|k| k.id), Some("placement"));
}

#[test]
fn a_timber_concrete_composite_draws_its_slab_stress_profile_and_slip_curve() {
    let mut document = compliant();
    document.cases = vec![case_of("tcc", "T-90", &[("a1", 48_000.0), ("i1", 25.6e6), ("e1", 33_000.0), ("h1", 80.0), ("s1", 150.0), ("a2", 28_800.0), ("i2", 138.24e6), ("e2", 11_500.0), ("h2", 240.0), ("k", 15_000.0), ("l", 6000.0), ("m_d", 20e6), ("v_d", 10_000.0)], &[])];
    let all = diagrams(&document, &report_of(&document), None);
    let find = |id: &str| all.iter().find(|d| d.id == id).unwrap_or_else(|| panic!("{id}"));
    let series_of = |d: &scene::Diagram2d, id: &str| d.series.iter().find_map(|s| match s { scene::Series::Polyline { id: i, points, .. } if i == id => Some(points.clone()), _ => None }).unwrap_or_else(|| panic!("{id}"));
    let stress = find("case-joint-stress:tcc");
    let slip = series_of(stress, "slip");
    assert_eq!(slip.len(), 4);
    assert!((slip[0].y).abs() < 1e-12 && (slip[1].y - 240.0).abs() < 1e-12 && (slip[3].y - 320.0).abs() < 1e-12, "timber from 0 to 240 mm, the slab from 240 to 320 mm");
    assert!((slip[3].x + 5.7193).abs() < 1e-3 && (slip[2].x - 2.4801).abs() < 1e-3 && (slip[1].x + 1.5867).abs() < 1e-3 && (slip[0].x - 6.9854).abs() < 1e-3);
    assert!(stress.table_twin.columns.len() == 3 && stress.table_twin.columns.iter().all(|c| c.values.len() == 4));
    let curve = find("case-joint-slip:tcc");
    let u = series_of(curve, "slip");
    assert!((u[0].y - 0.38871).abs() < 1e-4 && (u[u.len() - 1].y - u[0].y).abs() < 1e-9 && u.iter().map(|q| q.y).fold(f64::INFINITY, f64::min) < 0.1 * u[0].y && (curve.markers[0].at.y - 0.38871).abs() < 1e-4);
    assert!(curve.markers[0].binding.as_ref().is_some_and(|b| b.path == "cases[id=tcc].numbers[id=k].value"));
    let (scene, _) = scene_of(&document, &[]);
    assert_eq!(scene.validate(), Ok(()));
}

#[test]
fn the_basis_badge_follows_the_document_annex_in_the_scene_and_every_diagram() {
    scene::projection_kit::test_support::assert_basis_follows_annex(norm::NormFamilyId::En1995, |annex| {
        let mut document = compliant();
        document.annex = annex;
        let (scene, report) = scene_of(&document, &[]);
        (scene, diagrams(&document, &report, None))
    });
}
