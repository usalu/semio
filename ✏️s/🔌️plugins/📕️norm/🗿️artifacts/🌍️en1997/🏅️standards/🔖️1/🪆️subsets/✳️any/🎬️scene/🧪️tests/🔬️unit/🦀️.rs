use super::mechanics as mech;
use super::model::{compliant_demo, noncompliant_demo, En1997Snapshot};
use super::scene::{canonical_json, Validate};
use super::*;
use super::kit::{check_owner, focus_of_outcome, owner_entity, route_of, CheckRoute, CHECK_OWNERS, DRAWN_FAMILIES, OUTCOME_FAMILIES};
use scene::projection_kit::base_id;
use scene::projection_kit::test_support::assert_handle_laws;
use serde_json::{json, Value};
use std::collections::BTreeMap;

fn report(doc: &En1997Snapshot) -> norm::CheckReport {
    checks::check_project(doc)
}

fn scene_of(doc: &En1997Snapshot) -> scene::StructuralScene {
    let report = report(doc);
    project(&SceneInput { document: doc, report: &report, placements: &[], revision: SceneRevision { document_seq: 1, report_seq: 1 } })
}

fn hashes(scene: &scene::StructuralScene) -> BTreeMap<String, String> {
    scene.entities.iter().map(|e| (e.entity.id.clone(), e.content_hash.clone())).collect()
}

fn oracle(case: &Value) -> Value {
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🧪️tests/🔮️oracle/🐍️.py");
    let out = norm::python_oracle::run_ok(script, &[], Some(case.to_string().as_bytes()));
    serde_json::from_slice(&out.stdout).expect("oracle json")
}

fn close(a: f64, b: f64, tolerance: f64) {
    assert!((a - b).abs() <= tolerance * b.abs().max(1.0), "{a} != {b}");
}

fn numbers(v: &Value) -> Vec<f64> {
    v.as_array().expect("array").iter().map(|x| x.as_f64().expect("number")).collect()
}

#[test]
fn compliant_scene_is_valid_and_complete() {
    let doc = compliant_demo();
    let scene = scene_of(&doc);
    if let Err(violations) = scene.validate() {
        panic!("{violations:?}");
    }
    let ids: Vec<&str> = scene.entities.iter().map(|e| e.entity.id.as_str()).collect();
    for expected in [
        "layer-sand",
        "layer-gravel",
        "groundwater",
        "footing-F1",
        "footing-F1:effective-area",
        "footing-F1:resultant",
        "footing-F1:wedge-active",
        "footing-F1:fan-right",
        "footing-F1:passive-left",
        "footing-F1:sliding-plane",
        "wall-W1",
        "wall-W1:backfill",
        "wall-W1:pressure-active",
        "wall-W1:pressure-passive",
        "wall-W1:resultant-active",
        "wall-W1:surcharge",
        "slope-S1",
        "slope-S1:slip-circle",
        "slope-S1:slice-1",
        "pile-P1",
        "pile-P1:cap",
        "pile-P1:pile-1",
        "pile-P1:base",
        "upl-U1",
        "settlement-field",
    ] {
        assert!(ids.contains(&expected), "missing {expected} in {ids:?}");
    }
    assert!(scene.layers.iter().any(|l| l.id == "mechanisms") && scene.legends.iter().any(|l| l.id == "settlement"));
}

#[test]
fn noncompliant_scene_is_valid_and_flags_failures() {
    let doc = noncompliant_demo();
    let scene = scene_of(&doc);
    scene.validate().expect("valid");
    let footing = scene.entities.iter().find(|e| e.entity.id == "footing-F1").expect("footing");
    assert!(footing.utilisation.as_ref().is_some_and(|u| u.ratio > 1.0));
    assert!(scene.annotations.iter().any(|a| matches!(a, scene::Annotation::FailureGlyph { .. })));
}

#[test]
fn projection_is_deterministic_and_incremental() {
    let doc = compliant_demo();
    let (a, b) = (scene_of(&doc), scene_of(&doc));
    assert_eq!(canonical_json(&a), canonical_json(&b));
    let mut wider = doc.clone();
    wider.footings[0].width += 0.5;
    let (before, after) = (hashes(&a), hashes(&scene_of(&wider)));
    for layer in ["layer-sand", "layer-gravel", "wall-W1", "slope-S1", "pile-P1"] {
        assert_eq!(before[layer], after[layer], "{layer} must keep its hash");
    }
    assert_ne!(before["footing-F1"], after["footing-F1"]);
    assert_ne!(before["footing-F1:effective-area"], after["footing-F1:effective-area"]);
}

#[test]
fn golden_scene_matches_committed_bytes() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🧪️tests/🧫️fixtures/🏛️scene/🔣️.json");
    scene::projection_kit::test_support::assert_golden(path, &canonical_json(&scene_of(&compliant_demo())));
}

#[test]
fn default_layout_rule_is_documented_rows() {
    let doc = compliant_demo();
    let layout = Layout::of(&doc, &[]);
    assert_eq!(layout.pose("footing-F1").origin, scene::Vec3::new(0.0, layout::ROW_FOOTINGS, 0.0));
    assert_eq!(layout.pose("pile-P1").origin, scene::Vec3::new(0.0, layout::ROW_PILES, 0.0));
    assert_eq!(layout.pose("wall-W1").origin, scene::Vec3::new(0.0, layout::ROW_WALLS, 0.0));
    assert_eq!(layout.pose("slope-S1").origin, scene::Vec3::new(layout::SLOPE_CREST, layout::ROW_SLOPES, 0.0));
    let placed = Layout::of(&doc, &[Placement { entity_id: "footing-F1".into(), origin: scene::Vec3::new(3.0, 2.0, 0.0), yaw: 0.5 }]);
    assert_eq!(placed.pose("footing-F1").origin, scene::Vec3::new(3.0, 2.0, 0.0));
    assert!(placed.max[0] > placed.min[0] && placed.depth >= 12.0);
}

#[test]
fn placements_move_entities_and_keep_the_scene_valid() {
    let doc = compliant_demo();
    let report = report(&doc);
    let placements = [Placement { entity_id: "footing-F1".into(), origin: scene::Vec3::new(4.0, -3.0, 0.0), yaw: 0.7 }, Placement { entity_id: "wall-W1".into(), origin: scene::Vec3::new(-5.0, 9.0, 0.0), yaw: -0.4 }];
    let moved = project(&SceneInput { document: &doc, report: &report, placements: &placements, revision: SceneRevision { document_seq: 1, report_seq: 1 } });
    moved.validate().expect("valid");
    assert_ne!(hashes(&moved)["footing-F1"], hashes(&scene_of(&doc))["footing-F1"]);
}

#[test]
fn handles_write_current_values_at_zero_drag() {
    let doc = compliant_demo();
    let layout = Layout::of(&doc, &[]);
    let grips = handles::all(&doc, &layout);
    assert_handle_laws(&grips);
    let scene = scene_of(&doc);
    let entities: Vec<&str> = scene.entities.iter().map(|e| e.entity.id.as_str()).collect();
    for h in &grips {
        h.validate().expect("valid handle");
        assert!(entities.contains(&h.entity.id.as_str()), "{} has no entity", h.id);
        assert!(scene.entities.iter().any(|e| e.handles.contains(&h.id)), "{} not referenced", h.id);
    }
    let value = |id: &str, path: &str| grips.iter().find(|h| h.id == id).and_then(|h| h.writes_at(0.0).into_iter().find(|w| w.path == path)).map(|w| w.value).expect(path);
    let f = &doc.footings[0];
    close(value("footing-F1:width", "footings[id=footing-F1].width"), f.width, 1e-9);
    close(value("footing-F1:length", "footings[id=footing-F1].length"), f.length, 1e-9);
    close(value("footing-F1:embedment", "footings[id=footing-F1].embedment"), f.embedment, 1e-9);
    close(value("layer-sand:interface", "layers[id=layer-sand].depthBottom"), doc.layers[0].depth_bottom, 1e-9);
    close(value("layer-sand:interface", "layers[id=layer-gravel].depthTop"), doc.layers[0].depth_bottom, 1e-9);
    close(value("groundwater:level", "groundwaterLevel"), doc.groundwater_level, 1e-9);
    let selected = handles::handles(&doc, &[], &[scene::EntityRef::new(scene::EntityKind::Solid, "footing-F1:wedge-active", "footings[id=footing-F1]")]);
    assert_handle_laws(&selected);
    assert!(selected.iter().all(|h| h.entity.id == "footing-F1") && selected.len() >= 5);
    let drag = grips.iter().find(|h| h.id == "footing-F1:embedment").expect("embedment handle").writes_at(0.5);
    close(drag[0].value, f.embedment - 0.5, 1e-9);
}

#[test]
fn diagrams_validate_and_carry_table_twins() {
    let doc = compliant_demo();
    let report = report(&doc);
    let list = diagrams(&doc, &report, None);
    let focused = diagrams(&doc, &report, Some("wall-W1"));
    assert!(focused.len() == 1 && focused[0].id == "earth-pressure:wall-W1");
    let ids: Vec<&str> = list.iter().map(|d| d.id.as_str()).collect();
    for expected in ["earth-pressure:wall-W1", "profile:cpt", "profile:spt", "bearing:factors", "load-settlement:footing-F1"] {
        assert!(ids.contains(&expected), "missing {expected}");
    }
    for d in &list {
        d.validate().unwrap_or_else(|v| panic!("{} {v:?}", d.id));
        assert!(d.table_twin.rows() >= 2, "{}", d.id);
    }
    let wall = &doc.retaining_walls[0];
    let pressure = list.iter().find(|d| d.id == "earth-pressure:wall-W1").expect("pressure");
    let scene::Series::Polyline { points, .. } = &pressure.series[0] else { panic!("active series") };
    close(points[0].x, mech::k_driving(wall) * wall.surcharge / 1000.0, 1e-9);
    close(points[0].y, wall.height, 1e-9);
}

