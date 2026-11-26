from pathlib import Path
import fcntl
import json
import os
import subprocess
import sys
import time

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
generated = ticket / "🗑️generated"
locks = generated / "current-native/t/debug"
package = "semio-s-artifact-puzzle-2d-history-edit-runtime-tests"
options = ["--config", "profile.test.package.semio-s-artifact-stdio-pdf-model.codegen-units=64"]
build = ["bash", str(ticket / "current-native-tests.sh"), "◻️2d", *options, "-p", package, "--lib", "--no-run", "--message-format=json-render-diagnostics"]
env = {**os.environ, "SEMIO_NATIVE_CARGO_JOBS": "1", "RUST_MIN_STACK":"134217728"}
for attempt in range(1, 5):
    with (locks / ".cargo-artifact-lock").open("r+") as gate:
        fcntl.flock(gate, fcntl.LOCK_SH)
    binary = None
    print("Native 2D package build attempt", attempt, flush=True)
    with subprocess.Popen(build, cwd=root, env=env, stdout=subprocess.PIPE, text=True) as cargo:
        for line in cargo.stdout:
            try:
                event = json.loads(line)
            except ValueError:
                print(line, end="", flush=True)
                continue
            if event.get("executable") and event["target"]["name"] == package.replace("-", "_"):
                binary = Path(event["executable"])
                project = Path(event["manifest_path"]).parent
        status = cargo.wait()
    if status == 0 and binary and binary.is_file():
        print("Running all registered 2D fill and history cases", binary, flush=True)
        raise SystemExit(subprocess.run([str(binary), "--nocapture", "--test-threads=2"], cwd=project, env=env).returncode)
    time.sleep(8)
raise SystemExit("Fresh 2D native package did not link within four rebuilds.")
