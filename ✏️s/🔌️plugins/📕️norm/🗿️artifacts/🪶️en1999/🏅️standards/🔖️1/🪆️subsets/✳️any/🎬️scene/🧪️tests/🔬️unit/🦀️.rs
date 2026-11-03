use super::*;
use scene::projection_kit::test_support::{revision, entity, assert_handle_laws};
use scene::{canonical_json, Validate};
use std::collections::BTreeSet;

fn scene_of(document: &En1999Snapshot, placements: &[Placement]) -> (StructuralScene, CheckReport) {
    let report = checks::evaluate_structure(document);
    let scene = project(&SceneInput { document, report: &report, placements, revision: revision() });
    (scene, report)
}

fn compliant() -> En1999Snapshot {
    En1999Snapshot::compliant_roof_purlin()
}

fn failing() -> En1999Snapshot {
    En1999Snapshot::noncompliant_multi_fail()
}

fn member_ref(id: &str) -> scene::EntityRef {
    scene::EntityRef::new(scene::EntityKind::Member, id, format!("members[id={id}]"))
}

#[test]
fn the_scene_of_every_fixture_validates() {
    for document in [compliant(), failing()] {
        let (scene, _) = scene_of(&document, &[]);
        assert_eq!(scene.validate(), Ok(()), "violations");
        assert_eq!(scene.schema, scene::SCENE_SCHEMA_ID);
    }
}

#[test]
fn ids_are_unique_and_layers_exist() {
    let (scene, _) = scene_of(&compliant(), &[]);
    let ids: BTreeSet<_> = scene.entities.iter().map(|e| e.entity.id.clone()).collect();
    assert_eq!(ids.len(), scene.entities.len());
    let annotation_ids: BTreeSet<_> = scene.annotations.iter().map(|a| a.id().to_string()).collect();
    assert_eq!(annotation_ids.len(), scene.annotations.len());
    let layers: BTreeSet<_> = scene.layers.iter().map(|l| l.id.as_str()).collect();
    assert!(scene.entities.iter().all(|e| layers.contains(e.layer.as_str())));
}

#[test]
fn projection_is_deterministic_and_locale_free() {
    let (a, _) = scene_of(&compliant(), &[]);
    let (b, _) = scene_of(&compliant(), &[]);
    assert_eq!(canonical_json(&a), canonical_json(&b));
    assert!(a.entities.iter().all(|e| !e.accessible_name.en.is_empty() && !e.accessible_name.de.is_empty()));
}

#[test]
fn an_extruded_i_section_matches_its_closed_form_area() {
    let document = compliant();
    let (scene, _) = scene_of(&document, &[]);
    let scene::Geometry::Member(geometry) = &entity(&scene, "purlin-1").geometry else { panic!("member geometry") };
    let s = &document.sections[0];
    let closed = 2.0 * s.width * s.flange_thickness + (s.height - 2.0 * s.flange_thickness) * s.web_thickness;
    assert!((geometry.section.area().expect("area") - closed).abs() / closed < 1e-9);
    let scene::Path3::Polyline(axis) = &geometry.axis else { panic!("polyline axis") };
    assert!((axis.points[0].distance(&axis.points[1]) - document.members[0].length).abs() < 1e-9);
    assert_eq!(entity(&scene, "purlin-1").entity.path, "members[id=purlin-1]");
}

#[test]
fn a_box_section_is_a_polygon_with_a_hole() {
    let mut document = compliant();
    document.sections[0].kind = semio_s_artifact_norm_en1999_model::SectionKind::Rhs;
    let (scene, _) = scene_of(&document, &[]);
    let scene::Geometry::Member(geometry) = &entity(&scene, "purlin-1").geometry else { panic!("member geometry") };
    let scene::SectionShape::Polygon { polygon } = &geometry.section else { panic!("polygon section") };
    assert_eq!(polygon.holes.len(), 1);
    let s = &document.sections[0];
    let closed = s.width * s.height - (s.width - 2.0 * s.web_thickness) * (s.height - 2.0 * s.flange_thickness);
    assert!((geometry.section.area().expect("area") - closed).abs() / closed < 1e-9);
}

#[test]
fn the_default_layout_runs_members_left_to_right_and_stands_cantilevers_upright() {
    let document = compliant();
    let layout = Layout::of(&document, &[]);
    let first = layout.member("purlin-1").expect("first");
    let post = layout.member("post-chs").expect("post");
    assert_eq!(first.frame.origin, scene::Vec3::ZERO);
    assert_eq!(first.frame.x, scene::Vec3::X);
    assert!((post.frame.origin.x - (document.members[0].length + layout::MEMBER_GAP)).abs() < 1e-12);
    assert_eq!(post.frame.x, scene::Vec3::Z);
}

#[test]
fn a_persisted_placement_overrides_the_default_and_rotates_about_z() {
    let placement = Placement { entity_id: "purlin-1".into(), origin: scene::Vec3::new(10.0, 2.0, 0.0), yaw: std::f64::consts::FRAC_PI_2 };
    let layout = Layout::of(&compliant(), &[placement]);
    let pose = layout.member("purlin-1").expect("pose");
    assert_eq!(pose.frame.origin, scene::Vec3::new(10.0, 2.0, 0.0));
    assert!(pose.frame.x.y > 0.999 && pose.frame.x.x.abs() < 1e-9);
}

#[test]
fn utilisation_colouring_follows_the_governing_check() {
    let (scene, report) = scene_of(&compliant(), &[]);
    let member = entity(&scene, "purlin-1");
    let binding = member.utilisation.as_ref().expect("binding");
    let expected = report.checks.iter().filter(|c| c.status != norm::CheckStatus::NotApplicable && c.utilization.is_finite()).filter(|c| ["purlin-1", "fat-1", "fire-1"].contains(&c.subject.entity_id.as_str())).map(|c| c.utilization).fold(0.0_f64, f64::max);
    assert!((binding.ratio - expected).abs() < 1e-12);
    assert_eq!(member.appearance.channel, scene::ColourChannel::Utilisation);
}

#[test]
fn a_failing_member_gets_error_style_hatch_failure_glyphs_and_a_mode_shape() {
    let (scene, _) = scene_of(&failing(), &[]);
    let member = entity(&scene, "beam-fail");
    assert_eq!(member.appearance.style, scene::StyleState::Error);
    assert_eq!(member.appearance.hatch, Some(scene::Hatch::Diagonal));
    assert!(scene.annotations.iter().any(|a| matches!(a, scene::Annotation::FailureGlyph { .. })));
    let overlay = scene.entities.iter().filter(|e| e.entity.id.starts_with("beam-fail:mode-")).find_map(|e| e.appearance.deformation.as_ref()).expect("a buckling check shows its mode on a mode layer");
    let _ = member;
    assert_eq!(overlay.kind, scene::DeformationKind::Mode);
    assert!(overlay.samples.first().is_some_and(|s| s.delta.length() < 1e-12) && overlay.samples.last().is_some_and(|s| s.delta.length() < 1e-9));
}

