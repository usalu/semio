"""🧾 Typecheck the live actor ABI receipt with matching native dependency metadata."""
from pathlib import Path
import json
import os
import subprocess
import sys

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
lane = Path(sys.argv[1]).resolve()
scratch = ticket / "🗑️generated" / "abi"
scratch.mkdir(parents=True, exist_ok=True)
owner = root / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io"
receipt = owner / "📥️actor-input-receipt/🦀️.rs"
declaration = '#[path = "📥️actor-input-receipt/🦀️.rs"]\npub mod actor_input_receipt;'
assert declaration in (owner / "🦀️.rs").read_text(encoding="utf-8")
metadata = {}
for sidecar in lane.glob("build/*/*/out/*.rmeta.semio-deps"):
    package = sidecar.name.removeprefix("lib").removesuffix(".rmeta.semio-deps")
    value = json.loads(sidecar.read_text(encoding="utf-8"))
    metadata[(package, value["svh"])] = (sidecar.with_suffix(""), value["dependencies"])

seed_name = "semio_framework_pack_json_output_receiving"
seeds = [key for key in metadata if key[0].rsplit("-", 1)[0] == seed_name]
assert len(seeds) == 1, seeds
seed = seeds[0]
deps = {package.rsplit("-", 1)[0]: (package, svh) for package, svh in metadata[seed][1]}
direct = [seed, *(deps[name] for name in (
    "semio_framework_job", "semio_framework_job_native_decode", "semio_framework_value"
))]
mounted = scratch / "receipt.rs"
source_path = Path(os.path.relpath(receipt, scratch)).as_posix()
mounted.write_text(
    f'#[path = {json.dumps(source_path, ensure_ascii=False)}]\npub mod actor_input_receipt;\n'
    'pub use actor_input_receipt::{OriginalActorInputFailureIdentity, OriginalActorInputFailureView, '
    'OriginalJsonOutputFailure, retire_step};\n'
    'pub fn original_json_failure(value: OriginalJsonOutputFailure) '
    '-> semio_framework_pack_json_output_receiving::OriginalJsonOutputFailure { value }\n',
    encoding="utf-8",
)
closure = set(direct)
for package in direct:
    closure.update(tuple(key) for key in metadata[package][1])
paths = sorted({metadata[key][0].parent for key in closure if key in metadata})
args = ["--edition=2021", "--crate-name", "actor_abi_receipt_reference", "--crate-type=lib", "--emit=metadata"]
args += [f"-Ldependency={path}" for path in paths]
args += [f"--extern={key[0].rsplit('-', 1)[0]}={metadata[key][0]}" for key in direct]
args += ["-o", str(scratch / "libactor_abi_receipt_reference.rmeta"), str(mounted)]
for package in direct:
    print("[DEBUG] exact native crate", *package, flush=True)
subprocess.run(["repo", "tools", "exec", "--light", "rustc", "--", *args], check=True)
print("[DEBUG] live actor ABI receipt module and shared type identity checked")
