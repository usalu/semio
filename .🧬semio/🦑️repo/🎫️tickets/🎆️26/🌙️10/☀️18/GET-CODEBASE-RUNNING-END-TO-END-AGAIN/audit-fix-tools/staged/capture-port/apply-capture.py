#!/usr/bin/env python3
"""Increment `capture-port`: applies AFTER runtime-seam. Adds the bounded capture family to the runtime module and moves the generator preview spawn of inventory-planning onto it. Anchored, drift-refusing, dry run by default.
  --module-root <dir>  tree holding 🧰️framework/🔨️modules/⚙️runtime (default: the repository, i.e. runtime-seam must be applied)
  --repo <dir>         tree holding the library files (default: the repository)
  --write              apply (only on GO, after runtime-seam); without it nothing is written
  --overlay <dir>      write the patched files into <dir> instead (verification)"""
import hashlib, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
args = sys.argv[1:]
def opt(name, default):
    return args[args.index(name) + 1] if name in args else default
REPO = opt("--repo", "/Users/ueli/Documents/semio")
MODULE_ROOT = opt("--module-root", REPO)
OVERLAY = opt("--overlay", None)
WRITE = "--write" in args
RT = "🧰️framework/🔨️modules/⚙️runtime"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
IP = LIB + "/🧹️normalization/🔍️inventory-planning/🟦️.ts"
files = {}
def read(root, rel):
    p = os.path.join(root, rel)
    if not os.path.isfile(p): sys.exit("refused: missing " + rel + " (is runtime-seam applied?)")
    return open(p, encoding="utf8").read()
def sub(t, old, new, label):
    if t.count(old) != 1: sys.exit("refused: anchor drift (%d) in %s: %s" % (t.count(old), label, old[:80]))
    return t.replace(old, new)
# 1. facade export
t = read(MODULE_ROOT, RT + "/🟦️.ts")
if "🎣️capture" in t: sys.exit("refused: already applied")
files[(MODULE_ROOT, RT + "/🟦️.ts")] = sub(t, 'export * from "./🚀️process/🟦️.ts";\n', 'export * from "./🚀️process/🟦️.ts";\nexport * from "./🎣️capture/🟦️.ts";\n', "facade")
# 2. contract: spawn options gain a bound for synchronous capture
rel = RT + "/📋️contract/🟦️.ts"; t = read(MODULE_ROOT, rel)
files[(MODULE_ROOT, rel)] = sub(t, "readonly timeout?: number; readonly signal?: AbortSignal; readonly detached?: boolean }", "readonly timeout?: number; readonly maxBuffer?: number; readonly signal?: AbortSignal; readonly detached?: boolean }", "contract")
# 3. adapters pass the bound through
rel = RT + "/🍞️bun/🧩️tooling/🟦️.ts"; t = read(MODULE_ROOT, rel)
files[(MODULE_ROOT, rel)] = sub(t, 'stderr: options.stderr ?? "pipe", timeout: options.timeout });\n  return { exitCode: result.exitCode', 'stderr: options.stderr ?? "pipe", timeout: options.timeout, maxBuffer: options.maxBuffer });\n  return { exitCode: result.exitCode', "bun spawnSync")
rel = RT + "/🪶️portable/🚀️spawn/🟦️.ts"; t = read(MODULE_ROOT, rel)
files[(MODULE_ROOT, rel)] = sub(t, "input, timeout: options.timeout, maxBuffer: Infinity, windowsHide: true });", "input, timeout: options.timeout, maxBuffer: options.maxBuffer ?? Infinity, windowsHide: true });", "portable spawnSync")
# 4. catalog messages (EN first, DE second)
import json
rel = RT + "/🧬️schema/🗣️messages.json"; raw = read(MODULE_ROOT, rel); catalog = json.loads(raw)
if json.dumps(catalog, indent=2, ensure_ascii=False) + "\n" != raw and json.dumps(catalog, indent=2, ensure_ascii=False) != raw: sys.exit("refused: messages catalog not in canonical form")
if "captureBoundRequired" in catalog: sys.exit("refused: message already present")
catalog["captureBoundRequired"] = {"en": "The capture bound {name} must be a positive number.", "de": "Die Erfassungsgrenze {name} muss eine positive Zahl sein."}
files[(MODULE_ROOT, rel)] = json.dumps(catalog, indent=2, ensure_ascii=False) + ("\n" if raw.endswith("\n") else "")
# 5. new family files
NEW = os.path.join(HERE, "new")
for dirpath, _, names in os.walk(NEW):
    for n in names:
        src = os.path.join(dirpath, n); rel = os.path.relpath(src, NEW)
        if os.path.exists(os.path.join(MODULE_ROOT, rel)): sys.exit("refused: exists " + rel)
        files[(MODULE_ROOT, rel)] = open(src, encoding="utf8").read()
