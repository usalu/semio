"""🧮 Moves the existing publication catalogue and task wire decoding into their owning components."""

from pathlib import Path

PLUGIN = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin")
APP = PLUGIN / "🧵️instance/🔌️plugin-app-impl"
CONTRACTS = APP / "🧾️typed-operation-results/🦀️.rs"
DISPATCH = APP / "📨️action-dispatch/🦀️.rs"
TOOLS = PLUGIN / "🧵️instance/👷️jobs/📥️staging/🧱️components/🦀️.rs"
HELPERS = PLUGIN / "⚛️reactor/🏃️runtime/📥️staging/🔌️reactor-app/🛠️runtime-helpers/🦀️.rs"


def stage(root, edit):
    text = (root / CONTRACTS).read_text(encoding="utf8")
    start = text.index("    pub(super) async fn tool_public_contracts")
    end = text.index("    pub(super) fn acknowledge_typed_operation_result", start)
    method = text[start:end]
    edit(CONTRACTS, method, """    pub(super) async fn tool_public_contracts(&self) -> Vec<ArtifactToolPublicContract> {
        self.tools.public_tool_contracts()
    }

""")
    move = method.replace("pub(super) async fn tool_public_contracts", "pub fn public_tool_contracts").replace("self.tools.", "self.")
    marker = "    /// 🔭️ Builds the component from the parts the composer constructs first;"
    edit(TOOLS, marker, "    /// 📚️ Lists this tool owner's registered public contracts in their declared order.\n" + move + marker)

    text = (root / DISPATCH).read_text(encoding="utf8")
    start = text.index("        async fn decode_lane<T: ::protocol::OpBinary>")
    end = text.index("        let log_generation_before", start)
    decoder = text[start:end]
    edit(DISPATCH, decoder, "        use semio_plugin_reactor_app::app_runtime_helpers::decode_task_emission_lane;\n")
    for lane in ("artifact", "config", "draft"):
        old = f"decode_lane::<A::{'' if lane == 'artifact' else lane.title()}Mutation>(&{lane}_ops).await?"
        edit(DISPATCH, old, old.replace("decode_lane", "decode_task_emission_lane").replace(".await", ""))
    moved = "".join(line[8:] if line.startswith("        ") else line for line in decoder.splitlines(keepends=True))
    moved = moved.replace("async fn decode_lane", "pub fn decode_task_emission_lane")
    moved = moved.replace("error.into_fault()", "semio_framework::FaultFrom::into_fault(error)")
    front = (root / HELPERS).read_text(encoding="utf8").splitlines()[0]
    edit(HELPERS, front, front + "\n\n/// 📬️ Decodes a resumed task's exact operation wire with the registered lane's native codec.\n" + moved)
