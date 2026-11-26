"""🧪️ Reuse the e25 provider graph for focused current-source Draw and DSL laws."""

import argparse
import json
import os
from pathlib import Path
import re
import subprocess
import sys


TICKET = Path(__file__).resolve().parent
REPO = next(parent for parent in TICKET.parents if (parent / "AGENTS.md").is_file())
GENERATED = TICKET / "🗑️generated"
OUTPUT = GENERATED / "focused-regressions"
CACHE = GENERATED / "e25/b/debug/build"
DSL = REPO / "🧰️framework/🔨️modules/🗣️dsl/🧬️schema"
RUSTC = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]
MUTATIONS = "semio_s_artifact_draw_drawing_mutations"
DRAW_METADATA = "semio_s_artifact_draw_drawing_metadata"
RECORD = "semio_framework_dsl_record"
FIELD_LAW = "optional_statements_match_json_and_release_each_cancelled_input"
DRAW_LAWS = {
    "binary::tests::original_operation_receives_addressed_rectangle",
    "binary::tests::original_operation_receives_empty_group",
    "binary::tests::original_operation_receives_nested_drawing_pages_from_the_declared_range",
    "binary::tests::original_operation_receives_renamed_layer_identifiers",
    "retained_inverse::tests::original_visible_inverse_receives_original_affected_value_and_closes_every_partial",
}


def cached_externs(alias, unit):
    name = unit.stem.removeprefix("lib-")
    artifacts = sorted(p for p in (unit.parents[1] / "out").glob(f"lib{name}-*")
                       if p.suffix in {".rlib", ".rmeta", ".dylib", ".so", ".dll"})
    if not artifacts:
        raise FileNotFoundError(f"No cached Rust artifact for {unit}")
    return [f"--extern={alias}={path}" for path in artifacts]


def captured_externs(args, name):
    return [arg for arg in args if arg.startswith(f"--extern={name}=")]


def replace_extern(args, name, artifact):
    args[:] = [arg for arg in args if not arg.startswith(f"--extern={name}=")]
    args.append(f"--extern={name}={artifact}")


def dsl_jobs(captured):
    by_digest = {unit.with_suffix("").read_text().strip(): unit
                 for unit in CACHE.glob("*/*/fingerprint/lib-*.json")
                 if unit.with_suffix("").is_file()}
    owners = {RECORD: DSL, RECORD + "_model": DSL / "📐️model",
              RECORD + "_ownership": DSL / "📐️model/🧳️ownership",
              RECORD + "_emission": DSL / "🖨️printing",
              "semio_framework_dsl_notation": DSL / "🖋️notation",
              RECORD + "_derive": DSL / "✨️derive"}
    jobs, outputs, inputs = [], {}, {}
    search_paths = [arg for arg in captured if arg.startswith("-Ldependency=")]

    def build(unit):
        name = unit.stem.removeprefix("lib-")
        if name in outputs:
            return outputs[name]
        directory = owners[name]
        macros = name.endswith("_derive")
        suffix = {"darwin": ".dylib", "win32": ".dll"}.get(sys.platform, ".so") if macros else ".rlib"
        artifact = OUTPUT / f"lib{name}{suffix}"
        fingerprint = json.loads(unit.read_text())
        args = [str(directory / "🦀️.rs"), "--edition=2021", "--crate-name", name,
                f"--crate-type={'proc-macro' if macros else 'rlib'}", "-Zthreads=1",
                "-Cdebuginfo=0", "-Cmetadata=focused_current_draw", "-o", str(artifact),
                f"-Ldependency={OUTPUT}", *search_paths]
        if macros:
            args += ["--extern", "proc_macro"]
        for feature in json.loads(fingerprint["features"]):
            args += ["--cfg", f'feature="{feature}"']
        for _, alias, _, digest in fingerprint["deps"]:
            if alias == "build_script_build":
                continue
            child = by_digest[digest.to_bytes(8, "little").hex()]
            child_name = child.stem.removeprefix("lib-")
            if child_name in owners:
                args.append(f"--extern={alias}={build(child)}")
            else:
                inputs[alias] = cached_externs(alias, child)
                args.extend(inputs[alias])
        jobs.append((name, args, directory / "📦️packages/🦀️rust", None))
        outputs[name] = artifact
        return artifact

    for name in (RECORD, RECORD + "_derive"):
        artifact = Path(captured_externs(captured, name)[0].split("=", 2)[-1])
        build(artifact.parents[1] / "fingerprint" / f"lib-{name}.json")

    name = "field_receiving"
    binary = OUTPUT / (name + (".exe" if os.name == "nt" else ""))
    args = [str(DSL / "🧪️tests/🧊️field-receiving/🦀️.rs"), "--test", "--edition=2021",
            "--crate-name", name, "-Zthreads=1", "-Cdebuginfo=0", "-o", str(binary),
            f"-Ldependency={OUTPUT}", *search_paths]
    for name in (RECORD, RECORD + "_derive"):
        args.append(f"--extern={name}={outputs[name]}")
    for name in ("serde", "serde_json", "semio_framework_test_fixtures"):
        args.extend(captured_externs(captured, name))
    for name in ("semio_framework_value", "semio_framework_value_ordered"):
        args.extend(inputs[name])
    for name, slot in (("semio_framework_value_testing_admission", "8ae7bb92ff719f3c"),
                       ("semio_framework_value_testing_allocator", "a8079d4cb7e437d9")):
        unit = CACHE / name.replace("_", "-") / slot / "fingerprint" / f"lib-{name}.json"
        args.extend(cached_externs(name, unit))
    jobs.append(("optional-statement-dsl", args, DSL / "📦️packages/🦀️rust", [binary, FIELD_LAW, "--exact"]))
    return jobs