#[test]
fn mechanics_match_closed_forms() {
    close(mech::bearing_factors(30.0)[0], 18.401_17, 1e-5);
    close(mech::bearing_factors(30.0)[1], 30.139_67, 1e-5);
    close(mech::bearing_factors(30.0)[2], 20.093_11, 1e-5);
    let (centre, width) = mech::effective_strip(2.5, 0.25);
    close(centre, 0.25, 1e-12);
    close(width, 2.0, 1e-12);
    let m = mech::mechanism(2.0, 30.0);
    assert!(m.depth >= m.active[2][1].abs() - 1e-12);
    assert_eq!(m.fans[0].len(), m.fans[1].len());
    for (r, l) in m.passive[0].iter().zip(&m.passive[1]) {
        close(r[0], -l[0], 1e-12);
        close(r[1], l[1], 1e-12);
    }
    let pile = &compliant_demo().piles[0];
    assert_eq!(mech::pile_grid(pile).len(), 1);
    let mut group = pile.clone();
    group.count = 6;
    let grid = mech::pile_grid(&group);
    assert_eq!(grid.len(), 6);
    close(grid.iter().map(|g| g[0]).sum::<f64>(), 0.0, 1e-12);
}

#[test]
fn numerics_match_the_numpy_scipy_shapely_oracle() {
    let doc = compliant_demo();
    let phis: Vec<f64> = vec![5.0, 15.0, 25.0, 32.0, 35.0, 45.0];
    let mechanism = mech::mechanism(2.5, 32.0);
    let stress: Vec<(f64, f64, f64, f64, f64, f64)> = vec![(2.5, 2.5, 0.0, 0.0, 1.0, 100_000.0), (2.5, 2.5, 1.25, 1.25, 2.0, 100_000.0), (3.0, 6.0, 4.0, 0.5, 3.0, 80_000.0), (2.0, 2.0, 0.3, -0.2, 0.5, 50_000.0)];
    let (proto, tan_phi, c) = slopes_proto(&doc);
    let (circle, _) = mech::critical_circle(&proto, tan_phi, c).expect("circle");
    let slices = mech::slip_slices(&circle, mech::SLICES);
    let case = json!({
        "phis": phis,
        "mechanism": { "b": 2.5, "phi": 32.0, "active": mechanism.active, "passive": mechanism.passive },
        "stress": stress.iter().map(|s| json!({ "b": s.0, "l": s.1, "px": s.2, "py": s.3, "z": s.4, "q": s.5 })).collect::<Vec<_>>(),
        "slope": { "h": proto.h, "run": mech::slope_run(proto.h, proto.beta_deg), "xc": circle.xc, "yc": circle.yc, "r": circle.r, "slices": slices.iter().map(|s| json!([s.x0, s.x1, s.base[0], s.base[1], s.top[0], s.top[1]])).collect::<Vec<_>>() },
    });
    let o = oracle(&case);
    for (i, phi) in phis.iter().enumerate() {
        let n = mech::bearing_factors(*phi);
        close(n[0], numbers(&o["nq"])[i], 1e-9);
        close(n[1], numbers(&o["nc"])[i], 1e-9);
        close(n[2], numbers(&o["ng"])[i], 1e-9);
    }
    let area = |p: &[[f64; 2]]| (0..p.len()).map(|i| p[i][0] * p[(i + 1) % p.len()][1] - p[(i + 1) % p.len()][0] * p[i][1]).sum::<f64>().abs() / 2.0;
    close(area(&mechanism.active), o["mechanism"]["active"].as_f64().expect("area"), 1e-9);
    close(area(&mechanism.passive[0]), numbers(&o["mechanism"]["passive"])[0], 1e-9);
    close(area(&mechanism.passive[1]), numbers(&o["mechanism"]["passive"])[1], 1e-9);
    let tip = mechanism.fans[0].last().expect("tip");
    close((tip[0] - 1.25).hypot(tip[1]), o["mechanism"]["r1"].as_f64().expect("r1"), 1e-9);
    close(mechanism.active[2][1].abs(), o["mechanism"]["depth"].as_f64().expect("depth"), 1e-9);
    for (i, s) in stress.iter().enumerate() {
        close(mech::sigma_z_at(s.5, s.0, s.1, s.2, s.3, s.4), numbers(&o["sigma"])[i], 1e-6);
    }
    close(o["slope"]["slices"].as_f64().expect("slices"), o["slope"]["exact"].as_f64().expect("exact"), 0.02);
}

fn slopes_proto(doc: &En1997Snapshot) -> (checks::catalogue::slopes::Circle, f64, f64) {
    slopes::proto(doc, &doc.slopes[0]).expect("proto")
}

#[test]
fn large_documents_project_with_unique_ids_in_bounded_time() {
    let mut doc = compliant_demo();
    let proto = doc.footings[0].clone();
    for i in 1..60 {
        let mut f = proto.clone();
        f.id = format!("footing-F{}", i + 1);
        doc.footings.push(f);
    }
    let started = std::time::Instant::now();
    let scene = scene_of(&doc);
    scene.validate().expect("valid");
    assert!(scene.entities.len() > 600, "{}", scene.entities.len());
    eprintln!("[latency] en1997/scene-cold-1000 p50_ms={0:.3} p95_ms={0:.3} budget_ms={1:.1}", started.elapsed().as_secs_f64() * 1e3, 20000.0);
    assert!(started.elapsed() < std::time::Duration::from_secs(20), "{:?}", started.elapsed());
}

#[test]
fn contact_pressure_stations_integrate_to_the_vertical_action_like_numpy() {
    let doc = compliant_demo();
    let report = report(&doc);
    let mut cases = Vec::new();
    let mut wanted = Vec::new();
    for check in report.checks.iter().filter(|c| c.station_field == "soil-pressure") {
        assert!(check.stations_consistent(), "{}", check.id);
        let footing = doc.footings.iter().find(|f| check.id.contains(&format!(".{}.", f.id))).expect("footing");
        let mut line: Vec<_> = check.stations.iter().filter(|s| s.at[1].abs() < 1e-12).collect();
        line.sort_by(|a, b| a.at[0].total_cmp(&b.at[0]));
        if line.first().is_some_and(|s| s.values[0].value > 0.0) {
            cases.push(json!({"x": line.iter().map(|s| s.at[0]).collect::<Vec<_>>(), "sigma": line.iter().map(|s| s.values[0].value).collect::<Vec<_>>(), "l": footing.length}));
            wanted.push(check.computed.value);
        }
    }
    assert!(!cases.is_empty(), "the demo footings emit contact pressure stations");
    let got = oracle(&json!({"contact": cases}));
    for (value, want) in numbers(&got["contact"]).into_iter().zip(wanted) {
        close(value, want, 1e-9);
    }
}

#[test]
fn scene_footing_carries_the_contact_pressure_field_and_slopes_follow_their_stations() {
    let doc = compliant_demo();
    let scene = scene_of(&doc);
    scene.validate().expect("valid");
    let report = report(&doc);
    if report.checks.iter().any(|c| c.station_field == "soil-pressure") {
        assert!(scene.entities.iter().any(|e| e.entity.id.ends_with(":contact-pressure")));
    }
    for check in report.checks.iter().filter(|c| c.station_field == "slip-slice") {
        let peak = check.stations.iter().map(|s| s.utilization).fold(0.0, f64::max);
        close(peak, check.utilization, 1e-12);
    }
}

#[test]
fn consolidation_track_blends_settlement_fields_and_matches_terzaghi() {
    let scene = scene_of(&compliant_demo());
    scene.validate().expect("valid");
    let track = scene.tracks.iter().find(|t| t.id == "consolidation").expect("consolidation track");
    assert_eq!(track.kind, scene::TrackKind::Time);
    assert!(track.min == 0.0 && track.max > track.min);
    let field_ids: Vec<&str> = scene.entities.iter().filter_map(|e| match &e.appearance.channel { scene::ColourChannel::Fields { fields, .. } => Some(fields), _ => None }).flatten().map(|f| f.id.as_str()).collect();
    assert!(track.keyframes.iter().all(|k| k.field.as_deref().is_some_and(|f| field_ids.contains(&f))), "{field_ids:?}");
    assert_eq!(track.keyframes.last().and_then(|k| k.field.as_deref()), Some("settlement"));
    assert!((tracks::degree(0.197) - 0.5).abs() < 2e-3);
    assert!((tracks::degree(0.848) - 0.9).abs() < 2e-3);
    assert_eq!(tracks::degree(0.0), 0.0);
}

#[test]
fn the_consolidation_stages_deepen_the_trough_itself_one_layer_per_stop() {
    let scene = scene_of(&compliant_demo());
    let track = scene.tracks.iter().find(|t| t.id == "consolidation").expect("consolidation track");
    assert_eq!(track.keyframes.last().and_then(|k| k.layer.as_deref()), Some("settlement"));
    let depth = |layer: &str| -> f64 {
        let entity = scene.entities.iter().find(|e| e.layer == layer && e.entity.id.starts_with("settlement-field")).unwrap_or_else(|| panic!("no trough on layer {layer}"));
        match &entity.geometry {
            scene::Geometry::Surface(surface) => match &surface.shape {
                scene::SurfaceShape::Grid { grid } => grid.points.iter().map(|p| -p.z).fold(f64::MIN, f64::max),
                _ => panic!("the trough is a grid"),
            },
            _ => panic!("the trough is a surface"),
        }
    };
    let depths: Vec<f64> = track.keyframes.iter().map(|k| depth(k.layer.as_deref().expect("every stop reveals a layer"))).collect();
    assert!(depths.windows(2).all(|w| w[1] >= w[0] - 1e-12), "{depths:?}");
    assert!(depths[0] < depths[depths.len() - 1] && depths[0] <= 0.0);
}

