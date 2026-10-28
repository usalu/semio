#!/usr/bin/env python3
"""📏 [audit12] independent oracle of batch `privparent`: evaluates the helper's own `privateGraphDeepestV1` (with node) for a private root below <checkout>/.cache/test-sandbox and compares it with the real deepest staging path the observation module writes (`path+"."+pid+"."+Date.now()+".tmp"`, pid up to 7 digits, 13-digit clock); exit 0 when the model never underestimates and the system scratch parent is offered. usage: python3 -I law_privparent.py <root> [checkout]"""
import os, re, subprocess, sys, time
root = sys.argv[1]; checkout = sys.argv[2] if len(sys.argv) > 2 else "/Users/ueli/Documents/semio"
support = open(os.path.join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🧰️support/🟦️.ts"), encoding="utf8").read()
observation = open(os.path.join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs"), encoding="utf8").read()
assert 'temporary=path+"."+process.pid+"."+Date.now()+".tmp"' in observation, "observation staging formula changed: update this oracle"
model = re.search(r"const privateGraphDeepestV1=\(root:string\)=>(.+?);\n", support).group(1)
private = checkout + "/.cache/test-sandbox/g000000"
modelled = int(subprocess.run(["node", "-e", f"const root={private!r};console.log({model})"], capture_output=True, text=True, check=True).stdout)
block = private + "/.cache/b/g/m/inference/contexts/" + "0" * 64 + "/inference/ob/" + "0" * 64 + ".br"
real = len(block + "." + str(4194304) + "." + str(int(time.time() * 1000)) + ".tmp")
fallback = '"/tmp","sg"' in support
print(f"[DEBUG] checkout {len(checkout)} units: model {modelled}, real {real}, model admits {modelled <= 256}, real fits {real <= 256}, system scratch parent offered {fallback}")
sys.exit(0 if modelled >= real and fallback else 1)