#[test]
fn haz_bands_cover_exactly_the_reduced_strips_of_the_welded_plates() {
    let document = compliant();
    let (scene, _) = scene_of(&document, &[]);
    let hazs: Vec<_> = scene.entities.iter().filter(|e| e.entity.id.starts_with("purlin-1:haz-")).collect();
    assert_eq!(hazs.len(), 5, "two sides of two flanges and the web root");
    let skin = 2.0 * 0.0005;
    let mut area = 0.0;
    for haz in &hazs {
        assert_eq!(haz.appearance.role, scene::MaterialRole::Weld);
        assert!(haz.appearance.opacity < 1.0 && haz.appearance.hatch == Some(scene::Hatch::Dots));
        assert_eq!(haz.layer, "haz");
        let scene::Geometry::Member(geometry) = &haz.geometry else { panic!("haz member") };
        area += geometry.section.area().expect("area");
    }
    let s = &document.sections[0];
    let flange = (s.width - s.web_thickness) / 2.0;
    let web_root = checks::part_1_1::haz_width(s.web_thickness) + s.elements[2].weld_position;
    let inflated = 4.0 * (flange + skin) * (s.flange_thickness + skin) + (s.web_thickness + skin) * (web_root + skin);
    assert!((area - inflated).abs() / inflated < 1e-6, "haz area {area} vs {inflated}");
}

#[test]
fn class_4_plates_get_an_effective_ghost_with_the_reported_reduction() {
    let document = failing();
    let (scene, report) = scene_of(&document, &[]);
    let ghost = entity(&scene, "beam-fail:effective");
    assert_eq!(ghost.appearance.style, scene::StyleState::Ghost);
    assert!(ghost.appearance.deformation.is_some());
    let reported = report.checks.iter().find(|c| c.id == "en1999.6.1.4.class.beam-fail.flange-top").expect("class check");
    let alloy = checks::part_1_1::resolve_alloy_for_thickness("aw6060-t6", checks::part_1_1::max_thickness(&document.sections[0])).expect("alloy");
    let element = &document.sections[0].elements[0];
    let rho = checks::part_1_1::rho_c(reported.computed.value, alloy.buckling_class_a, element.outstand, element.welded);
    assert!(rho < 1.0 && reported.status == norm::CheckStatus::Warning);
    assert!(scene.annotations.iter().any(|a| matches!(a, scene::Annotation::Leader { text, .. } if text.en.contains("ρ_c"))));
}

#[test]
fn a_combined_connection_projects_plate_grid_weld_haz_and_group() {
    let document = compliant();
    let (scene, _) = scene_of(&document, &[]);
    let conn = &document.connections[0];
    assert!(matches!(entity(&scene, "support-weld:plate").geometry, scene::Geometry::Plate(_)));
    let scene::Geometry::Fastener(bolts) = &entity(&scene, "support-weld:bolts").geometry else { panic!("bolts") };
    assert_eq!(bolts.pattern, Some(scene::Pattern::Grid { nx: conn.bolts.rows, ny: conn.bolts.bolts_per_row, sx: conn.bolts.pitch, sy: conn.bolts.gauge }));
    let scene::Geometry::Fastener(weld) = &entity(&scene, "support-weld:weld").geometry else { panic!("weld") };
    assert_eq!(weld.kind, scene::FastenerKind::Weld { throat: conn.welds.throat, weld: scene::WeldKind::Fillet });
    let scene::Geometry::Member(haz) = &entity(&scene, "support-weld:haz").geometry else { panic!("haz") };
    let scene::Path3::Polyline(axis) = &haz.axis else { panic!("axis") };
    assert!((axis.points[0].distance(&axis.points[1]) - conn.welds.haz_extent).abs() < 1e-9);
    let scene::Geometry::Group { members } = &entity(&scene, "support-weld").geometry else { panic!("group") };
    assert_eq!(members.len(), 4);
}

#[test]
fn rivets_and_pure_welds_project_their_own_fasteners() {
    let mut document = compliant();
    document.connections[0].bolts.material = "rivet-6082".into();
    let (scene, _) = scene_of(&document, &[]);
    let scene::Geometry::Fastener(rivets) = &entity(&scene, "support-weld:bolts").geometry else { panic!("rivets") };
    assert_eq!(rivets.kind, scene::FastenerKind::Dowel);
    document.connections[0].kind = semio_s_artifact_norm_en1999_model::ConnectionKind::Welded;
    let (scene, _) = scene_of(&document, &[]);
    assert!(scene.entities.iter().all(|e| e.entity.id != "support-weld:bolts"));
    let mut bolted = compliant();
    bolted.connections[0].kind = semio_s_artifact_norm_en1999_model::ConnectionKind::Bolted;
    let (scene, _) = scene_of(&bolted, &[]);
    assert!(scene.entities.iter().all(|e| e.entity.id != "support-weld:weld" && e.entity.id != "support-weld:haz"));
}

#[test]
fn sheets_shells_fire_and_fatigue_project_their_geometry() {
    let (scene, _) = scene_of(&compliant(), &[]);
    let scene::Geometry::Member(sheet) = &entity(&scene, "sheet-pass").geometry else { panic!("sheet") };
    assert!(matches!(sheet.section, scene::SectionShape::ThinWalled { .. }));
    assert!(matches!(entity(&scene, "shell-pass").geometry, scene::Geometry::Shell(_)));
    assert!(matches!(entity(&scene, "fire-1").geometry, scene::Geometry::Load(scene::LoadGlyph::Thermal { .. })));
    assert!(matches!(entity(&scene, "fat-1").geometry, scene::Geometry::Group { .. }));
}

#[test]
fn supports_and_restraints_follow_the_member_idealisation() {
    let (scene, _) = scene_of(&compliant(), &[]);
    let kind = |id: &str| match &entity(&scene, id).geometry {
        scene::Geometry::Support(s) => s.kind.clone(),
        other => panic!("support expected, got {other:?}"),
    };
    assert_eq!(kind("purlin-1:support-a"), scene::SupportKind::Pinned);
    assert!(matches!(kind("purlin-1:support-b"), scene::SupportKind::Roller { .. }));
    assert_eq!(kind("post-chs:support-a"), scene::SupportKind::Fixed);
    let restraints = scene.entities.iter().filter(|e| e.entity.id.starts_with("purlin-1:restraint-")).count();
    assert_eq!(restraints, 3);
}

#[test]
fn design_actions_become_load_glyphs() {
    let (scene, _) = scene_of(&compliant(), &[]);
    assert!(matches!(entity(&scene, "purlin-1:load-Q-snow:vz").geometry, scene::Geometry::Load(scene::LoadGlyph::Point { .. })));
    assert!(matches!(entity(&scene, "purlin-1:load-Q-snow:my").geometry, scene::Geometry::Load(scene::LoadGlyph::Moment { .. })));
    assert!(matches!(entity(&scene, "purlin-1:load-G:udl").geometry, scene::Geometry::Load(scene::LoadGlyph::Point { .. })));
}

