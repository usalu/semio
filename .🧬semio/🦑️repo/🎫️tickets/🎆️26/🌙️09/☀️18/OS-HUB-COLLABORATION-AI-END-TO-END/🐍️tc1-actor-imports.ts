/** 🛂️ Prints the exact jco import interfaces of a component against the frozen browser-actor allowlist. */
import { readFileSync } from "node:fs";
import { generate } from "@bytecodealliance/jco/component";
import { browserWasiInterfaces } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts";

const extra = (process.env.TC1_EXTRA_ADMITTED ?? "").split(",").filter(name => name.length > 0);
const admitted = Object.freeze(["semio:framework/pure@1.0.0", "semio:framework/host-async@1.0.0", ...browserWasiInterfaces, ...extra].sort());
const componentPath = process.argv[2];
if (!componentPath) throw new Error("usage: tc1-actor-imports <component.wasm>");
const bytes = readFileSync(componentPath);
const asyncImports = [
  "wasi:io/poll#poll", "wasi:io/poll#[method]pollable.block",
  "wasi:io/streams#[method]input-stream.blocking-read", "wasi:io/streams#[method]input-stream.blocking-skip",
  "wasi:io/streams#[method]output-stream.blocking-flush", "wasi:io/streams#[method]output-stream.blocking-write-and-flush",
  "wasi:io/streams#[method]output-stream.blocking-write-zeroes-and-flush", "wasi:io/streams#[method]output-stream.blocking-splice",
];
const output = await generate(bytes, { name: "browser-actor", instantiation: { tag: "async" }, noNodejsCompat: true, base64Cutoff: 0, map: admitted.map(name => [name, name]), asyncMode: { tag: "jspi", val: { imports: asyncImports, exports: [] } }, noTypescript: false, tlaCompat: false, validLiftingOptimization: false, tracing: false, noNamespacedExports: false, multiMemory: false, bindgenEnableWasmExnref: false, strict: false, asmjs: false });
const imports: string[] = [...output.imports].sort();
const unexpected = imports.filter(name => !admitted.includes(name));
const unused = admitted.filter(name => !imports.includes(name));
console.log(JSON.stringify({ component: componentPath, byteLength: bytes.byteLength, imports, unexpected, unusedAdmitted: unused, files: [...output.files].map(([name]) => name).sort() }, undefined, 2));
