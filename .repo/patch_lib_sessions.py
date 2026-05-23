# Temporary patch script (ticket work): remove KitGraph.sessions, fix replace_from_full_dto preserve, migrate command_tests off kit_session.
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LIB = ROOT / "semio" / "rs" / "lib.rs"


def main() -> None:
    s = LIB.read_text(encoding="utf-8")
    orig = s

    s = s.replace(
        "        /// 🗂️ Ephemeral shell sessions; draft bodies live on `the_kit_draft` / alternative `draft`.\n"
        "        pub sessions: std::collections::HashMap<Id, crate::kit_session::Session>,\n",
        "",
    )
    s = s.replace("                sessions: std::collections::HashMap::new(),\n", "")
    s = s.replace("            g.sessions = std::collections::HashMap::new();\n", "")

    old_preserve = (
        "                let preserve = (g.undo_past.clone(), g.undo_future.clone(), g.undo_inhibit, g.initial.clone(), g.checkpoints.clone(), g.alternatives.clone(), g.the_kit_head.clone(), g.the_kit_draft.clone(), g.sessions.clone(), g.children.clone());\n"
        "                *g = merged;\n"
        "                g.undo_past = preserve.0;\n"
        "                g.undo_future = preserve.1;\n"
        "                g.undo_inhibit = preserve.2;\n"
        "                g.initial = preserve.3;\n"
        "                g.checkpoints = preserve.4;\n"
        "                g.alternatives = preserve.5;\n"
        "                g.the_kit_head = preserve.6;\n"
        "                g.the_kit_draft = preserve.7;\n"
        "                g.sessions = preserve.8;\n"
        "                g.children = preserve.9;\n"
    )
    new_preserve = (
        "                let preserve = (g.undo_past.clone(), g.undo_future.clone(), g.undo_inhibit, g.initial.clone(), g.checkpoints.clone(), g.alternatives.clone(), g.the_kit_head.clone(), g.the_kit_draft.clone(), g.children.clone());\n"
        "                *g = merged;\n"
        "                g.undo_past = preserve.0;\n"
        "                g.undo_future = preserve.1;\n"
        "                g.undo_inhibit = preserve.2;\n"
        "                g.initial = preserve.3;\n"
        "                g.checkpoints = preserve.4;\n"
        "                g.alternatives = preserve.5;\n"
        "                g.the_kit_head = preserve.6;\n"
        "                g.the_kit_draft = preserve.7;\n"
        "                g.children = preserve.8;\n"
    )
    s = s.replace(old_preserve, new_preserve)

    s = s.replace("        use crate::SessionCommand;\n", "")

    sid_block = (
        "            let sid = match KitGraph::execute(&k, KitStoreCommand::NewSession).expect(\"ns\") {\n"
        "                kit_store_command::KitStoreCommandResult::NewSession { id } => id,\n"
        "                _ => panic!(),\n"
        "            };\n"
    )
    s = s.replace(sid_block, "")

    kit_newdraft_wrap = (
        "            let did = match KitGraph::execute(&k, KitStoreCommand::ExecuteSessionCommands { id: sid.clone(), commands: vec![SessionCommand::NewDraft { checkpoint_id: None, alternative_id: None }] }).expect(\"nd\") {\n"
        "                kit_store_command::KitStoreCommandResult::ExecuteSessionCommands { results } => match &results[0] {\n"
        "                    crate::kit_session::SessionCommandResult::NewDraft { draft_id } => draft_id.clone(),\n"
        "                    _ => panic!(),\n"
        "                },\n"
        "                _ => panic!(),\n"
        "            };"
    )
    kit_newdraft_direct = (
        "            let did = match KitGraph::execute(&k, KitStoreCommand::NewDraft { checkpoint_id: None, alternative_id: None }).expect(\"nd\") {\n"
        "                kit_store_command::KitStoreCommandResult::NewDraft { draft_id } => draft_id.clone(),\n"
        "                _ => panic!(),\n"
        "            };"
    )
    s = s.replace(kit_newdraft_wrap, kit_newdraft_direct)

    did0_wrap = (
        "            let did0 = match KitGraph::execute(&k, KitStoreCommand::ExecuteSessionCommands { id: sid.clone(), commands: vec![SessionCommand::NewDraft { checkpoint_id: None, alternative_id: None }] }).expect(\"nd\") {\n"
        "                kit_store_command::KitStoreCommandResult::ExecuteSessionCommands { results } => match &results[0] {\n"
        "                    crate::kit_session::SessionCommandResult::NewDraft { draft_id } => draft_id.clone(),\n"
        "                    _ => panic!(),\n"
        "                },\n"
        "                _ => panic!(),\n"
        "            };"
    )
    did0_direct = (
        "            let did0 = match KitGraph::execute(&k, KitStoreCommand::NewDraft { checkpoint_id: None, alternative_id: None }).expect(\"nd\") {\n"
        "                kit_store_command::KitStoreCommandResult::NewDraft { draft_id } => draft_id.clone(),\n"
        "                _ => panic!(),\n"
        "            };"
    )
    s = s.replace(did0_wrap, did0_direct)

    did1_alt_wrap = (
        "            let did1 = match KitGraph::execute(&k, KitStoreCommand::ExecuteSessionCommands { id: sid.clone(), commands: vec![SessionCommand::NewDraft { checkpoint_id: Some(cp_main.clone()), alternative_id: Some(alt_id.clone()) }] }).expect(\"nd1\")\n"
        "            {\n"
        "                kit_store_command::KitStoreCommandResult::ExecuteSessionCommands { results } => match &results[0] {\n"
        "                    crate::kit_session::SessionCommandResult::NewDraft { draft_id } => draft_id.clone(),\n"
        "                    _ => panic!(),\n"
        "                },\n"
        "                _ => panic!(),\n"
        "            };"
    )
    did1_alt_direct = (
        "            let did1 = match KitGraph::execute(\n"
        "                &k,\n"
        "                KitStoreCommand::NewDraft {\n"
        "                    checkpoint_id: Some(cp_main.clone()),\n"
        "                    alternative_id: Some(alt_id.clone()),\n"
        "                },\n"
        "            )\n"
        "            .expect(\"nd1\")\n"
        "            {\n"
        "                kit_store_command::KitStoreCommandResult::NewDraft { draft_id } => draft_id.clone(),\n"
        "                _ => panic!(),\n"
        "            };"
    )
    s = s.replace(did1_alt_wrap, did1_alt_direct)

    did1_alt2_wrap = (
        "            let did1 = match KitGraph::execute(&k, KitStoreCommand::ExecuteSessionCommands { id: sid.clone(), commands: vec![SessionCommand::NewDraft { checkpoint_id: Some(cp0.clone()), alternative_id: Some(alt.clone()) }] }).expect(\"nd1\") {\n"
        "                kit_store_command::KitStoreCommandResult::ExecuteSessionCommands { results } => match &results[0] {\n"
        "                    crate::kit_session::SessionCommandResult::NewDraft { draft_id } => draft_id.clone(),\n"
        "                    _ => panic!(),\n"
        "                },\n"
        "                _ => panic!(),\n"
        "            };"
    )
    did1_alt2_direct = (
        "            let did1 = match KitGraph::execute(\n"
        "                &k,\n"
        "                KitStoreCommand::NewDraft {\n"
        "                    checkpoint_id: Some(cp0.clone()),\n"
        "                    alternative_id: Some(alt.clone()),\n"
        "                },\n"
        "            )\n"
        "            .expect(\"nd1\")\n"
        "            {\n"
        "                kit_store_command::KitStoreCommandResult::NewDraft { draft_id } => draft_id.clone(),\n"
        "                _ => panic!(),\n"
        "            };"
    )
    s = s.replace(did1_alt2_wrap, did1_alt2_direct)

    start_tx = (
        "            let txid = match KitGraph::execute(&k, KitStoreCommand::ExecuteSessionCommands { id: sid.clone(), commands: vec![SessionCommand::ExecuteKitDraftCommands { id: did.clone(), commands: vec![KitDraftCommand::StartTransaction] }] })\n"
        "                .expect(\"st\")\n"
        "            {\n"
        "                kit_store_command::KitStoreCommandResult::ExecuteSessionCommands { results } => match &results[0] {\n"
        "                    crate::kit_session::SessionCommandResult::ExecuteKitDraftCommands { results } => match &results[0] {\n"
        "                        crate::kit_draft::KitDraftCommandResult::StartTransaction { transaction_id } => transaction_id.clone(),\n"
        "                        _ => panic!(),\n"
        "                    },\n"
        "                    _ => panic!(),\n"
        "                },\n"
        "                _ => panic!(),\n"
        "            };"
    )
    start_tx_direct = (
        "            let txid = match KitGraph::execute(\n"
        "                &k,\n"
        "                KitStoreCommand::ExecuteKitDraftCommands {\n"
        "                    alternative_id: None,\n"
        "                    draft_id: did.clone(),\n"
        "                    commands: vec![KitDraftCommand::StartTransaction],\n"
        "                },\n"
        "            )\n"
        "            .expect(\"st\")\n"
        "            {\n"
        "                kit_store_command::KitStoreCommandResult::ExecuteKitDraftCommands { results } => match &results[0] {\n"
        "                    crate::kit_draft::KitDraftCommandResult::StartTransaction { transaction_id } => transaction_id.clone(),\n"
        "                    _ => panic!(),\n"
        "                },\n"
        "                _ => panic!(),\n"
        "            };"
    )
    s = s.replace(start_tx, start_tx_direct)

    start_tx0 = start_tx.replace("did.clone()", "did0.clone()").replace("txid", "tx0")
    start_tx0_direct = start_tx_direct.replace("did.clone()", "did0.clone()").replace("txid", "tx0")
    s = s.replace(start_tx0, start_tx0_direct)

    start_tx1 = start_tx.replace("did.clone()", "did1.clone()").replace("txid", "tx1")
    start_tx1_direct = start_tx_direct.replace("did.clone()", "did1.clone()").replace("txid", "tx1")
    s = s.replace(start_tx1, start_tx1_direct)

    start_tx_plain = start_tx.replace("txid", "tx")
    start_tx_plain_direct = start_tx_direct.replace("txid", "tx")
    s = s.replace(start_tx_plain, start_tx_plain_direct)

    start_tx_closure = (
        "                let txid = match KitGraph::execute(&k, KitStoreCommand::ExecuteSessionCommands { id: sid.clone(), commands: vec![SessionCommand::ExecuteKitDraftCommands { id: did.clone(), commands: vec![KitDraftCommand::StartTransaction] }] })\n"
        "                    .expect(\"st\")\n"
        "                {\n"
        "                    kit_store_command::KitStoreCommandResult::ExecuteSessionCommands { results } => match &results[0] {\n"
        "                        crate::kit_session::SessionCommandResult::ExecuteKitDraftCommands { results } => match &results[0] {\n"
        "                            crate::kit_draft::KitDraftCommandResult::StartTransaction { transaction_id } => transaction_id.clone(),\n"
        "                            _ => panic!(),\n"
        "                        },\n"
        "                        _ => panic!(),\n"
        "                    },\n"
        "                    _ => panic!(),\n"
        "                };"
    )
    start_tx_closure_direct = (
        "                let txid = match KitGraph::execute(\n"
        "                    &k,\n"
        "                    KitStoreCommand::ExecuteKitDraftCommands {\n"
        "                        alternative_id: None,\n"
        "                        draft_id: did.clone(),\n"
        "                        commands: vec![KitDraftCommand::StartTransaction],\n"
        "                    },\n"
        "                )\n"
        "                .expect(\"st\")\n"
        "                {\n"
        "                    kit_store_command::KitStoreCommandResult::ExecuteKitDraftCommands { results } => match &results[0] {\n"
        "                        crate::kit_draft::KitDraftCommandResult::StartTransaction { transaction_id } => transaction_id.clone(),\n"
        "                        _ => panic!(),\n"
        "                    },\n"
        "                    _ => panic!(),\n"
        "                };"
    )
    s = s.replace(start_tx_closure, start_tx_closure_direct)

    undo_line = (
        "            KitGraph::execute(&k, KitStoreCommand::ExecuteSessionCommands { id: sid.clone(), commands: vec![SessionCommand::ExecuteKitDraftCommands { id: did.clone(), commands: vec![KitDraftCommand::Undo { count: 1 }] }] }).expect(\"undo dr\");"
    )
    undo_line_direct = (
        "            KitGraph::execute(\n"
        "                &k,\n"
        "                KitStoreCommand::ExecuteKitDraftCommands {\n"
        "                    alternative_id: None,\n"
        "                    draft_id: did.clone(),\n"
        "                    commands: vec![KitDraftCommand::Undo { count: 1 }],\n"
        "                },\n"
        "            )\n"
        "            .expect(\"undo dr\");"
    )
    s = s.replace(undo_line, undo_line_direct)

    # Unwrap ExecuteSessionCommands + nested draft commands → single ExecuteKitDraftCommands
    big_exec = (
        "                KitStoreCommand::ExecuteSessionCommands {\n"
        "                    id: sid.clone(),\n"
        "                    commands: vec![SessionCommand::ExecuteKitDraftCommands {\n"
        "                        id: did.clone(),\n"
        "                        commands:"
    )
    big_exec_new = (
        "                KitStoreCommand::ExecuteKitDraftCommands {\n"
        "                    alternative_id: None,\n"
        "                    draft_id: did.clone(),\n"
        "                    commands:"
    )
    s = s.replace(big_exec, big_exec_new)

    big_exec_sid = (
        "                KitStoreCommand::ExecuteSessionCommands {\n"
        "                    id: sid,\n"
        "                    commands: vec![SessionCommand::ExecuteKitDraftCommands {\n"
        "                        id: did,\n"
        "                        commands:"
    )
    big_exec_sid_new = (
        "                KitStoreCommand::ExecuteKitDraftCommands {\n"
        "                    alternative_id: None,\n"
        "                    draft_id: did,\n"
        "                    commands:"
    )
    s = s.replace(big_exec_sid, big_exec_sid_new)

    # Remove closing `}],` from SessionCommand wrapper (one `],` → `,` after draft commands vec close)
    s = s.replace(
        "                        }],\n"
        "                    }],\n"
        "                },\n"
        "            )\n"
        "            .expect(\"tx\")",
        "                        }],\n"
        "                },\n"
        "            )\n"
        "            .expect(\"tx\")",
    )
    s = s.replace(
        "                        }],\n"
        "                    }],\n"
        "                },\n"
        "            )\n"
        "            .expect(\"seq\")",
        "                        }],\n"
        "                },\n"
        "            )\n"
        "            .expect(\"seq\")",
    )
    s = s.replace(
        "                        }],\n"
        "                    }],\n"
        "                },\n"
        "            )\n"
        "            .expect(\"f\")",
        "                        }],\n"
        "                },\n"
        "            )\n"
        "            .expect(\"f\")",
    )

    s = s.replace(
        "                kit_store_command::KitStoreCommandResult::ExecuteSessionCommands { results } => {\n"
        "                    let tr = match &results[0] {\n"
        "                        crate::kit_session::SessionCommandResult::ExecuteKitDraftCommands { results: dr } => match &dr[0] {\n",
        "                kit_store_command::KitStoreCommandResult::ExecuteKitDraftCommands { results } => {\n                    let tr = match &results[0] {\n",
    )
    s = s.replace(
        "                kit_store_command::KitStoreCommandResult::ExecuteSessionCommands { results } => match &results[0] {\n"
        "                    crate::kit_session::SessionCommandResult::ExecuteKitDraftCommands { results } => match &results[0] {\n",
        "                kit_store_command::KitStoreCommandResult::ExecuteKitDraftCommands { results } => match &results[0] {\n",
    )
    s = s.replace(
        "                kit_store_command::KitStoreCommandResult::ExecuteSessionCommands { results } => match &results[0] {\n"
        "                    crate::kit_session::SessionCommandResult::ExecuteKitDraftCommands { results } => match &results[1] {\n",
        "                kit_store_command::KitStoreCommandResult::ExecuteKitDraftCommands { results } => match &results[1] {\n",
    )

    s = s.replace(
        "                            crate::kit_draft::KitDraftCommandResult::ExecuteTransactionCommands { results: tr } => tr,\n"
        "                            _ => panic!(),\n"
        "                        },\n"
        "                        _ => panic!(),\n"
        "                    };",
        "                            crate::kit_draft::KitDraftCommandResult::ExecuteTransactionCommands { results: tr } => tr,\n                        _ => panic!(),\n                    };",
    )

    # transaction_can_redo_read style: ExecuteSessionCommands with id: did
    s = s.replace(
        "                    KitStoreCommand::ExecuteSessionCommands {\n"
        "                        id: sid.clone(),\n"
        "                        commands: vec![SessionCommand::ExecuteKitDraftCommands { id: did.clone(), commands: vec![KitDraftCommand::ExecuteTransactionCommands { id: tx.clone(), commands: vec![command] }] }],\n"
        "                    },",
        "                    KitStoreCommand::ExecuteKitDraftCommands {\n"
        "                        alternative_id: None,\n"
        "                        draft_id: did.clone(),\n"
        "                        commands: vec![KitDraftCommand::ExecuteTransactionCommands { id: tx.clone(), commands: vec![command] }],\n"
        "                    },",
    )
    s = s.replace(
        "                KitStoreCommand::ExecuteSessionCommands {\n"
        "                    id: sid.clone(),\n"
        "                    commands: vec![SessionCommand::ExecuteKitDraftCommands { id: did, commands: vec![KitDraftCommand::ExecuteTransactionCommands { id: tx, commands: vec![TransactionCommand::CanRedo] }] }],\n"
        "                },",
        "                KitStoreCommand::ExecuteKitDraftCommands {\n"
        "                    alternative_id: None,\n"
        "                    draft_id: did,\n"
        "                    commands: vec![KitDraftCommand::ExecuteTransactionCommands { id: tx, commands: vec![TransactionCommand::CanRedo] }],\n"
        "                },",
    )

    s = s.replace("SessionCommand::", "SessionCommand_REMOVED::")
    if "SessionCommand_REMOVED::" in s:
        raise SystemExit("leftover SessionCommand:: — fix script")

    if s != orig:
        LIB.write_text(s, encoding="utf-8", newline="\n")
        print("patched", LIB)
    else:
        print("no changes")


if __name__ == "__main__":
    main()
