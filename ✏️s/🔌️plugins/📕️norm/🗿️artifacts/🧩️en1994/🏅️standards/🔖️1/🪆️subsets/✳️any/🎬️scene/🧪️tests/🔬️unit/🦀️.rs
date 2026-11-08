//! 🧪️ Scene laws and golden: validity, determinism, inventory per member kind, stud and rib geometry against independent oracles, tessellation, handles, diagrams and bridge, continuous and failing variants.

use super::*;
use scene::projection_kit::test_support::{revision, assert_handle_laws};
use model::{CompositeBeam, En1994Snapshot, Stud};
use scene::diagram_kit::curves;
use scene::tessellation::{extrude, fasteners, shells, Lod, Tolerance};
use scene::{canonical_json, CheckStatus, Geometry, Validate};

const GOLDEN_BUILDING: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧫️fixtures/🏛️scene/🔣️.json");
const GOLDEN_BRIDGE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧫️fixtures/🌉️bridge/🔣️.json");
const GOLDEN_SHOWCASE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json");
const ORACLE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🔮️oracles/🐍️.py");

fn project_of(document: &En1994Snapshot) -> (StructuralScene, CheckReport) {
    let report = semio_s_artifact_norm_en1994_inferences::evaluate(document);
    let scene = project(&SceneInput { document, report: &report, placements: &[], revision: revision() });
    (scene, report)
}

fn ids(scene: &StructuralScene) -> Vec<&str> {
    scene.entities.iter().map(|e| e.entity.id.as_str()).collect()
}

fn continuous() -> En1994Snapshot {
    let mut d = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    d.beams[0].support = semio_s_artifact_norm_en1994_model::SupportCondition::Continuous2Span;
    d.slabs[0].support = semio_s_artifact_norm_en1994_model::SupportCondition::Continuous2Span;
    d
}

fn column_kinds() -> En1994Snapshot {
    let mut d = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let base = d.columns[0].clone();
    d.columns.clear();
    for (id, kind, area) in [("col-chs", semio_s_artifact_norm_en1994_model::ColumnKind::ConcreteFilled, 0.0084), ("col-rhs", semio_s_artifact_norm_en1994_model::ColumnKind::ConcreteFilled, 0.0114), ("col-enc", semio_s_artifact_norm_en1994_model::ColumnKind::Encased, 0.0149), ("col-par", semio_s_artifact_norm_en1994_model::ColumnKind::PartiallyEncased, 0.0149)] {
        let mut c = base.clone();
        c.id = id.into();
        c.kind = kind;
        c.steel_a_m2 = area;
        d.columns.push(c);
    }
    d
}

fn failing() -> En1994Snapshot {
    let mut d = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    d.beams[0].span_m = 16.0;
    d.studs = Stud::typical_row("beam-B1", 8.0, 2.0, 4);
    d.columns[0].actions[0].n_k_n = 9.0e6;
    d.slabs[0].span_m = 6.0;
    d
}

fn validated(document: &En1994Snapshot) -> StructuralScene {
    let (scene, _) = project_of(document);
    if let Err(violations) = scene.validate() {
        panic!("scene violations: {violations:?}");
    }
    scene
}

#[test]
fn every_variant_projects_a_valid_scene() {
    for document in [En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), En1994Snapshot::unpropped_building(), En1994Snapshot::custom_plate_building(), En1994Snapshot::bridge_girder(), En1994Snapshot::fire_demanding(), continuous(), column_kinds(), failing(), En1994Snapshot::full_showcase()] {
        let scene = validated(&document);
        assert!(!scene.entities.is_empty());
        assert!(scene.bounds.min.x <= scene.bounds.max.x);
    }
}

#[test]
fn projection_is_deterministic_and_reacts_to_edits() {
    let document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let (a, _) = project_of(&document);
    let (b, _) = project_of(&document);
    assert_eq!(canonical_json(&a), canonical_json(&b));
    let mut longer = document.clone();
    longer.beams[0].span_m += 1.0;
    let (c, _) = project_of(&longer);
    let hash = |s: &StructuralScene, id: &str| s.entities.iter().find(|e| e.entity.id == id).map(|e| e.content_hash.clone());
    assert_ne!(hash(&a, "beam-B1"), hash(&c, "beam-B1"));
    assert_eq!(hash(&a, "col-C1"), hash(&c, "col-C1"));
    assert_eq!(hash(&a, "slab-S1"), hash(&c, "slab-S1"));
}

#[test]
fn building_inventory_names_every_spatial_part() {
    let scene = validated(&En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De));
    let present = ids(&scene);
    for expected in ["beam-B1", "beam-B1:slab", "beam-B1:beff", "beam-B1:sheeting", "beam-B1:pna", "beam-B1:rebar-transverse", "beam-B1:support-a", "beam-B1:support-b", "col-C1", "col-C1:concrete", "col-C1:support-a", "slab-S1", "slab-S1:sheeting", "slab-S1:rebar"] {
        assert!(present.contains(&expected), "missing {expected} in {present:?}");
    }
    let studs = present.iter().filter(|i| i.starts_with("beam-B1:stud-")).count();
    assert_eq!(studs, En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De).studs.len());
    assert!(scene.layers.iter().any(|l| l.id == "connectors") && scene.layers.iter().any(|l| l.id == "analysis"));
}

#[test]
fn continuous_beams_add_the_internal_support_cracked_zone_and_hogging_bars() {
    let scene = validated(&continuous());
    let present = ids(&scene);
    for expected in ["beam-B1:support-m", "beam-B1:cracked", "beam-B1:rebar-hogging", "slab-S1:support-m"] {
        assert!(present.contains(&expected), "missing {expected}");
    }
    let beam = scene.entities.iter().find(|e| e.entity.id == "beam-B1").unwrap();
    let Geometry::Member(m) = &beam.geometry else { panic!("beam is a member") };
    let length = match &m.axis {
        scene::Path3::Polyline(p) => p.points[0].distance(&p.points[1]),
        _ => panic!("polyline axis"),
    };
    assert!((length - 16.0).abs() < 1e-9);
}

#[test]
fn bridge_girders_are_named_as_bridge_girders() {
    let scene = validated(&En1994Snapshot::bridge_girder());
    let beam = scene.entities.iter().find(|e| e.entity.id == "beam-B1").unwrap();
    assert!(beam.accessible_name.en.starts_with("Bridge girder") && beam.accessible_name.de.starts_with("Brückenträger"));
    assert!(scene.description.en.contains("bridge") && scene.description.de.contains("Brücke"));
}

#[test]
fn unpropped_construction_shows_lateral_restraints() {
    let scene = validated(&En1994Snapshot::unpropped_building());
    assert!(ids(&scene).contains(&"beam-B1:restraint-1"));
}

#[test]
fn all_column_kinds_have_steel_and_concrete_bodies_with_rebar() {
    let scene = validated(&column_kinds());
    for id in ["col-chs", "col-rhs", "col-enc", "col-par"] {
        let concrete = scene.entities.iter().find(|e| e.entity.id == format!("{id}:concrete")).unwrap_or_else(|| panic!("no concrete body of {id}"));
        let Geometry::Member(m) = &concrete.geometry else { panic!("member") };
        let scene::SectionShape::Composite { rebar, .. } = &m.section else { panic!("composite section") };
        assert!(!rebar[0].positions.is_empty());
        let steel = scene.entities.iter().find(|e| e.entity.id == id).unwrap();
        assert!(steel.utilisation.is_some(), "{id} lacks utilisation");
    }
    let shape = |id: &str| facts::ColumnShape::of(column_kinds().columns.iter().find(|c| c.id == id).unwrap());
    assert!(matches!(shape("col-chs"), facts::ColumnShape::Chs { .. }) && matches!(shape("col-rhs"), facts::ColumnShape::Rhs { .. }));
    assert!(matches!(shape("col-enc"), facts::ColumnShape::Encased { .. }) && matches!(shape("col-par"), facts::ColumnShape::Partial { .. }));
}

