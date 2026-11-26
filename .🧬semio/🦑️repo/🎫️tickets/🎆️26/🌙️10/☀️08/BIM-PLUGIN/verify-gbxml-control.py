"""🌿️ Rebuild and exercise the gbXML exporter against matched test archives."""
from pathlib import Path
import json
import os
import re
import subprocess
import sys

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
archives = generated / "fixture-link"
wasi = "--wasi" in sys.argv
output = generated / ("wasi-gbxml-control" if wasi else "gbxml-control")
output.mkdir(exist_ok=True)
captures = generated / "compiler-retry"
record = captures / sys.argv[sys.argv.index("--native-receipt") + 1] if "--native-receipt" in sys.argv else max(
    captures.glob("w-svg-*.success.json" if wasi else "n-bim-*.success.json"), key=lambda p: p.stat().st_mtime)
receipt = json.loads(record.read_text())
rustc = receipt["compiler"]
original_args = receipt["arguments"]
search = [Path(original_args[i + 1].split("=", 1)[1]) for i, value in enumerate(original_args[:-1])
          if value == "-L" and original_args[i + 1].startswith(("dependency=", "native="))]
by_filename = {}
build = generated / ("b/wasm32-wasip2/debug/build" if wasi else "ntb/debug/build")
latest = generated / "refresh-wasi-energy-final"
for directory in ([latest, latest / "coherent"] if wasi else []) + [*search, *sorted(build.glob("*/*/out")), archives]:
    for file in directory.glob("lib*"):
        if file.is_file():
            by_filename.setdefault(file.name, file)
root = Path.cwd()
bim = root / "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model"
base = bim / "🏅️standards/🔖️1/🪆️subsets/✳️any"
prefix = "semio_s_artifact_bim_model_"
gbxml = prefix + "gbxml"
inferences = prefix + "inferences"
components = prefix + "components"
changed = {components, inferences, prefix + "curtain_walls"}


def identities(archive):
    view = subprocess.check_output([rustc, "-Zls=root", str(archive)], text=True)
    return dict(re.findall(r"^\d+ (\w+)-(\w+) hash ", view, re.M))


def archive(name, hash):
    return next(by_filename[key] for ext in (".rlib", ".dylib", ".so", ".dll")
                if (key := f"lib{name}-{hash}{ext}") in by_filename)


def externs(arguments, name, file):
    arguments += ["--extern", f"{name}={file}"]
    if file.suffix == ".rlib" and file.with_suffix(".rmeta").exists():
        arguments += ["--extern", f"{name}={file.with_suffix('.rmeta')}"]


base_args = [rustc, "--edition=2021", "-Zno-parallel-backend", "-Zthreads=1",
             "-L", f"dependency={archives}", "-L", f"dependency={output}", "-L", f"native={archives}"]
if wasi:
    base_args += ["--target=wasm32-wasip2", "-L", f"dependency={latest}", "-L", f"dependency={latest / 'coherent'}"]
for index, value in enumerate(original_args[:-1]):
    if value == "-L":
        base_args += [value, original_args[index + 1]]
original_inference = next(Path(value.split("=", 1)[1]).stem.rsplit("-", 1)[1] for value in original_args
                          if value.startswith(f"{inferences}=") and value.endswith(".rmeta"))
original = latest / f"lib{gbxml}-energyfix.rmeta" if wasi else next(
    file for key, file in by_filename.items() if key.startswith(f"lib{gbxml}-")
    and key.endswith(".rmeta") and identities(file).get(inferences) == original_inference)
gbxml_deps = identities(original)
built = {}
manifests = {}
for manifest in base.rglob("Cargo.toml"):
    package_name = re.search(r'^name = "([^"]+)"', manifest.read_text(), re.M)
    if package_name:
        manifests[package_name[1].replace("-", "_")] = manifest


def rebuild(name, origin):
    if name in built:
        return
    rebuilt = output / f"lib{name}-gbxml_control.rlib"
    if "--gbxml-only" in sys.argv and name != gbxml and rebuilt.is_file():
        built[name] = rebuilt
        return
    manifest = manifests[name]
    package = manifest.parent
    dependencies = identities(origin.with_suffix(".rmeta"))
    declarations = manifest.read_text().split("[dependencies]")[1].split("\n[")[0]
    declared = re.findall(r"^([\w-]+)(?:\.workspace)?\s*=", declarations, re.M)
    args = [*base_args, "--crate-name", name, "--crate-type", "rlib", "--emit=link,metadata",
            "-Cmetadata=gbxml_control", "-Cextra-filename=-gbxml_control", "--out-dir", str(output)]
    for field in declared:
        alias = field.replace("-", "_")
        if alias not in dependencies:
            continue
        dependency = archive(alias, dependencies[alias])
        if alias in manifests and (alias in changed or changed.intersection(identities(dependency.with_suffix(".rmeta")))):
            rebuild(alias, dependency)
        externs(args, alias, built.get(alias, dependency))
    env = dict(os.environ, CARGO_MANIFEST_DIR=str(package), CARGO_PKG_NAME=name.replace("_", "-"), CARGO_PKG_VERSION="0.1.0")
    print(f"[DEBUG] Rebuilding {name}", flush=True)
    if subprocess.run([*args, str(package / "🦀️.rs")], cwd=package, env=env).returncode:
        raise SystemExit(f"Failed to build {name}")
    built[name] = rebuilt


if "--link-only" in sys.argv:
    built = {name: output / f"lib{name}-gbxml_control.rlib" for name in (gbxml, inferences, components)}
else:
    rebuild(gbxml, original)
binary = output / ("gbxml-control-tests.wasm" if wasi else "gbxml-control-tests")
args = [*base_args, "--test", str(ticket / "gbxml-control-native.rs"), "--crate-name", "gbxml_control_laws", "-o", str(binary)]
if "--funded-tests" in sys.argv:
    args += ["--cfg", 'feature="funded-tests"']
if wasi:
    args += ["-Clink-arg=--threads=1", "-Clink-arg=--strip-all", "-Clink-arg=--max-memory=536870912", "-Clink-arg=-zstack-size=8388608"]
for name in (gbxml, inferences, components):
    externs(args, name, built[name])
for name in ("semio_framework_value", "semio_framework", "semio_framework_job", "semio_os_kernel", "semio_plugin",
             "semio_s_artifact_bim_model_snapshot", "semio_s_artifact_stdio_xml"):
    externs(args, name, archive(name, gbxml_deps[name]))
if subprocess.run(args).returncode:
    raise SystemExit("Failed to link the gbXML control replay")
if not wasi:
    test_args = ["--skip", "serializer_emits_the_committed_gbxml_files"] if "--funded-tests" in sys.argv else []
    subprocess.run([str(binary), *test_args], check=True)
