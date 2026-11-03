use super::*;
use scene::projection_kit::test_support::{revision, entity, assert_handle_laws, assert_scene_laws, assert_diagram_laws};
use semio_s_artifact_norm_en1992_checks as checks;
use model::{En1992Snapshot, MemberKind};
use scene::{canonical_json, Validate};
use std::collections::BTreeSet;

const ORACLE_MRD_B1: f64 = 217_897.637_832_300_88;
const ORACLE_SR_MAX_B1: f64 = 0.219_680_488_269_235_4;
const ORACLE_W_SS_B1: f64 = 0.003_272_727_272_727_272_7;
const ORACLE_ISOTHERM: [(f64, f64); 4] = [(30.0, 0.010_493_288_942_393_745), (60.0, 0.022_213_696_885_220_634), (90.0, 0.031_058_201_417_511_318), (120.0, 0.038_539_786_363_084_694)];
const ORACLE_ISO834: [(f64, f64); 2] = [(60.0, 945.340_051_348_972), (120.0, 1_049.039_568_745_648)];
const EXISTING_RULES: [&str; 26] = ["x", "y", "yaw", "span", "width", "height", "effectiveDepth", "cover", "longitudinal[].count", "longitudinal[].diameter", "stirrups.spacing", "fire.axisDistance", "prestress.eccentricity", "punching.columnWidth", "punching.columnDepth", "bars[].y", "bars[].z", "bars[].length", "tendonPoints[].x", "tendonPoints[].z", "studRows[].spacingFirst", "studRows[].count", "hEf", "d", "c1", "headDiameter"];

fn report_of(document: &En1992Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    for member in &document.members {
        report.extend(checks::evaluate_member(document, member));
    }
    for anchor in &document.anchors {
        report.extend(checks::evaluate_anchor(document, anchor));
    }
    report
}

fn built(document: &En1992Snapshot, placements: &[Placement]) -> (Built, CheckReport) {
    let report = report_of(document);
    let built = build(&SceneInput { document, report: &report, placements, revision: revision() });
    (built, report)
}

fn fixtures() -> Vec<En1992Snapshot> {
    vec![En1992Snapshot::compliant_office_frame(), En1992Snapshot::failing_under_reinforced(), En1992Snapshot::compliant_prestressed_beam(), En1992Snapshot::failing_prestressed_beam(), En1992Snapshot::liquid_retaining_fem_anchor(), En1992Snapshot::flat_slab_punching()]
}

fn close(a: f64, b: f64, tolerance: f64) {
    assert!((a - b).abs() <= tolerance * b.abs().max(1e-12), "{a} vs {b}");
}

#[test]
fn every_fixture_scene_and_diagram_validates() {
    for document in fixtures() {
        let (built, _) = built(&document, &[]);
        assert_scene_laws(&built.scene);
        assert_diagram_laws(&built.diagrams);
        assert_handle_laws(&built.handles);
        assert_eq!(built.scene.validate(), Ok(()), "{}", document.title);
        for diagram in &built.diagrams {
            assert_eq!(diagram.validate(), Ok(()), "{} / {}", document.title, diagram.id);
        }
        for handle in &built.handles {
            assert_eq!(handle.validate(), Ok(()), "{} / {}", document.title, handle.id);
        }
    }
}

#[test]
fn projection_is_deterministic_bilingual_and_ids_are_unique() {
    let (a, _) = built(&En1992Snapshot::compliant_office_frame(), &[]);
    assert_scene_laws(&a.scene);
    assert_diagram_laws(&a.diagrams);
    let (b, _) = built(&En1992Snapshot::compliant_office_frame(), &[]);
    assert_scene_laws(&b.scene);
    assert_diagram_laws(&b.diagrams);
    assert_eq!(canonical_json(&a.scene), canonical_json(&b.scene));
    assert!(a.scene.entities.iter().all(|e| !e.accessible_name.en.is_empty() && !e.accessible_name.de.is_empty() && !e.accessible_detail.de.is_empty()));
    let ids: BTreeSet<_> = a.scene.entities.iter().map(|e| e.entity.id.clone()).collect();
    assert_eq!(ids.len(), a.scene.entities.len());
}

#[test]
fn a_beam_has_one_entity_per_bar_a_link_cage_cover_supports_and_loads() {
    let document = En1992Snapshot::compliant_office_frame();
    let (built, _) = built(&document, &[]);
    assert_scene_laws(&built.scene);
    assert_diagram_laws(&built.diagrams);
    assert_handle_laws(&built.handles);
    let beam = document.members.iter().find(|m| m.id == "beam-B1").expect("beam");
    let bars = built.scene.entities.iter().filter(|e| e.entity.id.starts_with("beam-B1:bar-")).count();
    assert_eq!(bars, beam.longitudinal.iter().map(|l| l.count as usize).sum::<usize>());
    let links = entity(&built.scene, "beam-B1:links");
    let scene::Geometry::Fastener(g) = &links.geometry else { panic!("fastener") };
    let spacing = beam.stirrups.as_ref().expect("stirrups").spacing;
    let Some(scene::Pattern::Cage { zones, outline, .. }) = &g.pattern else { panic!("cage") };
    assert_eq!(zones.len(), 1);
    assert_eq!(zones[0].spacing, spacing);
    assert!(outline.signed_area().abs() > 0.0);
    assert_eq!(zones[0].count, ((beam.span - 2.0 * beam.cover) / spacing).floor() as u32 + 1);
    let scene::Geometry::Member(concrete) = &entity(&built.scene, "beam-B1").geometry else { panic!("member") };
    close(concrete.section.area().expect("area"), beam.width * beam.height, 1e-9);
    assert!(built.scene.entities.iter().any(|e| e.entity.id == "beam-B1:cover"));
    assert_eq!(built.scene.entities.iter().filter(|e| e.entity.id.starts_with("beam-B1:support-")).count(), 2);
    assert!(built.scene.entities.iter().any(|e| e.entity.id.starts_with("beam-B1:load-")));
    assert!(entity(&built.scene, "beam-B1").appearance.deformation.is_some());
}

#[test]
fn a_prestressed_beam_carries_a_parabolic_tendon_with_its_eccentricity() {
    let document = En1992Snapshot::compliant_prestressed_beam();
    let (built, _) = built(&document, &[]);
    assert_scene_laws(&built.scene);
    assert_diagram_laws(&built.diagrams);
    assert_handle_laws(&built.handles);
    let member = &document.members[0];
    let tendon = entity(&built.scene, "beam-PS1:tendon");
    let scene::Geometry::Fastener(g) = &tendon.geometry else { panic!("fastener") };
    let scene::FastenerKind::Tendon { profile: scene::Path3::Spline(spline), .. } = &g.kind else { panic!("spline tendon") };
    assert_eq!(spline.control.len(), 3);
    let curve_mid = 0.25 * spline.control[0].z + 0.5 * spline.control[1].z + 0.25 * spline.control[2].z;
    close(spline.control[0].z - curve_mid, member.prestress.as_ref().expect("prestress").eccentricity, 1e-9);
    assert!(built.handles.iter().any(|h| h.id == "beam-PS1/tendon/eccentricity"));
}

