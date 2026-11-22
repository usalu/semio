"""🧪️ Replays the native ToolRun close law from exact production helpers and warmed WASI libraries."""
import json
import os
import sqlite3
import subprocess
import sys
from pathlib import Path


def main():
    """🔬️ Compile the old and corrected methods against the same real root owner and fixture."""
    ticket = Path(__file__).resolve().parent
    output = ticket / "🗑️generated"
    tool = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run")
    member = tool / "👥️member-owners-and-visitors"
    test = member / "🧪️tests"
    fixture = (test / "🧫️fixtures/🔣️.json").resolve()
    production = (member / "🦀️.rs").read_text()
    helpers = production.split("pub fn tool_grant_funds", 1)[1].split("#[cfg(test)]", 1)[0]
    demands = (tool / "🎞️tick/🎮️operations/🦀️.rs").read_text().split("pub fn tool_erased_retirement_demand", 1)[1].split("\nfn invalid", 1)[0]
    law = (test / "🦀️.rs").read_text().split("\n", 1)[1].replace("#[test]\nfn ", "fn ")
    law = law.replace('include_str!("🧫️fixtures/🔣️.json")', f'include_str!(r#"{fixture}"#)')
    source = f"""use semio_framework_value::{{RetainedCloneStep, ValueError, ValueRefusalKind}};
mod store {{ pub use semio_framework_value::ErasedSnapshotRetirement; }}
pub fn tool_grant_funds{helpers}
pub fn tool_erased_retirement_demand{demands}
{law}
fn main() {{ tool_run_closes_owned_and_aliased_store_roots_through_the_resident_reply(); }}
""".replace("pub(super)", "pub")
    corrected = "semio_framework_value::close_owner(current.as_mut(),grant.maximum_copy_bytes,effect)"
    assert source.count(corrected) == 1
    env = {**os.environ, "SEMIO_WORKER_RECEIPT_RUN": sys.argv[1]}
    for variant, body in [("before", source.replace(corrected, "current.close_step(grant.maximum_copy_bytes,effect)")), ("fixed", source)]:
        target = output / f"tool-root-retirement-{variant}.rs"
        target.write_text(body)
        subprocess.run([sys.executable, str(ticket / "generator-worker-run.py"), str(target), "semio_framework_value", "semio_framework_job", "serde_json", "--link"], env=env, check=True)
    with sqlite3.connect(":memory:") as db:
        for case in json.loads(fixture.read_text())["cases"]:
            row = db.execute("select ? as denied, not 0 as terminal_empty, ? as payload_alive", (0, int(case["keepAlias"]))).fetchone()
            assert row == (case["deniedItems"], case["terminalEmpty"], case["payloadAlive"])
            print("[DEBUG] Independent SQLite tool-root retirement", case["id"], row)


if __name__ == "__main__":
    main()