#[test]
fn wave_two_entities_are_visible_selectable_and_bound_to_their_checks() {
    let doc = compliant_demo();
    let scene = scene_of(&doc);
    scene.validate().unwrap_or_else(|violations| panic!("{violations:?}"));
    let ids: Vec<&str> = scene.entities.iter().map(|e| e.entity.id.as_str()).collect();
    for expected in ["ewall-E1", "ewall-E1:retained-soil", "ewall-E1:active-pressure", "ewall-E1:passive-resistance", "anchor-A1", "anchor-A1:free", "anchor-A1:fixed", "anchor-A1:force", "group-G1", "group-G1:cap", "group-G1:pile-1-1", "group-G1:pile-2-2", "lab-T1", "water-gravel"] {
        assert!(ids.contains(&expected), "missing scene entity {expected}");
    }
    for id in ["ewall-E1", "anchor-A1", "group-G1", "lab-T1", "water-gravel"] {
        let entity = scene.entities.iter().find(|e| e.entity.id == id).expect(id);
        assert!(entity.utilisation.is_some(), "{id} must carry the utilisation of its checks");
    }
}

#[test]
fn wave_two_failures_show_in_the_scene_and_handles_write_existing_leaves() {
    let doc = noncompliant_demo();
    let scene = scene_of(&doc);
    for id in ["ewall-E1", "anchor-A1", "group-G1", "lab-T1", "water-gravel"] {
        let entity = scene.entities.iter().find(|e| e.entity.id == id).expect(id);
        assert_eq!(entity.utilisation.as_ref().map(|b| b.status), Some(scene::CheckStatus::Fail), "{id} fails in the non-compliant demo");
    }
    let tree = semio_framework_value::ToValue::to_value(&doc);
    let layout = Layout::of(&doc, &[]);
    let all = handles::all(&doc, &layout);
    assert_handle_laws(&all);
    for handle in all {
        for write in &handle.writes {
            if write.path.starts_with("placements[") {
                continue;
            }
            semio_s_plugin_norm_app_surface::get_value_at_path(&tree, &write.path).unwrap_or_else(|e| panic!("handle {} writes {}: {e}", handle.id, write.path));
        }
    }
}

#[test]
fn wave_two_entity_registry_resolves_every_collection() {
    let doc = compliant_demo();
    for (id, collection) in [("ewall-E1", "embeddedWalls"), ("anchor-A1", "anchors"), ("group-G1", "pileGroups"), ("lab-T1", "labTests"), ("water-gravel", "layerWaters")] {
        assert_eq!(entity_kind_of(&doc, id).map(|k| k.collection), Some(collection));
    }
    assert!(entity_dependencies(&doc, "anchor-A1").contains(&"ewall-E1".to_string()));
    assert!(entity_dependencies(&doc, "lab-T1").contains(&"layer-sand".to_string()));
    assert!(entity_dependencies(&doc, "layer-sand").contains(&"lab-T1".to_string()));
}

#[test]
fn slip_circle_override_is_drawn_and_dragged_with_three_handles() {
    let mut doc = compliant_demo();
    let auto = scene_of(&doc);
    let auto_circle = auto.entities.iter().find(|e| e.entity.id == "slope-S1:slip-circle").expect("critical circle").accessible_detail.en.clone();
    doc.slopes[0].circle_centre_x = 4.0;
    doc.slopes[0].circle_centre_z = 12.0;
    doc.slopes[0].circle_radius = 14.0;
    let scene = scene_of(&doc);
    scene.validate().unwrap_or_else(|violations| panic!("{violations:?}"));
    let detail = &scene.entities.iter().find(|e| e.entity.id == "slope-S1:slip-circle").expect("override circle").accessible_detail.en;
    assert!(detail.contains("radius 14.0 m") && detail.contains("centre (4.0, 12.0)"), "{detail}");
    assert_ne!(*detail, auto_circle);
    let layout = Layout::of(&doc, &[]);
    let grips: Vec<_> = handles::all(&doc, &layout).into_iter().filter(|h| h.id.starts_with("slope-S1:circle-")).collect();
    assert_eq!(grips.len(), 3);
    for grip in &grips {
        let fields: Vec<&str> = grip.writes.iter().map(|w| w.path.rsplit('.').next().unwrap()).collect();
        assert_eq!(fields, ["circleCentreX", "circleCentreZ", "circleRadius"], "{}", grip.id);
    }
    let radius = grips.iter().find(|h| h.id.ends_with("circle-r")).unwrap();
    let moved = radius.writes.iter().find(|w| w.path.ends_with("circleRadius")).unwrap();
    close(moved.map.base + moved.map.k * 1.0, 15.0, 1e-9);
}

#[test]
fn flow_net_lines_exist_only_with_a_head_and_carry_the_finite_difference_exit_gradient() {
    let mut doc = compliant_demo();
    assert!(scene_of(&doc).entities.iter().all(|e| !e.entity.id.contains(":seepage-")), "no head, no streamlines");
    doc.groundwater_level = 1.0;
    let scene = scene_of(&doc);
    scene.validate().unwrap_or_else(|violations| panic!("{violations:?}"));
    let streams: Vec<_> = scene.entities.iter().filter(|e| e.entity.id.starts_with("ewall-E1:seepage-")).collect();
    let potentials: Vec<_> = scene.entities.iter().filter(|e| e.entity.id.starts_with("ewall-E1:equipotential-")).collect();
    assert!(streams.len() >= 5 && potentials.len() >= 5, "{} streamline and {} equipotential segments", streams.len(), potentials.len());
    let first = streams.iter().find(|e| e.entity.id == "ewall-E1:seepage-1-1").expect("first segment");
    assert!(first.accessible_detail.en.contains(&format!("{:.3}", 0.142_306_890_928_401_57)), "scipy sparse solution of the same stencil: {}", first.accessible_detail.en);
}

#[test]
fn extruded_slip_body_is_labelled_with_the_three_dimensional_factor() {
    let scene = scene_of(&compliant_demo());
    assert!(scene.annotations.iter().any(|a| a.id() == "slope-S1:factor-3d"));
}

#[test]
fn seepage_and_characteristic_bound_diagrams_have_table_twins() {
    let mut doc = compliant_demo();
    let report = report(&doc);
    assert!(diagrams(&doc, &report, None).iter().all(|d| !d.id.starts_with("seepage-gradient:")));
    let envelope = diagrams(&doc, &report, None).into_iter().find(|d| d.id == "lab-envelope:lab-T1").expect("envelope");
    assert!(envelope.series.iter().any(|s| matches!(s, scene::Series::Polyline { id, .. } if id == "characteristic")), "lower bound line");
    assert_eq!(envelope.table_twin.columns.len(), 3);
    doc.groundwater_level = 1.0;
    let report = report_of(&doc);
    let seepage = diagrams(&doc, &report, None).into_iter().find(|d| d.id == "seepage-gradient:ewall-E1").expect("gradient diagram");
    assert_eq!(seepage.table_twin.columns.len(), 3);
}

#[test]
fn the_hydraulic_failure_view_carries_a_pore_pressure_section_that_grows_with_depth() {
    let scene = scene_of(&super::model::showcase_demo());
    scene.validate().expect("valid");
    let section = scene.entities.iter().find(|e| e.entity.id == "ewall-E1:pore-pressure").expect("pore-pressure section");
    let scene::ColourChannel::Fields { active, fields } = &section.appearance.channel else { panic!("the section is coloured by its field") };
    assert_eq!(active, "pore-pressure");
    let scene::FieldSamples::Vertex { values } = &fields[0].samples else { panic!("vertex samples") };
    let scene::Geometry::Surface(surface) = &section.geometry else { panic!("a surface") };
    let scene::SurfaceShape::Grid { grid } = &surface.shape else { panic!("a grid") };
    assert_eq!(values.len(), (grid.rows * grid.columns) as usize);
    let columns = grid.columns as usize;
    assert!(values.iter().all(|u| u.is_finite() && *u >= 0.0));
    assert!(values[columns * (grid.rows as usize - 1)..].iter().sum::<f64>() > values[..columns].iter().sum::<f64>(), "the pressure rises with depth");
    assert!(scene.legends.iter().any(|l| l.id == "pore-pressure"));
}

fn report_of(doc: &En1997Snapshot) -> norm::CheckReport {
    report(doc)
}

#[test]
fn showcase_demo_shows_the_seepage_streamlines_and_the_gradient_diagram() {
    let doc = super::model::showcase_demo();
    let scene = scene_of(&doc);
    assert!(scene.entities.iter().any(|e| e.entity.id.starts_with("ewall-E1:seepage-")));
    assert!(diagrams(&doc, &report(&doc), None).iter().any(|d| d.id == "seepage-gradient:ewall-E1"));
}

fn fold_into(document: &mut En1997Snapshot, commit: &scene::projection_kit::PlacementCommit) {
    let mut rows = scene::projection_kit::placements_of(document);
    commit.apply(&mut rows);
    document.placements = rows.into_iter().map(|row| model::Placement { id: row.entity_id, x: row.origin.x, y: row.origin.y, z: row.origin.z, yaw: row.yaw }).collect();
}

#[test]
fn dragging_an_entity_commits_one_undoable_event_and_the_reload_stands_at_the_same_pose() {
    use scene::projection_kit::{PlacementCommit, PlacementMove};
    let mut document = compliant_demo();
    let id = "footing-F1".to_string();
    let before = scene::projection_kit::placements_of(&document);
    assert!(before.is_empty());
    let target = scene::Vec3::new(7.5, -2.0, 0.0);
    let commit = PlacementCommit::of(&before, &[PlacementMove { entity_id: id.clone(), origin: target.clone(), yaw: 0.75 }]);
    assert_eq!(commit.ops.len(), 1);
    fold_into(&mut document, &commit);
    let reloaded = scene::projection_kit::placements_of(&document);
    assert_eq!(reloaded.len(), 1);
    assert_eq!(Layout::of(&document, &reloaded).pose(&id).origin.clone(), target);
    let again = PlacementCommit::of(&reloaded, &[PlacementMove { entity_id: id.clone(), origin: target, yaw: 0.75 }]);
    assert!(again.is_empty());
    fold_into(&mut document, &commit.inverse(&before));
    assert_eq!(scene::projection_kit::placements_of(&document), before);
}

