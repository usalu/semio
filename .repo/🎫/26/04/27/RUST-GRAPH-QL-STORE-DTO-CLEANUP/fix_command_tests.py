# Temporary: bulk-fix command_tests after removing KitStoreCommand session variants.
import re
from pathlib import Path

path = Path(r"c:\git\compose\compose\rs\lib.rs")
s = path.read_text(encoding="utf-8")

# Remove NewSession block
s = re.sub(
    r"\n            let sid = match KitGraph::execute\(&k, KitStoreCommand::NewSession\)\.expect\(\"ns\"\) \{\s*"
    r"kit_store_command::KitStoreCommandResult::NewSession \{ id \} => id,\s*_ => panic!\(\),\s*\};\s*",
    "\n",
    s,
)

s = s.replace(
    "KitStoreCommand::ExecuteSessionCommands { id: sid.clone(), commands: vec![SessionCommand::NewDraft { checkpoint_id: None, alternative_id: None }] }",
    "KitStoreCommand::NewDraft { checkpoint_id: None, alternative_id: None }",
)
s = s.replace(
    "KitStoreCommand::ExecuteSessionCommands { id: sid.clone(), commands: vec![SessionCommand::NewDraft { checkpoint_id: Some(cp_main.clone()), alternative_id: Some(alt_id.clone()) }] }",
    "KitStoreCommand::NewDraft { checkpoint_id: Some(cp_main.clone()), alternative_id: Some(alt_id.clone()) }",
)
s = s.replace(
    "KitStoreCommand::ExecuteSessionCommands { id: sid.clone(), commands: vec![SessionCommand::NewDraft { checkpoint_id: Some(cp0.clone()), alternative_id: Some(alt.clone()) }] }",
    "KitStoreCommand::NewDraft { checkpoint_id: Some(cp0.clone()), alternative_id: Some(alt.clone()) }",
)

old_newdraft = """                kit_store_command::KitStoreCommandResult::ExecuteSessionCommands { results } => match &results[0] {
                    crate::kit_session::SessionCommandResult::NewDraft { draft_id } => draft_id.clone(),
                    _ => panic!(),
                },
                _ => panic!(),"""
new_newdraft = """                kit_store_command::KitStoreCommandResult::NewDraft { draft_id } => draft_id.clone(),
                _ => panic!(),"""
s = s.replace(old_newdraft, new_newdraft)

for did in ("did.clone()", "did0.clone()", "did1.clone()"):
    s = s.replace(
        f"KitStoreCommand::ExecuteSessionCommands {{ id: sid.clone(), commands: vec![SessionCommand::ExecuteKitDraftCommands {{ id: {did}, commands: vec![KitDraftCommand::StartTransaction] }}] }}",
        f"KitStoreCommand::ExecuteKitDraftCommands {{ alternative_id: None, draft_id: {did}, commands: vec![KitDraftCommand::StartTransaction] }}",
    )

s = s.replace(
    "KitStoreCommand::ExecuteSessionCommands { id: sid.clone(), commands: vec![SessionCommand::ExecuteKitDraftCommands { id: did1.clone(), commands: vec![KitDraftCommand::StartTransaction] }] }",
    "KitStoreCommand::ExecuteKitDraftCommands { alternative_id: Some(alt_id.clone()), draft_id: did1.clone(), commands: vec![KitDraftCommand::StartTransaction] }",
)

old_tx = """                kit_store_command::KitStoreCommandResult::ExecuteSessionCommands { results } => match &results[0] {
                    crate::kit_session::SessionCommandResult::ExecuteKitDraftCommands { results } => match &results[0] {
                        crate::kit_draft::KitDraftCommandResult::StartTransaction { transaction_id } => transaction_id.clone(),
                        _ => panic!(),
                    },
                    _ => panic!(),
                },
                _ => panic!(),"""
new_tx = """                kit_store_command::KitStoreCommandResult::ExecuteKitDraftCommands { results } => match &results[0] {
                        crate::kit_draft::KitDraftCommandResult::StartTransaction { transaction_id } => transaction_id.clone(),
                        _ => panic!(),
                },
                _ => panic!(),"""
s = s.replace(old_tx, new_tx)