#[test]
fn every_stud_row_is_drawn_as_its_own_fastener_bound_to_the_per_stud_check() {
    let document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let (scene, _) = project_of(&document);
    let drawn: Vec<_> = scene.entities.iter().filter(|e| e.entity.id.starts_with("beam-B1:stud-")).collect();
    assert_eq!(drawn.len(), document.studs.len());
    for (entity, stud) in drawn.iter().zip(&document.studs) {
        assert_eq!(entity.entity.id, format!("beam-B1:stud-{}", stud.id));
        let Geometry::Fastener(g) = &entity.geometry else { panic!("fastener") };
        assert!(g.pattern.is_none(), "one stud, no pattern");
        assert!((g.diameter - stud.diameter_m).abs() < 1e-12);
        let ratio = entity.utilisation.as_ref().expect("bound").ratio;
        assert!(ratio.is_finite() && ratio >= 0.0, "{ratio}");
    }
}

#[test]
fn studs_at_one_position_spread_over_the_flange_and_share_one_station() {
    let mut document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let twin = Stud { id: "twin".into(), ..document.studs[10].clone() };
    document.studs.push(twin);
    let f = facts::BeamFacts::of(&document, &document.beams[0]);
    let ys = f.stud_ys();
    let (a, b) = (f.studs.rows.iter().position(|r| r.id == "twin").expect("twin"), 10);
    assert_eq!(f.studs.rows[a].station, f.studs.rows[b].station);
    assert_eq!((f.studs.rows[a].n_r, f.studs.rows[b].n_r), (2, 2));
    assert!((ys[a] - ys[b]).abs() > 1e-6, "the two studs of a rib do not coincide: {} {}", ys[a], ys[b]);
    assert_eq!(f.station_xs().len(), document.studs.len() - 1);
    validated(&document);
}

#[test]
fn plastic_neutral_axis_is_in_equilibrium_and_matches_the_oracle() {
    let document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let f = facts::BeamFacts::of(&document, &document.beams[0]);
    assert!(f.pna.residual.abs() < 1e-9 * f.pna.n_a, "residual {}", f.pna.residual);
    let cases: Vec<serde_json::Value> = [0.4, 0.7, 1.0]
        .iter()
        .map(|eta| {
            let p = facts::plastic_neutral_axis(&f.steel, 355e6 / 1.0, 0.85 * 30e6 / 1.5, f.b_eff, f.hc, *eta);
            serde_json::json!({
                "kind": "pna", "h": f.steel.height_m, "b": f.steel.width_m, "tw": f.steel.tw_m, "tf": f.steel.tf_m,
                "fyd": 355e6, "fcd85": 0.85 * 30e6 / 1.5, "bEff": f.b_eff, "hc": f.hc, "eta": eta,
                "xC": p.x_c, "zS": p.z_s, "nC": p.n_c
            })
        })
        .collect();
    oracle(&cases);
}

fn oracle(cases: &[serde_json::Value]) {
    let input = serde_json::to_vec(&serde_json::json!({ "cases": cases })).unwrap();
    let output = norm::python_oracle::run_ok(ORACLE, &[], Some(&input));
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("oracle prints JSON");
    assert_eq!(report["failures"], serde_json::json!([]), "oracle disagreement: {report}");
}

#[test]
fn rib_profile_and_stud_slip_and_curves_match_the_oracle() {
    let sheeting = model::ProfiledSheeting::trapezoidal();
    let line = facts::rib_centreline(&sheeting, 3.0);
    let length: f64 = line.windows(2).map(|w| (w[1].0 - w[0].0).hypot(w[1].1 - w[0].1)).sum();
    let p_rk = 100.0;
    let slip: Vec<serde_json::Value> = [0.5, 1.0, 2.0, 6.0].iter().map(|d| serde_json::json!({ "kind": "slip", "pRk": p_rk, "delta": d, "load": diagrams::stud_load(p_rk, *d), "inverse": diagrams::stud_slip_at(p_rk, diagrams::stud_load(p_rk, *d)) })).collect();
    let shape: Vec<serde_json::Value> = [0.1, 0.25, 0.5, 0.75].iter().map(|u| serde_json::json!({ "kind": "deflection", "u": u, "shape": diagrams::deflection_shape(*u) })).collect();
    let chi: Vec<serde_json::Value> = [0.4, 1.0, 2.0].iter().map(|l| serde_json::json!({ "kind": "buckling", "lambda": l, "alpha": 0.21, "chi": curves::buckling_chi(*l, 0.21) })).collect();
    let (pitch, crest, bottom, e) = facts::rib_profile(&sheeting);
    let mut cases = vec![serde_json::json!({ "kind": "rib", "hp": sheeting.height_m, "length": 3.0, "pitch": pitch, "crest": crest, "bottom": bottom, "web": e, "polyline": length, "points": line.len() })];
    cases.extend(slip);
    cases.extend(shape);
    cases.extend(chi);
    oracle(&cases);
}

#[test]
fn interaction_polygon_area_matches_the_oracle() {
    let document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let (_, report) = project_of(&document);
    let diagram = diagrams(&document, &report, Some("col-C1")).into_iter().find(|d| d.id == "mn:col-C1").expect("mn diagram");
    let table = &diagram.table_twin;
    oracle(&[serde_json::json!({ "kind": "polygon", "n": table.columns[0].values, "m": table.columns[1].values })]);
    let scene::Series::Polygon { ring, .. } = &diagram.series[0] else { panic!("polygon first") };
    let area: f64 = (0..ring.len()).map(|i| ring[i].x * ring[(i + 1) % ring.len()].y - ring[(i + 1) % ring.len()].x * ring[i].y).sum::<f64>() / 2.0;
    assert!(area.abs() > 0.0);
}

fn volume(mesh: &scene::tessellation::MeshData) -> f64 {
    let p = |i: u32| mesh.positions[i as usize].map(f64::from);
    mesh.indices.chunks(3).map(|t| {
        let (a, b, c) = (p(t[0]), p(t[1]), p(t[2]));
        (a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0]) + a[2] * (b[0] * c[1] - b[1] * c[0])) / 6.0
    }).sum::<f64>().abs()
}

#[test]
fn every_entity_tessellates_and_the_slab_volume_is_exact() {
    let document = column_kinds();
    let mut all = document.clone();
    all.beams = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De).beams;
    all.slabs = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De).slabs;
    let scene = validated(&all);
    let tol = Tolerance::default();
    for e in &scene.entities {
        let mesh = match &e.geometry {
            Geometry::Member(g) => Some(extrude::member_mesh(g, tol, Lod::Lod2).unwrap_or_else(|v| panic!("{}: {v:?}", e.entity.id))),
            Geometry::Plate(g) => Some(shells::plate_mesh(g, tol, Lod::Lod2).unwrap_or_else(|v| panic!("{}: {v:?}", e.entity.id))),
            Geometry::Fastener(g) => Some(fasteners::fastener_mesh(g, tol, Lod::Lod2).unwrap_or_else(|v| panic!("{}: {v:?}", e.entity.id))),
            _ => None,
        };
        if let Some(mesh) = mesh {
            assert!(!mesh.indices.is_empty(), "{} has an empty mesh", e.entity.id);
        }
    }
    let slab = scene.entities.iter().find(|e| e.entity.id == "beam-B1:slab").unwrap();
    let Geometry::Plate(g) = &slab.geometry else { panic!("plate") };
    let f = facts::BeamFacts::of(&all, &all.beams[0]);
    let expected = f.l_total * f.slab_width() * f.hc;
    assert!((volume(&shells::plate_mesh(g, tol, Lod::Lod2).unwrap()) - expected).abs() < 1e-6 * expected);
}