#[test]
fn a_flat_slab_has_a_column_and_reduced_control_perimeters() {
    let document = En1992Snapshot::compliant_office_frame();
    let (built, _) = built(&document, &[]);
    assert_scene_laws(&built.scene);
    assert_diagram_laws(&built.diagrams);
    assert_handle_laws(&built.handles);
    let slab = document.members.iter().find(|m| m.kind == MemberKind::FlatSlab).expect("flat slab");
    assert!(matches!(entity(&built.scene, &slab.id).geometry, scene::Geometry::Plate(_)));
    assert!(matches!(entity(&built.scene, &format!("{}:column", slab.id)).geometry, scene::Geometry::Solid(_)));
    let (u0, u1) = (entity(&built.scene, &format!("{}:punching-u0", slab.id)), entity(&built.scene, &format!("{}:punching-u1", slab.id)));
    let ring_len = |e: &scene::SceneEntity| {
        let scene::Geometry::Surface(scene::SurfaceGeometry { shape: scene::SurfaceShape::Polygon { polygon } }) = &e.geometry else { panic!("polygon") };
        let p = &polygon.outer.points;
        (0..p.len()).map(|i| p[i].distance(&p[(i + 1) % p.len()])).sum::<f64>()
    };
    let p = slab.punching.as_ref().expect("punching");
    close(ring_len(u0), 2.0 * (p.column_width + p.column_depth), 1e-9);
    let d = slab.effective_depth;
    let analytic = 2.0 * (p.column_width + p.column_depth) + 2.0 * std::f64::consts::PI * 2.0 * d;
    assert!((ring_len(u1) - analytic).abs() / analytic < 0.01, "{} vs {analytic}", ring_len(u1));
}

#[test]
fn an_anchor_has_a_cone_cut_at_a_close_edge_and_bound_dimensions() {
    let document = En1992Snapshot::compliant_office_frame();
    let (built, _) = built(&document, &[]);
    assert_scene_laws(&built.scene);
    assert_diagram_laws(&built.diagrams);
    assert_handle_laws(&built.handles);
    let anchor = &document.anchors[0];
    assert!(anchor.c1 < 1.5 * anchor.h_ef);
    let scene::Geometry::Solid(scene::SolidGeometry { shape: scene::SolidShape::Mesh { vertices, .. } }) = &entity(&built.scene, "anc-1:cone").geometry else { panic!("mesh") };
    let min_x = vertices.iter().map(|v| v.x).fold(f64::MAX, f64::min);
    close(min_x - layout::Layout::of(&document, &[]).anchor("anc-1").expect("anchor").x, -anchor.c1, 1e-9);
    assert!(built.scene.annotations.iter().any(|a| a.id() == "anc-1:dim-hef"));
    assert!(built.handles.iter().any(|h| h.id == "anc-1/embedment"));
}

#[test]
fn the_interaction_surface_encloses_the_origin_and_the_design_point_is_drawn() {
    let (built, _) = built(&En1992Snapshot::compliant_office_frame(), &[]);
    assert_scene_laws(&built.scene);
    assert_diagram_laws(&built.diagrams);
    assert_handle_laws(&built.handles);
    let surface = entity(&built.scene, "col-C1:interaction");
    let scene::Geometry::Surface(scene::SurfaceGeometry { shape: scene::SurfaceShape::Grid { grid } }) = &surface.geometry else { panic!("grid") };
    assert_eq!(grid.rows as usize * grid.columns as usize, grid.points.len());
    assert!(matches!(entity(&built.scene, "col-C1:load-point").geometry, scene::Geometry::Load(_)));
    assert!(built.diagrams.iter().any(|d| d.id == "interaction-my-n:col-C1"));
}

#[test]
fn bending_resistance_matches_the_strain_compatibility_oracle() {
    let document = En1992Snapshot::compliant_office_frame();
    let beam = document.members.iter().find(|m| m.id == "beam-B1").expect("beam");
    let sec = mechanics::member_sec(&document, beam).expect("section");
    let (m_rd, _) = sec.m_rd_y(0.0).expect("N = 0 reachable");
    close(m_rd / 1.0e3, ORACLE_MRD_B1, 2e-3);
    let state = mechanics::crack_state(&sec, beam, &document.action_factors()).expect("crack state");
    close(state.sr_max, ORACLE_SR_MAX_B1, 1e-6);
    let w = mechanics::deflection_coefficient(model::SupportCondition::SimplySupported, 0.5);
    close(w, 5.0 / 384.0, 1e-12);
    let mut loaded = beam.clone();
    loaded.actions.clear();
    loaded.udl = 20.0e3;
    close(mechanics::deflection(&sec, &loaded, 0.5), ORACLE_W_SS_B1, 1e-9);
}

#[test]
fn the_material_laws_and_the_slab_isotherm_follow_the_stdlib_oracle() {
    for (minutes, expected) in ORACLE_ISO834 {
        close(checks::iso834(minutes), expected, 1e-12);
    }
    close(checks::fire_field::conductivity(20.0), 2.0 - 0.2451 * 0.2 + 0.0107 * 0.04, 1e-12);
    close(checks::fire_field::specific_heat(500.0, 1.5), 1100.0, 1e-12);
    close(checks::fire_field::specific_heat(110.0, 1.5), 1470.0, 1e-12);
    close(checks::fire_field::density(500.0), 2300.0 * (0.95 - 0.07 * 100.0 / 800.0), 1e-12);
    for (minutes, expected) in ORACLE_ISOTHERM {
        let field = checks::fire_field::solve(1.0, 0.4, (false, false, true, false), minutes, checks::fire_field::MOISTURE_DEFAULT);
        let depth = field.isotherm_depth(checks::fire_field::LEVEL_500);
        assert!((depth - expected).abs() < 2.5e-3, "{minutes} min: {depth} vs {expected}");
    }
}

#[test]
fn the_isotherm_advances_with_time_and_a_column_is_heated_from_four_sides_symmetrically() {
    let depths: Vec<f64> = [30.0, 60.0, 90.0, 120.0].map(|t| checks::fire_field::solve(0.4, 0.4, (true, true, true, true), t, checks::fire_field::MOISTURE_DEFAULT).isotherm_depth(checks::fire_field::LEVEL_500)).to_vec();
    assert!(depths.windows(2).all(|w| w[1] > w[0]) && depths[0] > 0.005);
    let field = checks::fire_field::solve(0.4, 0.4, (true, true, true, true), 60.0, checks::fire_field::MOISTURE_DEFAULT);
    close(field.at(-0.1, 0.05), field.at(0.1, 0.05), 1e-9);
    close(field.at(0.05, -0.1), field.at(-0.05, 0.1), 1e-9);
    assert!(field.at(0.19, 0.19) > field.at(0.19, 0.0), "the corner is hotter than the face centre");
    let (y0, y1, z0, z1) = field.reduced_rectangle(checks::fire_field::LEVEL_500).expect("reduced");
    close(y0, -y1, 1e-6);
    close(z0, -z1, 1e-6);
    assert_eq!(field.isotherm_ring(checks::fire_field::LEVEL_500, 72).len(), 72);
}