/// 🧅 The pressure bulb radii of a 2.5 m square footing under 88 kPa against the Boussinesq integral of the point-load kernel (`scipy.integrate.dblquad`, `scipy.optimize.brentq`): centre line and the 45 degree ray.
#[test]
fn pressure_bulb_radii_match_the_scipy_boussinesq_integral() {
    let (q, b) = (88_000.0, 2.5);
    for (ratio, centre, diagonal) in [(0.5, 1.821_506_835_780_593, 1.552_144_928_988_428_4), (0.2, 3.507_793_382_274_176, 2.497_858_717_848_212), (0.1, 5.218_444_509_576_822, 3.446_804_017_467_877)] {
        close(mech::bulb_radius(q, b, b, 0.0, 0.0, ratio), centre, 2e-4);
        close(mech::bulb_radius(q, b, b, std::f64::consts::FRAC_PI_4, 0.0, ratio), diagonal, 2e-4);
    }
    let (vertices, triangles) = mech::bulb_mesh(q, b, b, 0.2, 24, 10);
    assert!(triangles.iter().flatten().all(|i| (*i as usize) < vertices.len()) && vertices.iter().all(|v| v.iter().all(|c| c.is_finite())));
    assert!(vertices.iter().all(|v| v[2] <= 1e-9), "the bulb lies below the base plane");
}

/// 🕳️ The strength fitted to a laboratory series against `scipy.stats.linregress`: direct shear (GI-12) and the stress path of a triaxial series (GI-11, sin phi = tan alpha).
#[test]
fn laboratory_strength_matches_the_scipy_regression() {
    use checks::catalogue::investigation::lab_strength;
    let (phi, c) = lab_strength("directShear", &[50e3, 100e3, 200e3], &[32.5e3, 65e3, 130e3]).expect("shear fit");
    close(phi, 33.023_867_555_796_65, 1e-9);
    assert!(c.abs() < 1e-6);
    let (phi, c) = lab_strength("triaxial", &[100e3, 200e3, 300e3], &[60e3, 100e3, 140e3]).expect("triaxial fit");
    close(phi, 23.578_178_478_201_83, 1e-9);
    close(c, 21_821.789_023_599_238, 1e-9);
    assert!(lab_strength("oedometer", &[1.0, 2.0, 3.0], &[1.0, 2.0, 3.0]).is_none() && lab_strength("directShear", &[1.0, 2.0], &[1.0, 2.0]).is_none());
}

/// 🌍️ Wave 4 visuals: borehole log and sample per laboratory series, the pressure bulb under the footing, the lateral deflection line of the pile, the live factor of safety on every checked entity and, with a seismic coefficient, the Mononobe-Okabe wedge at the angle of the independent maximisation.
#[test]
fn geotechnical_visuals_stratigraphy_bulb_deflection_line_seismic_wedge_and_factor_of_safety() {
    let doc = super::model::showcase_demo();
    let scene = scene_of(&doc);
    scene.validate().unwrap_or_else(|violations| panic!("{violations:?}"));
    let ids: Vec<&str> = scene.entities.iter().map(|e| e.entity.id.as_str()).collect();
    for expected in ["lab-T1:log", "lab-T1:sample", "footing-F1:bulb-50", "footing-F1:bulb-20", "footing-F1:bulb-10", "pile-P1:lateral-line", "pile-P1:load-h", "wall-W1:seismic-wedge", "wall-W1:resultant-seismic"] {
        assert!(ids.contains(&expected), "missing {expected}");
    }
    let labels: BTreeMap<&str, &str> = scene.annotations.iter().filter_map(|a| match a {
        scene::Annotation::Label { id, text, .. } => Some((id.as_str(), text.en.as_str())),
        _ => None,
    }).collect();
    for entity in ["footing-F1", "pile-P1", "wall-W1", "slope-S1", "upl-U1", "ewall-E1", "anchor-A1", "group-G1", "lab-T1", "water-gravel"] {
        let text = labels.get(format!("{entity}:fos").as_str()).unwrap_or_else(|| panic!("{entity} needs a factor of safety label"));
        assert!(text.starts_with("FoS"), "{text}");
    }
    let wall = scene.entities.iter().find(|e| e.entity.id == "wall-W1").and_then(|e| e.utilisation.as_ref()).expect("wall utilisation");
    assert!(labels["wall-W1:fos"].contains(&format!("{:.2}", safety::factor_of_safety(wall.ratio))), "{} for ratio {}", labels["wall-W1:fos"], wall.ratio);
    close(safety::factor_of_safety(0.25), 4.0, 1e-12);
    assert!(safety::factor_of_safety(0.0).is_infinite());
    let state = checks::lateral_and_seismic::seismic_wall(&doc.retaining_walls[0], doc.retaining_walls[0].base_width).expect("seismic state");
    assert!((state.wedge_deg - 47.907_640_844_827_18).abs() < 1e-3 && (state.thrust.e_ae - 60_604.029_397_552_96).abs() < 0.01 * 60_604.0 * 1e-4 + 1.0, "{} deg, {} N/m", state.wedge_deg, state.thrust.e_ae);
    let compliant = scene_of(&compliant_demo());
    assert!(!compliant.entities.iter().any(|e| e.entity.id.ends_with(":seismic-wedge")), "k_h = 0 draws no wedge");
}

//#region 🗂️CasesIn3d
fn case_row(id: &str, entry: &str, numbers: &[(&str, f64)]) -> model::CaseRow {
    model::CaseRow { id: id.into(), label_en: format!("Case {id}"), label_de: format!("Fall {id}"), entry_id: entry.into(), numbers: numbers.iter().map(|(key, value)| model::CaseNumber { id: (*key).into(), value: *value }).collect(), texts: Vec::new() }
}

fn cased() -> En1997Snapshot {
    let mut doc = compliant_demo();
    doc.cases = vec![
        case_row("case-footing", "GEO-F02", &[("b", 2.4), ("l", 3.0), ("d", 0.8), ("v_d", 1.0e6)]),
        case_row("case-pile", "PILE-04", &[("d", 0.6), ("l", 12.0), ("h_d", 120_000.0)]),
        case_row("case-wall", "RW-11", &[("t_stem", 0.4), ("h", 4.0), ("b", 3.2), ("t_base", 0.5), ("a_toe", 0.6), ("q", 10_000.0)]),
        case_row("case-slope", "SLOPE-04", &[("h", 6.0), ("beta_deg", 45.0)]),
        case_row("case-group", "PILE-06", &[("d", 0.5), ("m", 3.0), ("n", 4.0), ("s", 1.8), ("v", 6.0e6)]),
        case_row("case-nominal", "PF-01", &[]),
    ];
    doc
}

fn parts_of<'a>(scene: &'a scene::StructuralScene, id: &str) -> Vec<&'a scene::SceneEntity> {
    scene.entities.iter().filter(|e| e.entity.id == id || e.entity.id.starts_with(&format!("{id}:"))).collect()
}

#[test]
fn every_catalogue_entry_stands_in_the_scene_as_its_own_entity() {
    let doc = cased();
    let scene = scene_of(&doc);
    scene.validate().unwrap_or_else(|violations| panic!("{violations:?}"));
    for case in &doc.cases {
        assert!(scene.entities.iter().any(|e| e.entity.id == case.id), "{} is not drawn", case.id);
    }
    let ids: Vec<&str> = scene.entities.iter().map(|e| e.entity.id.as_str()).collect();
    assert!(ids.contains(&"case-footing:soil-0") && ids.contains(&"case-pile:soil-0") && ids.contains(&"case-wall:backfill") && ids.contains(&"case-group:cap"), "{ids:?}");
    assert!(entities_of(&doc).iter().any(|e| e.id == "case-nominal"));
    assert_eq!(entity_kind_of(&doc, "case-footing").map(|k| k.collection), Some("cases"));
    let scene::SolidShape::Box { size, frame } = (match &entity(&scene, "case-footing").geometry {
        scene::Geometry::Solid(solid) => solid.shape.clone(),
        other => panic!("{other:?}"),
    }) else { panic!("the footing pad is a block") };
    close(size.x, 2.4, 1e-9);
    close(size.y, 3.0, 1e-9);
    close(frame.origin.z, -0.8, 1e-9);
    assert_eq!(parts_of(&scene, "case-group").iter().filter(|e| e.entity.id.contains("pile")).count(), 11, "twelve piles: the body and eleven more");
}

fn entity<'a>(scene: &'a scene::StructuralScene, id: &str) -> &'a scene::SceneEntity {
    scene::projection_kit::test_support::entity(scene, id)
}

#[test]
fn a_case_is_placed_by_its_placement_and_by_default_in_its_own_row() {
    let doc = cased();
    let default = Layout::of(&doc, &[]);
    assert!(default.pose("case-footing").origin.y >= layout::ROW_CASES, "cases start in their own row");
    assert!(default.pose("case-wall").origin.x > default.pose("case-footing").origin.x, "cases follow the row index");
    let moved = Layout::of(&doc, &[Placement { entity_id: "case-pile".into(), origin: scene::Vec3::new(5.0, -7.0, 0.0), yaw: 0.5 }]);
    close(moved.pose("case-pile").origin.x, 5.0, 1e-9);
    close(moved.pose("case-pile").yaw, 0.5, 1e-9);
}

