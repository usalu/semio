#!/usr/bin/env python3
"""🪪️ S19 set `sourcing-actor` (guest, T6 round 4+): every Sourcing curation edit made by a hub member is refused.
Measured: the hub stamps each edit's actor as `user:<uuid>#session:<uuid>` (86 bytes, hub 7800 p33 CAS), while both
Sourcing one-item preparation factories (document + config lane) admitted at most a PRIVATE 64-byte actor/description
(`SOURCING_CURATION_{DOCUMENT,CONFIG}_METADATA_BYTES`) → `begin` hands the request back → store fault
"app-owned one-item preparation factory rejected its exact owner bundle" (S18 p33 hub sweep, A15 audit). Locally the
actor is `local`, so every local law stayed green. Every other plugin bounds actor + description by the store's own
identity capacity `store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES` (256); Sourcing now does too and its two private figures
are gone. Law `a_hub_member_edit_prepares_under_the_stores_identity_capacity` (measured hub actor + a full-capacity
actor; document lane `curationAdd` lands, config lane `setFilterQuery` publishes no fault page). (2) G12 p33-5:
curation verbs naming an object outside the stock settled `no-change` (a silent success); `CurationDecision::UnknownObject`
now refuses them by name (`app.command.invalid-args`), as does `curationSetCount` without delta/value. Law
`a_curation_verb_outside_the_stock_is_refused_by_name`. usage: s19-sourcing-actor.py [--dry-run|--write|--revert]"""
import importlib.util
import os
import sys

spec = importlib.util.spec_from_file_location("s19_setlib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "s19_setlib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)

EDITOR = "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
TESTS = "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"
BOUND = "store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES"


def editor(text):
    if "SOURCING_CURATION_CONFIG_METADATA_BYTES" not in text and "SOURCING_CURATION_DOCUMENT_METADATA_BYTES" not in text:
        return text
    for constant in ("SOURCING_CURATION_CONFIG_METADATA_BYTES", "SOURCING_CURATION_DOCUMENT_METADATA_BYTES"):
        declaration = f"const {constant}: usize = 64;\n"
        assert text.count(declaration) == 1, f"declaration of {constant} x{text.count(declaration)}"
        text = text.replace(declaration, "")
        assert text.count(constant) == 2, f"{constant} uses x{text.count(constant)} (want 2: preflight + begin)"
        text = text.replace(constant, BOUND)
    return text


LAW = '''
//#region 🪪️HubMemberActor
/// 🪪️ LAW: a hub member's edit prepares. The one-item lanes bound the edit's actor and description by the store's
/// own identity capacity (`store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES`), never a private figure — measured on hub 7800:
/// every actor is `user:<uuid>#session:<uuid>` (86 bytes) and each curation edit was refused "app-owned one-item
/// preparation factory rejected its exact owner bundle" while the local `local` actor passed every law.
#[semio_framework_async_macros::async_test]
async fn a_hub_member_edit_prepares_under_the_stores_identity_capacity() {
    let full = "a".repeat(store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES);
    for actor in ["user:01a0ecdf-4622-7eac-a35e-0c45cd3f81c7#session:01a0edf2-b4ec-76cf-8234-91cd0a010125", full.as_str()] {
        let mut app = new_app().await;
        let object_id = crate::stock_of(&app.snapshot().expect("boot snapshot"))[0].id.clone();
        let meta = semio_framework_plugin::ActionMeta { actor: actor.into(), instance_id: 1, view_state: None };
        for command in [SourcingCurationCommand::CurationAdd(curation_add::CurationAdd { object_id: object_id.clone() }), SourcingCurationCommand::SetFilterQuery(set_filter_query::SetFilterQuery { value: "timber".into() })] {
            app.dispatch_typed(command, &meta).await.expect("edit dispatch");
            for _ in 0..100_000 {
                app.maintenance_step(1, 4_096).expect("maintenance step");
                app.advance_typed_operation_publication().await.expect("typed operation publication");
                if let Some(page) = app.take_typed_operation_result_page(1) {
                    assert_ne!(page.lane, TypedOperationResultLane::Fault, "a {}-byte actor's edit must prepare: {}", actor.len(), String::from_utf8_lossy(page.bytes()));
                    assert!(app.acknowledge_typed_operation_result(page.token).expect("typed operation acknowledgement"));
                }
                app.take_typed_operation_effect();
                app.take_typed_operation_event();
                app.take_typed_operation_ui_scope();
                if !app.has_pending_typed_operations() {
                    break;
                }
                std::thread::yield_now();
            }
        }
        assert!(app.snapshot().expect("snapshot").curated.iter().any(|item| item.object_id == object_id), "a {}-byte actor's document edit must land", actor.len());
    }
}
//#endregion 🪪️HubMemberActor
'''


