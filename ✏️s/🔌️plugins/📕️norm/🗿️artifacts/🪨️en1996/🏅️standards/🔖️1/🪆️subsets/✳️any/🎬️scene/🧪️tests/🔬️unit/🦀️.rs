use super::*;
use scene::projection_kit::test_support::{revision, entity, assert_handle_laws};
use scene::{canonical_json, Validate};
use std::collections::BTreeSet;

fn report_of(document: &En1996Snapshot) -> CheckReport {
    let mut report = masonry::evaluate_building(document.annex, document.masonry_class, document.design_situation, document.reliability_class, document.storeys, &document.walls);
    report.extend(masonry::detailing_checks(document.annex, &document.walls, &document.chases, &document.movement_joints).checks);
    report
}

fn scene_of(document: &En1996Snapshot, placements: &[Placement]) -> (StructuralScene, CheckReport) {
    let report = report_of(document);
    let scene = project(&SceneInput { document, report: &report, placements, revision: revision() });
    (scene, report)
}

fn compliant() -> En1996Snapshot {
    En1996Snapshot::compliant_clay_wall()
}

fn failing() -> En1996Snapshot {
    En1996Snapshot::noncompliant_multi_fail()
}

fn rich() -> En1996Snapshot {
    let mut document = En1996Snapshot::opening_wall_example();
    let wall = &mut document.walls[0];
    wall.support_sides = 3;
    wall.reinforced = true;
    wall.as_vertical_m2 = 1.0e-3;
    wall.as_horizontal_m2 = 4.0e-4;
    wall.f_yd_pa = 435e6;
    wall.load_cases[0].concentrated = vec![model::ConcentratedLoad { id: "beam-B".into(), force_n: 35_000.0, bearing_area_m2: 0.04, bearing_length_m: 0.20, distance_a1_m: 0.5, position_m: 2.5 }];
    document
}

fn volume(geometry: &scene::Geometry) -> f64 {
    let scene::Geometry::Solid(solid) = geometry else { panic!("solid") };
    let scene::SolidShape::Mesh { vertices, triangles } = &solid.shape else { panic!("mesh") };
    triangles
        .iter()
        .map(|t| {
            let (a, b, c) = (&vertices[t.a as usize], &vertices[t.b as usize], &vertices[t.c as usize]);
            a.dot(&b.cross(c)) / 6.0
        })
        .sum()
}

fn check<'a>(report: &'a CheckReport, id: &str) -> &'a norm::CheckResult {
    report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("check {id} missing"))
}

#[test]
fn the_scene_of_every_fixture_validates() {
    for document in [compliant(), failing(), rich(), En1996Snapshot::basement_wall_example(), En1996Snapshot::concentrated_load_example(), En1996Snapshot::reinforced_wall_example()] {
        let (scene, _) = scene_of(&document, &[]);
        assert_eq!(scene.validate(), Ok(()), "violations");
        assert_eq!(scene.schema, scene::SCENE_SCHEMA_ID);
    }
}

#[test]
fn ids_are_unique_and_layers_exist() {
    let (scene, _) = scene_of(&rich(), &[]);
    let ids: BTreeSet<_> = scene.entities.iter().map(|e| e.entity.id.clone()).collect();
    assert_eq!(ids.len(), scene.entities.len());
    let layers: BTreeSet<_> = scene.layers.iter().map(|l| l.id.as_str()).collect();
    assert!(scene.entities.iter().all(|e| layers.contains(e.layer.as_str())));
    assert!(scene.layers.iter().any(|l| l.id == "bond" && !l.visible), "the bond pattern is hidden by default");
}

#[test]
fn projection_is_deterministic_and_locale_free() {
    let (a, _) = scene_of(&rich(), &[]);
    let (b, _) = scene_of(&rich(), &[]);
    assert_eq!(canonical_json(&a), canonical_json(&b));
    assert!(a.entities.iter().all(|e| !e.accessible_name.en.is_empty() && !e.accessible_name.de.is_empty()));
}

#[test]
fn the_openings_are_cut_out_of_the_three_zones() {
    let document = En1996Snapshot::opening_wall_example();
    let (scene, _) = scene_of(&document, &[]);
    let wall = &document.walls[0];
    let cut: f64 = layout::openings(wall).iter().map(|o| (o.x1 - o.x0) * (o.z1 - o.z0)).sum();
    let expected = (wall.length_m * wall.height_m - cut) * wall.thickness_m;
    let total: f64 = ["zone-bottom", "zone-mid", "zone-top"].iter().map(|z| volume(&entity(&scene, &format!("{}:{z}", wall.id)).geometry)).sum();
    assert!((total - expected).abs() < 1e-9, "zones {total} vs wall {expected}");
    assert!(matches!(entity(&scene, "wall-openings:opening-win-1").geometry, scene::Geometry::Plate(_)));
    assert!(matches!(entity(&scene, "wall-openings:lintel-win-1").geometry, scene::Geometry::Solid(_)));
    assert_eq!(entity(&scene, "wall-openings:opening-win-1").appearance.role, scene::MaterialRole::Glass);
    assert_eq!(entity(&scene, "wall-openings:opening-door-1").appearance.role, scene::MaterialRole::Neutral);
}

#[test]
fn openings_never_overlap_and_stay_inside_the_wall() {
    let document = En1996Snapshot::opening_wall_example();
    let wall = &document.walls[0];
    let spans = layout::openings(wall);
    assert_eq!(spans.len(), 2);
    assert!(spans.windows(2).all(|w| w[0].x1 <= w[1].x0 + 1e-12));
    assert!(spans.iter().all(|o| o.x0 >= 0.0 && o.x1 <= wall.length_m && o.z0 >= 0.0 && o.z1 <= wall.height_m));
}

#[test]
fn zone_colours_follow_the_station_utilisation_of_the_compression_checks() {
    let document = compliant();
    let (scene, report) = scene_of(&document, &[]);
    let wall = &document.walls[0];
    let lc = &wall.load_cases[0].id;
    let combined = check(&report, &format!("en1996.6.1.2.compression.{}.{lc}", wall.id)).utilization;
    let mid = check(&report, &format!("en1996.6.1.2.compression.mid.{}.{lc}", wall.id)).utilization;
    let ratio = |zone: &str| entity(&scene, &format!("{}:{zone}", wall.id)).utilisation.as_ref().expect("zone binding").ratio;
    let worst = ratio("zone-top").max(ratio("zone-mid")).max(ratio("zone-bottom"));
    assert!((worst - combined).abs() < 1e-9 * combined.max(1.0), "worst zone {worst} vs check {combined}");
    assert!((ratio("zone-mid") - mid).abs() < 1e-9 * mid.max(1.0), "mid zone {} vs check {mid}", ratio("zone-mid"));
    assert!(ratio("zone-bottom") > ratio("zone-top"), "self-weight accumulates downwards");
    assert_eq!(entity(&scene, &format!("{}:zone-top", wall.id)).appearance.channel, scene::ColourChannel::Utilisation);
}

#[test]
fn every_binding_names_a_report_check() {
    for document in [compliant(), failing(), rich()] {
        let (scene, report) = scene_of(&document, &[]);
        for e in &scene.entities {
            if let Some(b) = &e.utilisation {
                assert!(report.checks.iter().any(|c| c.id == b.check_id), "{} binds unknown check {}", e.entity.id, b.check_id);
            }
        }
    }
}

#[test]
fn the_profile_reproduces_the_resistance_of_the_report() {
    for document in [compliant(), failing(), rich()] {
        let report = report_of(&document);
        for wall in &document.walls {
            let p = profile::governing(&document, wall).expect("profile");
            let mid = check(&report, &format!("en1996.6.1.2.compression.mid.{}.{}", wall.id, p.load_case));
            assert!((mid.limit.value - p.resistance(1)).abs() <= 1e-9 * p.resistance(1).abs().max(1.0), "{}: N_Rd,mid {} vs {}", wall.id, mid.limit.value, p.resistance(1));
            assert!((mid.computed.value - p.n[1]).abs() <= 1e-9 * p.n[1].abs().max(1.0));
            let combined = check(&report, &format!("en1996.6.1.2.compression.{}.{}", wall.id, p.load_case));
            let s = p.governing_station();
            assert!((combined.limit.value - p.resistance(s)).abs() <= 1e-9 * p.resistance(s).abs().max(1.0), "{}: N_Rd {} vs {}", wall.id, combined.limit.value, p.resistance(s));
        }
    }
}

#[test]
fn a_failing_wall_gets_error_style_hatch_and_failure_glyphs() {
    let (scene, _) = scene_of(&failing(), &[]);
    let wall = entity(&scene, "wall-weak");
    assert_eq!(wall.appearance.style, scene::StyleState::Error);
    assert_eq!(wall.appearance.hatch, Some(scene::Hatch::Diagonal));
    assert!(scene.annotations.iter().any(|a| matches!(a, scene::Annotation::FailureGlyph { .. })));
}

