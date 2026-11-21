/** 🛂️ Describes the actual Cargo-linked Draw component and checks its two semantic forms. */
import {resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {verifySemanticDescriptorPairBytesV1} from "@semio-tech/plugin-registry/catalog-verification/descriptor-pair";
import {validateDrawGuestScenarios} from "../../../../../../../🌎️hub/🧩️compositions/🖍️draw/🧪️tests/🌐️guest/🟦️.ts";

const [componentPath, emitterPath, outputDirectory] = process.argv.slice(2);
if (!componentPath || !emitterPath) throw Error("Expected linked Draw component and published descriptor emitter paths");
const component = resolve(componentPath);
const emitter = resolve(emitterPath);
const out = outputDirectory ? resolve(outputDirectory) : fileURLToPath(new URL("./🗑️generated/fresh-draw-descriptor/", import.meta.url));
const header = Buffer.from(await Bun.file(component).slice(0, 8).arrayBuffer());
if (header.toString("hex") !== "0061736d0d000100") throw Error("Expected a genuine WebAssembly component binary");
const child = Bun.spawn([emitter, "describe", component, "--out", out], {
    stdin: "ignore", stdout: "inherit", stderr: "inherit", signal: AbortSignal.timeout(300_000),
});
const exit = await child.exited;
if (exit !== 0) throw Error("Draw descriptor emitter exited " + exit);

const json = resolve(out, "🔣️.json");
const pack = resolve(out, "🛂️.descriptor.semio");
const descriptor = await Bun.file(json).bytes();
const packed = await Bun.file(pack).bytes();
const pair = verifySemanticDescriptorPairBytesV1(descriptor, packed);
if (pair.pluginId !== "draw" || pair.role !== "plugin") throw Error("Unexpected component descriptor " + pair.packageId);
await validateDrawGuestScenarios(process.cwd(), json);
const current = resolve("🌎️hub/🧩️compositions/🖍️draw/🧩️component");
const same = async (file: string, bytes: Uint8Array) => Buffer.from(bytes).equals(Buffer.from(await Bun.file(resolve(current, file)).bytes()));
console.log({component, descriptor: json, pack, semanticSha256: pair.semanticSha256,
    committedJsonEqual: await same("🔣️.json", descriptor), committedPackEqual: await same("🛂️.descriptor.semio", packed)});
