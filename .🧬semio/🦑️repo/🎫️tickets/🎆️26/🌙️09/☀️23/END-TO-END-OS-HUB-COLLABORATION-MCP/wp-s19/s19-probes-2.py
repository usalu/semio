#!/usr/bin/env python3
"""[DEBUG] S19 one-off: appends (--write) or removes (--revert) TEMPORARY test-only probes (rule 22) that reproduce this
session's routed defects natively on the LIVE tree: the hub/MCP seed path (`load_document_pack` of a genesis pair, then
agent preview → transaction prepare → commit) for generation3d/generation2d `addWidget` and flow `addWidget`; the
sourcing hub-member actor (86 bytes); the norm op round trip over every roster example of all 15 families. Every probe
prints `[DEBUG] probe2 …` lines and never asserts, so one run measures all of them. Backups (byte-exact, taken before
the first write) under `.🧬semio/🌐hub/s14-s19-backup/probes-2/`. usage: s19-probes-2.py --write | --revert"""
import os
import shutil
import sys

ROOT = "/Users/ueli/Documents/semio"
BACKUP = f"{ROOT}/.🧬semio/🌐hub/s14-s19-backup/probes-2"
MARK = "\n//#region 🧮️S19Probe2\n"
END = "//#endregion 🧮️S19Probe2\n"

HUB_PAIR = '''#[semio_framework_async_macros::async_test]
async fn s19_probe2_{label}_hub_pair() {{
    use semio_framework_plugin::PluginApp;
    {serial}
    let mut app = {constructor};
    let pair = semio_framework_plugin::artifact_app_genesis_pair::<semio_framework_plugin::EditorApp<{editor}>>("artifact-00000000000000000000000000000019").await.expect("[DEBUG] genesis pair");
    let loaded = app.load_document_pack(&pair).await;
    eprintln!("[DEBUG] probe2 {label} load={{loaded:?}}");
    let definition = {definition}();
    let preview = semio_framework_plugin::artifact_app_laws::agent_preview({appref}, &definition, "addWidget", &semio_framework::DslValue::Object(Vec::new())).await;
    eprintln!("[DEBUG] probe2 {label} preview={{preview:?}}");
    if let Some(wire) = app.take_last_emit_wire().await {{
        let ops = if wire.document.is_empty() {{ Vec::new() }} else {{ protocol::os_spr::causal::decode_ops_vec(&wire.document).expect("[DEBUG] ops") }};
        eprintln!("[DEBUG] probe2 {label} wire document_ops={{}} child_bytes={{}}", ops.len(), wire.children.len());
        let outcome = app.transaction_prepare("s19-probe2", "", &[], &ops, &wire.children, "addWidget", None).await;
        eprintln!("[DEBUG] probe2 {label} prepare={{:?}}", outcome.rejection);
        if outcome.rejection.is_none() {{
            let committed = app.transaction_commit("s19-probe2", &semio_framework_plugin::artifact_app_laws::meta("agent")).await;
            eprintln!("[DEBUG] probe2 {label} commit={{committed:?}}");
        }}
    }}
    {close}
}}
'''

