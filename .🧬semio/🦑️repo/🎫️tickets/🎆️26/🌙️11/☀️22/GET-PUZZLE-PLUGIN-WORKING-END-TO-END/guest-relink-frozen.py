"""🔗️ Relink the unchanged Puzzle entry point using the exact old Cargo library closure."""

from collections import defaultdict
from json import loads
from os import environ, link
from pathlib import Path
from subprocess import run
from sys import argv


ticket = Path(__file__).resolve().parent
workspace = ticket.parents[6]
generated = ticket / "🗑️generated"
old = generated / "builder/lane-wasip2/b"
wasip2 = old / "wasm32-wasip2/prototype/build"
host = old / "prototype/build"
root = wasip2 / "semio-hub-puzzle-component/a9c05e62fe0c6811/fingerprint/lib-semio_hub_puzzle_component.json"
scratch = generated / "guest-relink"
archives = scratch / "deps"
archives.mkdir(parents=True, exist_ok=True)


def retain(source, target):
    if target.exists():
        assert source.samefile(target), target
    else:
        link(source.resolve(), target)


index = defaultdict(list)
for directory in (wasip2, host):
    for fingerprint in directory.glob("*/*/fingerprint/*.json"):
        metadata = fingerprint.with_suffix("")
        if metadata.is_file():
            digest = metadata.read_text().strip()
            if len(digest) == 16:
                index[digest].append(fingerprint)


def key(number):
    return int(number).to_bytes(8, "little").hex()


selected, unavailable = {}, set()
queue = list(loads(root.read_text())["deps"])
while queue:
    _, name, _, digest = queue.pop()
    digest = key(digest)
    if digest in selected or (name, digest) in unavailable:
        continue
    hits = index[digest]
    if not hits and (name, digest) == ("semio_framework_test_fixtures", "86b12219f8952c8e"):
        unit = wasip2 / "semio-framework-test-fixtures/f90dcbf7e79aa478"
        candidate = unit / "fingerprint/lib-semio_framework_test_fixtures.json"
        fixture_artifacts = list((unit / "out").glob("libsemio_framework_test_fixtures-*"))
        frozen_time = (generated / "guest-frozen-component.wasm").stat().st_mtime_ns
        if {artifact.suffix for artifact in fixture_artifacts} == {".rmeta", ".rlib"} and all(path.stat().st_mtime_ns < frozen_time for path in [candidate, *fixture_artifacts]):
            hits = [candidate]
    if len(hits) != 1:
        unavailable.add((name, digest))
        continue
    selection = hits[0]
    selected[digest] = selection
    if selection.name.startswith("lib-"):
        queue.extend(loads(selection.read_text())["deps"])

assert not unavailable, f"Exact archived dependency fingerprints missing or ambiguous: {sorted(unavailable)}"

native_paths = []
for fingerprint in selected.values():
    unit = fingerprint.parents[1]
    if fingerprint.name.startswith("run-build-script-"):
        output = unit / "output"
        if output.is_file():
            for line in output.read_text().splitlines():
                if line.startswith(("cargo:rustc-link-search=", "cargo::rustc-link-search=")):
                    value = line.split("=", 1)[1]
                    native_paths += ["-L", value if "=" in value else "native=" + value]
    if fingerprint.name.startswith("lib-"):
        for artifact in (unit / "out").glob("*"):
            if artifact.suffix in {".rlib", ".rmeta", ".so"}:
                retain(artifact, archives / artifact.name)

externs = []
for _, name, _, digest in loads(root.read_text())["deps"]:
    dependency = selected[key(digest)]
    stem = dependency.stem.removeprefix("lib-")
    matches = list(archives.glob(f"lib{stem}-{dependency.parents[1].name}.rmeta"))
    assert len(matches) == 1, name
    externs += ["--extern", f"{name}={matches[0]}"]

print(f"[DEBUG] Retained {len(selected)} exact Cargo unit fingerprints and {len(list(archives.iterdir()))} artifacts", flush=True)
hub = workspace / "🌎️hub"
component = hub / "🧩️compositions/🧩️puzzle/🧩️component"
manifest = component / "📦️packages/🦀️rust"
rustc = workspace / ".cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin/rustc"
environment = dict(environ, CARGO_MANIFEST_DIR=str(manifest), CARGO_MANIFEST_PATH=str(manifest / "Cargo.toml"), CARGO_PKG_NAME="semio-hub-puzzle-component", CARGO_PKG_VERSION="0.1.0", RUST_MIN_STACK="134217728", TMPDIR=str(generated / "guest-tmp"))
output = scratch / "semio_hub_puzzle_component.wasm"
mode = "metadata" if "--metadata-only" in argv else "link"
if mode == "metadata":
    output = scratch / "semio_hub_puzzle_component.rmeta"
flags = ["-Z", "threads=1", "-C", "linker-features=+lld", "-C", "link-self-contained=+linker", "-Z", "unstable-options", "-C", "opt-level=s", "-C", "embed-bitcode=no", "-C", "strip=symbols", "-Zembed-metadata=no", "-C", "link-arg=--max-memory=536870912", "-C", "link-arg=-zstack-size=8388608"]
args = [str(rustc), "--crate-name", "semio_hub_puzzle_component", "--edition=2021", str(component / "🦀️.rs"), "--crate-type", "cdylib", f"--emit={mode}", "--target", "wasm32-wasip2", "-o", str(output), "-L", f"dependency={archives}", *native_paths, *externs, *flags]
raise SystemExit(run(args, cwd=hub, env=environment).returncode)
