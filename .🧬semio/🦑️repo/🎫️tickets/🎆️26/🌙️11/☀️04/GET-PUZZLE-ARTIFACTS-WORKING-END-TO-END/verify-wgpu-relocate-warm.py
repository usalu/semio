"""🧭️ Run the native cardinal relocate law with six rebuilt World3D libraries."""

import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tomllib

repo = Path.cwd()
generated = Path(__file__).resolve().parent / "🗑️generated"
cached = generated / "cargo-puzzle-clipboard-build" / "debug" / "build"
work = generated / "relocate-warm"
deps = work / "deps"
deps.mkdir(parents=True, exist_ok=True)
world = repo / "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world"
units = ["🧩️components", "🧲️cursors", "🧱️scene", "🌉️bridge", "🧵️interaction", "🎛️host"]
environment = dict(os.environ, RUST_MIN_STACK="134217728",
                   ZIG_GLOBAL_CACHE_DIR=str(generated / "2d-zig-global"),
                   ZIG_LOCAL_CACHE_DIR=str(generated / "2d-zig-local"))


def fingerprints(path):
    metadata = subprocess.check_output(["rustc", "-Z", "ls=all", path], text=True)
    result = {}
    for name, fingerprint in re.findall(r"^\d+ ([a-z][a-z0-9_]+)-([0-9a-f]{16}) hash ", metadata, re.MULTILINE):
        result.setdefault(name, []).append(fingerprint)
    return result


def artifact(path):
    return max(path.glob("*/out/lib*.rmeta"), key=lambda item: item.stat().st_mtime_ns)


host = artifact(cached / "semio-os-infinite-world-host")
host_tree = fingerprints(host)
known = {f"lib{name}-{fingerprint}" for name, hashes in host_tree.items() for fingerprint in hashes}
native_dirs = set()
for output in cached.glob("*/*/out"):
    for source in output.iterdir():
        if source.stem in known and source.suffix in (".rmeta", ".rlib", ".so", ".dll", ".dylib"):
            destination = deps / source.name
            if not destination.exists():
                destination.symlink_to(source)
        if source.suffix == ".a":
            native_dirs.add(output)


def externs(declarations, tree, built):
    arguments = []
    for alias, details in declarations.items():
        package = details.get("package", alias) if isinstance(details, dict) else alias
        name = package.replace("-", "_")
        alias = alias.replace("-", "_")
        if name in built:
            arguments += ["--extern", f"{alias}={built[name]}"]
            continue
        candidates = [deps / f"lib{name}-{fingerprint}" for fingerprint in tree.get(name, [])]
        candidates = [path for path in candidates if any(path.with_suffix(suffix).exists() for suffix in (".rlib", ".so", ".rmeta"))]
        if not candidates:
            if isinstance(details, dict) and details.get("optional", False):
                continue
            raise SystemExit(f"[DEBUG] Warm extern absent for {name}")
        if len(candidates) != 1:
            raise SystemExit(f"[DEBUG] Warm extern ambiguous for {name}: {candidates}")
        prefix = candidates[0]
        for suffix in (".rmeta", ".rlib", ".so", ".dll", ".dylib"):
            path = prefix.with_suffix(suffix)
            if path.exists():
                arguments.extend(["--extern", f"{alias}={path}"])
    return arguments


def compile_unit(arguments, manifest_dir):
    if shutil.disk_usage(repo).free <= 12 * 2**30:
        raise SystemExit("[DEBUG] Warm relocate build stopped at its 12 GiB free-space floor")
    command = ["nice", "-n", "8", "rustc", "--edition=2021", "-Cdebuginfo=0", "-Zthreads=1",
               "-Ccodegen-units=1", "-L", f"dependency={deps}", *arguments]
    subprocess.run(command, cwd=work, check=True,
                   env=environment | {"CARGO_MANIFEST_DIR": str(manifest_dir), "CARGO_PKG_VERSION": "0.1.0"})