#[test]
fn every_advertised_handle_exists_and_writes_a_known_leaf() {
    let document = compliant();
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
    assert!(all.iter().all(|h| h.writes.iter().all(|w| w.map.k.abs() > 1e-12) && h.limits.as_ref().is_some_and(|l| l.min < l.max)));
    let unique: BTreeSet<_> = all.iter().map(|h| h.id.clone()).collect();
    assert_eq!(unique.len(), all.len());
}

#[test]
fn dragging_the_length_handle_writes_the_snapped_length() {
    let all = handles(&compliant(), &[], &[member_ref("purlin-1")]);
    assert_handle_laws(&all);
    let length = all.iter().find(|h| h.id == "purlin-1:length").expect("length handle");
    let writes = length.writes_at(6.012);
    assert_eq!(writes.len(), 1);
    assert_eq!(writes[0].path, "members[id=purlin-1].length");
    assert!((writes[0].value - 6.0).abs() < 1e-9);
    assert_eq!(length.writes_at(1000.0)[0].value, 60.0);
}

#[test]
fn the_depth_handle_keeps_the_web_plate_tied_to_the_section() {
    let all = handles(&compliant(), &[], &[member_ref("purlin-1")]);
    assert_handle_laws(&all);
    let depth = all.iter().find(|h| h.id == "purlin-1:depth").expect("depth handle");
    let writes = depth.writes_at(0.160);
    let value = |path: &str| writes.iter().find(|w| w.path == path).map(|w| w.value).unwrap_or_else(|| panic!("no write {path}"));
    assert!((value("sections[id=sec-i120].height") - 0.160).abs() < 1e-9);
    assert!((value("sections[id=sec-i120].elements[id=web].width") - 0.140).abs() < 1e-9);
    let width = all.iter().find(|h| h.id == "purlin-1:width").expect("width handle").writes_at(0.090);
    let flange = width.iter().find(|w| w.path == "sections[id=sec-i120].elements[id=flange-top].width").expect("flange write");
    assert!((flange.value - (0.090 - 0.008) / 2.0).abs() < 1e-9);
    let thickness = all.iter().find(|h| h.id == "purlin-1:flange-thickness").expect("flange thickness").writes_at(0.012);
    assert!(thickness.iter().any(|w| w.path.ends_with("flange-bot].thickness") && (w.value - 0.012).abs() < 1e-9));
    assert!(thickness.iter().any(|w| w.path.ends_with("elements[id=web].width") && (w.value - (0.140 - 2.0 * 0.012)).abs() < 1e-9));
}

#[test]
fn weld_position_and_bolt_grid_handles_exist() {
    let all = handles(&compliant(), &[], &[member_ref("purlin-1"), scene::EntityRef::new(scene::EntityKind::Group, "support-weld", "connections[id=support-weld]")]);
    assert_handle_laws(&all);
    let tags: BTreeSet<_> = all.iter().map(|h| h.id.clone()).collect();
    for expected in ["purlin-1:weld-flange-top", "purlin-1:weld-web", "purlin-1:ltb-length", "purlin-1:buckling-z", "support-weld:pitch", "support-weld:gauge", "support-weld:edge-distance", "support-weld:rows", "support-weld:per-row", "support-weld:weld-length", "support-weld:weld-throat", "support-weld:haz-extent"] {
        assert!(tags.contains(expected), "missing {expected}");
    }
    let rows = all.iter().find(|h| h.id == "support-weld:rows").expect("rows");
    assert_eq!(rows.writes_at(2.6)[0].value, 3.0);
    assert_eq!(rows.writes[0].path, "connections[id=support-weld].bolts.rows");
}

#[test]
fn derived_scene_ids_resolve_to_their_document_entity() {
    let document = compliant();
    let derived = handles(&document, &[], &[scene::EntityRef::new(scene::EntityKind::Fastener, "support-weld:bolts", "connections[id=support-weld]")]);
    assert_handle_laws(&derived);
    assert!(derived.iter().any(|h| h.id == "support-weld:pitch"));
    assert_eq!(entity_kind_of(&document, "support-weld:bolts").map(|k| k.collection), Some("connections"));
    assert_eq!(entity_kind_of(&document, "nothing"), None);
}

#[test]
fn dependencies_name_what_a_projection_reads() {
    let document = compliant();
    let deps = entity_dependencies(&document, "purlin-1");
    for expected in ["sec-i120", "mat-6082", "support-weld", "fire-1", "fat-1", "Q-snow"] {
        assert!(deps.iter().any(|d| d == expected), "purlin-1 should depend on {expected}: {deps:?}");
    }
    assert!(entity_dependencies(&document, "support-weld").contains(&"purlin-1".to_string()));
}

#[test]
fn every_diagram_validates_and_has_a_table_twin() {
    for document in [compliant(), failing()] {
        let report = checks::evaluate_structure(&document);
        let all = diagrams(&document, &report, None);
        let ids: BTreeSet<_> = all.iter().map(|d| d.id.clone()).collect();
        let member = &document.members[0].id;
        let mut expected = vec![format!("buckling:{member}"), format!("class:{member}"), format!("effective:{member}"), format!("haz:{member}"), format!("hazwidth:{member}"), format!("mn:{member}"), format!("ltb:{member}"), format!("ltb-spacing:{member}")];
        expected.extend(document.fatigue_details.iter().map(|f| format!("sn:{}", f.id)));
        expected.extend(document.fire_scenarios.iter().flat_map(|f| [format!("fire:{}", f.id), format!("fire-k:{}", f.id), format!("fire-proof:{}", f.id)]));
        expected.extend(document.shells.iter().map(|s| format!("shell:{}", s.id)));
        for id in &expected {
            assert!(ids.contains(id), "missing diagram {id} in {ids:?}");
        }
        for diagram in &all {
            assert_eq!(diagram.validate(), Ok(()), "diagram {}", diagram.id);
            assert!(diagram.table_twin.rows() > 0, "diagram {} has no table twin", diagram.id);
            let lengths: BTreeSet<_> = diagram.table_twin.columns.iter().map(|c| c.values.len()).collect();
            assert_eq!(lengths.len(), 1, "diagram {} twin columns differ", diagram.id);
        }
    }
}

