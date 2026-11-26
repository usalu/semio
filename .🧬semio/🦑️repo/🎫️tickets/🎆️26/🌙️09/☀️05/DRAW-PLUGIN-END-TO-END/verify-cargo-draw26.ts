import { createHash } from "node:crypto";
import { createReadStream } from "node:fs";
import { join, resolve } from "node:path";
import { verifySemanticDescriptorPairBytesV1 } from "@semio-tech/plugin-registry/catalog-verification/descriptor-pair";
import { validateDrawGuestScenarios } from "../../../../../../../🌎️hub/🧩️compositions/🖍️draw/🧪️tests/🌐️guest/🟦️.ts";

const [directory] = process.argv.slice(2);
if (!directory) throw Error("Expected the ticket's diagnostic Cargo output directory");
const output = resolve(directory);
const receipt = await Bun.file(join(output, "artifact.json")).json();
if (receipt.kind !== "draw-cargo-component-diagnostic" || receipt.cargoExit !== 0 ||
    receipt.filename !== "component.wasm" || receipt.artifact?.reason !== "compiler-artifact" ||
    receipt.artifact?.target?.name !== "semio_hub_draw_component" ||
    !receipt.artifact?.filenames?.includes(receipt.source) ||
    !Number.isSafeInteger(receipt.bytes) || receipt.bytes <= 8 || receipt.bytes > 512 * 1024 ** 2) {
  throw Error("Expected the bounded Cargo diagnostic artifact evidence");
}
const component = join(output, "component.wasm");
const file = Bun.file(component);
const header = Buffer.from(await file.slice(0, 8).arrayBuffer()).toString("hex");
if (file.size !== receipt.bytes || header !== "0061736d0d000100") throw Error("Captured component has an incorrect extent or header");
const hash = createHash("sha256");
let length = 0;
for await (const chunk of createReadStream(component)) {
  hash.update(chunk);
  length += chunk.length;
  if (length > receipt.bytes) throw Error("Captured component grew during receipt verification");
}
const componentSha256 = hash.digest("hex");
if (componentSha256 !== receipt.sha256 || length !== receipt.bytes) throw Error("Captured component differs from its diagnostic receipt");

const jsonPath = join(output, "descriptor", "🔣️.json");
const packPath = join(output, "descriptor", "🛂️.descriptor.semio");
const pair = verifySemanticDescriptorPairBytesV1(await Bun.file(jsonPath).bytes(), await Bun.file(packPath).bytes());
if (pair.pluginId !== "draw" || pair.packageId !== "semio:draw" || pair.role !== "plugin") throw Error("Expected the Draw plugin descriptor");
await validateDrawGuestScenarios(process.cwd(), jsonPath);
console.log({ componentSha256, componentBytes: length, descriptorSha256: pair.semanticSha256 });
