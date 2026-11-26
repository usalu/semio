"""🧾 Compile the actual capture transition functions against the model-independent fixture."""
import json
from pathlib import Path
import subprocess

ticket=Path(__file__).resolve().parent
generated=ticket/"🗑️generated"
native=Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🏠️native")
source=(native/"🌱️projection.rs").read_text()
stage=source[source.index("#[derive(Clone, Copy, Debug, Default)]"):source.index("impl CaptureStep")]
state=source[source.index("#[derive(Debug, Default)]"):source.index("impl GuestFrozenHistoryCapture")]
transition=source[source.index(" fn complete_step("):source.index("#[cfg(test)]")]
local=source[source.index(" fn advance_local_stages"):source.index(" fn complete_step(")] if " fn advance_local_stages" in source else ""
tests=[]
for row in json.loads((native/"escrow-capture.json").read_text())["localPasses"]:
    tests.append('''{
let mut capture=GuestFrozenHistoryCapture{rows:%s,..Default::default()};
let mut calls=0u64;
capture.advance_local_stages(|_,_|{calls+=1;Ok::<_,()>((calls!=%s,calls<%s))}).unwrap();
assert_eq!(calls,%s);assert_eq!(format!("{:?}",capture.step),"%s");
}'''%(row["rows"],row["park"] or 0,row["funded"],row["calls"],row["expected"]))
probe=generated/"frozen_local_capture.rs"
probe.write_text(stage+state+"impl GuestFrozenHistoryCapture {\n"+local+transition+"fn main(){\n"+"\n".join(tests)+"\n}")
binary=generated/"frozen-local-capture"
registry=json.loads(subprocess.check_output(["repo","tools","list","--json"]))
rustup=next(row["command"] for row in registry if row["id"]=="rustup")
channel=next(line.split('"')[1] for line in Path("rust-toolchain.toml").read_text().splitlines() if line.startswith("channel"))
subprocess.run([rustup,"run",channel,"rustc",probe,"-o",binary],check=True)
subprocess.run([binary],check=True)
