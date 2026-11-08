use super::*;
use scene::projection_kit::test_support::{revision, entity, assert_diagram_laws, assert_handle_laws, assert_scene_laws};
use scene::{canonical_json, Validate};
use semio_s_artifact_norm_en1990_examples as examples;
use std::collections::BTreeSet;

fn scene_with(document: &En1990Snapshot, combination: Option<&str>) -> StructuralScene {
    let report = inferences::evaluate(document);
    project(&SceneInput { document, report: &report, placements: &[], revision: revision(), combination, traffic_group: None })
}

fn scene_of(document: &En1990Snapshot) -> StructuralScene {
    scene_with(document, None)
}

fn office() -> En1990Snapshot {
    En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)
}

fn failing_office() -> En1990Snapshot {
    let mut document = office();
    document.members[0].rd_str = 1.0;
    document
}

fn fixtures() -> Vec<(&'static str, En1990Snapshot)> {
    vec![
        ("office", office()),
        ("failing-office", failing_office()),
        ("accidental-seismic-compliant", examples::accidental_seismic_compliant::reference_snapshot()),
        ("accidental-seismic-failing", examples::accidental_seismic_failing::reference_snapshot()),
        ("fatigue-compliant", examples::fatigue_compliant::reference_snapshot()),
        ("fatigue-failing", examples::fatigue_failing::reference_snapshot()),
        ("high-consequence-office", examples::high_consequence_office::reference_snapshot()),
        ("road-bridge-compliant", examples::road_bridge_compliant::reference_snapshot()),
        ("road-bridge-failing", examples::road_bridge_failing::reference_snapshot()),
    ]
}

fn column_id(document: &En1990Snapshot, family: combinations::Family, leading: &str) -> String {
    combinations::columns(document).into_iter().find(|c| c.family == family && c.leading() == leading).unwrap_or_else(|| panic!("no {family:?} column leading {leading}")).id
}

fn magnitude(scene: &StructuralScene, id: &str) -> f64 {
    match &entity(scene, id).geometry {
        scene::Geometry::Load(scene::LoadGlyph::Line { profile: scene::LoadProfile::Uniform { value }, .. }) => value.value,
        scene::Geometry::Load(scene::LoadGlyph::Point { magnitude, .. }) => magnitude.value,
        scene::Geometry::Load(scene::LoadGlyph::Impact { energy_or_force, .. }) => energy_or_force.value,
        other => panic!("{id} is no force glyph: {other:?}"),
    }
}

#[test]
fn the_scene_of_every_fixture_validates() {
    for (name, document) in fixtures() {
        let scene = scene_of(&document);
        assert_scene_laws(&scene);
        assert_eq!(scene.validate(), Ok(()), "violations in {name}");
        assert_eq!(scene.schema, scene::SCENE_SCHEMA_ID);
    }
}

#[test]
fn ids_are_unique_and_layers_exist() {
    for (name, document) in fixtures() {
        let scene = scene_of(&document);
        assert_scene_laws(&scene);
        let ids: BTreeSet<_> = scene.entities.iter().map(|e| e.entity.id.clone()).collect();
        assert_eq!(ids.len(), scene.entities.len(), "{name}");
        let annotation_ids: BTreeSet<_> = scene.annotations.iter().map(|a| a.id().to_string()).collect();
        assert_eq!(annotation_ids.len(), scene.annotations.len(), "{name}");
        let layers: BTreeSet<_> = scene.layers.iter().map(|l| l.id.as_str()).collect();
        assert!(scene.entities.iter().all(|e| layers.contains(e.layer.as_str())), "{name}");
    }
}

#[test]
fn projection_is_deterministic_and_locale_free() {
    let (a, b) = (scene_of(&office()), scene_of(&office()));
    assert_scene_laws(&a);
    assert_scene_laws(&b);
    assert_eq!(canonical_json(&a), canonical_json(&b));
    assert!(a.entities.iter().all(|e| !e.accessible_name.en.is_empty() && !e.accessible_name.de.is_empty()));
}

#[test]
fn a_member_spans_its_length_with_supports_and_a_deflected_shape() {
    let document = office();
    let scene = scene_of(&document);
    assert_scene_laws(&scene);
    let member = entity(&scene, "beam-B1");
    let scene::Geometry::Member(geometry) = &member.geometry else { panic!("member geometry") };
    let scene::Path3::Polyline(axis) = &geometry.axis else { panic!("polyline axis") };
    assert!((axis.points[0].distance(&axis.points[1]) - document.members[0].span).abs() < 1e-9);
    assert_eq!(member.entity.path, "members[id=beam-B1]");
    assert!(matches!(entity(&scene, "beam-B1:support-a").geometry, scene::Geometry::Support(_)));
    assert!(matches!(entity(&scene, "beam-B1:support-b").geometry, scene::Geometry::Support(_)));
    let overlay = member.appearance.deformation.as_ref().expect("deflected shape");
    assert_eq!(overlay.kind, scene::DeformationKind::Deformed);
    let m = &document.members[0];
    let net = m.deflection_w1 + m.deflection_w2 + m.deflection_w3 - m.precamber;
    let mid = &overlay.samples[overlay.samples.len() / 2];
    assert!((mid.delta.z + net).abs() < 1e-12, "mid-span deflection is w_net,fin downwards");
    assert!((overlay.scale - members::exaggeration(m.span, net)).abs() < 1e-12);
}

#[test]
fn the_deflection_components_stack_as_bound_dimensions() {
    let scene = scene_of(&office());
    assert_scene_laws(&scene);
    for (tag, field) in [("w1", "deflectionW1"), ("w2", "deflectionW2"), ("w3", "deflectionW3")] {
        let found = scene.annotations.iter().find_map(|a| match a {
            scene::Annotation::Dimension { id, binding, value, .. } if id == &format!("beam-B1:{tag}") => Some((binding.clone(), *value)),
            _ => None,
        });
        let (binding, value) = found.unwrap_or_else(|| panic!("dimension {tag}"));
        assert_eq!(binding.expect("binding").path, format!("members[id=beam-B1].{field}"));
        assert!(value.value > 0.0);
    }
}

#[test]
fn a_low_natural_frequency_shows_its_mode_shape() {
    let mut document = office();
    let before = scene_of(&document);
    assert_scene_laws(&before);
    assert!(before.entities.iter().all(|e| e.entity.id != "beam-B1:mode"));
    document.members[0].vibration_frequency = 2.0;
    let scene = scene_of(&document);
    assert_scene_laws(&scene);
    let mode = entity(&scene, "beam-B1:mode");
    let overlay = mode.appearance.deformation.as_ref().expect("mode overlay");
    assert_eq!(overlay.kind, scene::DeformationKind::Mode);
    assert_eq!(overlay.frequency.map(|f| f.value), Some(2.0));
    assert_eq!(mode.appearance.style, scene::StyleState::Ghost);
}

#[test]
fn a_bridge_deck_deforms_and_vibrates() {
    let scene = scene_of(&examples::road_bridge_compliant::reference_snapshot());
    assert_scene_laws(&scene);
    let deck = entity(&scene, "sls-D1");
    assert!(matches!(deck.geometry, scene::Geometry::Plate(_)));
    assert_eq!(deck.appearance.deformation.as_ref().map(|o| o.kind), Some(scene::DeformationKind::Deformed));
    assert_eq!(entity(&scene, "sls-D1:mode").appearance.deformation.as_ref().map(|o| o.kind), Some(scene::DeformationKind::Mode));
    assert!(scene.annotations.iter().any(|a| a.id() == "sls-D1:acceleration"));
}

#[test]
fn utilisation_colouring_follows_the_governing_check() {
    let document = office();
    let report = inferences::evaluate(&document);
    let scene = scene_of(&document);
    assert_scene_laws(&scene);
    let member = entity(&scene, "beam-B1");
    let binding = member.utilisation.as_ref().expect("binding");
    let expected = report.checks.iter().filter(|c| c.status != norm::CheckStatus::NotApplicable && c.subject.entity_id == "beam-B1" && c.utilization.is_finite()).map(|c| c.utilization).fold(0.0_f64, f64::max);
    assert!((binding.ratio - expected).abs() < 1e-12);
    assert_eq!(member.appearance.channel, scene::ColourChannel::Utilisation);
}

#[test]
fn a_failing_member_gets_error_style_hatch_and_failure_glyphs() {
    let scene = scene_of(&failing_office());
    assert_scene_laws(&scene);
    let member = entity(&scene, "beam-B1");
    assert_eq!(member.appearance.style, scene::StyleState::Error);
    assert_eq!(member.appearance.hatch, Some(scene::Hatch::Diagonal));
    assert!(scene.annotations.iter().any(|a| matches!(a, scene::Annotation::FailureGlyph { mode: scene::FailureMode::Yielding, .. })));
}

#[test]
fn every_action_has_a_load_glyph_of_its_kind() {
    let scene = scene_of(&office());
    assert_scene_laws(&scene);
    assert!(matches!(entity(&scene, "G-sup:beam-B1").geometry, scene::Geometry::Load(scene::LoadGlyph::Line { .. })));
    assert!(matches!(entity(&scene, "Q-office:beam-B1").geometry, scene::Geometry::Load(scene::LoadGlyph::Line { .. })));
    assert!(matches!(entity(&scene, "Q-snow:beam-B1").geometry, scene::Geometry::Load(scene::LoadGlyph::Line { .. })));
    assert!(matches!(entity(&scene, "Q-wind:beam-B1").geometry, scene::Geometry::Load(scene::LoadGlyph::Point { .. })));
    let seismic = scene_of(&examples::accidental_seismic_compliant::reference_snapshot());
    assert_scene_laws(&seismic);
    assert!(seismic.entities.iter().any(|e| matches!(e.geometry, scene::Geometry::Load(scene::LoadGlyph::Impact { .. }))));
    assert!(seismic.entities.iter().any(|e| matches!(&e.appearance.role, scene::MaterialRole::Load { kind: scene::LoadKind::Seismic })));
    let office_scene = scene_of(&office());
    assert_scene_laws(&office_scene);
    let kinds: BTreeSet<_> = office_scene.entities.iter().filter_map(|e| match &e.appearance.role { scene::MaterialRole::Load { kind } => Some(format!("{kind:?}")), _ => None }).collect();
    assert_eq!(kinds, ["Permanent", "Snow", "Variable", "Wind"].into_iter().map(String::from).collect());
}

