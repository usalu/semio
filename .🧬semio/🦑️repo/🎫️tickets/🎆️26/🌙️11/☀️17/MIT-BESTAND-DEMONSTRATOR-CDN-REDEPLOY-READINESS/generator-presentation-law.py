from pathlib import Path
import json
import subprocess
import sys

workspace = Path.cwd()
ticket = Path(__file__).resolve().parent
scratch = ticket / "🗑️generated" / "construction-native"
deps = scratch / "presentation-deps"
deps.mkdir(exist_ok=True)
products = {}
for line in (scratch / "cargo.jsonl").read_text().splitlines():
    event = json.loads(line)
    if event["reason"] == "compiler-artifact":
        for name in event["filenames"]:
            library = Path(name)
            link = deps / library.name
            if library.suffix in {".rmeta", ".rlib", ".dylib", ".so", ".dll"}:
                if not link.is_symlink():
                    link.symlink_to(library)
                products[event["target"]["name"], library.suffix] = link

externs = []
wire = "--wire" in sys.argv
required = ["semio_framework_job", "semio_framework_value", "semio_framework_tool_run",
            "semio_framework_tool_run_presentation", "semio_framework_tool_run_entities"]
if wire:
    required.extend(("semio_framework_job_payload", "semio_framework_tool_run_tick_wire"))
for crate in required:
    for suffix in (".rlib", ".rmeta"):
        library = products[crate, suffix]
        externs.extend(("--extern", f"{crate}={library}"))

source = (workspace / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🎞️tick/🛬️native/🦀️.rs").read_text()
from_owner = source[source.index("fn advance_presentation"):source.index("impl Drop for OriginalToolRunNativeState")]
funds = next(line for line in source.splitlines() if line.startswith("fn funds("))
imports = """
use semio_framework_job::{StepContext, OriginalTurnFailure};
use semio_framework_value::{ValueError, RetirementDemand, RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_tool_run::{ToolRunIdentity, ToolRunState, ToolRunTick};
use semio_framework_tool_run_presentation::slot::ToolRunPresentationSlot;
use semio_framework_tool_run_entities::ToolRunEntityEditGrant;
"""
out = scratch / "generator-presentation-current"
test = imports + funds + "\n" + from_owner
if wire:
    route = workspace / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🎞️tick/🎮️operations/🦀️.rs"
    retired = next(line for line in route.read_text().splitlines() if line.startswith("fn retired("))
    diagnostics = """
impl OriginalToolRunNativeState {
    pub fn fixture_cursor_close_demand(&self,body:usize)->Option<RetirementDemand>{
        self.cursor.as_ref().and_then(|cursor|cursor.close_demands(body).ok())
    }
    pub fn fixture_diagnostic(&self) -> (u8,bool,bool,bool,bool,Option<u64>,bool,bool) {
        (self.phase,self.cursor.is_some(),self.outcome.is_some(),self.stepped.is_some(),self.owned.tick.is_some(),
         self.owned.presentation.as_ref().and_then(|p|p.body()).map(|body|body.progress.sequence),
         self.owned.presentation.as_ref().is_some_and(ToolRunPresentationSlot::ready),
         self.cursor.as_ref().is_some_and(|cursor|cursor.frame_pending()))
    }
}
"""
    test = "mod native {\n" + source + diagnostics + "\n}\n" + retired + "\n" + (ticket / "generator-native-tick-law.rs").read_text()
    out = scratch / "generator-native-tick-current"
subprocess.run(["repo", "tools", "exec", "rustc", "--", "--edition=2021", "--crate-name", "generator_presentation_law",
                "--test", "-Zembed-metadata=no", "--cap-lints", "allow", "-L", f"dependency={deps}", *externs, "-o", str(out), "-"],
               input=test, text=True, check=True)
sys.exit(subprocess.run([str(out), "--test-threads=1"]).returncode)
