import { join, resolve } from "node:path";
import { existsSync, mkdirSync, writeFileSync, readFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { pathToFileURL } from "node:url";
import assert from "node:assert/strict";

const output = join(import.meta.dirname, "🗑️generated", "guest");
const modulePath = join(output, "bim.js");
const bundle = resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle");
const source = (name: string) => import(pathToFileURL(join(bundle, name, "🟦️.ts")).href);
if (process.argv[2] === "prepare") {
  const { nativeTranspileArgsV1, nativeTranspileBinaryNameV1, PLUGIN_COMPONENT_IMPORT_MAPS_V1 } = await source("🏗️materialization/🦀️native-transpile");
  const { rewriteJcoAsyncResultLiftingAt, rewriteJcoComponentFactoryAt } = await source("🏗️materialization/🧬️jco-module-rewrites");
  const { hostShimSource } = await source("🏗️materialization/🪢️host-shim-source");
  const { pluginComponentBridgeSource } = await source("🏗️materialization/🌉️component-bridge-source");
  mkdirSync(output, { recursive: true });
  const binary = resolve(process.argv[3]);
  const witPath = join(output, "component.wit");
  const wit = spawnSync("node", [resolve("node_modules/.bin/jco"), "wit", binary, "-o", witPath], { stdio: "inherit" });
  assert.equal(wit.status, 0);
  const contract = readFileSync(witPath, "utf8");
  for (const facet of ["reactor", "jobs", "checkpoint", "describe", "codec"]) {
    assert.match(contract, new RegExp("\\bexport\\s+(?:[\\w:.-]+/)?"+facet+"(?:[;:@\\s])"));
  }
  const native = join(import.meta.dirname, "🗑️generated", "host", "debug", nativeTranspileBinaryNameV1());
  const args = nativeTranspileArgsV1(binary, output, "bim", PLUGIN_COMPONENT_IMPORT_MAPS_V1);
  const result = existsSync(native)
    ? spawnSync(native, args, { stdio: "inherit" })
    : spawnSync("node", [resolve("node_modules/.bin/jco"), ...args], { stdio: "inherit" });
  assert.equal(result.status, 0);
  rewriteJcoAsyncResultLiftingAt(modulePath);
  rewriteJcoComponentFactoryAt(modulePath);
  writeFileSync(join(output, "package.json"), JSON.stringify({ type: "module" }));
  writeFileSync(join(output, "🟨️.js"), hostShimSource());
  writeFileSync(join(output, "bridge.js"), pluginComponentBridgeSource("bim"));
} else {
  const frames: unknown[] = [];
  Object.defineProperty(globalThis, "self", { value: { postMessage(message: {frame: {envelope: {payload: {kind: string}}}}) {
    assert.notEqual(message.frame.envelope.payload.kind, "effect-request", "opening needs a host effect");
    frames.push(structuredClone(message));
  } } });
  const { decodeActorInstanceLifecycle } = await import(pathToFileURL(resolve("🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts")).href);
  const { createActorApi } = await import(pathToFileURL(join(output, "bridge.js")).href);
  const policy = JSON.parse(readFileSync(resolve("🌎️hub/🧩️compositions/🖍️draw/🧪️tests/🌐️guest/scenarios.json"), "utf8")).caller;
  const contract = JSON.parse(readFileSync(resolve("🌎️hub/🧩️compositions/🏙️bim/🧪️tests/🧫️fixtures/🛂️committed-descriptor/🔣️.json"), "utf8"));
  const { exercise } = await import("./guest-scenarios.ts");
  await exercise(createActorApi, decodeActorInstanceLifecycle, contract.applicationIds, policy, () => process.hrtime.bigint() / 1000n, async (url: string) => WebAssembly.compile(readFileSync(new URL(url))));
}