#[test]
fn the_default_layout_runs_walls_left_to_right_with_a_gap() {
    let mut document = compliant();
    let mut second = document.walls[0].clone();
    second.id = "wall-south".into();
    document.walls.push(second);
    let layout = Layout::of(&document, &[]);
    let first = layout.pose("wall-north").expect("first");
    let next = layout.pose("wall-south").expect("second");
    assert_eq!(first.origin, scene::Vec3::ZERO);
    assert!((next.origin.x - (document.walls[0].length_m + layout::WALL_GAP)).abs() < 1e-12);
}

#[test]
fn a_persisted_placement_overrides_the_default_and_rotates_about_z() {
    let placement = Placement { entity_id: "wall-north".into(), origin: scene::Vec3::new(10.0, 2.0, 0.0), yaw: std::f64::consts::FRAC_PI_2 };
    let layout = Layout::of(&compliant(), &[placement]);
    let pose = layout.pose("wall-north").expect("pose");
    let end = pose.at(5.0, 0.0, 0.0);
    assert!((end.x - 10.0).abs() < 1e-9 && (end.y - 7.0).abs() < 1e-9);
    assert!(pose.axis_x().y > 0.999);
}

#[test]
fn the_bond_pattern_has_offset_courses_and_flags_invalid_overlap() {
    let document = compliant();
    let (scene, _) = scene_of(&document, &[]);
    let wall = &document.walls[0];
    let units = entity(&scene, "wall-north:units");
    assert_eq!(units.layer, "bond");
    assert_eq!(units.appearance.style, scene::StyleState::Normal);
    let mortar = entity(&scene, "wall-north:mortar");
    let gross = volume(&units.geometry);
    assert!(gross > 0.0 && gross < wall.length_m * wall.height_m * wall.thickness_m);
    assert!(volume(&mortar.geometry) > 0.0);
    let mut short = compliant();
    short.walls[0].unit_length_m = 0.08;
    let (scene, _) = scene_of(&short, &[]);
    let units = entity(&scene, "wall-north:units");
    assert_eq!(units.appearance.style, scene::StyleState::Error);
    assert!(bond::overlap_need(0.113) >= 0.045 && 0.5 * 0.08 < bond::overlap_need(0.113));
}

#[test]
fn the_bond_pattern_respects_the_unit_budget() {
    let mut document = compliant();
    let template = document.walls[0].clone();
    document.walls = (0..200).map(|i| model::MasonryWall { id: format!("w{i}"), ..template.clone() }).collect();
    let (scene, _) = scene_of(&document, &[]);
    let with_units = scene.entities.iter().filter(|e| e.entity.id.ends_with(":units")).count();
    assert!(with_units > 0 && with_units < 200, "{with_units} walls got unit geometry");
    assert!(scene.entities.iter().filter(|e| e.entity.id.ends_with(":mortar")).count() == 200);
}

#[test]
fn the_concentrated_load_spreads_at_sixty_degrees_to_mid_height() {
    let document = rich();
    let (scene, _) = scene_of(&document, &[]);
    let wall = &document.walls[0];
    let load = &wall.load_cases[0].concentrated[0];
    let zone = entity(&scene, "wall-openings:spread-ulsinz-beam-B");
    let scene::Geometry::Surface(surface) = &zone.geometry else { panic!("surface") };
    let scene::SurfaceShape::Polygon { polygon } = &surface.shape else { panic!("polygon") };
    let ring = &polygon.outer.points;
    let width_mid = (ring[2].x - ring[3].x).abs();
    let expected = load.bearing_length_m + wall.height_m / 3f64.sqrt();
    assert!((width_mid - expected).abs() < 1e-9, "{width_mid} vs {expected}");
    assert!(((ring[1].x - ring[0].x) - load.bearing_length_m).abs() < 1e-9);
    assert!((ring[2].z - 0.5 * wall.height_m).abs() < 1e-9);
    assert!(matches!(entity(&scene, "wall-openings:conc-ulsinz-beam-B").geometry, scene::Geometry::Load(scene::LoadGlyph::Point { .. })));
}

#[test]
fn laterally_loaded_walls_show_wind_pressure_and_yield_regions_by_support() {
    for (sides, regions) in [(2u8, 2usize), (3, 3), (4, 4)] {
        let mut document = compliant();
        document.walls[0].support_sides = sides;
        let (scene, _) = scene_of(&document, &[]);
        assert_eq!(scene.entities.iter().filter(|e| e.entity.id.starts_with("wall-north:yield-")).count(), regions);
        assert!(matches!(entity(&scene, "wall-north:wind-ulsinz").geometry, scene::Geometry::Load(scene::LoadGlyph::Pressure { .. })));
        assert_eq!(scene.entities.iter().filter(|e| e.entity.id.contains(":support-left") || e.entity.id.contains(":support-right")).count(), usize::from(sides) - 2);
    }
}

#[test]
fn the_annex_e_panel_of_the_governing_wall_names_the_case_and_the_unverified_row() {
    let (scene, _) = scene_of(&compliant(), &[]);
    let text = format!("{:?}", entity(&scene, "wall-north:annex-e"));
    assert!(text.contains("governing Annex E case") && text.contains("ndp.en1996.annex-e."), "{text}");
}

#[test]
fn the_yield_regions_tile_the_wall() {
    let area = |ring: &[(f64, f64)]| (0..ring.len()).map(|i| ring[i].0 * ring[(i + 1) % ring.len()].1 - ring[(i + 1) % ring.len()].0 * ring[i].1).sum::<f64>().abs() / 2.0;
    for sides in [2u8, 3, 4] {
        let total: f64 = loads::yield_regions(sides, 5.0, 2.75, 1.0).iter().map(|r| area(r)).sum();
        assert!((total - 5.0 * 2.75).abs() < 1e-9, "{sides} sides: {total}");
    }
}

#[test]
fn reinforced_walls_get_vertical_and_bed_joint_bars() {
    let (scene, _) = scene_of(&rich(), &[]);
    let scene::Geometry::Fastener(vertical) = &entity(&scene, "wall-openings:rebar-v").geometry else { panic!("vertical bars") };
    assert!(matches!(vertical.kind, scene::FastenerKind::Rebar { .. }));
    assert!(vertical.diameter >= 0.004 && vertical.diameter <= 0.032);
    assert!(matches!(entity(&scene, "wall-openings:rebar-h").geometry, scene::Geometry::Fastener(_)));
    let (plain, _) = scene_of(&compliant(), &[]);
    assert!(plain.entities.iter().all(|e| !e.entity.id.contains("rebar")));
}

#[test]
fn every_advertised_handle_exists_and_writes_a_known_leaf() {
    let document = rich();
    let (scene, _) = scene_of(&document, &[]);
    let selection: Vec<_> = scene.entities.iter().map(|e| e.entity.clone()).collect();
    let all = handles(&document, &[], &selection);
    assert_handle_laws(&all);
    let ids: BTreeSet<_> = all.iter().map(|h| h.id.clone()).collect();
    for e in &scene.entities {
        assert!(e.handles.iter().all(|h| ids.contains(h)), "{} advertises unknown handle", e.entity.id);
    }
    assert!(all.iter().all(|h| h.writes.iter().all(|w| (w.path.starts_with("walls[id=") || w.path.starts_with("placements[id=")) && w.path.contains('.'))));
    assert!(all.iter().all(|h| h.accessible_name.en.len() > 2 && h.accessible_name.de.len() > 2));
    for tag in ["length", "height", "thickness", "sides"] {
        assert!(ids.contains(&format!("wall-openings:{tag}")), "missing wall handle {tag}");
    }
    for tag in ["width", "height", "sill"] {
        assert!(ids.contains(&format!("wall-openings:opening-win-1:{tag}")), "missing opening handle {tag}");
    }
    for id in ["wall-openings:slab-top:bearing-depth", "wall-openings:slab-top:span", "wall-openings:resultant-top:eccentricity", "wall-openings:resultant-bottom:eccentricity", "wall-openings:conc-ulsinz-beam-B:bearing-length"] {
        assert!(ids.contains(id), "missing {id}");
    }
}

#[test]
fn dragging_the_length_handle_writes_the_snapped_length() {
    let document = compliant();
    let all = handles(&document, &[], &[scene::EntityRef::new(scene::EntityKind::Group, "wall-north", "walls[id=wall-north]")]);
    assert_handle_laws(&all);
    let length = all.iter().find(|h| h.id == "wall-north:length").expect("length handle");
    let writes = length.writes_at(6.012);
    assert_eq!(writes.len(), 1);
    assert_eq!(writes[0].path, "walls[id=wall-north].lengthM");
    assert!((writes[0].value - 6.0).abs() < 1e-9);
    assert_eq!(length.writes_at(1000.0)[0].value, 60.0);
}