#[test]
fn geometric_inputs_have_handles_that_write_the_case_numbers() {
    let doc = cased();
    let layout = Layout::of(&doc, &[]);
    let all = handles::all(&doc, &layout);
    assert_handle_laws(&all);
    let tree = semio_framework_value::ToValue::to_value(&doc);
    let of = |entity: &str| all.iter().filter(|h| h.entity.id == entity).map(|h| h.id.clone()).collect::<Vec<_>>();
    assert_eq!(of("case-footing"), ["case-footing:width", "case-footing:length", "case-footing:embed", "case-footing:load", "case-footing:move-x", "case-footing:move-y"]);
    assert_eq!(of("case-nominal"), ["case-nominal:move-x", "case-nominal:move-y"]);
    let width = all.iter().find(|h| h.id == "case-footing:width").expect("width handle");
    assert_eq!(width.writes[0].path, "cases[id=case-footing].numbers[id=b].value");
    close(width.writes[0].map.base, 2.4, 1e-9);
    close(width.writes[0].map.k, 2.0, 1e-9);
    let embed = all.iter().find(|h| h.id == "case-footing:embed").expect("embed handle");
    close(embed.writes[0].map.k, -1.0, 1e-9);
    for handle in &all {
        for write in handle.writes.iter().filter(|w| !w.path.starts_with("placements[")) {
            semio_s_plugin_norm_app_surface::get_value_at_path(&tree, &write.path).unwrap_or_else(|e| panic!("handle {} writes {}: {e}", handle.id, write.path));
        }
    }
}

#[test]
fn the_case_obeys_the_case_and_dimension_laws() {
    let doc = cased();
    let scene = scene_of(&doc);
    let layout = Layout::of(&doc, &[]);
    let all = handles::all(&doc, &layout);
    let ids: Vec<&str> = doc.cases.iter().map(|c| c.id.as_str()).collect();
    scene::projection_kit::test_support::assert_case_laws(&ids, &scene, &all);
    scene::projection_kit::test_support::assert_case_dimension_laws::<checks::catalogue::En1997Cases>(&doc.cases);
}

#[test]
fn the_case_glyph_carries_the_utilisation_of_its_checks_and_its_handles() {
    let doc = cased();
    let scene = scene_of(&doc);
    let footing = scene.entities.iter().find(|e| e.entity.id == "case-footing").expect("footing case");
    assert!(footing.handles.contains(&"case-footing:width".to_string()));
    assert_eq!(footing.layer, "cases");
    assert!(footing.utilisation.is_some(), "the glyph is bound to the checks addressed to cases[id=case-footing]");
}
//#endregion 🗂️CasesIn3d

#[test]
fn every_example_and_showcase_projection_obeys_the_scene_diagram_and_handle_laws() {
    for document in [compliant_demo(), noncompliant_demo(), super::model::showcase_demo()] {
        let scene = scene_of(&document);
        let report = report(&document);
        let selection: Vec<scene::EntityRef> = scene.entities.iter().map(|e| e.entity.clone()).collect();
        scene::projection_kit::test_support::assert_projection_laws(&scene, &diagrams(&document, &report, None), &handles::handles(&document, &[], &selection));
    }
}

#[test]
fn the_default_places_of_the_cases_overlap_nothing() {
    let doc = cased();
    let layout = Layout::of(&doc, &[]);
    let pose = |id: &str| doc.cases.iter().any(|c| c.id == id).then(|| scene::projection_kit::design_case::forms::CasePose { origin: layout.pose(id).origin, yaw: 0.0 });
    scene::projection_kit::test_support::assert_case_layout_clear::<checks::catalogue::En1997Cases>(&doc.cases, &pose, &scene::projection_kit::design_case::forms::no_extras, &[]);
}

#[test]
fn the_concrete_class_of_a_pile_is_a_choice_stepper_writing_its_typed_field_with_every_class() {
    let doc = compliant_demo();
    let selected = handles::handles(&doc, &[], &[scene::EntityRef::new(scene::EntityKind::Group, "pile-P1", "piles[id=pile-P1]")]);
    assert_handle_laws(&selected);
    let grip = selected.iter().find(|h| h.id == "pile-P1:concrete-class").expect("concrete class grip");
    assert_eq!(grip.writes.len(), 1);
    assert_eq!(grip.writes[0].path, "piles[id=pile-P1].concreteClass");
    let Some(scene::Quantise::Token { tokens }) = &grip.writes[0].quantise else { panic!("token quantiser") };
    let listed = tokens.iter().map(|t| t.token.as_str()).collect::<Vec<_>>();
    assert_eq!(listed, super::model::PileConcreteClass::ALL.iter().map(|class| class.token()).collect::<Vec<_>>());
}

#[test]
fn slip_circle_diagrams_and_slices_follow_the_checked_circle_while_the_centre_is_dragged() {
    use checks::catalogue::slopes as slope_checks;
    let mut doc = compliant_demo();
    let column = |d: &scene::Diagram2d, i: usize| d.table_twin.columns[i].values.clone();
    for (x, z, r) in [(0.0, 0.0, 0.0), (4.0, 12.0, 14.0), (3.0, 12.0, 14.0)] {
        doc.slopes[0].circle_centre_x = x;
        doc.slopes[0].circle_centre_z = z;
        doc.slopes[0].circle_radius = r;
        let list = diagrams(&doc, &report(&doc), Some("slope-S1"));
        let ids: Vec<&str> = list.iter().map(|d| d.id.as_str()).collect();
        assert_eq!(ids[..2], ["slip-forces:slope-S1", "slip-centre:slope-S1"]);
        assert!(ids.len() == 2 || ids[2] == "spencer-equilibrium:slope-S1", "{ids:?}");
        for d in &list {
            d.validate().unwrap_or_else(|v| panic!("{} {v:?}", d.id));
        }
        let (proto, tan_phi, c) = slopes_proto(&doc);
        let (circle, factor) = slopes::current_circle(&doc.slopes[0], &proto, tan_phi, c).expect("circle");
        let analysis = slope_checks::bishop_forces(&circle, tan_phi, c, mech::SLICES).expect("analysis");
        close(analysis.factor, factor, 1e-12);
        let (driving, shear) = (column(&list[0], 1).iter().sum::<f64>(), column(&list[0], 2).iter().sum::<f64>());
        close(shear, driving, 1e-6);
        let scene::Marker { at, .. } = &list[1].markers[0];
        close(at.y, factor, 1e-12);
        let drawn = mech::slip_slices(&circle, mech::SLICES);
        let checked = slope_checks::slices(&circle, mech::SLICES).expect("slices");
        assert_eq!(drawn.len(), checked.len());
        for (a, b) in drawn.iter().zip(&checked) {
            assert!(a.weight == b.w && a.pore == b.u && a.alpha == b.alpha && a.x0 == b.x0 && a.x1 == b.x1);
        }
    }
    let at = |x: f64| {
        doc.slopes[0].circle_centre_x = x;
        let (proto, tan_phi, c) = slopes_proto(&doc);
        slope_checks::bishop(&slope_checks::Circle { xc: x, yc: 12.0, r: 14.0, ..proto }, tan_phi, c, mech::SLICES)
    };
    assert_ne!(at(4.0), at(3.0));
}

#[test]
fn the_hydraulic_failure_view_carries_a_gradient_section_capped_by_the_critical_gradient() {
    let scene = scene_of(&super::model::showcase_demo());
    scene.validate().expect("valid");
    let section = scene.entities.iter().find(|e| e.entity.id == "ewall-E1:hydraulic-gradient").expect("gradient section");
    let scene::ColourChannel::Fields { active, fields } = &section.appearance.channel else { panic!("the section is coloured by its field") };
    assert_eq!(active, "hydraulic-gradient");
    let scene::FieldSamples::Vertex { values } = &fields[0].samples else { panic!("vertex samples") };
    let scene::Geometry::Surface(surface) = &section.geometry else { panic!("a surface") };
    let scene::SurfaceShape::Grid { grid } = &surface.shape else { panic!("a grid") };
    assert_eq!(values.len(), (grid.rows * grid.columns) as usize);
    assert!(values.iter().all(|i| i.is_finite() && *i >= 0.0));
    let legend = scene.legends.iter().find(|l| l.id == "hydraulic-gradient").expect("legend");
    let scene::LegendScale::Scalar { map, .. } = &legend.scale else { panic!("scalar legend") };
    let doc = super::model::showcase_demo();
    let layer = mech::layer_at(&doc.layers, doc.embedded_walls[0].retained_height + 0.5 * doc.embedded_walls[0].embedment).expect("layer");
    close(map.max, layer.gamma_prime / mech::GAMMA_W, 1e-12);
}

/// 🔺️ The heave prism at the wall toe carries the `heave_prism` ratio of HYD-03 as its utilisation and the seepage arrows point down on the retained side and up on the excavation side, growing with the gradient.
#[test]
fn the_heave_prism_takes_the_check_utilisation_and_the_seepage_arrows_follow_the_flow_net() {
    let doc = super::model::showcase_demo();
    let (scene, checks) = (scene_of(&doc), report(&doc));
    scene.validate().expect("valid");
    let prism = scene.entities.iter().find(|e| e.entity.id == "ewall-E1:heave-prism").expect("heave prism");
    let check = checks.checks.iter().find(|c| c.subject.entity_id == "ewall-E1" && c.id.contains("hyd-03") && c.id.ends_with("heave_prism")).expect("heave check");
    close(prism.utilisation.as_ref().expect("utilisation").ratio, check.utilization, 1e-12);
    let arrows: Vec<_> = scene.entities.iter().filter(|e| e.entity.id.starts_with("ewall-E1:flow-arrow-") && !e.entity.id.ends_with("-head")).collect();
    assert!(arrows.len() >= 8, "{} arrows", arrows.len());
    let direction = |id: &str| {
        let entity = scene.entities.iter().find(|e| e.entity.id == id).unwrap_or_else(|| panic!("{id}"));
        let scene::Geometry::Solid(solid) = &entity.geometry else { panic!("a solid") };
        let scene::SolidShape::Cylinder { frame, .. } = &solid.shape else { panic!("a cylinder") };
        frame.z.z
    };
    assert!(direction("ewall-E1:flow-arrow-1-3") > 0.0, "up on the excavation side next to the wall");
    assert!(direction("ewall-E1:flow-arrow-1-2") < 0.0, "down on the retained side next to the wall");
}

