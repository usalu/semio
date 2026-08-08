from pathlib import Path

op = Path("✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🔧️op/🦀️component.rs")
text = op.read_text()
old = "pub use crate::artifacts::jack::mutations::{apply_trinity_graph_mutation, inverse_trinity_graph_mutation, TrinityGraphMutation};"
new = """pub use crate::artifacts::jack::mutations::{
    apply_trinity_graph_mutation, apply_trinity_graph_mutations, create_trinity_graph_envelope,
    dispatch_trinity_graph_mutations, inverse_trinity_graph_mutation, TrinityGraphEnvelope,
    TrinityGraphMutation, TrinityGraphStore,
};"""
if old in text:
    op.write_text(text.replace(old, new))
    print("op reexports expanded")
else:
    print("op reexport line missing; current head:")
    print(text[:800])

spr = Path("✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📡️spr/🦀️component.rs")
t = spr.read_text()
print("\n### spr OpText hits")
for i, line in enumerate(t.splitlines(), 1):
    if "OpText" in line or "OpBinary" in line or "impl protocol::Op" in line:
        print(f"{i}:{line}")

mut = Path("✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🧬️mutations/🦀️component.rs")
t = mut.read_text()
print("\n### mutations Op/Dsl hits")
for i, line in enumerate(t.splitlines(), 1):
    if any(k in line for k in ("OpText", "OpBinary", "DslOps", "DslVariants", "derive", "enum TrinityGraphMutation")):
        print(f"{i}:{line}")

print("\n### legacy type hits")
root = Path("✏️s/🔌️plugins/🔱️trinity")
for pat in (
    "GraphFixture",
    "to_fixture",
    "from_fixture",
    "RewriteRuleModel",
    "RewriteRuleDocument",
    "TrinityGraphDocument",
    "conflicting implementations",
):
    hits = []
    for p in root.rglob("*.rs"):
        try:
            t = p.read_text()
        except Exception:
            continue
        if pat not in t:
            continue
        for i, line in enumerate(t.splitlines(), 1):
            if pat in line:
                hits.append(f"{p}:{i}:{line.strip()[:120]}")
    print(f"\n{pat} ({len(hits)})")
    for h in hits[:40]:
        print(h)
