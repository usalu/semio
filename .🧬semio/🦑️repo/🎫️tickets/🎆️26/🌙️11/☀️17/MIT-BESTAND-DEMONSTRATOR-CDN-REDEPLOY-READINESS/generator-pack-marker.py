import json
import os
from pathlib import Path
import subprocess
import sys
import tomllib

lane = Path(sys.argv[1])
proof = Path(sys.argv[2])
manifest = Path(sys.argv[3])
package = tomllib.loads(manifest.read_text(encoding="utf-8"))
crate = package["package"]["name"].replace("-", "_")
target = "wasm32-wasip2"
outputs = lane.glob("**/out/*")
files = {file.name: file.resolve() for file in outputs if file.suffix in (".rmeta", ".rlib", ".dylib")}
provenance = sorted(lane.glob(f"{target}/release/build/*/*/out/lib{crate}-*.rmeta.semio-deps"))
assert len(provenance) == 1, provenance
identities = [entry[0] for entry in json.loads(provenance[0].read_text())["dependencies"]]
externs = {}
deps = proof / "deps"
deps.mkdir(parents=True, exist_ok=True)
for identity in identities:
    name = identity.rsplit("-", 1)[0]
    for suffix in (".rmeta", ".rlib", ".dylib"):
        file = files.get("lib" + identity + suffix)
        if file is None:
            continue
        destination = deps / file.name
        if not destination.is_symlink():
            destination.symlink_to(file)
        externs.setdefault(name, destination)
args = ["--edition=2021", "--crate-name", crate, "--crate-type=rlib", "--target", target, f"-Ldependency={deps}"]
for dependency in package["dependencies"]:
    name = dependency.replace("-", "_")
    args.extend(("--extern", f"{name}={externs[name]}"))
emit = sys.argv[4] if len(sys.argv) > 4 else "metadata"
extension = "rmeta" if emit == "metadata" else "rlib"
args.extend((str((manifest.parent / package["lib"]["path"]).resolve()), f"--emit={emit}", "-o", str(proof / f"lib{crate}.{extension}")))
rustc = ["repo", "tools", "exec", "--light", "rustc", "--", "+nightly-2026-07-20"]
subprocess.run(rustc + args, check=True)

if emit == "link":
    sysroot = Path(subprocess.check_output(rustc + ["--print", "sysroot"], text=True).strip())
    version = subprocess.check_output(rustc + ["-vV"], text=True)
    host = next(line.split(": ", 1)[1] for line in version.splitlines() if line.startswith("host: "))
    binary = "rust-lld.exe" if os.name == "nt" else "rust-lld"
    linker = sysroot / "lib" / "rustlib" / host / "bin" / binary
    witness = Path(__file__).with_suffix(".rs")
    shared = [
        "--edition=2021", "--target", target, "--crate-type=cdylib",
        "-C", "panic=abort", "-C", f"linker={linker}", f"-Ldependency={deps}",
    ]
    for name in ("semio_framework_value", "semio_framework_replication_codec", "semio_framework_value_control_contract"):
        shared.extend(("--extern", f"{name}={externs[name]}"))
    previous = Path(str(provenance[0]).removesuffix(".semio-deps"))
    for name, libraries in (
        ("prior", [previous, previous.with_suffix(".rlib")]),
        ("proof", [proof / f"lib{crate}.rlib"]),
    ):
        libraries_args = [value for library in libraries for value in ("--extern", f"{crate}={library}")]
        subprocess.run(rustc + shared + libraries_args + [str(witness), "-o", str(proof / f"{name}.wasm")], check=True)
    subprocess.run(["repo", "tools", "exec", "--light", "node", "--", str(witness.with_suffix(".cjs")), str(proof)], check=True)