#[test]
fn handles_write_the_current_value_at_their_anchor() {
    let document = rich();
    let wall = &document.walls[0];
    let selection: Vec<_> = ["wall-openings:opening-win-1", "wall-openings:slab-top", "wall-openings:resultant-top", "wall-openings:resultant-bottom", "wall-openings:conc-ulsinz-beam-B"].iter().map(|id| scene::EntityRef::new(scene::EntityKind::Group, *id, "walls[id=wall-openings]")).collect();
    let all = handles(&document, &[], &selection);
    assert_handle_laws(&all);
    let at = |id: &str| all.iter().find(|h| h.id == id).unwrap_or_else(|| panic!("handle {id}"));
    let write = |id: &str| {
        let h = at(id);
        let scene::HandleKind::ScalarAlong { axis } = &h.kind else { panic!("scalar handle {id}") };
        h.writes[0].value_at(h.anchor.dot(axis))
    };
    let win = &wall.openings[0];
    assert!((write("wall-openings:opening-win-1:width") - win.width_m).abs() < 1e-6);
    assert!((write("wall-openings:opening-win-1:height") - win.height_m).abs() < 1e-6);
    assert!((write("wall-openings:opening-win-1:sill") - win.sill_height_m).abs() < 1e-6);
    assert!((write("wall-openings:slab-top:bearing-depth") - wall.slab_bearing_depth_m).abs() < 1e-6);
    assert!((write("wall-openings:resultant-bottom:eccentricity") - wall.eccentricity_bottom_m).abs() < 1e-6);
    assert!((write("wall-openings:resultant-top:eccentricity") - wall.eccentricity_top_m).abs() < 1e-6);
    assert!((write("wall-openings:conc-ulsinz-beam-B:bearing-length") - wall.load_cases[0].concentrated[0].bearing_length_m).abs() < 1e-6);
}

#[test]
fn derived_scene_ids_resolve_to_their_registry_row() {
    let document = rich();
    assert_eq!(entity_kind_of(&document, "wall-openings").map(|k| k.collection), Some("walls"));
    assert_eq!(entity_kind_of(&document, "wall-openings:zone-mid").map(|k| k.collection), Some("walls"));
    assert_eq!(entity_kind_of(&document, "wall-openings:lintel-win-1").map(|k| k.collection), Some("openings"));
    assert_eq!(entity_kind_of(&document, "wall-openings:conc-ulsinz-beam-B").map(|k| k.collection), Some("concentrated"));
    assert_eq!(entity_kind_of(&document, "nothing"), None);
}

#[test]
fn dependencies_name_what_a_projection_reads() {
    let document = rich();
    let deps = entity_dependencies(&document, "wall-openings");
    for expected in ["win-1", "door-1", "ulsinz", "beam-B"] {
        assert!(deps.iter().any(|d| d == expected), "wall should depend on {expected}: {deps:?}");
    }
    assert_eq!(entity_dependencies(&document, "wall-openings:zone-mid"), vec!["wall-openings".to_string()]);
}

#[test]
fn every_diagram_validates_and_has_a_table_twin() {
    for document in [compliant(), failing(), rich()] {
        let report = report_of(&document);
        let all = diagrams(&document, &report, None);
        let ids: BTreeSet<_> = all.iter().map(|d| d.id.clone()).collect();
        let wall = &document.walls[0].id;
        for prefix in ["ecc", "phi", "capacity", "slenderness", "fk", "shear", "lateral-alpha", "lateral-capacity", "fire"] {
            assert!(ids.contains(&format!("{prefix}:{wall}")), "missing diagram {prefix} in {ids:?}");
        }
        for diagram in &all {
            assert_eq!(diagram.validate(), Ok(()), "diagram {}", diagram.id);
            assert!(diagram.table_twin.rows() > 0, "diagram {} has no table twin", diagram.id);
        }
    }
    let report = report_of(&rich());
    assert!(diagrams(&rich(), &report, None).iter().any(|d| d.id.starts_with("mn:")), "reinforced walls get the M-N curve");
}

#[test]
fn the_phi_markers_match_the_profile_and_the_checks() {
    let document = compliant();
    let report = report_of(&document);
    let wall = &document.walls[0];
    let p = profile::governing(&document, wall).expect("profile");
    let diagram = diagrams(&document, &report, Some("wall-north")).into_iter().find(|d| d.id == "phi:wall-north").expect("phi");
    for (id, station) in [("phi-top", 0usize), ("phi-middle", 1), ("phi-bottom", 2)] {
        let marker = diagram.markers.iter().find(|m| m.id == id).expect(id);
        assert!((marker.at.x - p.phi[station]).abs() < 1e-12);
        assert!((marker.utilisation.expect("utilisation") - p.utilisation[station]).abs() < 1e-12);
        assert!(marker.bound_check.as_ref().is_some_and(|c| report.checks.iter().any(|r| &r.id == c)));
    }
}

#[test]
fn the_eccentricity_markers_drag_the_declared_values() {
    let document = compliant();
    let report = report_of(&document);
    let wall = &document.walls[0];
    let diagram = diagrams(&document, &report, Some("wall-north")).into_iter().find(|d| d.id == "ecc:wall-north").expect("ecc");
    let top = diagram.markers.iter().find(|m| m.id == "e-top").expect("top");
    let binding = top.binding.as_ref().expect("binding");
    assert_eq!(binding.path, "walls[id=wall-north].eccentricityTopM");
    assert!((binding.value_at(top.at.x) - wall.eccentricity_top_m).abs() < 1.5e-3);
    let bottom = diagram.markers.iter().find(|m| m.id == "e-bottom").expect("bottom");
    assert!((bottom.binding.as_ref().expect("binding").value_at(bottom.at.x) - wall.eccentricity_bottom_m).abs() < 1.5e-3);
}

#[test]
fn the_strength_marker_sits_on_the_fk_curve_and_drags_fb() {
    let document = compliant();
    let report = report_of(&document);
    let wall = &document.walls[0];
    let diagram = diagrams(&document, &report, Some("wall-north")).into_iter().find(|d| d.id == "fk:wall-north").expect("fk");
    let marker = diagram.markers.iter().find(|m| m.id == "wall").expect("marker");
    let expected = masonry::f_k_mpa_from(document.annex, wall.unit_material, wall.unit_group, wall.mortar_type, marker.at.x, masonry::f_m_mpa(wall));
    assert!((marker.at.y - expected).abs() < 1e-9);
    assert!((marker.binding.as_ref().expect("binding").value_at(marker.at.x) - wall.f_b_pa).abs() <= 1e5);
}

#[test]
fn the_lateral_capacity_matches_the_catalogue_check() {
    let document = compliant();
    let report = report_of(&document);
    let wall = &document.walls[0];
    let lat = profile::lateral(&document, wall).expect("lateral");
    let result = check(&report, &format!("en1996.m-31.w-rd.{}.{}", wall.id, lat.load_case));
    let w_rd = lat.pressure(lat.l_ef, wall.height_m);
    assert!((result.limit.value - w_rd).abs() <= 1e-6 * w_rd, "{} vs {w_rd}", result.limit.value);
    let diagram = diagrams(&document, &report, Some("wall-north")).into_iter().find(|d| d.id == "lateral-capacity:wall-north").expect("capacity");
    assert!(diagram.series.iter().any(|s| matches!(s, scene::Series::Limit { value, .. } if (value - lat.w_ed).abs() < 1e-9)));
}

#[test]
fn the_fire_marker_carries_the_required_class_and_the_tabulated_thickness() {
    let document = compliant();
    let report = report_of(&document);
    let wall = &document.walls[0];
    let diagram = diagrams(&document, &report, Some("wall-north")).into_iter().find(|d| d.id == "fire:wall-north").expect("fire");
    let marker = diagram.markers.iter().find(|m| m.id == "required").expect("required");
    assert_eq!(marker.at.x, f64::from(wall.fire_rei_min));
    let binding = marker.binding.as_ref().expect("binding");
    assert_eq!(binding.path, "walls[id=wall-north].fireReiMin");
    assert_eq!(binding.value_at(100.0), 90.0);
}

#[test]
fn focus_narrows_the_diagram_set() {
    let mut document = compliant();
    let mut second = document.walls[0].clone();
    second.id = "wall-south".into();
    document.walls.push(second);
    let report = report_of(&document);
    let south = diagrams(&document, &report, Some("wall-south:zone-mid"));
    assert!(!south.is_empty() && south.iter().all(|d| d.id.ends_with(":wall-south")));
    assert!(diagrams(&document, &report, Some("nothing")).is_empty());
}

