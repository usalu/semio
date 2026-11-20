"""🧪️ Links the typed DSL audit against one completed component's WASI units."""

import argparse
import json
import os
import subprocess
from pathlib import Path


def unit_dependencies(unit, index, found):
    identity = unit["artifact"]
    if identity in found:
        return
    found[identity] = unit
    for dependency in unit["dependencies"]:
        for artifact in dependency["artifacts"]:
            if artifact in index:
                unit_dependencies(index[artifact], index, found)
                break


def completed_cohort(cache, family, run):
    units_path = cache / "components/runs" / run / "r/units.json"
    units = json.loads(units_path.read_text())["units"]
    indexed = {unit["artifact"]: unit for unit in units}
    model = next(unit for unit in units if unit["package"] == f"semio-s-artifact-norm-{family}-model" and unit["target"] == "wasm32-wasip2")
    needed = {}
    unit_dependencies(model, indexed, needed)
    pack = next(unit for unit in needed.values() if unit["crate"] == "pack")
    lookup = {
        path: sorted((cache / "u/l").glob("*/" + path.relative_to(cache / "u/l").as_posix().split("/", 1)[1]))
        for path in {Path(unit["artifact"]).parent for unit in needed.values()}
        if not path.is_dir()
    }
    directories = {}
    for unit in needed.values():
        path = Path(unit["artifact"]).parent
        if path.is_dir():
            directories[path] = path
        elif lookup[path]:
            directories[path] = lookup[path][0]
        else:
            raise FileNotFoundError(f"Cached unit absent: {path}")

    externs = {}
    for unit in (pack, model):
        artifact = Path(unit["artifact"])
        externs[unit["crate"]] = directories[artifact.parent] / f"lib{artifact.name}.rlib"
    return set(directories.values()), externs


def lane_cohort(cache, family, lane):
    root = cache / "u/l" / lane
    build = root / "wasm32-wasip2/prototype/build"
    model = f"semio-s-artifact-norm-{family}-model"
    externs = {}
    for package, alias in (("semio-framework-pack", "pack"), (model, model.replace("-", "_"))):
        libs = [path for path in (build / package).glob("*/out/*.rlib") if path.with_suffix(".rmeta").is_file()]
        if not libs:
            raise FileNotFoundError(f"No completed RLIB and metadata for {package} in lane {lane}")
        externs[alias] = max(libs, key=lambda path: path.stat().st_mtime)
    directories = set(build.glob("*/*/out")) | set((root / "prototype/build").glob("*/*/out"))
    return directories, externs


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("family", choices=("iso16757", "vdi3805", "din18599", "din4108", "din16798"))
    source = parser.add_mutually_exclusive_group(required=True)
    source.add_argument("--run", help="Directory name under compiler-store components/runs")
    source.add_argument("--lane", help="Directory name under the compiler store's u/l")
    parser.add_argument("--rustc", type=Path, required=True)
    parser.add_argument("--harness", help="An alternate Rust audit file in the ticket")
    arguments = parser.parse_args()
    ticket = Path(__file__).resolve().parent
    workspace = ticket.parents[6]
    cache = workspace / ".cache/b/r/m"
    directories, externs = (
        completed_cohort(cache, arguments.family, arguments.run)
        if arguments.run else lane_cohort(cache, arguments.family, arguments.lane)
    )
    harness = ticket / (arguments.harness or "audit-eurocode-assets.rs")
    stem = harness.stem if arguments.harness else f"norm-dsl-{arguments.family}"
    output = ticket / "🗑️generated" / f"{stem}.wasm"
    flags = [
        "--edition=2021", "--crate-name=norm_dsl_audit", "--target=wasm32-wasip2",
        "-Cpanic=abort", "-Copt-level=1", "-Cstrip=debuginfo",
        f"--cfg=feature=\"{arguments.family}\"", f"-o{output}", str(harness),
    ]
    flags.extend(f"-Ldependency={directory}" for directory in sorted(directories))
    for alias, artifact in externs.items():
        for suffix in (".rlib", ".rmeta"):
            library = artifact.with_suffix(suffix)
            if not library.is_file():
                raise FileNotFoundError(library)
            flags.extend(("--extern", f"{alias}={library}"))
    flags_path = output.with_suffix(".args")
    flags_path.write_text("\n".join(flags) + "\n")
    command = [os.fspath(arguments.rustc.resolve()), f"@{flags_path}"]
    subprocess.run(command, cwd=workspace, check=True)
    print(output)


if __name__ == "__main__":
    main()
