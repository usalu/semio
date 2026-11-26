"""🧪️ Verify Draw's current full native catalog under one serial admission pipeline."""

import argparse
import os
from pathlib import Path
import re
import shlex
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
REPO = next(path for path in TICKET.parents if (path / "AGENTS.md").is_file())
OUTPUT = TICKET / "🗑️generated"
PROJECT = "@semio-tech/draw-drawing-rs"
EXPECTED = {
    "semio-s-artifact-draw-drawing": [
        "io_descriptor_tests::io_descriptors_match_neutral_fixture",
    ],
    "semio-s-artifact-draw-drawing-model-tests": [
        "io::sqlite::snapshot::tests::sqlite_snapshot_draw_actual_io_declaration_capability",
        "io::sqlite::snapshot::tests::sqlite_snapshot_draw_owned_complete_fields_both_native_encodings",
        "io::sqlite::snapshot::tests::sqlite_snapshot_draw_exact_foreign_dialect_and_initial_cancellation",
        "io::sqlite::snapshot::tests::sqlite_snapshot_draw_every_scalar_word_typed_file_and_erased_native_io",
    ],
    "semio-s-artifact-draw-drawing-editor-tests": [
        "window_ownership_tests::drawing_canvas_window_ownership_runtime_isolates_reloads_and_restores_exact_instances",
        "window_ownership_tests::drawing_canvas_window_ownership_matches_neutral_fixture_and_codecs",
        "window_ownership_tests::config_inverse_sums_to_the_negative_diff",
        "window_ownership_tests::set_mutation_wire_is_the_flat_tagged_record",
        "unit_tests::canvas_laws::renders_canvas_scene_with_segments",
        "unit_tests::canvas_laws::export_document_downloads_a_real_pdf_of_the_document",
    ],
    "semio-s-artifact-draw-drawing-editor-interaction-tests": [
        "panels::properties::tests::authored_geometry_controls_match_shared_inspector_contract",
    ],
    "semio-s-artifact-draw-drawing-viewer": [
        "modes::view::windows::canvas::tests::definition_declares_a_canvas2d_window",
    ],
    "semio-s-artifact-draw-drawing-geometry": [
        "tests::geometry_fixtures",
        "tests::path_editing_geometry_fixtures",
    ],
    "semio-s-artifact-draw-drawing-scene-mounted": [
        "tests::mounted_vector_geometry_drives_real_source_and_exact_complete_canvas",
    ],
}


def commands(memory, direct=False):
    prefix = ["repo", "capacity", "run", f"--memory={memory}", "--cpu=1", "--jobs=1", "--"]
    tests = ["long", "--no-fail-fast", "--status-level=pass"]
    if direct:
        script = "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/📜️script.ts"
        owner = [*prefix, "repo", "tools", "exec", "--light", "bun", "--", str(TICKET / "draw-native-dev.ts"), str(OUTPUT)]
        return [(phase, [*owner, "native-" + phase + "26", script, phase, *args]) for phase, args in (("test", tests), ("check", []))]
    return [
        ("test", [*prefix, "repo", "run", PROJECT + ":test", "--lang=en", "--parallel=1", "--", *tests]),
        ("check", [*prefix, "repo", "run", PROJECT + ":check", "--lang=en", "--parallel=1"]),
    ]


def verify_results(log, offset=0):
    passed = set()
    candidates = [(package, law) for package, laws in EXPECTED.items() for law in laws]
    with log.open("rb") as stream:
        stream.seek(offset)
        for row in stream:
            raw = row.decode("utf-8", errors="replace")
            line = re.sub(r"\x1b\[[0-?]*[ -/]*[@-~]", "", raw)
            if re.search(r"\bPASS\s+\[", line):
                for package, law in candidates:
                    if re.search(r"\b" + re.escape(package) + r"(?=\s|::)", line) and law in line:
                        passed.add((package, law))
    missing = sorted(set(candidates) - passed)
    for package, law in missing:
        print(f"Missing recorded PASS for {package} {law}", file=sys.stderr)
    if not missing:
        print(f"Verified {len(passed)} required native test receipts.")
    return not missing


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--memory", type=int, default=4608, help="single-job parent memory request in MiB")
    parser.add_argument("--direct-owner", action="store_true", help="invoke the registered Rust owner with the ticket script envelope")
    parser.add_argument("--run-native", action="store_true", help="run once the existing component pipeline is terminal")
    parser.add_argument("--verify-log", type=Path, help="verify named assertions in an already completed managed log")
    options = parser.parse_args()
    if options.verify_log:
        return 0 if verify_results(options.verify_log) else 1
    if options.memory < 1:
        parser.error("--memory must be positive")
    for phase, command in commands(options.memory, options.direct_owner):
        print(shlex.join(command), flush=True)
        if not options.run_native:
            continue
        environment = {**os.environ, "SEMIO_OFFLOAD": "never", "SEMIO_BUILD_MODE": "prototype"}
        log = OUTPUT / f"draw-native-current26-{phase}.log"
        owner_log = OUTPUT / f"draw-native-native-{phase}26.log" if options.direct_owner else log
        offset = owner_log.stat().st_size if options.direct_owner and owner_log.exists() else 0
        with log.open("w") as stream:
            result = subprocess.run(command, cwd=REPO, env=environment, stdout=stream, stderr=subprocess.STDOUT)
        print(f"{phase} exited {result.returncode}; {log}", flush=True)
        if result.returncode:
            return result.returncode
        if phase == "test" and not verify_results(owner_log, offset):
            return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
