"""🎡️ Runs the official browser checks after all five Norm modules publish."""
import json
import os
from pathlib import Path
import subprocess
import sys
import time
import urllib.request

root = Path.cwd()
ticket = Path(__file__).resolve().parent
output = ticket/"🗑️generated"
play = root/"🏢️semio-tech/🎡️play"
producers = root/"🧰️framework/🛍️products/💻️os/🔨️modules"
modules = root/".cache/b/p/m/browser/prototype/modules"
catalogue = root/"✏️s/🧑‍💻dev/🧩️catalog/🤖️generated"
required = {"norm", "norm-eurocode", "norm-eurocode-structural", "norm-eurocode-materials", "norm-energy"}
env = dict(os.environ, SEMIO_TECH_PLAY_SCOPE="standards", SEMIO_TECH_PLAY_PORT="6033",
           PLAYWRIGHT_BASE_URL="http://127.0.0.1:6033", SEMIO_PLUGIN_CATALOG=str(catalogue),
           PLAYWRIGHT_BROWSERS_PATH=str(root/".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"))

def run(command, name, cwd=root):
    """🏃️ Record one verification or activation under its ticket."""
    print(f"[DEBUG] {name}", flush=True)
    with (output/f"{name}.log").open("w") as f:
        return subprocess.run(command, cwd=cwd, env=env, stdout=f, stderr=subprocess.STDOUT).returncode

def probe(script, name, *args, cwd=root):
    """🌐️ Run a browser fixture against the production Standards host."""
    return run(["repo","tools","exec","--light","bun","--",str(ticket/script),*args],name,cwd)

def activate(variant):
    """🔑️ Publish the native session, prepared lane and activation."""
    driver = str(ticket/"native-exec.ts")
    registry = str(producers/"🔌️plugin/📇️registry")
    dev = str(producers/"🧑‍💻dev/📦️packages/🟦️typescript")
    for owner, step, tail in [(registry,"session",[]),(dev,"prepare",["react","prototype"]),(dev,"activate",["react","prototype"])]:
        if run(["repo",driver,owner,"./📜️script.ts",step,variant,*tail],f"standards-{step}-{variant}"):
            return False
    return True

superseded_norm_sha = os.environ.get("NORM_SUPERSEDED_WRAPPER_SHA")
deadline = time.monotonic() + int(os.environ.get("NORM_PUBLICATION_WAIT_SECONDS", "10800"))
previous = set()
while time.monotonic()<deadline:
    ready = set()
    for p in modules.glob("*/🧾️receipt.json"):
        try:
            data = json.loads(p.read_text())
        except (FileNotFoundError, json.JSONDecodeError):
            continue
        plugin = data.get("pluginId")
        if plugin == "norm" and data.get("wasmSha256") == superseded_norm_sha:
            continue
        if plugin in required and data.get("wasmSha256") and any(p.parent.glob("*.core.wasm")):
            ready.add(plugin)
    if ready != previous:
        print(f"[DEBUG] Published Norm modules {', '.join(sorted(ready))}",flush=True)
        previous = ready
    if ready == required:
        break
    time.sleep(12)
else:
    raise SystemExit("Missing signed Norm modules " + ", ".join(sorted(required-previous)))

activations = [activate(v) for v in ("din4108","din18599","en1990","en1992","en1995")]
if not all(activations):
    raise SystemExit("A Standards lane failed activation")

with (output/"standards-live-server.log").open("w") as capture:
    server = subprocess.Popen(["repo",str(ticket/"serve-play-native.ts")],env=env,cwd=root,
                              stdin=subprocess.DEVNULL,stdout=capture,stderr=subprocess.STDOUT,start_new_session=os.name=="posix")
try:
    for _ in range(90):
        if server.poll() is not None:
            raise SystemExit("The Standards server exited before serving")
        try:
            with urllib.request.urlopen(env["PLAYWRIGHT_BASE_URL"],timeout=2) as response:
                if response.status == 200:
                    break
        except OSError:
            time.sleep(2)
    else:
        raise SystemExit("The Standards server never responded")

    if probe("verify-play-components.mjs","standards-signed-delivery","delivery"):
        raise SystemExit("Signed delivery failed")
    failures = [
        probe("probe-archive-play.mjs","standards-din4108-archive",env["PLAYWRIGHT_BASE_URL"]+"/#din4108","300",cwd=play),
        probe("probe-eurocode-viewports.mjs","standards-en1990-en1992-capacity","en1990","en1992"),
        probe("verify-play-components.mjs","standards-all-browser-workflows","browser"),
        probe("probe-norm-catalogue-examples.mjs","standards-catalogue-examples",cwd=play),
    ]
    if any(failures):
        raise SystemExit("One or more browser checks failed")
    print("[DEBUG] All Standards browser verification completed",flush=True)
finally:
    if server.poll() is None:
        print(f"[DEBUG] Standards server remains available at {env['PLAYWRIGHT_BASE_URL']}, pid {server.pid}",flush=True)
