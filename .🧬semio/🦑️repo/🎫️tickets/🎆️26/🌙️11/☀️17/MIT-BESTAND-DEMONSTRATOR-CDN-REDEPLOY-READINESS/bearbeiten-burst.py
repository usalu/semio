"""📥️ Compare the live host producer with a bounded same-turn admission burst."""
import os
from pathlib import Path
import subprocess
import sys

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
host = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/📨️emission/🏛️host")
original = "    pub fn advance(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let mut admission=std::mem::take(&mut self.construction);let settled=self.advance_with(cx,&mut admission);self.construction=admission;settled}"
candidate = """    pub fn advance(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        let mut admission=std::mem::take(&mut self.construction);
        let settled=(||{
            for _ in 0..32 {
                let progress=cx.retained_progress();
                let work=cx.original_work_units_spent();
                if self.advance_with(cx,&mut admission)?{return Ok(true)}
                if cx.should_yield()||cx.retained_progress()==progress&&cx.original_work_units_spent()==work{break}
            }
            Ok(false)
        })();
        self.construction=admission;
        settled
    }"""
mode = sys.argv[1]
assert mode in ("red", "green")
source = (host / "🦀️.rs").read_text()
assert source.count(original) + source.count(candidate) == 1
source = source.replace(candidate, original) if mode == "red" else source.replace(original, candidate)
law = (ticket / "bearbeiten-parser-law.rs").read_text().replace(
    "use semio_plugin_host_configuration::OriginalHostConfiguration;",
    "use original_host_configuration::OriginalHostConfiguration;",
)
law = law.replace("    let mut mutation=None;", "    if text.len()>20000{assert!(steps<=7000,\"Process pack needed {steps} owner turns\");}\n    let mut mutation=None;")
enclosed = "mod original_host_configuration {\n" + source.split("\n", 1)[1] + "\n}\n"
contents = law + enclosed
name = "bearbeiten-burst-" + mode
driver = generated / (name + ".rs")
driver.write_text(contents)
env = {**os.environ, "SEMIO_BEARBEITEN_JSON_FIXTURE": str((host / "🧪️tests/🔣️.json").resolve())}
names = ["semio_framework_actor", "semio_framework_job", "semio_framework_value", "semio_plugin_decl_emission", "semio_plugin_job_receiving", "semio_framework_pack_json", "serde_json"]
subprocess.run(["repo", "tools", "exec", "--light", "python", "--", ticket / "generator-worker-run.py", str(driver), "--link", *names], env=env, check=True)