#[test]
fn glyph_sizes_follow_the_design_values_of_the_active_combination() {
    let document = office();
    let office_lead = column_id(&document, combinations::Family::Str, "Q-office");
    let wind_lead = column_id(&document, combinations::Family::Str, "Q-wind");
    let a = scene_with(&document, Some(&office_lead));
    assert_scene_laws(&a);
    let b = scene_with(&document, Some(&wind_lead));
    assert_scene_laws(&b);
    assert!((magnitude(&a, "G-sup:beam-B1") - 1.35 * 80_000.0).abs() < 1e-6);
    assert!((magnitude(&a, "Q-office:beam-B1") - 1.5 * 40_000.0).abs() < 1e-6);
    assert!((magnitude(&b, "Q-office:beam-B1") - 1.5 * 0.7 * 40_000.0).abs() < 1e-6);
    assert!((magnitude(&b, "Q-wind:beam-B1") - 1.5 * 20_000.0).abs() < 1e-6);
    assert_ne!(magnitude(&a, "Q-office:beam-B1"), magnitude(&b, "Q-office:beam-B1"));
}

#[test]
fn an_action_outside_the_active_combination_is_a_ghost() {
    let document = office();
    let quasi = column_id(&document, combinations::Family::SlsQp, "-");
    let scene = scene_with(&document, Some(&quasi));
    assert_scene_laws(&scene);
    assert_eq!(entity(&scene, "Q-wind:beam-B1").appearance.style, scene::StyleState::Ghost);
    assert_eq!(entity(&scene, "G-sup:beam-B1").appearance.style, scene::StyleState::Normal);
    assert!((magnitude(&scene, "Q-wind:beam-B1") - 20_000.0).abs() < 1e-6, "a ghost keeps its characteristic value");
}

#[test]
fn the_default_combination_is_the_governing_str_column() {
    let document = office();
    let columns = combinations::columns(&document);
    let governing = combinations::active(&columns, None).expect("active");
    assert_eq!(governing.family, combinations::Family::Str);
    assert!(columns.iter().filter(|c| c.family == combinations::Family::Str).all(|c| c.combination.total() <= governing.combination.total() + 1e-9));
    assert_eq!(combinations::active(&columns, Some("nothing")).map(|c| c.id.clone()), Some(governing.id.clone()));
}

#[test]
fn a_check_resolves_to_the_combination_it_was_evaluated_on() {
    let document = office();
    let report = inferences::evaluate(&document);
    let columns = combinations::columns(&document);
    let check = report.checks.iter().find(|c| c.id.starts_with("en1990.str.beam-B1.")).expect("str check");
    let column = combinations::column_for_check(&columns, check).expect("column");
    assert_eq!(column.family, combinations::Family::Str);
    assert!(check.explanation.en.contains(&format!("{} leading={}", column.equation().label(), column.leading())), "{}", check.explanation.en);
    let sls = report.checks.iter().find(|c| c.id == "en1990.sls.beam-B1.6.15b");
    if let Some(sls) = sls {
        assert_eq!(combinations::column_for_check(&columns, sls).map(|c| c.family), Some(combinations::Family::SlsFreq));
    }
}

#[test]
fn badges_state_classes_reliability_and_the_active_combination() {
    let scene = scene_of(&office());
    assert_scene_laws(&scene);
    let ids: BTreeSet<_> = scene.annotations.iter().map(|a| a.id().to_string()).collect();
    for expected in ["badge:consequence", "badge:reliability", "badge:beta", "badge:annex", "badge:combination"] {
        assert!(ids.contains(expected), "missing {expected} in {ids:?}");
    }
    let mut mismatch = office();
    mismatch.reliability_class = model::ReliabilityClass::RC3;
    let mismatch_scene = scene_of(&mismatch);
    assert_scene_laws(&mismatch_scene);
    let tone = mismatch_scene.annotations.iter().find_map(|a| match a {
        scene::Annotation::Label { id, tone, .. } if id == "badge:reliability" => Some(*tone),
        _ => None,
    });
    assert_eq!(tone, Some(scene::Tone::Warning));
}

#[test]
fn the_selector_group_advertises_its_handle() {
    let scene = scene_of(&office());
    assert_scene_laws(&scene);
    let selector = entity(&scene, COMBINATION_ENTITY);
    assert!(matches!(selector.geometry, scene::Geometry::Group { .. }));
    assert_eq!(selector.handles, vec!["combination:select".to_string()]);
}

#[test]
fn every_advertised_handle_exists_and_writes_a_known_leaf() {
    for (name, document) in fixtures() {
        let scene = scene_of(&document);
        assert_scene_laws(&scene);
        let selection: Vec<_> = scene.entities.iter().map(|e| e.entity.clone()).collect();
        let all = handles(&document, &[], &selection);
        assert_handle_laws(&all);
        let ids: BTreeSet<_> = all.iter().map(|h| h.id.clone()).collect();
        for e in &scene.entities {
            assert!(e.handles.iter().all(|h| ids.contains(h)), "{name}: {} advertises an unknown handle", e.entity.id);
        }
        assert!(all.iter().all(|h| h.writes.iter().all(|w| ((w.path.contains("[id=") || w.path.starts_with("effects[")) && w.path.contains('.')) || w.path == COMBINATION_PATH || w.path == TRAFFIC_GROUP_PATH)), "{name}");
        assert!(all.iter().all(|h| h.accessible_name.en.len() > 2 && h.accessible_name.de.len() > 2), "{name}");
        assert!(all.iter().all(|h| h.limits.as_ref().is_some_and(|l| l.min < l.max)), "{name}");
    }
}

#[test]
fn dragging_the_span_handle_writes_the_snapped_span() {
    let all = handles(&office(), &[], &[scene::EntityRef::new(scene::EntityKind::Member, "beam-B1", "members[id=beam-B1]")]);
    assert_handle_laws(&all);
    let span = all.iter().find(|h| h.id == "beam-B1:span").expect("span handle");
    let writes = span.writes_at(6.012);
    assert_eq!(writes.len(), 1);
    assert_eq!(writes[0].path, "members[id=beam-B1].span");
    assert!((writes[0].value - 6.0).abs() < 1e-9);
    assert_eq!(span.writes_at(1000.0)[0].value, 200.0);
}

#[test]
fn the_precamber_handle_divides_out_the_exaggeration() {
    let document = office();
    let all = handles(&document, &[], &[scene::EntityRef::new(scene::EntityKind::Member, "beam-B1", "members[id=beam-B1]")]);
    assert_handle_laws(&all);
    let precamber = all.iter().find(|h| h.id == "beam-B1:precamber").expect("precamber handle");
    let m = &document.members[0];
    let scale = members::exaggeration(m.span, m.deflection_w1 + m.deflection_w2 + m.deflection_w3 - m.precamber);
    let write = &precamber.writes_at(0.01 * scale)[0];
    assert_eq!(write.path, "members[id=beam-B1].precamber");
    assert!((write.value - 0.01).abs() < 1.5e-3, "{}", write.value);
}

#[test]
fn the_magnitude_handle_writes_the_characteristic_value() {
    let document = office();
    let all = handles(&document, &[], &[scene::EntityRef::new(scene::EntityKind::Load, "Q-office:beam-B1", "variables[id=Q-office]")]);
    assert_handle_laws(&all);
    let magnitude = all.iter().find(|h| h.id == "Q-office:magnitude").expect("magnitude handle");
    let write = &magnitude.writes_at(1.0)[0];
    assert_eq!(write.path, "variables[id=Q-office].qk");
    assert_eq!(write.unit, "N");
    assert!((write.value - 40_000.0).abs() < 1e-6, "{}", write.value);
}

#[test]
fn the_combination_handle_steps_through_the_columns() {
    let document = office();
    let count = combinations::columns(&document).len();
    let all = handles(&document, &[], &[scene::EntityRef::new(scene::EntityKind::Group, COMBINATION_ENTITY, "combinations[id=combination]")]);
    assert_handle_laws(&all);
    let select = all.iter().find(|h| h.id == "combination:select").expect("selector handle");
    assert_eq!(select.writes[0].path, COMBINATION_PATH);
    assert_eq!(select.writes_at(2.4)[0].value, 2.0);
    assert_eq!(select.writes_at(1000.0)[0].value, (count - 1) as f64);
}

#[test]
fn the_traffic_group_handle_writes_the_group_index_and_the_choice_makes_that_group_solid() {
    let mut document = office();
    document.traffic_groups = vec![
        model::TrafficGroup { id: "g1".into(), group: model::LoadGroup::GR1A, variable_id: document.variables[0].id.clone() },
        model::TrafficGroup { id: "g2".into(), group: model::LoadGroup::GR2, variable_id: document.variables[0].id.clone() },
    ];
    let all = handles(&document, &[], &[scene::EntityRef::new(scene::EntityKind::Group, COMBINATION_ENTITY, "combinations[id=combination]")]);
    assert_handle_laws(&all);
    let group = all.iter().find(|h| h.id == "combination:group").expect("traffic group handle");
    assert_eq!(group.writes[0].path, TRAFFIC_GROUP_PATH);
    assert_eq!(group.writes_at(0.4)[0].value, 0.0);
    assert_eq!(group.writes_at(1000.0)[0].value, 1.0);
    assert_eq!(present_groups(&document), vec![model::LoadGroup::GR1A, model::LoadGroup::GR2]);
    assert_eq!(traffic_group_at(&document, Some(1)), Some(model::LoadGroup::GR2));
    assert_eq!(traffic_group_at(&document, Some(5)), None);
}

#[test]
fn the_typed_view_carries_combination_and_group_without_a_string_codec_and_the_live_scene_honours_the_lane() {
    let mut document = office();
    document.traffic_groups = vec![
        model::TrafficGroup { id: "g1".into(), group: model::LoadGroup::GR1A, variable_id: document.variables[0].id.clone() },
        model::TrafficGroup { id: "g2".into(), group: model::LoadGroup::GR2, variable_id: document.variables[0].id.clone() },
    ];
    let view = view_of(Some("c1"), Some(model::LoadGroup::GR2));
    assert_eq!((view.combination(), view.traffic_group()), (Some("c1"), Some("GR2")));
    assert_eq!(view_of(None, None).stamp_text(), "");
    let lane = scene::projection::ViewOptions::default().with(scene::projection::ViewKey::TrafficGroupItem, scene::projection::ViewValue::Index(1)).with(scene::projection::ViewKey::CombinationItem, scene::projection::ViewValue::Index(0));
    let resolved = resolved_view(&document, &lane);
    assert_eq!(resolved.traffic_group(), Some("GR2"), "the lane's group position names the second declared group");
    assert_eq!(resolved.combination(), combinations::columns(&document).first().map(|column| column.id.as_str()), "the lane's combination position names that column");
    assert_eq!(resolved_view(&document, &scene::projection::ViewOptions::default().with(scene::projection::ViewKey::TrafficGroupItem, scene::projection::ViewValue::Index(9))).traffic_group(), None, "a stale position follows the combination");
    let report = inferences::evaluate(&document);
    let by_lane = live_scene_of(&document, &report, &lane);
    let placements = scene::projection_kit::placements_of(&document);
    let revision = scene::projection_kit::revision_of(&document, &placements, &report);
    let direct = project(&SceneInput { document: &document, report: &report, placements: &placements, revision, combination: resolved.combination(), traffic_group: Some(model::LoadGroup::GR2) });
    let shown = |scene: &StructuralScene| (scene.entities.iter().map(|e| e.entity.id.clone()).collect::<Vec<_>>(), scene.annotations.iter().map(|a| a.id().to_owned()).collect::<Vec<_>>());
    assert_eq!(shown(&by_lane), shown(&direct), "the live scene under the lane shows the same entities and glyphs as the plain projection of that group");
    let other = live_scene_of(&document, &report, &scene::projection::ViewOptions::default().with(scene::projection::ViewKey::TrafficGroupItem, scene::projection::ViewValue::Index(0)));
    assert!(by_lane.entities != other.entities || by_lane.annotations != other.annotations, "another group of the lane re-projects the traffic lanes");
}

