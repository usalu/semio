use super::vectors::at;
use super::*;
use crate::*;

fn close(a: &Vec3, b: [f64; 3]) {
    assert!((a.x - b[0]).abs() < 1e-12 && (a.y - b[1]).abs() < 1e-12 && (a.z - b[2]).abs() < 1e-12, "{a:?} vs {b:?}");
}

#[test]
fn frames_and_yaw_agree_with_the_oracle() {
    close(&rotate_yaw(&Vec3::X, std::f64::consts::FRAC_PI_2), [6.123233995736766e-17, 1.0, 0.0]);
    let f = member_frame(&Vec3::ZERO, &v3(2.0, 0.0, 0.0));
    close(&f.x, [1.0, 0.0, 0.0]);
    close(&f.y, [0.0, 1.0, 0.0]);
    close(&f.z, [0.0, 0.0, 1.0]);
    let upright = member_frame(&Vec3::ZERO, &v3(0.0, 0.0, 3.0));
    close(&upright.x, [0.0, 0.0, 1.0]);
    close(&upright.y, [0.0, -1.0, 0.0]);
    close(&upright.z, [1.0, 0.0, 0.0]);
}

#[test]
fn boxes_are_anchored_at_the_centre_of_their_base() {
    let corner = Frame { origin: v3(1.0, 2.0, 0.5), x: Vec3::Y, y: v3(-1.0, 0.0, 0.0), z: Vec3::Z };
    let SolidShape::Box { frame, size } = solid::box_from_corner(&corner, v3(4.0, 2.0, 1.0)) else { panic!("box") };
    close(&frame.origin, [0.0, 4.0, 0.5]);
    close(&size, [4.0, 2.0, 1.0]);
    close(&at(&frame, -size.x / 2.0, -size.y / 2.0, 0.0), [1.0, 2.0, 0.5]);
}

#[test]
fn tessellated_boxes_and_viewport_share_the_base_centre_convention() {
    let shape = SolidShape::Box { frame: Frame::WORLD, size: v3(2.0, 4.0, 1.0) };
    let mesh = crate::tessellation::solids::solid_mesh(&shape, crate::tessellation::Tolerance::default(), crate::tessellation::Lod::Lod0).expect("mesh");
    let fold = |pick: fn(f32, f32) -> f32, start: f32| mesh.positions.iter().fold([start; 3], |m, p| [pick(m[0], p[0]), pick(m[1], p[1]), pick(m[2], p[2])]);
    let (lo, hi) = (fold(f32::min, f32::MAX), fold(f32::max, f32::MIN));
    assert_eq!((lo, hi), ([-1.0, -2.0, 0.0], [1.0, 2.0, 1.0]));
}

#[test]
fn statuses_map_and_unverified_basis_warns() {
    assert_eq!(status(norm::CheckStatus::UnverifiedBasis), CheckStatus::Warning);
    assert_eq!(status(norm::CheckStatus::NotApplicable), CheckStatus::NotApplicable);
    assert_eq!(status(norm::CheckStatus::Fail), CheckStatus::Fail);
}

#[test]
fn ids_are_owned_by_their_base() {
    assert_eq!(base_id("joint-j1:bolts"), "joint-j1");
    assert_eq!(entity_ref(EntityKind::Member, "m1:notch", "members").path, "members[id=m1]");
}

fn member_check() -> norm::CheckResult {
    let stations = [(0.0, 0.2), (0.25, 0.6), (0.5, 0.9), (1.0, 0.4)].iter().map(|&(s, u)| norm::Station::along(s, u)).collect();
    norm::CheckResult::assess("beam", "Part", norm::ClauseId::new("Z", "1", "1"), norm::SubjectRef::whole(norm::LocalizedCopy::new("Beam", "Träger")), norm::LocalizedCopy::new("Beam", "Träger")).stations(stations).build()
}

fn close_all(got: &[f64], want: &[f64]) {
    assert_eq!(got.len(), want.len());
    assert!(got.iter().zip(want).all(|(a, b)| (a - b).abs() < 1e-12), "{got:?} vs {want:?}");
}