#[test]
fn failing_documents_colour_red_and_raise_failure_glyphs() {
    let (scene, _) = project_of(&failing());
    let failing_entities = scene.entities.iter().filter(|e| e.utilisation.as_ref().is_some_and(|u| u.status == CheckStatus::Fail)).count();
    assert!(failing_entities > 0);
    assert!(scene.annotations.iter().any(|a| matches!(a, scene::Annotation::FailureGlyph { .. })));
    assert!(scene.entities.iter().any(|e| e.appearance.hatch.is_some() && e.appearance.style == scene::StyleState::Error));
}

#[test]
fn out_of_range_materials_raise_the_materials_failure_glyph() {
    let mut document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    document.beams[0].concrete_f_ck_pa = 90e6;
    let (scene, report) = project_of(&document);
    let materials = report.checks.iter().find(|c| c.id == "en1994.3.materials.beam-B1").expect("materials check");
    let raised = scene.annotations.iter().any(|a| matches!(a, scene::Annotation::FailureGlyph { id, .. } if id == "beam-B1:failure-materials"));
    assert_eq!(raised, matches!(materials.status, norm::CheckStatus::Fail | norm::CheckStatus::Warning));
}

#[test]
fn deflection_overlay_follows_the_deflection_check() {
    let document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let (scene, report) = project_of(&document);
    let beam = scene.entities.iter().find(|e| e.entity.id == "beam-B1").unwrap();
    let delta = report.checks.iter().find(|c| c.id == "en1994.7.3.1.deflection.beam-B1").map(|c| c.computed.value);
    assert_eq!(beam.appearance.deformation.is_some(), delta.is_some_and(|d| d > 1e-9));
    if let (Some(overlay), Some(d)) = (&beam.appearance.deformation, delta) {
        let mid = overlay.samples.iter().map(|s| s.delta.z.abs()).fold(0.0, f64::max);
        assert!((mid - d).abs() < 1e-9 * d.max(1.0));
    }
}

#[test]
fn handles_are_advertised_resolvable_and_round_trip() {
    let document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let (scene, _) = project_of(&document);
    let mut all = Vec::new();
    for (entity, count) in [("beam-B1", 16 + 1 + 3 * document.studs.len()), ("col-C1", 9), ("slab-S1", 7)] {
        let selection = [scene::EntityRef::new(scene::EntityKind::Member, entity, format!("x[id={entity}]"))];
        let list = handles(&document, &[], &selection);
        assert_handle_laws(&list);
        assert_eq!(list.len(), count, "{entity}");
        let advertised: Vec<&String> = scene.entities.iter().find(|e| e.entity.id == entity).unwrap().handles.iter().collect();
        assert_eq!(advertised.len(), list.len());
        for h in &list {
            assert!(advertised.contains(&&h.id));
            h.kind.validate().and(h.snap.validate()).and(h.writes.iter().try_for_each(|w| w.validate())).unwrap_or_else(|v| panic!("{}: {v:?}", h.id));
        }
        all.extend(list);
    }
    let get = |id: &str| all.iter().find(|h| h.id == id).unwrap();
    assert!((get("beam-B1:span").writes_at(10.0)[0].value - 10.0).abs() < 1e-9);
    assert_eq!(get("beam-B1:slab-thickness").writes_at(0.1234)[0].path, "beams[id=beam-B1].slabThicknessM");
    assert_eq!(get("beam-B1:orientation").writes_at(0.9)[0].value, 1.0);
    assert!((get("beam-B1:section").writes_at(0.405)[0].value - 0.4).abs() < 1e-12);
    let f = facts::BeamFacts::of(&document, &document.beams[0]);
    let beff = get("beam-B1:effective-width");
    let write = beff.writes_at(f.b_eff)[0].clone();
    if f.b_eff_capped {
        assert_eq!(write.path, "beams[id=beam-B1].spacingM");
    } else {
        assert!((write.value - document.beams[0].span_m).abs() < 1e-9, "{write:?}");
    }
    assert!((get("col-C1:size").writes_at(0.15)[0].value - 0.3).abs() < 1e-12);
    assert!((get("col-C1:wall").writes_at(0.14)[0].value - 0.01).abs() < 1e-12);
    assert!((get("beam-B1:adjacent-span").writes_at(12.0)[0].value - 4.0).abs() < 1e-9);
    assert!((get("beam-B1:patch-bearing").writes_at(4.1)[0].value - 0.2).abs() < 1e-9);
    assert_eq!(get("beam-B1:bridge-lanes").writes_at(2.0)[0].path, "beams[id=beam-B1].bridgeLanes");
    assert_eq!(get("col-C1:introduction").writes_at(1.75)[0].path, "columns[id=col-C1].introducedForceN");
    assert_eq!(get("beam-B1:action-Q-office").writes_at(1.0)[0].path, "beams[id=beam-B1].actions[id=Q-office].qAreaPa");
    let depth = f.depth + 0.02;
    assert!((get("beam-B1:action-Q-office").writes_at(depth + 0.1)[0].value - 2.0e3).abs() <= 100.0);
    assert!((get("beam-B1:action-Q-office").writes_at(depth + 0.2)[0].value - 4.0e3).abs() <= 100.0);
    let t = document.slabs[0].concrete_thickness_m;
    let rebar = get("slab-S1:rebar-area");
    assert_eq!(rebar.writes_at(0.0)[0].path, "slabs[id=slab-S1].asM2PerM");
    assert!(rebar.writes_at(0.0)[0].value.abs() < 1e-12);
    assert!((rebar.writes_at(0.5)[0].value - 0.04 * t).abs() < 1e-9);
    assert_eq!(get("slab-S1:action-Q-slab").writes_at(0.3)[0].path, "slabs[id=slab-S1].actions[id=Q-slab].qAreaPa");
    assert_eq!(get("col-C1:action-Q-col").writes_at(4.0)[0].path, "columns[id=col-C1].actions[id=Q-col].nKN");
    assert_eq!(get("col-C1:action-Q-col-m").writes_at(0.1)[0].path, "columns[id=col-C1].actions[id=Q-col].mKNm");
    assert_eq!(get("beam-B1:stud-cross-spacing").writes_at(0.05)[0].path, "beams[id=beam-B1].studTransverseSpacingM");
    let first = &document.studs[0].id;
    assert_eq!(get(&format!("beam-B1:stud-{first}-h")).writes_at(0.1)[0].path, format!("studs[id={first}].heightM"));
    assert_eq!(get(&format!("beam-B1:stud-{first}-x")).writes_at(1.0)[0].path, format!("studs[id={first}].xM"));
    assert!((get(&format!("beam-B1:stud-{first}-x")).writes_at(1.25)[0].value - 1.25).abs() < 1e-12);
    let diameter = get(&format!("beam-B1:stud-{first}-d"));
    assert_eq!(diameter.writes_at(0.0095)[0].path, format!("studs[id={first}].diameterM"));
    assert!((diameter.writes_at(0.0095)[0].value - 0.019).abs() < 1e-9, "the grip sits at the radius: d = 2 s");
}

