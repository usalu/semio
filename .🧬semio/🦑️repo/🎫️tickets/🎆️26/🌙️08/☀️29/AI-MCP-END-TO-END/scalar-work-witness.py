from pathlib import Path
from statistics import median
import json
import os
import re
import shlex
import subprocess
import sys
import tempfile

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
build = generated / "native-mcp" / "debug" / "build"
output = generated / "scalar-work"
output.mkdir(exist_ok=True)
channel = re.search(r'^channel\s*=\s*"([^"]+)"', Path("rust-toolchain.toml").read_text(), re.M)[1]
rustc = ["repo", "tools", "exec", "--light", "rustup", "--", "run", channel, "rustc", "--edition=2021",
         "-Copt-level=3", "-Cdebug-assertions=on", "-Coverflow-checks=on", "-Cdebuginfo=0", "-Zthreads=1"]
wasm = output / "loop.wasm"
wasm.write_bytes(bytes.fromhex("0061736d010000000105016000017f030201000707010372756e00000a1d011b01017f410021000340200041016a220041c0843d480d000b20000b"))
oracle = ["repo", "tools", "exec", "--light", "bun", "--", "-e", "console.log(new WebAssembly.Instance(new WebAssembly.Module(await Bun.file(process.argv[1]).arrayBuffer())).exports.run());", str(wasm)]
expected = int(subprocess.check_output(oracle, text=True).strip())

rows = {}
for fingerprint in build.glob("*/*/fingerprint/lib-*.json"):
    unit = fingerprint.parent.parent
    name = fingerprint.stem.removeprefix("lib-")
    for extension in ("rlib", "dylib", "so", "dll"):
        artifact = unit / "out" / f"lib{name}-{unit.name}.{extension}"
        if artifact.is_file() and fingerprint.with_suffix("").is_file():
            rows[fingerprint.with_suffix("").read_text().strip()] = (name, artifact, json.loads(fingerprint.read_text()))
            break


def direct(row):
    return {name: rows[key.to_bytes(8, "little").hex()] for _, name, _, key in row[2]["deps"] if name != "build_script_build"}


def paths(row, closure):
    file = row[1]
    if file not in closure:
        closure.add(file)
        for dependency in direct(row).values():
            paths(dependency, closure)


name = "semio_plugin_interpreter"
for core in sorted((row for row in rows.values() if row[0] == name), key=lambda row: row[1].stat().st_mtime, reverse=True):
    try:
        dependencies = direct(core)
        if "testkit" not in json.loads(dependencies["semio_framework_job"][2]["features"]):
            continue
        closure = set()
        paths(core, closure)
    except KeyError:
        continue
    break
else:
    raise SystemExit("The optimized native build with a complete fixture Source is required")

search = [part for directory in sorted({file.parent for file in closure}) for part in ("-L", f"dependency={directory}")]


def extern(alias, artifact):
    metadata = artifact.with_suffix(".rmeta")
    paths = [metadata, artifact] if metadata.is_file() else [artifact]
    return [part for path in paths for part in ("--extern", f"{alias}={path}")]


def compile(arguments):
    result = subprocess.run([*rustc, *arguments])
    if result.returncode:
        raise SystemExit(result.returncode)


externs = [part for alias, row in dependencies.items() for part in extern(alias, row[1])]
source = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🦀️.rs").resolve()
baseline = output / f"lib{name}_baseline.rlib"
current = output / f"lib{name}_current.rlib"


def compile_library(path, root):
    compile(["--crate-type=rlib", "--crate-name", name, *externs, *search,
             "-Cmetadata=scalar-work-witness", f"--remap-path-prefix={root.parent}={source.parent}", str(root), "-o", str(path)])


unit = core[1].parent.parent.name
dep = core[1].parent / f"{name}-{unit}.d"
references = dep.read_text().splitlines()[0].split(": ", 1)[1]
invocation = Path("🏗️instance/▶️invocation/🦀️.rs")
before = "{if !grow_values(&mut machine.values,cx)?{return Ok(CoreOriginalInvocationStatus::Pending)}}if !pay_work(cx)?{return Ok(CoreOriginalInvocationStatus::Pending)}"
after = "{if !grow_values(&mut machine.values,cx)?||!pay_work(cx)?{return Ok(CoreOriginalInvocationStatus::Pending)}}else{if cx.deadline_exceeded(){return Ok(CoreOriginalInvocationStatus::Pending)}cx.consume_fuel(1);}"
with tempfile.TemporaryDirectory(prefix="base-", dir=output) as directory:
    root = Path(directory)
    for reference in set(shlex.split(references)):
        target = Path(reference).resolve()
        relative = target.relative_to(source.parent)
        alias = root / relative
        assert len(os.fsencode(alias)) <= 256, alias
        alias.parent.mkdir(parents=True, exist_ok=True)
        if relative == invocation:
            code = target.read_text()
            assert code.count(after) == 1
            alias.write_text(code.replace(after,before), encoding="utf8")
        else:
            os.link(target, alias)
    compile_library(baseline, root / source.name)
compile_library(current, source)

executables = {}
for variant, artifact in (("baseline", baseline), ("current", current)):
    suffix = ".exe" if sys.platform == "win32" else ""
    exe = output / (variant + suffix)
    witness_externs = [part for alias in ("semio_framework_actor", "semio_framework_job", "semio_framework_value")
                       for part in extern(alias, dependencies[alias][1])]
    compile([*witness_externs, *extern(name, artifact), *search,
             str(ticket / "scalar-work-witness.rs"), "-o", str(exe)])
    executables[variant] = exe

timings = {name: [] for name in executables}
expected_receipt = None
for repeat in range(7):
    order = list(executables) if repeat % 2 == 0 else list(reversed(executables))
    for variant in order:
        nanos, result, *receipt = map(int, subprocess.check_output([str(executables[variant]), str(wasm)], text=True).split())
        assert result == expected
        if expected_receipt is None:
            expected_receipt = receipt
        assert receipt == expected_receipt
        timings[variant].append(round(nanos / 1e6, 3))
print("[DEBUG]", json.dumps(dict(result=expected, receipt=expected_receipt, timingsMs=timings, mediansMs={name:median(times) for name,times in timings.items()})))
