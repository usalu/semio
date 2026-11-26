"""🧾️ Check whether the completed WASI archives still fit the recorded hub compiler invocation."""

from os import environ, link
from pathlib import Path
from shlex import split
from subprocess import run

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
workspace = ticket.parents[6]
snapshot = generated / "wasm-archive-check"
snapshot.mkdir()
output = snapshot / "out"
output.mkdir()

failure = (generated / "guest-component-current7.log").read_text().splitlines()[-1]
assert failure.startswith("  process didn't exit successfully: `")
arguments = split(failure.split("`")[1])
assert arguments[1:4] == ["--crate-name", "semio_hub_puzzle", "--edition=2021"]
assert arguments[arguments.index("--target") + 1] == "wasm32-wasip2"
inputs = {}


def retain(original):
    if original not in inputs:
        selected = snapshot / str(len(inputs))
        selected.mkdir()
        inputs[original] = selected
        if original.is_dir():
            for path in original.iterdir():
                if path.is_file() and path.suffix in {".a", ".rmeta", ".rlib", ".so", ".dylib"}:
                    try:
                        link(path.resolve(), selected / path.name)
                    except FileNotFoundError:
                        pass
    return inputs[original]


for index, argument in enumerate(arguments):
    if argument == "-L":
        kind, path = arguments[index + 1].split("=", 1)
        arguments[index + 1] = f"{kind}={retain(Path(path))}"
    elif argument == "--extern":
        crate, path = arguments[index + 1].split("=", 1)
        source = Path(path)
        local = retain(source.parent) / source.name
        assert local.is_file(), crate
        arguments[index + 1] = f"{crate}={local}"
    elif argument == "--out-dir":
        arguments[index + 1] = str(output)
    elif argument.startswith("--emit="):
        arguments[index] = "--emit=metadata"

hub = workspace / "🌎️hub"
manifest = hub / "🧩️compositions/🧩️puzzle/📦️packages/🦀️rust"
assert (hub / arguments[4]).is_file()
environment = dict(environ, CARGO_MANIFEST_DIR=str(manifest), CARGO_MANIFEST_PATH=str(manifest / "Cargo.toml"), CARGO_PKG_NAME="semio-hub-puzzle", CARGO_PKG_VERSION="0.1.0", RUST_MIN_STACK="134217728")
print(f"[DEBUG] Checking {len(inputs)} already built input directories", flush=True)
raise SystemExit(run(arguments, cwd=hub, env=environment).returncode)