#[test]
fn member_stations_become_fields_segments_and_binding_values_like_the_oracle() {
    let check = member_check();
    close_all(&field_along(&check, &[-0.1, 0.0, 0.125, 0.5, 0.75, 1.0, 1.2]), &[0.2, 0.2, 0.4, 0.9, 0.65, 0.4, 0.4]);
    close_all(&segment_values(&check, 4), &[0.6, 0.9, 0.9, 0.65]);
    close_all(&field_at_points(&check, &[[0.125, 5.0, 5.0]]), &[0.4]);
    let binding = binding_of_check(&check);
    assert_eq!(binding.ratio, 0.9);
    assert_eq!(binding.per_station.as_ref().map(Vec::len), Some(4));
    assert!(binding.per_station.unwrap().windows(2).all(|w| w[0].at < w[1].at));
}

#[test]
fn plate_stations_interpolate_by_inverse_distance_like_the_oracle() {
    let stations = vec![norm::Station::point(0.0, 0.0, 0.2), norm::Station::point(1.0, 0.0, 0.6), norm::Station::point(0.0, 1.0, 0.4)];
    let check = member_check_with(stations);
    close_all(&field_at_points(&check, &[[0.0, 0.0, 0.0], [0.5, 0.5, 0.0], [1.0, 1.0, 0.0]]), &[0.2, 0.4, 0.44]);
    assert!(station_values(&check).is_empty() && binding_of_check(&check).per_station.is_none());
}

fn member_check_with(stations: Vec<norm::Station>) -> norm::CheckResult {
    norm::CheckResult { stations, ..member_check() }
}

#[test]
fn duplicate_station_positions_keep_the_largest_value() {
    let check = member_check_with(vec![norm::Station::along(0.5, 0.3), norm::Station::along(0.5, 0.8), norm::Station::along(0.0, 0.1)]);
    let values = station_values(&check);
    assert_eq!(values.iter().map(|v| (v.at, v.value)).collect::<Vec<_>>(), vec![(0.0, 0.1), (0.5, 0.8)]);
}

#[test]
fn envelope_takes_the_largest_interpolated_utilisation_over_the_union_of_positions() {
    let flat = member_check_with(vec![norm::Station::along(0.0, 0.5), norm::Station::along(0.75, 0.5), norm::Station::along(1.0, 0.5)]);
    let values = envelope([&member_check(), &flat]);
    assert_eq!(values.iter().map(|v| v.at).collect::<Vec<_>>(), vec![0.0, 0.25, 0.5, 0.75, 1.0]);
    close_all(&values.iter().map(|v| v.value).collect::<Vec<_>>(), &[0.5, 0.6, 0.9, 0.65, 0.5]);
    assert!(envelope(std::iter::empty::<&norm::CheckResult>()).is_empty());
}

#[test]
fn track_constructors_emit_valid_tracks() {
    let timed = tracks::timed("fire", TrackKind::Time, ("Fire", "Brand"), &[(0.0, "t0", "Start", "Beginn"), (1800.0, "t30", "30 min", "30 min"), (3600.0, "t60", "60 min", "60 min")]).expect("timed");
    assert_eq!((timed.min, timed.max, timed.step, timed.looped), (0.0, 3600.0, 1800.0, false));
    assert_eq!(timed.validate(), Ok(()));
    let stops = |n: usize| (1..=n).map(|i| (format!("Mode {i}"), format!("Eigenform {i}"), None, Some(format!("mode-{i}")))).collect::<Vec<_>>();
    assert!(tracks::indexed("mode", TrackKind::Mode, ("Mode", "Eigenform"), stops(1)).is_none());
    let modes = tracks::indexed("mode", TrackKind::Mode, ("Mode", "Eigenform"), stops(3)).expect("indexed");
    assert_eq!((modes.min, modes.max, modes.step, modes.keyframes.len()), (1.0, 3.0, 1.0, 3));
    assert_eq!(modes.validate(), Ok(()));
    let phase = tracks::phase();
    assert!(phase.looped && phase.keyframes.is_empty());
    assert_eq!(phase.validate(), Ok(()));
    assert!(tracks::timed("bad", TrackKind::Time, ("Bad", "Schlecht"), &[(10.0, "a", "", ""), (5.0, "b", "", "")]).is_none());
}

