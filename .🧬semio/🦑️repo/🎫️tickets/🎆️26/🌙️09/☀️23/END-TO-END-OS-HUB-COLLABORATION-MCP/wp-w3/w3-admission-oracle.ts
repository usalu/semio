/** 🔬️ Oracle: compares `browserActorImportAdmissionV1` (own parser) with the pinned jco codegen's own import manifest on real components. */
import { readFileSync } from "node:fs";
import { generate } from "@bytecodealliance/jco/component";
import { browserActorImportAdmissionV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts";
import { browserWasiInterfaces } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts";

const admittedList = ["semio:framework/pure@1.0.0", "semio:framework/host-async@1.0.0", ...browserWasiInterfaces].sort();
const asyncImports = ["wasi:io/poll#poll", "wasi:io/poll#[method]pollable.block", "wasi:io/streams#[method]input-stream.blocking-read", "wasi:io/streams#[method]input-stream.blocking-skip", "wasi:io/streams#[method]output-stream.blocking-flush", "wasi:io/streams#[method]output-stream.blocking-write-and-flush", "wasi:io/streams#[method]output-stream.blocking-write-zeroes-and-flush", "wasi:io/streams#[method]output-stream.blocking-splice"];
let agree = 0, disagree = 0;
for (const path of process.argv.slice(2)) {
  const bytes = readFileSync(path);
  const started = performance.now();
  const ours = browserActorImportAdmissionV1(bytes);
  const parseMs = performance.now() - started;
  let jcoImports: string[] = [];
  try {
    const output = await generate(bytes, { name: "browser-actor", instantiation: { tag: "async" }, noNodejsCompat: true, base64Cutoff: 0, map: admittedList.map(name => [name, name]), asyncMode: { tag: "jspi", val: { imports: asyncImports, exports: [] } }, noTypescript: true, tlaCompat: false, validLiftingOptimization: false, tracing: false, noNamespacedExports: false, multiMemory: false, bindgenEnableWasmExnref: false, strict: false, asmjs: false } as never);
    jcoImports = [...(output as { imports: string[] }).imports].sort();
  } catch (error) {
    jcoImports = [`<jco error: ${error instanceof Error ? error.message.slice(0, 120) : String(error)}>`];
  }
  const jcoAdmitted = jcoImports.filter(name => admittedList.includes(name));
  const jcoRefused = jcoImports.filter(name => !admittedList.includes(name));
  const same = JSON.stringify(jcoAdmitted) === JSON.stringify(ours.admitted) && (jcoRefused.length > 0) === (ours.refused.length > 0);
  if (same) agree++; else disagree++;
  console.log(`${same ? "AGREE   " : "DISAGREE"} parse=${parseMs.toFixed(1)}ms ${path.replace(/^.*🔌️plugins\//, "")} ours.admitted=${ours.admitted.length} ours.refused=[${ours.refused.join(",")}] jco.refused=[${jcoRefused.join(",")}]${same ? "" : ` ours=${JSON.stringify(ours.admitted)} jco=${JSON.stringify(jcoAdmitted)}`}`);
}
console.log(`oracle: agree=${agree} disagree=${disagree}`);
process.exit(disagree === 0 ? 0 : 1);