#[test]
fn a_zero_minute_fire_leaves_the_whole_section_at_ambient_temperature() {
    let field = checks::fire_field::solve(0.3, 0.3, (true, true, true, true), 0.0, checks::fire_field::MOISTURE_DEFAULT);
    assert!(field.temperatures.iter().all(|t| (*t - 20.0).abs() < 1e-9));
}

#[test]
fn the_surface_row_of_centric_tension_and_full_compression_match_the_closed_forms() {
    let document = En1992Snapshot::compliant_office_frame();
    let column = document.members.iter().find(|m| m.id == "col-C1").expect("column");
    let sec = mechanics::member_sec(&document, column).expect("section");
    let area: f64 = sec.bars.iter().map(|b| b.a).sum();
    close(mechanics::tension(&sec).n, -sec.fyd * area, 1e-12);
    let squash = sec.fcd * sec.gross_props().area + (sec.es * sec.c.eps_c2 / 1000.0).min(sec.fyd) * area - sec.fcd * area;
    close(mechanics::n0(&sec), squash, 5e-3);
    let rows = mechanics::surface(&sec, 24);
    assert_eq!(rows.len(), mechanics::X_LADDER.len() + 1);
    assert!(rows.iter().all(|r| r.len() == 25));
}

#[test]
fn the_section_geometry_and_fire_diagrams_show_the_reduced_sections() {
    let (built, _) = built(&En1992Snapshot::compliant_office_frame(), &[]);
    assert_scene_laws(&built.scene);
    assert_diagram_laws(&built.diagrams);
    assert_handle_laws(&built.handles);
    let geometry = built.diagrams.iter().find(|d| d.id == "section-geometry:beam-B1").expect("geometry diagram");
    assert!(geometry.series.iter().any(|s| s.id() == "torsion-wall") && geometry.series.iter().any(|s| s.id().starts_with("ac-eff")));
    assert!(geometry.table_twin.columns.iter().all(|c| c.values.len() == 1));
    let fire = built.diagrams.iter().find(|d| d.id == "fire-section:beam-B1").expect("fire diagram");
    assert!(fire.series.iter().any(|s| s.id() == "isotherm") && fire.series.iter().any(|s| s.id() == "zone"));
    assert!(built.scene.entities.iter().any(|e| e.entity.id == "beam-B1:zone-section"));
}

#[test]
fn the_section_geometry_wall_is_the_effective_wall_of_the_torsion_check() {
    let document = En1992Snapshot::compliant_office_frame();
    let (built, _) = built(&document, &[]);
    let mut seen = 0;
    for m in &document.members {
        let Some(sec) = mechanics::member_sec(&document, m) else { continue };
        let Some(diagram) = built.diagrams.iter().find(|d| d.id == format!("section-geometry:{}", m.id)) else { continue };
        let column = diagram.table_twin.columns.iter().find(|c| c.header.en == "Effective wall t_ef").expect("t_ef column");
        let expected = sec.torsion_section(checks::torsion_t_min_mm(m)).map_or(0.0, |t| t.t_ef * 1e-3);
        assert!((column.values[0] - expected).abs() < 1e-12, "{}: {} vs {}", m.id, column.values[0], expected);
        seen += 1;
    }
    assert!(seen > 0);
}

#[test]
fn fire_capacity_falls_over_time_and_the_liquid_wall_shows_its_pressure() {
    let (built_frame, _) = built(&En1992Snapshot::compliant_office_frame(), &[]);
    assert_diagram_laws(&built_frame.diagrams);
    let capacity = built_frame.diagrams.iter().find(|d| d.id == "fire-capacity:beam-B1").expect("fire capacity diagram");
    let moments = &capacity.table_twin.columns[1].values;
    assert!(moments.len() == 9 && moments[8] <= moments[1] && moments[8] > 0.0);
    let (tank, _) = built(&En1992Snapshot::liquid_retaining_fem_anchor(), &[]);
    assert_diagram_laws(&tank.diagrams);
    let pressure = tank.diagrams.iter().find(|d| d.id.starts_with("liquid-pressure:")).expect("liquid pressure diagram");
    let values = &pressure.table_twin.columns[1].values;
    assert!(values.first().is_some_and(|p| *p > 0.0) && values.last().is_some_and(|p| p.abs() < 1e-9));
}

#[test]
fn the_default_layout_runs_members_left_to_right_with_a_gap_and_placements_win() {
    let document = En1992Snapshot::compliant_office_frame();
    let layout = Layout::of(&document, &[]);
    let first = layout.member(&document.members[0].id).expect("first");
    let second = layout.member(&document.members[1].id).expect("second");
    assert_eq!(first.frame.origin.x, 0.0);
    close(second.frame.origin.x, first.length + layout::MEMBER_GAP, 1e-12);
    let moved = Layout::of(&document, &[Placement { entity_id: document.members[1].id.clone(), origin: scene::Vec3::new(40.0, 3.0, 0.5), yaw: 0.0 }]);
    assert_eq!(moved.member(&document.members[1].id).expect("moved").frame.origin, scene::Vec3::new(40.0, 3.0, 0.5));
}

#[test]
fn bar_positions_respect_cover_and_never_overlap() {
    for document in fixtures() {
        for member in &document.members {
            let width = if member.kind == MemberKind::FlatSlab { member.span } else { member.width };
            for layer in reinforcement::bar_layout(member, width) {
                let spec = &member.longitudinal[layer.layer];
                let inset = member.cover + reinforcement::link_diameter(member) + spec.diameter / 2.0;
                for &(y, z) in &layer.points {
                    assert!(y.abs() <= width / 2.0 - inset + 1e-9 && z.abs() <= member.height / 2.0 - inset + 1e-9, "{} {y} {z}", member.id);
                }
                for (i, a) in layer.points.iter().enumerate() {
                    for b in &layer.points[i + 1..] {
                        assert!((a.0 - b.0).hypot(a.1 - b.1) >= spec.diameter - 1e-9, "{} overlapping bars", member.id);
                    }
                }
            }
        }
    }
}