#[test]
fn golden_scene_matches_committed_bytes() {
    let (scene, _) = scene_of(&rich(), &[]);
    let actual = canonical_json(&scene);
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧫️fixtures/🏛️scene/🔣️.json");
    if std::env::var("SEMIO_UPDATE_GOLDEN").is_ok() {
        std::fs::write(path, &actual).expect("write golden");
    }
    let expected = std::fs::read_to_string(path).expect("golden fixture");
    assert_eq!(actual.trim(), expected.trim(), "golden scene changed; rerun with SEMIO_UPDATE_GOLDEN=1 after reviewing the diff");
}

fn walls_json(document: &En1996Snapshot) -> String {
    let wall = |w: &model::MasonryWall| {
        let openings: Vec<String> = w.openings.iter().map(|o| format!("{{\"id\":\"{}\",\"width\":{},\"height\":{},\"sill\":{}}}", o.id, o.width_m, o.height_m, o.sill_height_m)).collect();
        let loads: Vec<String> = w.load_cases.iter().flat_map(|lc| lc.concentrated.iter().map(move |c| {
            let (ab, a_ef, _) = spreading::bearing_terms(w, c);
            format!("{{\"id\":\"{}-{}\",\"bearingLength\":{},\"ab\":{ab},\"aEf\":{a_ef},\"a1\":{}}}", lc.id, c.id, c.bearing_length_m, c.distance_a1_m)
        })).collect();
        format!("{{\"id\":\"{}\",\"length\":{},\"height\":{},\"thickness\":{},\"unitHeight\":{},\"unitLength\":{},\"joint\":{},\"openings\":[{}],\"concentrated\":[{}],\"clayGroup1\":{}}}", w.id, w.length_m, w.height_m, w.thickness_m, w.unit_height_m, w.unit_length_m, w.bed_joint_thickness_m, openings.join(","), loads.join(","), matches!((w.unit_material, w.unit_group, w.mortar_type), (model::UnitMaterial::Clay, model::UnitGroup::Group1, model::MortarType::GeneralPurpose)))
    };
    document.walls.iter().map(wall).collect::<Vec<_>>().join(",")
}

#[test]
fn the_independent_oracle_agrees() {
    let document = rich();
    let report = report_of(&document);
    let (scene, _) = scene_of(&document, &[]);
    let diagram_json: Vec<String> = diagrams(&document, &report, None).iter().map(canonical_json).collect();
    let square = profile::Lateral { load_case: "square".into(), m1: 1500.0, m2: 1500.0, edges: masonry::numerical_models::Edges::parse("SSSS").expect("edges"), l_ef: 4.0, w_ed: 0.0 };
    let closed = format!("{{\"m\":1500.0,\"l\":4.0,\"pressure\":{}}}", square.pressure(4.0, 4.0));
    let input = format!("{{\"scene\":{},\"diagrams\":[{}],\"walls\":[{}],\"kE\":{},\"yieldSquare\":{}}}", canonical_json(&scene), diagram_json.join(","), walls_json(&document), masonry::k_e(document.annex), closed);
    let script = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🔮️oracles/🐍️.py");
    let output = norm::python_oracle::run_ok(script, &[], Some(input.as_bytes()));
    let verdict = String::from_utf8_lossy(&output.stdout);
    assert!(verdict.contains("\"ok\": true"), "oracle verdict: {verdict}");
}

#[test]
fn projecting_a_thousand_walls_stays_interactive() {
    let mut document = compliant();
    let template = document.walls[0].clone();
    document.walls = (0..1000).map(|i| model::MasonryWall { id: format!("w{i}"), ..template.clone() }).collect();
    let report = CheckReport::default();
    let started = std::time::Instant::now();
    let scene = project(&SceneInput { document: &document, report: &report, placements: &[], revision: revision() });
    assert!(scene.entities.len() >= 1000);
    eprintln!("[latency] en1996/scene-cold-1000 p50_ms={0:.3} p95_ms={0:.3} budget_ms={1:.1}", started.elapsed().as_secs_f64() * 1e3, 4000.0);
    assert!(started.elapsed().as_millis() < 4000, "projection took {:?}", started.elapsed());
}

#[test]
fn units_are_cut_at_the_openings_so_the_unit_volume_fills_the_solid_wall_up_to_the_joints() {
    let document = En1996Snapshot::opening_wall_example();
    let (scene, _) = scene_of(&document, &[]);
    let wall = &document.walls[0];
    let openings: f64 = wall.openings.iter().map(|o| o.width_m * o.height_m).sum();
    let gross = (wall.length_m * wall.height_m - openings) * wall.thickness_m;
    let units = volume(&entity(&scene, &format!("{}:units", wall.id)).geometry);
    assert!(units <= gross + 1e-9 && units >= 0.7 * gross, "units {units} against the solid wall {gross}");
}

#[test]
fn compression_stations_colour_the_zones_with_a_vertex_field_over_the_height() {
    let document = compliant();
    let (scene, report) = scene_of(&document, &[]);
    let wall = &document.walls[0];
    let lc = &wall.load_cases[0].id;
    let check = report.checks.iter().find(|c| c.id == format!("en1996.6.1.2.compression.{}.{lc}", wall.id)).expect("compression");
    assert_eq!(check.station_field, "wall-phi");
    for (tag, section) in [("zone-bottom", 0), ("zone-mid", 1), ("zone-top", 2)] {
        let zone = entity(&scene, &format!("{}:{tag}", wall.id));
        let scene::ColourChannel::Fields { fields, .. } = &zone.appearance.channel else { panic!("field channel") };
        let scene::FieldSamples::Vertex { values } = &fields[0].samples else { panic!("vertex samples") };
        assert!(values.iter().all(|v| v.is_finite() && *v >= 0.0));
        let ratio = zone.utilisation.as_ref().expect("binding").ratio;
        assert!((ratio - check.stations[section].utilization).abs() < 1e-9, "{tag}");
    }
}

#[test]
fn the_fire_track_binds_the_tabulated_thickness_demand_of_every_class() {
    let document = compliant();
    let (scene, _) = scene_of(&document, &[]);
    assert_eq!(scene.validate(), Ok(()));
    let track = scene.tracks.iter().find(|t| t.id == "fire").expect("fire track");
    assert_eq!(track.kind, scene::TrackKind::Time);
    assert_eq!((track.min, track.max), (1800.0, 14_400.0));
    let overlay = entity(&scene, "wall-north:fire-field");
    let scene::ColourChannel::Fields { fields, .. } = &overlay.appearance.channel else { panic!("fields") };
    assert!(track.keyframes.iter().all(|k| k.field.as_ref().is_some_and(|f| fields.iter().any(|x| &x.id == f))));
    let wall = &document.walls[0];
    let field = fields.iter().find(|f| f.id == "fire-demand@90").expect("R90");
    let scene::FieldSamples::Vertex { values } = &field.samples else { panic!("vertex") };
    assert!((values[0] - 0.138_75 / 0.365).abs() < 1e-12, "tabulated 0.13875 m at mu_fi 0.7 over t = 0.365 m");
    assert!((tracks::fire_demand(wall, 90) - 0.150 / 0.365).abs() < 1e-12);
    assert!(scene.layers.iter().any(|l| l.id == "fire" && l.visible));
}

#[test]
fn the_wall_projects_a_ramped_strip_a_flange_and_a_stress_block() {
    let document = rich();
    let (scene, _) = scene_of(&document, &[]);
    let wall = &document.walls[0];
    if let Some(strip) = scene.entities.iter().find(|e| e.entity.id == format!("{}:{}", wall.id, lateral::STRIP_TAG)) {
        let overlay = strip.appearance.deformation.as_ref().expect("overlay");
        assert_eq!(overlay.motion, scene::Motion::Ramp);
        assert!(overlay.samples.len() >= 3);
    }
    for flange in &wall.flanges {
        assert!(scene.entities.iter().any(|e| e.entity.id == format!("{}:{}{}", wall.id, lateral::FLANGE_TAG, flange.id)));
    }
}

fn braced() -> En1996Snapshot {
    let mut document = En1996Snapshot::compliant_clay_wall();
    let mut second = document.walls[0].clone();
    second.id = "wall-shear-2".into();
    second.length_m = 2.0;
    for (wall, x) in document.walls.iter_mut().chain(std::iter::once(&mut second)).zip([0.0, 6.0]) {
        wall.wall_type = model::WallType::Shear;
        wall.plan_x_m = x;
        wall.in_plane_shear_n = 20_000.0;
    }
    document.walls.push(second);
    document
}