def tests(text):
    if "a_hub_member_edit_prepares_under_the_stores_identity_capacity" in text:
        return text
    anchor = "//#region 🧪️RetainedConfigOracle\n"
    assert text.count(anchor) == 1, f"anchor x{text.count(anchor)}"
    return text.replace(anchor, LAW.lstrip("\n") + "\n" + anchor)


# ── (2) G12 p33-5: curation verbs with an object outside the stock settled `no-change` (silent success) ──────────────
SCHEMA = "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"
COMMANDS = "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands"
HANDLERS = ["➕️curation-add", "➖️curation-remove", "🏊️drop-on-pool", "🔢️curation-set-count", "🧺️drop-on-curated"]
UNKNOWN = "CurationDecision::UnknownObject { object_id: object_id.to_string() }"


def schema(text):
    if "UnknownObject" in text:
        return text
    text = lib.replace_once("pub enum CurationDecision {\n    NoOp,\n", "pub enum CurationDecision {\n    NoOp,\n    UnknownObject { object_id: String },\n")(text)
    guard = "let Some(extra) = document.stock_extra.iter().find(|extra| extra.id == object_id) else { return CurationDecision::NoOp };"
    assert text.count(guard) == 2, f"stock guards x{text.count(guard)}"
    text = text.replace(guard, f"let Some(extra) = document.stock_extra.iter().find(|extra| extra.id == object_id) else {{ return {UNKNOWN} }};")
    text = lib.replace_once("        CurationDecision::NoOp => {}\n        CurationDecision::Create(item) => document.curated.push(item),", "        CurationDecision::NoOp | CurationDecision::UnknownObject { .. } => {}\n        CurationDecision::Create(item) => document.curated.push(item),")(text)
    for old in ("/// to `0..=availability`. Unknown `object_id` (absent from stock) resolves to `NoOp`.", "/// `object_id` (absent from stock) resolves to `NoOp`."):
        assert text.count(old) == 1, old
        text = text.replace(old, old.replace("resolves to `NoOp`.", "resolves to `UnknownObject`."))
    return text


HELPER_OLD = """/// 🔀️ Turns a resolved curation decision into the real `SourcingMutation` it corresponds to — `None`
/// for a no-op adjustment (e.g. dropping an already-zero item), which must NOT be recorded in
/// history at all (mirrors `apply_sourcing_mutation`'s former no-op-if-unknown-id silence, now
/// expressed as "emit nothing" instead of "emit a snapshot no-op").
fn mutation_for(decision: CurationDecision) -> Option<SourcingMutation> {
    match decision {
        CurationDecision::NoOp => None,
        CurationDecision::Create(item) => Some(crate::mutations::create_curated_item(item)),
        CurationDecision::ChangeCount { object_id, new_count } => Some(crate::mutations::change_curated_item_count(object_id, new_count)),
        CurationDecision::Delete { object_id } => Some(crate::mutations::delete_curated_item(object_id)),
    }
}

fn emit_decision(decision: CurationDecision) -> Emit<SourcingMutation, SourcingCurationConfigMutation> {
    match mutation_for(decision) {
        Some(mutation) => Emit::mutations(vec![mutation]),
        None => Emit::default(),
    }
}
"""
HELPER_NEW = """/// 🔀️ Turns a resolved curation decision into the real `SourcingMutation` it corresponds to — `None`
/// for a no-op adjustment (e.g. dropping an already-zero item), which must NOT be recorded in
/// history at all; an object outside the stock is refused by name, never settled as a silent success.
fn mutation_for(decision: CurationDecision) -> Result<Option<SourcingMutation>, Fault> {
    Ok(match decision {
        CurationDecision::NoOp => None,
        CurationDecision::UnknownObject { object_id } => return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("object '{object_id}' is not in this curation's stock"))),
        CurationDecision::Create(item) => Some(crate::mutations::create_curated_item(item)),
        CurationDecision::ChangeCount { object_id, new_count } => Some(crate::mutations::change_curated_item_count(object_id, new_count)),
        CurationDecision::Delete { object_id } => Some(crate::mutations::delete_curated_item(object_id)),
    })
}

fn emit_decision(decision: CurationDecision) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation>, Fault> {
    Ok(match mutation_for(decision)? {
        Some(mutation) => Emit::mutations(vec![mutation]),
        None => Emit::default(),
    })
}
"""