#[test]
fn a_persisted_placement_overrides_the_default_layout() {
    let default = Layout::of(&office(), &[]);
    assert_eq!(default.member("beam-B1").expect("pose").frame.origin, scene::Vec3::ZERO);
    let placement = Placement { entity_id: "beam-B1".into(), origin: scene::Vec3::new(10.0, 2.0, 0.0), yaw: std::f64::consts::FRAC_PI_2 };
    let layout = Layout::of(&office(), &[placement]);
    let pose = layout.member("beam-B1").expect("pose");
    assert_eq!(pose.frame.origin, scene::Vec3::new(10.0, 2.0, 0.0));
    assert!(pose.frame.x.y > 0.999 && pose.frame.x.x.abs() < 1e-9);
}

#[test]
fn derived_scene_ids_resolve_to_their_document_entity() {
    let document = office();
    assert_eq!(entity_kind_of(&document, "beam-B1:mode").map(|k| k.collection), Some("members"));
    assert_eq!(entity_kind_of(&document, "Q-wind:beam-B1").map(|k| k.collection), Some("variables"));
    assert_eq!(entity_kind_of(&document, COMBINATION_ENTITY).map(|k| k.collection), Some("combinations"));
    assert_eq!(entity_kind_of(&document, "nothing"), None);
}

#[test]
fn dependencies_name_what_a_projection_reads() {
    let document = office();
    let member = entity_dependencies(&document, "beam-B1");
    for expected in ["G-sup", "G-inf", "Q-office", "Q-wind", "Q-snow"] {
        assert!(member.iter().any(|d| d == expected), "beam-B1 should depend on {expected}: {member:?}");
    }
    let action = entity_dependencies(&document, "Q-office");
    assert!(action.iter().any(|d| d == "beam-B1") && action.iter().any(|d| d == "Q-wind"), "{action:?}");
}

#[test]
fn every_diagram_validates_and_has_a_table_twin() {
    for (name, document) in fixtures() {
        let report = inferences::evaluate(&document);
        let all = diagrams(&document, &report, None, None);
        assert_diagram_laws(&all);
        let ids: BTreeSet<_> = all.iter().map(|d| d.id.clone()).collect();
        for expected in ["matrix:document", "psi:document", "reliability:document", "form:document", "annex-d:document"] {
            assert!(ids.contains(expected), "{name}: missing diagram {expected} in {ids:?}");
        }
        for diagram in &all {
            assert_eq!(diagram.validate(), Ok(()), "{name}: diagram {}", diagram.id);
            assert!(diagram.table_twin.rows() > 0, "{name}: diagram {} has no table twin", diagram.id);
        }
    }
    let fatigue = examples::fatigue_compliant::reference_snapshot();
    let all = diagrams(&fatigue, &inferences::evaluate(&fatigue), None, None);
    assert_diagram_laws(&all);
    assert!(all.iter().any(|d| d.id.starts_with("fatigue:")), "a member with R_d,FAT shows its damage diagram");
}

#[test]
fn the_matrix_repeats_the_generator_factors_and_hatches_the_leading_action() {
    let document = office();
    let report = inferences::evaluate(&document);
    let columns = combinations::columns(&document);
    let active = column_id(&document, combinations::Family::Str, "Q-wind");
    let set = diagrams(&document, &report, None, Some(&active));
    assert_diagram_laws(&set);
    let matrix = set.into_iter().find(|d| d.id == "matrix:document").expect("matrix");
    let actions = combinations::structure_actions(&document);
    for (r, action) in actions.iter().enumerate() {
        let table = &matrix.table_twin.columns[r + 1];
        assert_eq!(table.header.en, format!("γ·ψ of {}", action.id));
        for (c, column) in columns.iter().enumerate() {
            assert!((table.values[c] - column.factor(&action.id).unwrap_or(0.0)).abs() < 1e-12);
        }
    }
    let at = columns.iter().position(|c| c.id == active).expect("column");
    let row = actions.iter().position(|a| a.id == "Q-wind").expect("row");
    let hatched = matrix.series.iter().find_map(|s| match s {
        scene::Series::Polygon { id, hatch, .. } if id == &format!("cell-{at}-{row}") => Some(*hatch),
        _ => None,
    });
    assert_eq!(hatched, Some(Some(scene::Hatch::Diagonal)));
    assert!(matrix.series.iter().any(|s| s.id() == "active-column"));
    let marker = matrix.markers.iter().find(|m| m.id == format!("combination-{active}")).expect("column marker");
    assert_eq!(marker.tone, scene::Tone::Error);
    assert!(marker.bound_check.is_some());
    let cell = matrix.markers.iter().find(|m| m.id == format!("pick-{at}-{row}")).expect("cell marker");
    assert_eq!(cell.bound_check, marker.bound_check, "clicking a cell selects the worst check of its combination column");
}

#[test]
fn the_reliability_marker_sits_on_the_curve_and_writes_beta() {
    let document = office();
    let report = inferences::evaluate(&document);
    let set = diagrams(&document, &report, None, None);
    assert_diagram_laws(&set);
    let diagram = set.into_iter().find(|d| d.id == "reliability:document").expect("reliability");
    let marker = diagram.markers.iter().find(|m| m.id == "beta").expect("beta marker");
    assert!((marker.at.x - document.beta_computed).abs() < 1e-12);
    assert!((marker.at.y - checks::phi_cdf(-document.beta_computed)).abs() < 1e-15);
    let binding = marker.binding.as_ref().expect("binding");
    assert_eq!(binding.path, "betaComputed");
    assert!((binding.value_at(3.904) - 3.9).abs() < 1e-9);
    assert_eq!(marker.bound_check.as_deref(), Some("en1990.annex-c.beta"));
    assert!(diagram.axes.iter().any(|a| a.scale == scene::AxisScale::Log10));
}

#[test]
fn the_form_design_point_lies_on_the_reliability_circle_along_the_annex_c_alphas() {
    let document = office();
    let report = inferences::evaluate(&document);
    let set = diagrams(&document, &report, None, None);
    assert_diagram_laws(&set);
    let diagram = set.into_iter().find(|d| d.id == "form:document").expect("form");
    let star = &diagram.markers.iter().find(|m| m.id == "design-point").expect("design point").at;
    assert!((star.x.hypot(star.y) - document.beta_computed).abs() < 1e-9);
    assert!(star.x > 0.0 && star.y < 0.0, "high action effect, low resistance");
    let (alpha_r, alpha_e) = checks::annex_c_alphas(document.annex, 1.0, 1.0);
    assert!((star.x / star.y - alpha_e / alpha_r).abs() < 1e-9);
}

#[test]
fn the_annex_d_factors_are_the_table_values() {
    let document = office();
    let report = inferences::evaluate(&document);
    let set = diagrams(&document, &report, None, None);
    assert_diagram_laws(&set);
    let diagram = set.into_iter().find(|d| d.id == "annex-d:document").expect("annex d");
    let n = &diagram.table_twin.columns[0].values;
    let (known, unknown) = (&diagram.table_twin.columns[1].values, &diagram.table_twin.columns[2].values);
    let at = |nodes: &[f64], value: f64| nodes[n.iter().position(|x| (*x - value).abs() < 1e-9).expect("node")];
    assert!((at(known, 10.0) - 1.72).abs() < 1e-9 && (at(unknown, 10.0) - 1.92).abs() < 1e-9);
    assert!((at(known, 3.0) - 1.89).abs() < 1e-9 && (at(unknown, 3.0) - 3.37).abs() < 1e-9);
}

#[test]
fn focus_narrows_the_diagram_set() {
    let document = office();
    let report = inferences::evaluate(&document);
    let none = diagrams(&document, &report, Some("nothing"), None);
    assert_diagram_laws(&none);
    assert!(none.is_empty());
    let member = diagrams(&document, &report, Some("beam-B1"), None);
    assert_diagram_laws(&member);
    assert!(member.iter().any(|d| d.id == "matrix:document"));
    assert!(member.iter().all(|d| !d.id.starts_with("fatigue:")), "office beam has no R_d,FAT");
}

#[test]
fn golden_scene_matches_committed_bytes() {
    let scene = scene_of(&office());
    assert_scene_laws(&scene);
    let actual = canonical_json(&scene);
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧫️fixtures/🏛️scene/🔣️.json");
    scene::projection_kit::test_support::assert_golden(path, &actual);
}

fn oracle_document(document: &En1990Snapshot) -> String {
    let actions: Vec<String> = combinations::structure_actions(document)
        .iter()
        .map(|a| format!("{{\"id\":\"{}\",\"class\":\"{:?}\",\"value\":{},\"category\":\"{}\"}}", a.id, a.class, a.value, a.category))
        .collect();
    let members: Vec<String> = document.members.iter().map(|m| format!("{{\"id\":\"{}\",\"span\":{}}}", m.id, m.span)).collect();
    format!("{{\"annex\":\"{}\",\"kFi\":{},\"referencePeriodYears\":{},\"beta\":{},\"actions\":[{}],\"members\":[{}]}}", document.annex.label(), checks::k_fi_for_rc(document.reliability_class.number()).unwrap_or(1.0), document.reference_period_years, document.beta_computed, actions.join(","), members.join(","))
}

fn oracle_columns(document: &En1990Snapshot) -> String {
    let rows: Vec<String> = combinations::columns(document)
        .iter()
        .map(|c| {
            let factors: Vec<String> = c.combination.terms.iter().map(|t| format!("\"{}\":{}", t.action, t.factor)).collect();
            format!("{{\"id\":\"{}\",\"family\":\"{}\",\"equation\":\"{}\",\"leading\":\"{}\",\"factors\":{{{}}}}}", c.id, c.family.key(), c.equation().label(), c.leading(), factors.join(","))
        })
        .collect();
    rows.join(",")
}