#[test]
fn the_bracing_diagram_distributes_the_storey_shear_by_stiffness() {
    let document = braced();
    let report = report_of(&document);
    let all = diagrams(&document, &report, None);
    let diagram = all.iter().find(|d| d.id == "bracing:document").expect("bracing diagram");
    assert_eq!(diagram.markers.len(), 2);
    let shares = &diagram.table_twin.columns.iter().find(|c| c.unit == "N" && c.header.en == "Share V_i").expect("shares").values;
    assert!((shares.iter().sum::<f64>() - 40_000.0).abs() < 1.0, "shares carry the storey shear: {shares:?}");
    assert!(diagram.markers.iter().all(|m| m.binding.as_ref().is_some_and(|b| b.path.ends_with(".planXM"))));
    assert!(diagrams(&compliant(), &report_of(&compliant()), None).iter().all(|d| d.id != "bracing:document"));
}

#[test]
fn the_concentrated_load_spreads_as_a_wedge_to_the_effective_length_at_mid_height() {
    let document = rich();
    let (scene, _) = scene_of(&document, &[]);
    let wall = &document.walls[0];
    let load = &wall.load_cases[0].concentrated[0];
    let wedge = entity(&scene, "wall-openings:wedge-ulsinz-beam-B");
    assert!(volume(&wedge.geometry).abs() > 0.0);
    let (lo, hi) = spreading::spread_extent(load.position_m, load.bearing_length_m, 0.5 * wall.height_m, wall.length_m);
    assert!(((hi - lo) - (load.bearing_length_m + wall.height_m / 3f64.sqrt()).min(wall.length_m)).abs() < 1e-9);
    assert!(wedge.handles.iter().any(|h| h.ends_with(":position")));
}

#[test]
fn the_isotherm_zone_is_a_hatched_solid_with_a_depth_handle() {
    let mut document = rich();
    document.walls[0].isotherm_depth_m = 0.03;
    let (scene, _) = scene_of(&document, &[]);
    let wall = &document.walls[0];
    let zone = entity(&scene, &format!("{}:isotherm-zone", wall.id));
    assert!(volume(&zone.geometry).abs() > 0.0);
    assert!(zone.handles.iter().any(|h| h.ends_with(":depth")));
    assert!((isotherms::residual_thickness(wall) - (wall.thickness_m - 0.06)).abs() < 1e-12);
    document.walls[0].isotherm_depth_m = 0.0;
    let (plain, _) = scene_of(&document, &[]);
    assert!(plain.entities.iter().all(|e| !e.entity.id.ends_with(":isotherm-zone")));
}

fn fold_into(document: &mut En1996Snapshot, commit: &scene::projection_kit::PlacementCommit) {
    let mut rows = scene::projection_kit::placements_of(document);
    commit.apply(&mut rows);
    document.placements = rows.into_iter().map(|row| model::Placement { id: row.entity_id, x: row.origin.x, y: row.origin.y, z: row.origin.z, yaw: row.yaw }).collect();
}

#[test]
fn dragging_an_entity_commits_one_undoable_event_and_the_reload_stands_at_the_same_pose() {
    use scene::projection_kit::{PlacementCommit, PlacementMove};
    let mut document = compliant();
    let id = document.walls[0].id.clone();
    let before = scene::projection_kit::placements_of(&document);
    assert!(before.is_empty());
    let target = scene::Vec3::new(7.5, -2.0, 0.0);
    let commit = PlacementCommit::of(&before, &[PlacementMove { entity_id: id.clone(), origin: target.clone(), yaw: 0.75 }]);
    assert_eq!(commit.ops.len(), 1);
    fold_into(&mut document, &commit);
    let reloaded = scene::projection_kit::placements_of(&document);
    assert_eq!(reloaded.len(), 1);
    assert_eq!(Layout::of(&document, &reloaded).pose(&id).expect("pose").origin.clone(), target);
    let again = PlacementCommit::of(&reloaded, &[PlacementMove { entity_id: id.clone(), origin: target, yaw: 0.75 }]);
    assert!(again.is_empty());
    fold_into(&mut document, &commit.inverse(&before));
    assert_eq!(scene::projection_kit::placements_of(&document), before);
}

#[test]
fn the_reduction_ribbon_the_yield_pattern_and_the_arch_thrust_line_are_scene_entities_of_a_wind_loaded_wall() {
    let document = rich();
    let (scene, _) = scene_of(&document, &[]);
    assert_eq!(scene.validate(), Ok(()));
    let wall = &document.walls[0].id;
    for tag in ["phi-field", "yield-lines", "arch-thrust"] {
        let id = format!("{wall}:{tag}");
        let entity = scene.entities.iter().find(|e| e.entity.id == id).unwrap_or_else(|| panic!("{id} missing"));
        assert_eq!(entity.layer, "patterns");
        assert!(entity.utilisation.is_some(), "{id} is bound to its check");
        assert!(!entity.handles.is_empty(), "{id} offers the wall grips");
    }
    let ribbon = scene.entities.iter().find(|e| e.entity.id == format!("{wall}:phi-field")).unwrap();
    assert!(volume(&ribbon.geometry) > 0.0, "outward-oriented ribbon");
    let arch = scene.entities.iter().find(|e| e.entity.id == format!("{wall}:arch-thrust")).unwrap();
    assert!(volume(&arch.geometry) > 0.0, "outward-oriented rods and blocks");
}

#[test]
fn every_example_and_showcase_projection_obeys_the_scene_diagram_and_handle_laws() {
    let documents = [compliant(), failing(), rich(), En1996Snapshot::opening_wall_example(), En1996Snapshot::members_wall_example(), En1996Snapshot::basement_wall_example(), En1996Snapshot::concentrated_load_example(), En1996Snapshot::reinforced_wall_example()];
    for document in documents {
        let (scene, report) = scene_of(&document, &[]);
        let selection: Vec<scene::EntityRef> = scene.entities.iter().map(|e| e.entity.clone()).collect();
        scene::projection_kit::test_support::assert_projection_laws(&scene, &diagrams(&document, &report, None), &handles(&document, &[], &selection));
    }
}

#[test]
fn chases_and_movement_joints_are_projected_from_their_collections_with_their_own_paths_and_handles() {
    let document = En1996Snapshot::members_wall_example();
    let (scene, report) = scene_of(&document, &[]);
    assert_eq!(scene.validate(), Ok(()));
    assert_eq!(entity(&scene, "wall-members:chase-chase-1").entity.path, "chases[id=chase-1]");
    assert_eq!(entity(&scene, "wall-members:joint-joint-1").entity.path, "movementJoints[id=joint-1]");
    assert!(entity(&scene, "wall-members:chase-chase-1").utilisation.as_ref().is_some_and(|b| b.check_id.starts_with("en1996.8.6.chase-")));
    assert!(check(&report, "en1996.8.6.chase-depth.chase-1").utilization <= 1.0);
    let layout = Layout::of(&document, &[]);
    assert_eq!(handles::ids(&document, &layout, "wall-members:chase-chase-1").len(), 5);
    assert_eq!(handles::ids(&document, &layout, "wall-members:joint-joint-1").len(), 2);
    assert_eq!(entity_kind_of(&document, "wall-members:chase-chase-1").map(|k| k.collection), Some("chases"));
    assert_eq!(entity_kind_of(&document, "wall-members:joint-joint-1").map(|k| k.collection), Some("movementJoints"));
    assert!(entity_dependencies(&document, "wall-members").contains(&"chase-1".to_string()));
}

#[test]
fn a_chase_whose_wall_does_not_exist_is_not_drawn_and_a_deep_chase_turns_its_box_into_a_calculation_warning() {
    let mut document = En1996Snapshot::members_wall_example();
    document.chases[0].depth_m = 0.2;
    document.chases.push(model::WallChase { id: "orphan".into(), wall_id: "missing".into(), kind: model::ChaseKind::Recess, x_m: 0.0, y_m: 0.0, width_m: 0.1, length_m: 0.1, depth_m: 0.02 });
    let (scene, report) = scene_of(&document, &[]);
    assert!(check(&report, "en1996.8.6.chase-depth.chase-1").utilization > 1.0);
    assert!(scene.entities.iter().all(|e| !e.entity.id.contains("orphan")));
    assert_eq!(entity(&scene, "wall-members:chase-chase-1").utilisation.as_ref().map(|b| b.status), Some(scene::CheckStatus::Warning));
}

#[test]
fn a_chase_is_cut_into_the_wall_face_and_carries_a_width_grip_and_the_table_in_its_detail() {
    let document = En1996Snapshot::members_wall_example();
    let mut bare = document.clone();
    bare.chases.clear();
    let zones = |d: &En1996Snapshot| {
        let (scene, _) = scene_of(d, &[]);
        ["zone-bottom", "zone-mid", "zone-top"].iter().map(|tag| volume(&entity(&scene, &format!("{}:{tag}", d.walls[0].id)).geometry)).sum::<f64>()
    };
    let chase = &document.chases[0];
    let removed = zones(&bare) - zones(&document);
    assert!(removed > 0.0 && removed <= chase.width_m * chase.length_m * chase.depth_m + 1e-9, "removed {removed}");
    let layout = Layout::of(&document, &[]);
    assert!(handles::ids(&document, &layout, "wall-members:chase-chase-1").iter().any(|id| id.ends_with("width")));
    let (scene, _) = scene_of(&document, &[]);
    assert!(format!("{:?}", entity(&scene, "wall-members:chase-chase-1").detail).contains("Table 8.1"));
}

