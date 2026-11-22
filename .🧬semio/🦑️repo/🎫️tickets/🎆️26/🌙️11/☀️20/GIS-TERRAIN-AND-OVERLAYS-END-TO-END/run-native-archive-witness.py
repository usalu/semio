from pathlib import Path
import hashlib
import re
import subprocess
import sys


ticket = Path(__file__).resolve().parent
output = ticket / "🗑️generated"
library = output / "qr"
native = next(Path.cwd().glob(".*/🦑*/🎫*/🎆26/🌙11/☀*/GIS-TERRAIN-AND-OVERLAYS-END-TO-END")) / "🗑️generated/n/debug/build"
macros = output / "aw"
macros.mkdir(exist_ok=True)
for artifact in native.glob("*/*/out/*.dylib"):
    link = macros / artifact.name
    if not link.is_symlink():
        link.symlink_to(artifact)
toolchain = re.search(r'^channel = "([^"]+)"$', Path("rust-toolchain.toml").read_text(), re.M)[1]
binary = output / "native-archive-witness"
archive = output / "mounted-terrain.semio-archive"
assert hashlib.sha256(archive.read_bytes()).hexdigest() == "4180dd9b9303b2b0a8f4228715bb85c7c91aca6e590ed4d475132788ffc04dbf"
history_only = len(sys.argv) > 1 and sys.argv[1] == "history"
libraries = {
    "semio_plugin": "210b2103f4be30e7",
    "semio_os_kernel": "bb17cc266bb76573",
    "semio_os_spr": "c78228f82adacf8a",
    "semio_s_artifact_gis_gisterrain_editor": sys.argv[1] if len(sys.argv) > 1 else "526cc1907456600d",
}
if history_only:
    libraries = {
        "semio_os_spr": libraries["semio_os_spr"],
        "protocol": "46d8e718b1b8ec6d",
        "semio_framework_value": "05431580f111efc8",
        "semio_framework_value_native_owned": "996df75d9ed9fa1b",
        "semio_framework_job": "31261f37020cf87c",
        "semio_framework_job_native_decode": "01f0fe32b5c8b42a",
        "semio_framework_job_testing_caller": "3495cdbbce9436b7",
    }
rust = ["repo", "tools", "exec", "--light", "rustup", "--", "run", toolchain, "rustc"]
rust += ["--edition=2024", "-C", "opt-level=1", "-C", "debuginfo=1", "-L", f"dependency={library}", "-L", f"dependency={macros}", "-o", str(binary), str(ticket / "native-archive-witness.rs")]
if history_only:
    rust += ["--cfg", "history_only"]
for name, fingerprint in libraries.items():
    rlib = library / f"lib{name}-{fingerprint}.rlib"
    assert rlib.is_file(), rlib
    rust += ["--extern", f"{name}={rlib}"]
    metadata = rlib.with_suffix(".rmeta")
    if metadata.is_file():
        rust += ["--extern", f"{name}={metadata}"]
with (output / "native-archive-witness.log").open("w") as log:
    result = subprocess.run(rust, stdout=log, stderr=log)
    if result.returncode == 0:
        result = subprocess.run([str(binary), str(archive)], stdout=log, stderr=log)
raise SystemExit(result.returncode)