#[test]
fn the_independent_oracle_agrees() {
    let document = office();
    let report = inferences::evaluate(&document);
    let scene = scene_of(&document);
    assert_scene_laws(&scene);
    let diagram_set = diagrams(&document, &report, None, None);
    assert_diagram_laws(&diagram_set);
    let diagram_json: Vec<String> = diagram_set.iter().map(canonical_json).collect();
    let office_lead = column_id(&document, combinations::Family::Str, "Q-office");
    let glyphs = scene_with(&document, Some(&office_lead));
    assert_scene_laws(&glyphs);
    let glyph_rows: Vec<String> = combinations::structure_actions(&document).iter().map(|a| format!("{{\"column\":\"{office_lead}\",\"action\":\"{}\",\"magnitude\":{}}}", a.id, magnitude(&glyphs, &format!("{}:beam-B1", a.id)))).collect();
    let input = format!("{{\"scene\":{},\"diagrams\":[{}],\"document\":{},\"columns\":[{}],\"glyphs\":[{}]}}", canonical_json(&scene), diagram_json.join(","), oracle_document(&document), oracle_columns(&document), glyph_rows.join(","));
    let script = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🔮️oracles/🐍️.py");
    let output = norm::python_oracle::run_ok(script, &[], Some(input.as_bytes()));
    let verdict = String::from_utf8_lossy(&output.stdout);
    assert!(verdict.contains("\"ok\": true"), "oracle verdict: {verdict}");
}

#[test]
fn projecting_a_thousand_members_stays_interactive() {
    let mut document = office();
    let template = document.members[0].clone();
    document.members = (0..1000).map(|i| model::Member { id: format!("m{i}"), ..template.clone() }).collect();
    document.effects.clear();
    let report = CheckReport::default();
    let started = std::time::Instant::now();
    let scene = project(&SceneInput { document: &document, report: &report, placements: &[], revision: revision(), combination: None, traffic_group: None });
    assert_scene_laws(&scene);
    assert!(scene.entities.len() >= 1000);
    eprintln!("[latency] en1990/scene-cold-1000 p50_ms={0:.3} p95_ms={0:.3} budget_ms={1:.1}", started.elapsed().as_secs_f64() * 1e3, 2000.0);
    assert!(started.elapsed().as_millis() < 2000, "projection took {:?}", started.elapsed());
}

#[test]
fn the_combination_track_scrubs_every_generated_column() {
    let document = office();
    let scene = scene_of(&document);
    assert_scene_laws(&scene);
    let columns = combinations::columns(&document);
    let track = scene.tracks.iter().find(|t| t.id == "combination").expect("combination track");
    assert_eq!((track.kind.clone(), track.min, track.max, track.step, track.keyframes.len()), (scene::TrackKind::Combination, 1.0, columns.len() as f64, 1.0, columns.len()));
    assert_eq!(scene.validate(), Ok(()));
    let loads: Vec<_> = scene.entities.iter().filter(|e| matches!(e.geometry, scene::Geometry::Load(_))).collect();
    assert!(!loads.is_empty());
    for e in loads {
        let scene::ColourChannel::Fields { fields, .. } = &e.appearance.channel else { panic!("{} has no fields", e.entity.id) };
        assert!(track.keyframes.iter().all(|k| fields.iter().any(|f| Some(&f.id) == k.field.as_ref())));
    }
}

#[test]
fn a_combination_frame_holds_the_generator_factor_of_the_action() {
    let document = office();
    let id = column_id(&document, combinations::Family::Str, "Q-office");
    let column = combinations::columns(&document).into_iter().find(|c| c.id == id).expect("column");
    let scene = scene_of(&document);
    assert_scene_laws(&scene);
    let glyph = scene.entities.iter().find(|e| e.entity.id.starts_with("Q-office:")).expect("Q-office glyph");
    let scene::ColourChannel::Fields { fields, .. } = &glyph.appearance.channel else { panic!("fields") };
    let field = fields.iter().find(|f| f.id == format!("combination:{id}")).expect("field");
    assert_eq!(field.samples, scene::FieldSamples::Instance { values: vec![column.factor("Q-office").expect("factor")] });
}

#[test]
fn the_sls_limit_lines_follow_the_elastic_line_and_the_span_ratio() {
    let document = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let report = inferences::evaluate(&document);
    let member = &document.members[0];
    let set = diagrams(&document, &report, None, None);
    assert_diagram_laws(&set);
    let diagram = set.into_iter().find(|d| d.id == format!("sls-limits:{}", member.id)).expect("sls limit diagram");
    assert_eq!(diagram.validate(), Ok(()));
    let net = (member.deflection_w1 + member.deflection_w2 + member.deflection_w3 - member.precamber) * 1000.0;
    let Series::Polyline { points, .. } = &diagram.series[0] else { panic!("net line") };
    assert!((points[points.len() / 2].y - net).abs() < 1e-9 * net.abs().max(1.0));
    assert!((diagrams::sls_line(0.5) - 1.0).abs() < 1e-12);
    let Series::Limit { value, .. } = &diagram.series[2] else { panic!("limit") };
    assert!((value - member.span / member.deflection_limit_ratio * 1000.0).abs() < 1e-9);
    assert!(diagram.markers[0].bound_check.is_some());
}

fn analysed() -> En1990Snapshot {
    use model::{FatigueSpectrum, ReliabilityAnalysis, TestSeries};
    let mut document = office();
    document.fatigue_spectra = vec![FatigueSpectrum { id: "FS-1".into(), member_id: "beam-B1".into(), detail_category: 71.0e6, stress_range: 80.0e6, cycles: 2.0e6 }];
    document.reliability_analyses = vec![ReliabilityAnalysis { id: "RA-1".into(), resistance_distribution: semio_s_artifact_norm_en1990_model::DistributionKind::Lognormal, resistance_mean: 600.0e3, resistance_sd: 60.0e3, action_distribution: semio_s_artifact_norm_en1990_model::DistributionKind::Gumbel, action_mean: 250.0e3, action_sd: 50.0e3, second_action_distribution: semio_s_artifact_norm_en1990_model::DistributionKind::Normal, second_action_mean: 0.0, second_action_sd: 0.0, member_id: String::new() }];
    document.test_series = vec![TestSeries { id: "TS-1".into(), quantity: "strength".into(), distribution: semio_s_artifact_norm_en1990_model::TestDistribution::Normal, sample_size: 10, mean: 30.0, std_dev: 3.0, variation_known: 0.0, conversion_factor: 1.0, required: 20.0, pairs: String::new(), theory_variation: 0.0, theory_mean: 0.0 }];
    document
}

#[test]
fn analysis_rows_become_bound_bar_entities_with_grips_and_diagrams() {
    let document = analysed();
    let scene = scene_of(&document);
    assert_scene_laws(&scene);
    assert_eq!(scene.validate(), Ok(()));
    for id in ["FS-1", "RA-1", "RA-1:action", "TS-1", "TS-1:required"] {
        assert!(scene.entities.iter().any(|e| e.entity.id == id), "missing entity {id}");
    }
    assert!(entity(&scene, "FS-1").utilisation.as_ref().is_some_and(|u| u.status == scene::CheckStatus::Fail), "the Miner damage colours the bin");
    assert!(entity(&scene, "RA-1").utilisation.is_some() && entity(&scene, "TS-1").utilisation.is_some());
    let selection = ["FS-1", "RA-1", "TS-1"].map(|id| scene::EntityRef::new(scene::EntityKind::Plate, id, ""));
    let all = handles(&document, &[], &selection);
    assert_handle_laws(&all);
    let ids: BTreeSet<_> = all.iter().map(|h| h.id.as_str()).collect();
    for id in ["FS-1:stressRange", "RA-1:resistanceMean", "RA-1:actionMean", "TS-1:required"] {
        assert!(ids.contains(id), "missing handle {id} in {ids:?}");
    }
    let grip = all.iter().find(|h| h.id == "FS-1:stressRange").expect("grip");
    assert_eq!(grip.writes[0].path, "fatigueSpectra[id=FS-1].stressRange");
    let resolved = grip.writes_at(1.6);
    assert!((resolved[0].value - 80.0e6).abs() < 1.0, "grip height 1.6 m is 80 MPa, got {}", resolved[0].value);
    let report = inferences::evaluate(&document);
    let diagram_set = diagrams::diagrams(&document, &report, None, None);
    assert_diagram_laws(&diagram_set);
    let diagram_ids: Vec<_> = diagram_set.into_iter().map(|d| d.id).collect();
    assert!(diagram_ids.iter().any(|id| id == "form:RA-1") && diagram_ids.iter().any(|id| id == "fatigue:beam-B1"), "{diagram_ids:?}");
}

#[test]
fn multi_action_form_bars_the_test_scatter_and_the_member_badge_follow_the_document() {
    let mut document = analysed();
    document.reliability_analyses[0].second_action_mean = 80.0e3;
    document.reliability_analyses[0].second_action_sd = 20.0e3;
    document.reliability_analyses[0].member_id = "beam-B1".into();
    document.test_series[0].pairs = "412:400;388:380;455:430;352:360;431:410;398:390;376:370;423:405".into();
    document.test_series[0].theory_variation = 0.09;
    document.test_series[0].theory_mean = 400.0;
    document.test_series[0].required = 300.0;
    let report = inferences::evaluate(&document);
    let diagram_set = diagrams::diagrams(&document, &report, None, None);
    assert_diagram_laws(&diagram_set);
    let bars = diagram_set.iter().find(|d| d.id == "form-bars:RA-1").expect("form bars");
    assert_eq!(bars.markers.len(), 3, "R, E1 and E2");
    let squares: f64 = bars.markers.iter().map(|m| m.at.y * m.at.y).sum();
    assert!((squares - 1.0).abs() < 1e-6, "sum of alpha squared is one, got {squares}");
    assert!(bars.markers[0].at.y > 0.0 && bars.markers[1].at.y < 0.0 && bars.markers[2].at.y < 0.0);
    let scatter = diagram_set.iter().find(|d| d.id == "test-pairs:TS-1").expect("test pairs");
    assert!(scatter.series.iter().any(|series| matches!(series, scene::Series::Scatter { points, .. } if points.len() == 8)));
    assert!(scatter.markers.iter().any(|m| m.id == "r-k") && scatter.markers.iter().any(|m| m.id == "r-d"));
    assert_eq!(scatter.markers[0].bound_check.as_deref(), Some("en1990.annex-d.TS-1.standard"));
    let scene = scene_of(&document);
    assert_scene_laws(&scene);
    assert!(scene.annotations.iter().any(|a| a.id() == "beam-B1:reliability:RA-1"), "the member carries the reliability badge");
    assert!(scene.entities.iter().any(|e| e.entity.id == "RA-1:action2"), "the second action has a bar");
    let selection = ["RA-1", "FS-1", "beam-B1"].map(|id| scene::EntityRef::new(scene::EntityKind::Plate, id, ""));
    let all = handles(&document, &[], &selection);
    assert_handle_laws(&all);
    let ids: BTreeSet<_> = all.iter().map(|h| h.id.as_str()).collect();
    for id in ["RA-1:secondActionMean", "RA-1:secondActionSd", "FS-1:detailCategory", "FS-1:cycles", "beam-B1:rdStr", "beam-B1:slsLimitChar"] {
        assert!(ids.contains(id), "missing handle {id} in {ids:?}");
    }
    assert!(entity_dependencies(&document, "RA-1").contains(&"beam-B1".to_string()));
}