#[test]
fn every_handle_resolves_round_trips_and_is_referenced_by_its_entity() {
    for document in fixtures() {
        let (built, _) = built(&document, &[]);
        assert_scene_laws(&built.scene);
        assert_diagram_laws(&built.diagrams);
        assert_handle_laws(&built.handles);
        let known: BTreeSet<&str> = built.handles.iter().map(|h| h.id.as_str()).collect();
        assert_eq!(known.len(), built.handles.len(), "unique handle ids");
        for entity in &built.scene.entities {
            assert!(entity.handles.iter().all(|h| known.contains(h.as_str())), "{} references an unknown handle", entity.entity.id);
        }
        let referenced: BTreeSet<&str> = built.scene.entities.iter().flat_map(|e| e.handles.iter().map(String::as_str)).collect();
        for handle in &built.handles {
            assert!(referenced.contains(handle.id.as_str()), "{} is not attached to an entity", handle.id);
            for write in &handle.writes {
                let tail = write.path.split_once("].").map_or(write.path.as_str(), |(_, rest)| rest);
                let generic = generic_path(tail);
                assert!(EXISTING_RULES.contains(&generic.as_str()), "unknown field {generic}");
            }
            let at_rest = handle.writes_at(0.0);
            for (write, resolved) in handle.writes.iter().zip(&at_rest) {
                assert!((write.map.base - resolved.value).abs() < 1e-2, "{}: {} vs {}", handle.id, write.map.base, resolved.value);
            }
        }
    }
}

fn generic_path(tail: &str) -> String {
    let mut out = String::new();
    let mut rest = tail;
    while let Some(i) = rest.find("[id=") {
        out.push_str(&rest[..i]);
        out.push_str("[]");
        rest = &rest[i..];
        rest = rest.find(']').map_or("", |j| &rest[j + 1..]);
    }
    out.push_str(rest);
    out
}

fn geometry_document() -> En1992Snapshot {
    let mut d = En1992Snapshot::compliant_office_frame();
    let beam = d.members.iter_mut().find(|m| m.kind == MemberKind::Beam).expect("beam");
    let layer = beam.longitudinal[0].clone();
    let z = -beam.height / 2.0 + beam.cover + 0.03;
    beam.bars = (0..3).map(|k| model::BarPoint { id: format!("b{k}"), layer_id: layer.id.clone(), y: -0.08 + 0.08 * k as f64, z, diameter: 0.016, length: 0.0, bend: semio_s_artifact_norm_en1992_model::BarBend::Auto }).collect();
    beam.tendon_points = vec![model::TendonPoint { id: "t0".into(), x: 0.0, y: 0.0, z: 0.0 }, model::TendonPoint { id: "t1".into(), x: beam.span / 2.0, y: 0.0, z: -0.1 }, model::TendonPoint { id: "t2".into(), x: beam.span, y: 0.0, z: 0.0 }];
    let slab = d.members.iter_mut().find(|m| m.kind == MemberKind::FlatSlab).expect("flat slab");
    slab.stud_rows = vec![model::StudRow { id: "r1".into(), angle: 0.0, count: 4, spacing_first: 0.15, spacing_radial: 0.15, diameter: 0.012 }];
    d
}

#[test]
fn explicit_bars_tendon_points_and_stud_rows_become_entities_with_handles() {
    let document = geometry_document();
    let (built, _) = built(&document, &[]);
    assert_scene_laws(&built.scene);
    assert_diagram_laws(&built.diagrams);
    assert_handle_laws(&built.handles);
    assert_eq!(built.scene.validate(), Ok(()));
    let beam = document.members.iter().find(|m| m.kind == MemberKind::Beam).expect("beam");
    let layer = &beam.longitudinal[0];
    for k in 0..3 {
        let bar = entity(&built.scene, &format!("{}:bar-{}-{k}", beam.id, layer.id));
        assert_eq!(bar.handles.len(), if k == 0 { 5 } else { 3 });
    }
    assert!(built.handles.iter().any(|h| h.writes.iter().any(|w| w.path.ends_with("tendonPoints[id=t1].z"))));
    let slab = document.members.iter().find(|m| m.kind == MemberKind::FlatSlab).expect("flat slab");
    for k in 0..4 {
        entity(&built.scene, &format!("{}:stud-r1-{k}", slab.id));
    }
    assert!(built.handles.iter().any(|h| h.writes.iter().any(|w| w.path.ends_with("studRows[id=r1].spacingFirst"))));
    assert!(built.handles.iter().any(|h| h.writes.iter().any(|w| w.path.ends_with("studRows[id=r1].count"))));
    assert!(built.diagrams.iter().any(|g| g.id.starts_with("moment-curvature:")));
}

#[test]
fn explicit_bars_replace_the_layer_distribution_and_the_tension_area() {
    let document = geometry_document();
    let beam = document.members.iter().find(|m| m.kind == MemberKind::Beam).expect("beam");
    let layers = reinforcement::bar_layout(beam, beam.width);
    assert_eq!(layers[0].points.len(), 3);
    assert!(layers[0].explicit && layers[0].diameters.iter().all(|d| (*d - 0.016).abs() < 1e-12));
    close(beam.a_s_tension(), 3.0 * std::f64::consts::PI * 0.008_f64.powi(2) + beam.longitudinal.iter().skip(1).filter(|l| l.position == semio_s_artifact_norm_en1992_model::BarPosition::Bottom || l.position == semio_s_artifact_norm_en1992_model::BarPosition::Tension).map(|l| l.count as f64 * std::f64::consts::PI * (l.diameter * 0.5).powi(2)).sum::<f64>(), 1e-12);
    close(beam.tendon_eccentricity_mid().expect("eccentricity"), 0.1, 1e-12);
}

#[test]
fn the_moment_curvature_curve_rises_to_the_section_resistance_and_cracks_first() {
    let document = En1992Snapshot::compliant_office_frame();
    let beam = document.members.iter().find(|m| m.kind == MemberKind::Beam).expect("beam");
    let sec = mechanics::member_sec(&document, beam).expect("section");
    let curve = sec.moment_curvature(0.0, 48, false);
    assert!(curve.len() > 10 && curve.windows(2).all(|w| w[1].0 > w[0].0));
    let peak = curve.iter().map(|p| p.1).fold(0.0, f64::max);
    let (m_cr, k_cr) = sec.cracking(0.0);
    assert!(m_cr > 0.0 && k_cr > 0.0 && peak > m_cr);
    close(peak, sec.m_rd_y(0.0).expect("capacity").0, 0.08);
}

#[test]
fn the_golden_scene_matches_the_committed_bytes() {
    let (built, _) = built(&En1992Snapshot::compliant_office_frame(), &[]);
    assert_scene_laws(&built.scene);
    assert_diagram_laws(&built.diagrams);
    assert_handle_laws(&built.handles);
    let actual = canonical_json(&built.scene);
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🧪️tests/🧫️fixtures/🏛️scene/🔣️.json");
    if std::env::var("SEMIO_UPDATE_GOLDEN").is_ok() {
        std::fs::write(&path, &actual).expect("bless golden");
    }
    assert_eq!(actual, std::fs::read_to_string(&path).expect("golden"), "golden differs; rerun with SEMIO_UPDATE_GOLDEN=1 to update");
}

#[test]
fn entity_dependencies_name_the_grades_a_member_reads() {
    let document = En1992Snapshot::compliant_office_frame();
    let dependencies = entity_dependencies(&document, "beam-B1:bar-bot-0");
    assert_eq!(dependencies, vec!["b500".to_string(), "c30".to_string()]);
}

