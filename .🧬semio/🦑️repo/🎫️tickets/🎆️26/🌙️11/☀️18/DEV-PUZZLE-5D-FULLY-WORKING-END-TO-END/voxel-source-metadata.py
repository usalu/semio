import json
import os
from pathlib import Path
import subprocess
import sys


def dependencies(build: Path, unit: Path):
    root = unit / "fingerprint"
    leaf = next(root.glob("test-lib-*.json"))
    index = {}
    for fingerprint in build.glob("*/*/fingerprint/lib-*.json"):
        marker = fingerprint.with_suffix("")
        if marker.is_file():
            raw = marker.read_text().strip()
            if len(raw) == 16:
                index[int.from_bytes(bytes.fromhex(raw), "little")] = fingerprint
    seen = set()
    pending = [leaf]
    publication = None
    while pending:
        current = pending.pop()
        if current in seen:
            continue
        seen.add(current)
        if current.name == "lib-semio_os_store_one_item_publication.json":
            publication = current
            break
        data = json.loads(current.read_text())
        pending.extend(index[key] for _, _, _, key in data["deps"] if key in index)
    if publication is None:
        raise ValueError("native Puzzle5D fingerprint has no publication dependency")
    return publication, index


def compile_arguments(publication: Path, index: dict, manifest: Path, output: Path):
    data = json.loads(publication.read_text())
    source = manifest.parent.parent.parent / "🦀️.rs"
    arguments = [
        "--edition=2021",
        "--crate-name=semio_os_store_one_item_publication",
        "--crate-type=lib",
        "--emit=metadata=" + str(output / "voxel-source.rmeta"),
        str(source),
    ]
    for feature in json.loads(data["features"]):
        arguments.append(f'--cfg=feature="{feature}"')
    pending = [index[key] for _, _, _, key in data["deps"] if key in index]
    seen = set()
    while pending:
        current = pending.pop()
        if current in seen:
            continue
        seen.add(current)
        arguments.append("-Ldependency=" + str(current.parents[1] / "out"))
        node = json.loads(current.read_text())
        pending.extend(index[key] for _, _, _, key in node["deps"] if key in index)
    for _, alias, _, key in data["deps"]:
        dependency = index[key]
        out = dependency.parents[1] / "out"
        crate = dependency.stem.removeprefix("lib-")
        artifacts = sorted(path for path in out.glob(f"lib{crate}-*") if path.suffix in (".rmeta", ".rlib", ".so", ".dylib", ".dll"))
        metadata = next((path for path in artifacts if path.suffix == ".rmeta"), None)
        if metadata is None:
            metadata = next(path for path in artifacts if path.suffix in (".so", ".dylib", ".dll", ".rlib"))
        arguments.append(f"--extern={alias}={metadata}")
    return arguments


def main():
    binary = Path(sys.argv[1]).resolve()
    manifest = Path(sys.argv[2]).resolve()
    output = Path(sys.argv[3]).resolve()
    root = binary.parents[4] / "build"
    publication, index = dependencies(root, binary.parents[1])
    print(f"[DEBUG] Publication fingerprint {publication}", flush=True)
    arguments = compile_arguments(publication, index, manifest, output)
    response = output / "voxel-source-check.args"
    response.write_text("\n".join(arguments) + "\n")
    if sys.argv[4:] == ["--arguments-only"]:
        return
    env = os.environ.copy()
    env["CARGO_MANIFEST_DIR"] = str(manifest.parent)
    command = ["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "@" + str(response)]
    subprocess.run(command, env=env, check=True)


if __name__ == "__main__":
    main()