#[test]
fn analysis_entities_depend_on_their_member() {
    let document = analysed();
    assert!(entity_dependencies(&document, "FS-1").contains(&"beam-B1".to_string()));
    assert!(entity_dependencies(&document, "beam-B1").contains(&"FS-1".to_string()));
    assert_eq!(entity_kind_of(&document, "RA-1").map(|k| k.collection), Some("reliabilityAnalyses"));
}

#[test]
fn deflection_components_and_scatter_have_grips_that_write_their_leaves() {
    let document = analysed();
    let selection = ["beam-B1", "RA-1"].map(|id| scene::EntityRef::new(scene::EntityKind::Member, id, ""));
    let all = handles(&document, &[], &selection);
    assert_handle_laws(&all);
    for (id, path) in [
        ("beam-B1:w1", "members[id=beam-B1].deflectionW1"),
        ("beam-B1:w2", "members[id=beam-B1].deflectionW2"),
        ("beam-B1:w3", "members[id=beam-B1].deflectionW3"),
        ("RA-1:resistanceSd", "reliabilityAnalyses[id=RA-1].resistanceSd"),
        ("RA-1:actionSd", "reliabilityAnalyses[id=RA-1].actionSd"),
    ] {
        let grip = all.iter().find(|h| h.id == id).unwrap_or_else(|| panic!("missing handle {id}"));
        assert_eq!(grip.writes[0].path, path);
    }
    let w3 = all.iter().find(|h| h.id == "beam-B1:w3").expect("w3");
    let net = document.members[0].deflection_w1 + document.members[0].deflection_w2 + document.members[0].deflection_w3 - document.members[0].precamber;
    let scale = (document.members[0].span / 25.0 / net.abs()).clamp(1.0, 200.0);
    let resolved = w3.writes_at(document.members[0].deflection_w3 * scale);
    assert!((resolved[0].value - document.members[0].deflection_w3).abs() < 1e-3, "the grip distance over the exaggeration is the component, got {}", resolved[0].value);
    let sd = all.iter().find(|h| h.id == "RA-1:resistanceSd").expect("sd");
    let at = |s: f64| sd.writes_at(s)[0].value;
    assert!(at(2.0) > at(1.0), "the whisker grows with the pointer");
    let scene = scene_of(&document);
    assert_scene_laws(&scene);
    assert!(entity(&scene, "RA-1").handles.iter().any(|h| h == "RA-1:resistanceSd") && entity(&scene, "RA-1:action").handles.iter().any(|h| h == "RA-1:actionSd"));
    assert!(entity(&scene, "beam-B1").handles.iter().any(|h| h == "beam-B1:w1"));
}

#[test]
fn the_gamma_psi_bar_chart_scales_each_variable_action_and_has_a_table_twin() {
    let document = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let report = inferences::evaluate(&document);
    let set = diagrams(&document, &report, None, None);
    assert_diagram_laws(&set);
    let chart = set.into_iter().find(|d| d.id == "gamma-psi:document").expect("gamma psi chart");
    assert_eq!(chart.validate(), Ok(()));
    assert_eq!(chart.series.len() % 3, 0);
    assert!(!chart.series.is_empty() && chart.table_twin.rows() >= 1);
}

fn fold_into(document: &mut En1990Snapshot, commit: &scene::projection_kit::PlacementCommit) {
    let mut rows = scene::projection_kit::placements_of(document);
    commit.apply(&mut rows);
    document.placements = rows.into_iter().map(|row| model::Placement { id: row.entity_id, x: row.origin.x, y: row.origin.y, z: row.origin.z, yaw: row.yaw }).collect();
}