#[test]
fn check_stations_drive_the_member_binding_the_cracks_and_the_deflected_shape() {
    let document = En1992Snapshot::compliant_office_frame();
    let (built, report) = built(&document, &[]);
    assert_scene_laws(&built.scene);
    assert_diagram_laws(&built.diagrams);
    let beam = document.members.iter().find(|m| m.id == "beam-B1").expect("beam");
    let flexure = report.checks.iter().find(|c| c.id == format!("en1992.6.1.flexure.{}", beam.id)).expect("flexure");
    assert_eq!(flexure.stations.len(), 11);
    let concrete = entity(&built.scene, &beam.id);
    let stations = concrete.utilisation.as_ref().and_then(|u| u.per_station.as_ref()).expect("per station");
    assert_eq!(stations.len(), 11);
    let envelope = stations.iter().map(|s| s.value).fold(0.0, f64::max);
    assert!(envelope >= flexure.utilization - 1e-9);
    assert!(built.scene.entities.iter().filter(|e| e.entity.id.starts_with(&format!("{}:crack-", beam.id))).count() <= 40);
}

const ORACLE_CREEP: [(f64, f64); 2] = [(365.0, 1.805_093_045_593_872_7), (18_250.0, 2.369_266_541_894_601)];
const ORACLE_SHRINKAGE: [(f64, f64); 2] = [(365.0, 0.000_375_264_706_058_797), (18_250.0, 0.000_466_602_330_444_620_44)];

fn field_ids(scene: &scene::StructuralScene) -> BTreeSet<String> {
    scene.entities.iter().filter_map(|e| match &e.appearance.channel { scene::ColourChannel::Fields { fields, .. } => Some(fields.iter().map(|f| f.id.clone())), _ => None }).flatten().collect()
}

#[test]
fn creep_and_shrinkage_follow_the_annex_b_closed_forms() {
    let h0 = tracks::notional_size(0.30, 0.50);
    close(h0, 187.5, 1e-12);
    for (days, expected) in ORACLE_CREEP {
        close(tracks::creep_phi(38.0, 50.0, h0, 28.0, days), expected, 1e-12);
    }
    for (days, expected) in ORACLE_SHRINKAGE {
        close(tracks::shrinkage_strain(30.0, 50.0, h0, days, 3.0), expected, 1e-12);
    }
    assert_eq!(tracks::creep_phi(38.0, 50.0, h0, 28.0, 28.0), 0.0);
}

#[test]
fn every_track_validates_and_binds_fields_that_exist_on_entities() {
    for document in fixtures() {
        let (built, _) = built(&document, &[]);
        assert_scene_laws(&built.scene);
        assert_diagram_laws(&built.diagrams);
        assert_handle_laws(&built.handles);
        assert_eq!(built.scene.validate(), Ok(()), "{}", document.title);
        let ids = field_ids(&built.scene);
        for track in &built.scene.tracks {
            assert!(track.keyframes.iter().all(|k| k.field.as_ref().is_some_and(|f| ids.contains(f))), "{} / {}", document.title, track.id);
        }
        let has_fire = document.members.iter().any(|m| m.fire.is_some());
        assert_eq!(built.scene.tracks.iter().any(|t| t.id == "fire"), has_fire, "{}", document.title);
    }
}

#[test]
fn the_office_frame_offers_creep_and_shrinkage_over_the_concrete_age() {
    let (built, _) = built(&En1992Snapshot::compliant_office_frame(), &[]);
    assert_scene_laws(&built.scene);
    assert_diagram_laws(&built.diagrams);
    assert_handle_laws(&built.handles);
    let ids: Vec<_> = built.scene.tracks.iter().map(|t| (t.id.as_str(), t.kind)).collect();
    assert!(ids.contains(&("creep", scene::TrackKind::Age)) && ids.contains(&("shrinkage", scene::TrackKind::Age)));
    let creep = built.scene.tracks.iter().find(|t| t.id == "creep").expect("creep track");
    close(creep.max, 18_250.0 * 86_400.0, 1e-12);
    let beam = built.scene.entities.iter().find(|e| e.entity.id.ends_with(":age-field")).expect("age field entity");
    let scene::ColourChannel::Fields { fields, .. } = &beam.appearance.channel else { panic!("fields") };
    let field = fields.iter().find(|f| f.id == "creep@365").expect("creep@365");
    let scene::FieldSamples::Vertex { values } = &field.samples else { panic!("vertex") };
    close(values[0], tracks::creep_phi(38.0, 50.0, tracks::notional_size(0.30, 0.50), 28.0, 365.0), 1e-9);
}

#[test]
fn the_outline_solver_reproduces_the_rectangle_and_advances_less_into_a_thick_web() {
    let rect = checks::fire_field::solve(0.4, 0.4, (true, true, true, true), 60.0, checks::fire_field::MOISTURE_DEFAULT);
    let outline = [(-0.2, -0.2), (0.2, -0.2), (0.2, 0.2), (-0.2, 0.2)];
    let same = checks::fire_field::solve_outline(&outline, (true, true, true, true), 60.0, checks::fire_field::MOISTURE_DEFAULT);
    close(same.ring_depth(checks::fire_field::LEVEL_500, 72), rect.isotherm_depth(checks::fire_field::LEVEL_500), 0.15);
    let tee = [(-0.15, -0.38), (0.15, -0.38), (0.15, 0.07), (0.5, 0.07), (0.5, 0.22), (-0.5, 0.22), (-0.5, 0.07), (-0.15, 0.07)];
    let field = checks::fire_field::solve_outline(&tee, (true, true, true, false), 60.0, checks::fire_field::MOISTURE_DEFAULT);
    let depth = field.ring_depth(checks::fire_field::LEVEL_500, 72);
    assert!(depth > 0.01 && depth < 0.12, "{depth}");
    assert!(field.isotherm_ring(checks::fire_field::LEVEL_500, 72).len() == 72);
}

#[test]
fn a_tee_section_with_a_fire_rating_gets_an_isotherm_surface_along_the_member() {
    let mut document = En1992Snapshot::compliant_office_frame();
    let beam = document.members.iter_mut().find(|m| m.kind == MemberKind::Beam).expect("beam");
    beam.section_points = [(-0.15, -0.38), (0.15, -0.38), (0.15, 0.07), (0.5, 0.07), (0.5, 0.22), (-0.5, 0.22), (-0.5, 0.07), (-0.15, 0.07)].iter().enumerate().map(|(i, p)| model::SectionPoint { id: format!("p{i}"), y: p.0, z: p.1 }).collect();
    let id = beam.id.clone();
    let (built, _) = built(&document, &[]);
    assert_scene_laws(&built.scene);
    assert_diagram_laws(&built.diagrams);
    assert_handle_laws(&built.handles);
    assert_eq!(built.scene.validate(), Ok(()));
    if document.members.iter().find(|m| m.id == id).and_then(|m| m.fire.as_ref()).is_some() {
        entity(&built.scene, &format!("{id}:isotherm-surface"));
    }
}

