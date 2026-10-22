#!/usr/bin/env python3
"""Copy of the runtime-seam set at the ticket root (`r6-runtime-seam-tools`, same depth: apply.ts derives the repository root from its location) re-based on the round-6 tree:
 - the two recipe anchors (`process.versions.bun` in the build bootstrap / dependency storage recipe) are gone after pm-seam + lock-sync, their manual edits are cut;
 - rcache-server's remote-cache server (HTTPS option, two `Bun.file(entry.path)` bodies) is carried over: the seam's serve options gain `tls` ({certificate, key} paths) in the contract, the Bun adapter and the portable `node:https` server, and the server edit uses it;
 - manual-edits.json is regenerated from the current tree (manual-gen.py)."""
import os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/runtime-seam-tools", "r6-runtime-seam-tools", beside=True)
for _law in (T + "/src/law.ts", T + "/staged/new/\U0001F9F0\ufe0fframework/\U0001F528\ufe0fmodules/\u2699\ufe0fruntime/\u2696\ufe0flaw/\U0001F7E6\ufe0f.ts"):
    patch(_law, 'import { readFileSync } from "node:fs";', 'import { existsSync, readFileSync } from "node:fs";')
    patch(_law, 'SOURCE_FILE.test(path) && !isRuntimeLawExemptPathV1(path));', 'SOURCE_FILE.test(path) && !isRuntimeLawExemptPathV1(path) && existsSync(join(repositoryRoot, path)));')
for _tests in (T + "/src/tests.ts", T + "/staged/new/\U0001F9F0\ufe0fframework/\U0001F528\ufe0fmodules/\u2699\ufe0fruntime/\U0001F9EA\ufe0ftests/\U0001F7E6\ufe0f.ts"):
    patch(_tests, '"{a,.hid}/**/*.ts", ".hid/**", "a/f.ts"', '"{a,.hid}/**/*.ts", "a/f.ts"')
G = T + "/manual-gen.py"
text = open(G, encoding="utf8").read()
text = text.replace("def need(path, *names):", """def try_sub(path, old, new, **kw):
    try:
        sub(path, old, new, **kw)
        return True
    except SystemExit as error:
        if str(error).startswith("NO MATCH"):
            return False
        raise
def need(path, *names):""", 1)
OBSOLETE = ("\U0001F3D7\ufe0fbuild/\U0001F680\ufe0fbootstrap/\U0001F7E6\ufe0f.ts", "\U0001F4E6\ufe0fdependencies/\U0001F4BE\ufe0fstorage/\U0001F7E6\ufe0f.ts", "\U0001F4E6\ufe0fdependencies/\U0001F9FE\ufe0fgeneration/\U0001F4BE\ufe0fstorage/\U0001F7E6\ufe0f.ts", "\U0001F4DA\ufe0flibrary/\U0001F3C3\ufe0fprocess/\U0001F7E6\ufe0f.ts", "\U0001F310\ufe0fremote-cache/\U0001F5A5\ufe0fserver/\U0001F7E6\ufe0f.ts", "\U0001F9FE\ufe0fclosures/\U0001F7E6\ufe0f.ts", "\U0001F4CF\ufe0fheadroom/\U0001F7E6\ufe0f.ts")
raise_old = '        raise SystemExit(f"NO MATCH {path}: {old[:80]}")\n    if n != count'
rx_old = '        if not found: raise SystemExit(f"NO MATCH {path}: {old[:80]}")\n'
assert text.count(rx_old) == 1
text = text.replace(rx_old, '        if not found:\n            if path.endswith(%r):\n                SKIPPED.add(path)\n                return\n            raise SystemExit(f"NO MATCH {path}: {old[:80]}")\n' % (OBSOLETE,))
assert text.count(raise_old) == 1
text = text.replace(raise_old, '        if path.endswith(%r):\n            SKIPPED.add(path)\n            return\n' % (OBSOLETE,) + raise_old)
text = text.replace("files = {}\n", "files = {}\nSKIPPED = set()\n", 1)
need_old = "def need(path, *names):\n    row = entry(path)\n"
assert text.count(need_old) == 1
text = text.replace(need_old, "def need(path, *names):\n    if path in SKIPPED and not files.get(path, {}).get(\"edits\"):\n        return\n    row = entry(path)\n")
rows = text.split("\n")
at = next(k for k, l in enumerate(rows) if l.startswith("sub(gen, "))
rows[at + 1:at + 1] = [
    'sub(gen, \'revision:String((globalThis as {Bun?:{revision?:string}}).Bun?.revision??"")\', \'revision:String(runtimeIdentityV1().revision)\'); need(gen, "runtimeIdentityV1")',
    'sub(store, "adoptRuntime={bun:process.versions.bun,", "adoptRuntime={bun:runtimeIdentityV1().version,"); need(store, "runtimeIdentityV1")',
]
text = "\n".join(rows)
RCS = os.path.join(repo_root(), "\U0001F9F0\ufe0fframework/\U0001F6CD\ufe0fproducts/\U0001F991\ufe0frepo/\U0001F528\ufe0fmodules/\U0001F4DA\ufe0flibrary/\U0001F3D7\ufe0fbuild/\U0001F310\ufe0fremote-cache/\U0001F5A5\ufe0fserver/\U0001F7E6\ufe0f.ts")
RC_TLS = "options.tls" in open(RCS, encoding="utf8").read()
text = text.replace('sub(rc, "Bun.file(entry.path)", "runtimeFileV1(entry.path).body()")', 'sub(rc, "Bun.file(entry.path)", "runtimeFileV1(entry.path).body()"%s)' % (", count=2" if RC_TLS else ""))
old_serve = 'sub(rc, "const server=Bun.serve({hostname:options.host,port:options.port,idleTimeout:60,maxRequestBodySize:options.maximumEntryBytes*2,fetch:request=>handler(request)});", '
assert text.count(old_serve) == 1
i = text.index(old_serve)
j = text.index("\n", i)
row = text[i:j]
new_row = 'sub(rc, "const server=Bun.serve({hostname:options.host,port:options.port,idleTimeout:60,maxRequestBodySize:options.maximumEntryBytes*2,...(options.tls?{tls:{cert:Bun.file(options.tls.certificate),key:Bun.file(options.tls.key)}}:{}),fetch:request=>handler(request)});", "const server=await runtimeServeV1({hostname:options.host,port:options.port,idleTimeoutSeconds:60,maximumRequestBodyBytes:options.maximumEntryBytes*2,...(options.tls?{tls:{certificate:options.tls.certificate,key:options.tls.key}}:{}),fetch:request=>handler(request)});"); need(rc, "runtimeFileV1", "runtimeServeV1")'
if not RC_TLS:
    new_row = new_row.replace('...(options.tls?{tls:{cert:Bun.file(options.tls.certificate),key:Bun.file(options.tls.key)}}:{}),', '').replace('...(options.tls?{tls:{certificate:options.tls.certificate,key:options.tls.key}}:{}),', '')
