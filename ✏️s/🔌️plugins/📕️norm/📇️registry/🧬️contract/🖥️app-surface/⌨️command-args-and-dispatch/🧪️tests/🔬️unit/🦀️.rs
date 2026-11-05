use super::*;
use semio_framework_value::DslValue;
use semio_s_plugin_norm_compliance::{AnnexChoice, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use crate::edit_commands::tests::model;

fn args(entries: Vec<(&str, DslValue)>) -> DslValue {
    DslValue::object(entries.into_iter().map(|(k, v)| (k.to_string(), v)))
}

fn write(path: &str, value: DslValue) -> DslValue {
    args(vec![("path", DslValue::String(path.into())), ("value", value)])
}

#[test]
fn set_fields_parses_and_validates_the_bundle() {
    let ok = args(vec![("writes", DslValue::Array(vec![write("members[id=b1].length", DslValue::float(4.25))]))]);
    assert_eq!(LiveCommand::parse("setFields", Some(&ok)), Ok(LiveCommand::SetFields { writes: vec![FieldWrite::number("members[id=b1].length", 4.25)] }));
    assert!(LiveCommand::parse("setFields", None).is_err());
    assert!(LiveCommand::parse("setFields", Some(&args(vec![("writes", DslValue::Array(vec![args(vec![("path", DslValue::String("a".into()))])]))]))).is_err());
    assert!(validate_writes(&[]).is_err());
    assert!(validate_writes(&[FieldWrite::number("a", 1.0), FieldWrite::number("a", 2.0)]).unwrap_err().contains("twice"));
    assert!(validate_writes(&[FieldWrite::number("a", f64::NAN)]).is_err());
    let many: Vec<_> = (0..=NORM_LIVE_MAX_WRITES).map(|i| FieldWrite::number(format!("p{i}"), 1.0)).collect();
    assert!(validate_writes(&many).is_err());
}

#[test]
fn a_write_bundle_is_one_emit_with_one_mutation_per_leaf_and_typed_like_the_leaf() {
    let writes = [FieldWrite::number("configuration.a", 4.25), FieldWrite::number("configuration.count", 6.0)];
    let emit = handle_set_fields(&model::document(), &writes, model::resolve).expect("emit");
    assert_eq!(emit.artifact_mutations.len(), 2);
    assert_eq!(model::replay(&model::document(), &emit.artifact_mutations).configuration, model::Configuration { a: 4.25, b: 2.0, count: 6 });
    assert_eq!(coerce_like(&DslValue::uint(1), &DslValue::float(5.6)), DslValue::uint(6));
    assert_eq!(coerce_like(&DslValue::int(1), &DslValue::float(-2.4)), DslValue::int(-2));
    assert_eq!(coerce_like(&DslValue::String("x".into()), &DslValue::float(1.0)), DslValue::float(1.0));
}

#[test]
fn a_failing_write_fails_the_whole_bundle() {
    let writes = [FieldWrite::number("configuration.a", 4.25), FieldWrite::number("configuration.zz", 1.0)];
    let fault = handle_set_fields(&model::document(), &writes, model::resolve).err().expect("fault");
    assert!(format!("{fault:?}").contains("zz"), "{fault:?}");
    let mut calls = 0;
    assert!(resolve_set_fields(&model::document(), &[FieldWrite::number("configuration.a", 1.0)], |d, e| { calls += 1; model::resolve(d, e) }).is_ok() && calls == 1);
}

#[test]
fn the_other_live_commands_parse_to_lane_events_only() {
    assert_eq!(LiveCommand::parse("clearPreview", None), Ok(LiveCommand::ClearPreview));
    assert_eq!(LiveCommand::parse("cancelEvaluation", None), Ok(LiveCommand::CancelEvaluation));
    let preview = args(vec![("checkId", DslValue::String("c1".into())), ("remedyIndex", DslValue::uint(2))]);
    assert_eq!(LiveCommand::parse("previewRemedy", Some(&preview)), Ok(LiveCommand::PreviewRemedy { check_id: "c1".into(), index: 2, option: 0 }));
    let units = args(vec![("units", DslValue::Array(vec![DslValue::String("u1".into())])), ("reason", DslValue::String("commit".into()))]);
    assert_eq!(LiveCommand::parse("evaluateUnits", Some(&units)), Ok(LiveCommand::EvaluateUnits { units: Some(vec!["u1".into()]), reason: EvalReason::Commit }));
    assert_eq!(LiveCommand::parse("evaluateUnits", None), Ok(LiveCommand::EvaluateUnits { units: None, reason: EvalReason::Manual }));
    assert!(LiveCommand::parse("evaluateUnits", Some(&args(vec![("reason", DslValue::String("?".into()))]))).is_err());
    assert!(LiveCommand::parse("evaluateUnits", Some(&args(vec![("units", DslValue::uint(1))]))).is_err());
    assert!(LiveCommand::parse("setField", None).is_err());
}

#[test]
fn every_live_tool_id_has_a_command_row() {
    let commands = [
        LiveCommand::SetFields { writes: vec![] },
        LiveCommand::PreviewRemedy { check_id: String::new(), index: 0, option: 0 },
        LiveCommand::ClearPreview,
        LiveCommand::CancelEvaluation,
        LiveCommand::EvaluateUnits { units: None, reason: EvalReason::Edit },
        LiveCommand::CopyEntities { collection: "members".into(), ids: vec![] },
        LiveCommand::DuplicateEntities { collection: "members".into(), ids: vec![] },
        LiveCommand::PasteEntities { clip_json: String::new(), at: None },
        LiveCommand::MoveEntity { path: "members".into(), id: "a".into(), target: MoveTarget::To(0) },
        LiveCommand::DeleteEntities { collection: "members".into(), ids: vec![] },
    ];
    let command_ids: Vec<&str> = commands.iter().map(LiveCommand::id).collect();
    let expected_ids: Vec<&str> = NORM_LIVE_TOOL_IDS.iter().copied().filter(|id| *id != "viewportAct").collect();
    assert_eq!(command_ids, expected_ids, "every live tool but the view action is a LiveCommand");
}

#[test]
fn the_entity_commands_parse_their_collection_ids_and_clip() {
    let ids = DslValue::Array(vec![DslValue::String("a".into()), DslValue::String("b".into())]);
    let selected = args(vec![("collection", DslValue::String("members".into())), ("ids", ids)]);
    assert_eq!(LiveCommand::parse("duplicateEntities", Some(&selected)), Ok(LiveCommand::DuplicateEntities { collection: "members".into(), ids: vec!["a".into(), "b".into()] }));
    assert_eq!(LiveCommand::parse("copyEntities", Some(&selected)), Ok(LiveCommand::CopyEntities { collection: "members".into(), ids: vec!["a".into(), "b".into()] }));
    assert_eq!(LiveCommand::parse("deleteEntities", Some(&selected)), Ok(LiveCommand::DeleteEntities { collection: "members".into(), ids: vec!["a".into(), "b".into()] }));
    assert!(LiveCommand::parse("deleteEntities", Some(&args(vec![("collection", DslValue::String("members".into())), ("ids", DslValue::Array(vec![]))]))).is_err(), "deleting nothing is refused");
    assert!(LiveCommand::parse("deleteEntities", None).is_err());
    assert!(LiveCommand::parse("copyEntities", Some(&args(vec![("collection", DslValue::String("members".into())), ("ids", DslValue::Array(vec![]))]))).is_err(), "an empty selection is refused");
    assert!(LiveCommand::parse("duplicateEntities", None).is_err());
    let clip = args(vec![("clip", args(vec![("collection", DslValue::String("members".into()))])), ("at", DslValue::uint(2))]);
    let Ok(LiveCommand::PasteEntities { clip_json, at }) = LiveCommand::parse("pasteEntities", Some(&clip)) else { panic!("a paste") };
    assert!(clip_json.contains("members") && at == Some(2));
    assert!(LiveCommand::parse("pasteEntities", None).is_err());
}

#[test]
fn the_move_command_parses_exactly_one_target_and_is_refused_otherwise() {
    let base = |extra: Vec<(&str, DslValue)>| args([vec![("path", DslValue::String("members".into())), ("id", DslValue::String("a".into()))], extra].concat());
    assert_eq!(LiveCommand::parse("moveEntity", Some(&base(vec![("to", DslValue::uint(2))]))), Ok(LiveCommand::MoveEntity { path: "members".into(), id: "a".into(), target: MoveTarget::To(2) }));
    assert_eq!(LiveCommand::parse("moveEntity", Some(&base(vec![("over", DslValue::String("c".into()))]))), Ok(LiveCommand::MoveEntity { path: "members".into(), id: "a".into(), target: MoveTarget::Over("c".into()) }));
    assert_eq!(LiveCommand::parse("moveEntity", Some(&base(vec![("step", DslValue::int(-1))]))), Ok(LiveCommand::MoveEntity { path: "members".into(), id: "a".into(), target: MoveTarget::Step(-1) }));
    assert!(LiveCommand::parse("moveEntity", Some(&base(vec![]))).is_err(), "a target is needed");
    assert!(LiveCommand::parse("moveEntity", Some(&base(vec![("to", DslValue::uint(1)), ("over", DslValue::String("c".into()))]))).is_err(), "two targets are refused");
    assert!(LiveCommand::parse("moveEntity", Some(&base(vec![("step", DslValue::int(2))]))).is_err(), "only single steps");
    assert!(LiveCommand::parse("moveEntity", None).is_err());
}

#[test]
fn a_remedy_preview_is_the_ghost_apply_remedy_would_write() {
    let subject = SubjectRef::new("m1", "resistanceKn", LocalizedCopy::new("Member", "Bauteil"));
    let mut report = CheckReport::default();
    report.push(
        CheckResult::assess("c.uls", "demo", ClauseId::new("demo", "§1", "1"), subject.clone(), LocalizedCopy::new("ULS", "GZT"))
            .utilization(Quantity::new(QuantityKind::Force, 150_000.0), Quantity::new(QuantityKind::Force, 80_000.0))
            .explanation(LocalizedCopy::new("raise", "erhöhen"))
            .annex(AnnexChoice::De)
            .remedy(Remedy::at_least(subject, Quantity::new(QuantityKind::Force, 80_000.0), Quantity::new(QuantityKind::Force, 150_000.0), LocalizedCopy::new("Raise R", "R erhöhen")))
            .build(),
    );
    let tree = args(vec![("resistanceKn", DslValue::float(80_000.0))]);
    let ghost = preview_remedy(&report, &tree, "c.uls", 0, 0).expect("ghost");
    assert_eq!(ghost.source, GhostSource::Remedy { check_id: "c.uls".into(), index: 0 });
    assert_eq!(ghost.writes.len(), 1);
    let mut applied = tree.clone();
    apply_remedy_edit(&report, "c.uls", 0, 0, &mut applied).expect("apply");
    assert_eq!(get_value_at_path(&applied, &ghost.writes[0].path).unwrap(), &ghost.writes[0].value);
    assert_eq!(tree.get("resistanceKn"), Some(&DslValue::float(80_000.0)));
    assert!(preview_remedy(&report, &tree, "nope", 0, 0).is_err());
    assert!(preview_remedy(&report, &tree, "c.uls", 9, 0).is_err());
}


#[test]
fn the_insert_ghost_command_parses_with_an_optional_spec_and_position() {
    let insert = args(vec![("insert", args(vec![("spec", DslValue::String("member".into())), ("at", DslValue::Array(vec![DslValue::uint(1), DslValue::uint(2), DslValue::uint(3)])), ("free", DslValue::Bool(true))]))]);
    let parsed = LiveCommand::parse("previewRemedy", Some(&insert)).expect("insert ghost");
    assert_eq!(parsed, LiveCommand::PreviewInsert { spec: Some("member".into()), at: Some([1.0, 2.0, 3.0]), free: true });
    assert_eq!(parsed.id(), "previewRemedy");
    let armed = args(vec![("insert", args(vec![("spec", DslValue::String("bar".into()))]))]);
    assert_eq!(LiveCommand::parse("previewRemedy", Some(&armed)), Ok(LiveCommand::PreviewInsert { spec: Some("bar".into()), at: None, free: false }));
    let moved = args(vec![("insert", args(vec![("at", DslValue::Array(vec![DslValue::uint(1), DslValue::uint(2)]))]))]);
    assert!(LiveCommand::parse("previewRemedy", Some(&moved)).is_err());
}