#[test]
fn the_crack_pattern_places_lines_at_the_crack_spacing_coded_by_the_check_widths() {
    use super::crack_pattern::{crack_positions, pattern, width_class, WidthClass};
    let positions = crack_positions(1.0, 2.0, 0.3, 40);
    assert_eq!(positions.len(), 3);
    close(positions[0], 1.05 + 0.15, 1e-12);
    close(positions[1] - positions[0], 0.3, 1e-12);
    assert!(crack_positions(1.0, 1.2, 0.3, 40).is_empty());
    assert_eq!(crack_positions(0.0, 100.0, 0.3, 40).len(), 40);
    assert_eq!(width_class(0.1e-3, 0.3e-3), WidthClass::Fine);
    assert_eq!(width_class(0.2e-3, 0.3e-3), WidthClass::Near);
    assert_eq!(width_class(0.4e-3, 0.3e-3), WidthClass::Exceeding);
    let document = En1992Snapshot::compliant_office_frame();
    let (built, report) = built(&document, &[]);
    assert_scene_laws(&built.scene);
    assert_diagram_laws(&built.diagrams);
    let beam = document.members.iter().find(|m| m.id == "beam-B1").expect("beam");
    let sec = mechanics::member_sec(&document, beam).expect("section");
    let input = SceneInput { document: &document, report: &report, placements: &[], revision: revision() };
    let (cracks, sr_max, w_max, _) = pattern(&input, beam, &sec).expect("cracked beam");
    close(sr_max, ORACLE_SR_MAX_B1, 1e-6);
    assert!(!cracks.is_empty() && cracks.windows(2).all(|w| (w[1].x - w[0].x - sr_max).abs() < 1e-9));
    assert!(cracks.iter().all(|c| c.wk <= w_max * 1.0001 || c.class == WidthClass::Exceeding));
    let diagram = built.diagrams.iter().find(|d| d.id == "crack-pattern:beam-B1").expect("crack pattern diagram");
    assert_eq!(diagram.table_twin.columns.len(), 5);
    assert_eq!(diagram.table_twin.columns[0].values.len(), cracks.len());
    assert_eq!(diagram.validate(), Ok(()));
}

