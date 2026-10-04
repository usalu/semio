#!/usr/bin/env bun
/** 🔬️ Diagnoses `emitted descriptor pack is not canonical`: re-emits the puzzle descriptor into the ticket's
 * `🗑️generated/e2e/describe-diag`, round-trips the pack through the TS codec and prints the first differing byte,
 * both lengths, hex context and the JSON/pack-form disagreement. Usage: `bun 🧪️s4-describe-pack-diff.ts [component.wasm]`. */
import { mkdirSync, readFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
import { isDeepStrictEqual } from "node:util";
import { ensureBuiltBin, extractPluginCore } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts";
import { decodePackValue, encodePackValue, packValueToExactJson } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts";

const repo = "/Users/ueli/Documents/semio";
const component = process.argv[2] ?? join(repo, "🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/dist/component-dev/semio_hub_puzzle.wasm");
const out = join(import.meta.dir, "🗑️generated/e2e/describe-diag");
rmSync(out, { recursive: true, force: true });
mkdirSync(join(out, "core"), { recursive: true });
const core = extractPluginCore(repo, component, join(out, "core"), "semio_hub_puzzle");
const emitter = ensureBuiltBin(repo);
const run = spawnSync(emitter, ["describe", component, "--core", core, "--out", out], { cwd: repo, stdio: "inherit" });
if (run.status !== 0) throw new Error(`emitter exited ${run.status}`);
const pack = readFileSync(join(out, "🛂️.descriptor.semio"));
const json = JSON.parse(readFileSync(join(out, "🔣️.json"), "utf8"));
const value = decodePackValue(pack);
const again = Buffer.from(encodePackValue(value));
console.log(`[DEBUG] emitted ${pack.byteLength} bytes, re-encoded ${again.byteLength} bytes`);
let at = 0;
while (at < pack.byteLength && at < again.byteLength && pack[at] === again[at]) at++;
console.log(`[DEBUG] first difference at ${at}`);
const hex = (b: Uint8Array) => Buffer.from(b).toString("hex").replace(/(..)/g, "$1 ");
console.log(`[DEBUG] emitted   ${hex(pack.subarray(Math.max(0, at - 32), at + 32))}`);
console.log(`[DEBUG] re-encoded ${hex(again.subarray(Math.max(0, at - 32), at + 32))}`);
console.log(`[DEBUG] ascii around: ${JSON.stringify(Buffer.from(pack.subarray(Math.max(0, at - 160), at + 32)).toString("latin1"))}`);
const exact = packValueToExactJson(value) as Record<string, unknown>;
const diffs: string[] = [];
const walk = (a: unknown, b: unknown, path: string) => {
  if (diffs.length >= 12 || isDeepStrictEqual(a, b)) return;
  if (a && b && typeof a === "object" && typeof b === "object" && Array.isArray(a) === Array.isArray(b)) {
    for (const k of new Set([...Object.keys(a as object), ...Object.keys(b as object)])) walk((a as any)[k], (b as any)[k], `${path}/${k}`);
  } else diffs.push(`${path}: json=${JSON.stringify(a)?.slice(0, 120)} pack=${JSON.stringify(b)?.slice(0, 120)}`);
};
walk(json, exact, "");
console.log(`[DEBUG] json/pack disagreements (first 12):\n${diffs.join("\n")}`);