#[test]
fn a_slab_point_load_draws_an_arrow_and_its_magnitude_grip_writes_the_point_force_kind_leaf() {
    let mut document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    document.slabs[0].actions.push(model::SlabAction { id: "P-1".into(), kind: semio_s_artifact_norm_en1994_model::ActionKind::Imposed, category: "B".into(), stage: semio_s_artifact_norm_en1994_model::ActionStage::Composite, q_area_pa: 0.0, f_k_n: 20.0e3, delta_sigma_k_pa: 0.0, delta_tau_k_pa: 0.0, x_m: 0.0, y_m: 0.0 });
    let scene = validated(&document);
    assert!(ids(&scene).contains(&"slab-S1:load-P-1"));
    let selection = [scene::EntityRef::new(scene::EntityKind::Plate, "slab-S1", "slabs[id=slab-S1]")];
    let list = handles(&document, &[], &selection);
    assert_handle_laws(&list);
    let grip = list.iter().find(|h| h.id == "slab-S1:action-P-1").expect("point load grip");
    let top = document.slabs[0].concrete_thickness_m + 0.02;
    let write = &grip.writes_at(top + 0.1)[0];
    assert_eq!(write.path, "slabs[id=slab-S1].actions[id=P-1].fKN");
    assert!((write.value - 20.0e3).abs() <= 500.0, "{write:?}");
    let along = list.iter().find(|h| h.id == "slab-S1:action-P-1-x").expect("position grip along the strip");
    assert_eq!(along.writes_at(1.25)[0].path, "slabs[id=slab-S1].actions[id=P-1].xM");
    assert!((along.writes_at(1.25)[0].value - 1.25).abs() < 1e-9);
    let across = list.iter().find(|h| h.id == "slab-S1:action-P-1-y").expect("position grip across the strip");
    assert_eq!(across.writes_at(0.25)[0].path, "slabs[id=slab-S1].actions[id=P-1].yM");
}

#[test]
fn a_beam_point_load_stands_at_its_position_and_its_position_grip_writes_the_typed_leaf_in_one_commit() {
    let mut document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    document.beams[0].actions.push(model::CharacteristicAction { x_m: 2.0, ..model::CharacteristicAction::point_force("P-1", semio_s_artifact_norm_en1994_model::ActionStage::Composite, semio_s_artifact_norm_en1994_model::ActionKind::Imposed, "B", 50.0e3) });
    let facts = facts::BeamFacts::of(&document, &document.beams[0]);
    assert!((facts.point_x(document.beams[0].actions.last().expect("point load")) - 2.0).abs() < 1e-12);
    let selection = [scene::EntityRef::new(scene::EntityKind::Member, "beam-B1", "beams[id=beam-B1]")];
    let list = handles(&document, &[], &selection);
    assert_handle_laws(&list);
    let grip = list.iter().find(|h| h.id == "beam-B1:action-P-1-x").expect("position grip");
    let write = &grip.writes_at(3.5)[0];
    assert_eq!(write.path, "beams[id=beam-B1].actions[id=P-1].xM");
    assert!((write.value - 3.5).abs() < 1e-9, "{write:?}");
    assert_eq!(grip.writes_at(3.5).len(), 1);
    assert!(ids(&validated(&document)).contains(&"beam-B1:load-P-1"));
}

#[test]
fn the_clamped_point_position_never_leaves_the_member_span() {
    let document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let facts = facts::BeamFacts::of(&document, &document.beams[0]);
    let far = model::CharacteristicAction { x_m: 1.0e3, ..model::CharacteristicAction::point_force("", semio_s_artifact_norm_en1994_model::ActionStage::Any, semio_s_artifact_norm_en1994_model::ActionKind::Permanent, "", 0.0) };
    assert!((facts.point_x(&far) - document.beams[0].span_m).abs() < 1e-12);
    let unset = model::CharacteristicAction::point_force("", semio_s_artifact_norm_en1994_model::ActionStage::Any, semio_s_artifact_norm_en1994_model::ActionKind::Permanent, "", 0.0);
    assert!((facts.point_x(&unset) - facts.l_total / 2.0).abs() < 1e-12);
}

#[test]
fn removing_an_action_removes_its_arrow_and_its_grip() {
    let mut document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let selection = [scene::EntityRef::new(scene::EntityKind::Member, "beam-B1", "beams[id=beam-B1]")];
    assert!(handles(&document, &[], &selection).iter().any(|h| h.id == "beam-B1:action-Q-office"));
    document.beams[0].actions.retain(|a| a.id != "Q-office");
    assert!(!handles(&document, &[], &selection).iter().any(|h| h.id == "beam-B1:action-Q-office"));
    assert!(!ids(&validated(&document)).contains(&"beam-B1:load-Q-office"));
}

#[test]
fn continuous_span_handle_halves_the_dragged_total_length() {
    let document = continuous();
    let selection = [scene::EntityRef::new(scene::EntityKind::Member, "beam-B1", "beams[id=beam-B1]")];
    let list = handles(&document, &[], &selection);
    assert_handle_laws(&list);
    let span = list.iter().find(|h| h.id == "beam-B1:span").unwrap();
    assert!((span.writes_at(16.0)[0].value - 8.0).abs() < 1e-9);
}

#[test]
fn diagrams_validate_and_bind_to_editable_fields() {
    for document in [En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), En1994Snapshot::bridge_girder(), continuous(), failing()] {
        let report = semio_s_artifact_norm_en1994_inferences::evaluate(&document);
        let list = diagrams(&document, &report, None);
        let names: Vec<&str> = list.iter().map(|d| d.id.as_str()).collect();
        for expected in ["stress-blocks:beam-B1", "effective-width:beam-B1", "partial-connection:beam-B1", "moment-curvature:beam-B1", "stud-slip:beam-B1", "slip-distribution:beam-B1", "shear-flow:beam-B1", "mn:col-C1", "buckling:col-C1", "slab-span:slab-S1"] {
            assert!(names.contains(&expected), "missing {expected}");
        }
        for d in &list {
            d.validate().unwrap_or_else(|v| panic!("{}: {v:?}", d.id));
            assert!(d.table_twin.rows() > 0);
        }
        assert_eq!(diagrams(&document, &report, Some("col-C1")).len(), 2);
    }
}

#[test]
fn partial_connection_curve_runs_from_steel_alone_to_full_connection_and_its_marker_shows_the_degree_of_the_persisted_studs() {
    let document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let report = semio_s_artifact_norm_en1994_inferences::evaluate(&document);
    let f = facts::BeamFacts::of(&document, &document.beams[0]);
    let d = diagrams(&document, &report, Some("beam-B1")).into_iter().find(|d| d.id == "partial-connection:beam-B1").unwrap();
    let scene::Series::Polyline { points, .. } = &d.series[0] else { panic!("curve") };
    assert!((points[0].y - f.m_pla / 1e3).abs() < 1e-9 && (points.last().unwrap().y - f.m_pl_rd / 1e3).abs() < 1e-9);
    let marker = &d.markers[0];
    assert!(marker.binding.is_none(), "the degree follows from the studs: no field to drag");
    assert!((marker.at.x - f.eta.min(1.0)).abs() < 1e-12);
}

#[test]
fn stud_slip_curve_passes_through_the_design_resistance() {
    let document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let report = semio_s_artifact_norm_en1994_inferences::evaluate(&document);
    let f = facts::BeamFacts::of(&document, &document.beams[0]);
    let d = diagrams(&document, &report, Some("beam-B1")).into_iter().find(|d| d.id == "stud-slip:beam-B1").unwrap();
    let marker = &d.markers[0];
    let p_rk = f.p_rd / 1e3 * f.params.gamma_v;
    assert!((diagrams::stud_load(p_rk, marker.at.x) - marker.at.y).abs() < 1e-9);
}