#[test]
fn the_buckling_marker_sits_on_the_member_curve_and_edits_the_buckling_length() {
    let document = compliant();
    let report = checks::evaluate_structure(&document);
    let diagram = diagrams(&document, &report, Some("purlin-1")).into_iter().find(|d| d.id == "buckling:purlin-1").expect("buckling");
    let member = &document.members[0];
    let section = &document.sections[0];
    let alloy = checks::part_1_1::resolve_alloy_for_thickness("aw6082-t6", checks::part_1_1::max_thickness(section)).expect("alloy");
    let (_, _, i_z, ..) = checks::part_1_1::section_geometry(section);
    let a_loc = {
        let report_eta = facts::eta_of(&report, "purlin-1", section, &alloy);
        checks::part_1_1::props(section, &checks::part_1_1::EffCtx { f_o_pa: alloy.f_o_pa, class_a: alloy.buckling_class_a, eta_web: report_eta, local: true, haz_rho: None, haz_extents: &[] }).a
    };
    let lambda = checks::part_1_1::lambda_bar(member.buckling_length_z, i_z, a_loc, alloy.f_o_pa);
    let (alpha, lambda_0) = checks::part_1_1::buckling_curve(alloy.buckling_class_a);
    let marker = diagram.markers.iter().find(|m| m.id == "marker-z").expect("marker z");
    assert!((marker.at.x - lambda).abs() < 1e-9);
    assert!((marker.at.y - checks::part_1_1::chi_from_lambda(lambda, alpha, lambda_0)).abs() < 1e-9);
    let binding = marker.binding.as_ref().expect("binding");
    assert_eq!(binding.path, "members[id=purlin-1].bucklingLengthZ");
    assert!((binding.value_at(lambda) - member.buckling_length_z).abs() < 0.03);
}

#[test]
fn the_interaction_demand_lies_inside_the_polygon_when_compliant() {
    let document = compliant();
    let report = checks::evaluate_structure(&document);
    let diagram = diagrams(&document, &report, Some("purlin-1")).into_iter().find(|d| d.id == "mn:purlin-1").expect("mn");
    let marker = diagram.markers.iter().find(|m| m.id == "demand").expect("demand");
    let scene::Series::Polygon { ring, .. } = &diagram.series[0] else { panic!("polygon first") };
    let ring: Vec<[f64; 2]> = ring.iter().map(|p| [p.x, p.y]).collect();
    assert!(scene::diagram_kit::interaction_polygon::contains(&ring, [marker.at.x, marker.at.y]));
    assert!(marker.utilisation.expect("utilisation") <= 1.0);
}

#[test]
fn the_haz_markers_edit_the_plate_thickness_in_millimetres() {
    let document = compliant();
    let report = checks::evaluate_structure(&document);
    let diagram = diagrams(&document, &report, Some("purlin-1")).into_iter().find(|d| d.id == "haz:purlin-1").expect("haz");
    let marker = diagram.markers.iter().find(|m| m.id == "rho-flange-top").expect("marker");
    assert!((marker.at.x - 10.0).abs() < 1e-9);
    let binding = marker.binding.as_ref().expect("binding");
    assert_eq!(binding.path, "sections[id=sec-i120].elements[id=flange-top].thickness");
    assert!((binding.value_at(12.0) - 0.012).abs() < 1e-12);
}

#[test]
fn focus_narrows_the_diagram_set() {
    let document = compliant();
    let report = checks::evaluate_structure(&document);
    assert_eq!(diagrams(&document, &report, Some("fat-1")).iter().map(|d| d.id.as_str()).collect::<Vec<_>>(), vec!["sn:fat-1"]);
    let fire_ids: Vec<String> = diagrams(&document, &report, Some("fire-1")).into_iter().map(|d| d.id).collect();
    assert_eq!(&fire_ids[..3], ["fire:fire-1", "fire-k:fire-1", "fire-proof:fire-1"]);
    assert!(fire_ids.iter().all(|id| id.ends_with(":fire-1")), "{fire_ids:?}");
    assert!(diagrams(&document, &report, Some("nothing")).is_empty());
}

#[test]
fn the_fire_resistance_diagram_marks_the_required_duration_and_drags_it() {
    let document = compliant();
    let report = checks::evaluate_structure(&document);
    let all = diagrams(&document, &report, Some("fire-1"));
    let proof = all.iter().find(|d| d.id == "fire-proof:fire-1").expect("proof strength");
    assert!(proof.series.iter().any(|s| matches!(s, Series::Polyline { id, points, .. } if id == "f-theta" && points.first().is_some_and(|p| p.y > points.last().map_or(0.0, |q| q.y)))));
    if let Some(resistance) = all.iter().find(|d| d.id == "fire-resistance:fire-1") {
        let marker = resistance.markers.iter().find(|m| m.id == "required").expect("required duration marker");
        assert_eq!(marker.binding.as_ref().expect("binding").path, "fireScenarios[id=fire-1].durationS");
        assert!((marker.binding.as_ref().expect("binding").value_at(30.0) - 1800.0).abs() < 1e-9);
    }
}

#[test]
fn the_sn_diagram_keeps_the_detail_categories_on_log_axes() {
    let document = compliant();
    let report = checks::evaluate_structure(&document);
    let diagram = diagrams(&document, &report, Some("fat-1")).remove(0);
    assert!(diagram.axes.iter().all(|a| a.scale == scene::AxisScale::Log10));
    assert!(diagram.series.iter().filter(|s| matches!(s, scene::Series::Polyline { .. })).count() >= 7);
    let marker = &diagram.markers[0];
    assert_eq!(marker.binding.as_ref().map(|b| b.path.as_str()), Some("fatigueDetails[id=fat-1].nCycles"));
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
    for document in [compliant(), failing(), wave3::eccentric(), wave3::instantaneous_centre()] {
        let report = checks::evaluate_structure(&document);
        let (scene, _) = scene_of(&document, &[]);
        let diagram_json: Vec<String> = diagrams(&document, &report, None).iter().map(canonical_json).collect();
        let sections: Vec<String> = document.sections.iter().map(|s| format!("{{\"id\":\"{}\",\"kind\":\"{}\",\"h\":{},\"b\":{},\"tw\":{},\"tf\":{},\"d\":{}}}", s.id, s.kind, s.height, s.width, s.web_thickness, s.flange_thickness, s.outer_diameter)).collect();
        let fatigue: Vec<String> = document.fatigue_details.iter().map(|f| format!("{{\"id\":\"{}\",\"dc\":{},\"m1\":{},\"m2\":{}}}", f.id, checks::part_1_3::detail_category_delta_sigma_c(&f.detail_category).unwrap_or(f.delta_sigma_c) / 1.0e6, f.m1, if f.m2 > 0.0 { f.m2 } else { f.m1 + 2.0 })).collect();
        let fires: Vec<String> = document.fire_scenarios.iter().filter_map(|f| {
            let member = document.members.iter().find(|m| m.id == f.member_id)?;
            Some(format!("{{\"id\":\"{}\",\"section\":\"{}\",\"thetaA\":{},\"durationS\":{}}}", f.id, member.section_id, f.theta_a, f.duration_s))
        }).collect();
        let shells: Vec<String> = document.shells.iter().map(|s| format!("{{\"id\":\"{}\",\"r\":{},\"t\":{},\"l\":{}}}", s.id, s.radius, s.thickness, s.length)).collect();
        let input = format!("{{\"scene\":{},\"diagrams\":[{}],\"sections\":[{}],\"fatigue\":[{}],\"fires\":[{}],\"shells\":[{}]{}}}", canonical_json(&scene), diagram_json.join(","), sections.join(","), fatigue.join(","), fires.join(","), shells.join(","), wave3::oracle_extras(&document, &diagrams(&document, &report, None)));
        let script = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🔮️oracles/🐍️.py");
        let output = norm::python_oracle::run_ok(script, &[], Some(input.as_bytes()));
        let verdict = String::from_utf8_lossy(&output.stdout);
        assert!(verdict.contains("\"ok\": true"), "oracle verdict: {verdict}");
    }
}

