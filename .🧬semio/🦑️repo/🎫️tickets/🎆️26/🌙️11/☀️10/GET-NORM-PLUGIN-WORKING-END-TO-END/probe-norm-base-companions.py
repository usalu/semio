"""🌐️ Probes the independent Norm routes while the DIN 4108 panel rebuilds."""
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
port = os.environ.get("SEMIO_TECH_PLAY_PORT","6036")
env = dict(os.environ, SEMIO_TECH_PLAY_PORT=port, PLAYWRIGHT_BASE_URL=f"http://127.0.0.1:{port}",
           PLAYWRIGHT_BROWSERS_PATH=str(root/".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"))
def browser(script, name, scope, workspace, *arguments):
    """🧪️ Run the route or catalogue workflow on the scoped Play service."""
    with (output/f"{name}-{scope}.log").open("w") as f:
        return subprocess.run(["repo","tools","exec","--light","bun","--",str(ticket/script),*arguments,scope],
                              cwd=workspace,env=env,stdout=f,stderr=subprocess.STDOUT).returncode

def wait_for_module(plugin_id):
    """🧾️ Wait for the requested signed browser component and its core."""
    modules = root / ".cache/b/p/m/browser/prototype/modules"
    deadline = time.monotonic() + int(os.environ.get("NORM_PUBLICATION_WAIT_SECONDS", "10800"))
    while time.monotonic() < deadline:
        for receipt in modules.glob("*/🧾️receipt.json"):
            try:
                state = json.loads(receipt.read_text(encoding="utf-8"))
            except (FileNotFoundError, json.JSONDecodeError):
                continue
            if state.get("pluginId") == plugin_id and state.get("wasmSha256") and any(receipt.parent.glob("*.core.wasm")):
                print(f"[DEBUG] {plugin_id} published {state['wasmSha256']}", flush=True)
                return
        time.sleep(8)
    raise SystemExit(f"{plugin_id} did not publish during the build window")

producer = root/"🧰️framework/🛍️products/💻️os/🔨️modules"
registry = str(producer/"🔌️plugin/📇️registry")
dev = str(producer/"🧑‍💻dev/📦️packages/🟦️typescript")
env["SEMIO_PLUGIN_CATALOG"] = str(root/"✏️s/🧑‍💻dev/🧩️catalog/🤖️generated")
scopes = sys.argv[1:] or ["din16798","iso16757","vdi3805"]
if plugin := os.environ.get("NORM_WAIT_FOR_MODULE"):
    wait_for_module(plugin)
failed = False
for scope in scopes:
    for owner, command, tail in [(registry,"session",[]),(dev,"prepare",["react","prototype"]),(dev,"activate",["react","prototype"])]:
        with (output/f"norm-base-preview-{command}-{scope}.log").open("w") as log:
            result = subprocess.run(["repo",str(ticket/"native-exec.ts"),owner,"./📜️script.ts",command,scope,*tail],
                                    cwd=root,env=env,stdout=log,stderr=subprocess.STDOUT).returncode
        if result:
            print(f"[DEBUG] {scope} {command} outcome {result}",flush=True)
            failed = True
            break
    if result:
        continue
    with (output/f"norm-base-preview-service-{scope}.log").open("w") as f:
        server = subprocess.Popen(["repo",str(ticket/"serve-play-native.ts")],cwd=root,
                                  env=dict(env,SEMIO_TECH_PLAY_SCOPE=scope),stdout=f,stderr=subprocess.STDOUT)
        try:
            ready = False
            for _ in range(30):
                if server.poll() is not None:
                    break
                try:
                    with urllib.request.urlopen(env["PLAYWRIGHT_BASE_URL"],timeout=2) as response:
                        if response.status == 200:
                            ready = True
                            break
                except OSError:
                    time.sleep(2)
            if not ready:
                print(f"[DEBUG] {scope} server did not start",flush=True)
                failed = True
                continue
            if os.environ.get("VERIFY_SIGNED_MODULES") == "1":
                if browser("verify-play-components.mjs","norm-scoped-signed-delivery",scope,root,"delivery"):
                    failed = True
                    continue
            if scope in ("en1990","en1992"):
                result = browser("probe-eurocode-viewports.mjs","norm-published-eurocode-viewport",scope,root)
            else:
                result = browser("probe-norm-routes.mjs","norm-base-preview-route",scope,root)
                if result == 0 and scope in ("din16798","iso16757","vdi3805"):
                    result = browser("probe-norm-catalogue-examples.mjs","norm-base-preview-examples",scope,play)
            print(f"[DEBUG] {scope} browser outcome {result}",flush=True)
            failed |= result!=0
        finally:
            if server.poll() is None:
                server.terminate()
                server.wait(timeout=15)
raise SystemExit(int(failed))
