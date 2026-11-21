from pathlib import Path
import os
import re
import subprocess
import tomllib


ticket = Path(__file__).resolve().parent
out = ticket / "🗑️generated"
out.mkdir(exist_ok=True)
base = Path("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any").resolve()
meta = Path(os.environ.get("FEM3D_HOUSE_ARCHIVED_METADATA", out / "m"))
archive = Path(os.environ.get("FEM3D_HOUSE_ARCHIVED_LIBRARIES", "E:/b/fem3d-rcpt/probe-flat/lib"))
compiler = ["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]
toolchain = Path(subprocess.check_output([*compiler, "--print", "sysroot"], encoding="utf-8").splitlines()[-1])
linker = toolchain / "lib/rustlib/x86_64-pc-windows-msvc/bin/rust-lld.exe"
flags = ["--edition=2024", "-Copt-level=1", "-Cstrip=debuginfo", *(f"-Ldependency={p}" for p in (meta, archive, out))]
current = {}


def external(label, crate, suffix):
    base_name = f"{crate}-{suffix}"
    locations = (meta / f"lib{base_name}.rmeta", archive / f"lib{base_name}.rlib", archive / f"{base_name}.dll")
    return [f"--extern={label}={path}" for path in locations if path.exists()]


def run(name, args):
    response = out / f"{name}.rsp"
    response.write_text("\n".join([*flags, *args]), encoding="utf-8")
    subprocess.run([*compiler, "@" + str(response)], check=True)


def package(folder, crate, suffix):
    manifest = folder / "📦️packages/🦀️rust/Cargo.toml"
    info = tomllib.loads(manifest.read_text(encoding="utf-8"))
    source = manifest.parent / info["lib"]["path"]
    catalog = subprocess.check_output([*compiler, "-Zls=root", str(meta / f"lib{crate}-{suffix}.rmeta")], encoding="utf-8")
    listed = dict((name, suffix) for name, suffix in re.findall(r"^\d+ (\w+)-([a-f0-9]+) hash ", catalog, re.M))
    dependencies = []
    for dependency, metadata in info["dependencies"].items():
        label = dependency.replace("-", "_")
        actual = metadata.get("package", dependency).replace("-", "_")
        if actual in current:
            dependencies.append(f"--extern={label}={current[actual]}")
        elif actual in listed:
            dependencies.extend(external(label, actual, listed[actual]))
        elif label in listed:
            dependencies.extend(external(label, label, listed[label]))
        else:
            print(f"[DEBUG] Cannot resolve {label} ({actual}) for {crate}", flush=True)
    compiled = out / f"lib{crate}-housefresh.rlib"
    run(crate, ["--crate-type=rlib", f"--crate-name={crate}", "-Cmetadata=housefresh", str(source), "-o", str(compiled), *dependencies])
    current[crate] = compiled


package(base / "🎬️scene", "semio_s_artifact_fem_3d_scene", "ec13ef27582d5477")
archived = {
    "semio_plugin": "dd352147c783eebd",
    "semio_s_artifact_fem_3d_model": "e4ec249b4997b105",
    "semio_s_artifact_fem_3d_editor_interaction": "2683654c805d72ff",
    "semio_s_artifact_fem_3d_editor_terminology": "32b78334302af245",
    "semio_s_artifact_fem_3d_editor_model_window": "aae1ccbf419fba9f",
    "semio_s_artifact_fem_3d_editor_results_window": "b6ca8443e305d945",
    "semio_s_fem_app_surface": "5fe689e19eea0237",
    "semio_ui_contract": "ad9612288c8ac264",
    "semio_ui_runtime_presentation": "050e3ce73802f7eb",
    "semio_ui_runtime_reconcile_cursor": "750b3941c319a088",
    "semio_ui_runtime_reconcile_primitives": "cd70533118c38085",
}
deps = [f"--extern={name}={path}" for name, path in current.items()]
for name, suffix in archived.items():
    deps.extend(external(name, name, suffix))
deps.extend(external("previous_scene", "semio_s_artifact_fem_3d_scene", "ec13ef27582d5477"))
run("house-scene-native", [f"-Clinker={linker}", "-Clinker-flavor=lld-link", "-Clink-arg=/DEBUG:NONE",
                           str(ticket / "house-scene-native.rs"), "-o", str(out / "house-scene-native.exe"), *deps])
