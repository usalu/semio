from pathlib import Path

# 1) Strip duplicate Op codecs from jack op (spr owns OpText/OpBinary)
op = Path("✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🔧️op/🦀️component.rs")
op.write_text(
    """//! ⚡️ TrinityGraph artifact — grammar + store/mutation re-exports for `TrinityGraphMutation`.

pub use crate::artifacts::jack::mutations::{
    apply_trinity_graph_mutation, apply_trinity_graph_mutations, create_trinity_graph_envelope,
    dispatch_trinity_graph_mutations, inverse_trinity_graph_mutation, TrinityGraphEnvelope,
    TrinityGraphMutation, TrinityGraphStore,
};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar
"""
)
print("op cleaned")

# 2) Fix apply_trinity_graph_mutations mixed vars + dispatch mutations binding
mut = Path("✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🧬️mutations/🦀️component.rs")
t = mut.read_text()
old_apply = """pub fn apply_trinity_graph_mutations(fixture: JackSnapshot, operations: &[TrinityGraphMutation]) -> Result<JackSnapshot, crate::artifacts::jack::TrinityRamError> {
    let mut projection = fixture;
    for operation in operations {
        validate_trinity_graph_operation(operation, &snapshot)?;
        snapshot = apply_mutation(&snapshot, operation);
    }
    Ok(projection)
}"""
new_apply = """pub fn apply_trinity_graph_mutations(fixture: JackSnapshot, operations: &[TrinityGraphMutation]) -> Result<JackSnapshot, crate::artifacts::jack::TrinityRamError> {
    let mut snapshot = fixture;
    for operation in operations {
        validate_trinity_graph_operation(operation, &snapshot)?;
        snapshot = apply_mutation(&snapshot, operation);
    }
    Ok(snapshot)
}"""
if old_apply in t:
    t = t.replace(old_apply, new_apply)
    print("apply_trinity_graph_mutations fixed")
else:
    print("apply_trinity_graph_mutations pattern missing")

old_dispatch = """pub fn dispatch_trinity_graph_mutations(store: &mut TrinityGraphStore, operations: Vec<TrinityGraphMutation>) -> Result<(), crate::artifacts::jack::TrinityRamError> {
    if operations.is_empty() {
        return Ok(());
    }
    let mut snapshot = store.snapshot()?;
    for operation in &operations {
        validate_trinity_graph_operation(operation, &snapshot)?;
        snapshot = apply_mutation(&snapshot, operation);
    }
    store.dispatch(DocumentCommand::Apply { mutations, description: None }).map_err(crate::artifacts::jack::TrinityRamError::from).map(|_| ())
}"""
new_dispatch = """pub fn dispatch_trinity_graph_mutations(store: &mut TrinityGraphStore, operations: Vec<TrinityGraphMutation>) -> Result<(), crate::artifacts::jack::TrinityRamError> {
    if operations.is_empty() {
        return Ok(());
    }
    let mut snapshot = store.snapshot()?;
    for operation in &operations {
        validate_trinity_graph_operation(operation, &snapshot)?;
        snapshot = apply_mutation(&snapshot, operation);
    }
    store
        .dispatch(DocumentCommand::Apply { mutations: operations, description: None })
        .map_err(crate::artifacts::jack::TrinityRamError::from)
        .map(|_| ())
}"""
if old_dispatch in t:
    t = t.replace(old_dispatch, new_dispatch)
    print("dispatch_trinity_graph_mutations fixed")
else:
    print("dispatch pattern missing; showing around line 142")
    lines = t.splitlines()
    for i in range(141, min(155, len(lines))):
        print(f"{i+1}:{lines[i]}")

mut.write_text(t)

# 3) language-service: lex -> tokenize? check usages
ls = Path("✏️s/🔌️plugins/🔱️trinity/🗣️language-service/🦀️component.rs")
t = ls.read_text()
print("lex( count", t.count("lex("))
print("tokenize count", t.count("tokenize("))
for i, line in enumerate(t.splitlines(), 1):
    if "lex(" in line or "fn lex" in line:
        print(f"{i}:{line}")

# 4) query command str size
q = Path("✏️s/🔌️plugins/🔱️trinity/🎛️apps/🔌️jack/🎮️commands/🔎️query/🦀️component.rs")
print("\n### query command around 25")
lines = q.read_text().splitlines()
for i in range(max(0, 15), min(len(lines), 45)):
    print(f"{i+1}:{lines[i]}")

# 5) apps whole_document_operation bodies
for app in ["🔌️jack", "♻️rewrite"]:
    p = Path(f"✏️s/🔌️plugins/🔱️trinity/🎛️apps/{app}/🦀️component.rs")
    lines = p.read_text().splitlines()
    print(f"\n### {app} DocumentApp methods")
    for i, line in enumerate(lines, 1):
        if "initial_config" in line or "whole_document_operation" in line or "SetFixture" in line or "SetState" in line:
            if "fn " in line or "SetFixture" in line or "SetState" in line or "initial_snapshot" in line:
                print(f"{i}:{line}")
                if i < len(lines):
                    print(f"{i+1}:{lines[i]}")