#[test]
fn dragging_an_entity_commits_one_undoable_event_and_the_reload_stands_at_the_same_pose() {
    use scene::projection_kit::{PlacementCommit, PlacementMove};
    let mut document = office();
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
fn the_envelope_highlights_the_governing_ultimate_combination_between_its_limits() {
    for (name, document) in fixtures() {
        let report = inferences::evaluate(&document);
        let set = diagrams(&document, &report, None, None);
        assert_diagram_laws(&set);
        let envelope = set.into_iter().find(|d| d.id == "envelope:document").unwrap_or_else(|| panic!("{name}: envelope"));
        let columns = combinations::columns(&document);
        assert_eq!(envelope.table_twin.rows(), columns.len(), "{name}: one bar per combination");
        let totals = &envelope.table_twin.columns[1].values;
        let governing: Vec<usize> = envelope.table_twin.columns[2].values.iter().enumerate().filter(|(_, flag)| **flag == 1.0).map(|(index, _)| index).collect();
        assert_eq!(governing.len(), 1, "{name}: exactly one governing combination");
        let ultimate = |family: combinations::Family| matches!(family, combinations::Family::Str | combinations::Family::Equ | combinations::Family::Acc | combinations::Family::Sei);
        let best = columns.iter().enumerate().filter(|(_, column)| ultimate(column.family)).map(|(index, _)| totals[index]).fold(f64::MIN, f64::max);
        assert!((totals[governing[0]] - best).abs() < 1e-9, "{name}: the governing bar is the largest ultimate effect");
        let high = totals.iter().copied().fold(0.0, f64::max);
        assert!(envelope.series.iter().any(|series| matches!(series, scene::Series::Limit { id, value, .. } if id == "envelope-max" && (*value - high).abs() < 1e-9)), "{name}: envelope maximum");
        assert!(envelope.markers.iter().any(|marker| marker.id.starts_with("envelope-") && marker.bound_check.is_some()), "{name}: markers bind to checks");
    }
}

#[test]
fn the_beta_target_diagram_draws_every_class_and_places_the_project() {
    let document = office();
    let report = inferences::evaluate(&document);
    let set = diagrams(&document, &report, None, None);
    assert_diagram_laws(&set);
    let diagram = set.into_iter().find(|d| d.id == "beta-targets:document").expect("beta targets");
    for rc in 1..=3u8 {
        assert!(diagram.series.iter().any(|series| series.id() == format!("rc{rc}")), "RC{rc} target line");
    }
    let target = checks::target_reliability_index_for(document.reliability_class.number(), document.reference_period_years).expect("target");
    let marker = diagram.markers.iter().find(|m| m.id == "reference:target").expect("target marker");
    assert!((marker.at.y - target).abs() < 1e-12 && (marker.at.x - document.reference_period_years).abs() < 1e-12);
    let computed = diagram.markers.iter().find(|m| m.id == "computed").expect("computed marker");
    assert_eq!(computed.bound_check.as_deref(), Some("en1990.annex-c.beta"));
    assert!(diagram.table_twin.rows() == 7 && diagram.table_twin.columns.len() == 7);
    let (g_sup, _) = checks::gamma_g(document.annex, checks::Domain::Building, checks::FactorSet::StrB);
    for rc in 1..=3u8 {
        let column = diagram.table_twin.columns.iter().find(|c| c.header.en == format!("K_FI·γ_G,sup of RC{rc}")).expect("K_FI column");
        assert!((column.values[0] - checks::k_fi_for_rc(rc).expect("k_fi") * g_sup).abs() < 1e-12);
    }
}

fn overridden_office() -> En1990Snapshot {
    let mut document = office();
    document.annex = semio_s_plugin_norm_compliance::AnnexChoice::De;
    document.factor_overrides = vec![
        model::FactorOverride { id: "O1".into(), factor: "psi0".into(), target: "Q-wind".into(), value: 0.3, justification: "wind tunnel study".into() },
        model::FactorOverride { id: "O2".into(), factor: "gamma".into(), target: "Q-office".into(), value: 1.2, justification: "controlled use".into() },
    ];
    document
}

#[test]
fn factor_overrides_show_deviation_badges_handles_and_a_table_twin_diagram() {
    let document = overridden_office();
    let _projected = scene_of(&document);
    assert_scene_laws(&_projected);
    let report = inferences::evaluate(&document);
    assert_eq!(report.checks.iter().find(|c| c.id == "en1990.override.O1").map(|c| c.status), Some(norm::CheckStatus::Warning));
    assert_eq!(report.checks.iter().find(|c| c.id == "en1990.override.O2").map(|c| c.status), Some(norm::CheckStatus::Fail), "the German annex fixes gamma");
    let layout = Layout::of(&document, &[]);
    let wind = handles::ids(&document, &layout, "Q-wind");
    assert!(wind.contains(&"Q-wind:ovr-O1".to_string()) && wind.contains(&"Q-wind:magnitude".to_string()), "{wind:?}");
    let handle = handles::build(&document, &layout, "Q-wind").into_iter().find(|h| h.id == "Q-wind:ovr-O1").expect("override handle");
    assert_eq!(handle.writes[0].path, "factorOverrides[id=O1].value");
    assert!(handle.accessible_name.en.contains("deviates from NA") && handle.accessible_name.de.contains("weicht vom NA ab"));
    assert!(handles::action_ids(&document, "Q-wind").contains(&"Q-wind:ovr-O1".to_string()));
    let set = diagrams(&document, &report, None, None);
    assert_diagram_laws(&set);
    let diagram = set.into_iter().find(|d| d.id == "factor-overrides:document").expect("override diagram");
    assert_eq!(diagram.table_twin.rows(), 2);
    assert!((diagram.table_twin.columns[2].values[0] - 0.3).abs() < 1e-12);
    assert_eq!(diagram.table_twin.columns[3].values, vec![1.0, 0.0]);
    assert!(diagram.markers.iter().all(|m| m.bound_check.is_some() && m.binding.is_some()));
    let plain = diagrams(&office(), &inferences::evaluate(&office()), None, None);
    assert_diagram_laws(&plain);
    assert!(plain.iter().all(|d| d.id != "factor-overrides:document"));
}

#[test]
fn factor_override_rows_are_outliner_entities_with_bars_handles_and_action_badges_and_reset_clears_them() {
    let document = overridden_office();
    let scene = scene_of(&document);
    assert_scene_laws(&scene);
    let ids: BTreeSet<&str> = scene.entities.iter().map(|e| e.entity.id.as_str()).collect();
    assert!(ids.contains("O1") && ids.contains("O1:annex") && ids.contains("O2"), "{ids:?}");
    let bar = scene.entities.iter().find(|e| e.entity.id == "O1").expect("override bar");
    assert!(bar.entity.path.starts_with("factorOverrides["), "{}", bar.entity.path);
    assert!(bar.accessible_detail.en.contains("deviates from NA") && bar.accessible_detail.de.contains("weicht vom NA ab"));
    assert!(bar.utilisation.is_some(), "the bar is bound to its override check");
    assert!(entities_of(&document).iter().any(|e| e.id == "O1") && entity_kind_of(&document, "O1").map(|k| k.id) == Some("factorOverride"));
    let layout = Layout::of(&document, &[]);
    let handle = handles::build(&document, &layout, "O1").into_iter().find(|h| h.id == "O1:value").expect("override bar grip");
    assert_eq!(handle.writes[0].path, "factorOverrides[id=O1].value");
    assert!(handles::build(&document, &layout, "O1:annex").iter().all(|h| h.id != "O1:annex:value"));
    let badge = scene.annotations.iter().find_map(|a| match a {
        scene::Annotation::Label { id, text, tone, .. } if id == "badge:override:Q-wind:beam-B1" => Some((text.clone(), *tone)),
        _ => None,
    });
    let (text, tone) = badge.expect("the affected action carries a badge");
    assert!(text.en.contains("ψ₀ 0.300 (NA ") && text.de.contains("Weicht vom NA ab") && tone == scene::Tone::Warning, "{}", text.en);
    let annex = inferences::factor_overrides::annex_for_action(&document, "psi0", "Q-wind").expect("annex psi0");
    let mut reset = document.clone();
    reset.factor_overrides[0].value = annex;
    let after = scene_of(&reset);
    assert_scene_laws(&after);
    assert!(after.annotations.iter().all(|a| a.id() != "badge:override:Q-wind:beam-B1"), "a row reset to the annex value no longer deviates");
    assert!(entity_dependencies(&document, "Q-wind").contains(&"O1".to_string()) && entity_dependencies(&document, "O1").contains(&"Q-wind".to_string()));
}

#[test]
fn the_governing_combination_of_every_member_is_labelled_in_3d_and_drawn_in_the_sweep_diagram() {
    let document = office();
    let report = inferences::evaluate(&document);
    let scene = scene_of(&document);
    assert_scene_laws(&scene);
    let label = scene.annotations.iter().find_map(|a| match a {
        scene::Annotation::Label { id, text, .. } if id == "beam-B1:governing" => Some(text.clone()),
        _ => None,
    });
    let label = label.expect("governing label");
    assert!(label.en.contains("Governing:") && label.de.contains("Maßgebend:"), "{}", label.en);
    let columns = combinations::columns(&document);
    let set = diagrams(&document, &report, None, None);
    assert_diagram_laws(&set);
    let sweep = set.iter().find(|d| d.id == "governing-combinations:document").expect("sweep diagram");
    let marker = sweep.markers.iter().find(|m| m.id == "governing-beam-B1").expect("governing marker");
    let check = report.checks.iter().find(|c| Some(&c.id) == marker.bound_check.as_ref()).expect("bound check");
    assert_eq!(check.subject.entity_id, "beam-B1");
    let worst = report.checks.iter().filter(|c| c.subject.entity_id == "beam-B1" && c.utilization.is_finite() && combinations::column_for_check(&columns, c).is_some()).map(|c| c.utilization).fold(0.0, f64::max);
    assert!((check.utilization - worst).abs() < 1e-12, "the marker is bound to the governing check");
    assert_eq!(sweep.markers.iter().filter(|m| m.id.starts_with("sweep-")).count(), columns.len());
    assert_eq!(sweep.table_twin.columns[0].values.len(), columns.len());
}

#[test]
fn rail_diagrams_draw_the_frequency_band_and_the_resonance_sweep_with_table_twins() {
    let mut document = office();
    document.structure_kind = semio_s_artifact_norm_en1990_model::StructureKind::RailBridge;
    document.members[0].span = 20.0;
    document.bridge_sls = vec![model::BridgeSls { id: "BS".into(), member_id: "beam-B1".into(), deck_acceleration: 0.0, deck_acceleration_limit: 0.0, deck_twist: 0.0, deck_twist_limit: 0.0, bridge_deflection: 0.0, bridge_deflection_limit: 0.0, track_gauge: 0.0, line_speed: 250.0, ballast_depth: 0.35, deck_mass: 5000.0, natural_frequency: 4.0, end_rotation: 0.0, basic_wind_speed: 0.0, damping_ratio: 0.01 }];
    let report = inferences::evaluate(&document);
    let all = diagrams(&document, &report, None, None);
    assert_diagram_laws(&all);
    let band = all.iter().find(|d| d.id == "rail-frequency-limits:BS").expect("band");
    assert_eq!(band.table_twin.columns.len(), 3);
    assert!(band.table_twin.columns[1].values.iter().zip(&band.table_twin.columns[2].values).all(|(lower, upper)| lower < upper));
    assert_eq!(band.markers[0].bound_check.as_deref(), Some("en1990.a2.rail-dynamic.beam-B1.BS"));
    let sweep = all.iter().find(|d| d.id == "rail-resonance:BS").expect("sweep");
    assert_eq!(sweep.table_twin.rows(), 40);
    assert!(sweep.markers.iter().any(|m| m.id == "governing" && m.bound_check.as_deref() == Some("en1990.a2.rail-resonance.beam-B1.BS")));
    assert!(sweep.table_twin.columns[1].values.iter().any(|a| *a > 0.0));
    let plain = diagrams(&office(), &inferences::evaluate(&office()), None, None);
    assert_diagram_laws(&plain);
    assert!(plain.iter().all(|d| !d.id.starts_with("rail-")));
}

#[test]
fn the_traffic_group_diagram_marks_components_and_binds_the_check() {
    let mut document = office();
    document.structure_kind = semio_s_artifact_norm_en1990_model::StructureKind::RoadBridge;
    let template = document.variables[0].clone();
    document.variables = [("Q-ts", "road_traffic", "gr1a"), ("Q-sa", "road_single_axle", "gr1a")]
        .into_iter()
        .map(|(id, category, group)| {
            let mut variable = template.clone();
            variable.id = id.into();
            variable.category = category.into();
            variable.group = group.into();
            variable
        })
        .collect();
    let report = inferences::evaluate(&document);
    let set = diagrams(&document, &report, None, None);
    assert_diagram_laws(&set);
    let diagram = set.into_iter().find(|d| d.id == "traffic-groups:document").expect("traffic groups");
    assert_eq!(diagram.markers.len(), 2);
    assert!(diagram.markers.iter().all(|m| m.bound_check.as_deref() == Some("en1990.a2.traffic-groups") && m.tone == scene::Tone::Warning));
    let check = report.checks.iter().find(|c| c.id == "en1990.a2.traffic-groups").expect("check");
    assert!((check.utilization - 2.0).abs() < 1e-12, "1 + offending share");
}

#[test]
fn traffic_group_components_stand_on_the_deck_and_only_the_active_group_is_solid() {
    use model::{LoadGroup, TrafficGroup};
    let mut document = office();
    document.structure_kind = semio_s_artifact_norm_en1990_model::StructureKind::RoadBridge;
    let template = document.variables[0].clone();
    document.variables = [("Q-ts", "road_traffic", 60_000.0), ("Q-sa", "road_single_axle", 50_000.0)]
        .into_iter()
        .map(|(id, category, qk)| {
            let mut variable = template.clone();
            variable.id = id.into();
            variable.category = category.into();
            variable.qk = qk;
            variable
        })
        .collect();
    document.effects = document.variables.iter().map(|v| model::MemberEffect { member_id: "beam-B1".into(), action_id: v.id.clone(), influence: 1.0 }).collect();
    document.traffic_groups = vec![
        TrafficGroup { id: "TG-ts".into(), group: LoadGroup::GR1A, variable_id: "Q-ts".into() },
        TrafficGroup { id: "TG-sa".into(), group: LoadGroup::GR1B, variable_id: "Q-sa".into() },
    ];
    let scene = scene_of(&document);
    assert_scene_laws(&scene);
    assert_eq!(entity(&scene, "TG-ts:beam-B1").appearance.style, scene::StyleState::Normal, "gr1a acts in the governing combination");
    assert_eq!(entity(&scene, "TG-sa:beam-B1").appearance.style, scene::StyleState::Ghost, "gr1b is a ghost while gr1a is active");
    let ids: BTreeSet<_> = scene.annotations.iter().map(|a| a.id().to_string()).collect();
    assert!(ids.contains("badge:traffic-group:gr1a") && ids.contains("badge:traffic-group:gr1b"), "{ids:?}");
}

fn diagram_of(document: &En1990Snapshot, report: &CheckReport, id: &str) -> scene::Diagram2d {
    diagrams(document, report, None, None).into_iter().find(|d| d.id == id).unwrap_or_else(|| panic!("diagram {id}"))
}

fn values(diagram: &scene::Diagram2d, header: &str) -> Vec<f64> {
    diagram.table_twin.columns.iter().find(|c| c.header.en == header).unwrap_or_else(|| panic!("column {header}")).values.clone()
}

#[test]
fn a_magnitude_drag_through_the_live_path_reranks_the_combinations_and_moves_the_governing_marker() {
    let mut document = office();
    let wind = column_id(&document, combinations::Family::Str, "Q-wind");
    let base = inferences::live::report(&document);
    let before_ranking = diagram_of(&document, &base, "combination-ranking:document");
    let before_envelope = diagram_of(&document, &base, "envelope:document");
    let mut previous_top = values(&before_ranking, "Utilisation")[0];
    for _ in 0..4 {
        document.variables.iter_mut().find(|v| v.id == "Q-wind").expect("wind").qk *= 4.0;
        let live = inferences::live::report(&document);
        let full = inferences::evaluate(&document);
        let (ranking, reference) = (diagram_of(&document, &live, "combination-ranking:document"), diagram_of(&document, &full, "combination-ranking:document"));
        assert_diagram_laws(&[ranking.clone(), diagram_of(&document, &live, "envelope:document")]);
        let (u_live, u_full) = (values(&ranking, "Utilisation"), values(&reference, "Utilisation"));
        assert_eq!(u_live.len(), u_full.len());
        assert!(u_live.iter().zip(&u_full).all(|(a, b)| (a - b).abs() < 1e-9), "the live path ranks like the full evaluation");
        assert!(u_live.windows(2).all(|w| w[0] >= w[1]), "bars are sorted by descending utilisation: {u_live:?}");
        assert!(u_live[0] >= previous_top - 1e-12, "a larger wind never lowers the governing utilisation");
        previous_top = u_live[0];
    }
    let live = inferences::live::report(&document);
    let ranking = diagram_of(&document, &live, "combination-ranking:document");
    let governing = ranking.markers.iter().find(|m| matches!(m.shape, scene::MarkerShape::Diamond)).expect("governing marker");
    assert!(governing.id.ends_with(":Q-wind"), "a dominant wind makes a wind-led combination govern: {}", governing.id);
    assert!(values(&ranking, "Utilisation")[0] > values(&before_ranking, "Utilisation")[0]);
    let after_envelope = diagram_of(&document, &live, "envelope:document");
    assert!(values(&after_envelope, "Design effect Σγψ·E").iter().zip(values(&before_envelope, "Design effect Σγψ·E")).any(|(after, before)| (after - before).abs() > 1.0), "every factored effect bar follows the dragged magnitude");
    let at = combinations::columns(&document).iter().position(|c| c.id == wind).expect("column");
    assert_eq!(values(&after_envelope, "Governing")[at], 1.0, "the envelope marks the same combination as governing");
}

#[test]
fn the_beta_k_fi_diagram_moves_its_design_point_with_the_reliability_class() {
    use norm::ReliabilityClass;
    let mut document = office();
    let mut seen = Vec::new();
    for class in [ReliabilityClass::RC1, ReliabilityClass::RC2, ReliabilityClass::RC3] {
        document.reliability_class = class;
        let report = inferences::live::report(&document);
        let set = diagrams(&document, &report, None, None);
        assert_diagram_laws(&set);
        let diagram = set.into_iter().find(|d| d.id == "beta-k-fi:document").expect("beta over K_FI");
        let point = diagram.markers.iter().find(|m| m.id == "design-point").expect("design point");
        assert!((point.at.x - class.k_fi(document.annex)).abs() < 1e-12, "x is K_FI of the declared class");
        assert!((point.at.y - document.beta_computed.clamp(0.0, 7.0)).abs() < 1e-12, "y is the computed beta");
        assert_eq!(point.bound_check.as_deref(), Some("en1990.annex-c.beta"));
        assert_eq!(point.binding.as_ref().map(|b| b.path.as_str()), Some("betaComputed"));
        let target = diagram.markers.iter().find(|m| m.id == format!("reference:class-rc{}", class.number())).expect("class marker");
        assert!((target.at.y - checks::target_reliability_index_for(class.number(), document.reference_period_years).expect("target")).abs() < 1e-12);
        seen.push(point.at.x);
    }
    assert!(seen.windows(2).all(|w| w[0] != w[1]), "each class puts the design point at its own K_FI: {seen:?}");
}

#[test]
fn the_gamma_bands_colour_each_permanent_action_by_sup_or_inf_per_combination() {
    let document = office();
    let scene = scene_of(&document);
    assert_scene_laws(&scene);
    let bands = entity(&scene, "beam-B1:gamma-bands");
    let scene::ColourChannel::Fields { active, fields } = &bands.appearance.channel else { panic!("bands carry the per-combination fields") };
    let columns = combinations::columns(&document);
    assert_eq!(fields.len(), columns.len(), "one field per combination column, the ones the combination stepper selects");
    assert!(fields.iter().any(|f| &f.id == active));
    let column = columns.iter().find(|c| c.family == combinations::Family::Str).expect("str column");
    let field = fields.iter().find(|f| f.id == super::dynamics::field_id(column)).expect("field of the column");
    let scene::FieldSamples::Vertex { values } = &field.samples else { panic!("vertex samples") };
    assert_eq!(values.len(), 2 * 4, "two rows, two vertices per band, two permanent actions");
    let (sup, inf) = (values[0], values[2]);
    assert_eq!(values[1], sup);
    assert_eq!(values[3], inf);
    assert_eq!(sup, column.factor("G-sup").expect("G-sup factor"), "the adding action takes the unfavourable factor of the column");
    assert!(inf < column.factor("G-inf").expect("G-inf factor"), "the relieving action takes the favourable factor");
    assert!(scene.legends.iter().any(|l| l.id == "gamma"), "the colour band has its legend");
}

#[test]
fn the_bridge_effect_and_spectrum_diagrams_project_the_document_with_table_twins() {
    let mut document = analysed();
    document.bridge_sls = vec![model::BridgeSls { id: "BS".into(), member_id: "beam-B1".into(), bridge_deflection: 0.02, bridge_deflection_limit: 0.04, deck_twist: 3.0, deck_twist_limit: 4.5, deck_acceleration: 2.0, deck_acceleration_limit: 3.5, track_gauge: 0.0, line_speed: 0.0, ballast_depth: 0.0, deck_mass: 0.0, natural_frequency: 0.0, end_rotation: 0.0, basic_wind_speed: 0.0, damping_ratio: 0.0 }];
    let mut second = document.fatigue_spectra[0].clone();
    second.id = "FS-2".into();
    second.stress_range = 40.0e6;
    second.cycles = 1.0e6;
    document.fatigue_spectra.push(second);
    let report = inferences::evaluate(&document);
    let set = diagrams(&document, &report, None, None);
    assert_diagram_laws(&set);
    let bridge = set.iter().find(|d| d.id == "bridge-sls:BS").expect("bridge serviceability diagram");
    assert_eq!(bridge.table_twin.rows(), 3);
    assert!((bridge.table_twin.columns[3].values[0] - 0.5).abs() < 1e-12 && bridge.markers.iter().any(|m| m.binding.as_ref().is_some_and(|b| b.path == "bridgeSls[id=BS].bridgeDeflection")));
    let effects = set.iter().find(|d| d.id == "effects:beam-B1").expect("effect envelope");
    assert!(effects.table_twin.rows() >= 1 && effects.table_twin.columns[3].values.iter().zip(&effects.table_twin.columns[2].values).all(|(high, low)| high >= low));
    let spectrum = set.iter().find(|d| d.id == "fatigue-spectrum:beam-B1").expect("spectrum");
    assert_eq!(spectrum.table_twin.rows(), 2);
    assert!(spectrum.table_twin.columns[0].values[0] < spectrum.table_twin.columns[0].values[1], "bins sorted by range");
    let total: f64 = spectrum.table_twin.columns[2].values.iter().sum();
    assert!(spectrum.markers.iter().any(|m| m.id == "total" && (m.utilisation.unwrap_or(0.0) - total).abs() < 1e-12));
}

#[test]
fn seismic_and_test_series_grips_write_their_leaves_and_show_the_live_fractile() {
    let mut document = analysed();
    document.seismics = vec![model::SeismicAction { id: "E-1".into(), a_ek: 50.0e3, importance_class: model::ImportanceClass::III }];
    let effect = model::MemberEffect { member_id: "beam-B1".into(), action_id: "E-1".into(), influence: 1.0 };
    document.effects.push(effect);
    let layout = Layout::of(&document, &[]);
    let seismic = handles::build(&document, &layout, "E-1");
    let stepper = seismic.iter().find(|h| h.id == "E-1:importance").expect("importance stepper");
    assert!(matches!(stepper.kind, scene::HandleKind::ChoiceStepper) && stepper.writes[0].path == "seismics[id=E-1].importanceClass");
    assert!(stepper.accessible_name.en.contains("A_Ed = 60.0 kN"), "{}", stepper.accessible_name.en);
    assert!(handles::action_ids(&document, "E-1").contains(&"E-1:importance".to_string()));
    let all = handles(&document, &[], &[scene::EntityRef::new(scene::EntityKind::Plate, "TS-1", "")]);
    assert_handle_laws(&all);
    let mean = all.iter().find(|h| h.id == "TS-1:mean").expect("mean grip");
    let sd = all.iter().find(|h| h.id == "TS-1:sd").expect("deviation grip");
    assert_eq!((mean.writes[0].path.as_str(), sd.writes[0].path.as_str()), ("testSeries[id=TS-1].mean", "testSeries[id=TS-1].stdDev"));
    assert!(mean.accessible_name.en.contains("k_n = ") && mean.accessible_name.de.contains("X_d = "));
    let scale = analyses::bars(&document, &layout).into_iter().find(|b| b.owner == "TS-1" && b.suffix.is_none()).expect("bar").per_metre;
    assert!((mean.writes_at(30.0 / scale)[0].value - 30.0).abs() < 1e-9 && (sd.writes_at(33.0 / scale)[0].value - 3.0).abs() < 1e-9);
    let scene = scene_of(&document);
    assert!(entity(&scene, "TS-1").handles.contains(&"TS-1:mean".to_string()) && entity(&scene, "E-1").handles.contains(&"E-1:importance".to_string()));
}

#[test]
fn effects_and_placements_are_insertable_kinds() {
    for (id, collection) in [("effect", "effects"), ("placement", "placements")] {
        assert!(ENTITY_KINDS.iter().any(|k| k.id == id && k.collection == collection), "{id} is in the palette registry");
    }
}

#[test]
fn the_serviceability_combination_diagram_ranks_characteristic_above_frequent_above_quasi_permanent() {
    let mut document = office();
    let member = &mut document.members[0];
    for limit in [&mut member.sls_limit_char, &mut member.sls_limit_freq, &mut member.sls_limit_qp] {
        *limit = 1.0e6;
    }
    let report = inferences::evaluate(&document);
    let id = format!("sls-combinations:{}", document.members[0].id);
    let diagram = diagrams(&document, &report, None, None).into_iter().find(|d| d.id == id).expect("sls combination diagram");
    assert_eq!(diagram.validate(), Ok(()));
    assert_diagram_laws(&[diagram.clone()]);
    let utilisation = values(&diagram, "Utilisation");
    assert_eq!(utilisation.len(), 3);
    assert!(utilisation[0] >= utilisation[1] && utilisation[1] >= utilisation[2], "{utilisation:?}");
    let effects = values(&diagram, "Effect E_d");
    for (effect, u) in effects.iter().zip(&utilisation) {
        assert!((effect / 1.0e6 - u).abs() < 1e-9);
    }
    assert!(diagram.markers.iter().all(|m| m.bound_check.as_deref().is_some_and(|c| c.starts_with("en1990.sls."))));
    document.members[0].sls_limit_freq = 0.0;
    let reduced = inferences::evaluate(&document);
    let two = diagrams(&document, &reduced, None, None).into_iter().find(|d| d.id == id).expect("two bars");
    assert_eq!(values(&two, "Utilisation").len(), 2);
}

fn snapshot_schema() -> serde_json::Value {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../🧬️schema/📸️snapshot/🔣️.json");
    serde_json::from_slice(&std::fs::read(path).expect("snapshot schema")).expect("json")
}

fn row_properties<'a>(schema: &'a serde_json::Value, collection: &str) -> &'a serde_json::Map<String, serde_json::Value> {
    let def = schema["properties"][collection]["items"]["$ref"].as_str().and_then(|r| r.rsplit('/').next()).unwrap_or_else(|| panic!("{collection} is no row array"));
    schema["$defs"][def]["properties"].as_object().unwrap_or_else(|| panic!("{def}"))
}

