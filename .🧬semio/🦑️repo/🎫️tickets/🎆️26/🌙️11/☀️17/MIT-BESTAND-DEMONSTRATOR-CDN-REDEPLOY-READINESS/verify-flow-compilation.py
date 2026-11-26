"""🧪️ Check current Flow sources against one completed Procedural dependency capture."""
import json
import os
from pathlib import Path
import subprocess
import sys
from urllib.parse import unquote, urlparse

def main():
    """📜️ Forward exact released WASI dependencies, features and aliases to the pinned metadata compiler."""
    capture = Path(sys.argv[1]).resolve()
    output = Path(__file__).resolve().parent / "🗑️generated" / "flow-dependency-check"
    output.mkdir(exist_ok=True)
    graph = json.loads((capture / "unit-graph-release-00.json").read_text())
    units = graph["units"]
    artifacts = {}
    for line in (capture / "cargo-release-00.jsonl").open():
        try:
            event = json.loads(line)
        except ValueError:
            continue
        if event.get("reason") != "compiler-artifact":
            continue
        target = event["target"]
        candidates = [Path(name) for name in event["filenames"] if Path(name).suffix in (".rlib", ".rmeta", ".dylib", ".so", ".dll")]
        if not candidates:
            continue
        platform = "wasm32-wasip2" if "wasm32-wasip2" in candidates[0].parts else None
        key = (event["package_id"], target["name"], platform)
        artifacts[key] = candidates
        for artifact in candidates:
            if artifact.is_file() and not (output / artifact.name).is_symlink():
                (output / artifact.name).symlink_to(artifact)
    for name in sys.argv[2:]:
        unit = next(unit for unit in units if unit["target"]["name"] == name and unit["platform"] == "wasm32-wasip2")
        target = unit["target"]
        source = Path(target["src_path"])
        manifest = Path(unquote(urlparse(unit["pkg_id"]).path))
        env = dict(os.environ, CARGO_MANIFEST_DIR=str(manifest), CARGO_PKG_NAME=name.replace("_", "-"), CARGO_PKG_VERSION="0.1.0")
        command = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc",
                   f"--edition={target['edition']}", f"--crate-name={name}", "--crate-type=rlib", "--target=wasm32-wasip2",
                   "--emit=metadata", "-L", f"dependency={output}", "-C", "metadata=generator-release-check"]
        for dependency in unit["dependencies"]:
            dep = units[dependency["index"]]
            key = (dep["pkg_id"], dep["target"]["name"], dep["platform"])
            for artifact in artifacts[key]:
                if artifact.suffix == ".rlib":
                    continue
                command += ["--extern", f"{dependency['extern_crate_name']}={output / artifact.name}"]
        for feature in unit["features"]:
            command += ["--cfg", f'feature="{feature}"']
        subprocess.run([*command, "-o", str(output / f"lib{name}-generator-release-check.rmeta"), source], env=env, check=True)
        print(f"[DEBUG] metadata passed {name}", flush=True)

if __name__ == "__main__":
    main()