#[test]
fn projecting_a_thousand_members_stays_interactive() {
    let mut document = compliant();
    let template = document.members[0].clone();
    document.members = (0..1000).map(|i| model::AluminiumMember { id: format!("m{i}"), actions: Vec::new(), ..template.clone() }).collect();
    document.connections.clear();
    document.fire_scenarios.clear();
    document.fatigue_details.clear();
    let report = CheckReport::default();
    let started = std::time::Instant::now();
    let scene = project(&SceneInput { document: &document, report: &report, placements: &[], revision: revision() });
    assert!(scene.entities.len() >= 1000);
    let limit = if cfg!(debug_assertions) { 10_000 } else { 2_000 };
    eprintln!("[latency] en1999/scene-cold-1000 p50_ms={0:.3} p95_ms={0:.3} budget_ms={1:.1}", started.elapsed().as_secs_f64() * 1e3, limit as f64);
    assert!(started.elapsed().as_millis() < limit, "projection took {:?}", started.elapsed());
}

fn aluminium_parts() -> (model::AluminiumMember, model::AluminiumSection) {
    let d = compliant();
    let member = d.members[0].clone();
    let section = d.sections.iter().find(|s| s.id == member.section_id).expect("section").clone();
    (model::AluminiumMember { support: model::SupportCondition::SimplySupported, length: 6.0, ltb_length: 0.0, ..member }, section)
}

fn near(actual: f64, expected: f64, tolerance: f64, what: &str) {
    assert!((actual - expected).abs() <= tolerance * expected.abs(), "{what}: {actual} vs {expected}");
}

#[test]
fn the_flexural_mode_has_the_euler_load_of_the_section_and_support() {
    let (member, section) = aluminium_parts();
    let frame = kit::member_frame(&scene::Vec3::ZERO, &kit::v3(1.0, 0.0, 0.0));
    let (_, iy, iz, _, _, _, _) = checks::part_1_1::section_geometry(&section);
    let euler = |l: f64| std::f64::consts::PI.powi(2) * checks::na_de::E_PA * iy.min(iz) / (l * l);
    near(eigen::flexural(&member, &section, &frame).expect("pinned").factor, euler(6.0), 2e-3, "pinned");
    let cantilever = model::AluminiumMember { support: model::SupportCondition::Cantilever, ..member.clone() };
    near(eigen::flexural(&cantilever, &section, &frame).expect("cantilever").factor, euler(12.0), 2e-3, "cantilever");
    let fixed = model::AluminiumMember { support: model::SupportCondition::Continuous, ..member };
    near(eigen::flexural(&fixed, &section, &frame).expect("fixed").factor, euler(3.0), 2e-3, "fixed");
}

#[test]
fn the_lateral_torsional_critical_moment_rises_when_the_unrestrained_length_shortens() {
    let (member, section) = aluminium_parts();
    let frame = kit::member_frame(&scene::Vec3::ZERO, &kit::v3(1.0, 0.0, 0.0));
    let long = eigen::lateral_torsional(&member, &section, &frame, true, 10_000.0, 0.2).expect("long");
    let braced = eigen::lateral_torsional(&model::AluminiumMember { ltb_length: 2.0, ..member }, &section, &frame, true, 10_000.0, 0.2).expect("braced");
    assert!(long.factor > 0.0 && braced.factor > 1.5 * long.factor, "{} vs {}", braced.factor, long.factor);
}

#[test]
fn the_deflection_line_peaks_at_the_computed_deflection() {
    let (member, section) = aluminium_parts();
    let frame = kit::member_frame(&scene::Vec3::ZERO, &kit::v3(1.0, 0.0, 0.0));
    let line = eigen::deflection(&member, &section, &frame, true, 0.012).expect("line");
    let peak = line.samples.iter().map(|s| s.delta.z.abs()).fold(0.0, f64::max);
    near(peak, 0.012, 1e-9, "peak");
    near(line.factor * 0.012, eigen::DEFLECTION_TARGET * 6.0, 1e-9, "magnification");
}

#[test]
fn members_are_coloured_per_station_and_a_governing_buckling_check_annotates_its_load_factor() {
    let (scene, report) = scene_of(&failing(), &[]);
    for member in scene.entities.iter().filter(|e| e.entity.kind == scene::EntityKind::Member && !e.entity.id.contains(':')) {
        let Some(per) = member.utilisation.as_ref().and_then(|b| b.per_station.as_ref()) else { continue };
        assert!(per.windows(2).all(|w| w[0].at < w[1].at));
        let worst = report.checks.iter().filter(|c| c.subject.entity_id == member.entity.id).map(|c| c.utilization).fold(0.0_f64, f64::max);
        assert!((per.iter().map(|s| s.value).fold(0.0, f64::max) - worst).abs() < 1e-9);
    }
    assert!(scene.entities.iter().any(|e| e.utilisation.as_ref().is_some_and(|b| b.per_station.is_some())), "at least one member carries stations");
}


#[test]
fn the_mode_track_reveals_one_eigenform_layer_per_keyframe() {
    for document in [compliant(), failing()] {
        let (scene, _) = scene_of(&document, &[]);
        assert_eq!(scene.validate(), Ok(()));
        let layers: BTreeSet<_> = scene.entities.iter().map(|e| e.layer.as_str()).filter(|l| l.starts_with("mode-")).collect();
        assert_eq!(scene.tracks.iter().filter(|track| track.kind == scene::TrackKind::Mode).count(), usize::from(layers.len() >= 2));
        for track in scene.tracks.iter().filter(|track| track.kind == scene::TrackKind::Mode) {
            assert_eq!((track.kind.clone(), track.min, track.max, track.step, track.id.as_str()), (scene::TrackKind::Mode, 1.0, track.keyframes.len() as f64, 1.0, "mode"));
            for key in &track.keyframes {
                let layer = key.layer.as_deref().expect("layer");
                assert!(scene.layers.iter().any(|l| l.id == layer));
                let carriers: Vec<_> = scene.entities.iter().filter(|e| e.layer == layer).collect();
                assert!(!carriers.is_empty() && carriers.iter().all(|e| e.appearance.deformation.as_ref().is_some_and(|o| o.kind == scene::DeformationKind::Mode && o.motion == scene::Motion::Harmonic && o.index as f64 == key.at)));
            }
        }
    }
}

