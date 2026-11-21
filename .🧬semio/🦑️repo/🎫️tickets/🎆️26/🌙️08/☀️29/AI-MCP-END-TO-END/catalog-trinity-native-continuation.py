"""Link the real Trinity composition and descriptor emitter using compiled owner dependencies."""

from collections import defaultdict
from pathlib import Path
import os
import re
import subprocess

root = Path.cwd()
ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
target = generated / "native-mcp"
build = (target / "debug/build").resolve()
output = target / "catalog-trinity-native-continuation"
output.mkdir(exist_ok=True)

channel = re.search(
    r'^channel\s*=\s*"([^"]+)"', (root / "rust-toolchain.toml").read_text(), re.M
)[1]
rustc = ["rustup", "run", channel, "rustc"]
owner = root / "🌎️hub/🧩️compositions/🔱️trinity"
describe = root / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe"
environment = {**os.environ, "SEMIO_BUILD_MODE": "prototype"}

compiled_roots = {
    "semio-s-dev-trinity-jack": "a1b90e2fd690fa81",
    "semio-s-artifact-trinity-rewriting": "f7ee37178fead734",
    "semio-s-artifact-trinity-rewriting-editor": "ddc3d902ec74233c",
    "semio-plugin-interpreter-runtime": "a3737e20183fd20e",
}

def metadata(path):
    return subprocess.check_output(
        [*rustc, "-Zls=root", str(path)], text=True, env=environment
    )

tags = defaultdict(set)
for package, unit in compiled_roots.items():
    artifact, = (build / package / unit / "out").glob("*.rmeta")
    for tag in re.findall(
        r"^(?:[0-9]+ |name )([\w]+-[0-9a-f]{16})(?: |$)", metadata(artifact), re.M
    ):
        crate = tag.rsplit("-", 1)[0]
        tags[crate].add(tag)

search = sorted(build.glob("*/*/out"))
artifacts = defaultdict(list)
for directory in search:
    for artifact in directory.glob("lib*"):
        artifacts[artifact.stem[3:]].append(artifact)

def externs(dependencies):
    arguments = []
    for dependency in dependencies:
        tag, = tags[dependency]
        files = [p for p in artifacts[tag] if p.suffix in (".rmeta", ".rlib", ".dylib", ".so", ".dll")]
        assert files and any(p.suffix != ".rmeta" for p in files), tag
        print("[DEBUG] Link dependency", tag, flush=True)
        arguments.extend(f"--extern={dependency}={path}" for path in files)
    return arguments

def manifest_dependencies(directory):
    manifest = (directory / "📦️packages/🦀️rust/Cargo.toml").read_text()
    section = manifest.split("[dependencies]\n", 1)[1].split("\n[", 1)[0]
    names = re.findall(r"^([a-z][a-z0-9-]+)[.\s]*=", section, re.M)
    return [name.replace("-", "_") if name != "semio-framework-pack" else "pack" for name in names]

def compile_crate(name, source, crate_type, dependencies, extra=()):
    print("[DEBUG] Compile", name, source, flush=True)
    args = [
        f"--crate-name={name}", "--edition=2021", f"--crate-type={crate_type}",
        "-Cembed-bitcode=no", "-Cdebuginfo=0", "-Zthreads=1",
    ]
    args.extend(f"-Ldependency={directory}" for directory in search)
    args.extend(externs(dependencies))
    args.extend(extra)
    artifact = output / (
        f"lib{name}.rlib" if crate_type == "rlib"
        else f"{name}.exe" if os.name == "nt" else name
    )
    args.extend([str(source), f"--emit=link={artifact}"])
    arguments = output / f"{name}.args"
    arguments.write_text("\n".join(args) + "\n")
    subprocess.run([*rustc, "@" + str(arguments)], env=environment, check=True)
    return artifact

bundle = compile_crate(
    "semio_hub_trinity", owner / "🦀️.rs", "rlib", manifest_dependencies(owner)
)
descriptor = compile_crate(
    "semio_plugin_describe", describe / "🛂️descriptor-emission/🦀️.rs", "rlib",
    manifest_dependencies(describe),
)
emitter = compile_crate(
    "mcp_native_descriptor", ticket / "mcp-native-descriptor.rs", "bin",
    ["semio_framework_async", "semio_framework_value", "semio_framework_pack_json", "semio_os_kernel", "semio_plugin"],
    ['--cfg=feature="trinity"', f"--extern=semio_hub_trinity={bundle}", f"--extern=semio_plugin_describe={descriptor}"],
)
subprocess.run(
    [str(emitter), "trinity", str(generated / "descriptor-refresh/trinity")],
    env=environment, check=True,
)