#[test]
fn a_document_with_two_combinations_yields_a_combination_track() {
    let captions = |names: &[&str]| names.iter().map(|name| (name.to_string(), format!("{name} de"))).collect::<Vec<_>>();
    assert!(tracks::combination_track(captions(&["a"])).is_none());
    assert!(tracks::combination_track(captions(&["a", "a"])).is_none());
    let mut scene = crate::projection::empty_scene();
    tracks::push_combination_track(&mut scene, captions(&["a", "b", "b", "c"]));
    test_support::assert_combination_track_law(&scene, 3);
    tracks::push_combination_track(&mut scene, captions(&["x", "y"]));
    test_support::assert_combination_track_law(&scene, 3);
    test_support::assert_combination_track_law(&crate::projection::empty_scene(), 1);
}

#[test]
fn a_placement_drag_is_one_commit_and_one_undo_step() {
    let current = vec![Placement { entity_id: "wall-1".into(), origin: v3(1.0, 2.0, 0.0), yaw: 0.0 }];
    let moves = vec![
        PlacementMove { entity_id: "wall-1".into(), origin: v3(3.0, 2.0, 0.0), yaw: 0.5 },
        PlacementMove { entity_id: "wall-2".into(), origin: v3(0.0, 4.0, 0.0), yaw: 0.0 },
    ];
    let commit = PlacementCommit::of(&current, &moves);
    assert_eq!(commit.ops.len(), 2);
    assert!(matches!(&commit.ops[0], PlacementOp::Set(writes) if writes.len() == 4 && writes[0].path == "placements[id=wall-1].x" && writes[0].value == 3.0));
    assert!(matches!(&commit.ops[1], PlacementOp::Insert(row) if row.entity_id == "wall-2"));
    let mut rows = current.clone();
    commit.apply(&mut rows);
    assert_eq!(placed(&rows, "wall-1").map(|p| (p.origin.x, p.yaw)), Some((3.0, 0.5)));
    assert_eq!(rows.len(), 2);
    commit.inverse(&current).apply(&mut rows);
    assert_eq!(rows, current);
}

#[test]
fn a_release_at_the_origin_commits_nothing() {
    let current = vec![Placement { entity_id: "a".into(), origin: v3(1.0, 1.0, 0.0), yaw: 0.25 }];
    assert!(PlacementCommit::of(&current, &[PlacementMove { entity_id: "a".into(), origin: v3(1.0, 1.0, 0.0), yaw: 0.25 }]).is_empty());
}

#[test]
fn revision_stamps_change_exactly_with_their_inputs() {
    let rows = vec![Placement { entity_id: "a".into(), origin: v3(0.0, 0.0, 0.0), yaw: 0.0 }];
    let base = revision_of(&"doc", &rows, &"report");
    assert_eq!(base, revision_of(&"doc", &rows, &"report"));
    assert_ne!(base.document_seq, revision_of(&"doc2", &rows, &"report").document_seq);
    assert_ne!(base.document_seq, revision_of(&"doc", &[], &"report").document_seq);
    assert_eq!(base.report_seq, revision_of(&"doc2", &rows, &"report").report_seq);
    assert_ne!(base.report_seq, revision_of(&"doc", &rows, &"report2").report_seq);
}