fn fold_into(document: &mut En1999Snapshot, commit: &scene::projection_kit::PlacementCommit) {
    let mut rows = scene::projection_kit::placements_of(document);
    commit.apply(&mut rows);
    document.placements = rows.into_iter().map(|row| model::Placement { id: row.entity_id, x: row.origin.x, y: row.origin.y, z: row.origin.z, yaw: row.yaw }).collect();
}

#[test]
fn dragging_an_entity_commits_one_undoable_event_and_the_reload_stands_at_the_same_pose() {
    use scene::projection_kit::{PlacementCommit, PlacementMove};
    let mut document = compliant();
    let id = document.members[0].id.clone();
    let before = scene::projection_kit::placements_of(&document);
    assert!(before.is_empty());
    let target = scene::Vec3::new(7.5, -2.0, 0.0);
    let commit = PlacementCommit::of(&before, &[PlacementMove { entity_id: id.clone(), origin: target.clone(), yaw: 0.75 }]);
    assert_eq!(commit.ops.len(), 1);
    fold_into(&mut document, &commit);
    let reloaded = scene::projection_kit::placements_of(&document);
    assert_eq!(reloaded.len(), 1);
    assert_eq!(Layout::of(&document, &reloaded).member(&id).expect("pose").frame.origin.clone(), target);
    let again = PlacementCommit::of(&reloaded, &[PlacementMove { entity_id: id.clone(), origin: target, yaw: 0.75 }]);
    assert!(again.is_empty());
    fold_into(&mut document, &commit.inverse(&before));
    assert_eq!(scene::projection_kit::placements_of(&document), before);
}

#[test]
fn entities_without_a_placement_row_get_move_grips_whose_first_drag_inserts_the_row_and_the_next_continues_it() {
    use scene::projection_kit::{PlacementCommit, PlacementOp, PlacementWrite};
    let mut document = compliant();
    assert!(document.placements.is_empty());
    let selection: Vec<scene::EntityRef> = document.members.iter().map(|m| scene::EntityRef::new(scene::EntityKind::Member, &m.id, format!("members[id={}]", m.id))).collect();
    let id = document.members.iter().find(|m| !super::layout::is_upright(m)).expect("a free member").id.clone();
    let grips = super::handles::handles(&document, &[], &selection);
    for tag in ["move-x", "move-y", "move-yaw"] {
        assert!(grips.iter().any(|h| h.id == format!("{id}:{tag}")), "{tag}");
    }
    let x = grips.iter().find(|h| h.id == format!("{id}:move-x")).expect("move-x");
    let write = PlacementWrite { path: x.writes[0].path.clone(), value: x.writes[0].map.base + 2.0 };
    let standing = |wanted: &str| (wanted == id).then(|| Placement { entity_id: id.clone(), origin: x.anchor.clone(), yaw: 0.0 });
    let first = PlacementCommit::of_writes(&[], &standing, std::slice::from_ref(&write));
    assert!(matches!(&first.ops[..], [PlacementOp::Insert(row)] if row.entity_id == id && row.origin.x == write.value));
    fold_into(&mut document, &first);
    let rows = scene::projection_kit::placements_of(&document);
    let again = super::handles::handles(&document, &rows, &selection);
    let moved = again.iter().find(|h| h.id == format!("{id}:move-x")).expect("move-x");
    assert_eq!(moved.writes[0].map.base, write.value);
    let next = PlacementCommit::of_writes(&rows, &standing, &[PlacementWrite { path: write.path.clone(), value: write.value + 1.0 }]);
    assert!(matches!(&next.ops[..], [PlacementOp::Set(w)] if w.len() == 4 && w[0].value == write.value + 1.0));
    let sheet_and_shell: Vec<String> = document.cold_formed.iter().map(|c| c.id.clone()).chain(document.shells.iter().map(|c| c.id.clone())).collect();
    let layout = Layout::of(&document, &rows);
    for other in sheet_and_shell {
        assert!(super::handles::ids(&document, &layout, &other).contains(&format!("{other}:move-x")), "{other}");
    }
}

fn wave5() -> En1999Snapshot {
    use super::model::{FatigueCategory, PatchLoad, RingStiffener};
    let mut document = compliant();
    document.fatigue_categories.push(FatigueCategory { id: "site-1".into(), member_id: "purlin-1".into(), element_id: String::new(), detail_category: "71".into(), detail_type: semio_s_artifact_norm_en1999_model::DetailType::ButtWeld, location_x: 3.0, m1: 0.0, m2: 0.0 });
    document.patch_loads.push(PatchLoad { id: "patch-1".into(), subject_id: "purlin-1".into(), position: semio_s_artifact_norm_en1999_model::PatchLoadPosition::Interior, x: 2.0, bearing_length: 0.05, web_height: 0.0, web_angle: 0.0, corner_radius: 0.0, load: 10_000.0 });
    document.ring_stiffeners.push(RingStiffener { id: "ring-1".into(), shell_id: "shell-pass".into(), x: 1.0, depth: 0.05, thickness: 0.006, internal: false });
    document
}

#[test]
fn fatigue_sites_patch_loads_and_ring_stiffeners_are_entities_with_handles_bound_to_their_checks() {
    let document = wave5();
    let (scene, report) = scene_of(&document, &[]);
    assert_eq!(scene.validate(), Ok(()), "violations");
    let site = entity(&scene, "site-1");
    assert!(matches!(&site.geometry, scene::Geometry::Solid(_)));
    assert_eq!(site.binding.as_ref().map(|b| b.check_id.as_str()), Some("en1999.1-3.fat.category.site-1"));
    assert!(!site.handles.is_empty());
    assert!(matches!(entity(&scene, "patch-1").geometry, scene::Geometry::Load(scene::LoadGlyph::Point { .. })));
    assert!(matches!(entity(&scene, "patch-1:patch").geometry, scene::Geometry::Solid(_)));
    assert!(matches!(entity(&scene, "ring-1").geometry, scene::Geometry::Shell(_)));
    assert!(report.checks.iter().any(|c| c.id == "en1999.1-5.stiffener.ring-1"));
}

#[test]
fn a_patch_load_has_grips_for_bearing_position_kind_and_web_height_and_a_web_buckling_zone_that_follows_the_bearing_length() {
    let mut document = wave5();
    let layout = Layout::of(&document, &[]);
    let tags = handles::ids(&document, &layout, "patch-1");
    for tag in ["position", "bearing", "force", "kind", "web-height"] {
        assert!(tags.iter().any(|h| h.ends_with(tag)), "{tag} in {tags:?}");
    }
    let zone_length = |document: &En1999Snapshot| match &entity(&scene_of(document, &[]).0, "patch-1:zone").geometry {
        scene::Geometry::Member(scene::MemberGeometry { axis: scene::Path3::Polyline(line), .. }) => line.points[0].distance(&line.points[1]),
        other => panic!("{other:?}"),
    };
    let before = zone_length(&document);
    document.patch_loads[0].bearing_length = 0.2;
    assert!(zone_length(&document) > before, "a longer stiff bearing widens the effective web length");
}

