"""🔎 Link the exact staged media owner against one coherent native lane."""

import json
import os
from pathlib import Path
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
STAGE = TICKET / "🗑️generated" / "artifact-output-queue"
TREE = STAGE / "q"
LIVE = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🎞️media")
CACHE = Path(sys.argv[1])
DIRECT = (
    "semio_framework", "semio_framework_job", "semio_framework_job_payload",
    "semio_framework_value", "semio_plugin_decl_views", "semio_plugin_ownership", "serde_json",
)


def linked():
    """📦 Select artifacts that the exact view and owner publications jointly require."""
    libraries = {path.stem.removeprefix("lib"): path for path in CACHE.glob("*/*/out/*.rlib")}
    anchors = {}
    for name in ["semio_plugin_decl_views", "semio_plugin_ownership", "semio_framework"]:
        candidates = [path for key, path in libraries.items() if key.rsplit("-", 1)[0] == name]
        if len(candidates) != 1:
            raise ValueError(f"{name} requires one actual native publication, found {len(candidates)}")
        anchors[name] = candidates[0]
    exact = {}
    for path in anchors.values():
        metadata = path.with_suffix(".rmeta.semio-deps")
        for key, witness in json.loads(metadata.read_text())["dependencies"]:
            name = key.rsplit("-", 1)[0]
            if name in DIRECT and key in libraries:
                if name in exact and exact[name] != (key, witness):
                    raise ValueError(f"Selected native publications disagree on {name}")
                exact[name] = key, witness
    selected = {}
    for name in DIRECT:
        if name in anchors:
            selected[name] = anchors[name]
        elif name in exact:
            key, witness = exact[name]
            seen = json.loads(libraries[key].with_suffix(".rmeta.semio-deps").read_text())["svh"]
            assert seen == witness, (seen, witness, name)
            selected[name] = libraries[key]
        else:
            candidates = [path for key, path in libraries.items() if key.rsplit("-", 1)[0] == name]
            assert len(candidates) == 1, (name, candidates)
            selected[name] = candidates[0]
    return selected


def stage_tree():
    """🧺 Place source and native test fixtures only under the ignored ticket directory."""
    changes = {Path(row["path"]).relative_to(LIVE): row["after"].encode() for row in json.loads((STAGE / "overlay.json").read_text())}
    paths = {path.relative_to(LIVE): path for path in LIVE.rglob("*") if path.is_file() and path.suffix in {".rs", ".json"} and "📦️packages" not in path.parts}
    for relative in paths.keys() | changes.keys():
        path = TREE / relative
        assert len(str(path).encode("utf-16-le")) // 2 <= 256, path
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(changes[relative] if relative in changes else paths[relative].read_bytes())


def run():
    """🧪 Run the same Original producer laws with real job and retirement artifacts."""
    selected = linked()
    stage_tree()
    target = STAGE / ("probe.exe" if os.name == "nt" else "probe")
    args = ["--edition=2021", "--test", "--crate-name", "semio_plugin_decl_media", str(TREE / "🦀️.rs"), "-o", str(target)]
    for name, library in selected.items():
        args.extend(("--extern", f"{name}={library.with_suffix('.rmeta')}", "--extern", f"{name}={library}"))
    for directory in sorted({path.parent for path in CACHE.glob("*/*/out/*") if path.suffix in {".rmeta", ".rlib", ".so", ".dll", ".dylib"}}):
        args.extend(("-L", f"dependency={directory}"))
    response = STAGE / "args"
    response.write_text("\n".join(args) + "\n")
    subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "@" + str(response)], check=True, env={**os.environ, "CARGO_MANIFEST_DIR": str(LIVE.resolve() / "📦️packages" / "🦀️rust")})
    subprocess.run([str(target), "--nocapture"], check=True)


if __name__ == "__main__":
    run()
