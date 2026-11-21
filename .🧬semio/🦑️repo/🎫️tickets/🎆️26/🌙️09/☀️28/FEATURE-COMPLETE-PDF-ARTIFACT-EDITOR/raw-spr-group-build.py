"""🔨 Compiles the staged actual causal package with the governed native cohort."""

import json
from pathlib import Path
import re
import runpy
import struct
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
CACHE = Path(sys.argv[1]).resolve()
OUT = TICKET / "🗑️generated/raw-spr-group"
STAGE = runpy.run_path(str(TICKET / "raw-spr-group-stage.py"))
STAGE["stage"]()
POST = json.loads(STAGE["OUT"].read_text(encoding="utf8"))["candidate"]
OUT.mkdir(parents=True, exist_ok=True)

MAIN = STAGE["MANIFEST"].with_name("🦀️.rs")


def rewritten(path, stage):
    content = POST.get(str(path))
    if content is None:
        content = (ROOT / path).read_text(encoding="utf8")
    return re.sub(r'(#\[path\s*=\s*)"([^"]+)"',
                  lambda m: f'{m[1]}"{stage(m[2])}"', content)


binary = OUT / "binary.rs"
child = OUT / "multipart.rs"
child.write_text(POST[str(STAGE["MODULE"])], encoding="utf8")
binary.write_text(rewritten(STAGE["OWNER"], lambda path: child if "atomic-group" in path
                            else (ROOT / STAGE["OWNER"].parent / path).resolve()), encoding="utf8")
entry = OUT / "causal.rs"
entry.write_text(rewritten(MAIN, lambda path: binary if "💾️binary" in path
                           else (ROOT / MAIN.parent / path).resolve()), encoding="utf8")

DEPENDENCIES = (
    "semio_framework_replication_codec", "semio_framework_replication_edit", "semio_framework_replication_error",
    "semio_framework_replication_ids", "semio_framework_replication_mutation", "semio_framework_replication_wire_core",
    "semio_framework_value", "semio_framework_hash", "serde",
)

args = ["--crate-type=rlib", "--edition=2021", "--crate-name", "semio_framework_replication_causal", str(entry),
        "-o", str(OUT / "libsemio_framework_replication_causal.rlib")]

for name in DEPENDENCIES:
    [library] = CACHE.glob(f"*/*/out/lib{name}-*.rlib")
    args += ["--extern", f"{name}={library.with_suffix('.rmeta')}", "--extern", f"{name}={library}"]
args += [f"-Ldependency={path}" for path in sorted({
    p.parent for p in CACHE.glob("*/*/out/*") if p.suffix in (".rlib", ".rmeta", ".so", ".dylib", ".dll")
})]
response = OUT / "rustc.args"
response.write_text("\n".join(args) + "\n", encoding="utf8")
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", f"@{response}"], check=True)
print("[DEBUG] Compiled the complete staged atomic-backbone causal library")

if "--law" in sys.argv:
    (OUT / "header").write_bytes(struct.pack("<4s32sIH", bytes.fromhex("80004701"), bytes([0x17] * 32), 473_683, 7))
    target = OUT / "probe"
    selected = ["semio_framework_replication_edit", "semio_framework_replication_ids", "semio_framework_value", "semio_framework_hash"]
    paths = [arg for arg in args if arg.startswith("-Ldependency=")]
    laws = ["--test", "--edition=2024", str(TICKET / "raw-spr-group-law.rs"), "-o", str(target),
            "--extern", f"semio_framework_replication_causal={OUT / 'libsemio_framework_replication_causal.rlib'}", *paths]
    for name in selected:
        [library] = CACHE.glob(f"*/*/out/lib{name}-*.rlib")
        laws += ["--extern", f"{name}={library.with_suffix('.rmeta')}", "--extern", f"{name}={library}"]
    response = OUT / "probe.args"
    response.write_text("\n".join(laws) + "\n", encoding="utf8")
    subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", f"@{response}"], check=True)
    subprocess.run(["repo", "capacity", "run", "--memory=256", "--", str(target), "--nocapture"], check=True)