#[test]
fn the_patch_resistance_diagram_pins_the_design_point_to_the_check_and_drags_the_bearing_length() {
    let mut document = wave5();
    let report = checks::evaluate_structure(&document);
    let diagram = diagrams(&document, &report, Some("patch-1")).into_iter().find(|d| d.id == "patch-resistance:patch-1").expect("the resistance over the bearing length");
    assert_eq!(diagram.validate(), Ok(()));
    let marker = &diagram.markers[0];
    assert_eq!(marker.binding.as_ref().map(|b| b.path.as_str()), Some("patchLoads[id=patch-1].bearingLength"));
    let check = report.checks.iter().find(|c| c.id == "en1999.6.7.5.web.patch-1").expect("the member patch check");
    assert!((marker.utilisation.expect("design point") - check.utilization).abs() < 1e-9, "{:?} vs {}", marker.utilisation, check.utilization);
    let curve = &diagram.table_twin.columns[1].values;
    assert!(curve.windows(2).all(|w| w[1] >= w[0] - 1e-9), "the resistance does not fall with a longer bearing");
    document.patch_loads[0].position = semio_s_artifact_norm_en1999_model::PatchLoadPosition::End;
    let end = diagrams(&document, &checks::evaluate_structure(&document), Some("patch-1")).into_iter().find(|d| d.id == "patch-resistance:patch-1").expect("diagram without resistance");
    assert_eq!(end.validate(), Ok(()));
    assert!(end.series.iter().all(|s| !matches!(s, scene::Series::Polyline { .. })) && end.markers[0].utilisation.is_none() && end.annotations.len() == 1, "the end position stays unverified and draws no invented curve");
}

#[test]
fn every_example_and_showcase_projection_obeys_the_scene_diagram_and_handle_laws() {
    for document in [compliant(), failing()] {
        let (scene, report) = scene_of(&document, &[]);
        let selection: Vec<scene::EntityRef> = scene.entities.iter().map(|e| e.entity.clone()).collect();
        scene::projection_kit::test_support::assert_projection_laws(&scene, &diagrams(&document, &report, None), &handles(&document, &[], &selection));
    }
}

#[test]
fn the_full_showcase_projects_a_valid_scene_with_an_entity_per_populated_collection() {
    let (scene, _) = scene_of(&En1999Snapshot::full_showcase(), &[]);
    assert!(scene.validate().is_ok());
    assert!(!scene.entities.is_empty());
    assert!(scene.entities.iter().all(|e| e.content_hash == e.compute_hash()));
}

#[test]
fn the_lip_depth_and_thickness_handles_each_bind_a_typed_kind_and_a_drag_is_one_commit() {
    let mut document = compliant();
    let element = document.sections[0].elements.iter_mut().find(|e| e.id == "flange-top").expect("flange");
    element.edge_stiffener_depth = 0.012;
    element.edge_stiffener_thickness = 0.002;
    let selection = [scene::EntityRef::new(scene::EntityKind::Member, "purlin-1:edge-flange-top", "members[id=purlin-1]")];
    let all = handles(&document, &[], &selection);
    assert_handle_laws(&all);
    let depth = all.iter().find(|h| h.id == "purlin-1:edge-flange-top:depth").expect("lip depth handle");
    let thickness = all.iter().find(|h| h.id == "purlin-1:edge-flange-top:thickness").expect("lip thickness handle");
    let normalised = |path: &str| path.replace("[id=sec-i120]", "[]").replace("[id=flange-top]", "[]");
    let rules = semio_s_artifact_norm_en1999_mutations::EDIT_RULES;
    for (handle, leaf, value) in [(depth, "edgeStiffenerDepth", 0.016), (thickness, "edgeStiffenerThickness", 0.003)] {
        let writes = handle.writes_at(value);
        assert_eq!(writes.len(), 1, "a drag of {} is one commit", handle.id);
        assert_eq!(writes[0].path, format!("sections[id=sec-i120].elements[id=flange-top].{leaf}"));
        assert!(rules.set_field.iter().any(|rule| rule.path == normalised(&writes[0].path)), "{} has no typed kind", handle.id);
    }
}

#[test]
fn the_transverse_stiffener_plates_carry_their_checks_and_each_grip_writes_one_typed_kind() {
    use super::model::TransverseStiffener;
    let mut document = compliant();
    document.transverse_stiffeners.push(TransverseStiffener { id: "stiffener-1".into(), girder_id: "purlin-1".into(), position: 2.0, width: 0.05, thickness: 0.006, sides: semio_s_artifact_norm_en1999_model::StiffenerSides::Two });
    document.transverse_stiffeners.push(TransverseStiffener { id: "stiffener-thin".into(), girder_id: "purlin-1".into(), position: 4.0, width: 0.01, thickness: 0.001, sides: semio_s_artifact_norm_en1999_model::StiffenerSides::One });
    let (scene, report) = scene_of(&document, &[]);
    assert_eq!(scene.validate(), Ok(()), "violations");
    let plate = entity(&scene, "stiffener-1");
    assert!(matches!(&plate.geometry, scene::Geometry::Member(_)));
    assert!(plate.binding.as_ref().is_some_and(|b| b.check_id.starts_with("en1999.6.7.6.web-")));
    assert!(plate.handles.iter().any(|h| h.ends_with(":position")) && plate.handles.iter().any(|h| h.ends_with(":thickness")));
    assert!(matches!(&entity(&scene, "stiffener-1:mirror").geometry, scene::Geometry::Member(_)), "a double-sided stiffener has a second plate");
    assert!(scene.entities.iter().all(|e| e.entity.id != "stiffener-thin:mirror"), "a single-sided stiffener has one plate");
    assert!(entity(&scene, "stiffener-1:mirror").handles.is_empty());
    assert_eq!(entity(&scene, "stiffener-thin").binding.as_ref().map(|b| b.status), Some(scene::CheckStatus::Fail), "the thin flat bar colours as failing");
    assert!(report.checks.iter().any(|c| c.id == "en1999.6.7.6.web-stiffness.stiffener-1") && report.checks.iter().any(|c| c.id == "en1999.6.7.6.web-strength.stiffener-1"));
    let selection = [scene::EntityRef::new(scene::EntityKind::Member, "stiffener-1", "transverseStiffeners[id=stiffener-1]")];
    let all = handles(&document, &[], &selection);
    assert_handle_laws(&all);
    let rules = semio_s_artifact_norm_en1999_mutations::EDIT_RULES;
    for (tag, leaf, value) in [("position", "position", 2.5), ("width", "width", 0.06), ("thickness", "thickness", 0.008)] {
        let handle = all.iter().find(|h| h.id == format!("stiffener-1:{tag}")).expect("grip");
        let writes = handle.writes_at(value);
        assert_eq!(writes.len(), 1, "a drag of {} is one commit", handle.id);
        assert_eq!(writes[0].path, format!("transverseStiffeners[id=stiffener-1].{leaf}"));
        assert!(rules.set_field.iter().any(|rule| rule.path == format!("transverseStiffeners[].{leaf}")), "{} has no typed kind", handle.id);
    }
}

