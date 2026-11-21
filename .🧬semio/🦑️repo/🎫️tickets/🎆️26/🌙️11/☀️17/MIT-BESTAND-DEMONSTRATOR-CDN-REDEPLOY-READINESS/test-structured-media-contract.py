"""🎟️ Check the shipped media export declaration against the Rust admission rule."""
from pathlib import Path
import json
import re
import sqlite3
import subprocess
import sys

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
policy = (root / "🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs").read_text()
media = (root / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎞️media/📤️structured/🦀️.rs").read_text()
fixture = json.loads((ticket / "🔣️structured-media-contract.json").read_text())
contract = policy[policy.index("#[derive", policy.index("/// 🔒️ Exact, reviewable admission bounds")):policy.index("pub struct ErasedToolJob")]
declaration = re.search(r"fn execution_contract\(&self\) -> ToolExecutionContract \{(.*?)\n    \}", media, re.S)[1]
out = ticket / "🗑️generated"
program = out / "structured-media-contract.rs"
program.write_text(contract + "\nfn main(){for output in [" + ",".join(str(i) for i in fixture["outputBytes"]) + "]{let c={" + declaration.replace("S::MAX_BYTES","output") + "}; println!(\"{} {} {}\",c.max_step_micros,c.max_output_bytes,c.validate().is_ok());}}")
exe = out / "structured-media-contract"
subprocess.run([sys.argv[1],program,"-o",exe],check=True)
db=sqlite3.connect(":memory:")
lines=subprocess.check_output([exe],text=True).splitlines()
assert len(lines)==len(fixture["outputBytes"])
for output,line in zip(fixture["outputBytes"],lines):
    micros,volume,accepted=line.split()
    expected=db.execute("select ? > 0 and ? < ? and ? = ?",(int(micros),int(micros),fixture["exclusiveMicros"],int(volume),output)).fetchone()[0]
    assert (accepted=="true")==bool(expected) and expected, line