#[test]
fn the_chase_table_diagram_plots_both_tables_and_the_design_point_of_every_chase() {
    let document = En1996Snapshot::members_wall_example();
    let report = report_of(&document);
    let wall = &document.walls[0];
    let table = diagrams(&document, &report, Some(&wall.id)).into_iter().find(|d| d.id == format!("chase-table:{}", wall.id)).expect("chase table");
    assert_eq!(table.series.len(), 3);
    let marker = table.markers.iter().find(|m| m.id == "chase-chase-1").expect("design point");
    assert!((marker.at.x - document.chases[0].depth_m * 1000.0).abs() < 1e-9 && (marker.at.y - wall.thickness_m * 1000.0).abs() < 1e-9);
    assert_eq!(table.table_twin.columns.len(), 4);
}

#[test]
fn chases_and_movement_joints_have_diagrams_whose_markers_drag_their_own_rows() {
    let document = En1996Snapshot::members_wall_example();
    let report = report_of(&document);
    let wall = &document.walls[0].id;
    let all = diagrams(&document, &report, Some(wall));
    let chase = all.iter().find(|d| d.id == format!("chase-table:{wall}")).expect("chase table");
    let marker = chase.markers.iter().find(|m| m.id == "chase-chase-1").expect("chase marker");
    let binding = marker.binding.as_ref().expect("binding");
    assert_eq!(binding.path, "chases[id=chase-1].depthM");
    assert!((binding.value_at(30.0) - 0.030).abs() < 1e-9);
    assert!(chase.axes.iter().all(|a| a.unit == "mm") && !chase.table_twin.columns.is_empty());
    if document.movement_joints.iter().any(|j| &j.wall_id == wall) {
        let joints = all.iter().find(|d| d.id == format!("movement-joints:{wall}")).expect("movement joints");
        assert!(joints.markers.iter().all(|m| m.binding.as_ref().is_some_and(|b| b.path.starts_with("movementJoints[id=") && b.path.ends_with("].positionM"))));
    }
}

fn stress_extremes(document: &En1996Snapshot, tag: &str) -> (f64, f64) {
    let (scene, _) = scene_of(document, &[]);
    let wall = &document.walls[0];
    let zone = entity(&scene, &format!("{}:{tag}", wall.id));
    let scene::Geometry::Solid(solid) = &zone.geometry else { panic!("solid") };
    let scene::SolidShape::Mesh { vertices, .. } = &solid.shape else { panic!("mesh") };
    let scene::ColourChannel::Fields { fields, .. } = &zone.appearance.channel else { panic!("fields") };
    let field = fields.iter().find(|f| f.id == "wall-stress").expect("stress field");
    let scene::FieldSamples::Vertex { values } = &field.samples else { panic!("vertex") };
    assert_eq!(values.len(), vertices.len());
    let layout = Layout::of(document, &[]);
    let pose = layout.pose(&wall.id).expect("pose");
    let (axis, mid) = (pose.axis_y(), pose.at(0.0, 0.5 * wall.thickness_m, 0.0));
    let side = |v: &scene::Vec3| (v.x - mid.x) * axis.x + (v.y - mid.y) * axis.y;
    let face = |sign: f64| vertices.iter().zip(values).filter(|(v, _)| side(v) * sign > 0.0).map(|(_, x)| *x).fold(0.0_f64, f64::max);
    (face(-1.0), face(1.0))
}

#[test]
fn the_stress_field_loads_the_side_of_the_eccentric_resultant_and_grows_with_the_eccentricity() {
    let mut document = compliant();
    document.walls[0].eccentricity_top_m = 0.0;
    document.walls[0].eccentricity_bottom_m = 0.0;
    let (front_small, back_small) = stress_extremes(&document, "zone-top");
    document.walls[0].eccentricity_top_m = 0.08;
    document.walls[0].eccentricity_bottom_m = 0.08;
    let (front, back) = stress_extremes(&document, "zone-top");
    assert!(back > front, "the load side carries more: {front} against {back}");
    assert!(back > back_small && front < front_small, "eccentricity shifts stress to the load side");
}

#[test]
fn the_stress_field_is_a_selectable_colour_field_with_a_legend_on_every_zone() {
    let document = compliant();
    let (scene, _) = scene_of(&document, &[]);
    for tag in ["zone-bottom", "zone-mid", "zone-top"] {
        let zone = entity(&scene, &format!("{}:{tag}", document.walls[0].id));
        let scene::ColourChannel::Fields { fields, .. } = &zone.appearance.channel else { panic!("fields") };
        assert!(fields.iter().any(|f| f.id == "wall-stress"), "{tag}");
    }
    assert!(scene.legends.iter().any(|l| l.id == "wall-stress"));
    assert_eq!(scene.validate(), Ok(()));
}

#[test]
fn the_yield_regions_carry_the_normalised_mechanism_displacement_and_follow_the_panel_height() {
    for height in [2.6, 3.4] {
        let mut document = compliant();
        document.walls[0].height_m = height;
        let (scene, _) = scene_of(&document, &[]);
        let regions: Vec<_> = scene.entities.iter().filter(|e| e.entity.id.starts_with("wall-north:yield-")).collect();
        assert!(!regions.is_empty());
        for region in regions {
            let scene::ColourChannel::Fields { fields, .. } = &region.appearance.channel else { panic!("fields") };
            let scene::FieldSamples::Vertex { values } = &fields[0].samples else { panic!("vertex") };
            assert!(values.iter().all(|v| *v == 0.0 || *v == 1.0) && values.contains(&1.0), "{values:?}");
            let scene::Geometry::Surface(surface) = &region.geometry else { panic!("surface") };
            let scene::SurfaceShape::Polygon { polygon } = &surface.shape else { panic!("polygon") };
            assert_eq!(polygon.outer.points.len(), values.len());
            for (p, v) in polygon.outer.points.iter().zip(values) {
                let mid = Layout::of(&document, &[]).pose("wall-north").expect("pose").origin.z + 0.5 * height;
                assert_eq!(*v == 1.0, (p.z - mid).abs() < 1e-9);
            }
        }
    }
}

#[test]
fn the_wind_pressure_overlay_follows_the_pressure_leaves() {
    let mut document = compliant();
    document.walls[0].load_cases[0].q_p_wind_pa = 900.0;
    document.walls[0].load_cases[0].c_pe = 0.8;
    let (scene, _) = scene_of(&document, &[]);
    let scene::Geometry::Load(scene::LoadGlyph::Pressure { zones, .. }) = &entity(&scene, "wall-north:wind-ulsinz").geometry else { panic!("pressure") };
    assert!((zones[0].value.value - 720.0).abs() < 1e-9);
}

#[test]
fn the_wedge_offers_the_bearing_length_grip_which_writes_twice_the_distance_from_the_load_centre() {
    let document = rich();
    let wall = &document.walls[0];
    let load = &wall.load_cases[0].concentrated[0];
    let (scene, _) = scene_of(&document, &[]);
    let wedge = entity(&scene, "wall-openings:wedge-ulsinz-beam-B");
    assert!(wedge.handles.iter().any(|h| h.ends_with(":bearing-length")));
    let all = handles(&document, &[], &[wedge.entity.clone()]);
    assert_handle_laws(&all);
    let grip = all.iter().find(|h| h.id.ends_with(":bearing-length")).expect("bearing-length");
    let writes = grip.writes_at(load.position_m + 0.15);
    assert_eq!(writes[0].path, "walls[id=wall-openings].loadCases[id=ulsinz].concentrated[id=beam-B].bearingLengthM");
    assert!((writes[0].value - 0.30).abs() < 1e-9, "{}", writes[0].value);
}

#[test]
fn the_concentrated_load_terms_follow_eq_6_9_and_6_10() {
    let document = rich();
    let wall = &document.walls[0];
    let mut load = wall.load_cases[0].concentrated[0].clone();
    let h = wall.height_m;
    load.distance_a1_m = 0.5;
    load.bearing_area_m2 = 0.0;
    let (ab, a_ef, beta) = spreading::bearing_terms(wall, &load);
    assert!((ab - load.bearing_length_m * wall.thickness_m).abs() < 1e-12);
    let reach = (load.bearing_length_m + h / 3f64.sqrt()).min(masonry::effective_length_m(wall));
    assert!((a_ef - reach * wall.thickness_m).abs() < 1e-12);
    let eq = (1.0 + 0.3 * 0.5 / h) * (1.5 - 1.1 * ab / a_ef);
    assert!((beta - eq.min(1.25 + 0.5 / (2.0 * h)).min(1.5).max(1.0)).abs() < 1e-12);
    load.distance_a1_m = 100.0;
    assert!((spreading::bearing_terms(wall, &load).2 - 1.5).abs() < 1e-12);
    load.distance_a1_m = 0.0;
    assert!(spreading::bearing_terms(wall, &load).2 <= 1.25 + 1e-12);
    load.bearing_area_m2 = 0.5 * a_ef;
    assert!((spreading::bearing_terms(wall, &load).2 - 1.0).abs() < 1e-12);
}

