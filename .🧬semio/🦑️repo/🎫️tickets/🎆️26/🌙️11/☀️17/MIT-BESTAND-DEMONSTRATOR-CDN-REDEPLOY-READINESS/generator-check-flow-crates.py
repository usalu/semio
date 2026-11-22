"""🔬️ Typechecks current Flow crates against one exact retained WASI receipt without a module build."""
import json
import os
import subprocess
import sys
from pathlib import Path
from urllib.parse import unquote


def main():
    """🧾️ Resolve compiler-produced dependencies and materialize only diagnostic metadata."""
    ticket = Path(__file__).resolve().parent
    generated = ticket / "🗑️generated"
    output = generated / os.environ.get("GENERATOR_METADATA_SUBDIR", "")
    output.mkdir(exist_ok=True)
    receipt = Path(sys.argv[1])
    directory = Path(os.environ.get("GENERATOR_RETAINED_DEPS", generated / "generator-worker-deps"))
    units = json.loads((receipt / "unit-graph-release-00.json").read_text())["units"]
    flow = ("semio_os_flow_host_core", "semio_os_flow_host_read", "semio_os_flow_host_session", "semio_os_flow_host", "semio_os_flow")
    targets = sys.argv[2:] or flow
    replacements = {}
    for name in flow:
        artifact = output / f"lib{name}-generator-check.rmeta"
        if sys.argv[2:] and artifact.is_file():
            replacements[name] = artifact

    def resolve(index, rebuild=False):
        unit = units[index]
        target = unit["target"]
        name = target["name"]
        if name in replacements:
            return replacements[name]
        artifact = output / f"lib{name}-generator-check.rmeta"
        if sys.argv[2:] and not rebuild and artifact.is_file():
            return artifact
        macro = "proc-macro" in target["kind"]
        extension = "dylib" if macro else "rmeta"
        matches = [path for path in directory.glob(f"lib{name}-*.{extension}")
                   if path.is_file() and (macro or "wasm32-wasip2" in path.resolve().parts)]
        if len(matches) == 1 and not rebuild:
            return matches[0]
        if macro or unit["platform"] != "wasm32-wasip2" or len(matches) > 1:
            raise ValueError(f"Unable to resolve one coherent {name}: {matches}")
        package_url, _, package = unit["pkg_id"].partition("#")
        package_dir = unquote(package_url.removeprefix("path+file://"))
        env = {**os.environ, "CARGO_MANIFEST_DIR": package_dir, "CARGO_PKG_NAME": package.split("@")[0], "CARGO_PKG_VERSION": "0.1.0"}
        cmd = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", f"--edition={target['edition']}", "--crate-name", name, "--crate-type=rlib", "--target=wasm32-wasip2", "--emit=metadata", "-L", f"dependency={directory}", "-L", f"dependency={output}"]
        for feature in unit["features"]:
            cmd.extend(("--cfg", f'feature="{feature}"'))
        for dep in unit["dependencies"]:
            build_unit = units[dep["index"]]
            if "custom-build" in build_unit["target"]["kind"]:
                print("[DEBUG] Needs generated output", name, unit["pkg_id"], flush=True)
                continue
            dependency = resolve(dep["index"])
            cmd.extend(("--extern", f"{dep['extern_crate_name']}={dependency}"))
        subprocess.run([*cmd, "-C", "metadata=generator-check", "-o", str(artifact), target["src_path"]], check=True, env=env)
        replacements[name] = artifact
        print("[DEBUG] Typechecked", name, flush=True)
        return artifact

    for name in targets:
        index = next(index for index, unit in enumerate(units)
                     if unit["target"]["name"] == name and unit["platform"] == "wasm32-wasip2")
        resolve(index, rebuild=True)


if __name__ == "__main__":
    main()