# 6. preview spawn of inventory-planning
t = read(REPO, IP)
start = t.index("export function invokeGeneratorPreview(")
end_marker = "  return { manifest, digest: sha256(stdout) };\n}\n"
end = t.index(end_marker, start) + len(end_marker)
old = t[start:end]
EXPECTED = os.environ.get("CAPTURE_PORT_EXPECT_SHA")
if EXPECTED and hashlib.sha256(old.encode()).hexdigest() != EXPECTED: sys.exit("refused: invokeGeneratorPreview drifted")
lines = old.split("\n")
sig = lines[0]
prelude_end = next(i for i, l in enumerate(lines) if l.lstrip().startswith("const result = spawnSync("))
prelude = lines[1:prelude_end]
spawn_line = lines[prelude_end]
cwd = spawn_line[spawn_line.index("cwd: "): spawn_line.index(", encoding")]
env = spawn_line[spawn_line.index("env: "): spawn_line.rindex(" });")]
tail = lines[prelude_end + 1:]
# tail: checkCancellation, stdout/stderr, failure check, progress, roots, manifest, return
iline = next(i for i, l in enumerate(tail) if l.lstrip().startswith("if (result.error"))
after_failure = tail[iline + 1:]
fail_msg = tail[iline][tail[iline].index("throw new Error("):]
signature_params = sig[sig.index("(") + 1: sig.rindex("): {")]
ret_type = sig[sig.rindex("): {") + 3:]
body = "\n".join(
  ["interface GeneratorPreviewPlanV1 { readonly request: RuntimeCaptureRequestV1; finish(result: RuntimeCaptureResultV1): { readonly manifest: TaxonomyGeneratorPreviewManifest; readonly digest: string } }",
   "",
   "/** 🎣️ Builds the bounded capture request of one owned generator preview and the exact judgement of its result, shared by the blocking and the cancellable invocation. */",
   "function generatorPreviewPlan(" + signature_params + "): GeneratorPreviewPlanV1 {"]
  + prelude
  + ["  const request: RuntimeCaptureRequestV1 = { command: [execution.command, ...execution.args], " + cwd + ", stdin: input, maxOutputBytes: limits.maxOutputBytes, timeoutMilliseconds: duration, " + env + " };",
     "  const finish = (result: RuntimeCaptureResultV1): { readonly manifest: TaxonomyGeneratorPreviewManifest; readonly digest: string } => {"]
  + ["  " + tail[0], "    const stdout = result.stdout, stderr = result.stderr;",
     "    if (result.outcome !== \"exited\" || result.exitCode !== 0) throw new Error(`Generator preview command failed for ${id}: status=${result.exitCode ?? -1}, stdout=${sha256(stdout)}, stderr=${sha256(stderr)}`);"]
  + ["  " + l for l in after_failure[:-3]]
  + ["    return { manifest, digest: sha256(stdout) };", "  };", "  return { request, finish };", "}", "",
     "/** 🎣️ Runs the generator preview of one owned contract, blocking the caller, within the declared time and output bounds. */",
     "export function invokeGeneratorPreview(" + signature_params + "): " + ret_type[:ret_type.rindex("{")].strip() + " {",
     "  const plan = generatorPreviewPlan(inventory, id, contract, taxonomy, projection, cancelFile);",
     "  return plan.finish(runtimeCaptureSyncV1(plan.request));",
     "}", "",
     "/** 🎣️ Runs the same generator preview without blocking the caller: the child stops when the caller's signal aborts or the cancellation file appears, and every progress line reaches `onProgress` while the child runs. */",
     "export async function invokeGeneratorPreviewAsyncV1(" + signature_params + ", signal?: AbortSignal, onProgress?: (event: RuntimeCaptureProgressV1) => void): Promise<" + ret_type[:ret_type.rindex("{")].strip().lstrip(": ").strip() + "> {",
     "  const plan = generatorPreviewPlan(inventory, id, contract, taxonomy, projection, cancelFile);",
     "  return plan.finish(await runtimeCaptureV1({ ...plan.request, signal, onProgress, cancelled: () => { try { checkCancellation(inventory.repoRoot, cancelFile); return false; } catch { return true; } } }));",
     "}", ""])
t = t[:start] + body + t[end:]
facade = os.path.relpath(os.path.join(REPO, RT, "🟦️.ts"), os.path.dirname(os.path.join(REPO, IP)))
t = sub(t, 'import { spawnSync } from "node:child_process";\n', 'import { runtimeCaptureSyncV1, runtimeCaptureV1, type RuntimeCaptureProgressV1, type RuntimeCaptureRequestV1, type RuntimeCaptureResultV1 } from "' + facade + '";\n', "import")
files[(REPO, IP)] = t
mode = "overlay" if OVERLAY else "write" if WRITE else "dry-run"
for (root, rel), content in files.items():
    if mode == "overlay": dest = os.path.join(OVERLAY, rel)
    else: dest = os.path.join(root, rel)
    if mode != "dry-run":
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        open(dest, "w", encoding="utf8").write(content)
print(mode, "files:", len(files))