text = text[:i] + new_row + text[j:]
open(G, "w", encoding="utf8").write(text)


def module_file(*tail):
    """The module file by its path tail (the law ledger)."""
    base = T + "/staged/new"
    for root, _, files in os.walk(base):
        path = os.path.join(root, tail[-1])
        if os.path.isfile(path) and root.replace(base, "").endswith(tuple(tail[:-1])) and "\u2699\ufe0fruntime" in root:
            return path
    raise SystemExit("maker: runtime module file not found: %r" % (tail,))


def containing(needle):
    """The one staged module file that holds `needle` (the owner moves code between files)."""
    hits = []
    base = T + "/staged/new"
    for root, _, files in os.walk(base):
        for f in files:
            path = os.path.join(root, f)
            if f.endswith(".ts") and needle in open(path, encoding="utf8").read():
                hits.append(path)
    if len(hits) != 1:
        raise SystemExit("maker: %d module files hold %r" % (len(hits), needle[:70]))
    return hits[0]


contract = containing("export interface RuntimeServeOptionsV1")
patch(contract, "readonly maximumRequestBodyBytes?: number; readonly fetch:", "readonly maximumRequestBodyBytes?: number; readonly tls?: Readonly<{ certificate: string; key: string }>; readonly fetch:")
bun = containing("maxRequestBodySize: options.maximumRequestBodyBytes")
patch(bun, "maxRequestBodySize: options.maximumRequestBodyBytes, fetch: options.fetch })", "maxRequestBodySize: options.maximumRequestBodyBytes, ...(options.tls === undefined ? {} : { tls: { cert: bun().file(options.tls.certificate), key: bun().file(options.tls.key) } }), fetch: options.fetch })")
serve = containing("function requestOf(incoming: IncomingMessage")
patch(serve, 'import { createServer, type IncomingMessage, type ServerResponse } from "node:http";\n', 'import { readFileSync } from "node:fs";\nimport { createServer, type IncomingMessage, type ServerResponse } from "node:http";\nimport { createServer as createSecureServer } from "node:https";\n')
patch(serve, "function requestOf(incoming: IncomingMessage, hostname: string, port: number, limit: number | undefined): Request {", "function requestOf(incoming: IncomingMessage, hostname: string, port: number, limit: number | undefined, scheme: string): Request {")
patch(serve, "url = `http://${incoming.headers.host", "url = `${scheme}://${incoming.headers.host")
patch(serve, "requestOf(incoming, hostname, boundPort, options.maximumRequestBodyBytes)", "requestOf(incoming, hostname, boundPort, options.maximumRequestBodyBytes, options.tls === undefined ? \"http\" : \"https\")")
patch(serve, "    const server = createServer((incoming, outgoing) => {\n", "    const listener = (incoming: IncomingMessage, outgoing: ServerResponse): void => {\n")
patch(serve, "      })();\n    });\n    if (options.idleTimeoutSeconds", "      })();\n    };\n    const server = options.tls === undefined ? createServer(listener) : createSecureServer({ cert: readFileSync(options.tls.certificate), key: readFileSync(options.tls.key) }, listener);\n    if (options.idleTimeoutSeconds")
G2 = T + "/manual-gen2.py"
closure_file = os.path.join(os.environ.get("RUNTIME_SEAM_ROOT") or repo_root(), "\U0001F9F0\ufe0fframework/\U0001F6CD\ufe0fproducts/\U0001F991\ufe0frepo/\U0001F528\ufe0fmodules/\U0001F4DA\ufe0flibrary/\U0001F3D7\ufe0fbuild/\U0001F4E5\ufe0fsource/\U0001F50F\ufe0fclosure/\U0001F7E6\ufe0f.ts")
if "function sourceClosureLoaderV1(" in open(closure_file, encoding="utf8").read():
    g2 = open(G2, encoding="utf8").read()
    old_start = g2.index('extract(CL + "/')
    old_end = g2.index("\n", old_start)
    new_line = 'extract(CL + "/\U0001F7E6\ufe0f.ts", "/** \U0001F3C3\ufe0f Runs only checksum-bound captured modules at their original Source URLs. */\\nfunction sourceClosureLoaderV1(", "\\n`;\\n}", CL + "/\U0001F35E\ufe0fbun/\U0001F7E6\ufe0f.ts", ["sourceClosureLoaderV1"], reexport=False, prefix=\'import schema from "../\U0001F9EC\ufe0fschema/\U0001F523\ufe0f.json" with { type: "json" };\\nimport{capturedSourceImportDiagnosticV1}from"../\U0001F9FE\ufe0fdiagnostics/\U0001F7E6\ufe0f.ts";\\nimport{closureRefusalMessagesV1,closureRefusalTextV1,closureTargetSchemesV1}from"../../../../\u26A1\ufe0fcaching/\U0001F680\ufe0fbootstrap/\U0001F6E0\ufe0ftools/\U0001F50F\ufe0fauthorizers/\U0001F9ED\ufe0fsites/\U0001F7E6\ufe0f.ts";\\n\\nconst policy = schema.definitions.Policy.const;\\n\\n\')'
    g2 = g2[:old_start] + new_line + g2[old_end:]
    open(G2, "w", encoding="utf8").write(g2)