/// 📉️ A single footing already shows its settlement trough: the elastic series is Love's rectangle on the equivalent half-space at the footing centre, the footing edges are marked, the 3D dish carries the undeformed ground ghost, and widening the footing changes the diagram and the ghost bounds (the settlement unit recomputes both).
#[test]
fn the_settlement_trough_under_a_footing_is_live_and_the_dish_carries_a_ghost() {
    let mut doc = compliant_demo();
    let footing = doc.footings[0].clone();
    let profile = |doc: &En1997Snapshot| diagrams(doc, &report(doc), Some(&footing.id)).into_iter().find(|d| d.id == format!("settlement-profile:{}", footing.id)).expect("profile");
    let first = profile(&doc);
    let scene::Series::Polyline { points, .. } = &first.series[0] else { panic!("elastic series") };
    let centre = points.iter().min_by(|a, b| a.x.abs().total_cmp(&b.x.abs())).expect("point");
    let (nu, e, _) = checks::part_1::elastic_equivalent(&doc.layers, footing.width, footing.embedment).expect("equivalent half-space");
    close(-centre.y / 1000.0, checks::catalogue::settlement::displacement_rect_at(mech::service_pressure(&footing), footing.width, footing.length, 0.0, 0.0, nu, e), 1e-9);
    assert!(centre.y <= 0.0 && first.markers.len() == 3 && first.table_twin.columns.len() == 3);
    let scene = scene_of(&doc);
    scene.validate().expect("valid");
    assert!(scene.entities.iter().any(|e| e.entity.id == "settlement-field.ghost"), "undeformed ghost");
    doc.footings[0].width *= 1.5;
    let wider = profile(&doc);
    let scene::Series::Polyline { points: wider_points, .. } = &wider.series[0] else { panic!("elastic series") };
    let wider_centre = wider_points.iter().min_by(|a, b| a.x.abs().total_cmp(&b.x.abs())).expect("point");
    assert!((wider_centre.y - centre.y).abs() > 1e-9 && wider_points.last().map(|p| p.x) > points.last().map(|p| p.x), "the trough follows the footing edit");
}

#[test]
fn uplift_anchor_pile_group_and_head_diagrams_are_pure_projections_that_follow_the_document() {
    let mut doc = compliant_demo();
    let all = |doc: &En1997Snapshot| diagrams(doc, &report(doc), None);
    let list = all(&doc);
    let find = |list: &[scene::Diagram2d], id: &str| list.iter().find(|d| d.id == id).unwrap_or_else(|| panic!("{id}")).clone();
    let twin = |d: &scene::Diagram2d, i: usize| d.table_twin.columns[i].values.clone();
    let uplift = find(&list, "uplift-stack:upl-U1");
    close(twin(&uplift, 1)[0], 900.0, 1e-9);
    close(twin(&uplift, 1)[1], 550.0, 1e-9);
    let anchors = find(&list, "anchor-layout:ewall-E1");
    close(twin(&anchors, 3)[0], 8.0, 1e-12);
    close(twin(&anchors, 4)[0], 6.0, 1e-12);
    assert!(anchors.markers.iter().all(|m| m.utilisation.is_some()));
    let group = find(&list, "pile-group-plan:group-G1");
    let reactions = twin(&group, 3);
    assert_eq!(reactions.len(), 4);
    close(reactions.iter().sum::<f64>(), 1800.0, 1e-9);
    let head = find(&list, "head-profile:water-gravel");
    let heads = twin(&head, 1);
    close(heads[0], 1.0, 1e-12);
    close(*heads.last().unwrap(), 0.0, 1e-12);
    for d in list.iter().filter(|d| ["uplift-stack:", "anchor-layout:", "pile-group-plan:", "head-profile:"].iter().any(|p| d.id.starts_with(p))) {
        d.validate().unwrap_or_else(|v| panic!("{} {v:?}", d.id));
        assert!(d.table_twin.columns.iter().all(|c| c.values.len() == d.table_twin.columns[0].values.len()), "{}", d.id);
    }
    doc.uplift_cases[0].permanent_destabilizing += 100_000.0;
    doc.pile_groups[0].columns = 3;
    let edited = all(&doc);
    assert_eq!(find(&edited, "pile-group-plan:group-G1").table_twin.columns[0].values.len(), 6);
    close(twin(&find(&edited, "uplift-stack:upl-U1"), 1)[1], 650.0, 1e-9);
    for id in ["anchor-layout:ewall-E1", "head-profile:water-gravel"] {
        assert_eq!(find(&list, id), find(&edited, id), "{id} is untouched by the uplift and pile edits");
    }
    assert_eq!(edited, all(&doc.clone()), "diagrams are a pure function of the document");
}

/// 🕳️ Every footing carries its ghost column of the required investigation depth z_a = max(6 m, 2 B) (EN 1997-2 Annex B.3, GI-01): worked example B = 4 m gives 8 m, B = 2 m gives the 6 m floor; the column follows the footing width on every edit.
#[test]
fn every_footing_carries_a_ghost_column_of_the_required_investigation_depth() {
    let mut doc = compliant_demo();
    let id = format!("{}:investigation-ghost", doc.footings[0].id);
    assert!((checks::part_2::min_investigation_depth_m(4.0) - 8.0).abs() < 1e-12 && (checks::part_2::min_investigation_depth_m(2.0) - 6.0).abs() < 1e-12);
    let scene = scene_of(&doc);
    scene.validate().expect("valid");
    let geometry = |scene: &scene::StructuralScene| scene.entities.iter().find(|e| e.entity.id == id).map(|e| format!("{:?}", e.geometry)).expect("ghost column");
    let before = geometry(&scene);
    doc.footings[0].width = 9.0;
    assert_ne!(geometry(&scene_of(&doc)), before, "the ghost follows the footing width");
}

/// 🕳️ Investigation points are columns at their x/y from the surface to their depth, each with a profile ribbon of its readings; the grips write position and depth to the row; the ghost of the required depth names the point that reaches it (GI-01); the q_c / N / c_u soil profile diagrams follow every edit of a reading, a layer or a basis.
#[test]
fn investigation_points_are_columns_with_a_reading_ribbon_grips_and_a_live_soil_profile() {
    let mut doc = super::model::showcase_demo();
    let scene = scene_of(&doc);
    scene.validate().expect("valid");
    let entity = |scene: &scene::StructuralScene, id: &str| scene.entities.iter().find(|e| e.entity.id == id).map(|e| format!("{:?}", e.geometry)).unwrap_or_else(|| panic!("no entity {id}"));
    let (column, ribbon) = (entity(&scene, "cpt-1"), entity(&scene, "cpt-1:ribbon"));
    assert!(column.contains("Cylinder") && ribbon.contains("Grid"), "column and ribbon geometry");
    let selected = handles::handles(&doc, &[], &[scene::EntityRef::new(scene::EntityKind::Solid, "cpt-1", "investigations[id=cpt-1]")]);
    assert_handle_laws(&selected);
    let paths: Vec<&str> = selected.iter().flat_map(|h| h.writes.iter().map(|w| w.path.as_str())).collect();
    assert_eq!(paths, ["investigations[id=cpt-1].positionX", "investigations[id=cpt-1].positionY", "investigations[id=cpt-1].depth"]);
    let ghost = scene.entities.iter().find(|e| e.entity.id.ends_with(":investigation-ghost")).expect("ghost");
    assert!(ghost.accessible_detail.en.contains("by cpt-1"), "the ghost names the point that reaches z_a");
    let profile = |doc: &En1997Snapshot, id: &str| diagrams(doc, &report(doc), None).into_iter().find(|d| d.id == id).unwrap_or_else(|| panic!("no diagram {id}"));
    let (qc, n) = (profile(&doc, "ground-profile:qc"), profile(&doc, "ground-profile:n"));
    qc.validate().expect("valid q_c profile");
    n.validate().expect("valid N profile");
    assert_eq!(qc.table_twin.columns[0].values.len(), 11);
    doc.investigations[0].readings[2].measurement *= 2.0;
    doc.investigations[0].position_x += 3.0;
    let edited = scene_of(&doc);
    assert_ne!(entity(&edited, "cpt-1"), column, "the column follows the position");
    assert_ne!(entity(&edited, "cpt-1:ribbon"), ribbon, "the ribbon follows the reading");
    assert_ne!(profile(&doc, "ground-profile:qc").table_twin.columns[0].values, qc.table_twin.columns[0].values, "the profile follows the reading");
}

/// 🧮️ The c_u profile has one polyline per CPT point and a dashed characteristic line per layer with a derived basis: it appears only with a basis (or a cohesive layer) and follows the layer boundary.
#[test]
fn the_cu_profile_carries_the_characteristic_line_of_a_layer_with_a_derived_basis() {
    use super::model::{CharacteristicBasis, CharacteristicParameter, ParameterBasis};
    let mut doc = super::model::showcase_demo();
    let cu = |doc: &En1997Snapshot| diagrams(doc, &report(doc), None).into_iter().find(|d| d.id == "ground-profile:cu");
    let lines = |d: &scene::Diagram2d| d.series.iter().filter(|s| matches!(s, scene::Series::Polyline { id, .. } if id.ends_with("c_u,k"))).count();
    assert!(cu(&doc).is_some_and(|d| lines(&d) == 0), "sand is not cohesive and has no basis: no characteristic c_u line");
    let layer = doc.layers[0].id.clone();
    doc.characteristic_bases = vec![CharacteristicBasis { id: "basis-1".into(), layer_id: layer, parameter: CharacteristicParameter::UndrainedStrength, basis: ParameterBasis::Derived }];
    let with_basis = cu(&doc).expect("c_u profile");
    with_basis.validate().expect("valid");
    assert_eq!(lines(&with_basis), 1);
}