GEN = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/{d}/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"
PROBES = {
    GEN.format(d="🧊️generation3d"): HUB_PAIR.format(label="generation3d", serial="let _serial = crate::publication_authority::lock();", constructor="app_with_registry().await", editor="Generation3dPlayApp", definition="super::create_generation3d_app", appref="&mut *app", close="semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);"),
    GEN.format(d="🌀️generation2d"): HUB_PAIR.format(label="generation2d", serial="let _serial = crate::publication_authority::lock();", constructor="context::app_with_registry().await", editor="super::Generation2dPlayApp", definition="super::create_generation2d_app", appref="&mut app", close="semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);"),
    "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs": HUB_PAIR.format(label="flow", serial="", constructor="flow_app_with_registry().await", editor="FlowPlayApp", definition="create_flow_app", appref="&mut *app", close=""),
    "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs": '''#[semio_framework_async_macros::async_test]
async fn s19_probe2_sourcing_hub_actor() {
    for actor in ["local", "user:01a0ecdf-4622-7eac-a35e-0c45cd3f81c7#session:01a0edf2-b4ec-76cf-8234-91cd0a010125"] {
        let mut app = new_app().await;
        let object_id = crate::stock_of(&app.snapshot().expect("[DEBUG] boot snapshot"))[0].id.clone();
        let meta = semio_framework_plugin::ActionMeta { actor: actor.into(), instance_id: 1, view_state: None };
        let dispatched = app.dispatch_typed(SourcingCurationCommand::CurationAdd(curation_add::CurationAdd { object_id: object_id.clone() }), &meta).await.map(drop);
        let mut faults = Vec::new();
        for _ in 0..100_000 {
            app.maintenance_step(1, 4_096).expect("[DEBUG] maintenance step");
            app.advance_typed_operation_publication().await.expect("[DEBUG] publication");
            if let Some(page) = app.take_typed_operation_result_page(1) {
                if page.lane == TypedOperationResultLane::Fault {
                    faults.push(String::from_utf8_lossy(page.bytes()).to_string());
                }
                app.acknowledge_typed_operation_result(page.token).expect("[DEBUG] ack");
            }
            app.take_typed_operation_effect();
            app.take_typed_operation_event();
            app.take_typed_operation_ui_scope();
            if !app.has_pending_typed_operations() {
                break;
            }
        }
        let landed = app.snapshot().expect("[DEBUG] snapshot").curated.iter().any(|item| item.object_id == object_id);
        eprintln!("[DEBUG] probe2 sourcing actor_bytes={} dispatch={dispatched:?} landed={landed} faults={faults:?}", actor.len());
    }
}
''',
    "✏️s/🔌️plugins/📕️norm/🧪️tests/🔬️surface/🦀️.rs": '''#[path = "/Users/ueli/Documents/semio/.tmp-ticket/wp-s19/s19-emitter-mod.rs"]
mod s19_emitter;

#[test]
#[ignore]
fn s19_emitter_serve() {
    s19_emitter::main();
}

fn s19_probe2_round_trip<M: semio_framework_os_kernel::OpBinary + PartialEq>(op: &M) -> Result<(), String> {
    let bytes = op.encode_op().map_err(|error| error.to_string())?;
    let decoded = M::decode_op(&bytes).map_err(|error| error.to_string())?;
    if &decoded == op { Ok(()) } else { Err("decodes to another op".into()) }
}

macro_rules! s19_probe2_norm {
    ($name:ident, $editor:ty, $set_active:path, $snapshot:ty) => {
        #[test]
        fn $name() {
            use semio_framework_plugin::ArtifactEditor;
            use $set_active as set_active_example;
            let boot = <$snapshot>::default();
            let config = semio_framework_plugin::NoConfig::default();
            for example in <$editor as ArtifactEditor>::examples() {
                let Ok(emit) = set_active_example::handle(&set_active_example::SetActiveExample { example_id: example.id().to_string() }, &semio_framework_plugin::ArtifactView::new(&boot, &semio_framework_plugin::HistoryView::empty()), &semio_framework_plugin::ConfigView { snapshot: &config, window: None }) else { continue };
                let failures: Vec<String> = emit.artifact_mutations.iter().filter_map(|op| s19_probe2_round_trip(op).err()).collect();
                eprintln!("[DEBUG] probe2 norm {} example={} ops={} failures={:?}", stringify!($name), example.id(), emit.artifact_mutations.len(), failures.first());
            }
        }
    };
}
''' + "".join(f"s19_probe2_norm!(s19_probe2_{f}, semio_s_artifact_norm_{f}::editor::{f}::{t}PlayApp, semio_s_artifact_norm_{f}::editor::{f}::commands::set_active_example, semio_s_artifact_norm_{f}::{t}Snapshot);\n" for f, t in [("din4108", "Din4108"), ("din16798", "Din16798"), ("din18599", "Din18599"), ("en1990", "En1990"), ("en1991", "En1991"), ("en1992", "En1992"), ("en1993", "En1993"), ("en1994", "En1994"), ("en1995", "En1995"), ("en1996", "En1996"), ("en1997", "En1997"), ("en1998", "En1998"), ("en1999", "En1999"), ("iso16757", "Iso16757"), ("vdi3805", "Vdi3805")]),
}

mode = sys.argv[1]
if mode == "--revert":
    import glob
    built = sorted((path for path in glob.glob(f"{ROOT}/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b/debug/build/semio-s-plugin-norm/*/out/semio_s_plugin_norm-*") if not path.endswith(".d")), key=os.path.getmtime)
    if built:
        os.makedirs(f"{ROOT}/.🧬semio/🌐hub/s14-s19-bin", exist_ok=True)
        shutil.copyfile(built[-1], f"{ROOT}/.🧬semio/🌐hub/s14-s19-bin/norm-surface-emitter")
        os.chmod(f"{ROOT}/.🧬semio/🌐hub/s14-s19-bin/norm-surface-emitter", 0o755)
        print(f"kept {built[-1]} as s14-s19-bin/norm-surface-emitter")
for rel, probe in PROBES.items():
    path = os.path.join(ROOT, rel)
    backup = os.path.join(BACKUP, rel)
    if mode == "--revert":
        if os.path.exists(backup):
            shutil.copyfile(backup, path)
            os.remove(backup)
            print(f"restored {rel}")
        continue
    text = open(path, encoding="utf-8").read()
    if MARK in text:
        print(f"already {rel}")
        continue
    os.makedirs(os.path.dirname(backup), exist_ok=True)
    if not os.path.exists(backup):
        shutil.copy2(path, backup)
    open(path, "w", encoding="utf-8").write(text.rstrip("\n") + "\n" + MARK + probe + END)
    print(f"probed {rel}")
