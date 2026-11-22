/** 🧾️ Reconciles the native Draw model and its materialized browser files with their original receipt. */
import { createHash } from "node:crypto";
import { createReadStream } from "node:fs";
import { resolve } from "node:path";
import { verifySemanticDescriptorPairBytesV1 } from "@semio-tech/plugin-registry/catalog-verification/descriptor-pair";
import { finalizePluginDescriptor } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts";

const [model, browser] = Bun.argv.slice(2);
if (!model || !browser) throw Error("Expected the Draw model and materialized module directories");
const committed = resolve("🌎️hub/🧩️compositions/🖍️draw/🧩️component");
const load = async (root: string, name: string) => Buffer.from(await Bun.file(resolve(root, name)).bytes());
const wasmPath = resolve(model, "📦️component.wasm");
const wasm = Bun.file(wasmPath);
const hasher = createHash("sha256");
for await (const page of createReadStream(wasmPath)) hasher.update(page);
const sha = hasher.digest("hex");
const receipt = await Bun.file(resolve(model, "🧾️receipt.json")).json();
const browserReceipt = await Bun.file(resolve(browser, "🧾️receipt.json")).json();
if (sha !== receipt.wasmSha256 || receipt.wasmBytes !== wasm.size ||
    JSON.stringify(receipt) !== JSON.stringify(browserReceipt) ||
    Buffer.from(await wasm.slice(0, 8).arrayBuffer()).toString("hex") !== "0061736d0d000100") {
  throw Error("Draw receipt disagrees with the actual WASI bytes or browser module");
}

const json = "🔣️.json";
const pack = "🛂️.descriptor.semio";
const modelJson = await load(model, json);
const modelPack = await load(model, pack);
if (!modelJson.equals(await load(committed, json)) || !modelPack.equals(await load(committed, pack))) {
  throw Error("Draw native descriptor differs from the authored pair");
}
const pair = verifySemanticDescriptorPairBytesV1(modelJson, modelPack);
if (pair.pluginId !== "draw" || pair.semanticSha256 !== receipt.semanticDescriptorSha256) throw Error("Draw semantic receipt differs");

const deployed = finalizePluginDescriptor(modelPack, "draw");
if (!Buffer.from(deployed.pack).equals(await load(browser, pack)) || !Buffer.from(deployed.json).equals(await load(browser, json))) {
  throw Error("Draw browser descriptor differs from the published semantic Pack");
}
console.log(JSON.stringify({ sha, wasmBytes: wasm.size, semantic: pair.semanticSha256, committedModel: true, materializedBrowser: true }));