#[test]
fn characteristic_bases_and_investigations_are_registered_kinds_that_link_to_their_layer() {
    use super::model::{CharacteristicBasis, CharacteristicParameter, ParameterBasis};
    let mut doc = super::model::showcase_demo();
    let layer = doc.layers[0].id.clone();
    doc.characteristic_bases = vec![CharacteristicBasis { id: "basis-1".into(), layer_id: layer.clone(), parameter: CharacteristicParameter::UndrainedStrength, basis: ParameterBasis::Derived }];
    for (id, collection) in [("characteristicBasis", "characteristicBases"), ("investigation", "investigations")] {
        let hit = ENTITY_KINDS.iter().find(|k| k.id == id).expect("kind");
        assert_eq!(hit.collection, collection);
        assert!(!hit.label_en.is_empty() && !hit.label_de.is_empty());
    }
    assert_eq!(entity_kind_of(&doc, "basis-1").map(|k| k.collection), Some("characteristicBases"));
    doc.placements = vec![super::model::Placement { id: "orphan".into(), x: 0.0, y: 0.0, z: 0.0, yaw: 0.0 }];
    assert_eq!(entity_kind_of(&doc, "orphan").map(|k| k.collection), Some("placements"));
    assert!(entity_dependencies(&doc, "basis-1").contains(&layer));
    assert!(entity_dependencies(&doc, &layer).contains(&"basis-1".to_string()));
    assert_eq!(entity_kind_of(&doc, "cpt-1").map(|k| k.collection), Some("investigations"));
    assert!(entities_of(&doc).iter().any(|e| e.id == "cpt-1"));
}

/// ⏳️ The consolidation curve of a footing takes its coefficient from the oedometer series (`cv = T90 h² / t90`), its t90 marker sits at the matching day, and the series agrees with the finite-volume fixture.
#[test]
fn consolidation_time_diagram_uses_the_oedometer_cv_and_the_series_matches_the_blessed_pde() {
    let mut doc = compliant_demo();
    let test = doc.lab_tests.iter_mut().next().expect("lab test");
    test.kind = super::model::LabTestKind::Oedometer;
    test.drainage_height = 0.01;
    test.consolidation_time = 1800.0;
    let footing = doc.footings[0].clone();
    let list = diagrams(&doc, &report(&doc), None);
    let curve = list.iter().find(|d| d.id == format!("consolidation-time:{}", footing.id)).expect("consolidation diagram");
    curve.validate().expect("valid");
    let path = 2.0 * footing.width.max(footing.length);
    let t90_days = path * path * 1800.0 / (0.01 * 0.01 * tracks::DAY);
    let marker = curve.markers.iter().find(|m| m.id.ends_with(":t90")).expect("t90 marker");
    close(marker.at.x, t90_days, 1e-9);
    assert!(curve.description.en.contains("oedometer series") && curve.table_twin.columns.iter().all(|c| c.values.len() == curve.table_twin.columns[0].values.len()));
    let settlement = &curve.table_twin.columns[3].values;
    assert!(settlement.windows(2).all(|w| w[1] <= w[0] + 1e-12) && settlement[0] <= 0.0);
    close(settlement[settlement.len() - 1].abs(), -curve.markers.iter().find(|m| m.id.ends_with(":t50")).map_or(0.0, |m| m.at.y) * 2.0, 5e-3);
    let fixture: Value = serde_json::from_str(&std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🧪️tests/🧫️fixtures/⏳️consolidation/🔣️.json")).expect("fixture")).expect("json");
    for row in fixture["rows"].as_array().expect("rows") {
        close(tracks::degree(row["timeFactor"].as_f64().expect("tv")), row["degree"].as_f64().expect("degree"), 1e-3);
    }
    let mut slower = doc.clone();
    slower.lab_tests[0].consolidation_time = 3600.0;
    let slow = diagrams(&slower, &report(&slower), None);
    let later = slow.iter().find(|d| d.id == curve.id).and_then(|d| d.markers.iter().find(|m| m.id.ends_with(":t90"))).expect("later");
    close(later.at.x, 2.0 * t90_days, 1e-9);
}

/// 🪵️ The pile group draws its equivalent raft as a ghost and as a section; the raft follows PILE-12 (outer pile edges plus the 1:4 spread over two thirds of the length) and the group settlement is linear in the cap load.
#[test]
fn pile_group_carries_an_equivalent_raft_ghost_and_a_section_with_linear_settlement() {
    let mut doc = compliant_demo();
    let scene = scene_of(&doc);
    scene.validate().unwrap_or_else(|violations| panic!("{violations:?}"));
    assert!(scene.entities.iter().any(|e| e.entity.id == "group-G1:equivalent-raft"), "raft ghost");
    let group = doc.pile_groups[0].clone();
    let pile = doc.piles.iter().find(|p| p.id == group.pile_id).expect("pile").clone();
    let list = diagrams(&doc, &report(&doc), None);
    let section = list.iter().find(|d| d.id == "pile-group-raft:group-G1").expect("raft section");
    section.validate().expect("valid");
    let grow = 2.0 * mech::RAFT_SPREAD * mech::RAFT_DEPTH_FACTOR * pile.length;
    close(section.table_twin.columns[0].values[0], (f64::from(group.columns) - 1.0) * group.spacing + pile.diameter + grow, 1e-12);
    close(section.table_twin.columns[2].values[0], mech::RAFT_DEPTH_FACTOR * pile.length, 1e-12);
    let single = mech::group_settlement(&group, &pile, &doc.layers);
    assert!(single > 0.0);
    doc.pile_groups[0].cap_vertical *= 2.0;
    close(mech::group_settlement(&doc.pile_groups[0], &pile, &doc.layers), 2.0 * single, 1e-9);
}

/// ⛰️ Spencer diagram: whenever the catalogue finds a root for the current circle the diagram exists, its marker is that root, and both equilibrium curves agree there to the grid resolution.
#[test]
fn spencer_diagram_marks_the_catalogue_root_where_moment_and_force_curves_meet() {
    use super::checks::catalogue::slopes as slope_checks;
    let doc = compliant_demo();
    let (proto, tan_phi, c) = slopes_proto(&doc);
    let (circle, _) = slopes::current_circle(&doc.slopes[0], &proto, tan_phi, c).expect("circle");
    let Some((factor, lambda)) = slope_checks::spencer(&circle, tan_phi, c, 60) else { return };
    let list = diagrams(&doc, &report(&doc), Some("slope-S1"));
    let d = list.iter().find(|d| d.id == "spencer-equilibrium:slope-S1").expect("spencer diagram");
    d.validate().expect("valid");
    let marker = &d.markers[0];
    close(marker.at.y, factor, 1e-12);
    close(marker.at.x, lambda.atan().to_degrees(), 1e-12);
    let (theta, moment, force) = (&d.table_twin.columns[0].values, &d.table_twin.columns[1].values, &d.table_twin.columns[2].values);
    let nearest = (0..theta.len()).min_by(|a, b| (theta[*a] - marker.at.x).abs().total_cmp(&(theta[*b] - marker.at.x).abs())).expect("rows");
    assert!((moment[nearest] - force[nearest]).abs() < 0.1 * factor, "{} vs {}", moment[nearest], force[nearest]);
}

/// 🧭️ Every check family of the spec trace has an owning collection that is a registered entity kind, and the longest prefix decides.
#[test]
fn every_check_family_names_a_registered_owner_collection() {
    assert!(!CHECK_OWNERS.is_empty());
    for (prefix, collection) in CHECK_OWNERS {
        assert!(prefix.starts_with("en1997."), "{prefix}");
        assert!(ENTITY_KINDS.iter().any(|k| k.collection == *collection), "{prefix} -> {collection}");
    }
    assert_eq!(check_owner("en1997.gi-10.cv"), Some("labTests"));
    assert_eq!(check_owner("en1997.anc-02.pull_out"), Some("anchors"));
    assert_eq!(check_owner("en1997.unknown.x"), None);
}

/// 🧭️ Every check of every demo document is drawn: it resolves to an entity row the scene projects, or to the global badge through a document-level family, and the focus of the outcome names the same row.
#[test]
fn every_check_outcome_routes_to_its_entity_or_the_document_badge() {
    for doc in [compliant_demo(), noncompliant_demo(), super::model::showcase_demo(), cased()] {
        let report = report(&doc);
        let scene = scene_of(&doc);
        assert!(!report.checks.is_empty());
        for check in &report.checks {
            let route = route_of(&doc, check).unwrap_or_else(|| panic!("check {} (subject {:?}) has no owner", check.id, check.subject.entity_id));
            match route {
                CheckRoute::Entity(id) => {
                    assert!(scene.entities.iter().any(|e| base_id(&e.entity.id) == id), "check {} routes to {id}, which the scene does not draw", check.id);
                    assert_eq!(focus_of_outcome(&doc, &report, &check.id).as_deref(), Some(id.as_str()));
                }
                CheckRoute::Document => assert!(focus_of_outcome(&doc, &report, &check.id).is_none(), "{}", check.id),
            }
        }
    }
}

/// 🧭️ The scene binds an entity to the governing check it owns: the wall entity carries the utilisation of its worst owned check.
#[test]
fn entity_utilisation_is_the_worst_check_the_owner_table_routes_to_it() {
    let doc = noncompliant_demo();
    let report = report(&doc);
    let scene = scene_of(&doc);
    for id in ["wall-W1", "pile-P1", "upl-U1", "slope-S1"] {
        let worst = report.checks.iter().filter(|c| c.utilization.is_finite() && c.status != norm::CheckStatus::NotApplicable && owner_entity(&doc, c).as_deref() == Some(id)).map(|c| c.utilization).fold(0.0, f64::max);
        let entity = scene.entities.iter().find(|e| e.entity.id == id).expect(id);
        let bound = entity.utilisation.as_ref().map_or(0.0, |b| b.ratio);
        assert!(bound <= worst + 1e-9 && bound > 0.0, "{id}: bound {bound} vs worst owned {worst}");
    }
}

/// 📈️ Every check family of the owner table is drawn by a dedicated diagram or by the outcome diagram of its entity, never by both and never by neither.
#[test]
fn every_check_family_is_drawn_by_a_dedicated_diagram_or_the_outcome_diagram() {
    for (prefix, _) in CHECK_OWNERS {
        let drawn = DRAWN_FAMILIES.contains(prefix);
        let outcome = OUTCOME_FAMILIES.contains(prefix);
        assert!(drawn != outcome, "{prefix}: drawn {drawn}, outcome {outcome}");
    }
    for prefix in DRAWN_FAMILIES.iter().chain(OUTCOME_FAMILIES) {
        assert!(CHECK_OWNERS.iter().any(|(p, _)| p == prefix), "{prefix} is not a check family of the owner table");
    }
}

/// 📊️ A pile with compression, tension, negative skin friction and buckling checks gets one outcome diagram with a bar per check, the limit line and a table twin.
#[test]
fn the_outcome_diagram_draws_a_bar_per_check_of_an_undiagrammed_family() {
    let doc = compliant_demo();
    let report = report(&doc);
    let list = diagrams(&doc, &report, None);
    let outcomes: Vec<&scene::Diagram2d> = list.iter().filter(|d| d.id.starts_with("check-outcomes:")).collect();
    assert!(!outcomes.is_empty(), "no outcome diagram");
    for d in outcomes {
        d.validate().expect("valid");
        let id = d.id.trim_start_matches("check-outcomes:");
        let expected = report.checks.iter().filter(|c| c.utilization.is_finite() && c.status != norm::CheckStatus::NotApplicable && OUTCOME_FAMILIES.iter().any(|p| c.id.starts_with(p)) && owner_entity(&doc, c).as_deref() == Some(id)).count();
        assert_eq!(d.table_twin.columns[1].values.len(), expected, "{}", d.id);
        assert_eq!(d.markers.len(), expected);
        assert!(d.series.iter().any(|s| matches!(s, scene::Series::Limit { value, .. } if (*value - 1.0).abs() < 1e-12)));
        assert!(!d.title.en.is_empty() && !d.title.de.is_empty());
    }
    let one = diagrams(&doc, &report, Some("no-such-row"));
    assert!(one.iter().all(|d| !d.id.starts_with("check-outcomes:")));
}

/// 🎚️ Every numeric field of a row has a viewport grip (geometric grip or gauge) or a documented exemption, and none has both.
#[test]
fn every_numeric_field_of_a_row_has_a_viewport_grip_or_a_documented_exemption() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../🧬️schema/📸️snapshot/🔣️.json");
    let schema: Value = serde_json::from_slice(&std::fs::read(path).expect("snapshot schema")).expect("json");
    let gauged = handles::gauges::coverage();
    let geometric: [(&str, &str, &[&str]); 12] = [
        ("layers", "SoilLayer", &["depthTop", "depthBottom"]),
        ("footings", "SpreadFoundation", &["width", "length", "embedment"]),
        ("piles", "Pile", &["length", "diameter", "count", "settlingThickness"]),
        ("retainingWalls", "RetainingWall", &["height", "embedment", "baseWidth", "stemThickness", "reinforcementSpacing"]),
        ("slopes", "Slope", &["height", "angleDeg", "width", "circleCentreX", "circleCentreZ", "circleRadius"]),
        ("upliftCases", "UpliftCase", &["permanentStabilizing", "permanentDestabilizing", "variableDestabilizing", "porePressure"]),
        ("embeddedWalls", "EmbeddedWall", &["retainedHeight", "embedment", "propDepth"]),
        ("anchors", "Anchor", &["freeLength", "fixedLength", "inclinationDeg"]),
        ("pileGroups", "PileGroup", &["rows", "columns", "spacing"]),
        ("labTests", "LabTest", &["stressIncrement"]),
        ("layerWaters", "LayerWater", &["wellWaterDepth", "piezometricDepth"]),
        ("investigations", "Investigation", &["positionX", "positionY", "depth"]),
    ];
    for (collection, def, grips) in geometric {
        let properties = schema["$defs"][def]["properties"].as_object().unwrap_or_else(|| panic!("{def}"));
        let gauges = gauged.iter().find(|(c, _)| *c == collection).map(|(_, f)| f.clone()).unwrap_or_default();
        let mut covered = 0;
        let mut numeric = 0;
        for (field, spec) in properties.iter().filter(|(_, v)| matches!(v["type"].as_str(), Some("number" | "integer"))) {
            numeric += 1;
            let exempt = handles::gauges::EXEMPT.contains(&(collection, field.as_str()));
            let by_grip = grips.contains(&field.as_str());
            let by_gauge = gauges.contains(&field.as_str());
            assert!(by_grip || by_gauge || exempt, "{collection}.{field} has no grip ({spec})");
            assert!(u8::from(by_grip) + u8::from(by_gauge) + u8::from(exempt) == 1, "{collection}.{field} is covered twice");
            covered += usize::from(by_grip || by_gauge);
        }
        assert!(numeric - covered <= 1, "{collection}: {covered} of {numeric} numeric fields have a grip");
    }
}