/// 🎚️ The numeric fields a geometric or bar grip writes, per collection; every other numeric schema field must be a gauge or an exemption.
const GRIPPED: &[(&str, &[&str])] = &[
    ("permanents", &["gk"]),
    ("variables", &["qk"]),
    ("accidentals", &["ad"]),
    ("seismics", &["aEk"]),
    ("members", &["span", "precamber", "deflectionW1", "deflectionW2", "deflectionW3", "rdStr", "rdGeo", "rdEquStab", "rdEquDestab", "rdFat", "slsLimitChar", "slsLimitFreq", "slsLimitQp"]),
    ("bridgeSls", &["bridgeDeflection", "bridgeDeflectionLimit"]),
    ("fatigueSpectra", &["detailCategory", "stressRange", "cycles"]),
    ("reliabilityAnalyses", &["resistanceMean", "resistanceSd", "actionMean", "actionSd", "secondActionMean", "secondActionSd"]),
    ("testSeries", &["mean", "stdDev", "required"]),
    ("placements", &["x", "y", "yaw"]),
    ("factorOverrides", &["value"]),
];

/// 🎚️ Every numeric field of every row collection of the snapshot schema has exactly one viewport grip (geometric, bar, move or gauge) or a documented exemption, and every grip names a numeric schema field; design cases carry their grips from the case forms.
#[test]
fn every_numeric_field_of_a_row_has_a_viewport_grip_or_a_documented_exemption() {
    let schema = snapshot_schema();
    let gauged = handles::gauges::coverage();
    let mut seen = 0;
    for (collection, spec) in schema["properties"].as_object().expect("properties") {
        if spec["items"]["$ref"].is_null() || collection == "cases" {
            continue;
        }
        let properties = row_properties(&schema, collection);
        let numeric: BTreeSet<&str> = properties.iter().filter(|(_, v)| matches!(v["type"].as_str(), Some("number" | "integer"))).map(|(k, _)| k.as_str()).collect();
        let grips: &[&str] = GRIPPED.iter().find(|(c, _)| c == collection).map_or(&[], |(_, f)| f);
        let gauges: Vec<(&str, f64)> = gauged.iter().find(|(c, _)| c == collection).map(|(_, f)| f.clone()).unwrap_or_default();
        for field in grips.iter().copied().chain(gauges.iter().map(|(f, _)| *f)) {
            assert!(numeric.contains(field), "{collection}.{field} is no numeric schema field");
        }
        for (field, step) in &gauges {
            if let Some(schema_step) = properties[*field]["x-semio-ui"]["step"].as_f64() {
                assert!((schema_step - step).abs() < 1e-12, "{collection}.{field}: gauge step {step} vs schema step {schema_step}");
            }
        }
        for field in &numeric {
            let exempt = handles::gauges::EXEMPT.contains(&(collection.as_str(), *field));
            let by_grip = grips.contains(field);
            let by_gauge = gauges.iter().any(|(f, _)| f == field);
            assert!(u8::from(by_grip) + u8::from(by_gauge) + u8::from(exempt) == 1, "{collection}.{field}: grip {by_grip}, gauge {by_gauge}, exempt {exempt}");
            seen += 1;
        }
    }
    assert!(seen >= 80, "{seen} numeric fields");
}

