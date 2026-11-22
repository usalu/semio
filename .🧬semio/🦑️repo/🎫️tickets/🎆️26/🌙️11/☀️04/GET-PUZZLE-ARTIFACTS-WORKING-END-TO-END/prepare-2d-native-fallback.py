"""🧪️ Resolve standalone native manifests for the actual Puzzle 2D test sources."""

from collections import defaultdict
import json
import os
from pathlib import Path
import shutil
import subprocess

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
generated = ticket / "🗑️generated"
workspace = root / "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/Cargo.toml"
environment = {**os.environ, "RUSTC": "rustc", "RUSTC_WRAPPER": ""}
if (generated / "cargo-home").is_dir():
    environment["CARGO_HOME"] = str(generated / "cargo-home")
metadata = json.loads(subprocess.check_output(
    ["cargo", "metadata", "--locked", "--offline", "--no-deps", "--format-version", "1",
     "--manifest-path", workspace],
    cwd=root, env=environment,
))
packages = {package["name"]: package for package in metadata["packages"]}

probe = generated / "clipboard-2d-probe"
probe_manifest = probe / "Cargo.toml"
if not probe_manifest.exists():
    paths = {
        dependency["name"]: dependency["path"]
        for package in packages.values()
        for dependency in package["dependencies"] if dependency.get("path")
    }
    paths.update({name: Path(package["manifest_path"]).parent for name, package in packages.items()})
    dependencies = {
        "puzzle2d_model": "semio-s-artifact-puzzle-2d-mut-any-10",
        "pack_json": "semio-framework-pack-json",
        "value": "semio-framework-value",
        "semio-framework-diagnostic": "semio-framework-diagnostic",
        "semio-os-kernel": "semio-os-kernel",
        "semio-os-infinite": "semio-os-infinite",
        "semio-plugin": "semio-plugin",
    }
    probe.mkdir(parents=True, exist_ok=True)
    lines = ['[package]', 'name = "puzzle-2d-clipboard-probe"', 'version = "0.1.0"',
             'edition = "2021"', '', '[workspace]', '', '[dependencies]', 'serde_json = "1"']
    for alias, name in dependencies.items():
        path = Path(os.path.relpath(paths[name], probe)).as_posix()
        lines.append(f'{alias} = {{ package = "{name}", path = "{path}" }}')
    source = Path(os.path.relpath(ticket / "verify-2d-clipboard.rs", probe)).as_posix()
    lines.extend(('', '[[bin]]', 'name = "puzzle-2d-clipboard-probe"', f'path = "{source}"'))
    probe_manifest.write_text("\n".join(lines) + "\n", encoding="utf-8")
    shutil.copyfile(workspace.with_name("Cargo.lock"), probe / "Cargo.lock")
print(probe_manifest.relative_to(ticket).as_posix())

for name, test in [
    ("v1-editor-tests", "registered-clipboard"),
    ("history-edit-runtime-tests", "history-edit-runtime"),
    ("mutations", "replacement-wire"),
]:
    package = packages[f"semio-s-artifact-puzzle-2d-{name}"]
    lib, = (target for target in package["targets"] if "lib" in target["kind"])
    directory = generated / f"puzzle-2d-{test}"
    directory.mkdir(parents=True, exist_ok=True)
    quote = lambda value: json.dumps(value, ensure_ascii=False)
    relative = lambda path: Path(os.path.relpath(path, directory)).as_posix()
    lines = [
        "[package]",
        f'name = "{package["name"]}-standalone"',
        f'version = "{package["version"]}"',
        f'edition = "{package["edition"]}"',
        f'rust-version = "{package["rust_version"]}"',
        "autolib = false",
        "autotests = false",
        "",
        "[workspace]",
        "",
        "[[test]]",
        f'name = "{test}"',
        f'path = {quote(relative(lib["src_path"]))}',
    ]
    groups = defaultdict(list)
    for dependency in package["dependencies"]:
        section = {"dev": "dev-dependencies", None: "dependencies"}[dependency["kind"]]
        if dependency["target"]:
            section = f'target.{quote(dependency["target"])}.{section}'
        key = dependency["rename"] or dependency["name"]
        source = dependency.get("path")
        fields = [f"path = {quote(relative(source))}"] if source else [f'version = {quote(dependency["req"])}']
        if not source:
            assert dependency["source"] == "registry+https://github.com/rust-lang/crates.io-index"
        if key != dependency["name"]:
            fields.append(f'package = {quote(dependency["name"])}')
        if dependency["features"]:
            fields.append(f'features = {quote(dependency["features"])}')
        if dependency["optional"]:
            fields.append("optional = true")
        if not dependency["uses_default_features"]:
            fields.append("default-features = false")
        groups[section].append(f'{key} = {{ {", ".join(fields)} }}')
    for section, dependencies in groups.items():
        lines.extend(("", f"[{section}]", *dependencies))
    if package["features"]:
        lines.extend(("", "[features]"))
        lines.extend(f"{name} = {quote(features)}" for name, features in package["features"].items())
    manifest = directory / "Cargo.toml"
    manifest.write_text("\n".join(lines) + "\n", encoding="utf-8")
    lock = directory / "Cargo.lock"
    if not lock.exists():
        probe_lock = generated / "clipboard-2d-probe" / "Cargo.lock"
        seed = probe_lock if probe_lock.is_file() else workspace.with_name("Cargo.lock")
        shutil.copyfile(seed, lock)
    print(manifest.relative_to(ticket).as_posix())
