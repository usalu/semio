use super::*;
use scene::projection_kit::test_support::{revision, entity, assert_handle_laws};
use scene::{canonical_json, Validate};
use std::collections::BTreeSet;

fn scene_of(document: &En1993Snapshot, placements: &[Placement]) -> (StructuralScene, CheckReport) {
    let report = checks::check_full_steel_structure(document);
    let scene = project(&SceneInput { document, report: &report, placements, revision: revision() });
    (scene, report)
}

fn compliant() -> En1993Snapshot {
    En1993Snapshot::compliant_heb240_frame()
}

fn overloaded() -> En1993Snapshot {
    En1993Snapshot::noncompliant_overloaded_frame()
}

#[test]
fn the_scene_of_every_fixture_validates() {
    for document in [compliant(), overloaded()] {
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
    let annotation_ids: BTreeSet<_> = scene.annotations.iter().map(|a| format!("{a:?}").split('"').nth(1).unwrap_or_default().to_string()).collect();
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
fn a_member_extrudes_its_catalogue_section_along_its_length() {
    let document = compliant();
    let (scene, _) = scene_of(&document, &[]);
    let member = entity(&scene, "member-b1");
    let scene::Geometry::Member(geometry) = &member.geometry else { panic!("member geometry") };
    let area = geometry.section.area().expect("area");
    let catalogue = document.sections[0].area;
    assert!((area - catalogue).abs() / catalogue < 0.05, "outline {area} vs catalogue {catalogue}");
    let scene::Path3::Polyline(axis) = &geometry.axis else { panic!("polyline axis") };
    assert!((axis.points[0].distance(&axis.points[1]) - document.members[0].length).abs() < 1e-9);
    assert_eq!(member.entity.path, "members[id=member-b1]");
}

#[test]
fn the_default_layout_runs_members_left_to_right_with_a_gap() {
    let mut document = compliant();
    let mut second = document.members[0].clone();
    second.id = "member-b2".into();
    document.members.push(second);
    let layout = Layout::of(&document, &[]);
    let first = layout.member("member-b1").expect("first");
    let next = layout.member("member-b2").expect("second");
    assert_eq!(first.frame.origin, scene::Vec3::ZERO);
    assert!((next.frame.origin.x - (document.members[0].length + layout::MEMBER_GAP)).abs() < 1e-12);
    assert_eq!(first.frame.x, scene::Vec3::X);
    assert_eq!(first.frame.z, scene::Vec3::Z);
}

#[test]
fn a_persisted_placement_overrides_the_default_and_rotates_about_z() {
    let placement = Placement { entity_id: "member-b1".into(), origin: scene::Vec3::new(10.0, 2.0, 0.0), yaw: std::f64::consts::FRAC_PI_2 };
    let layout = Layout::of(&compliant(), &[placement]);
    let pose = layout.member("member-b1").expect("pose");
    assert_eq!(pose.frame.origin, scene::Vec3::new(10.0, 2.0, 0.0));
    assert!(pose.frame.x.y > 0.999 && pose.frame.x.x.abs() < 1e-9);
}

#[test]
fn utilisation_colouring_follows_the_governing_check() {
    let (scene, report) = scene_of(&compliant(), &[]);
    let member = entity(&scene, "member-b1");
    let binding = member.utilisation.as_ref().expect("binding");
    let expected = report.checks.iter().filter(|c| c.status != norm::CheckStatus::NotApplicable).filter(|c| ["member-b1", "fat-1", "fire-1", "br-1", "tw-1", "crane-1"].contains(&c.subject.entity_id.as_str())).map(|c| c.utilization).fold(0.0_f64, f64::max);
    assert!((binding.ratio - expected).abs() < 1e-12);
    assert_eq!(member.appearance.channel, scene::ColourChannel::Utilisation);
}

#[test]
fn a_failing_member_gets_error_style_hatch_and_failure_glyphs() {
    let (scene, _) = scene_of(&overloaded(), &[]);
    let member = entity(&scene, "member-b1");
    assert_eq!(member.appearance.style, scene::StyleState::Error);
    assert_eq!(member.appearance.hatch, Some(scene::Hatch::Diagonal));
    assert!(scene.annotations.iter().any(|a| matches!(a, scene::Annotation::FailureGlyph { .. })));
    assert!(scene.entities.iter().any(|e| e.entity.id.starts_with("member-b1:mode-") && e.appearance.deformation.is_some()), "a buckling check shows its mode shape on a mode layer");
}

#[test]
fn a_bolted_joint_projects_plate_grid_and_group() {
    let document = compliant();
    let (scene, _) = scene_of(&document, &[]);
    let joint = &document.joints[0];
    assert!(matches!(entity(&scene, "joint-j1:plate").geometry, scene::Geometry::Plate(_)));
    let scene::Geometry::Fastener(bolts) = &entity(&scene, "joint-j1:bolts").geometry else { panic!("bolts") };
    assert_eq!(bolts.pattern, Some(scene::Pattern::Grid { nx: joint.bolt_rows, ny: joint.bolts_per_row, sx: joint.pitch, sy: joint.gauge }));
    assert_eq!(bolts.diameter, joint.bolt_diameter);
    let scene::Geometry::Group { members } = &entity(&scene, "joint-j1").geometry else { panic!("group") };
    assert_eq!(members.len(), 2);
}

#[test]
fn a_welded_joint_projects_a_fillet_weld() {
    let mut document = compliant();
    document.joints[0].kind = semio_s_artifact_norm_en1993_model::JointKind::Welded;
    document.joints[0].weld_throat = 0.005;
    document.joints[0].weld_length = 0.2;
    let (scene, _) = scene_of(&document, &[]);
    let scene::Geometry::Fastener(weld) = &entity(&scene, "joint-j1:weld").geometry else { panic!("weld") };
    assert_eq!(weld.kind, scene::FastenerKind::Weld { throat: 0.005, weld: scene::WeldKind::Fillet });
    assert!(scene.entities.iter().all(|e| e.entity.id != "joint-j1:bolts"));
}

#[test]
fn special_elements_each_project_their_geometry() {
    let (scene, _) = scene_of(&compliant(), &[]);
    assert!(matches!(entity(&scene, "pp-1").geometry, scene::Geometry::Plate(_)));
    assert!(matches!(entity(&scene, "silo-1").geometry, scene::Geometry::Shell(_)));
    assert!(matches!(entity(&scene, "ten-1").geometry, scene::Geometry::Member(_)));
    assert!(matches!(entity(&scene, "cf-1").geometry, scene::Geometry::Member(_)));
    assert!(matches!(entity(&scene, "pile-1").geometry, scene::Geometry::Member(_)));
    assert!(matches!(entity(&scene, "tw-1").geometry, scene::Geometry::Load(scene::LoadGlyph::Point { .. })));
    assert!(matches!(entity(&scene, "crane-1").geometry, scene::Geometry::Load(scene::LoadGlyph::Point { .. })));
    assert!(matches!(entity(&scene, "fire-1").geometry, scene::Geometry::Load(scene::LoadGlyph::Thermal { .. })));
    assert!(matches!(entity(&scene, "silo-1:janssen").geometry, scene::Geometry::Load(scene::LoadGlyph::Pressure { .. })));
    assert!(matches!(entity(&scene, "fat-1").geometry, scene::Geometry::Group { .. }));
}

#[test]
fn design_actions_become_load_glyphs_and_members_get_supports() {
    let (scene, _) = scene_of(&compliant(), &[]);
    assert!(matches!(entity(&scene, "act-b1-g:n").geometry, scene::Geometry::Load(scene::LoadGlyph::Point { .. })));
    assert!(matches!(entity(&scene, "act-b1-g:my").geometry, scene::Geometry::Load(scene::LoadGlyph::Moment { .. })));
    assert!(matches!(entity(&scene, "member-b1:support-a").geometry, scene::Geometry::Support(_)));
    assert!(matches!(entity(&scene, "member-b1:restraint-1").geometry, scene::Geometry::Support(_)));
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
}

#[test]
fn dragging_the_length_handle_writes_the_snapped_length() {
    let document = compliant();
    let all = handles(&document, &[], &[scene::EntityRef::new(scene::EntityKind::Member, "member-b1", "members[id=member-b1]")]);
    assert_handle_laws(&all);
    let length = all.iter().find(|h| h.id == "member-b1:length").expect("length handle");
    let writes = length.writes_at(5.012);
    assert_eq!(writes.len(), 1);
    assert_eq!(writes[0].path, "members[id=member-b1].length");
    assert!((writes[0].value - 5.0).abs() < 1e-9);
    assert_eq!(length.writes_at(1000.0)[0].value, 60.0);
}

#[test]
fn the_section_handle_snaps_to_catalogue_heights() {
    let all = handles(&compliant(), &[], &[scene::EntityRef::new(scene::EntityKind::Member, "member-b1", "members[id=member-b1]")]);
    assert_handle_laws(&all);
    let section = all.iter().find(|h| h.id == "member-b1:section").expect("section handle");
    let write = &section.writes_at(0.27)[0];
    assert_eq!(write.unit, handles::SECTION_HEIGHT_UNIT);
    assert!((write.value - 0.26).abs() < 1e-9 || (write.value - 0.28).abs() < 1e-9);
    assert!((section.writes_at(0.2403)[0].value - 0.24).abs() < 1e-9);
}

#[test]
fn bolt_grid_handles_cover_pitch_gauge_distances_and_counts() {
    let all = handles(&compliant(), &[], &[scene::EntityRef::new(scene::EntityKind::Group, "joint-j1", "joints[id=joint-j1]")]);
    assert_handle_laws(&all);
    let tags: BTreeSet<_> = all.iter().map(|h| h.id.rsplit(':').next().unwrap_or_default().to_string()).collect();
    for tag in ["pitch", "gauge", "end-distance", "edge-distance", "rows", "per-row"] {
        assert!(tags.contains(tag), "missing {tag}");
    }
    let rows = all.iter().find(|h| h.id.ends_with(":rows")).expect("rows");
    assert_eq!(rows.writes_at(2.6)[0].value, 3.0);
}

#[test]
fn derived_scene_ids_resolve_to_their_document_entity() {
    let document = compliant();
    let derived = handles(&document, &[], &[scene::EntityRef::new(scene::EntityKind::Fastener, "joint-j1:bolts", "joints[id=joint-j1]")]);
    assert_handle_laws(&derived);
    assert!(derived.iter().any(|h| h.id == "joint-j1:pitch"));
    assert_eq!(entity_kind_of(&document, "joint-j1:bolts").map(|k| k.collection), Some("joints"));
    assert_eq!(entity_kind_of(&document, "nothing"), None);
}

#[test]
fn dependencies_name_what_a_projection_reads() {
    let document = compliant();
    let deps = entity_dependencies(&document, "member-b1");
    for expected in ["sec-heb240", "mat-s355", "joint-j1", "act-b1-g", "g-permanent"] {
        assert!(deps.iter().any(|d| d == expected), "member-b1 should depend on {expected}: {deps:?}");
    }
    assert_eq!(entity_dependencies(&document, "joint-j1"), vec!["member-b1".to_string()]);
}

#[test]
fn every_diagram_validates_and_has_a_table_twin() {
    for document in [compliant(), overloaded()] {
        let report = checks::check_full_steel_structure(&document);
        let all = diagrams(&document, &report, None);
        let ids: BTreeSet<_> = all.iter().map(|d| d.id.clone()).collect();
        for expected in ["buckling:member-b1", "ltb:member-b1", "ltb-spacing:member-b1", "mn:member-b1", "class:member-b1", "sn:fat-1", "fire:fire-1", "fire-k:fire-1", "fire-utilisation:fire-1"] {
            assert!(ids.contains(expected), "missing diagram {expected} in {ids:?}");
        }
        for diagram in &all {
            assert_eq!(diagram.validate(), Ok(()), "diagram {}", diagram.id);
            assert!(diagram.table_twin.rows() > 0, "diagram {} has no table twin", diagram.id);
        }
    }
}

#[test]
fn the_buckling_marker_sits_on_the_member_curve() {
    let document = compliant();
    let report = checks::check_full_steel_structure(&document);
    let all = diagrams(&document, &report, Some("member-b1"));
    let diagram = all.iter().find(|d| d.id == "buckling:member-b1").expect("buckling");
    let section = &document.sections[0];
    let material = &document.materials[0];
    let member = &document.members[0];
    let lambda = checks::part_1_1::lambda_bar(member.buckling_length_z, checks::part_1_1::radius_of_gyration(section.iz, section.area), material.e_modulus, material.fy);
    let curve = checks::part_1_1::buckling_curve_rolled_i(section, 'z', material.fy);
    let marker = diagram.markers.iter().find(|m| m.id == "marker-z").expect("marker z");
    assert!((marker.at.x - lambda).abs() < 1e-9);
    assert!((marker.at.y - checks::part_1_1::chi(lambda, curve)).abs() < 1e-9);
    let binding = marker.binding.as_ref().expect("binding");
    assert_eq!(binding.path, "members[id=member-b1].bucklingLengthZ");
    assert!((binding.value_at(lambda) - member.buckling_length_z).abs() < 0.03);
}

#[test]
fn the_interaction_demand_lies_inside_the_polygon_when_compliant() {
    let document = compliant();
    let report = checks::check_full_steel_structure(&document);
    let diagram = diagrams(&document, &report, Some("member-b1")).into_iter().find(|d| d.id == "mn:member-b1").expect("mn");
    let marker = diagram.markers.iter().find(|m| m.id == "demand").expect("demand");
    let scene::Series::Polygon { ring, .. } = &diagram.series[0] else { panic!("polygon first") };
    let ring: Vec<[f64; 2]> = ring.iter().map(|p| [p.x, p.y]).collect();
    assert!(scene::diagram_kit::interaction_polygon::contains(&ring, [marker.at.x, marker.at.y]));
    assert!(marker.utilisation.expect("utilisation") <= 1.0);
}

#[test]
fn focus_narrows_the_diagram_set() {
    let document = compliant();
    let report = checks::check_full_steel_structure(&document);
    let fatigue_only = diagrams(&document, &report, Some("fat-1"));
    assert_eq!(fatigue_only.iter().map(|d| d.id.as_str()).collect::<Vec<_>>(), vec!["sn:fat-1"]);
    assert!(diagrams(&document, &report, Some("nothing")).is_empty());
}

#[test]
fn the_sn_diagram_keeps_the_spectrum_on_log_axes() {
    let document = compliant();
    let report = checks::check_full_steel_structure(&document);
    let diagram = diagrams(&document, &report, Some("fat-1")).remove(0);
    assert!(diagram.axes.iter().all(|a| a.scale == scene::AxisScale::Log10));
    let scene::Series::Scatter { points, .. } = diagram.series.iter().find(|s| matches!(s, scene::Series::Scatter { .. })).expect("scatter") else { panic!("scatter") };
    assert_eq!(points.len(), document.fatigue_details[0].spectrum.len());
}

#[test]
fn golden_scene_matches_committed_bytes() {
    let (scene, _) = scene_of(&compliant(), &[]);
    let actual = canonical_json(&scene);
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧫️fixtures/🏛️scene/🔣️.json");
    scene::projection_kit::test_support::assert_golden(path, &actual);
}

#[test]
fn the_independent_oracle_agrees() {
    let document = compliant();
    let report = checks::check_full_steel_structure(&document);
    let (scene, _) = scene_of(&document, &[]);
    let diagram_set = diagrams(&document, &report, None);
    let diagram_json: Vec<String> = diagram_set.iter().map(canonical_json).collect();
    let input = format!("{{\"scene\":{},\"diagrams\":[{}],\"sections\":[{}]}}", canonical_json(&scene), diagram_json.join(","), document.sections.iter().map(|s| format!("{{\"id\":\"{}\",\"area\":{},\"h\":{},\"b\":{},\"tw\":{},\"tf\":{}}}", s.id, s.area, s.h, s.b, s.tw, s.tf)).collect::<Vec<_>>().join(","));
    let script = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🔮️oracles/🐍️.py");
    let output = norm::python_oracle::run_ok(script, &[], Some(input.as_bytes()));
    let verdict = String::from_utf8_lossy(&output.stdout);
    assert!(verdict.contains("\"ok\": true"), "oracle verdict: {verdict}");
}

#[test]
fn projecting_a_thousand_members_stays_interactive() {
    let mut document = compliant();
    let template = document.members[0].clone();
    document.members = (0..1000).map(|i| model::SteelMember { id: format!("m{i}"), ..template.clone() }).collect();
    document.member_actions.clear();
    document.joints.clear();
    let report = CheckReport::default();
    let started = std::time::Instant::now();
    let scene = project(&SceneInput { document: &document, report: &report, placements: &[], revision: revision() });
    assert!(scene.entities.len() >= 1000);
    eprintln!("[latency] en1993/scene-cold-1000 p50_ms={0:.3} p95_ms={0:.3} budget_ms={1:.1}", started.elapsed().as_secs_f64() * 1e3, 2000.0);
    assert!(started.elapsed().as_millis() < 2000, "projection took {:?}", started.elapsed());
}

#[test]
fn members_are_coloured_per_station_from_the_check_stations() {
    let (scene, report) = scene_of(&overloaded(), &[]);
    let member = entity(&scene, "member-b1");
    let per = member.utilisation.as_ref().and_then(|b| b.per_station.as_ref()).expect("per-station values");
    assert!(per.len() >= 3 && per.windows(2).all(|w| w[0].at < w[1].at));
    let worst = report.checks.iter().filter(|c| c.subject.entity_id == "member-b1" && !c.stations.is_empty()).map(|c| c.utilization).fold(0.0_f64, f64::max);
    assert!((per.iter().map(|s| s.value).fold(0.0, f64::max) - worst).abs() < 1e-9);
    let scene::Geometry::Member(geometry) = &member.geometry else { panic!("member geometry") };
    assert_eq!(geometry.stations.len(), per.len());
}

#[test]
fn a_governing_buckling_check_annotates_its_critical_load_factor() {
    let (scene, _) = scene_of(&overloaded(), &[]);
    let member = entity(&scene, "member-b1");
    let overlay = scene.entities.iter().filter(|e| e.entity.id.starts_with("member-b1:mode-")).find_map(|e| e.appearance.deformation.as_ref()).expect("overlay");
    let _ = member;
    assert!(overlay.load_factor.is_some_and(|a| a.is_finite() && a > 0.0), "{:?}", overlay.load_factor);
    assert!(scene.annotations.iter().any(|a| matches!(a, scene::Annotation::Label { id, .. } if id == "member-b1:alpha-cr")));
}


#[test]
fn the_mode_track_reveals_one_eigenform_layer_per_keyframe() {
    for document in [compliant(), overloaded()] {
        let (scene, _) = scene_of(&document, &[]);
        assert_eq!(scene.validate(), Ok(()));
        let layers: BTreeSet<_> = scene.entities.iter().map(|e| e.layer.as_str()).filter(|l| l.starts_with("mode-")).collect();
        let modes: Vec<_> = scene.tracks.iter().filter(|t| t.kind == scene::TrackKind::Mode).collect();
        assert_eq!(modes.len(), usize::from(layers.len() >= 2));
        for track in modes {
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

#[test]
fn the_cold_formed_support_shows_the_bearing_patch_and_the_web_crippling_zone() {
    let document = En1993Snapshot::special_parts_showcase();
    let (scene, _) = scene_of(&document, &[]);
    for id in ["cf-1:bearing", "cf-1:crippling"] {
        assert!(matches!(entity(&scene, id).geometry, scene::Geometry::Plate(_)), "{id}");
    }
}

#[test]
fn the_showcase_joint_rows_show_bolts_strips_welds_handles_and_the_moment_rotation_diagram() {
    let document = En1993Snapshot::special_parts_showcase();
    let (scene, report) = scene_of(&document, &[]);
    for id in ["row-1", "row-1:bolts", "row-1:tstub", "row-2:bolts", "joint-ep:weld-tension", "joint-ep:weld-compression", "joint-ep:weld-web", "joint-ep:components"] {
        entity(&scene, id);
    }
    assert!(matches!(entity(&scene, "row-1:bolts").geometry, scene::Geometry::Fastener(_)));
    assert!(!scene.entities.iter().any(|e| e.entity.id == "joint-ep:bolts"), "explicit rows replace the derived bolt grid");
    let selection = vec![scene::EntityRef::new(scene::EntityKind::Group, "row-1", "jointRows[id=row-1]")];
    let tags: BTreeSet<_> = handles(&document, &[], &selection).into_iter().map(|h| h.id).collect();
    assert!(["row-1:row-lever", "row-1:row-m", "row-1:row-edge", "row-1:row-bolts"].iter().all(|t| tags.contains(*t)));
    let diagrams = diagrams(&document, &report, None);
    let mphi = diagrams.iter().find(|d| d.id == "mphi:joint-ep").expect("moment-rotation diagram");
    assert_eq!(mphi.markers.len(), 1);
    let zones: Vec<&str> = mphi.series.iter().filter(|x| matches!(x, scene::Series::Band { .. })).map(|x| x.id()).collect();
    assert_eq!(zones, ["zone-pinned", "zone-semi-rigid", "zone-rigid"]);
    let band = |id: &str| mphi.series.iter().find_map(|x| match x { scene::Series::Band { id: i, lower, upper, .. } if i == id => Some((lower.clone(), upper.clone())), _ => None }).expect("band");
    let (pinned_lo, pinned_up) = band("zone-pinned");
    let (semi_lo, semi_up) = band("zone-semi-rigid");
    let (rigid_lo, rigid_up) = band("zone-rigid");
    assert!(pinned_lo.iter().all(|p| p.y == 0.0) && pinned_up.iter().zip(&semi_lo).all(|(a, b)| a.y == b.y), "zones tile the plane without gaps");
    assert!(semi_up.iter().zip(&rigid_lo).all(|(a, b)| a.y == b.y) && semi_lo.iter().zip(&semi_up).all(|(a, b)| a.y <= b.y), "semi-rigid lies between the pinned and the rigid boundary");
    let y_top = rigid_up[0].y;
    assert!(rigid_up.iter().all(|p| p.y == y_top) && rigid_lo.iter().all(|p| p.y <= y_top));
    for id in ["chi-lt:restraint-1", "mcr-mode:restraint-1", "hollow:hollow-1"] {
        assert!(diagrams.iter().any(|d| d.id == id), "{id}");
    }
}

#[test]
fn a_parametric_section_stands_in_its_own_row_with_handles() {
    let document = En1993Snapshot::special_parts_showcase();
    let (scene, _) = scene_of(&document, &[]);
    let id = &document.parametric_sections[0].id;
    assert!(matches!(entity(&scene, id).geometry, scene::Geometry::Member(_)));
    let selection = vec![scene::EntityRef::new(scene::EntityKind::Member, id, format!("parametricSections[id={id}]"))];
    assert!(handles(&document, &[], &selection).len() >= 3);
}

fn fold_into(document: &mut En1993Snapshot, commit: &scene::projection_kit::PlacementCommit) {
    use scene::projection_kit::PlacementOp;
    for op in &commit.ops {
        match op {
            PlacementOp::Insert(row) => document.placements.push(model::Placement { id: row.entity_id.clone(), x: row.origin.x, y: row.origin.y, z: row.origin.z, yaw: row.yaw }),
            PlacementOp::Remove(id) => document.placements.retain(|row| &row.id != id),
            PlacementOp::Set(writes) => {
                for write in writes {
                    let (id, leaf) = write.path.strip_prefix("placements[id=").and_then(|rest| rest.split_once("].")).expect("path");
                    let row = document.placements.iter_mut().find(|row| row.id == id).expect("row");
                    match leaf {
                        "x" => row.x = write.value,
                        "y" => row.y = write.value,
                        "z" => row.z = write.value,
                        _ => row.yaw = write.value,
                    }
                }
            }
        }
    }
}

#[test]
fn dragging_an_entity_commits_one_undoable_event_and_the_reload_stands_at_the_same_pose() {
    use scene::projection_kit::{placements_of, PlacementCommit, PlacementMove};
    let mut document = compliant();
    let before = placements_of(&document);
    assert!(before.is_empty());
    let target = scene::Vec3::new(7.5, -2.0, 0.0);
    let commit = PlacementCommit::of(&before, &[PlacementMove { entity_id: "member-b1".into(), origin: target.clone(), yaw: 0.75 }]);
    assert_eq!(commit.ops.len(), 1);
    fold_into(&mut document, &commit);
    let reloaded = placements_of(&document);
    assert_eq!(reloaded.len(), 1);
    let pose = Layout::of(&document, &reloaded).member("member-b1").expect("pose").frame.origin.clone();
    assert_eq!(pose, target);
    let (moved_scene, _) = scene_of(&document, &reloaded);
    assert_eq!(moved_scene.validate(), Ok(()));
    let again = PlacementCommit::of(&reloaded, &[PlacementMove { entity_id: "member-b1".into(), origin: target, yaw: 0.75 }]);
    assert!(again.is_empty());
    fold_into(&mut document, &commit.inverse(&before));
    assert_eq!(placements_of(&document), before);
}

#[test]
fn the_detail_sketches_exist_in_3d_and_as_diagrams_with_table_twins() {
    let mut document = compliant();
    document.joints[0].beam_span = 6.0;
    let (scene, report) = scene_of(&document, &[]);
    assert_eq!(scene.validate(), Ok(()));
    let panel = document.plated_panels[0].id.clone();
    let joint = document.joints[0].id.clone();
    for id in [format!("{panel}:ineffective"), format!("{joint}:gusset"), format!("{joint}:haunch")] {
        assert!(scene.entities.iter().any(|e| e.entity.id == id), "missing {id}");
    }
    let diagrams = diagrams(&document, &report, None);
    for id in [format!("effective-width:{panel}"), format!("gusset:{joint}"), format!("haunch:{joint}")] {
        let diagram = diagrams.iter().find(|d| d.id == id).unwrap_or_else(|| panic!("missing {id}"));
        assert!(diagram.table_twin.rows() > 0);
        assert!(!diagram.title.de.is_empty() && !diagram.title.en.is_empty());
    }
    let only = diagrams_for(&document, &report, &panel);
    assert!(only.iter().all(|d| d.id.contains(&panel)));
}

fn diagrams_for(document: &En1993Snapshot, report: &CheckReport, id: &str) -> Vec<scene::Diagram2d> {
    diagrams(document, report, Some(id))
}

#[test]
fn a_sway_frame_below_the_elastic_limit_shows_the_deformed_frame_and_the_buckling_mode() {
    let mut document = En1993Snapshot::frame_portal_showcase();
    document.frame_nodes.iter_mut().filter(|n| n.load_z != 0.0).for_each(|n| n.load_z *= 6.0);
    let (scene, _) = scene_of(&document, &[]);
    assert_eq!(scene.validate(), Ok(()));
    let bar = document.frame_bars[0].id.clone();
    let mode = scene.entities.iter().find(|e| e.entity.id == format!("{bar}:{}", tracks::LAYER_MODE_FRAME)).expect("frame mode sibling");
    let overlay = mode.appearance.deformation.as_ref().expect("mode overlay");
    assert_eq!(overlay.kind, scene::DeformationKind::Mode);
    assert_eq!(overlay.motion, scene::Motion::Harmonic);
    assert!(overlay.load_factor.is_some_and(|a| a > 1.0));
    assert!(overlay.samples.len() > 2);
    assert!(scene.tracks.iter().any(|t| t.kind == scene::TrackKind::Mode));
    let bar_entity = scene.entities.iter().find(|e| e.entity.id == bar).expect("bar");
    assert!(bar_entity.appearance.deformation.as_ref().is_some_and(|o| o.kind == scene::DeformationKind::Deformed));
}

#[test]
fn every_example_and_showcase_projection_obeys_the_scene_diagram_and_handle_laws() {
    let mut documents = vec![compliant(), overloaded(), En1993Snapshot::frame_portal_showcase(), En1993Snapshot::annex_showcase(), En1993Snapshot::special_parts_showcase()];
    let mut welded = compliant();
    welded.joints[0].kind = semio_s_artifact_norm_en1993_model::JointKind::Welded;
    welded.joints[0].weld_throat = 0.005;
    welded.joints[0].weld_length = 0.2;
    documents.push(welded);
    for document in documents {
        let (scene, report) = scene_of(&document, &[]);
        let selection: Vec<scene::EntityRef> = scene.entities.iter().map(|e| e.entity.clone()).collect();
        scene::projection_kit::test_support::assert_projection_laws(&scene, &diagrams(&document, &report, None), &handles(&document, &[], &selection));
    }
}

#[test]
fn the_full_showcase_projects_a_valid_scene_with_an_entity_per_populated_collection() {
    let (scene, _) = scene_of(&semio_s_artifact_norm_en1993_showcase::full_showcase(), &[]);
    assert!(scene.validate().is_ok());
    assert!(!scene.entities.is_empty());
    assert!(scene.entities.iter().all(|e| e.content_hash == e.compute_hash()));
}

fn showcase_scene() -> (StructuralScene, CheckReport, En1993Snapshot) {
    let document = semio_s_artifact_norm_en1993_showcase::full_showcase();
    let (scene, report) = scene_of(&document, &[]);
    (scene, report, document)
}

#[test]
fn a_stiffened_pressurised_panel_stands_with_outline_sections_pressure_arrows_and_a_deflection_surface() {
    let (scene, _, _) = showcase_scene();
    for id in ["pp-1:stiffener-1", "pp-1:stiffener-2"] {
        let stiffener = entity(&scene, id);
        let scene::Geometry::Member(member) = &stiffener.geometry else { panic!("a stiffener is an extruded section") };
        assert!(matches!(member.section, scene::SectionShape::Polygon { .. }));
    }
    assert!(matches!(&entity(&scene, "pp-1:pressure-load").geometry, scene::Geometry::Load(scene::LoadGlyph::Area { .. })));
    let overlay = entity(&scene, "pp-1").appearance.deformation.as_ref().expect("deflection surface");
    assert!(overlay.surface.len() > 100 && overlay.surface.iter().all(|s| s.delta.z <= 0.0) && overlay.scale >= 1.0);
}

#[test]
fn the_panel_offers_five_outline_grips_and_a_pressure_grip_that_write_typed_leaves() {
    let (_, _, document) = showcase_scene();
    let all = handles(&document, &[], &[scene::EntityRef::new(scene::EntityKind::Plate, "pp-1", "platedPanels[id=pp-1]")]);
    assert_handle_laws(&all);
    let find = |tag: &str| all.iter().find(|h| h.id == format!("pp-1:{tag}")).unwrap_or_else(|| panic!("grip {tag}"));
    let fields = [("stiffener-shape", "stiffenerShape"), ("stiffener-depth", "stiffenerDepth"), ("stiffener-thickness", "stiffenerThickness"), ("stiffener-flange-width", "stiffenerFlangeWidth"), ("stiffener-flange-thickness", "stiffenerFlangeThickness"), ("pressure", "lateralPressure")];
    for (tag, field) in fields {
        assert_eq!(find(tag).writes.len(), 1);
        assert_eq!(find(tag).writes[0].path, format!("platedPanels[id=pp-1].{field}"), "{tag}");
    }
    let shape = find("stiffener-shape");
    assert_eq!(shape.kind, scene::HandleKind::ChoiceStepper);
    let tokens: Vec<_> = (0..4).map(|s| shape.writes_at(f64::from(s))[0].token.clone().expect("token")).collect();
    assert_eq!(tokens, ["None", "Flat", "Tee", "Angle"]);
    let warp = handles(&document, &[], &[scene::EntityRef::new(scene::EntityKind::Member, "member-torsion", "members[id=member-torsion]")]);
    let warp = warp.iter().find(|h| h.id.ends_with(":warping-support")).expect("warping grip");
    assert_eq!(warp.writes[0].token_at(2.4), Some("FixedFork"));
    let pressure = find("pressure").writes_at(0.2);
    assert!((pressure[0].value - 0.2 * handles::PRESSURE_PER_METRE).abs() < 1.0);
}

#[test]
fn a_torsion_member_shows_its_bimoment_diagram_and_the_warping_stress_field() {
    let (scene, _, document) = showcase_scene();
    let member = document.members.iter().find(|m| m.id == "member-torsion").expect("torsion member");
    assert_eq!(member.warping_support, semio_s_artifact_norm_en1993_model::WarpingSupport::Fixed);
    assert!(matches!(&entity(&scene, "member-torsion:bimoment").geometry, scene::Geometry::Plate(_)));
    let shell = entity(&scene, "member-torsion:warping-stress");
    let scene::ColourChannel::Fields { active, fields } = &shell.appearance.channel else { panic!("a field channel") };
    assert_eq!(active, "warping-stress");
    assert!(matches!(&fields[0].samples, scene::FieldSamples::Station { values } if values.len() > 2 && values.iter().any(|v| v.value > 0.0)));
    assert!(scene.legends.iter().any(|l| l.id == "warping-stress"));
}

#[test]
fn a_hybrid_girder_shows_flange_and_web_grades_as_colour_fields() {
    let (scene, _, _) = showcase_scene();
    let strength = |id: &str| match &entity(&scene, id).appearance.channel {
        scene::ColourChannel::Fields { fields, .. } => match &fields[0].samples {
            scene::FieldSamples::Instance { values } => values[0],
            other => panic!("instance samples, got {other:?}"),
        },
        other => panic!("field channel, got {other:?}"),
    };
    assert!((strength("member-hybrid:grade-flange-top") - 460.0).abs() < 1e-9 && (strength("member-hybrid:grade-web") - 355.0).abs() < 1e-9);
    assert!(scene.legends.iter().any(|l| l.id == "yield-strength"));
}

#[test]
fn a_rainflow_detail_shows_histogram_bars_a_damage_bar_and_both_diagrams() {
    let (scene, report, document) = showcase_scene();
    assert!(scene.entities.iter().any(|e| e.entity.id.starts_with("fat-rf:rainflow-")));
    assert!(matches!(&entity(&scene, "fat-rf:damage").geometry, scene::Geometry::Plate(_)));
    let ours = diagrams(&document, &report, Some("fat-rf"));
    let histogram = ours.iter().find(|d| d.id == "rainflow:fat-rf").expect("histogram");
    let counted: f64 = histogram.table_twin.columns[1].values.iter().sum();
    assert!((counted - 5.0).abs() < 1e-9, "half cycles sum to 5 for the 11-point history, got {counted}");
    assert!(ours.iter().any(|d| d.id == "damage:fat-rf"));
}

#[test]
fn a_fatigue_detail_draws_the_s_n_reference_ranges_as_planes_in_descending_order() {
    let (scene, _, document) = showcase_scene();
    let detail = document.fatigue_details.iter().find(|d| d.id == "fat-rf").expect("detail");
    let limits = fatigue_views::limits(detail);
    let category = f64::from(detail.category);
    assert!(limits.iter().all(|(_, range, _, _)| *range > 0.0 && *range <= category + 1e-9));
    assert!(limits.windows(2).all(|w| w[0].1 > w[1].1), "C above D above L: {limits:?}");
    for (tag, _, _, _) in &limits {
        assert!(matches!(&entity(&scene, &format!("fat-rf:limit-{tag}")).geometry, scene::Geometry::Plate(_)));
    }
}

#[test]
fn placed_entities_get_move_and_rotate_grips_and_sections_get_dimension_grips() {
    let document = compliant();
    let all = handles(&document, &[], &[scene::EntityRef::new(scene::EntityKind::Member, "member-b1", "members[id=member-b1]")]);
    assert_handle_laws(&all);
    scene::projection_kit::test_support::assert_angular_handle_laws(&all);
    let tags: BTreeSet<_> = all.iter().map(|h| h.id.rsplit(':').next().unwrap_or_default().to_string()).collect();
    for tag in ["move-x", "move-y", "move-yaw", "section-h", "section-b", "section-tw", "section-tf"] {
        assert!(tags.contains(tag), "missing {tag}");
    }
    let drag = all.iter().find(|h| h.id == "member-b1:move-x").expect("move grip").writes_at(0.0);
    assert_eq!(drag[0].path, "placements[id=member-b1].x");
    let height = all.iter().find(|h| h.id == "member-b1:section-h").expect("section grip");
    assert!(height.writes[0].path.starts_with("sections[id=") && height.writes[0].path.ends_with("].h"));
}

#[test]
fn nested_action_rows_get_magnitude_grips_on_their_glyphs() {
    let document = compliant();
    let (scene, _) = scene_of(&document, &[]);
    let glyph = entity(&scene, "act-b1-g:n");
    assert!(glyph.handles.iter().any(|h| h == "act-b1-g:n:magnitude"));
    let all = handles(&document, &[], &[glyph.entity.clone()]);
    assert_handle_laws(&all);
    let grip = all.iter().find(|h| h.id == "act-b1-g:n:magnitude").expect("magnitude grip");
    assert_eq!(grip.writes[0].path, "memberActions[id=act-b1-g].action.n");
    let doubled = grip.writes_at(grip.writes[0].map.base.abs() / handles::FORCE_PER_METRE);
    assert!((doubled[0].value.abs() - 2.0 * grip.writes[0].map.base.abs()).abs() <= 250.0);
}

#[test]
fn materials_stand_as_gripped_strength_columns_and_a_failing_grade_colours_the_column() {
    let document = compliant();
    let (scene, _) = scene_of(&document, &[]);
    let material = &document.materials[0];
    let column = entity(&scene, &material.id);
    assert!(matches!(&column.geometry, scene::Geometry::Plate(p) if (p.thickness - material.fy / handles::STRENGTH_PER_METRE).abs() < 1e-9));
    assert!(column.handles.iter().any(|h| *h == format!("{}:yield-strength", material.id)));
    let all = handles(&document, &[], &[column.entity.clone()]);
    assert_handle_laws(&all);
    let grip = all.iter().find(|h| h.id == format!("{}:yield-strength", material.id)).expect("strength grip");
    assert_eq!(grip.writes[0].path, format!("materials[id={}].fy", material.id));
    let mut failing = compliant();
    failing.materials[0].fy = 1.0e9;
    failing.materials[0].fu = 1.1e9;
    let (scene, _) = scene_of(&failing, &[]);
    assert!(matches!(entity(&scene, &failing.materials[0].id).utilisation.as_ref().map(|u| u.status), Some(scene::CheckStatus::Fail)));
}

#[test]
fn load_cases_stand_as_cards_and_their_action_glyphs_sit_on_one_layer_per_case() {
    let document = compliant();
    let (scene, _) = scene_of(&document, &[]);
    assert!(document.load_cases.iter().all(|c| scene.entities.iter().any(|e| e.entity.id == c.id)));
    let row = document.member_actions.iter().find(|a| a.id == "act-b1-g").expect("action row");
    assert_eq!(entity(&scene, "act-b1-g:n").layer, kit::case_layer(&row.load_case_id));
    let layers: BTreeSet<String> = scene.layers.iter().filter(|l| l.id.starts_with(kit::LAYER_CASE_PREFIX)).map(|l| l.id.clone()).collect();
    assert!(layers.contains(&kit::case_layer(&row.load_case_id)));
    let track = scene.tracks.iter().find(|t| matches!(t.kind, scene::TrackKind::Combination));
    assert_eq!(track.is_some(), layers.len() >= 2);
    if let Some(track) = track {
        assert!(track.keyframes.iter().all(|k| k.layer.as_ref().is_some_and(|l| layers.contains(l))));
    }
    assert_eq!(scene.validate(), Ok(()));
}

#[test]
fn joint_plates_and_foundation_blocks_own_their_plates_bolts_block_grout_and_anchors_with_grips() {
    let document = En1993Snapshot::special_parts_showcase();
    let (scene, report) = scene_of(&document, &[]);
    assert_eq!(scene.validate(), Ok(()), "violations");
    for id in ["plate-fin", "plate-fin:plate-1", "plate-fin:bolt-3", "plate-flange:plate-1", "plate-flange:bolt-6", "plate-web:plate-1", "plate-web:plate-2", "plate-web:bolt-2", "block-bp", "block-bp:block", "block-bp:grout", "block-bp:anchor-1", "block-bp:anchor-2", "block-bp:c-strip", "block-bp:cone-1", "block-bp:cone-2"] {
        entity(&scene, id);
    }
    assert!(matches!(entity(&scene, "block-bp:block").geometry, scene::Geometry::Solid(_)));
    assert!(matches!(entity(&scene, "block-bp:anchor-1").geometry, scene::Geometry::Fastener(_)));
    assert!(scene.entities.iter().filter(|e| e.entity.id.starts_with("plate-fin:bolt-")).count() == 3);
    let selection: Vec<_> = ["plate-fin", "plate-flange", "block-bp"].iter().map(|id| scene.entities.iter().find(|e| e.entity.id == *id).expect("group").entity.clone()).collect();
    let all = handles(&document, &[], &selection);
    assert_handle_laws(&all);
    let tags: BTreeSet<_> = all.iter().map(|h| h.id.clone()).collect();
    for t in ["plate-fin:plate-thickness", "plate-fin:plate-eccentricity", "plate-fin:plate-rows", "plate-flange:plate-pitch", "block-bp:block-depth", "block-bp:grout-thickness", "block-bp:anchor-count", "block-bp:anchor-embedment", "block-bp:base-plate-thickness"] {
        assert!(tags.contains(t), "missing grip {t}");
    }
    let owned = scene.entities.iter().find(|e| e.entity.id == "block-bp").expect("block group");
    assert!(owned.handles.iter().all(|h| tags.contains(h)));
    let anchors = report.checks.iter().find(|c| c.id == "en1993.1-8.6.2.anchort.block-bp").expect("anchor check");
    assert!(anchors.utilization > 0.0);
    for (id, check) in [("block-bp:c-strip", "en1993.1-8.6.2.cbend.block-bp"), ("block-bp:cone-1", "en1993.1-8.6.2.cone.block-bp")] {
        let zone = entity(&scene, id);
        assert!(zone.appearance.opacity < 1.0, "{id} is translucent");
        let bound = zone.utilisation.as_ref().expect("zone utilisation");
        close_to(bound.ratio, report.checks.iter().find(|c| c.id == check).expect("zone check").utilization);
    }
}

fn close_to(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "{a} vs {b}");
}

#[test]
fn the_design_point_diagrams_exist_validate_and_carry_the_operating_point() {
    let document = semio_s_artifact_norm_en1993_showcase::full_showcase();
    let report = checks::check_full_steel_structure(&document);
    let all = diagrams(&document, &report, None);
    for id in ["pile-mv:pile-1", "bridge-sn:br-1", "crane-wheel:crane-1", "panel-rho:pp-1", "load-case-n:g-permanent"] {
        let diagram = all.iter().find(|d| d.id == id).unwrap_or_else(|| panic!("missing {id}"));
        assert_eq!(diagram.validate(), Ok(()), "{id}");
        assert!(diagram.table_twin.rows() > 0, "{id} has no table twin");
        assert!(!diagram.markers.is_empty() || id.starts_with("load-case"), "{id} has no marker");
    }
    let sn = all.iter().find(|d| d.id == "bridge-sn:br-1").expect("sn");
    assert_eq!(sn.markers[0].at.x, 2.0e6);
    assert!(sn.axes.iter().all(|a| a.scale == scene::AxisScale::Log10));
    let rho = all.iter().find(|d| d.id == "panel-rho:pp-1").expect("rho");
    assert!((0.0..=1.0).contains(&rho.markers[0].at.y));
    let crane = all.iter().find(|d| d.id == "crane-wheel:crane-1").expect("crane");
    let peak = crane.table_twin.columns[1].values.iter().cloned().fold(0.0, f64::max);
    assert!((crane.markers[0].at.y - peak).abs() < 1e-6 * peak.max(1.0));
}

#[test]
fn every_check_carrying_collection_gets_a_utilisation_ladder() {
    let document = semio_s_artifact_norm_en1993_showcase::full_showcase();
    let report = checks::check_full_steel_structure(&document);
    let all = diagrams(&document, &report, None);
    let ids: BTreeSet<_> = all.iter().map(|d| d.id.clone()).collect();
    let owners = document.tower_legs.iter().map(|x| x.id.clone()).chain(document.tension_components.iter().map(|x| x.id.clone())).chain(document.storeys.iter().map(|x| x.id.clone())).chain(document.built_up_members.iter().map(|x| x.id.clone()));
    for id in owners {
        if report.checks.iter().any(|c| c.subject.entity_id == id && c.utilization.is_finite()) {
            assert!(ids.iter().any(|d| d.starts_with("ladder:") && d.ends_with(&format!(":{id}"))), "no ladder for {id}");
        }
    }
    for diagram in all.iter().filter(|d| d.id.starts_with("ladder:")) {
        assert_eq!(diagram.validate(), Ok(()), "{}", diagram.id);
        assert_eq!(diagram.table_twin.columns[1].values.len(), diagram.annotations.len());
    }
}

#[test]
fn every_frame_bar_gets_n_v_and_m_courses_that_validate_and_match_the_statical_moment() {
    let document = En1993Snapshot::frame_portal_showcase();
    let report = checks::check_full_steel_structure(&document);
    let all = diagrams(&document, &report, None);
    for bar in &document.frame_bars {
        for key in ["n", "v", "m"] {
            let diagram = all.iter().find(|d| d.id == format!("frame-{key}:{}", bar.id));
            if let Some(diagram) = diagram {
                assert_eq!(diagram.validate(), Ok(()), "{}", diagram.id);
                assert!(diagram.table_twin.columns.len() >= 3);
            }
        }
    }
    let column = all.iter().find(|d| d.id == "frame-m:bar-1").expect("moment course of the left column");
    let first = &column.table_twin.columns[1].values;
    let (head, foot) = (first.last().unwrap().abs(), first.first().unwrap().abs());
    assert!(foot > 1.0 || head > 1.0, "the column carries bending under the horizontal load");
    let focused = diagrams(&document, &report, Some("bar-1"));
    assert!(focused.iter().any(|d| d.id == "frame-m:bar-1") && focused.iter().all(|d| !d.id.ends_with(":bar-2")));
}

#[test]
fn frame_actions_get_glyphs_with_grips_on_their_load_case_layer_and_per_case_courses_and_an_envelope() {
    let document = En1993Snapshot::frame_cases_showcase();
    let (scene, report) = scene_of(&document, &[]);
    assert_eq!(scene.validate(), Ok(()));
    for (id, layer) in [("fa-g-beam:line", "case:g-permanent"), ("fa-q-beam:line", "case:q-imposed"), ("fa-w-eaves:force", "case:w-wind")] {
        let entity = scene.entities.iter().find(|e| e.entity.id == id).unwrap_or_else(|| panic!("glyph {id}"));
        assert_eq!(entity.layer, layer);
        assert!(!entity.handles.is_empty(), "{id} carries grips");
    }
    for case in ["g-permanent", "q-imposed", "w-wind"] {
        assert!(scene.entities.iter().any(|e| e.entity.id.starts_with("bar-2:moment:case:") && e.layer == format!("case:{case}") && e.entity.id.contains(case)), "moment surface of {case}");
    }
    assert!(scene.entities.iter().any(|e| e.entity.id.starts_with("bar-2:moment:envelope-max") && e.layer == "frame-envelope") && scene.layers.iter().any(|l| l.id == "frame-envelope"));
    assert!(scene.layers.iter().filter(|l| l.id.starts_with("case:")).count() >= 3, "the load-case track steps through the cases");
    let sway: Vec<_> = scene.entities.iter().filter(|e| e.entity.id.ends_with(":imperfection")).collect();
    assert!(!sway.is_empty() && sway.iter().all(|e| e.layer == "frame-imperfection" && e.handles.is_empty()) && scene.layers.iter().any(|l| l.id == "frame-imperfection"), "the equivalent sway forces are glyphs on a toggleable layer");
    let all = diagrams(&document, &report, None);
    for key in ["n", "v", "m"] {
        for suffix in ["case:g-permanent", "case:q-imposed", "case:w-wind", "envelope"] {
            let diagram = all.iter().find(|d| d.id == format!("frame-{key}-{}:bar-2", suffix.replace(':', "-"))).unwrap_or_else(|| panic!("frame-{key}-{suffix}:bar-2"));
            assert_eq!(diagram.validate(), Ok(()), "{}", diagram.id);
        }
    }
    let envelope = all.iter().find(|d| d.id == "frame-m-envelope:bar-2").unwrap();
    let (max, min) = (&envelope.table_twin.columns[1].values, &envelope.table_twin.columns[2].values);
    assert!(max.iter().zip(min).all(|(hi, lo)| hi >= lo), "the envelope maximum bounds the minimum");
    let handles = handles(&document, &[], &[scene::EntityRef::new(scene::EntityKind::Load, "fa-w-eaves:force", "frameActions[id=fa-w-eaves]")]);
    assert_eq!(handles.len(), 2, "fx and fz");
}

#[test]
fn a_tower_leg_gets_its_n_m_interaction_with_the_design_point_from_the_tower_and_member_rows() {
    let document = semio_s_artifact_norm_en1993_showcase::full_showcase();
    let report = checks::check_full_steel_structure(&document);
    for tower in &document.tower_legs {
        let all = diagrams(&document, &report, Some(&tower.id));
        let diagram = all.iter().find(|d| d.id == format!("tower-nm:{}", tower.id)).expect("tower N-M diagram");
        assert_eq!(diagram.validate(), Ok(()));
        let point = checks::tower_design_point(&document, tower);
        assert!((diagram.markers[0].at.x - point.n / 1e3).abs() < 1e-9 && (diagram.markers[0].at.y - point.my / 1e3).abs() < 1e-9);
        let moments = &diagram.table_twin.columns[1].values;
        assert!(moments.first().unwrap() > moments.last().unwrap(), "the moment capacity falls to zero at N_pl,Rd");
    }
}

#[test]
fn a_member_failing_the_brittle_fracture_check_carries_a_failure_badge() {
    let mut document = compliant();
    let (section_id, material_id) = { let m = document.members.iter().find(|m| m.id == "member-b1").expect("member"); (m.section_id.clone(), m.material_id.clone()) };
    document.sections.iter_mut().filter(|s| s.id == section_id).for_each(|s| s.tf = 0.2);
    document.materials.iter_mut().filter(|m| m.id == material_id).for_each(|m| { m.grade = "S355".into(); m.subgrade = "J2".into(); });
    let (scene, report) = scene_of(&document, &[]);
    assert!(report.checks.iter().any(|c| c.id == "en1993.1-10.t.member-b1" && c.status == norm::CheckStatus::Fail), "a 200 mm flange of S355 J2 exceeds Table 2.1");
    assert!(scene.annotations.iter().any(|a| matches!(a, scene::Annotation::FailureGlyph { id, .. } if id == "member-b1:failure-brittle-fracture")));
    assert!(scene.annotations.iter().any(|a| matches!(a, scene::Annotation::Label { id, .. } if id == "member-b1:brittle-fracture")));
    assert_eq!(scene.validate(), Ok(()));
}

#[test]
fn a_member_gets_its_toughness_diagram_with_the_live_design_point() {
    let mut document = compliant();
    let (section_id, material_id) = { let m = document.members.iter().find(|m| m.id == "member-b1").expect("member"); (m.section_id.clone(), m.material_id.clone()) };
    document.sections.iter_mut().filter(|s| s.id == section_id).for_each(|s| s.tf = 0.045);
    document.materials.iter_mut().filter(|m| m.id == material_id).for_each(|m| { m.grade = "S355".into(); m.subgrade = "J2".into(); });
    document.members.iter_mut().filter(|m| m.id == "member-b1").for_each(|m| m.design_temperature = -20.0);
    let report = checks::check_full_steel_structure(&document);
    let all = diagrams(&document, &report, Some("member-b1"));
    let diagram = all.iter().find(|d| d.id == "toughness:member-b1").expect("toughness diagram");
    assert_eq!(diagram.validate(), Ok(()));
    assert!((diagram.markers[0].at.x + 20.0).abs() < 1e-9 && (diagram.markers[0].at.y - 45.0).abs() < 1e-9, "design point (T_Ed, t) = (-20 °C, 45 mm)");
    assert_eq!(diagram.table_twin.columns[0].values.len(), 7);
    assert!((diagram.table_twin.columns[1].values[0] - 35.0).abs() < 1e-9 && (diagram.table_twin.columns[1].values[6] - 90.0).abs() < 1e-9, "S355 J2 column of the registry at -50 and +10 °C");
    let t_max_at_t_ed = diagram.table_twin.columns[1].values[3];
    assert!((t_max_at_t_ed - 55.0).abs() < 1e-9 && (diagram.markers[0].utilisation.expect("utilisation") - 45.0 / t_max_at_t_ed).abs() < 1e-9);
}

#[test]
fn the_frame_moment_surfaces_are_split_by_sign_and_validate() {
    let document = En1993Snapshot::frame_portal_showcase();
    let (scene, _) = scene_of(&document, &[]);
    assert_eq!(scene.validate(), Ok(()));
    let ids: Vec<&str> = scene.entities.iter().map(|e| e.entity.id.as_str()).filter(|id| id.contains(":moment:")).collect();
    assert!(!ids.is_empty());
    assert!(ids.iter().all(|id| id.contains(":moment:pos") || id.contains(":moment:neg")));
    assert!(scene.entities.iter().all(|e| e.entity.id != format!("{}:moment", document.frame_bars[0].id)));
}

#[test]
fn a_tower_leg_with_member_bending_gets_a_n_m_check_and_a_design_point_off_the_axis() {
    let mut document = semio_s_artifact_norm_en1993_showcase::full_showcase();
    let tower = document.tower_legs[0].clone();
    let rows: Vec<_> = document.member_actions.iter().filter(|a| a.member_id == tower.member_id).cloned().collect();
    assert!(!rows.is_empty(), "the showcase tower member carries member rows");
    document.member_actions.iter_mut().filter(|a| a.member_id == tower.member_id).for_each(|a| a.action.my = 40_000.0);
    let report = checks::check_full_steel_structure(&document);
    assert!(report.checks.iter().any(|c| c.id == format!("en1993.3-1.tower.nm.{}", tower.id)));
    let point = checks::tower_design_point(&document, &tower);
    assert!(point.my > 0.0);
    let all = diagrams(&document, &report, Some(&tower.id));
    let diagram = all.iter().find(|d| d.id == format!("tower-nm:{}", tower.id)).expect("tower N-M diagram");
    assert!(diagram.markers[0].at.y > 0.0);
}

fn tower_document_of_kind(kind: semio_s_artifact_norm_en1993_model::SectionKind, h: f64, b: f64, t: f64, area: f64, iy: f64, iz: f64) -> (En1993Snapshot, String) {
    let mut document = semio_s_artifact_norm_en1993_showcase::full_showcase();
    let tower = document.tower_legs[0].clone();
    let section_id = document.members.iter().find(|m| m.id == tower.member_id).map(|m| m.section_id.clone()).expect("member");
    document.sections.iter_mut().filter(|s| s.id == section_id).for_each(|s| {
        s.kind = kind;
        (s.h, s.b, s.tw, s.tf, s.r, s.area, s.iy, s.iz) = (h, b, t, t, 0.0, area, iy, iz);
        (s.w_el_y, s.w_el_z, s.w_pl_y, s.w_pl_z) = (iy / (h / 2.0), iz / (b / 2.0), 1.3 * iy / (h / 2.0), 1.3 * iz / (b / 2.0));
    });
    document.member_actions.iter_mut().filter(|a| a.member_id == tower.member_id).for_each(|a| a.action.my = 4_000.0);
    (document, tower.id)
}

#[test]
fn a_circular_hollow_tower_leg_gets_its_n_m_diagram_with_the_hollow_section_resistance() {
    let (document, tower) = tower_document_of_kind(semio_s_artifact_norm_en1993_model::SectionKind::Chs, 0.1683, 0.1683, 0.0063, 3.205e-3, 1.0526e-5, 1.0526e-5);
    let report = checks::check_full_steel_structure(&document);
    assert!(report.checks.iter().any(|c| c.id == format!("en1993.3-1.tower.nm.{tower}")));
    let all = diagrams(&document, &report, Some(&tower));
    let diagram = all.iter().find(|d| d.id == format!("tower-nm:{tower}")).expect("tower N-M diagram");
    assert_eq!(diagram.validate(), Ok(()));
    let moments = &diagram.table_twin.columns[1].values;
    assert!(moments.first().unwrap() > moments.last().unwrap() && moments.windows(2).all(|w| w[0] >= w[1] - 1e-9), "M_N,Rd of the tube falls monotonically with N");
}

#[test]
fn an_angle_tower_leg_gets_its_n_m_diagram_on_the_principal_axes() {
    let (document, tower) = tower_document_of_kind(semio_s_artifact_norm_en1993_model::SectionKind::Angle, 0.1, 0.1, 0.01, 1.9e-3, 2.8658e-6, 7.3425e-7);
    let report = checks::check_full_steel_structure(&document);
    assert!(report.checks.iter().any(|c| c.id == format!("en1993.3-1.tower.nm.{tower}")));
    let all = diagrams(&document, &report, Some(&tower));
    let diagram = all.iter().find(|d| d.id == format!("tower-nm:{tower}")).expect("tower N-M diagram");
    assert_eq!(diagram.validate(), Ok(()));
    let moments = &diagram.table_twin.columns[1].values;
    assert!(moments.first().unwrap() > moments.last().unwrap(), "the elastic resistance falls to zero at N_Rd");
}

#[test]
fn the_member_outlines_of_circular_hollow_sections_and_angles_are_tube_and_l_shape() {
    use semio_s_artifact_norm_en1993_model::SectionKind;
    let mut section = semio_s_artifact_norm_en1993_showcase::full_showcase().sections[0].clone();
    (section.h, section.b, section.tw, section.tf) = (0.1683, 0.1683, 0.0063, 0.0063);
    section.kind = SectionKind::Chs;
    assert!(format!("{:?}", kit::section_shape(&section)).contains("Chs"));
    (section.h, section.b, section.tw, section.tf) = (0.1, 0.065, 0.008, 0.008);
    section.kind = SectionKind::Angle;
    assert!(format!("{:?}", kit::section_shape(&section)).contains("Angle"));
}

#[test]
fn frame_and_special_part_collections_and_placements_are_registered_kinds_resolved_per_row() {
    let mut frame = En1993Snapshot::frame_cases_showcase();
    let mut parts = En1993Snapshot::special_parts_showcase();
    frame.placements = vec![model::Placement { id: frame.members[0].id.clone(), x: 1.0, y: 2.0, z: 0.0, yaw: 0.0 }];
    parts.placements = frame.placements.clone();
    for (id, collection) in [("frameAction", "frameActions"), ("frameMemberBasis", "frameMemberBases"), ("jointPlate", "jointPlates"), ("foundationBlock", "foundationBlocks"), ("placement", "placements")] {
        let hit = ENTITY_KINDS.iter().find(|k| k.id == id).expect("kind");
        assert_eq!(hit.collection, collection);
        assert!(!hit.label_en.is_empty() && !hit.label_de.is_empty());
    }
    let owned = |document: &En1993Snapshot| entities_of(document).into_iter().map(|e| e.id).collect::<BTreeSet<_>>();
    let (frame_owned, parts_owned) = (owned(&frame), owned(&parts));
    for (document, owned, rows) in [
        (&frame, &frame_owned, vec![(frame.frame_actions.first().map(|r| r.id.clone()), "frameActions"), (frame.frame_member_bases.first().map(|r| r.id.clone()), "frameMemberBases")]),
        (&parts, &parts_owned, vec![(parts.joint_plates.first().map(|r| r.id.clone()), "jointPlates"), (parts.foundation_blocks.first().map(|r| r.id.clone()), "foundationBlocks")]),
    ] {
        for (id, collection) in rows {
            let id = id.unwrap_or_else(|| panic!("fixture has a {collection} row"));
            assert_eq!(entity_kind_of(document, &id).map(|k| k.collection), Some(collection));
            assert!(owned.contains(&id), "{collection} row {id} is a scene owner");
        }
    }
    assert_eq!(entity_kind_of(&frame, "nothing"), None);
    let placement = frame.placements[0].id.clone();
    assert_eq!(entity_kind_of(&frame, &placement).map(|k| k.collection), Some("members"), "the placement row follows the entity it moves");
    let (scene, _) = scene_of(&parts, &[]);
    for id in ["plate-fin", "block-bp"] {
        let entity = scene.entities.iter().find(|e| e.entity.id == id).expect("drawn owner");
        assert!(entity.utilisation.is_some() || !entity.handles.is_empty(), "{id} is pickable with grips or a bound check");
    }
}

#[test]
fn every_numeric_leaf_of_the_connection_panel_and_cold_formed_rows_has_a_grip() {
    let schema: serde_json::Value = serde_json::from_str(include_str!("../../../🧬️schema/📸️snapshot/🔣️.json")).expect("the snapshot schema parses");
    let documents = [semio_s_artifact_norm_en1993_showcase::full_showcase(), En1993Snapshot::special_parts_showcase(), compliant()];
    let mut seen = BTreeSet::new();
    for document in &documents {
        let rows: [(&str, &str, scene::EntityKind, Vec<&str>); 5] = [
            ("joints", "SteelJoint", scene::EntityKind::Group, document.joints.iter().map(|r| r.id.as_str()).collect()),
            ("jointPlates", "JointPlate", scene::EntityKind::Group, document.joint_plates.iter().map(|r| r.id.as_str()).collect()),
            ("hollowJoints", "HollowJoint", scene::EntityKind::Group, document.hollow_joints.iter().map(|r| r.id.as_str()).collect()),
            ("platedPanels", "PlatedPanel", scene::EntityKind::Plate, document.plated_panels.iter().map(|r| r.id.as_str()).collect()),
            ("coldFormedMembers", "ColdFormedMember", scene::EntityKind::Member, document.cold_formed_members.iter().map(|r| r.id.as_str()).collect()),
        ];
        for (collection, definition, kind, ids) in rows {
            let leaves: Vec<&str> = schema["$defs"][definition]["properties"].as_object().expect("definition").iter().filter(|(_, p)| matches!(p["type"].as_str(), Some("number" | "integer"))).map(|(name, _)| name.as_str()).collect();
            for id in ids {
                let all = handles(document, &[], &[scene::EntityRef::new(kind, id, format!("{collection}[id={id}]"))]);
                assert_handle_laws(&all);
                let written: BTreeSet<&str> = all.iter().flat_map(|h| h.writes.iter().map(|w| w.path.as_str())).collect();
                for leaf in &leaves {
                    assert!(written.contains(format!("{collection}[id={id}].{leaf}").as_str()), "{collection}[{id}].{leaf} has no grip");
                }
                seen.insert(collection);
            }
        }
    }
    assert_eq!(seen.len(), 5, "the fixtures cover every grouped collection: {seen:?}");
}

#[test]
fn sections_and_member_actions_are_registered_kinds_with_scene_owners_grips_and_dependencies() {
    for (id, collection) in [("section", "sections"), ("memberAction", "memberActions")] {
        let hit = ENTITY_KINDS.iter().find(|k| k.id == id).unwrap_or_else(|| panic!("{id} is a registered kind"));
        assert_eq!(hit.collection, collection);
        assert!(!hit.label_en.is_empty() && !hit.label_de.is_empty());
    }
    let document = compliant();
    let owned: BTreeSet<_> = entities_of(&document).into_iter().map(|e| e.id).collect();
    let (section, action) = (document.sections[0].id.clone(), document.member_actions[0].clone());
    assert!(owned.contains(&section) && owned.contains(&action.id));
    assert_eq!(entity_kind_of(&document, &section).map(|k| k.collection), Some("sections"));
    assert_eq!(entity_kind_of(&document, &format!("{}:n", action.id)).map(|k| k.collection), Some("memberActions"));
    let (scene, _) = scene_of(&document, &[]);
    let drawn = scene.entities.iter().find(|e| e.entity.id == section).expect("the section row is drawn");
    assert!(!drawn.handles.is_empty(), "the section row carries dimension grips");
    let section_grips = handles(&document, &[], &[drawn.entity.clone()]);
    assert_handle_laws(&section_grips);
    assert!(section_grips.iter().any(|h| h.writes.iter().any(|w| w.path == format!("sections[id={section}].h"))));
    assert!(scene.entities.iter().any(|e| e.entity.id.starts_with(&format!("{}:", action.id)) && e.entity.path.starts_with("memberActions")), "the action glyphs belong to the action row");
    let deps = entity_dependencies(&document, &action.id);
    for expected in [&action.member_id, &action.load_case_id] {
        assert!(deps.contains(expected), "{} depends on {expected}: {deps:?}", action.id);
    }
}

/// 🧭️ Every check of every fixture is drawn: it resolves to a row the scene projects, or to the global badge through a document-level family, and the focus of the outcome names the same row.
#[test]
fn every_check_outcome_routes_to_its_entity_or_the_document_badge() {
    use kit::{focus_of_outcome, route_of, CheckRoute};
    for document in [compliant(), overloaded(), En1993Snapshot::special_parts_showcase(), En1993Snapshot::frame_portal_showcase()] {
        let (scene, report) = scene_of(&document, &[]);
        assert!(!report.checks.is_empty());
        for check in &report.checks {
            match route_of(&document, check).unwrap_or_else(|| panic!("check {} (subject {:?}) has no owner", check.id, check.subject.entity_id)) {
                CheckRoute::Entity(id) => {
                    assert!(scene.entities.iter().any(|e| scene::projection_kit::base_id(&e.entity.id) == id), "check {} routes to {id}, which the scene does not draw", check.id);
                    assert_eq!(focus_of_outcome(&document, &report, &check.id).as_deref(), Some(id.as_str()));
                }
                CheckRoute::Document => assert!(focus_of_outcome(&document, &report, &check.id).is_none(), "{}", check.id),
            }
        }
    }
}

/// 📈️ Every family of the owner table is either drawn by a dedicated diagram or an outcome family, and every drawn family is a family of the table; the owner of a check is its longest matching family.
#[test]
fn every_check_family_is_drawn_by_a_dedicated_diagram_or_the_outcome_diagram() {
    use kit::{check_owner, is_outcome_check, CHECK_OWNERS, DRAWN_FAMILIES};
    for prefix in DRAWN_FAMILIES {
        assert!(CHECK_OWNERS.iter().any(|(p, _)| p == prefix), "{prefix} is not a check family of the owner table");
    }
    for (prefix, collection) in CHECK_OWNERS {
        let id = format!("{prefix}x");
        assert_eq!(is_outcome_check(&id), check_owner(&id).is_some_and(|_| CHECK_OWNERS.iter().find(|(p, _)| id.starts_with(p)).is_some_and(|(p, _)| !DRAWN_FAMILIES.contains(p))));
        assert!(check_owner(&id).is_some(), "{prefix} ({collection})");
    }
    assert_eq!(check_owner("en1993.6.3.1.nb.m1"), Some("members"));
    assert_eq!(check_owner("en1993.unknown.x"), None);
}

/// 📊️ A row outside the ladder collections with checks no dedicated diagram draws gets one `check-outcomes:` diagram with a bar per such check, the limit line and a table twin; a ladder row gets none.
#[test]
fn the_outcome_diagram_draws_a_bar_per_check_of_an_undiagrammed_family() {
    use kit::{is_outcome_check, owner_entity, LADDER_COLLECTIONS};
    let document = En1993Snapshot::special_parts_showcase();
    let (_, report) = scene_of(&document, &[]);
    let list = diagrams(&document, &report, None);
    let outcomes: Vec<&scene::Diagram2d> = list.iter().filter(|d| d.id.starts_with("check-outcomes:")).collect();
    assert!(!outcomes.is_empty(), "no outcome diagram");
    for d in outcomes {
        d.validate().expect("valid");
        let id = d.id.trim_start_matches("check-outcomes:");
        assert!(!LADDER_COLLECTIONS.contains(&kit::entity_kind_of(&document, id).expect(id).collection), "{id} has a ladder");
        let expected = report.checks.iter().filter(|c| c.utilization.is_finite() && c.status != norm::CheckStatus::NotApplicable && is_outcome_check(&c.id) && owner_entity(&document, c).as_deref() == Some(id)).count();
        assert_eq!(d.table_twin.columns[1].values.len(), expected, "{}", d.id);
        assert!(d.series.iter().any(|s| matches!(s, scene::Series::Limit { value, .. } if (*value - 1.0).abs() < 1e-12)));
    }
    assert!(diagrams(&document, &report, Some("no-such-row")).iter().all(|d| !d.id.starts_with("check-outcomes:")));
}

#[test]
fn the_basis_badge_follows_the_document_annex_in_the_scene_and_every_diagram() {
    scene::projection_kit::test_support::assert_basis_follows_annex(norm::NormFamilyId::En1993, |annex| {
        let mut document = compliant();
        document.annex = annex;
        let (scene, report) = scene_of(&document, &[]);
        (scene, diagrams(&document, &report, None))
    });
}
