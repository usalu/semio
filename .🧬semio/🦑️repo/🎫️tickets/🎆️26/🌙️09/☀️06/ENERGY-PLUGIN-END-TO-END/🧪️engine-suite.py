import json
import pathlib
import subprocess

ticket = pathlib.Path(__file__).resolve().parent
packages = json.loads((ticket / "🗑️generated/engine-tested-packages.json").read_text())
command = ["python", str(ticket / "📜️energy-cargo.py"), "test", "--manifest-path", "✏️s/🔌️plugins/🔋️energy/Cargo.toml", "--lib"]
for package in packages:
    command.extend(["-p", package])
with (ticket / "🗑️generated/engine-final-tests.log").open("w") as output:
    raise SystemExit(subprocess.call(command, stdout=output, stderr=subprocess.STDOUT))