#[test]
fn golden_scenes_match_the_committed_bytes() {
    for (path, document) in [(GOLDEN_BUILDING, En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)), (GOLDEN_BRIDGE, En1994Snapshot::bridge_girder()), (GOLDEN_SHOWCASE, En1994Snapshot::full_showcase())] {
        let text = canonical_json(&validated(&document));
        scene::projection_kit::test_support::assert_golden(path, &text);
        let committed = std::fs::read_to_string(path).expect("golden");
        let reparsed: serde_json::Value = serde_json::from_str(&committed).expect("golden is JSON");
        assert_eq!(reparsed["schema"], "semio.norm.structural-scene/v1");
    }
}

#[test]
fn dependencies_and_kinds_resolve_for_every_collection() {
    let document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    for (id, kind) in [("beam-B1", "beam"), ("beam-B1:stud-beam-B1-stud-0", "beam"), ("col-C1", "column"), ("slab-S1:rebar", "slab")] {
        assert_eq!(entity_kind_of(&document, id).map(|k| k.id), Some(kind));
    }
    assert!(entity_kind_of(&document, "missing").is_none());
    assert!(!entity_dependencies(&document, "beam-B1").is_empty());
    let _: &CompositeBeam = &document.beams[0];
}

fn line_of(position: checks::catalogue::SpanPosition, span: f64, adjacent: f64, fraction: f64) -> statics::Line {
    use checks::catalogue::continuous::{Layout, Stiffness};
    let stiffness = Stiffness { ei1: 1.0, ei2: 0.5, cracked_fraction: fraction };
    statics::uniform_load_line(&Layout::of(position, span, adjacent), stiffness).expect("line")
}

#[test]
fn the_deflection_line_matches_the_closed_form_of_a_uniformly_loaded_span() {
    let closed = |u: f64| 3.2 * (u - 2.0 * u.powi(3) + u.powi(4));
    for u in [0.1, 0.25, 0.5, 0.75, 0.9] {
        let got = statics::interpolate(&line_of(checks::catalogue::SpanPosition::Simple, 10.0, 0.0, 0.0).deflection, u);
        assert!((got - closed(u)).abs() < 2e-3, "u {u}: {got} vs {}", closed(u));
    }
}

#[test]
fn the_two_span_hogging_zone_lies_between_the_contraflexure_points_at_three_quarters_of_each_span() {
    let line = line_of(checks::catalogue::SpanPosition::End, 10.0, 0.0, 0.0);
    assert_eq!(line.hogging.len(), 1);
    let (a, b) = line.hogging[0];
    assert!((a - 0.375).abs() < 2e-3 && (b - 0.625).abs() < 2e-3, "{a} {b}");
    assert!(line_of(checks::catalogue::SpanPosition::Simple, 10.0, 0.0, 0.0).hogging.is_empty());
}

#[test]
fn interior_and_cantilever_spans_have_one_hogging_zone_per_support_and_a_downward_tip() {
    let interior = line_of(checks::catalogue::SpanPosition::Interior, 8.0, 6.0, 0.15);
    assert_eq!(interior.hogging.len(), 2);
    let cantilever = line_of(checks::catalogue::SpanPosition::Cantilever, 3.0, 8.0, 0.15);
    assert_eq!(cantilever.hogging.len(), 1);
    assert!((cantilever.deflection.last().expect("tip").1 - 1.0).abs() < 1e-6);
}

#[test]
fn beams_are_coloured_per_station_and_every_stud_is_bound_to_its_own_utilisation() {
    let (scene, report) = project_of(&continuous());
    let beam = scene.entities.iter().find(|e| e.entity.id == "beam-1" || e.entity.kind == scene::EntityKind::Member).expect("beam");
    let per = beam.utilisation.as_ref().and_then(|b| b.per_station.as_ref()).expect("per-station values");
    assert!(per.len() >= 5 && per.windows(2).all(|w| w[0].at < w[1].at));
    let worst = report.checks.iter().filter(|c| c.subject.entity_id == beam.entity.id).map(|c| c.utilization).fold(0.0_f64, f64::max);
    assert!((per.iter().map(|s| s.value).fold(0.0, f64::max) - worst).abs() < 1e-9);
    let studs: Vec<f64> = scene.entities.iter().filter(|e| e.entity.id.contains(":stud-")).filter_map(|e| e.utilisation.as_ref().map(|b| b.ratio)).collect();
    assert!(studs.len() >= 2 && studs.windows(2).any(|w| (w[0] - w[1]).abs() > 1e-9), "studs differ: {studs:?}");
}

#[test]
fn the_mode_track_reveals_one_eigenform_layer_per_keyframe() {
    let (scene, _) = project_of(&En1994Snapshot::unpropped_building());
    assert_eq!(scene.validate(), Ok(()));
    let track = scene.tracks.iter().find(|t| t.id == "mode").expect("mode track");
    assert_eq!((track.kind.clone(), track.min, track.max, track.step), (scene::TrackKind::Mode, 1.0, 2.0, 1.0));
    for key in &track.keyframes {
        let layer = key.layer.as_deref().expect("layer");
        assert!(scene.layers.iter().any(|l| l.id == layer));
        let carriers: Vec<_> = scene.entities.iter().filter(|e| e.layer == layer).collect();
        assert!(!carriers.is_empty() && carriers.iter().all(|e| e.appearance.deformation.as_ref().is_some_and(|o| o.kind == scene::DeformationKind::Mode && o.motion == scene::Motion::Harmonic && o.index as f64 == key.at && o.load_factor.is_none_or(|a| a > 0.0))));
    }
}

#[test]
fn a_document_with_a_single_mode_gets_neither_mode_entities_nor_a_track() {
    let (scene, _) = project_of(&En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De));
    assert!(scene.tracks.is_empty() && scene.entities.iter().all(|e| !e.layer.starts_with("mode-")));
}

#[test]
fn the_critical_loads_match_the_closed_forms() {
    let document = En1994Snapshot::unpropped_building();
    let facts = facts::ColumnFacts::of(&document, &document.columns[0]);
    let c = &facts.column;
    let ei = checks::part_column::ei_eff_nmm2(c, facts.annex, checks::catalogue::ecm_pa(c.concrete_f_ck_pa));
    let (ncr, _) = tracks::column_buckling(&facts).expect("column mode");
    let euler = std::f64::consts::PI.powi(2) * ei / c.length_m.powi(2);
    assert!((ncr - euler).abs() <= 2e-3 * euler, "{ncr} vs {euler}");
    let mut beam = document.beams[0].clone();
    beam.ltb_length_m = 0.0;
    let f = facts::BeamFacts::of(&document, &beam);
    let (h, b, tw, tf, l) = (f.steel.height_m, f.steel.width_m, f.steel.tw_m, f.steel.tf_m, f.l_total);
    let iz = 2.0 * tf * b.powi(3) / 12.0 + (h - 2.0 * tf) * tw.powi(3) / 12.0;
    let (e, g, j) = (210.0e9, 81.0e9, (2.0 * b * tf.powi(3) + (h - 2.0 * tf) * tw.powi(3)) / 3.0);
    let iw = iz * (h - tf).powi(2) / 4.0;
    let pi = std::f64::consts::PI;
    let mcr = pi / l * (e * iz * g * j * (1.0 + pi * pi * e * iw / (g * j * l * l))).sqrt();
    let (kernel, _) = tracks::girder_ltb(&f).expect("ltb mode");
    assert!((kernel - mcr).abs() <= 5e-3 * mcr, "{kernel} vs {mcr}");
}