def mutation_jobs(captured):
    metadata = (GENERATED / "metadata26" / f"{DRAW_METADATA}.args").read_text().splitlines()
    library = OUTPUT / f"lib{DRAW_METADATA}.rlib"
    metadata[metadata.index("-o") + 1] = str(library)
    binary = OUTPUT / (MUTATIONS + (".exe" if os.name == "nt" else ""))
    captured[captured.index("-o") + 1] = str(binary)
    replace_extern(captured, DRAW_METADATA, library)
    return [(DRAW_METADATA, metadata, Path(metadata[0]).parent / "📦️packages/🦀️rust", None),
            ("draw-mutations", captured, Path(captured[0]).parent / "📦️packages/🦀️rust", [binary])]


def run(jobs):
    status = 0
    for name, args, manifest, tests in jobs:
        environment = {**os.environ, "CARGO_MANIFEST_DIR": str(manifest), "RUST_MIN_STACK": "134217728"}
        response = OUTPUT / f"{name}.args"
        response.write_text("\n".join(args) + "\n")
        with (OUTPUT / f"{name}-compile.log").open("w") as log:
            compiled = subprocess.run([*RUSTC, f"@{response}"], cwd=REPO, env=environment, stdout=log, stderr=log)
        if compiled.returncode:
            print(f"{name} did not compile. See {log.name}", file=sys.stderr)
            return compiled.returncode
        if tests:
            listed = subprocess.run([*tests, "--list"], cwd=manifest, env=environment, text=True,
                                    capture_output=True, check=True)
            members = {line.removesuffix(": test") for line in listed.stdout.splitlines() if line.endswith(": test")}
            required = {FIELD_LAW} if name == "optional-statement-dsl" else DRAW_LAWS
            if absent := required - members:
                print(f"{name} did not enumerate {', '.join(sorted(absent))}", file=sys.stderr)
                return 1
            results = OUTPUT / f"{name}-results.log"
            with results.open("w") as log:
                result = subprocess.run([*tests, "--test-threads=1", "--nocapture"], cwd=manifest,
                                        env=environment, stdout=log, stderr=log)
            matched = re.search(r"^running (\d+) tests?\b", results.read_text(errors="replace"), re.MULTILINE)
            count = int(matched.group(1)) if matched else 0
            code = result.returncode or int(count != len(members))
            providers = "current record providers" if name == "optional-statement-dsl" else "cached transitive providers"
            print(f"{name}: {count} of {len(members)} tests, {providers}, exit {code}. See {results}")
            status = status or code
    return status


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("suite", choices=("dsl", "mutations", "all"), default="all", nargs="?")
    parser.add_argument("--run", action="store_true", help="compile and run when the local component pipeline is idle")
    options = parser.parse_args()
    captured = (GENERATED / "mutation-current26.args").read_text().splitlines()
    suites = []
    if options.suite != "mutations":
        suites.append(dsl_jobs(captured))
    if options.suite != "dsl":
        suites.append(mutation_jobs(captured))
    for name, args, manifest, tests in (job for jobs in suites for job in jobs):
        externs = [Path(arg.split("=", 2)[-1]) for arg in args if arg.startswith("--extern=")]
        externs.extend(Path(args[index + 1].split("=", 1)[-1]) for index, arg in enumerate(args)
                       if arg == "--extern" and "=" in args[index + 1])
        paths = [Path(args[0]), manifest / "Cargo.toml", *(path for path in externs if path.parent != OUTPUT)]
        for path in paths:
            if not path.is_file():
                raise FileNotFoundError(path)
        if not options.run:
            print(f"{name}: {'compile and run' if tests else 'compile'}")
    if not options.run:
        return 0

    OUTPUT.mkdir(exist_ok=True)
    results = [run(jobs) for jobs in suites]
    return next((status for status in results if status), 0)


if __name__ == "__main__":
    sys.exit(main())