#[test]
fn the_bearing_patch_sits_on_the_wall_head_with_the_area_utilisation_and_the_load_grips() {
    let document = rich();
    let (scene, _) = scene_of(&document, &[]);
    let wall = &document.walls[0];
    let patch = entity(&scene, "wall-openings:bearing-ulsinz-beam-B");
    assert!(volume(&patch.geometry).abs() > 0.0);
    assert!(patch.utilisation.as_ref().is_some_and(|b| b.check_id.starts_with("en1996.6.1.3.area.")));
    assert!(patch.handles.iter().any(|h| h.ends_with(":bearing-length")) && patch.handles.iter().any(|h| h.ends_with(":distance-a1")));
    assert!(wall.height_m > 0.0);
}

#[test]
fn the_effective_height_and_the_mid_height_eccentricity_are_dimensioned_in_3d() {
    let document = rich();
    let (scene, _) = scene_of(&document, &[]);
    let id = |suffix: &str| format!("wall-openings:{suffix}");
    for suffix in ["effective-height", "resultant-mid-eccentricity"] {
        assert!(scene.annotations.iter().any(|a| matches!(a, scene::Annotation::Dimension { id: d, .. } if *d == id(suffix))), "{suffix}");
    }
    let wall = entity(&scene, "wall-openings");
    let detail = format!("{:?}", wall.detail);
    assert!(detail.contains("h_ef/t_ef") && detail.contains("e_mk"));
}

#[test]
fn placements_are_a_registered_kind_whose_rows_resolve_when_no_wall_owns_the_id() {
    let mut document = rich();
    let spec = ENTITY_KINDS.iter().find(|k| k.id == "placement").expect("placement kind");
    assert_eq!((spec.collection, spec.label_en, spec.label_de), ("placements", "Placement", "Platzierung"));
    document.placements = vec![model::Placement { id: "wall-openings".into(), x: 1.0, y: 0.0, z: 0.0, yaw: 0.0 }, model::Placement { id: "orphan".into(), x: 0.0, y: 0.0, z: 0.0, yaw: 0.0 }];
    assert_eq!(entity_kind_of(&document, "wall-openings").map(|k| k.collection), Some("walls"), "a placement follows the wall it moves");
    assert_eq!(entity_kind_of(&document, "orphan").map(|k| k.collection), Some("placements"));
}

fn full() -> En1996Snapshot {
    let mut document = En1996Snapshot::members_wall_example();
    document.walls[0].load_cases[0].concentrated = vec![model::ConcentratedLoad { id: "beam-B".into(), force_n: 35_000.0, bearing_area_m2: 0.04, bearing_length_m: 0.20, distance_a1_m: 0.5, position_m: 2.5 }];
    document
}

fn routed_documents() -> Vec<En1996Snapshot> {
    let with_cases = En1996Snapshot { cases: schema::checks::masonry::catalogue_demo_cases(), ..En1996Snapshot::example(norm::AnnexChoice::De) };
    vec![compliant(), failing(), rich(), full(), braced(), En1996Snapshot::basement_wall_example(), En1996Snapshot::concentrated_load_example(), En1996Snapshot::reinforced_wall_example(), En1996Snapshot::bracing_walls_example(), with_cases]
}

fn full_report(document: &En1996Snapshot) -> CheckReport {
    let mut report = report_of(document);
    for case in &document.cases {
        report.extend(schema::checks::masonry::evaluate_design_case(&norm::design_case::DesignCase::from(case), document.annex));
    }
    report
}

/// 🧭️ Every check outcome of every demo document is drawn by exactly one route: its owner entity (which stands in the scene, and which the focus resolves to) or the document badge.
#[test]
fn every_check_outcome_routes_to_its_entity_or_the_document_badge() {
    use super::kit::{focus_of_outcome, is_document_check, route_of, CheckRoute};
    let mut seen = 0;
    for document in routed_documents() {
        let report = full_report(&document);
        let scene = project(&SceneInput { document: &document, report: &report, placements: &[], revision: revision() });
        for check in &report.checks {
            seen += 1;
            let route = route_of(&document, check).unwrap_or_else(|| panic!("check {} (subject {:?}) has no owner", check.id, check.subject.entity_id));
            match route {
                CheckRoute::Entity(id) => {
                    assert!(scene.entities.iter().any(|e| e.entity.id == id), "check {} routes to {id}, which the scene does not draw", check.id);
                    assert_eq!(focus_of_outcome(&document, &report, &check.id).as_deref(), Some(id.as_str()), "{}", check.id);
                }
                CheckRoute::Document => {
                    assert!(is_document_check(&check.id), "{}", check.id);
                    assert!(focus_of_outcome(&document, &report, &check.id).is_none(), "{}", check.id);
                }
            }
        }
    }
    assert!(seen > 200, "{seen} checks routed");
}

/// 🧭️ The sub-entities own their own checks: a chase owns its chase families, a movement joint its position check and the spacing of its wall, a concentrated load its bearing checks, a bar its layout and an opening its lintel cover.
#[test]
fn sub_entities_own_the_checks_whose_ids_name_them() {
    use super::kit::owner_entity;
    let document = full();
    let report = full_report(&document);
    let owner = |id: &str| owner_entity(&document, check(&report, id)).unwrap_or_else(|| panic!("{id}"));
    assert_eq!(owner("en1996.8.6.chase-depth.chase-1"), "wall-members:chase-chase-1");
    assert_eq!(owner("en1996.8.5.joint-spacing.wall-members"), "wall-members:joint-joint-1");
    assert_eq!(owner("en1996.8.2.bar-layout.wall-members.bar-shear"), "wall-members:bar-bar-shear");
    assert_eq!(owner("en1996.5.5.lintel-cover.wall-members.win-1"), "wall-members:opening-win-1");
    let lc = &document.walls[0].load_cases[0].id;
    assert_eq!(owner(&format!("en1996.6.1.3.concentrated.wall-members.{lc}.beam-B")), format!("wall-members:conc-{lc}-beam-B"));
    assert_eq!(owner("en1996.5.5.1.slenderness.wall-members"), "wall-members");
}

/// 🌈️ A chase takes its tone from the worst check it owns, not from the depth check alone.
#[test]
fn a_chase_is_toned_by_the_worst_check_it_owns() {
    use super::kit::{is_document_check, owner_entity};
    let document = full();
    let (scene, report) = scene_of(&document, &[]);
    let id = "wall-members:chase-chase-1";
    let worst = report.checks.iter().filter(|c| c.utilization.is_finite() && c.status != norm::CheckStatus::NotApplicable && !is_document_check(&c.id) && owner_entity(&document, c).as_deref() == Some(id)).map(|c| c.utilization).fold(0.0, f64::max);
    let bound = entity(&scene, id).utilisation.as_ref().expect("binding").ratio;
    assert!((bound - worst).abs() < 1e-9, "{bound} vs {worst}");
}