#[test]
fn moment_curvature_diagram_rises_to_the_nonlinear_resistance_below_the_plastic_moment() {
    let document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let report = semio_s_artifact_norm_en1994_inferences::evaluate(&document);
    let f = facts::BeamFacts::of(&document, &document.beams[0]);
    let d = diagrams(&document, &report, Some("beam-B1")).into_iter().find(|d| d.id == "moment-curvature:beam-B1").expect("M-kappa diagram");
    let scene::Series::Polyline { points, .. } = &d.series[0] else { panic!("curve") };
    assert!(points.first().is_some_and(|p| p.x == 0.0 && p.y.abs() < 1e-9));
    assert!(points.windows(2).all(|w| w[1].x > w[0].x));
    let peak = points.iter().map(|p| p.y).fold(0.0, f64::max);
    assert!(peak <= f.m_pl_rd / 1e3 * 1.001 && peak > 0.5 * f.m_pl_rd / 1e3);
    assert!(report.checks.iter().any(|c| c.id == "en1994.6.2.1.4.mkappa.beam-B1"));
    assert!(report.checks.iter().any(|c| c.id == "en1994.5.4.1.2.le.beam-B1"));
    assert!(report.checks.iter().any(|c| c.id == "en1994.6.7.4.introduction.col-C1"));
}

#[test]
fn the_slip_distribution_is_antisymmetric_and_zero_at_midspan() {
    let document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let (_, report) = project_of(&document);
    let diagram = diagrams(&document, &report, Some("beam-B1")).into_iter().find(|d| d.id == "slip-distribution:beam-B1").expect("slip distribution");
    assert_eq!(diagram.validate(), Ok(()));
    assert!(diagram.table_twin.rows() > 0);
    let Series::Polyline { points, .. } = &diagram.series[0] else { panic!("slip line") };
    let (first, last, middle) = (points.first().unwrap(), points.last().unwrap(), &points[points.len() / 2]);
    assert!(first.y >= 0.0 && last.y <= 0.0);
    assert!((first.y + last.y).abs() < 1e-9 * first.y.abs().max(1.0));
    assert!(middle.y.abs() < 1e-9);
    assert!(diagram.markers.iter().any(|m| m.bound_check.is_some()));
}

#[test]
fn slip_arrows_follow_the_slip_distribution_and_the_capacity() {
    use super::slip_arrows::{arrow_length, arrows, MAX_ARROWS};
    assert!((arrow_length(6.0, 1.0) - 0.8).abs() < 1e-12);
    assert!((arrow_length(-3.0, 1.0) - 0.4).abs() < 1e-12);
    assert!((arrow_length(0.0, 1.0) - 0.1).abs() < 1e-12);
    let document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let f = facts::BeamFacts::of(&document, &document.beams[0]);
    let found = arrows(&f);
    assert!(!found.is_empty() && found.len() <= MAX_ARROWS);
    assert!(found.first().unwrap().1 > 0.0 && found.last().unwrap().1 < 0.0, "{found:?}");
    let (peak, p_rk) = diagrams::slip_inputs(&f);
    assert!((found[0].1 - diagrams::slip_at(&f, peak, p_rk, found[0].0)).abs() < 1e-12);
    assert!(found.iter().all(|(_, slip)| slip.abs() <= 8.0 + 1e-9));
    let scene = validated(&document);
    let ids: Vec<&str> = ids(&scene).into_iter().filter(|i| i.starts_with("beam-B1:slip-")).collect();
    assert_eq!(ids.len(), found.len());
    assert!(scene.layers.iter().any(|l| l.id == "slip"));
    for (k, (_, slip)) in found.iter().enumerate() {
        let entity = scene.entities.iter().find(|e| e.entity.id == format!("beam-B1:slip-{}", k + 1)).unwrap();
        if let Some(u) = &entity.utilisation {
            assert!((u.ratio - slip.abs() / 6.0).abs() < 1e-9, "{} vs {}", u.ratio, slip.abs() / 6.0);
        }
    }
}

fn fold_into(document: &mut En1994Snapshot, commit: &scene::projection_kit::PlacementCommit) {
    let mut rows = scene::projection_kit::placements_of(document);
    commit.apply(&mut rows);
    document.placements = rows.into_iter().map(|row| model::Placement { id: row.entity_id, x: row.origin.x, y: row.origin.y, z: row.origin.z, yaw: row.yaw }).collect();
}

#[test]
fn dragging_an_entity_commits_one_undoable_event_and_the_reload_stands_at_the_same_pose() {
    use scene::projection_kit::{PlacementCommit, PlacementMove};
    let mut document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let id = document.beams[0].id.clone();
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

fn encased_on_fire() -> En1994Snapshot {
    let mut d = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    d.fire_rating = semio_s_artifact_norm_en1994_model::FireRating::R60;
    let c = &mut d.columns[0];
    c.kind = semio_s_artifact_norm_en1994_model::ColumnKind::Encased;
    c.outer_size_m = 0.4;
    c.reinforcement_as_m2 = 4.0 * 314.16e-6;
    c.section_height_m = 0.2;
    c.section_width_m = 0.2;
    c.section_tw_m = 0.0085;
    c.section_tf_m = 0.013;
    c.rebar_cover_m = 0.05;
    d
}

#[test]
fn encased_columns_carry_a_temperature_plate_with_one_field_per_fire_minute_and_a_fire_track() {
    let scene = validated(&encased_on_fire());
    let plate = scene.entities.iter().find(|e| e.entity.id == "col-C1:fire-plate").expect("fire plate");
    let scene::ColourChannel::Fields { fields, active } = &plate.appearance.channel else { panic!("fields channel") };
    assert_eq!(fields.iter().map(|f| f.id.as_str()).collect::<Vec<_>>(), ["fire-temp@30", "fire-temp@60"]);
    assert_eq!(active, "fire-temp@60");
    let scene::FieldSamples::Vertex { values } = &fields[1].samples else { panic!("vertex samples") };
    assert!(values.iter().all(|t| (20.0..1100.0).contains(t)));
    let centre = values[values.len() / 2];
    let corner = values[0];
    assert!(corner > centre + 100.0, "the surface is hotter than the core: corner {corner} centre {centre}");
    let track = scene.tracks.iter().find(|t| t.id == "fire").expect("fire track");
    assert_eq!(track.keyframes.iter().map(|k| k.field.clone().unwrap()).collect::<Vec<_>>(), ["fire-temp@30", "fire-temp@60"]);
    assert!(scene.legends.iter().any(|l| l.id == "fire-temperature"));
}

#[test]
fn filled_columns_and_unrated_documents_have_no_temperature_plate() {
    assert!(!ids(&validated(&En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De))).contains(&"col-C1:fire-plate"));
    let mut unrated = encased_on_fire();
    unrated.fire_rating = semio_s_artifact_norm_en1994_model::FireRating::Unrated;
    let scene = validated(&unrated);
    assert!(!ids(&scene).iter().any(|id| id.ends_with(":fire-plate")));
    assert!(scene.tracks.iter().all(|t| t.id != "fire"));
}

#[test]
fn protected_beams_carry_a_steel_temperature_plate_that_follows_the_lumped_capacity_model() {
    let scene = validated(&En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De));
    let plate = scene.entities.iter().find(|e| e.entity.id == "beam-B1:fire-plate").expect("beam fire plate");
    let scene::ColourChannel::Fields { fields, .. } = &plate.appearance.channel else { panic!("fields channel") };
    assert_eq!(fields.iter().map(|f| f.id.as_str()).collect::<Vec<_>>(), ["fire-temp@30", "fire-temp@60"]);
    let value = |f: &scene::ScalarField| match &f.samples {
        scene::FieldSamples::Vertex { values } => values[0],
        _ => panic!("vertex samples"),
    };
    assert!(value(&fields[1]) > value(&fields[0]) && value(&fields[0]) > 20.0);
    assert!(plate.utilisation.is_some(), "bound to the critical-temperature check");
}