fn all_handles(document: &En1990Snapshot) -> Vec<scene::Handle> {
    let scene = scene_of(document);
    let selection: Vec<_> = scene.entities.iter().map(|e| e.entity.clone()).collect();
    handles(document, &[], &selection)
}

fn path_exists_in_schema(schema: &serde_json::Value, path: &str) -> bool {
    let Some((head, field)) = path.split_once('.') else { return false };
    let collection = head.split('[').next().unwrap_or(head);
    schema["properties"][collection]["items"]["$ref"].is_string() && row_properties(schema, collection).contains_key(field)
}

/// 🎚️ A gauge writes the current value of its field at zero drag, moves it by twenty schema steps per metre, writes only leaves of the schema, and the document re-evaluates with the dragged value.
#[test]
fn gauges_write_schema_leaves_move_by_steps_and_the_dragged_document_re_evaluates() {
    let schema = snapshot_schema();
    let mut total = 0;
    for (name, document) in fixtures() {
        let all = all_handles(&document);
        assert_handle_laws(&all);
        let mut ids = BTreeSet::new();
        for h in all.iter().filter(|h| h.id.contains(":gauge-")) {
            assert!(ids.insert(h.id.clone()), "{name}: duplicate {}", h.id);
            for write in &h.writes {
                assert!(path_exists_in_schema(&schema, &write.path), "{name}: gauge {} writes {}", h.id, write.path);
                let step = match &write.quantise { Some(scene::Quantise::Round { step }) => *step, other => panic!("{}: {other:?}", h.id) };
                assert!((h.writes_at(0.0)[0].value - write.map.base).abs() <= step / 2.0 + 1e-9, "{name}: {} does not write its current value at zero drag", h.id);
            }
        }
        total += ids.len();
    }
    assert!(total >= 40, "{total} gauges");
    let doc = office();
    let all = all_handles(&doc);
    let member = &doc.members[0];
    let limit = all.iter().find(|h| h.id == format!("{}:gauge-deflection-limit-ratio-w3", member.id)).expect("w3 limit gauge");
    assert_eq!(limit.writes[0].path, format!("members[id={}].deflectionLimitRatioW3", member.id));
    let dragged = limit.writes_at(1.0)[0].value;
    assert!((dragged - (member.deflection_limit_ratio_w3 + 1000.0)).abs() < 1e-6, "{dragged}");
    let influence = all.iter().find(|h| h.id == "G-sup:gauge-influence-beam-B1").expect("influence gauge");
    assert_eq!(influence.writes[0].path, "effects[0].influence");
    assert!((influence.writes_at(1.0)[0].value - 1.02).abs() < 1e-9);
    let utilisations = |d: &En1990Snapshot| inferences::evaluate(d).checks.iter().map(|c| (c.id.clone(), c.utilization.to_bits())).collect::<Vec<_>>();
    let mut moved = doc.clone();
    moved.members[0].deflection_limit_ratio_w3 = 600.0;
    assert_ne!(utilisations(&moved), utilisations(&doc), "the dragged limit must change a check");
    let mut stronger = doc.clone();
    stronger.effects[0].influence = 1.02;
    assert_ne!(utilisations(&stronger), utilisations(&doc), "the dragged influence must change a check");
}

/// 🚦️ A traffic load group component has a choice grip that steps through the groups and writes the typed token of its row; the component advertises it.
#[test]
fn a_traffic_group_component_has_a_choice_grip_writing_the_group_token() {
    let mut doc = office();
    doc.traffic_groups = vec![
        model::TrafficGroup { id: "g1".into(), group: model::LoadGroup::GR1A, variable_id: doc.variables[0].id.clone() },
        model::TrafficGroup { id: "g2".into(), group: model::LoadGroup::GR2, variable_id: doc.variables[0].id.clone() },
    ];
    let scene = scene_of(&doc);
    let all = all_handles(&doc);
    assert_handle_laws(&all);
    for row in &doc.traffic_groups {
        let grip = all.iter().find(|h| h.id == format!("{}:group", row.id)).unwrap_or_else(|| panic!("no group grip for {}", row.id));
        assert_eq!(grip.writes[0].path, format!("trafficGroups[id={}].group", row.id));
        assert!(matches!(grip.kind, scene::HandleKind::ChoiceStepper));
        assert!(entity(&scene, &format!("{}:beam-B1", row.id)).handles.contains(&grip.id), "{} is not advertised", grip.id);
    }
}

/// 🧭️ Every check of every fixture is drawn: it resolves to an entity row the scene projects or to the global badge through a document-level family, the focus of the outcome names the same row, and an owner-table family files its checks under the collection of the row it routes to.
#[test]
fn every_check_outcome_routes_to_its_entity_or_the_document_badge() {
    for (name, doc) in fixtures() {
        let report = inferences::evaluate(&doc);
        let scene = scene_of(&doc);
        assert!(!report.checks.is_empty(), "{name}");
        for check in &report.checks {
            let route = route_of(&doc, check).unwrap_or_else(|| panic!("{name}: check {} (subject {:?}) has no owner", check.id, check.subject.entity_id));
            match route {
                CheckRoute::Entity(id) => {
                    assert!(scene.entities.iter().any(|e| scene::projection_kit::base_id(&e.entity.id) == id), "{name}: check {} routes to {id}, which the scene does not draw", check.id);
                    assert_eq!(focus_of_outcome(&doc, &report, &check.id).as_deref(), Some(id.as_str()));
                    if let (Some(owner), Some(kind)) = (check_owner(&check.id), entity_kind_of(&doc, &id)) {
                        assert_eq!(kind.collection, owner, "{name}: check {} sits on {id}, but its family belongs to {owner}", check.id);
                    }
                }
                CheckRoute::Document => assert!(focus_of_outcome(&doc, &report, &check.id).is_none(), "{name}: {}", check.id),
            }
        }
    }
}

/// 🧭️ The families of the owner table never shadow each other (a longer prefix precedes every prefix it extends) and never repeat one of the document families.
#[test]
fn the_owner_table_lists_longer_prefixes_first_and_no_document_family() {
    for (i, (prefix, _)) in CHECK_OWNERS.iter().enumerate() {
        assert!(CHECK_OWNERS[..i].iter().all(|(earlier, _)| !prefix.starts_with(earlier) || earlier == prefix), "{prefix} is shadowed by an earlier prefix");
        assert!(!DOCUMENT_CHECKS.iter().any(|p| prefix.starts_with(p) || p.starts_with(prefix)), "{prefix} is also a document family");
        assert!(ENTITY_KINDS.iter().any(|k| k.collection == CHECK_OWNERS[i].1), "{prefix}: unknown collection {}", CHECK_OWNERS[i].1);
    }
}

/// 📶️ The scene binds an entity to the worst check the owner table routes to it: the member carries the utilisation of its worst owned check, and the analysis bars carry the checks of their row.
#[test]
fn entity_utilisation_is_the_worst_check_the_owner_table_routes_to_it() {
    let doc = analysed();
    let report = inferences::evaluate(&doc);
    let scene = scene_of(&doc);
    let dependents_of = |id: &str| -> Vec<String> { std::iter::once(id.to_string()).chain(doc.bridge_sls.iter().filter(|s| s.member_id == id).map(|s| s.id.clone())).chain(doc.fatigue_spectra.iter().filter(|s| s.member_id == id).take(1).map(|s| s.id.clone())).collect() };
    for member in &doc.members {
        let owned = dependents_of(&member.id);
        let worst = report.checks.iter().filter(|c| c.utilization.is_finite() && c.status != norm::CheckStatus::NotApplicable && owner_entity(&doc, c).is_some_and(|o| owned.contains(&o))).map(|c| c.utilization).fold(0.0, f64::max);
        let bound = entity(&scene, &member.id).utilisation.as_ref().map_or(0.0, |b| b.ratio);
        assert!(bound <= worst + 1e-9 && (worst == 0.0 || bound > 0.0), "{}: bound {bound} vs worst owned {worst}", member.id);
    }
}

#[test]
fn the_basis_badge_follows_the_document_annex_in_the_scene_and_every_diagram() {
    scene::projection_kit::test_support::assert_basis_follows_annex(norm::NormFamilyId::En1990, |annex| {
        let mut document = office();
        document.annex = annex;
        let report = inferences::evaluate(&document);
        (scene_with(&document, None), diagrams(&document, &report, None, None))
    });
}
