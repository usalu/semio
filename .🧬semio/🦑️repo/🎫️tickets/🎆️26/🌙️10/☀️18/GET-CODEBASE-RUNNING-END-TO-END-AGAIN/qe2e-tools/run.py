#!/usr/bin/env python3
import json, os, subprocess, sys
ROOT = "/Users/ueli/Documents/semio"
C = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad"
wire = json.loads(subprocess.run(["bun", C + "/qe2e-tools/envwire2.ts"], cwd=ROOT, capture_output=True, text=True, env=dict(os.environ, SEMIO_REPO_ROOT=ROOT)).stdout)
art = ROOT + "/.cache/qe2e/art"
os.makedirs(art, exist_ok=True)
env = dict(os.environ, SEMIO_TEST_ARTIFACT_DIR=art, NO_COLOR="1", SEMIO_REPO_ROOT=ROOT, SEMIO_SCRIPT_PROCESS_INVOCATION=wire["invocation"], SEMIO_SCRIPT_PROVIDERS=wire["providers"], SEMIO_BUILD_MODE=os.environ.get("QE2E_MODE","prototype"), SEMIO_WORKSPACE_ROOT=ROOT, SEMIO_BUILD_TOOL="nx", NODE_OPTIONS="", SEMIO_BUILD_DYNAMIC_OUTPUT="false", SEMIO_BUILD_TERMINAL="false", VSCODE_INSPECTOR_OPTIONS="", PLAYWRIGHT_BROWSERS_PATH=S + "/ms-playwright")
for kv in os.environ.get("QE2E_ENV", "").split():
    k, v = kv.split("=", 1)
    env[k] = v
cwd = sys.argv[1]
os.chdir(cwd)
os.execvpe(sys.argv[2], sys.argv[2:], env)
