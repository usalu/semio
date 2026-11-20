"""🧪️ Link exact authored scene and gallery checks against one published Cargo unit cohort."""

import argparse
import json
import subprocess
from pathlib import Path


def unit_key(package, name, features, wasi):
    return package, name, tuple(sorted(features)), wasi


def build_structural_sources(units, artifacts, rustc, generated, dependencies, workspace, families):
    output = generated / "structural-scene-libs"
    output.mkdir(exist_ok=True)
    dependencies.append(f"-Ldependency={output}")
    logs = output / "artifacts.jsonl"
    events = []

    def build(index):
        unit = units[index]
        target = unit["target"]
        name = target["name"]
        key = unit_key(unit["pkg_id"], name, unit["features"], unit["platform"] is not None)
        if key in artifacts:
            return artifacts[key]
        if unit["platform"] != "wasm32-wasip2" or not name.startswith(tuple(f"semio_s_artifact_norm_en{family}_" for family in families)):
            raise ValueError(f"{name} is missing from the selected prebuilt dependency cohort")
        sources = [(dep["extern_crate_name"], build(dep["index"])) for dep in unit["dependencies"]]
        externs = [flag for alias, files in sources for path in files for flag in ("--extern", f"{alias}={path}")]
        cfgs = [flag for feature in unit["features"] for flag in ("--cfg", f'feature="{feature}"')]
        names = [output / f"lib{name}.{extension}" for extension in ("rlib", "rmeta")]
        options = [
            "--edition=2021", "--target=wasm32-wasip2", "-Cpanic=abort", "-Copt-level=0", "--crate-type=rlib",
            f"--crate-name={name}", f"--emit=link={names[0]},metadata={names[1]}", target["src_path"], *dependencies, *cfgs, *externs,
        ]
        response = output / f"{name}.args"
        response.write_text("\n".join(options) + "\n")
        print(f"Compiling {name}", flush=True)
        with (output / f"{name}.log").open("w") as log:
            subprocess.run([str(rustc), f"@{response}"], cwd=workspace, stdout=log, stderr=log, check=True)
        artifacts[key] = tuple(names)
        events.append({"reason": "compiler-artifact", "package_id": unit["pkg_id"], "target": target, "features": unit["features"], "filenames": [str(path) for path in names]})
        return artifacts[key]

    for family in families:
        for part in ("model", "inferences", "scene"):
            name = f"semio_s_artifact_norm_en{family}_{part}"
            (index,) = [i for i, unit in enumerate(units) if unit["target"]["name"] == name and unit["platform"] == "wasm32-wasip2"]
            build(index)
    logs.write_text("".join(json.dumps(event, ensure_ascii=False) + "\n" for event in events))
    return events


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("group", choices=("initial", "structural"))
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--lane", type=Path, required=True)
    parser.add_argument("--rustc", type=Path, required=True)
    parser.add_argument("--structural-graph", type=Path)
    parser.add_argument("--include-en1994", action="store_true")
    args = parser.parse_args()
    ticket = Path(__file__).resolve().parent
    workspace = ticket.parents[6]
    lane = args.lane.resolve()
    generated = ticket / "🗑️generated"
    output = generated / f"relinked-eurocode-{args.group}.wasm"
    rustc = args.rustc.resolve()
    dependencies = [f"-Ldependency={path}" for directory in (lane / "prototype/build", lane / "wasm32-wasip2/prototype/build") for path in sorted(directory.glob("*/*/out"))]

    by_name = {}
    by_unit = {}
    for line in args.artifacts.read_text().splitlines():
        event = json.loads(line)
        if event.get("reason") != "compiler-artifact":
            continue
        paths = tuple(Path(path) for path in event["filenames"] if Path(path).suffix in (".rlib", ".rmeta", ".dylib", ".so", ".dll"))
        if not paths or not all(path.is_file() for path in paths):
            continue
        wasi = paths[0].is_relative_to(lane / "wasm32-wasip2")
        key = unit_key(event["package_id"], event["target"]["name"], event["features"], wasi)
        by_unit[key] = paths
        if wasi:
            by_name.setdefault(event["target"]["name"], set()).add(paths)
    if args.structural_graph:
        if args.group != "structural":
            parser.error("--structural-graph requires the structural group")
        units = json.loads(args.structural_graph.read_text())["units"]
        families = (1992, 1993, 1994) if args.include_en1994 else (1992, 1993)
        for event in build_structural_sources(units, by_unit, rustc, generated, dependencies, workspace, families):
            by_name.setdefault(event["target"]["name"], set()).add(tuple(map(Path, event["filenames"])))

    options = ["--edition=2021", "--target=wasm32-wasip2", "-Cpanic=abort", "-Copt-level=1", "-Cstrip=debuginfo", "-o", str(output)]
    for feature in ("retained-json", "window-admission", "gallery-json"):
        options.extend(("--cfg", f'feature="{feature}"'))
    if args.group == "structural":
        options.extend(("--cfg", 'feature="structural"', "--cfg", 'feature="structural-only"'))
    else:
        options.extend(("--cfg", 'feature="palette-window"'))
    options.extend(dependencies)
    options.append(str(ticket / "audit-eurocode-scenes.rs"))

    externs = {name: name for name in (
        "semio_framework", "semio_framework_value", "semio_plugin", "semio_ui_contract", "semio_ui_contract_values", "semio_ui_locale", "semio_ui_scene", "serde_json",
        "semio_s_plugin_norm_app_surface_panels", "semio_s_plugin_norm_app_surface_visual_chrome", "semio_s_plugin_norm_structural_scene",
    )}
    externs["pack"] = "pack"
    families = (1990, 1991) if args.group == "initial" else (1992, 1993)
    for family in families:
        for part in ("model", "scene", "inferences"):
            crate = f"semio_s_artifact_norm_en{family}_{part}"
            externs[crate] = crate
    if args.group == "initial":
        crate = "semio_s_artifact_norm_en1990_scene_diagrams"
        externs[crate] = crate
    for alias, crate in externs.items():
        variants = by_name.get(crate, set())
        if len(variants) != 1:
            raise ValueError(f"{crate} must have one compiled WASI unit in the selected cohort, found {len(variants)}")
        (paths,) = variants
        if {path.suffix for path in paths} != {".rlib", ".rmeta"} or not all(path.is_file() for path in paths):
            raise FileNotFoundError(f"{crate} is missing its link archive or complete compiler metadata")
        for path in paths:
            options.extend(("--extern", f"{alias}={path}"))

    response = output.with_suffix(".args")
    response.write_text("\n".join(options) + "\n")
    subprocess.run([str(rustc), f"@{response}"], cwd=workspace, check=True)
    print(output)


if __name__ == "__main__":
    main()
