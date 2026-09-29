#!/usr/bin/env python3
"""[DEBUG] S19 one-off: appends (--write) or removes (--revert) TEMPORARY declared-verb census probes (test-only, rule 22) to the
unit test files of forms, sequence, generation3d, generation2d and the norm surface suite — every declared verb probed on the
shell lane and the agent lane (`artifact_app_laws::probe_declared_verbs`), each probe printed. Byte-exact backups under
`.🧬semio/🌐hub/s14-s19-backup/census-t7/`. usage: s19-census-probes.py --write | --revert"""
import os, shutil, sys

ROOT = "/Users/ueli/Documents/semio"
BACKUP = f"{ROOT}/.🧬semio/🌐hub/s14-s19-backup/census-t7"
MARK = "\n//#region 🧮️S19CensusProbe\n"
PRINT = '''            for probe in &probes {
                let findings: Vec<String> = semio_framework_plugin::artifact_app_laws::declared_verb_findings(probe).iter().map(ToString::to_string).collect();
                eprintln!("[DEBUG] census {} verb={} kind={:?} audience={:?} bridge={:?} shell={:?} agent={:?} findings={:?}", $label, probe.verb, probe.kind, probe.audience, probe.bridge, probe.windows.iter().map(|window| (&window.window, &window.staged)).collect::<Vec<_>>(), probe.agent, findings);
            }
'''
MACRO = '''macro_rules! s19_census {
    ($name:ident, $label:expr, $editor:ty, $definition:expr) => {
        #[semio_framework_async_macros::async_test]
        async fn $name() {
            let probes = semio_framework_plugin::artifact_app_laws::probe_declared_verbs::<semio_framework_plugin::EditorApp<$editor>, <$editor as semio_framework_plugin::ArtifactEditor>::Members>($definition, None).await;
''' + PRINT + '''        }
    };
}
'''
PROBES = {
    "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs": ["s19_census!(s19_census_forms, \"forms\", super::FormsPlayApp, super::create_forms_app);"],
    "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs": ["s19_census!(s19_census_sequence, \"sequence\", super::SequencePlayApp, super::create_sequence_app);"],
    "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs": ["s19_census!(s19_census_generation3d, \"generation3d\", super::Generation3dPlayApp, super::create_generation3d_app);"],
    "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs": ["s19_census!(s19_census_generation2d, \"generation2d\", super::Generation2dPlayApp, super::create_generation2d_app);"],
    "✏️s/🔌️plugins/📕️norm/🧪️tests/🔬️surface/🦀️.rs": [
        f"s19_census!(s19_census_{family}, \"{family}\", semio_s_artifact_norm_{family}::editor::{family}::{type_}PlayApp, semio_s_artifact_norm_{family}::editor::{family}::create_{family}_app);"
        for family, type_ in [("din4108", "Din4108"), ("din16798", "Din16798"), ("din18599", "Din18599"), ("en1990", "En1990"), ("en1991", "En1991"), ("en1992", "En1992"), ("en1993", "En1993"), ("en1994", "En1994"), ("en1995", "En1995"), ("en1996", "En1996"), ("en1997", "En1997"), ("en1998", "En1998"), ("en1999", "En1999"), ("iso16757", "Iso16757"), ("vdi3805", "Vdi3805")]
    ],
}

mode = sys.argv[1]
for rel, calls in PROBES.items():
    path = os.path.join(ROOT, rel)
    backup = os.path.join(BACKUP, rel)
    if mode == "--revert":
        if os.path.exists(backup):
            shutil.copy2(backup, path)
            print(f"restored {rel}")
        continue
    text = open(path, encoding="utf-8").read()
    if MARK in text:
        continue
    os.makedirs(os.path.dirname(backup), exist_ok=True)
    shutil.copy2(path, backup)
    open(path, "w", encoding="utf-8").write(text.rstrip("\n") + "\n" + MARK + MACRO + "\n".join(calls) + "\n//#endregion 🧮️S19CensusProbe\n")
    print(f"probed {rel}")

HUB_PAIR = '''#[semio_framework_async_macros::async_test]
async fn s19_census_sequence_hub_pair() {
    use semio_framework_plugin::PluginApp;
    for loaded in [false, true] {
        let mut app = context::new_app_with_registry_wired().await;
        if loaded {
            let pair = semio_framework_plugin::artifact_app_genesis_pair::<semio_framework_plugin::EditorApp<super::SequencePlayApp>>("artifact-00000000000000000000000000000019").await.expect("[DEBUG] genesis pair");
            app.load_document_pack(&pair).await.expect("[DEBUG] load pair");
        }
        let definition = super::create_sequence_app();
        let preview = semio_framework_plugin::artifact_app_laws::agent_preview(&mut app.0, &definition, "addStep", &semio_framework::DslValue::Object(Vec::new())).await;
        eprintln!("[DEBUG] census hub-pair loaded={loaded} preview={preview:?}");
        let Some(wire) = app.take_last_emit_wire().await else { continue };
        let ops = if wire.document.is_empty() { Vec::new() } else { protocol::os_spr::causal::decode_ops_vec(&wire.document).expect("[DEBUG] ops") };
        let outcome = app.transaction_prepare("s19-txn", "", &[], &ops, &wire.children, "addStep", None).await;
        eprintln!("[DEBUG] census hub-pair loaded={loaded} prepare={:?}", outcome.rejection);
        if outcome.rejection.is_none() {
            let committed = app.transaction_commit("s19-txn", &semio_framework_plugin::artifact_app_laws::meta("agent")).await;
            eprintln!("[DEBUG] census hub-pair loaded={loaded} commit={committed:?}");
            if committed.is_err() {
                let rolled = app.transaction_rollback("s19-txn").await;
                eprintln!("[DEBUG] census hub-pair loaded={loaded} rollback={rolled:?}");
            }
        }
    }
}
'''
SEQ = "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"
if mode == "--write":
    path = os.path.join(ROOT, SEQ)
    text = open(path, encoding="utf-8").read()
    end = "//#endregion 🧮️S19CensusProbe\n"
    if "s19_census_sequence_hub_pair" not in text and end in text:
        open(path, "w", encoding="utf-8").write(text.replace(end, HUB_PAIR + end))
        print(f"probed hub pair {SEQ}")