#[test]
fn the_placement_row_shape_is_flat_id_x_y_z_yaw_for_every_reader_and_writer() {
    let commit = PlacementCommit::of(&[Placement { entity_id: "a".into(), origin: v3(0.0, 0.0, 0.0), yaw: 0.0 }], &[PlacementMove { entity_id: "a".into(), origin: v3(1.0, 2.0, 3.0), yaw: 0.5 }]);
    let PlacementOp::Set(writes) = &commit.ops[0] else { panic!("set") };
    let leaves: Vec<&str> = writes.iter().map(|w| w.path.as_str()).collect();
    assert_eq!(leaves, ["placements[id=a].x", "placements[id=a].y", "placements[id=a].z", "placements[id=a].yaw"]);
    assert!(leaves.iter().all(|p| !p.contains("origin")));
}

#[test]
fn a_drag_on_an_entity_without_a_row_inserts_the_row_with_the_dragged_pose_in_one_undo_step() {
    let current = vec![Placement { entity_id: "placed".into(), origin: v3(1.0, 1.0, 0.0), yaw: 0.0 }];
    let standing = |id: &str| (id == "bare").then(|| Placement { entity_id: "bare".into(), origin: v3(6.0, 0.0, 2.0), yaw: 0.1 });
    let writes = vec![PlacementWrite { path: "placements[id=bare].x".into(), value: 7.5 }, PlacementWrite { path: "placements[id=bare].y".into(), value: -2.0 }, PlacementWrite { path: "placements[id=other].x".into(), value: 1.0 }, PlacementWrite { path: "members[0].span".into(), value: 9.0 }];
    let commit = PlacementCommit::of_writes(&current, &standing, &writes);
    assert_eq!(commit.ops.len(), 2);
    assert!(matches!(&commit.ops[0], PlacementOp::Insert(row) if row.entity_id == "bare" && row.origin == v3(7.5, -2.0, 2.0) && row.yaw == 0.1));
    assert!(matches!(&commit.ops[1], PlacementOp::Insert(row) if row.entity_id == "other" && row.origin == v3(1.0, 0.0, 0.0)));
    let mut rows = current.clone();
    commit.apply(&mut rows);
    assert_eq!(rows.len(), 3);
    let again = PlacementCommit::of_writes(&rows, &standing, &writes);
    assert!(again.is_empty());
    commit.inverse(&current).apply(&mut rows);
    assert_eq!(rows, current);
    let set = PlacementCommit::of_writes(&current, &standing, &[PlacementWrite { path: "placements[id=placed].x".into(), value: 4.0 }]);
    assert!(matches!(&set.ops[..], [PlacementOp::Set(w)] if w.len() == 4 && w[0].value == 4.0 && w[1].value == 1.0));
    assert_eq!(placement_target("placements[id=a-b].yaw"), Some(("a-b", "yaw")));
    assert_eq!(grip_base(&current, &v3(6.0, 5.0, 0.0), "bare", "y"), 5.0);
    assert_eq!(grip_base(&current, &v3(6.0, 5.0, 0.0), "placed", "y"), 1.0);
}

#[test]
fn move_handles_write_placement_leaves_and_yaw_anchors_at_the_angle() {
    let entity = EntityRef::new(EntityKind::Member, "beam-1", "members[id=beam-1]");
    let stored = vec![Placement { entity_id: "beam-1".into(), origin: v3(3.0, 4.0, 0.0), yaw: std::f64::consts::FRAC_PI_2 }];
    let grips = handle_kit::move_handles(&entity, "beam-1", &v3(3.0, 4.0, 0.0), &stored, 0.0, true);
    let ids: Vec<&str> = grips.iter().map(|h| h.id.as_str()).collect();
    assert_eq!(ids, ["beam-1:move-x", "beam-1:move-y", "beam-1:move-yaw"]);
    assert_eq!(grips[0].writes[0].path, "placements[id=beam-1].x");
    assert_eq!(grips[0].writes[0].map.base, 3.0);
    assert_eq!(grips[1].writes[0].map.base, 4.0);
    assert!(matches!(grips[2].kind, HandleKind::Angular { .. }));
    close(&grips[2].anchor, [3.0, 5.0, 0.0]);
    let bare = handle_kit::move_handles(&entity, "beam-1", &v3(1.0, 2.0, 0.0), &[], 0.0, false);
    assert_eq!(bare.len(), 2);
    assert_eq!(bare[0].writes[0].map.base, 1.0);
    let commit = PlacementCommit::of_writes(&[], &|_| None, &[PlacementWrite { path: bare[0].writes[0].path.clone(), value: 2.5 }]);
    assert!(matches!(&commit.ops[..], [PlacementOp::Insert(row)] if row.entity_id == "beam-1" && row.origin.x == 2.5));
}

