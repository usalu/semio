import os
import subprocess
import sys
from pathlib import Path

ticket = Path(__file__).resolve().parent
workspace = ticket.parents[6]
generated = ticket / "🗑️generated"
rustc = os.environ.get("RUSTC", "rustc")
lane = workspace / ".cache/b/r/m/u/l" / sys.argv[1] / "wasm32-wasip2/prototype/build"
output = generated / "record-receiving"
output.mkdir(exist_ok=True)
crate = "semio_framework_pack_record_retained"

def libraries(package):
    paths = sorted((lane / package).glob("*/out/lib*.rlib"))
    if len(paths) != 1:
        raise RuntimeError(f"Expected one library for {package}, found {len(paths)}")
    return external(package.replace("-", "_"), paths[0])

def external(name, link):
    metadata = link.with_suffix(".rmeta")
    arguments = ["--extern", f"{name}={link}"]
    if metadata.is_file():
        arguments.extend(("--extern", f"{name}={metadata}"))
    return arguments

def compile(label, options, env=None):
    flags = output / f"{label}.args"
    flags.write_text("\n".join(options) + "\n")
    subprocess.run([rustc, "@" + str(flags)], check=True, env=env)

def build(name, replacement=None):
    args = (generated / f"{name}-live.args").read_text().splitlines()
    position = args.index("--emit=metadata")
    del args[position:position + 3]
    for index, argument in enumerate(args[:]):
        if "=/" in argument and argument.endswith(".rmeta"):
            alias, path = argument.split("=", 1)
            if replacement and alias == replacement[0]:
                args[index] = f"{alias}={replacement[1].with_suffix('.rmeta')}"
                link = replacement[1]
            else:
                link = Path(path).with_suffix(".rlib")
            if link.is_file():
                args.extend(("--extern", f"{alias}={link}"))
    rlib = output / f"lib{name}.rlib"
    compile(name, [*args, "-Copt-level=1", f"--emit=link={rlib},metadata={rlib.with_suffix('.rmeta')}"])
    return args, rlib

catalog = "semio_framework_pack_format_catalog"
_, catalog_lib = build(catalog)
args, rlib = build(crate, (catalog, catalog_lib))
fixture = next((workspace / "🧰️framework/🔨️modules").glob("*pack/*value/🛬️decode/*receiving/*tests/*fixtures/🔣️.json"))
binary = output / "record-receiving.wasm"
proof = ["--edition=2021", "--target=wasm32-wasip2", "-Cstrip=symbols", "-o", str(binary), str(ticket / "check-record-receiving.rs")]
proof.extend(argument for argument in args if argument.startswith("-Ldependency="))
proof.append(f"-Ldependency={output}")
proof.extend(external(crate, rlib))
for package in (
    "semio-framework-dsl-record", "semio-framework-pack-record", "semio-framework-replication-codec",
    "semio-framework-value", "semio-framework-value-control-contract", "semio-framework-value-native-owned", "serde_json",
    "semio-framework-pack", "semio-os-semio", "semio-s-artifact-norm-din18599-model",
    "semio-s-artifact-norm-din18599-io", "semio-s-artifact-stdio-semio-table-model",
):
    proof.extend(libraries(package))
compile("proof", proof, dict(os.environ, RECORD_RECEIVING_FIXTURE=str(fixture), DIN18599_CAPTURED_PACK=str(generated / "din18599-observed.pack")))
subprocess.run([workspace / "node_modules/.bin/jco", "run", "--jco-dir", output / "js", binary], check=True)
