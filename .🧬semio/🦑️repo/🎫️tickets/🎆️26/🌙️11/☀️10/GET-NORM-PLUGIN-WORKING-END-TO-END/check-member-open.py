"""🫴️ Runs the actual record-member state machine against the captured climate child and current WASI cohort."""
from pathlib import Path
import json
import sqlite3
import subprocess
import sys

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
output = ticket / "🗑️generated"
lane = root / ".cache/b/r/m/u/l" / sys.argv[1]
graph = root / ".cache/b/r/m/components/runs" / sys.argv[2] / "l/unit-graph-prototype-00.json"
units = json.loads(graph.read_text())["units"]
unit = next(row for row in units if row["target"]["name"] == "semio_os_store_member_open_store"
            and row["platform"] == "wasm32-wasip2" and row["mode"] == "build")
record = Path(unit["target"]["src_path"]).resolve().parent / "🫴️record/🦀️.rs"
record_source = record.read_text().replace("#[derive(Clone,Copy,PartialEq,Eq)]", "#[derive(Clone,Copy,Debug,PartialEq,Eq)]")
record_source = record_source.replace("fn invalid()->ValueError{", "#[track_caller] fn invalid()->ValueError{eprintln!(\"[DEBUG] member refusal {}\",std::panic::Location::caller());")
record_source = record_source.replace("fn flat<E:semio_framework_value_native_owned::native_decoding::NativeReceivingEffectError>(error:E)->ValueError{", "fn flat<E:std::fmt::Debug+semio_framework_value_native_owned::native_decoding::NativeReceivingEffectError>(error:E)->ValueError{eprintln!(\"[DEBUG] typed receiving refusal {error:?}\");")
record_source = record_source.replace("   Ok(Some((poll,(grant,units))))", """   {
    use std::sync::atomic::{AtomicUsize,Ordering};
    static RECEIVING_CALL:AtomicUsize=AtomicUsize::new(0);
    if RECEIVING_CALL.fetch_add(1,Ordering::Relaxed)<18 {
     eprintln!("[DEBUG] receiving call: offered {offered:?}, work {work}, passed {grant:?}, units {units}, poll {poll:?}");
    }
   }
   Ok(Some((poll,(grant,units))))""")
record_source += """
impl<S:NativeReceive<RecordValue>+BorrowedDslRecord+pack::snapshot::ArtifactDsl+Send> OriginalRecordMemberSnapshotOpen<S>{
 pub fn probe_phase(&self)->String{format!("{:?}",self.phase)}
 pub fn probe_frontier(&self,cx:&StepContext<'_>,body:usize)->String{
  let pending=self.receiving.as_ref().map(|out|out.advance_pending());
  let last=self.last_receipt();
  let quote=self.opening_demand(cx,body);
  let work=self.receiving.as_ref().map(|receiver|receiver.selected_advance_work_demand(body));
  let items=self.receiving.as_ref().map(|receiver|receiver.minimum_advance_items(body,cx.fuel_remaining()));
  format!("quote {quote:?}, work {work:?}, items {items:?}, request {:?}, pending {pending:?}, receipt {last:?}",self.receive_request)
 }
}
"""
(output / "record-member.rs").write_text(record_source)
source = output / "record-member-proof.rs"
source.write_text('#[path="record-member.rs"] mod member_record;\n' + (ticket / "check-member-open.rs").read_text())
binary = output / "record-member-proof.wasm"
args = ["--edition=2021", "--target=wasm32-wasip2", "--crate-name=record_member_proof",
        "-Cpanic=abort", "-Cstrip=symbols", "-Clink-arg=-zstack-size=16777216", "-o", str(binary), str(source)]
args.extend(f"-Ldependency={path}" for path in lane.glob("**/out") if "/build/" in str(path))

def extern(alias, package):
    base = lane / "wasm32-wasip2/prototype/build" / package
    artifact = max(base.glob("*/out/*.rlib"), key=lambda path: path.stat().st_mtime)
    for suffix in [".rlib", ".rmeta"]:
        args.extend(["--extern", f"{alias}={artifact.with_suffix(suffix)}"])

for dep in unit["dependencies"]:
    package = units[dep["index"]]["pkg_id"].rsplit("#", 1)[-1].split("@", 1)[0]
    extern(dep["extern_crate_name"], package)
for package in [
    "semio-os-store-member-open-store", "semio-framework-artifact-reference", "semio-framework-actor-retained-context",
    "semio-framework-actor-retained-turn", "semio-s-artifact-norm-din18599-model", "semio-s-artifact-norm-din18599-io",
    "semio-s-artifact-stdio-semio-table-model",
]:
    extern(package.replace("-", "_"), package)

response = output / "record-member-proof.args"
response.write_text("\n".join(args) + "\n")
compiler = subprocess.check_output(["rustup", "which", "rustc"], text=True).strip()
subprocess.run([compiler, "@" + str(response)], cwd=root, check=True)
child = subprocess.Popen(
    [root / "node_modules/.bin/jco", "run", "--jco-dir", output / "record-member-jco", binary, *sys.argv[3:]],
    cwd=root, stdout=subprocess.PIPE, text=True,
)
observed = {}
for line in child.stdout:
    print(line, end="", flush=True)
    if line.startswith("[DEBUG] climate "):
        key, payload = line.removeprefix("[DEBUG] climate ").split(" ", 1)
        observed[key] = json.loads(payload)
if child.wait():
    raise SystemExit(child.returncode)

fixture = next(root.glob("✏️s/🔌️plugins/📕️norm/*artifacts/*din18599/*tests/*fixtures/*climate-table-derivation/🔣️.json"))
law = json.loads(fixture.read_text())
with sqlite3.connect(":memory:") as connection:
    projection = connection.execute(
        """SELECT temperature.value, irradiation.value
           FROM json_each(?, '$.thetaEC') AS temperature
           JOIN json_each(?, '$.gHWM2') AS irradiation USING (key)
           ORDER BY temperature.key""",
        (json.dumps(law["climate"]),) * 2,
    ).fetchall()
if observed["columns"] != law["columns"] or list(zip(observed["temperatures"], observed["irradiation"])) != projection:
    raise SystemExit("Mounted climate table differs from the SQLite fixture")
print(f"[DEBUG] SQLite verified all {len(projection)} mounted climate rows")