use semio_framework_value::DslValue;

struct RowsOnly {
    rows: DslValue,
    whole: std::cell::Cell<usize>,
}

impl semio_framework_value::ToValue for RowsOnly {
    fn to_value(&self) -> DslValue {
        self.whole.set(self.whole.get() + 1);
        DslValue::object(vec![(PLACEMENTS_FIELD.to_string(), self.rows.clone()), ("members".to_string(), DslValue::Array(Vec::new()))])
    }

    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, semio_framework_value::ValueError> {
        match path {
            [PLACEMENTS_FIELD] => Ok(self.rows.clone()),
            _ => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "only the placements are asked")),
        }
    }
}

struct WholeOnly(DslValue);

impl semio_framework_value::ToValue for WholeOnly {
    fn to_value(&self) -> DslValue {
        self.0.clone()
    }
}

fn mixed_placement_rows() -> DslValue {
    let row = |entries: Vec<(&str, DslValue)>| DslValue::object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect::<Vec<_>>());
    DslValue::Array(vec![
        row(vec![("id", DslValue::String("a".into())), ("x", DslValue::float(1.0)), ("y", DslValue::float(2.0)), ("z", DslValue::float(3.0)), ("yaw", DslValue::float(0.5))]),
        row(vec![("id", DslValue::String("b".into()))]),
        row(vec![("x", DslValue::float(9.0))]),
        DslValue::Null,
    ])
}

#[test]
fn placements_are_read_from_the_placements_field_alone_and_equal_the_whole_tree_reading() {
    let rows = mixed_placement_rows();
    let lazy = RowsOnly { rows: rows.clone(), whole: std::cell::Cell::new(0) };
    let expected = vec![Placement { entity_id: "a".into(), origin: v3(1.0, 2.0, 3.0), yaw: 0.5 }, Placement { entity_id: "b".into(), origin: v3(0.0, 0.0, 0.0), yaw: 0.0 }];
    assert_eq!(placements_of(&lazy), expected, "rows without an id or object shape are skipped, missing leaves read as zero");
    assert_eq!(lazy.whole.get(), 0, "the document is never converted as a whole tree");
    let whole = WholeOnly(DslValue::object(vec![(PLACEMENTS_FIELD.to_string(), rows)]));
    assert_eq!(placements_of(&whole), expected, "a document that only offers the whole tree reads the same rows");
    assert!(placements_of(&WholeOnly(DslValue::object(Vec::new()))).is_empty(), "no collection, no rows");
    assert!(placements_of(&WholeOnly(DslValue::Null)).is_empty(), "no object, no rows");
}

fn stamp_by_string<T: std::fmt::Debug>(value: &T) -> u64 {
    format!("{value:?}").bytes().fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3))
}

#[test]
fn the_content_stamp_equals_the_fnv_of_the_debug_text_and_the_published_fnv_vectors() {
    struct Raw(&'static str);
    impl std::fmt::Debug for Raw {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str(self.0)
        }
    }
    assert_eq!(content_stamp(&Raw("")), 0xcbf2_9ce4_8422_2325, "FNV-1a 64 of the empty text");
    assert_eq!(content_stamp(&Raw("a")), 0xaf63_dc4c_8601_ec8c, "FNV-1a 64 of \"a\"");
    assert_eq!(content_stamp(&Raw("foobar")), 0x8594_4171_f739_67e8, "FNV-1a 64 of \"foobar\"");
    let rows: Vec<Placement> = (0..50).map(|i| Placement { entity_id: format!("é{i}"), origin: v3(f64::from(i), 0.1, -2.5e-9), yaw: f64::from(i) / 7.0 }).collect();
    for value in [(rows.clone(), "report", Some(3_u8)), (Vec::new(), "", None)] {
        assert_eq!(content_stamp(&value), stamp_by_string(&value));
    }
    assert_eq!(revision_of(&"doc", &rows, &"report").document_seq, content_stamp(&(content_stamp(&"doc"), &rows[..])));
}

