/** 🔭️ Transpiles one component with a chosen js-component-bindgen vendor build and reports every async task.return binding. */
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";

const [vendorModule, componentPath, outDir] = process.argv.slice(2);
if (!vendorModule || !componentPath || !outDir) throw new Error("usage: jc1-transpile-probe <vendor js-component-bindgen-component.js> <component.wasm> <outDir>");

const admitted = [
  "semio:framework/pure@1.0.0", "semio:framework/host-async@1.0.0",
  "wasi:cli/environment@0.2.0", "wasi:cli/exit@0.2.0", "wasi:cli/stdin@0.2.0", "wasi:cli/stdout@0.2.0", "wasi:cli/stderr@0.2.0",
  "wasi:cli/terminal-input@0.2.0", "wasi:cli/terminal-output@0.2.0", "wasi:cli/terminal-stdin@0.2.0", "wasi:cli/terminal-stdout@0.2.0", "wasi:cli/terminal-stderr@0.2.0",
  "wasi:clocks/monotonic-clock@0.2.0", "wasi:clocks/wall-clock@0.2.0", "wasi:io/error@0.2.0", "wasi:io/poll@0.2.0", "wasi:io/streams@0.2.0",
  "wasi:random/insecure-seed@0.2.9",
].sort();
const asyncImports = [
  "wasi:io/poll#poll", "wasi:io/poll#[method]pollable.block",
  "wasi:io/streams#[method]input-stream.blocking-read", "wasi:io/streams#[method]input-stream.blocking-skip",
  "wasi:io/streams#[method]output-stream.blocking-flush", "wasi:io/streams#[method]output-stream.blocking-write-and-flush",
  "wasi:io/streams#[method]output-stream.blocking-write-zeroes-and-flush", "wasi:io/streams#[method]output-stream.blocking-splice",
];

const vendor = await import(vendorModule);
if (vendor.$init) await vendor.$init;
const { generate } = vendor;
const component = new Uint8Array(readFileSync(componentPath));
const started = Date.now();
const output = await generate(component, {
  name: "browser-actor", instantiation: { tag: "async" }, noNodejsCompat: true, base64Cutoff: 0,
  map: admitted.map(name => [name, name]), asyncMode: { tag: "jspi", val: { imports: asyncImports, exports: [] } },
  noTypescript: false, tlaCompat: false, validLiftingOptimization: false, tracing: false,
  noNamespacedExports: false, multiMemory: false, bindgenEnableWasmExnref: false, strict: false, asmjs: false,
});
const elapsed = Date.now() - started;
mkdirSync(outDir, { recursive: true });
const files = Object.fromEntries(output.files);
for (const [name, bytes] of output.files) { mkdirSync(dirname(join(outDir, name)), { recursive: true }); writeFileSync(join(outDir, name), bytes, { mode: 0o600 }); }
const source = new TextDecoder("utf-8", { fatal: true }).decode(files["browser-actor.js"]);
const lines = source.split("\n");

const starts = [];
for (let index = 0; index < lines.length; index += 1) {
  const match = /const (trampoline\d+) = taskReturn\.bind\(/.exec(lines[index]);
  if (match) starts.push({ trampoline: match[1], line: index });
}
const rows = starts.map(({ trampoline, line }, position) => {
  const end = position + 1 < starts.length ? starts[position + 1].line : lines.length;
  let useDirect = null;
  const outer = [];
  for (let cursor = line; cursor < end; cursor += 1) {
    if (useDirect === null) { const direct = /useDirectParams: (true|false)/.exec(lines[cursor]); if (direct) useDirect = direct[1]; }
    const flat = /variantFlatCount: ([A-Za-z0-9]+)/.exec(lines[cursor]);
    const size = /variantSize32: (\d+)/.exec(lines[cursor]);
    if (flat) outer.push({ line: cursor + 1, variantFlatCount: flat[1] });
    if (size && outer.length > 0 && outer[outer.length - 1].variantSize32 === undefined) outer[outer.length - 1].variantSize32 = size[1];
  }
  const last = outer[outer.length - 1];
  return { trampoline, line: line + 1, useDirectParams: useDirect, outerVariantSize32: last?.variantSize32, outerVariantFlatCount: last?.variantFlatCount };
});
const coreExport = /'\[task-return\]poll': Object\.assign\(exports\d+\['(\d+)'\]/.exec(source)?.[1];
const pollTrampoline = coreExport === undefined ? undefined : new RegExp(`'${coreExport}': Object\\.assign\\(_guardMayLeave\\(\\d+, (trampoline\\d+)\\)`).exec(source)?.[1];
console.log(JSON.stringify({ vendorModule, componentPath, elapsedMs: elapsed, actorBytes: files["browser-actor.js"].byteLength, fileCount: output.files.length, imports: output.imports, pollCoreExport: coreExport, pollTaskReturnTrampoline: pollTrampoline, pollRow: rows.find(row => row.trampoline === pollTrampoline), taskReturnTrampolines: rows }, null, 2));