replacements = [
    (
        """                KitStoreCommand::ExecuteSessionCommands {
                    id: sid.clone(),
                    commands: vec![SessionCommand::ExecuteKitDraftCommands {
                        id: did.clone(),""",
        """                KitStoreCommand::ExecuteKitDraftCommands {
                    alternative_id: None,
                    draft_id: did.clone(),""",
    ),
    (
        """                    KitStoreCommand::ExecuteSessionCommands {
                        id: sid.clone(),
                        commands: vec![SessionCommand::ExecuteKitDraftCommands {
                            id: did.clone(),""",
        """                    KitStoreCommand::ExecuteKitDraftCommands {
                        alternative_id: None,
                        draft_id: did.clone(),""",
    ),
    (
        """                KitStoreCommand::ExecuteSessionCommands {
                    id: sid.clone(),
                    commands: vec![SessionCommand::ExecuteKitDraftCommands {
                        id: did0.clone(),""",
        """                KitStoreCommand::ExecuteKitDraftCommands {
                    alternative_id: None,
                    draft_id: did0.clone(),""",
    ),
    (
        """                KitStoreCommand::ExecuteSessionCommands {
                    id: sid.clone(),
                    commands: vec![SessionCommand::ExecuteKitDraftCommands {
                        id: did1.clone(),""",
        """                KitStoreCommand::ExecuteKitDraftCommands {
                    alternative_id: Some(alt_id.clone()),
                    draft_id: did1.clone(),""",
    ),
    (
        """                KitStoreCommand::ExecuteSessionCommands {
                    id: sid,
                    commands: vec![SessionCommand::ExecuteKitDraftCommands {
                        id: did,""",
        """                KitStoreCommand::ExecuteKitDraftCommands {
                    alternative_id: None,
                    draft_id: did,""",
    ),
]
for a, b in replacements:
    s = s.replace(a, b)

# Remove SessionCommand wrapper closing: `}],` -> `},`
s = s.replace("                        ],\n                    }],\n                },", "                        ],\n                },")
s = s.replace("                    ],\n                    }],\n                },", "                    ],\n                },")

s = s.replace(
    'KitGraph::execute(&k, KitStoreCommand::ExecuteSessionCommands { id: sid.clone(), commands: vec![SessionCommand::ExecuteKitDraftCommands { id: did.clone(), commands: vec![KitDraftCommand::Undo { count: 1 }] }] }).expect("undo dr");',
    'KitGraph::execute(&k, KitStoreCommand::ExecuteKitDraftCommands { alternative_id: None, draft_id: did.clone(), commands: vec![KitDraftCommand::Undo { count: 1 }] }).expect("undo dr");',
)

s = s.replace(
    """                KitStoreCommand::ExecuteSessionCommands {
                    id: sid.clone(),
                    commands: vec![SessionCommand::ExecuteKitDraftCommands { id: did, commands: vec![KitDraftCommand::ExecuteTransactionCommands { id: tx, commands: vec![TransactionCommand::CanRedo] }] }],
                }""",
    """                KitStoreCommand::ExecuteKitDraftCommands {
                    alternative_id: None,
                    draft_id: did,
                    commands: vec![KitDraftCommand::ExecuteTransactionCommands { id: tx, commands: vec![TransactionCommand::CanRedo] }],
                }""",
)

s = s.replace(
    """                    KitStoreCommand::ExecuteSessionCommands {
                        id: sid.clone(),
                        commands: vec![SessionCommand::ExecuteKitDraftCommands { id: did.clone(), commands: vec![KitDraftCommand::ExecuteTransactionCommands { id: tx.clone(), commands: vec![command] }] }],
                    }""",
    """                    KitStoreCommand::ExecuteKitDraftCommands {
                        alternative_id: None,
                        draft_id: did.clone(),
                        commands: vec![KitDraftCommand::ExecuteTransactionCommands { id: tx.clone(), commands: vec![command] }],
                    }""",
)

s = s.replace(
    """                kit_store_command::KitStoreCommandResult::ExecuteSessionCommands { results } => {
                    match &results[0] {
                        crate::kit_session::SessionCommandResult::ExecuteKitDraftCommands { results: dr } => match &dr[0] {""",
    """                kit_store_command::KitStoreCommandResult::ExecuteKitDraftCommands { results } => match &results[0] {""",
)

s = s.replace(
    """                kit_store_command::KitStoreCommandResult::ExecuteSessionCommands { results } => match &results[0] {
                    crate::kit_session::SessionCommandResult::ExecuteKitDraftCommands { results } => match &results[1] {""",
    """                kit_store_command::KitStoreCommandResult::ExecuteKitDraftCommands { results } => match &results[1] {""",
)

path.write_text(s, encoding="utf-8", newline="\n")
print("done")