#[test]
fn stamping_twenty_thousand_rows_stays_within_the_commit_budget() {
    let rows: Vec<Placement> = (0..20_000).map(|i| Placement { entity_id: format!("member-{i}"), origin: v3(f64::from(i), 1.5, 0.25), yaw: 0.125 }).collect();
    let mut times: Vec<f64> = (0..9)
        .map(|round| {
            let started = std::time::Instant::now();
            let mut rows = rows.clone();
            rows[round].yaw += 1.0;
            let stamp = revision_of(&rows, &rows, &rows);
            assert_ne!(stamp.document_seq, 0);
            started.elapsed().as_secs_f64() * 1000.0
        })
        .collect();
    times.sort_by(f64::total_cmp);
    let (p50, p95, budget) = (times[4], times[8], if cfg!(debug_assertions) { 8000.0 } else { 400.0 });
    eprintln!("[latency] revision-stamp-20000-rows/commit p50_ms={p50:.1} p95_ms={p95:.1} budget_ms={budget:.1}");
    assert!(p95 <= budget, "stamping 20000 rows (three Debug passes) took {p95:.1} ms against {budget:.1} ms");
}

mod diagram_laws {
    use crate::projection_kit::test_support::{assert_basis_follows_annex, assert_diagram_laws, diagram_law_violations};
    use crate::projection_kit::diagram_parts::{axis, column, stroke};
    use crate::*;

    fn marker(id: &str, check: Option<&str>, utilisation: Option<f64>, tone: Tone) -> Marker {
        Marker { id: id.into(), at: Vec2::new(1.0, 1.0), shape: MarkerShape::Circle, tone, label: LocalizedCopy::new("x = 1 m", "x = 1 m"), utilisation, bound_check: check.map(Into::into), binding: None }
    }

