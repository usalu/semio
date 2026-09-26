"""🧯️ G10 guest patch (post `--packages all` landing window): the agent lane's preview refuses a verb whose change is a
whole-document load (`Effect::LoadDocument`), which the two-phase PureCommand → TransactionPrepare wire cannot carry.
Today such a verb previews zero ops and the gateway answers `no-change` (MCP side, landed 00:5x); with this patch the
agent learns WHY: a typed `interactive-job.preview-document-load` fault naming the action. The plugin framework is linked
into every guest, so this lands only with a full describe + restage (W2). usage: python3 g10-preview-effect-refusal.py [--apply]"""
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
OLD = """            if A::ROLE == AppRole::Viewer && !emit.artifact_mutations.is_empty() {
                return Err(viewer_read_only_fault(&address.action_id));
            }
            let mut artifact_op_bytes = Vec::with_capacity(emit.artifact_mutations.len());"""
NEW = """            if A::ROLE == AppRole::Viewer && !emit.artifact_mutations.is_empty() {
                return Err(viewer_read_only_fault(&address.action_id));
            }
            if emit.effects.iter().any(|effect| matches!(effect, Effect::LoadDocument { .. })) {
                return Err(Fault::new(
                    FaultOrigin::Framework,
                    FaultCode::new("interactive-job.preview-document-load"),
                    format!("action '{}' replaces the whole document through a host document load, which an agent's two-phase commit cannot carry; run it from the shell", address.action_id),
                ));
            }
            let mut artifact_op_bytes = Vec::with_capacity(emit.artifact_mutations.len());"""

text = open(PATH, encoding="utf-8").read()
count = text.count(OLD)
print(f"anchor matches: {count} (expected 1)")
if count != 1:
    sys.exit(1)
if "--apply" in sys.argv:
    open(PATH, "w", encoding="utf-8").write(text.replace(OLD, NEW))
    print("applied; next: cargo check -p semio-framework-plugin (+ --target wasm32-wasip2), map the fault code in 🌉️mcp/🔀️dispatch fault table, full restage")
else:
    print("dry run clean")