g2 = open(G2, encoding="utf8").read()
CODE_OLD = "sub(codef, 'control,\"bun\");bytes=result.bytes', 'control,\"runtime\");bytes=result.bytes')"
CODE_NEW = "sub(codef, 'control,\"bun\",\"program\");bytes=result.bytes', 'control,\"runtime\",\"program\");bytes=result.bytes')"
assert g2.count(CODE_OLD) == 1
open(G2, "w", encoding="utf8").write(g2.replace(CODE_OLD, CODE_NEW))
g2 = open(G2, encoding="utf8").read()
OA_OLD = "sub(oa, 'return { command: \"bun\", args, cwd: options.cwd as string };', 'return { command: runtimeCommandNameV1(), args, cwd: options.cwd as string };'); need(oa, \"runtimeCommandNameV1\")"
OA_NEW = "sub(oa, 'return { command: process.versions.bun ? process.execPath : \"bun\", args, cwd: options.cwd as string };', 'return { command: runtimeExecutableV1(), args, cwd: options.cwd as string };'); need(oa, \"runtimeCommandNameV1\", \"runtimeExecutableV1\")"
assert g2.count(OA_OLD) == 1
open(G2, "w", encoding="utf8").write(g2.replace(OA_OLD, OA_NEW))
run = subprocess.run([sys.executable, G], capture_output=True, text=True, cwd=T)
if run.returncode:
    raise SystemExit("maker: runtime-seam manual-gen.py failed: " + (run.stdout + run.stderr)[-800:])

if not os.path.isfile(T + "/ledger.py"):
    print("runtime-seam: the owner has no tolerance ledger any more (strict law): ledger step skipped")
else:
    root = os.environ.get("SEMIO_ROOT") or os.path.abspath(os.path.join(T, *[".."] * 8))
    ledger_py = T + "/ledger.py"
    lt = open(ledger_py, encoding="utf8").read()
    anchor = "def classify(path, rule, snippet):\n"
    assert lt.count(anchor) == 1
    lt = lt.replace(anchor, anchor + '    if "/\U0001F9F0\ufe0ftools/" in path and "/\U0001F3C3\ufe0fprocess/" in path: return ("tool-registry", "tool-iface", "the tool registry names the runtime as one registered external tool")\n    if "/\U0001F9EA\ufe0ftests/" in path: return ("test-source-scan", "runtime-seam", "a test that reads module source text or loads the Bun adapter of a storage module by its path")\n')
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
print("runtime-seam copy:", T, run.stdout.strip()[-120:])