#[test]
fn hogging_slabs_carry_a_crack_width_band_that_peaks_over_the_support() {
    let scene = validated(&continuous());
    let band = scene.entities.iter().find(|e| e.entity.id == "beam-B1:crack-band").expect("crack band");
    let scene::ColourChannel::Fields { fields, .. } = &band.appearance.channel else { panic!("fields channel") };
    let scene::FieldSamples::Vertex { values } = &fields[0].samples else { panic!("vertex samples") };
    assert!(band.utilisation.is_some(), "bound to the crack check");
    let peak = values.iter().copied().fold(0.0, f64::max);
    assert!(peak > 0.0 && peak.is_finite(), "w_k/w_lim peak {peak}");
    assert!(values.iter().copied().fold(f64::MAX, f64::min) < peak, "the band falls away from the support");
    assert!(scene.legends.iter().any(|l| l.id == "crack-width"));
    assert!(!ids(&validated(&En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De))).contains(&"beam-B1:crack-band"));
}

#[test]
fn the_shear_flow_diagram_steps_the_resistance_per_station_cell_of_the_persisted_studs() {
    let document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let f = facts::BeamFacts::of(&document, &document.beams[0]);
    assert!(f.studs.rows.windows(2).all(|w| w[0].x_m <= w[1].x_m) && f.studs.rows.len() == document.studs.len());
    let cells = f.studs.cells();
    let total: f64 = cells.iter().map(|c| c.resistance_n_per_m() * c.length_m()).sum();
    let weighted: f64 = f.studs.in_span().map(|s| s.p_rd_n).sum();
    assert!((total - weighted).abs() < 1e-6 * total.max(1.0), "{total} {weighted}");
    assert!((cells.iter().map(|c| c.length_m()).sum::<f64>() - f.beam.span_m).abs() < 1e-9, "the cells tile the span");
    let report = semio_s_artifact_norm_en1994_inferences::evaluate(&document);
    let flow = diagrams(&document, &report, None).into_iter().find(|d| d.id == "shear-flow:beam-B1").expect("shear flow");
    let points = flow.series.iter().find_map(|s| match s { scene::Series::Polyline { id, points, .. } if id == "v-rd-staircase" => Some(points), _ => None }).expect("staircase");
    assert_eq!(points.len(), 2 * cells.len());
}

#[test]
fn hogging_slabs_carry_a_bar_stress_band_with_the_tension_stiffening_increment() {
    let scene = validated(&continuous());
    let band = scene.entities.iter().find(|e| e.entity.id == "beam-B1:tension-band").expect("tension band");
    let scene::ColourChannel::Fields { fields, .. } = &band.appearance.channel else { panic!("fields channel") };
    let scene::FieldSamples::Vertex { values } = &fields[0].samples else { panic!("vertex samples") };
    assert!(band.utilisation.is_some(), "bound to the tension-stiffening check");
    let (peak, floor) = (values.iter().copied().fold(0.0, f64::max), values.iter().copied().fold(f64::MAX, f64::min));
    assert!(floor > 0.0 && peak > floor, "the increment lifts the whole band: {floor} .. {peak}");
    assert!(scene.legends.iter().any(|l| l.id == "rebar-stress"));
}

#[test]
fn hogging_bridge_girders_carry_a_reinforcement_fatigue_band_bound_to_the_rebar_check() {
    let mut document = En1994Snapshot::bridge_girder();
    document.fatigue_detail = semio_s_artifact_norm_en1994_model::FatigueDetail::Reinforcement;
    document.beams[0].support = semio_s_artifact_norm_en1994_model::SupportCondition::Continuous2Span;
    document.beams[0].as_hogging_m2_per_m = 0.004;
    let scene = validated(&document);
    let id = format!("{}:fatigue-band", document.beams[0].id);
    let band = scene.entities.iter().find(|e| e.entity.id == id).expect("fatigue band");
    let scene::ColourChannel::Fields { fields, .. } = &band.appearance.channel else { panic!("fields channel") };
    let scene::FieldSamples::Vertex { values } = &fields[0].samples else { panic!("vertex samples") };
    assert!(band.utilisation.is_some(), "bound to the rebar fatigue check");
    let (peak, floor) = (values.iter().copied().fold(0.0, f64::max), values.iter().copied().fold(f64::MAX, f64::min));
    assert!(floor > 0.0 && peak >= floor, "the increment lifts the whole band: {floor} .. {peak}");
    assert!(scene.legends.iter().any(|l| l.id == "rebar-fatigue"));
    assert!(!ids(&validated(&continuous())).contains(&"beam-B1:fatigue-band"));
}

#[test]
fn bridge_girder_details_add_an_end_plate_with_three_handles_that_write_the_detail_leaves() {
    let mut document = En1994Snapshot::bridge_girder();
    {
        let beam = &mut document.beams[0];
        beam.end_plate_width_m = 0.3;
        beam.end_plate_height_m = 0.25;
        beam.end_zone_length_m = 0.6;
    }
    let (scene, report) = project_of(&document);
    assert_eq!(scene.validate(), Ok(()));
    let plate = scene.entities.iter().find(|e| e.entity.id == "beam-B1:end-plate").expect("end plate");
    assert_eq!(plate.utilisation.as_ref().map(|u| u.check_id.as_str()), Some(report.checks.iter().find(|c| c.id.starts_with("en1994.2.6.6.5.end-plate.")).unwrap().id.as_str()));
    let selection = [scene::EntityRef::new(scene::EntityKind::Member, "beam-B1", "x[id=beam-B1]")];
    let list = handles(&document, &[], &selection);
    assert_handle_laws(&list);
    let get = |id: &str| list.iter().find(|h| h.id == id).unwrap_or_else(|| panic!("handle {id}"));
    assert!((get("beam-B1:end-zone-length").writes_at(0.8)[0].value - 0.8).abs() < 1e-12);
    assert!((get("beam-B1:end-plate-width").writes_at(0.2)[0].value - 0.4).abs() < 1e-12);
    let depth = facts::BeamFacts::of(&document, &document.beams[0]).depth;
    assert!((get("beam-B1:end-plate-height").writes_at(0.3 - depth)[0].value - 0.3).abs() < 1e-12);
    assert_eq!(get("beam-B1:end-plate-height").writes_at(0.0)[0].path, "beams[id=beam-B1].endPlateHeightM");
    let undeclared = project_of(&En1994Snapshot::bridge_girder()).0;
    assert!(!ids(&undeclared).contains(&"beam-B1:end-plate"));
}

#[test]
fn the_crack_band_profile_matches_the_pynite_support_moment_line_of_the_cracked_two_span_beam() {
    let ei1 = 210e9 * 6e-4;
    let layout = checks::catalogue::continuous::Layout::of(checks::catalogue::SpanPosition::End, 8.0, 8.0);
    let zones = checks::catalogue::continuous::hogging_band_profile(&layout, checks::catalogue::continuous::Stiffness { ei1, ei2: 0.5 * ei1, cracked_fraction: 0.15 }, 25);
    assert_eq!(zones.len(), 1);
    let zone = &zones[0];
    assert_eq!(zone.len(), 25);
    assert!((zone[0].0 - 6.3988).abs() < 2e-3 && (zone[24].0 - 9.6012).abs() < 2e-3, "{:?} {:?}", zone[0], zone[24]);
    for (k, expected) in [(0, 0.0), (6, 0.44996), (12, 1.0), (18, 0.44996), (24, 0.0)] {
        assert!((zone[k].1 - expected).abs() < 2e-3, "station {k}: {} against {expected}", zone[k].1);
    }
}