/// 🎚️ A gauge writes the current value of its field at zero drag, moves it by twenty schema steps per metre, writes only existing leaves, and the document re-evaluates with the dragged value.
#[test]
fn gauges_write_existing_leaves_move_by_steps_and_the_dragged_document_re_evaluates() {
    let doc = compliant_demo();
    let layout = Layout::of(&doc, &[]);
    let all = handles::all(&doc, &layout);
    assert_handle_laws(&all);
    let gauges: Vec<&scene::Handle> = all.iter().filter(|h| h.id.contains(":gauge-")).collect();
    assert!(gauges.len() >= 40, "{} gauges", gauges.len());
    let tree = semio_framework_value::ToValue::to_value(&doc);
    let mut ids = std::collections::BTreeSet::new();
    for h in &gauges {
        assert!(ids.insert(h.id.clone()), "duplicate {}", h.id);
        for write in &h.writes {
            semio_s_plugin_norm_app_surface::get_value_at_path(&tree, &write.path).unwrap_or_else(|e| panic!("gauge {} writes {}: {e}", h.id, write.path));
        }
    }
    let wall = doc.retaining_walls[0].clone();
    let surcharge = all.iter().find(|h| h.id == format!("{}:gauge-surcharge", wall.id)).expect("surcharge gauge");
    assert_eq!(surcharge.writes[0].path, format!("retainingWalls[id={}].surcharge", wall.id));
    assert!((surcharge.writes_at(0.0)[0].value - wall.surcharge).abs() <= 500.0);
    let dragged = surcharge.writes_at(1.0)[0].value;
    assert!((dragged - (wall.surcharge + 20_000.0)).abs() <= 500.0, "{dragged}");
    let mut moved = doc.clone();
    moved.retaining_walls[0].surcharge = dragged;
    assert_ne!(hashes(&scene_of(&moved)), hashes(&scene_of(&doc)), "the dragged surcharge must change the projected wall");
}

/// ⚖️ A characteristic-value basis is a drawn disc in its layer with a parameter and a basis choice grip writing the typed tokens, tied to the check outcome of its row.
#[test]
fn a_characteristic_basis_is_drawn_and_its_choice_grips_write_the_typed_tokens() {
    use super::model::{CharacteristicBasis, CharacteristicParameter, ParameterBasis};
    let mut doc = super::model::showcase_demo();
    let layer = doc.layers[0].id.clone();
    doc.characteristic_bases = vec![CharacteristicBasis { id: "basis-1".into(), layer_id: layer, parameter: CharacteristicParameter::UndrainedStrength, basis: ParameterBasis::Derived }];
    let scene = scene_of(&doc);
    scene.validate().unwrap_or_else(|v| panic!("{v:?}"));
    let drawn = entity(&scene, "basis-1");
    assert_eq!(drawn.handles, ["basis-1:parameter", "basis-1:basis"]);
    assert!(drawn.accessible_name.en.contains("basis-1") && drawn.accessible_detail.de.contains("Grundlage"));
    let layout = Layout::of(&doc, &[]);
    let grips = handles::build(&doc, &layout, "basis-1");
    assert_handle_laws(&grips);
    let tokens: Vec<String> = grips.iter().flat_map(|h| h.writes.iter().map(|w| w.path.clone())).collect();
    assert_eq!(tokens, ["characteristicBases[id=basis-1].parameter", "characteristicBases[id=basis-1].basis"]);
    assert!(entities_of(&doc).iter().any(|e| e.id == "basis-1" && e.kind == scene::EntityKind::Solid));
    let tree = semio_framework_value::ToValue::to_value(&doc);
    for write in grips.iter().flat_map(|h| &h.writes) {
        semio_s_plugin_norm_app_surface::get_value_at_path(&tree, &write.path).unwrap_or_else(|e| panic!("{}: {e}", write.path));
    }
}

#[test]
fn the_basis_badge_follows_the_document_annex_in_the_scene_and_every_diagram() {
    scene::projection_kit::test_support::assert_basis_follows_annex(norm::NormFamilyId::En1997, |annex| {
        let mut document = compliant_demo();
        document.annex = annex;
        let report = report(&document);
        (scene_of(&document), diagrams(&document, &report, None))
    });
}
