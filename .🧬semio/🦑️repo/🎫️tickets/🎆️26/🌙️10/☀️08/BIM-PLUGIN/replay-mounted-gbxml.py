"""🌿️ Compile the original mounted gbXML work and its law using matching testkit receipts."""
from pathlib import Path
import json
import os
import re
import subprocess
import sys

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
output = generated / "gbxml-control"
record = json.loads((generated / "compiler-retry/n-tests-38989.invocation.json").read_text())
compiler = record["compiler"]
original = record["arguments"]
base = Path.cwd() / "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any"
work = base / "✏️editor/🎮️commands/📤️export-model/🌿️gbxml/🦀️.rs"
test = work.parent.parent / "🧪️tests/🔬️unit/🌿️gbxml.rs"
source = output / "gbxml-mounted-law.rs"
declaration = re.search(r"pub const MOUNTED_WORK_UNITS: u64 = [0-9]+;", (work.parent.parent / "🦀️.rs").read_text())[0]
text = f'''extern crate self as semio_s_artifact_bim_model_io;
pub use semio_s_artifact_bim_model_gbxml::standards;
mod mounted {{
    {declaration}
    use semio_s_artifact_bim_model_bim::editor::bim::{{BimCommand, BimModelApp}};
    use semio_s_artifact_bim_model_bim::editor::bim::commands::export_model::{{INFER_PREVIEW, ENCODE_PREVIEW}};
    use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
    use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::{{Analysis, MountedAnalysisAccess}};
    use semio_framework_job::{{InteractiveJobCloseStep, OriginalTurnFailure, StepContext}};
    use semio_framework_value::retirement::controlled::ControlledRetirement;
    use semio_framework_value::{{RetirementDemand, ValueError, ValueRefusalKind}};
    use semio_plugin::retained_command::{{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep}};
    use semio_plugin::EditorApp;
    #[path = {json.dumps(str(work), ensure_ascii=False)}]
    pub mod implementation;
}}
pub use mounted::implementation::GbxmlExportWork;
'''
law = "\n".join(test.read_text().splitlines()[1:]).replace(
    "use semio_s_artifact_bim_model_bim::editor::bim::commands::export_model::GbxmlExportWork;",
    "use crate::GbxmlExportWork;",
)
text += re.sub(r'(include_(?:str|bytes)!\()"([^"]+)"', lambda m: m[1] + json.dumps(str((test.parent / m[2]).resolve()), ensure_ascii=False), law)
source.write_text(text)
args = [compiler, "--edition=2021", "--test", "--crate-name", "mounted_gbxml_law", "-Zno-parallel-backend", "-Zthreads=1",
        "-L", f"dependency={output}", str(source), "-o", str(output / "mounted-gbxml-laws"),
        "-Zno-link", "-Copt-level=s", "-Ccodegen-units=64"]
externs = {"semio_framework", "semio_framework_job", "semio_framework_async_macros", "semio_plugin",
           "semio_s_artifact_bim_model_bim", "semio_s_artifact_bim_model_snapshot", "serde_json"}
for index, flag in enumerate(original[:-1]):
    if flag == "-L":
        args += [flag, original[index + 1]]
    if flag == "--extern" and original[index + 1].split("=", 1)[0] in externs:
        args += [flag, original[index + 1]]
for rebuilt in ("semio_s_artifact_bim_model_inferences", "semio_s_artifact_bim_model_gbxml"):
    args += ["--extern", f"{rebuilt}={output / f'lib{rebuilt}-gbxml_control.rlib'}"]
gbxml = output / "libsemio_s_artifact_bim_model_gbxml-gbxml_control.rmeta"
metadata = subprocess.check_output([compiler, "-Zls=root", str(gbxml)], text=True)
value = re.search(r"^\d+ semio_framework_value-(\w+) hash ", metadata, re.M)[1]
value_meta = next(path for root in [generated / "ntb", generated / "fixture-link"] for path in root.rglob(f"libsemio_framework_value-{value}.rmeta"))
args += ["--extern", f"semio_framework_value={value_meta}"]
env = dict(os.environ, **record["cargo"], RUST_MIN_STACK=str(32 << 20))
link = []
if sys.platform == "darwin":
    env["SDKROOT"] = subprocess.check_output(["xcrun", "--sdk", "macosx26.5", "--show-sdk-path"], text=True).strip()
    link += ["-Clink-arg=-Wl,-no_deduplicate,-x"]
phase = sys.argv[1] if len(sys.argv) > 1 else "all"
if phase in {"all", "build"}:
    print("[DEBUG] Compiling the mounted gbXML law", flush=True)
    status = subprocess.run(args, env=env).returncode
    if status:
        raise SystemExit(f"Mounted gbXML law codegen ended with {status}")
if phase in {"all", "link"}:
    print("[DEBUG] Linking the mounted gbXML law", flush=True)
    compile_record = output / "mounted-gbxml-laws.rlink"
    invoke = [compiler, "--crate-name", "mounted_gbxml_law", "--test", "-Zlink-only", str(compile_record), *link]
    if sys.platform == "darwin":
        import fcntl
        with (generated / "bim-large-link.lock").open("a") as lock:
            fcntl.flock(lock, fcntl.LOCK_EX)
            status = subprocess.run(invoke, env=env).returncode
    else:
        status = subprocess.run(invoke, env=env).returncode
    if status:
        raise SystemExit(f"Mounted gbXML law link ended with {status}")
if phase in {"all", "run"}:
    raise SystemExit(subprocess.run([output / "mounted-gbxml-laws", "--nocapture"], env=env).returncode)