def handler(text):
    text = lib.replace_once(HELPER_OLD, HELPER_NEW)(text)
    text = lib.replace_once("use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};\n", "use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};\n")(text)
    lines = text.splitlines(keepends=True)
    for index, line in enumerate(lines):
        if line.startswith("    Ok(emit_decision(") and line.rstrip("\n").endswith("))"):
            lines[index] = "    " + line.strip()[3:-1] + "\n"
    text = "".join(lines)
    return lib.replace_once("    } else {\n        CurationDecision::NoOp\n    };", "    } else {\n        return Err(Fault::new(FaultOrigin::App, FaultCode::new(\"app.command.invalid-args\"), \"curationSetCount needs its 'delta' or 'value' argument\"));\n    };")(text) if "pub struct CurationSetCount" in text else text


UNKNOWN_LAW = """
//#region 🚫️UnknownObject
/// 🚫️ LAW: a curation verb that names an object outside the stock is refused by name (`app.command.invalid-args`) —
/// it settled as a silent success (`no-change`) for every agent that named an object this curation does not hold
/// (G12 p33-5), and `curationSetCount` without a delta or value is refused the same way.
#[semio_framework_async_macros::async_test]
async fn a_curation_verb_outside_the_stock_is_refused_by_name() {
    let app = new_app().await;
    let snapshot = app.snapshot().expect("snapshot");
    let config = SourcingCurationConfig::default();
    let doc = semio_framework_plugin::ArtifactView::new(&snapshot, &semio_framework_plugin::HistoryView::empty());
    let cfg = semio_framework_plugin::ConfigView { snapshot: &config, window: None };
    let refused = handle(&CurationAdd { object_id: "no-such-object".into() }, &doc, &cfg).err().expect("an object outside the stock is refused");
    assert_eq!(refused.code.0, "app.command.invalid-args");
    let refused = curation_set_count::handle(&curation_set_count::CurationSetCount { object_id: snapshot.stock_extra[2].id.clone(), delta: None, value: None }, &doc, &cfg).err().expect("a count without delta or value is refused");
    assert_eq!(refused.code.0, "app.command.invalid-args");
    assert!(handle(&CurationAdd { object_id: snapshot.stock_extra[2].id.clone() }, &doc, &cfg).expect("an object in the stock curates").artifact_mutations.len() == 1);
}
//#endregion 🚫️UnknownObject
"""
ADD_TESTS = f"{COMMANDS}/➕️curation-add/🧪️tests/🔬️unit/🦀️.rs"


def add_tests(text):
    if "fn a_curation_verb_outside_the_stock_is_refused_by_name" in text:
        return text
    return text.rstrip("\n") + "\n" + UNKNOWN_LAW


lib.run("sourcing-actor", [(EDITOR, editor), (TESTS, tests), (SCHEMA, schema)] + [(f"{COMMANDS}/{h}/🦀️.rs", handler) for h in HANDLERS] + [(ADD_TESTS, add_tests)], sys.argv[1:])
