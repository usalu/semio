from pathlib import Path
import json
import os
import subprocess
import sys

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
variant = sys.argv[1]
package, artifact, tests = {
    "5d": ("semio-s-artifact-puzzle-5d-v1-unit-tests", "🖐️5d", [(None, "mirror::editor::puzzle5d::unit_tests::history_edits_end_to_end", True)]),
    "2d": ("semio-s-artifact-puzzle-2d-unit-tests", "◻️2d", [(None, "history_edits_end_to_end", False)]),
    "3d": ("semio-s-artifact-puzzle-3d-editor", "🧊️3d", [
        ("editor-gestures", "mutation_latency::one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns", True),
        ("editor-action-producer", "settlement_work_ledger_is_finite_and_progresses", True),
    ]),
}[variant]
targets = [arg for target, _, _ in tests for arg in (["--lib"] if target is None else ["--test", target])]
build = ["bash", str(ticket / "current-native-tests.sh"), artifact, "-p", package,
         *targets, "--no-run", "--message-format=json-render-diagnostics"]
executables = {}
with subprocess.Popen(build, cwd=root, stdout=subprocess.PIPE, text=True) as cargo:
    for line in cargo.stdout:
        try:
            message = json.loads(line)
        except json.JSONDecodeError:
            print(line, end="", flush=True)
            continue
        if message["reason"] == "compiler-message":
            print(message["message"].get("rendered") or "", end="", flush=True)
        if message["reason"] == "compiler-artifact" and message.get("executable") and message["profile"]["test"]:
            executables[message["target"]["name"]] = message["executable"], Path(message["manifest_path"]).parent
    exit_code = cargo.wait()
if exit_code:
    raise SystemExit(exit_code)
status = 0
for target, test, exact in tests:
    key = target or package.replace("-", "_")
    if key not in executables:
        raise SystemExit(f"build did not produce the native test executable for {key}")
    executable, manifest = executables[key]
    env = {**os.environ, "RUST_MIN_STACK": "134217728"}
    options = ["--exact"] if exact else []
    listed = subprocess.check_output([executable, test, "--list", *options], cwd=manifest, env=env, text=True)
    if not any(line.endswith(": test") for line in listed.splitlines()):
        raise SystemExit(f"native test executable does not contain {test}")
    print(f"[DEBUG] Running {test} after Cargo released the artifact lock", flush=True)
    result = subprocess.run(["timeout", "1800", executable, test, "--nocapture", "--test-threads=1", *options],
                            cwd=manifest, env=env, check=False)
    print(f"[DEBUG] {test} exited {result.returncode}", flush=True)
    status = status or result.returncode
raise SystemExit(status)
