#!/usr/bin/env python3
"""Copy of the runtime-seam set at the ticket root (`r6-runtime-seam-tools`, same depth: apply.ts derives the repository root from its location) re-based on the round-6 tree:
 - the two recipe anchors (`process.versions.bun` in the build bootstrap / dependency storage recipe) are gone after pm-seam + lock-sync, their manual edits are cut;
 - rcache-server's remote-cache server (HTTPS option, two `Bun.file(entry.path)` bodies) is carried over: the seam's serve options gain `tls` ({certificate, key} paths) in the contract, the Bun adapter and the portable `node:https` server, and the server edit uses it;
 - manual-edits.json is regenerated from the current tree (manual-gen.py)."""
import os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/runtime-seam-tools", "r6-runtime-seam-tools", beside=True)
G = T + "/manual-gen.py"
text = open(G, encoding="utf8").read()
lines = text.split("\n")
kept = []
for line in lines:
    if line.startswith('sub(boot, "manifest.patchedDependencies,process.versions.bun,process.platform"') or line.startswith('sub(store, "sourceRoot,process.versions.bun,process.platform"'):
        continue
    kept.append(line)
assert len(kept) == len(lines) - 2
text = "\n".join(kept)
text = text.replace('sub(rc, "Bun.file(entry.path)", "runtimeFileV1(entry.path).body()")', 'sub(rc, "Bun.file(entry.path)", "runtimeFileV1(entry.path).body()", count=2)')
old_serve = 'sub(rc, "const server=Bun.serve({hostname:options.host,port:options.port,idleTimeout:60,maxRequestBodySize:options.maximumEntryBytes*2,fetch:request=>handler(request)});", '
assert text.count(old_serve) == 1
i = text.index(old_serve)
j = text.index("\n", i)
row = text[i:j]
new_row = 'sub(rc, "const server=Bun.serve({hostname:options.host,port:options.port,idleTimeout:60,maxRequestBodySize:options.maximumEntryBytes*2,...(options.tls?{tls:{cert:Bun.file(options.tls.certificate),key:Bun.file(options.tls.key)}}:{}),fetch:request=>handler(request)});", "const server=await runtimeServeV1({hostname:options.host,port:options.port,idleTimeoutSeconds:60,maximumRequestBodyBytes:options.maximumEntryBytes*2,...(options.tls?{tls:{certificate:options.tls.certificate,key:options.tls.key}}:{}),fetch:request=>handler(request)});"); need(rc, "runtimeFileV1", "runtimeServeV1")'
text = text[:i] + new_row + text[j:]
open(G, "w", encoding="utf8").write(text)


def module_file(*tail):
    base = T + "/staged/new"
    for root, _, files in os.walk(base):
        path = os.path.join(root, tail[-1])
        if os.path.isfile(path) and root.replace(base, "").endswith(tuple(tail[:-1])) and "⚙️runtime" in root:
            return path
    raise SystemExit("maker: runtime module file not found: %r" % (tail,))


contract = module_file("📋️contract", "🟦️.ts")
patch(contract, "readonly maximumRequestBodyBytes?: number; readonly fetch:", "readonly maximumRequestBodyBytes?: number; readonly tls?: Readonly<{ certificate: string; key: string }>; readonly fetch:")
bun = module_file("🍞️bun", "🟦️.ts")
patch(bun, "maxRequestBodySize: options.maximumRequestBodyBytes, fetch: options.fetch })", "maxRequestBodySize: options.maximumRequestBodyBytes, ...(options.tls === undefined ? {} : { tls: { cert: bun().file(options.tls.certificate), key: bun().file(options.tls.key) } }), fetch: options.fetch })")
serve = module_file("🪶️portable", "🌍️serve", "🟦️.ts")
patch(serve, 'import { createServer, type IncomingMessage, type ServerResponse } from "node:http";\n', 'import { readFileSync } from "node:fs";\nimport { createServer, type IncomingMessage, type ServerResponse } from "node:http";\nimport { createServer as createSecureServer } from "node:https";\n')
patch(serve, "function requestOf(incoming: IncomingMessage, hostname: string, port: number, limit: number | undefined): Request {", "function requestOf(incoming: IncomingMessage, hostname: string, port: number, limit: number | undefined, scheme: string): Request {")
patch(serve, "url = `http://${incoming.headers.host", "url = `${scheme}://${incoming.headers.host")
patch(serve, "requestOf(incoming, hostname, boundPort, options.maximumRequestBodyBytes)", "requestOf(incoming, hostname, boundPort, options.maximumRequestBodyBytes, options.tls === undefined ? \"http\" : \"https\")")
patch(serve, "    const server = createServer((incoming, outgoing) => {\n", "    const listener = (incoming: IncomingMessage, outgoing: ServerResponse): void => {\n")
patch(serve, "      })();\n    });\n    if (options.idleTimeoutSeconds", "      })();\n    };\n    const server = options.tls === undefined ? createServer(listener) : createSecureServer({ cert: readFileSync(options.tls.certificate), key: readFileSync(options.tls.key) }, listener);\n    if (options.idleTimeoutSeconds")
run = subprocess.run([sys.executable, G], capture_output=True, text=True, cwd=T)
if run.returncode:
    raise SystemExit("maker: runtime-seam manual-gen.py failed: " + (run.stdout + run.stderr)[-800:])

root = os.environ.get("SEMIO_ROOT") or os.path.abspath(os.path.join(T, *[".."] * 8))
ledger_py = T + "/ledger.py"
lt = open(ledger_py, encoding="utf8").read()
anchor = "def classify(path, rule, snippet):\n"
assert lt.count(anchor) == 1
lt = lt.replace(anchor, anchor + '    if "/\U0001F9F0\ufe0ftools/" in path and "/\U0001F3C3\ufe0fprocess/" in path: return ("tool-registry", "tool-iface", "the tool registry names the runtime as one registered external tool")\n')
open(ledger_py, "w", encoding="utf8").write(lt)
final = T + "/final.json"
dry = subprocess.run(["bun", T + "/apply.ts", "--root", root, "--report", final], capture_output=True, text=True, cwd=T)
if not os.path.isfile(final):
    raise SystemExit("maker: runtime-seam dry run wrote no report: " + (dry.stdout + dry.stderr)[-800:])
led = subprocess.run([sys.executable, ledger_py, final], capture_output=True, text=True, cwd=T)
if led.returncode:
    raise SystemExit("maker: runtime-seam ledger.py failed: " + (led.stdout + led.stderr)[-800:])
if "UNCLASSIFIED" in led.stdout:
    raise SystemExit("maker: runtime-seam ledger has unclassified findings:\n" + led.stdout[-1500:])
law = module_file("\u2696\ufe0flaw", "\U0001F4D2\ufe0f.json")
import shutil
shutil.copyfile(T + "/src/ledger.json", law)
print("runtime-seam copy:", T, run.stdout.strip()[-120:], "| ledger:", led.stdout.strip().split("\n")[0])