    fn diagram() -> Diagram2d {
        Diagram2d {
            id: "profile:wall".into(),
            title: LocalizedCopy::new("Profile", "Profil"),
            description: LocalizedCopy::new("The profile.", "Das Profil."),
            axes: vec![axis("Height z", "Höhe z", QuantityKind::Length, "m", AxisScale::Linear, None), axis("Pressure q", "Druck q", QuantityKind::Pressure, "kN/m²", AxisScale::Linear, None)],
            series: vec![Series::Polyline { id: "q".into(), label: LocalizedCopy::new("q", "q"), points: vec![Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)], style: stroke(Tone::Info, 2.0, Dash::Solid) }],
            markers: vec![marker("face-a", Some("c1"), Some(0.5), Tone::Info), marker("reference:corner", None, None, Tone::Neutral)],
            annotations: Vec::new(),
            focus: None,
            legend: true,
            table_twin: DataTable { columns: vec![column("Height z", "Höhe z", "m", vec![0.0, 1.0]), column("Pressure q", "Druck q", "kN/m²", vec![0.0, 1.0])] },
        }
    }

    #[test]
    fn a_bound_diagram_with_reference_corners_obeys_the_laws() {
        assert_eq!(diagram_law_violations(&diagram()), Vec::<String>::new());
        assert_diagram_laws(&[diagram()]);
    }

    #[test]
    fn an_unbound_marker_that_is_no_reference_breaks_the_law() {
        let mut d = diagram();
        d.markers.push(marker("work-point", None, None, Tone::Neutral));
        assert!(diagram_law_violations(&d).iter().any(|v| v.contains("work-point") && v.contains("names no check")));
        let mut d = diagram();
        d.markers.push(marker("reference:bound", Some("c1"), None, Tone::Neutral));
        assert!(diagram_law_violations(&d).iter().any(|v| v.contains("reference:bound")));
    }

    #[test]
    fn a_missing_german_title_breaks_the_law() {
        let mut d = diagram();
        d.axes[0].label = LocalizedCopy::new("Position", "Position");
        assert!(diagram_law_violations(&d).iter().any(|v| v.contains("German axis title")));
        d.axes[0].label = LocalizedCopy::new("Position", "Lage");
        d.title = LocalizedCopy::new("Profile", " ");
        assert!(diagram_law_violations(&d).iter().any(|v| v.contains("German title")));
        d.title = LocalizedCopy::new("M", "M");
        d.axes[0].label = LocalizedCopy::new("M", "M");
        assert_eq!(diagram_law_violations(&d), Vec::<String>::new());
    }

    #[test]
    fn a_non_canonical_unit_symbol_breaks_the_law() {
        for bad in ["kN/m2", "deg", "degC", "-", "m2", "kNm", ""] {
            let mut d = diagram();
            d.axes[1].unit = bad.into();
            assert!(diagram_law_violations(&d).iter().any(|v| v.contains("no canonical unit symbol")), "{bad}");
        }
        let mut d = diagram();
        d.table_twin.columns[1].unit = "deg".into();
        assert!(diagram_law_violations(&d).iter().any(|v| v.contains("table column")));
        for good in ["°", "°C", "1", "kN/m²", "kN·m", "‰", "1/km", "N·mm/mm", "kN·m/m"] {
            assert!(norm::UnitId::lookup(good).is_some_and(|u| u.symbol() == good), "{good}");
        }
    }

    #[test]
    fn a_unit_of_another_kind_breaks_the_law() {
        let mut d = diagram();
        d.axes[0].quantity = QuantityKind::Dimensionless;
        assert!(diagram_law_violations(&d).iter().any(|v| v.contains("does not fit the unit")));
        d.axes[0] = axis("Taper angle", "Neigungswinkel", QuantityKind::Angle, "°", AxisScale::Linear, None);
        assert_eq!(diagram_law_violations(&d), Vec::<String>::new());
    }

    #[test]
    fn a_foreign_diagram_id_breaks_the_law() {
        let mut d = diagram();
        for bad in ["qp-profile", "frame:a:b", "Profile:a", "profile:", "plain"] {
            d.id = bad.into();
            assert!(diagram_law_violations(&d).iter().any(|v| v.contains("<kind>:<row>")), "{bad}");
        }
    }

    #[test]
    fn the_basis_law_needs_a_badge_in_the_scene_and_every_diagram() {
        use crate::projection::{empty_scene, Basis};
        let family = norm::NormFamilyId::En1993;
        assert_basis_follows_annex(family, |annex| {
            let basis = Basis::new(family, annex);
            let mut scene = empty_scene();
            basis.apply(&mut scene);
            let mut shown = vec![diagram()];
            basis.apply_to_diagrams(&mut shown);
            (scene, shown)
        });
    }
}

#[test]
fn an_empty_golden_is_blessed_once_and_then_guards_the_bytes() {
    if std::env::var_os("SEMIO_UPDATE_GOLDEN").is_some() {
        return;
    }
    let dir = std::env::temp_dir().join(format!("semio-golden-{}", std::process::id()));
    let path = dir.join("nested").join("golden.json");
    let check = |text: &'static str| std::panic::catch_unwind(|| test_support::assert_golden(&path, text)).is_ok();
    assert!(!check("{\"a\":1}"), "the first run fails once after writing");
    assert_eq!(std::fs::read_to_string(&path).expect("written"), "{\"a\":1}\n");
    assert!(check("{\"a\":1}\n"), "the second run passes");
    assert!(!check("{\"a\":2}"), "changed bytes fail");
    assert_eq!(std::fs::read_to_string(&path).expect("kept"), "{\"a\":1}\n");
    std::fs::remove_dir_all(&dir).expect("cleanup");
}