#[test]
fn every_example_and_showcase_projection_obeys_the_scene_diagram_and_handle_laws() {
    let documents = [En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), En1994Snapshot::unpropped_building(), En1994Snapshot::custom_plate_building(), En1994Snapshot::bridge_girder(), En1994Snapshot::fire_demanding(), continuous(), column_kinds(), failing()];
    for document in documents {
        let (scene, report) = project_of(&document);
        let selection: Vec<scene::EntityRef> = scene.entities.iter().map(|e| e.entity.clone()).collect();
        scene::projection_kit::test_support::assert_projection_laws(&scene, &diagrams(&document, &report, None), &handles(&document, &[], &selection));
    }
}

fn fire_values(scene: &StructuralScene, id: &str, index: usize) -> Vec<f64> {
    let plate = scene.entities.iter().find(|e| e.entity.id == id).unwrap_or_else(|| panic!("missing {id}"));
    let scene::ColourChannel::Fields { fields, active } = &plate.appearance.channel else { panic!("fields channel") };
    assert_eq!(fields.iter().map(|f| f.id.as_str()).collect::<Vec<_>>(), ["fire-temp@30", "fire-temp@60"]);
    assert_eq!(active, "fire-temp@60");
    assert!(plate.utilisation.is_some(), "{id} is bound to a fire check");
    let scene::FieldSamples::Vertex { values } = &fields[index].samples else { panic!("vertex samples") };
    values.clone()
}

#[test]
fn beam_fire_plates_cover_the_steel_the_slab_and_the_reinforcement() {
    let scene = validated(&En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De));
    let steel = fire_values(&scene, "beam-B1:fire-plate", 1);
    let slab = fire_values(&scene, "beam-B1:fire-slab", 1);
    let rebar = fire_values(&scene, "beam-B1:fire-rebar", 1);
    assert!(steel.iter().all(|t| *t > 60.0 && *t < 1000.0));
    assert!(slab[0] > slab[slab.len() - 1] + 300.0, "the soffit is far hotter than the slab top: {} vs {}", slab[0], slab[slab.len() - 1]);
    assert!(rebar.iter().all(|t| (20.0..400.0).contains(t)), "the reinforcement layer stays far below the soffit: {rebar:?}");
    assert!(fire_values(&scene, "beam-B1:fire-slab", 0)[0] < slab[0]);
    assert!(scene.tracks.iter().any(|t| t.id == "fire"));
}

#[test]
fn beam_fire_diagrams_follow_the_check_and_the_resistance_never_grows() {
    let document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let report = semio_s_artifact_norm_en1994_inferences::evaluate(&document);
    let list = diagrams(&document, &report, Some("beam-B1"));
    let temperature = list.iter().find(|d| d.id == "fire-temperature:beam-B1").expect("temperature diagram");
    let resistance = list.iter().find(|d| d.id == "fire-resistance:beam-B1").expect("resistance diagram");
    for d in [temperature, resistance] {
        d.validate().unwrap_or_else(|v| panic!("{}: {v:?}", d.id));
        assert!(d.table_twin.rows() >= 13);
    }
    let scene::Series::Polyline { points, .. } = resistance.series.iter().find(|s| matches!(s, scene::Series::Polyline { id, .. } if id == "m-fi-rd")).expect("m-fi-rd") else { unreachable!() };
    assert!(points.windows(2).all(|w| w[1].y <= w[0].y + 1e-9));
    let check = report.checks.iter().find(|c| c.id == "en1994.1-2.4.3.mfi.beam-B1").expect("mfi check");
    assert_eq!(resistance.markers[0].bound_check.as_deref(), Some(check.id.as_str()));
    let expected = check.utilization;
    assert!((resistance.markers[0].utilisation.unwrap() - expected).abs() < 1e-12);
    assert!(diagrams(&document, &report, Some("beam-B1")).iter().all(|d| !d.id.starts_with("fatigue-")), "no fatigue diagrams without a bridge");
}

#[test]
fn bridge_girders_get_fatigue_diagrams_and_a_highlighted_governing_stud_row() {
    let document = En1994Snapshot::bridge_girder();
    let (scene, report) = project_of(&document);
    scene.validate().unwrap_or_else(|v| panic!("{v:?}"));
    let list = diagrams(&document, &report, Some("beam-B1"));
    let sn = list.iter().find(|d| d.id == "fatigue-sn:beam-B1").expect("S-N diagram");
    let rows = list.iter().find(|d| d.id == "fatigue-rows:beam-B1").expect("stud-row diagram");
    for d in [sn, rows] {
        d.validate().unwrap_or_else(|v| panic!("{}: {v:?}", d.id));
    }
    assert!(sn.markers.iter().any(|m| m.id == "stud-point") && sn.series.len() == 3);
    let marker = &rows.markers[0];
    let length = document.beams[0].span_m;
    assert!(marker.at.x.min(length - marker.at.x) < length / 4.0, "the governing row sits near a support: x = {}", marker.at.x);
    let highlight = scene.entities.iter().find(|e| e.entity.id == "beam-B1:fatigue-row").expect("highlight of the governing row");
    assert!(highlight.utilisation.is_some());
    assert!(!ids(&validated(&En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De))).contains(&"beam-B1:fatigue-row"));
}

#[test]
fn the_placement_plan_has_one_dragging_marker_per_placement_and_only_without_a_focus() {
    let mut document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    document.placements = vec![model::Placement { id: "beam-B1".into(), x: 1.5, y: -2.0, z: 0.25, yaw: std::f64::consts::FRAC_PI_2 }];
    let report = semio_s_artifact_norm_en1994_inferences::evaluate(&document);
    let plan = diagrams(&document, &report, None).into_iter().find(|d| d.id == "placements").unwrap();
    assert_eq!((plan.markers.len(), plan.table_twin.columns.len()), (1, 4));
    assert_eq!((plan.markers[0].at.x, plan.markers[0].at.y), (1.5, -2.0));
    assert_eq!(plan.markers[0].binding.as_ref().map(|b| b.path.as_str()), Some("placements[id=beam-B1].x"));
    assert!(diagrams(&document, &report, Some("beam-B1")).iter().all(|d| d.id != "placements:document"));
    document.placements.clear();
    assert!(diagrams(&document, &report, None).iter().all(|d| d.id != "placements:document"));
}

#[test]
fn placements_are_a_registered_kind_whose_rows_resolve_when_no_entity_owns_the_id() {
    let mut document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let spec = ENTITY_KINDS.iter().find(|k| k.id == "placement").expect("placement kind");
    assert_eq!((spec.collection, spec.label_en, spec.label_de), ("placements", "Placement", "Platzierung"));
    document.placements = vec![model::Placement { id: "beam-B1".into(), x: 1.0, y: 2.0, z: 0.0, yaw: 0.0 }, model::Placement { id: "orphan".into(), x: 0.0, y: 0.0, z: 0.0, yaw: 0.0 }];
    assert_eq!(entity_kind_of(&document, "beam-B1").map(|k| k.id), Some("beam"), "a placement follows the entity it moves");
    assert_eq!(entity_kind_of(&document, "orphan").map(|k| k.id), Some("placement"));
}

#[test]
fn the_basis_badge_follows_the_document_annex_in_the_scene_and_every_diagram() {
    scene::projection_kit::test_support::assert_basis_follows_annex(norm::NormFamilyId::En1994, |annex| {
        let mut document = continuous();
        document.annex = annex;
        let (scene, report) = project_of(&document);
        (scene, diagrams(&document, &report, None))
    });
}