#[test]
fn a_document_with_two_combinations_offers_the_combination_track() {
    for document in [compliant(), failing()] {
        let (scene, _) = scene_of(&document, &[]);
        let distinct = tracks::combination_captions(&document).into_iter().map(|(en, _)| en).collect::<BTreeSet<_>>().len();
        scene::projection_kit::test_support::assert_combination_track_law(&scene, distinct);
    }
}

#[test]
fn the_web_panel_diagram_follows_the_stiffener_rows_and_a_drag_redraws_it() {
    use super::model::TransverseStiffener;
    let mut document = compliant();
    let report = checks::evaluate_structure(&document);
    assert!(diagrams(&document, &report, None).iter().all(|d| !d.id.starts_with("web-panels:")), "without a stiffener there are no panels");
    for (id, x) in [("s1", 2.0), ("s2", 4.0)] {
        document.transverse_stiffeners.push(TransverseStiffener { id: id.into(), girder_id: "purlin-1".into(), position: x, width: 0.05, thickness: 0.006, sides: semio_s_artifact_norm_en1999_model::StiffenerSides::Two });
    }
    let panels = |document: &En1999Snapshot| diagrams(document, &checks::evaluate_structure(document), Some("s1")).into_iter().find(|d| d.id == "web-panels:purlin-1").expect("the panels of the focused stiffener's girder");
    let widths = |d: &scene::Diagram2d| d.table_twin.columns[3].values.clone();
    let before = panels(&document);
    assert_eq!(before.validate(), Ok(()));
    assert_eq!((before.table_twin.rows(), before.markers.len()), (3, 2));
    assert!(widths(&before).iter().all(|a| (a - 2.0).abs() < 1e-9));
    assert!(before.markers.iter().all(|m| m.binding.as_ref().is_some_and(|b| b.path == format!("transverseStiffeners[id={}].position", m.id))));
    document.transverse_stiffeners[0].position = 1.0;
    let after = panels(&document);
    let moved = widths(&after);
    assert!((moved[0] - 1.0).abs() < 1e-9 && (moved[1] - 3.0).abs() < 1e-9 && (moved[2] - 2.0).abs() < 1e-9, "{moved:?}");
    assert!(after.table_twin.columns[4].values[1] > before.table_twin.columns[4].values[1], "the wider panel has the larger aspect ratio");
}

#[test]
fn nested_action_rows_get_magnitude_grips_on_their_glyphs() {
    let document = compliant();
    let (scene, _) = scene_of(&document, &[]);
    for glyph in ["purlin-1:load-Q-snow:vz", "purlin-1:load-Q-snow:my", "purlin-1:load-G:udl"] {
        let entity = entity(&scene, glyph);
        assert_eq!(entity.handles, vec![format!("{glyph}:magnitude")], "{glyph}");
        let all = handles(&document, &[], &[entity.entity.clone()]);
        assert_handle_laws(&all);
        let grip = all.iter().find(|h| h.id == format!("{glyph}:magnitude")).expect("magnitude grip");
        assert!(grip.writes[0].path.starts_with("members[id=purlin-1].actions[id=") && grip.writes[0].path.contains("]."), "{}", grip.writes[0].path);
        assert!(grip.limits.as_ref().is_some_and(|l| l.contains(0.0)));
    }
}

#[test]
fn the_product_form_of_the_member_material_is_a_choice_stepper_on_the_member() {
    let all = handles(&compliant(), &[], &[member_ref("purlin-1")]);
    let grip = all.iter().find(|h| h.id == "purlin-1:product-form").expect("product form grip");
    assert_eq!(grip.writes.len(), 1);
    assert!(grip.writes[0].path.starts_with("materials[id=") && grip.writes[0].path.ends_with("].productForm"), "{}", grip.writes[0].path);
}

#[test]
fn the_section_outline_is_drawn_for_parametric_sections_with_width_and_web_markers_and_follows_the_edit() {
    let mut document = compliant();
    let section = document.sections[0].clone();
    let outline = |document: &En1999Snapshot, focus: Option<&str>| diagrams(document, &checks::evaluate_structure(document), focus).into_iter().find(|d| d.id == format!("outline:{}", section.id));
    let before = outline(&document, Some(&section.id)).expect("a parametric section has an outline diagram, found by its section id");
    assert_eq!(outline(&document, Some(&document.members[0].id)).as_ref(), Some(&before), "found by the member that uses it");
    assert_eq!(outline(&document, None).as_ref(), Some(&before));
    let width = before.markers.iter().find(|m| m.id == "reference:width").expect("width marker");
    assert!((width.at.x - section.width / 2.0).abs() < 1e-9 && width.binding.as_ref().is_some_and(|b| b.path == format!("sections[id={}].width", section.id) && b.map.k == 2.0));
    let web = before.markers.iter().find(|m| m.id == "reference:web-thickness").expect("web marker");
    assert!((web.at.x - section.web_thickness / 2.0).abs() < 1e-9);
    assert_eq!(before.validate(), Ok(()));
    document.sections[0].width *= 1.25;
    let after = outline(&document, None).expect("outline");
    assert_ne!(before, after);
    assert_eq!(after, outline(&document.clone(), None).expect("outline"), "a pure function of the document");
}

#[test]
fn the_palette_offers_sections_alloys_outline_vertices_and_placements() {
    let collections: Vec<&str> = ENTITY_KINDS.iter().map(|k| k.collection).collect();
    for collection in ["sections", "materials", "outlineVertices", "placements"] {
        assert!(collections.contains(&collection), "{collection}");
    }
    let document = compliant();
    assert_eq!(entity_kind_of(&document, &document.sections[0].id).map(|k| k.collection), Some("sections"));
    assert_eq!(entity_kind_of(&document, &document.materials[0].id).map(|k| k.collection), Some("materials"));
}

#[test]
fn the_basis_badge_follows_the_document_annex_in_the_scene_and_every_diagram() {
    scene::projection_kit::test_support::assert_basis_follows_annex(norm::NormFamilyId::En1999, |annex| {
        let mut document = compliant();
        document.annex = annex;
        let (scene, report) = scene_of(&document, &[]);
        (scene, diagrams(&document, &report, None))
    });
}