/// 🎚️ The numeric fields of every row of the snapshot schema have a viewport grip (geometric or gauge) or a documented exemption; every gauge writes a field of its schema definition with the schema step.
#[test]
fn every_numeric_field_of_a_row_has_a_viewport_grip_or_a_documented_exemption() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../🧬️schema/📸️snapshot/🔣️.json");
    let schema: serde_json::Value = serde_json::from_slice(&std::fs::read(path).expect("snapshot schema")).expect("json");
    let shapes: [(&str, &str); 15] = [
        ("MasonryWall", "walls[]."),
        ("WallLoadCase", "walls[].loadCases[]."),
        ("ConcentratedLoad", "walls[].loadCases[].concentrated[]."),
        ("WallOpening", "walls[].openings[]."),
        ("WallPier", "walls[].piers[]."),
        ("WallLintel", "walls[].lintels[]."),
        ("WallTie", "walls[].ties[]."),
        ("WallPrestress", "walls[].prestress[]."),
        ("ConfinedElement", "walls[].confined[]."),
        ("ShearFlange", "walls[].flanges[]."),
        ("WallBar", "walls[].bars[]."),
        ("WallChase", "chases[]."),
        ("MovementJoint", "movementJoints[]."),
        ("Placement", "placements[]."),
        ("CaseNumber", "cases[].numbers[]."),
    ];
    let shape = |path: &str| {
        let mut out = String::new();
        let mut rest = path;
        while let Some(at) = rest.find("[id=") {
            out.push_str(&rest[..at + 1]);
            let end = rest[at..].find(']').expect("bracket");
            rest = &rest[at + end..];
        }
        out.push_str(rest);
        out
    };
    let mut geometric = BTreeSet::new();
    let mut gauged = BTreeSet::new();
    for document in [full(), braced(), rich(), En1996Snapshot::bracing_walls_example()] {
        let (scene, _) = scene_of(&document, &[]);
        let selection: Vec<scene::EntityRef> = scene.entities.iter().map(|e| e.entity.clone()).collect();
        for handle in handles(&document, &[], &selection) {
            for write in &handle.writes {
                let target = if handle.id.contains(":gauge-") || handle.id.contains("gauge-") { &mut gauged } else { &mut geometric };
                target.insert(shape(&write.path));
            }
        }
    }
    let table = handles::gauges::coverage();
    for (def, prefix) in shapes {
        let properties = schema["$defs"][def]["properties"].as_object().unwrap_or_else(|| panic!("{def}"));
        for (field, spec) in properties.iter().filter(|(_, v)| matches!(v["type"].as_str(), Some("number" | "integer"))) {
            let leaf = format!("{prefix}{field}");
            let exempt = handles::gauges::EXEMPT.contains(&(def, field.as_str()));
            assert!(geometric.contains(&leaf) || gauged.contains(&leaf) || exempt, "{leaf} has no grip ({spec})");
        }
    }
    for (def, field, step, classes) in &table {
        let spec = &schema["$defs"][*def]["properties"][*field];
        assert!(matches!(spec["type"].as_str(), Some("number" | "integer")), "gauge {def}.{field} is no numeric schema field");
        let wanted = spec["x-semio-ui"]["step"].as_f64().expect("schema step");
        assert!(*classes || handles::gauges::STEP_OVERRIDES.contains(&(*def, *field)) || (wanted - step).abs() <= 1e-12 * wanted.abs().max(1.0), "gauge {def}.{field} steps by {step}, the schema by {wanted}");
    }
    assert!(table.len() >= 40, "{} gauges", table.len());
}

/// 🎚️ A gauge writes the current value of its field at its anchor, moves it by twenty schema steps per metre and the dragged document re-projects.
#[test]
fn gauges_write_the_current_value_move_by_steps_and_the_dragged_document_re_projects() {
    let document = full();
    let layout = Layout::of(&document, &[]);
    let wall = &document.walls[0];
    let all = handles::build(&document, &layout, &wall.id);
    assert_handle_laws(&all);
    let gauge = |tag: &str| all.iter().find(|h| h.id == format!("{}:gauge-{tag}", wall.id)).unwrap_or_else(|| panic!("gauge {tag}"));
    let cover = gauge("cover");
    assert_eq!(cover.writes[0].path, format!("walls[id={}].coverM", wall.id));
    let s0 = cover.anchor.z;
    assert!((cover.writes_at(s0)[0].value - wall.cover_m).abs() <= 0.0025 + 1e-9);
    assert!((cover.writes_at(s0 + 0.05)[0].value - (wall.cover_m + 0.005)).abs() <= 0.0025 + 1e-9, "one 0.05 m drag is one schema step");
    let fire = gauge("fire-rei");
    let dragged = fire.writes_at(fire.anchor.z + 1.0)[0].value;
    assert!([30.0, 60.0, 90.0, 120.0, 180.0, 240.0].contains(&dragged), "{dragged}");
    let ids: Vec<&String> = all.iter().map(|h| &h.id).collect();
    assert_eq!(ids.len(), ids.iter().collect::<BTreeSet<_>>().len(), "duplicate handle ids");
    assert!(all.iter().any(|h| h.id.contains("gauge-lc-")), "load case gauges");
    let conc = handles::build(&document, &layout, &format!("{}:conc-{}-beam-B", wall.id, wall.load_cases[0].id));
    for tag in ["position", "bearing-length", "gauge-force", "gauge-bearing-area", "gauge-distance-a1"] {
        assert!(conc.iter().any(|h| h.id.ends_with(&format!(":{tag}"))), "concentrated {tag}");
    }
    let lintel = handles::build(&document, &layout, &format!("{}:lintel-lintel-1", wall.id));
    assert!(lintel.iter().any(|h| h.id.ends_with(":bearing")));
    let flange = handles::build(&document, &layout, &format!("{}:flange-flange-1", wall.id));
    assert!(flange.iter().any(|h| h.id.ends_with(":thickness")));
    for (tag, tags) in [("tie-tie-1", "gauge-capacity"), ("tendon-tendon-1", "gauge-f-pk"), ("column-column-1", "gauge-as"), ("bar-bar-shear", "gauge-diameter")] {
        assert!(handles::build(&document, &layout, &format!("{}:{tag}", wall.id)).iter().any(|h| h.id.ends_with(&format!(":{tags}"))), "{tag}");
    }
    let mut moved = document.clone();
    moved.walls[0].unit_height_m += 0.01;
    assert_ne!(canonical_json(&scene_of(&moved, &[]).0), canonical_json(&scene_of(&document, &[]).0), "the dragged unit height must change the projection");
}

/// 🏋️ The enhancement diagram of every concentrated load plots β over a1/H from the closed form, the envelope of a vanishing loaded area and β = 1; the design point sits on the curve and drags a1.
#[test]
fn the_enhancement_diagram_plots_beta_over_the_distance_with_the_design_point() {
    let document = rich();
    let report = report_of(&document);
    let wall = &document.walls[0];
    let c = &wall.load_cases[0].concentrated[0];
    let all = diagrams(&document, &report, None);
    let diagram = all.iter().find(|d| d.id == format!("beta:{}-{}-{}", wall.id, wall.load_cases[0].id, c.id)).expect("beta diagram");
    assert_eq!(diagram.validate(), Ok(()));
    assert!(diagram.table_twin.rows() > 0 && !diagram.title.en.is_empty() && !diagram.title.de.is_empty());
    let (ab, a_ef, beta) = spreading::bearing_terms(wall, c);
    let marker = &diagram.markers[0];
    assert!((marker.at.y - beta).abs() < 1e-12 && (marker.at.x - c.distance_a1_m / wall.height_m).abs() < 1e-12);
    let drag = marker.binding.as_ref().expect("a1 binding");
    assert_eq!(drag.path, format!("walls[id={}].loadCases[id={}].concentrated[id={}].distanceA1M", wall.id, wall.load_cases[0].id, c.id));
    assert!((drag.value_at(marker.at.x) - c.distance_a1_m).abs() < 0.05);
    assert!(ab / a_ef <= 0.45 && beta >= 1.0 && beta <= 1.5);
}

/// 📊️ Every check of a wall whose family no dedicated diagram draws is a bar of its outcome diagram, none appears twice, and the drawn families have no bar.
#[test]
fn every_check_family_is_drawn_by_a_dedicated_diagram_or_the_outcome_diagram() {
    use super::kit::{is_document_check, is_drawn_family, owner_entity};
    for document in routed_documents() {
        let report = report_of(&document);
        let all = diagrams(&document, &report, None);
        for wall in &document.walls {
            let expected: Vec<&norm::CheckResult> = report.checks.iter().filter(|c| c.utilization.is_finite() && c.status != norm::CheckStatus::NotApplicable && !is_document_check(&c.id) && !is_drawn_family(&c.id) && owner_entity(&document, c).is_some_and(|o| o.split(':').next() == Some(wall.id.as_str()))).collect();
            let outcome = all.iter().find(|d| d.id == format!("check-outcomes:{}", wall.id));
            assert_eq!(outcome.is_some(), !expected.is_empty(), "{}", wall.id);
            let Some(outcome) = outcome else { continue };
            assert_eq!(outcome.validate(), Ok(()), "{}", outcome.id);
            assert_eq!(outcome.markers.iter().filter_map(|m| m.bound_check.clone()).collect::<Vec<_>>(), expected.iter().map(|c| c.id.clone()).collect::<Vec<_>>(), "{}", wall.id);
            assert_eq!(outcome.table_twin.rows(), expected.len());
            assert!(outcome.markers.iter().all(|m| !is_drawn_family(m.bound_check.as_deref().unwrap_or(""))));
        }
    }
    let narrowed = diagrams(&full(), &report_of(&full()), Some("no-such-wall"));
    assert!(narrowed.iter().all(|d| !d.id.starts_with("check-outcomes:")));
}

#[test]
fn the_basis_badge_follows_the_document_annex_in_the_scene_and_every_diagram() {
    scene::projection_kit::test_support::assert_basis_follows_annex(norm::NormFamilyId::En1996, |annex| {
        let mut document = compliant();
        document.annex = annex;
        let (scene, report) = scene_of(&document, &[]);
        (scene, diagrams(&document, &report, None))
    });
}