built = {}
for unit in units:
    manifest_dir = world / unit / "📦️packages/🦀️rust"
    manifest = tomllib.loads((manifest_dir / "Cargo.toml").read_text(encoding="utf-8"))
    package = manifest["package"]["name"]
    name = package.replace("-", "_")
    old = host if unit == units[-1] else deps / f"lib{name}-{host_tree[name][0]}.rmeta"
    tree = fingerprints(old)
    declarations = manifest.get("dependencies", {}).copy()
    for target, entries in manifest.get("target", {}).items():
        if target.startswith("cfg(not") or "wasm32" not in target:
            declarations.update(entries.get("dependencies", {}))
    output = deps / f"lib{name}-relocate.rlib"
    if "--use-built" not in sys.argv or not output.is_file():
        print(f"[DEBUG] Building {name} from current source", flush=True)
        compile_unit(["--crate-name", name, "--crate-type", "rlib", "-Cmetadata=relocate-warm",
                      world / unit / "🦀️.rs", "-o", output, *externs(declarations, tree, built)], manifest_dir)
    built[name] = output


test_source = world / "🎛️host" / "🧪️tests" / "🧲️scene-input-residency" / "🦀️.rs"
text = test_source.read_text(encoding="utf-8")


def function(name):
    return re.search(rf"^fn {name}\([\s\S]+?^}}", text, re.MULTILINE).group()


case = function("cardinal_relocate_retains_the_grabbed_plane_and_model_through_release_or_cancel")
case = case.replace("    let _gate = semio_os_infinite_world_host::world_test_gate();\n", "")
case = case.replace("state.scene.draws.push(", "state.scene.draws.push_prevalidated(")
last_assertion = "            assert_eq!(state.scene.draws[0].instances[0].model.cols, model.cols);\n        }"
if last_assertion not in case:
    raise SystemExit("[DEBUG] Warm relocate teardown seam changed")
case = case.replace(last_assertion, last_assertion + '\n        assert!(state.scene.draws.take_last().is_some());')
wrapper = work / "relocate_test.rs"
wrapper.write_text(
    "macro_rules! fixture_text { ($relative:expr) => { include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/\", $relative)) }; }\n"
    "use semio_os_infinite_world_host::World3dState;\n"
    "use semio_os_infinite_world_interaction::{world3d_begin_relocate_drag, world3d_update_relocate_drag, world3d_end_relocate_drag, world3d_cancel_relocate_drag};\n"
    "use semio_ui_theme::geometry::Rect;\n"
    "use semio_ui_viewport::{Viewport3dOrthographicView, Viewport3dProjectionMode, Viewport3dProjectionOrientation, Viewport3dProjectionSpec};\n"
    "use ui_scene::math::{Camera3d, CameraProjection3d, Instance3d, Mat4, OrbitController, SceneDraw3d, Vec3};\n"
    + "\n".join(function(name) for name in ("point", "pointer_for", "assert_point", "translation"))
    + "\n#[test]\n" + case + "\n", encoding="utf-8")
imports = {name: {} for name in ("semio_os_infinite_world_host", "semio_os_infinite_world_interaction",
                                  "semio_ui_theme", "semio_ui_viewport", "serde_json")}
imports["ui_scene"] = {"package": "semio-ui-scene"}
executable = work / "relocate-test"
link_flags = ["-Clinker-features=+lld", "-Clink-self-contained=+linker", "-Zunstable-options"]
for directory in sorted(native_dirs):
    link_flags.extend(["-L", f"native={directory}"])
compile_unit(["--test", wrapper, "-o", executable, *link_flags, *externs(imports, host_tree, built)],
             world / "🎛️host" / "📦️packages/🦀️rust")
subprocess.run([executable, "--exact", "cardinal_relocate_retains_the_grabbed_plane_and_model_through_release_or_cancel"],
               cwd=repo, check=True, env=environment)
