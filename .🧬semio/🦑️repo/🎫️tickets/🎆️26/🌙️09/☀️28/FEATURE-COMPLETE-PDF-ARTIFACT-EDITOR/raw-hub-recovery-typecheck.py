"""🧪️ Checks exact guarded DB recovery control against the governed Store recovery host."""

import json
from pathlib import Path
import re
import runpy
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
stage = runpy.run_path(str(TICKET / "raw-hub-paged-db-admission.py"))
stage["baseline"]()
overlay = json.loads(stage["OUT"].read_text(encoding="utf8"))
source = overlay["candidate"][str(stage["RECOVERY"])]
bounds = source.split("where\n", 1)[1].split("\n{", 1)[0]
body = re.search(r"match \{(\n\s+let mut report_progress = .*?host\.advance\(grant, &mut receiving\)\n\s+)\} \{", source, re.S)[1]
out = TICKET / "🗑️generated/raw-store-inbox/exact"
out.mkdir(exist_ok=True, parents=True)
probe = out / "recovery.rs"
probe.write_text(f"""
extern crate semio_os_kernel as pack;
extern crate semio_os_kernel as store;
pub fn checked<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
    host: &mut store::durable_group::DurableOwnedMapRecoveryHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>,
    grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
) -> store::durable_group::DurableOwnedMapRecoveryAdvanceV1
where
{bounds}
{{{body}}}
""", encoding="utf8")
base = Path(sys.argv[1]).resolve()
[kernel] = list(base.glob("semio-os-kernel/*/out/libsemio_os_kernel-*.rmeta"))
args = ["--crate-name", "db_recovery_control_probe", "--crate-type=rlib", "--edition=2021", "--emit=metadata",
        str(probe), "-o", str(out / "librecovery.rmeta"), "--extern", f"semio_os_kernel={kernel}", f"-Ldependency={out / 'deps'}"]
if "--wasm" in sys.argv:
    args += ["--target=wasm32-wasip2"]
response = out / "recovery.args"
response.write_text("\n".join(args) + "\n", encoding="utf8")
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", f"@{response}"], check=True)
print("[DEBUG] Actual Store recovery host accepts the guarded DB NativeDecodeControl and existing bounds")
