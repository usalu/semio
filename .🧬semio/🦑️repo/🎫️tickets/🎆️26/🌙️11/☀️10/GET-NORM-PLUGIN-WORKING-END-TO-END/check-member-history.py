"""🛂 Replay the actual Energy child history through its current original caller."""

import json
import os
from pathlib import Path
import subprocess
import sys

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
generated = ticket / "🗑️generated"
lane = root / ".cache/b/r/m/u/l" / sys.argv[1]
run = root / ".cache/b/r/m/components/runs" / sys.argv[2]
units = json.loads((run / "l/unit-graph-prototype-00.json").read_text())["units"]
unit = next(row for row in units if row["target"]["name"] == "semio_os_store_member_open_store"
            and row["platform"] == "wasm32-wasip2" and row["mode"] == "build")
owner = Path(unit["target"]["src_path"]).resolve().parent / "🛂️receiving/🦀️.rs"
source = owner.read_text()
if "trace" in sys.argv:
    start = " let(original,progress)=issued.unwrap_or_else"
    assert source.count(start) == 1
    source = source.replace(start, ''' if turn.work < u128::from(observed) {
 eprintln!("[DEBUG] Member child producer omitted work at {} reported={} observed={} issued={:?}",std::panic::Location::caller(),turn.work,observed,issued);
 }
''' + start)
    source = source.replace("fn receive<T,E>", "#[track_caller]\nfn receive<T,E>")
source += """
impl MemberHistoryReceiving {
    pub fn probe_state(&self) -> String {
        format!("member admitted={} closing={} decoded={} terminal={} fault={:?} inner={:?} decoder={:?}",
            self.admitted, self.closing, self.decoder.as_ref().is_some_and(|owner| owner.is_ready()),
            self.terminal_is_empty(), self.fault.as_ref(), self.inner_fault.as_ref(),
            self.decoder.as_ref().and_then(|owner| owner.fault()))
    }
}
"""
(generated / "member-history.rs").write_text(source)
input_source = generated / "member-history-proof.rs"
input_source.write_text('#[path="member-history.rs"] mod member_receiving;\n' + (ticket / "check-member-history.rs").read_text())
binary = generated / "member-history-proof.wasm"
flags = ["--edition=2021", "--target=wasm32-wasip2", "--crate-name=member_history_proof",
         "-Cpanic=abort", "-Cstrip=symbols", "-Clink-arg=-zstack-size=8388608", "-o", str(binary), str(input_source)]
flags.extend(f"-Ldependency={path}" for path in lane.glob("**/out") if "/build/" in str(path))


def include(alias, package):
    base = lane / "wasm32-wasip2/prototype/build" / package
    library = max(base.glob("*/out/*.rlib"), key=lambda path: path.stat().st_mtime)
    for suffix in [".rlib", ".rmeta"]:
        flags.extend(["--extern", f"{alias}={library.with_suffix(suffix)}"])


for edge in unit["dependencies"]:
    package = units[edge["index"]]["pkg_id"].rsplit("#", 1)[-1].split("@", 1)[0]
    include(edge["extern_crate_name"], package)
for package in [
    "semio-framework-actor-retained-context", "semio-framework-actor-retained-turn",
    "semio-framework-artifact-reference", "semio-framework-async", "semio-framework-job-native-decode", "semio-os-kernel",
    "semio-s-artifact-norm-din18599-io", "serde_json",
]:
    include(package.replace("-", "_"), package)

arguments = generated / "member-history-proof.args"
arguments.write_text("\n".join(flags) + "\n")
corpus = next(root.glob("🧰️framework/*products/*os/*modules/*spr/*history/*decode/*file/*tests/*fixtures/🔣️.json"))
env = dict(os.environ, MEMBER_HISTORY_READER_LAW=str(corpus))
compiler = subprocess.check_output(["rustup", "which", "rustc"], text=True).strip()
subprocess.run([compiler, "@" + str(arguments)], cwd=root, env=env, check=True)
subprocess.run([root / "node_modules/.bin/jco", "run", "--jco-dir", generated / "member-history-jco", binary], cwd=root, check=True)
