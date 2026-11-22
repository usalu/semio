"""🌐️ Activate and verify the newly published DIN 4108 browser lane."""
import json
import os
from pathlib import Path
import subprocess
import sys
import time
import urllib.request

root = Path.cwd()
here = Path(__file__).resolve().parent
output = here / "🗑️generated"
log = Path(sys.argv[1])
previous = sys.argv[2] if len(sys.argv) > 2 else ""
module = root / ".cache/b/p/m/browser/prototype/modules/📕️norm"
receipt = module / "🧾️receipt.json"
port = os.environ.get("SEMIO_TECH_PLAY_PORT", "6035")
preview_core = os.environ.get("PLAY_TEST_GUEST")
preview_bindings = os.environ.get("PLAY_TEST_GUEST_JS")
if bool(preview_core) != bool(preview_bindings):
    raise SystemExit("The diagnostic preview needs both the linked core and its bindings")
prefix = "norm-din4108-linked" if preview_core else "norm-din4108-scoped"
env = dict(os.environ, SEMIO_TECH_PLAY_SCOPE="din4108", SEMIO_TECH_PLAY_PORT=port,
           ONLY_LANES="din4108", PLAYWRIGHT_BASE_URL=f"http://127.0.0.1:{port}",
           PLAYWRIGHT_BROWSERS_PATH=str(root / ".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"))
play = root / "🏢️semio-tech/🎡️play"

def run(args, name, *, cwd=root, environment=env):
    """🏃️ Run a named diagnostic against the selected lane and retain its output."""
    print(f"[DEBUG] Running {name}", flush=True)
    with (output / f"{name}.log").open("w") as capture:
        subprocess.run(args, cwd=cwd, env=environment, stdout=capture, stderr=subprocess.STDOUT, check=True)

until = time.monotonic() + int(os.environ.get("NORM_PUBLICATION_WAIT_SECONDS", "10800"))
while time.monotonic() < until:
    if receipt.is_file() and any(module.glob("*.core.wasm")):
        try:
            current = json.loads(receipt.read_text())
        except json.JSONDecodeError:
            time.sleep(5)
            continue
        if preview_core and Path(preview_core).is_file() and Path(preview_bindings).is_file():
            print("[DEBUG] The locally linked diagnostic guest is ready", flush=True)
            break
        if not preview_core and current.get("pluginId") == "norm" and current.get("wasmSha256") != previous:
            print(f"[DEBUG] Canonical norm receipt present for {current['wasmSha256']}", flush=True)
            break
    if not preview_core and log.is_file() and "Native build failed:" in log.read_text(errors="replace")[-32768:]:
        raise SystemExit("DIN 4108 build failed before publication")
    time.sleep(8)
else:
    raise SystemExit("DIN 4108 did not publish during the build window")

producer = root / "🧰️framework/🛍️products/💻️os/🔨️modules"
driver = str(here/"native-exec.ts")
env["SEMIO_PLUGIN_CATALOG"] = str(root/"✏️s/🧑‍💻dev/🧩️catalog/🤖️generated")
registry = str(producer/"🔌️plugin/📇️registry")
dev = str(producer/"🧑‍💻dev/📦️packages/🟦️typescript")
for owner, step, tail in [(registry,"session",[]),(dev,"prepare",["react","prototype"]),(dev,"activate",["react","prototype"])]:
    run(["repo",driver,owner,"./📜️script.ts",step,"din4108",*tail], f"{prefix}-{step}")
with (output/f"{prefix}-server.log").open("w") as capture:
    server = subprocess.Popen(["repo", str(here/"serve-play-native.ts")], cwd=root,
                              env=env, stdin=subprocess.DEVNULL, stdout=capture, stderr=subprocess.STDOUT)
    try:
        for attempt in range(60):
            if server.poll() is not None:
                raise SystemExit("Scoped Play server exited before serving")
            try:
                with urllib.request.urlopen(env["PLAYWRIGHT_BASE_URL"], timeout=2) as response:
                    if response.status == 200:
                        break
            except OSError:
                time.sleep(2)
        else:
            raise SystemExit("Scoped Play server did not serve")
        if not preview_core:
            run(["repo", "tools", "exec", "--light", "bun", "--", str(here/"verify-play-components.mjs"), "delivery", "din4108"], f"{prefix}-delivery")
        probe = ["repo", "tools", "exec", "--light", "bun", "--", str(here/"probe-archive-play.mjs"), env["PLAYWRIGHT_BASE_URL"] + "/#din4108", "300"]
        run(probe, f"{prefix}-archive", cwd=play)
        run(probe, f"{prefix}-picker", cwd=play, environment=dict(env, VERIFY_EXAMPLE_WORKFLOW="1"))
    finally:
        if server.poll() is None:
            server.terminate()