fn fold_into(document: &mut En1992Snapshot, commit: &scene::projection_kit::PlacementCommit) {
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
    use scene::projection_kit::{PlacementCommit, PlacementMove};
    let mut document = En1992Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
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
fn the_crack_triangles_are_toned_by_their_own_width_against_the_w_max_ramp() {
    use super::crack_pattern::pattern;
    let document = En1992Snapshot::compliant_office_frame();
    let (built, report) = built(&document, &[]);
    assert_scene_laws(&built.scene);
    assert_diagram_laws(&built.diagrams);
    let beam = document.members.iter().find(|m| m.id == "beam-B1").expect("beam");
    let sec = mechanics::member_sec(&document, beam).expect("section");
    let input = SceneInput { document: &document, report: &report, placements: &[], revision: revision() };
    let (cracks, _, w_max, _) = pattern(&input, beam, &sec).expect("cracked beam");
    let triangles: Vec<_> = built.scene.entities.iter().filter(|e| e.entity.id.starts_with("beam-B1:crack-")).collect();
    assert!(!triangles.is_empty());
    for e in &triangles {
        let ratio = e.utilisation.as_ref().expect("bound").ratio;
        assert!(cracks.iter().any(|c| (c.wk / w_max - ratio).abs() < 1e-6 || ratio == 0.0), "{}: {ratio}", e.entity.id);
        assert!(matches!(e.appearance.channel, scene::ColourChannel::Utilisation));
    }
    let binding = scene::projection_kit::binding_at_ratio(triangles[0].utilisation.as_ref().unwrap(), 1.2);
    assert_eq!((binding.ratio, binding.status), (1.2, scene::CheckStatus::Fail));
    assert_eq!(scene::projection_kit::binding_at_ratio(&binding, 0.5).status, scene::CheckStatus::Pass);
    assert_eq!(scene::projection_kit::binding_at_ratio(&binding, 0.95).status, scene::CheckStatus::Warning);
}

#[test]
fn the_flat_slab_example_shows_perimeters_stud_rows_utilisation_colours_and_bound_stud_grips() {
    let document = En1992Snapshot::flat_slab_punching();
    let (built, report) = built(&document, &[]);
    assert_scene_laws(&built.scene);
    assert_diagram_laws(&built.diagrams);
    assert_handle_laws(&built.handles);
    assert_eq!(built.scene.validate(), Ok(()));
    for member in &document.members {
        assert!(matches!(entity(&built.scene, &format!("{}:column", member.id)).geometry, scene::Geometry::Solid(_)));
        entity(&built.scene, &format!("{}:punching-u0", member.id));
        let u1 = entity(&built.scene, &format!("{}:punching-u1", member.id));
        assert!(matches!(u1.appearance.channel, scene::ColourChannel::Fields { .. }), "{} u1 carries the perimeter utilisation field", member.id);
        assert_eq!(u1.utilisation.as_ref().map(|b| b.check_id.as_str()), Some(format!("en1992.6.4.punching.{}", member.id).as_str()));
        let studs: Vec<_> = built.scene.entities.iter().filter(|e| e.entity.id.starts_with(&format!("{}:stud-", member.id))).collect();
        assert_eq!(studs.len() as u32, member.stud_rows.iter().map(|r| r.count).sum::<u32>(), "{} one fastener per stud", member.id);
        for ring in &member.stud_rows {
            let axes: Vec<scene::Vec3> = (0..ring.count).map(|k| match &entity(&built.scene, &format!("{}:stud-{}-{k}", member.id, ring.id)).geometry {
                scene::Geometry::Fastener(g) => g.axis.from.clone(),
                _ => panic!("a stud is a fastener"),
            }).collect();
            for (i, a) in axes.iter().enumerate() {
                assert!(axes[i + 1..].iter().all(|b| a.distance(b) > 0.1), "{} row {} studs overlap", member.id, ring.id);
            }
            for leaf in ["count", "spacingFirst"] {
                let path = format!("members[id={}].studRows[id={}].{leaf}", member.id, ring.id);
                let grip = built.handles.iter().find(|h| h.writes.iter().any(|w| w.path == path)).unwrap_or_else(|| panic!("no grip writes {path}"));
                let expected = if leaf == "count" { f64::from(ring.count) } else { ring.spacing_first };
                assert!(grip.writes.iter().any(|w| w.path == path && (w.map.base - expected).abs() < 1e-9), "{path} is bound to the document value");
                assert!(studs.iter().any(|e| e.handles.contains(&grip.id)), "{path} grip is attached to a stud");
            }
        }
    }
    let studs_id = "en1992.9.4.3.studs.slab-FS-corner";
    assert_eq!(report.checks.iter().find(|c| c.id == studs_id).map(|c| c.status), Some(semio_s_plugin_norm_compliance::CheckStatus::Fail));
    let corner = entity(&built.scene, "slab-FS-corner:stud-r1-0");
    assert_eq!(corner.utilisation.as_ref().map(|b| (b.check_id.as_str(), b.status)), Some((studs_id, scene::CheckStatus::Fail)));
    let interior = entity(&built.scene, "slab-FS-interior:stud-r1-0");
    assert!(interior.utilisation.as_ref().is_some_and(|b| b.status != scene::CheckStatus::Fail));
    assert!(report.checks.iter().filter(|c| c.id.starts_with("en1992.6.4.punching.")).all(|c| c.status != semio_s_plugin_norm_compliance::CheckStatus::Fail));
}

#[test]
fn unbalanced_column_moments_show_a_moment_glyph_per_direction_and_grips_for_the_method_and_both_moments() {
    let mut document = En1992Snapshot::flat_slab_punching();
    for member in &mut document.members {
        let punching = member.punching.as_mut().expect("punching");
        punching.beta_method = semio_s_artifact_norm_en1992_model::PunchingBetaMethod::UnbalancedMoments;
        punching.moment_c1 = 40.0e3;
        punching.moment_c2 = 0.0;
    }
    let (built, _) = built(&document, &[]);
    assert_scene_laws(&built.scene);
    assert_handle_laws(&built.handles);
    assert_eq!(built.scene.validate(), Ok(()));
    for member in &document.members {
        let glyph = entity(&built.scene, &format!("{}:punching-moment-c1", member.id));
        assert!(matches!(&glyph.geometry, scene::Geometry::Load(scene::LoadGlyph::Moment { magnitude, .. }) if (magnitude.value - 40.0e3).abs() < 1e-9));
        assert!(built.scene.entities.iter().all(|e| e.entity.id != format!("{}:punching-moment-c2", member.id)), "a zero moment draws no glyph");
        for (leaf, expected) in [("momentC1", 40.0e3), ("momentC2", 0.0)] {
            let path = format!("members[id={}].punching.{leaf}", member.id);
            let grip = built.handles.iter().find(|h| h.writes.iter().any(|w| w.path == path)).unwrap_or_else(|| panic!("no grip writes {path}"));
            assert!(grip.writes.iter().any(|w| w.path == path && (w.map.base - expected).abs() < 1e-9), "{path} is bound to the document value");
            assert!(entity(&built.scene, &format!("{}:column", member.id)).handles.contains(&grip.id), "{path} grip is attached to the column");
        }
        let path = format!("members[id={}].punching.betaMethod", member.id);
        assert!(built.handles.iter().any(|h| matches!(h.kind, scene::HandleKind::ChoiceStepper) && h.writes.iter().any(|w| w.path == path)), "{path} has a choice stepper");
    }
    let simplified = En1992Snapshot::flat_slab_punching();
    let (plain, _) = built(&simplified, &[]);
    assert!(plain.handles.iter().any(|h| h.writes.iter().any(|w| w.path.ends_with("punching.betaMethod"))));
    assert!(plain.handles.iter().all(|h| h.writes.iter().all(|w| !w.path.contains("punching.moment"))), "the moments are only editable with the method from unbalanced moments");
}

#[test]
fn members_carry_a_hidden_grade_field_with_one_legend_per_grade_table() {
    let (built, _) = built(&En1992Snapshot::compliant_office_frame(), &[]);
    assert_scene_laws(&built.scene);
    let field = entity(&built.scene, "beam-B1:grade-field");
    let scene::ColourChannel::Fields { active, fields } = &field.appearance.channel else { panic!("grade field channel") };
    assert_eq!(active, grades::CONCRETE_FIELD);
    let value = |id: &str| match &fields.iter().find(|f| f.id == id).expect("field").samples {
        scene::FieldSamples::Vertex { values } => values[0],
        _ => panic!("vertex samples"),
    };
    close(value(grades::CONCRETE_FIELD), 30.0, 1e-9);
    close(value(grades::REINFORCEMENT_FIELD), 500.0, 1e-9);
    let layer = built.scene.layers.iter().find(|l| l.id == "grades").expect("grades layer");
    assert!(!layer.visible);
    for id in [grades::CONCRETE_FIELD, grades::REINFORCEMENT_FIELD] {
        let legend = built.scene.legends.iter().find(|l| l.id == id).expect("legend");
        assert!(matches!(&legend.scale, scene::LegendScale::Scalar { field_id, map, .. } if field_id == id && map.max > map.min));
    }
}

#[test]
fn members_and_anchors_advertise_move_and_rotate_grips() {
    let document = En1992Snapshot::compliant_office_frame();
    let (built, _) = built(&document, &[]);
    assert!(entity(&built.scene, "beam-B1").handles.iter().all(|h| built.handles.iter().any(|x| &x.id == h)));
    for tag in ["move-x", "move-y", "move-yaw"] {
        let grip = built.handles.iter().find(|h| h.id == format!("beam-B1:{tag}")).expect("move grip");
        assert!(grip.writes[0].path.starts_with("placements[id=beam-B1]."));
    }
    scene::projection_kit::test_support::assert_angular_handle_laws(&built.handles);
}

#[test]
fn the_flat_slab_example_draws_the_outer_perimeter_at_the_distance_the_check_demands() {
    let document = En1992Snapshot::flat_slab_punching();
    let (built, report) = built(&document, &[]);
    for member in &document.members {
        let check = report.checks.iter().find(|c| c.id == format!("en1992.6.4.uout.{}", member.id)).expect("outer perimeter check");
        let outer = member.stud_reach().expect("stud reach");
        let a_out = check.utilization * (outer + 1.5 * member.effective_depth);
        let ring = entity(&built.scene, &format!("{}:punching-u-out", member.id));
        assert!(matches!(ring.geometry, scene::Geometry::Surface(_)), "{}: u_out is a drawn perimeter", member.id);
        assert!(a_out > 0.0 && check.utilization < 1.0, "{}: u_out {a_out} lies within the stud reach", member.id);
    }
}

#[test]
fn openings_become_cut_outs_with_grips_and_cut_the_control_perimeter_between_the_tangents() {
    let mut document = En1992Snapshot::flat_slab_punching();
    let interior = document.members.iter_mut().find(|m| m.id == "slab-FS-interior").expect("interior slab");
    interior.openings = vec![model::Opening { id: "op-1".into(), x: 1.0, y: 0.0, width: 0.2, depth: 0.2 }, model::Opening { id: "op-far".into(), x: 2.5, y: 0.0, width: 0.2, depth: 0.2 }];
    let d = interior.effective_depth;
    let (c1, c2) = interior.punching.as_ref().map(|p| (p.column_width, p.column_depth)).expect("punching");
    let (built, report) = built(&document, &[]);
    assert_scene_laws(&built.scene);
    assert_handle_laws(&built.handles);
    assert_eq!(built.scene.validate(), Ok(()));
    for id in ["op-1", "op-far"] {
        assert!(matches!(entity(&built.scene, &format!("slab-FS-interior:opening-{id}")).geometry, scene::Geometry::Solid(_)));
        for leaf in ["x", "y", "width", "depth"] {
            let path = format!("members[id=slab-FS-interior].openings[id={id}].{leaf}");
            assert!(built.handles.iter().any(|h| h.writes.iter().any(|w| w.path == path)), "no grip writes {path}");
        }
    }
    let u1 = entity(&built.scene, "slab-FS-interior:punching-u1");
    let full = 2.0 * (c1 + c2) + 4.0 * std::f64::consts::PI * d;
    let lost = 2.0 * (c1 / 2.0 + 2.0 * d) / 9.0;
    assert!(u1.accessible_detail.en.contains(&format!("length {:.2} m", full - lost)), "{}", u1.accessible_detail.en);
    let punching = report.checks.iter().find(|c| c.id == "en1992.6.4.punching.slab-FS-interior").expect("punching check");
    assert!(punching.explanation.en.contains("between the opening tangents"));
}

#[test]
fn the_grade_legend_and_the_anchor_and_placement_diagrams_project_the_document() {
    let mut document = En1992Snapshot::compliant_office_frame();
    document.placements = vec![model::Placement { id: "anc-1".into(), x: 2.0, y: 1.0, z: 0.0, yaw: 0.5 }];
    let (built, _) = built(&document, &[]);
    assert_diagram_laws(&built.diagrams);
    let grades = built.diagrams.iter().find(|d| d.id == "grades:document").expect("grade legend");
    let total = document.concrete_grades.len() + document.reinforcement_grades.len() + document.prestress_steels.len();
    assert_eq!(grades.table_twin.rows(), total);
    let first = &document.concrete_grades[0];
    let marker = grades.markers.iter().find(|m| m.id == format!("reference:tip-concreteGrades-{}", first.id)).expect("f_ck marker");
    let binding = marker.binding.as_ref().expect("binding");
    assert_eq!(binding.path, format!("concreteGrades[id={}].fCk", first.id));
    close(binding.map.apply(first.f_ck / 1.0e6), first.f_ck, 1.0);
    let used = document.members.iter().filter(|m| m.concrete_grade_id == first.id).count() as f64;
    close(grades.table_twin.columns[2].values[0], used, 0.0);
    assert!(built.diagrams.iter().any(|d| d.focus.as_ref().is_some_and(|f| f.id == "anc-1")), "an anchor diagram");
    let plan = built.diagrams.iter().find(|d| d.id == "placements:document").expect("placement plan");
    assert_eq!(plan.table_twin.columns[0].values, vec![2.0]);
    assert_eq!(plan.markers[0].binding.as_ref().map(|b| b.path.as_str()), Some("placements[id=anc-1].x"));
    let (none, _) = built(&En1992Snapshot::compliant_office_frame(), &[]);
    assert!(none.diagrams.iter().all(|d| d.id != "placements:document"));
}

#[test]
fn every_section_point_of_an_outline_has_a_grip_per_axis_on_the_member() {
    let mut document = En1992Snapshot::compliant_office_frame();
    let beam = document.members.iter_mut().find(|m| m.kind == MemberKind::Beam).expect("beam");
    beam.section_points = [(-0.15, -0.38), (0.15, -0.38), (0.15, 0.07), (0.5, 0.07), (0.5, 0.22), (-0.5, 0.22), (-0.5, 0.07), (-0.15, 0.07)].iter().enumerate().map(|(i, p)| model::SectionPoint { id: format!("p{i}"), y: p.0, z: p.1 }).collect();
    let id = beam.id.clone();
    let (built, _) = built(&document, &[]);
    assert_handle_laws(&built.handles);
    for axis in ["y", "z"] {
        let grip = built.handles.iter().find(|h| h.id == format!("{id}/outline-p3-{axis}")).unwrap_or_else(|| panic!("no outline grip {axis}"));
        assert_eq!(grip.writes[0].path, format!("members[id={id}].sectionPoints[id=p3].{axis}"));
        assert!(entity(&built.scene, &id).handles.contains(&grip.id), "the member advertises the outline grip");
    }
    close(built.handles.iter().find(|h| h.id == format!("{id}/outline-p3-y")).expect("grip").writes[0].map.base, 0.5, 1e-12);
    let rectangle = En1992Snapshot::compliant_office_frame();
    let (plain, _) = built(&rectangle, &[]);
    assert!(plain.handles.iter().all(|h| !h.id.contains("/outline-")), "a rectangle has no outline grips");
}

#[test]
fn the_palette_offers_the_grade_tables() {
    for (id, collection) in [("concreteGrade", "concreteGrades"), ("reinforcementGrade", "reinforcementGrades"), ("prestressSteel", "prestressSteels")] {
        assert!(ENTITY_KINDS.iter().any(|k| k.id == id && k.collection == collection), "{id} is pickable");
    }
}

#[test]
fn placements_are_a_registered_kind_and_openings_and_stud_rows_are_scene_entities_of_their_member() {
    let hit = ENTITY_KINDS.iter().find(|k| k.id == "placement").expect("placement kind");
    assert_eq!((hit.collection, hit.label_en, hit.label_de), ("placements", "Placement", "Platzierung"));
    let mut document = En1992Snapshot::flat_slab_punching();
    let interior = document.members.iter_mut().find(|m| m.id == "slab-FS-interior").expect("interior slab");
    interior.openings = vec![model::Opening { id: "op-1".into(), x: 1.0, y: 0.0, width: 0.2, depth: 0.2 }];
    let (built, _) = built(&document, &[]);
    let opening = entity(&built.scene, "slab-FS-interior:opening-op-1");
    assert_eq!(opening.entity.path, "members[id=slab-FS-interior].openings[id=op-1]");
    assert!(!opening.handles.is_empty() && opening.handles.iter().all(|h| built.handles.iter().any(|g| &g.id == h)), "the opening is pickable with its grips");
    let studs: Vec<_> = built.scene.entities.iter().filter(|e| e.entity.path.contains(".studRows[id=")).collect();
    assert!(!studs.is_empty() && studs.iter().any(|e| !e.handles.is_empty()), "stud rows are pickable with grips");
    assert!(entities_of(&document).iter().any(|e| e.id == "slab-FS-interior"), "their member is the registry owner");
}

#[test]
fn the_basis_badge_follows_the_document_annex_in_the_scene_and_every_diagram() {
    scene::projection_kit::test_support::assert_basis_follows_annex(norm::NormFamilyId::En1992, |annex| {
        let mut document = En1992Snapshot::compliant_office_frame();
        document.annex = annex;
        let (built, report) = built(&document, &[]);
        (built.scene.clone(), diagrams(&document, &report, None))
    });
}
