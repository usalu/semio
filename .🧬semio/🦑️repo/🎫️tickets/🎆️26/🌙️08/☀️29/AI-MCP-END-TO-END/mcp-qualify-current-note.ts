import { constants } from "node:fs";
import { copyFile, mkdir, stat } from "node:fs/promises";
import { join, resolve } from "node:path";
import { stripCoreNamesV1 } from "./mcp-native-wasm-names.ts";

const [emitter, directory = "m-trace", linkedComponent] = process.argv.slice(2);
if (!emitter || !/^m-[a-z-]+$/u.test(directory)) throw Error("Exact descriptor emitter and generated model directory required");
const generated = resolve(import.meta.dirname, "🗑️generated");
const model = join(generated, directory, "browser", "prototype", "components", "6d8a4f7ae2584b94ed7f1643");
await mkdir(model, { recursive: true });
const raw = linkedComponent ? resolve(linkedComponent) : join(generated, "native-guests", "wasm32-wasip2", "debug", "semio_hub_note_component.wasm");
const component = join(model, "📦️component.wasm");
const maximum = 512 * 1024 ** 2;
if ((await stat(raw)).size > maximum) {
  const removed = await stripCoreNamesV1(raw, component, maximum);
  console.log("[DEBUG] verified optional Note name-section removal", removed);
} else {
  await copyFile(raw, component, constants.COPYFILE_FICLONE | constants.COPYFILE_EXCL);
}
const verification = Bun.spawn([process.execPath, join(import.meta.dirname, "mcp-native-preparation-oracle.ts"), component], { stdout: "inherit", stderr: "inherit" });
if (await verification.exited) throw Error("The traced Note component could not prepare without imports");
const preparation = "mcp-core-preparation-law" + (process.platform === "win32" ? ".exe" : "");
const image = Bun.spawn([join(generated, preparation), "--ignored", "--nocapture"], {
  env: { ...process.env, SEMIO_PREPARATION_COMPONENT: component, SEMIO_PREPARATION_IMAGE: join(model, "📦️actor.semio") },
  stdout: "inherit", stderr: "inherit",
});
const descriptor = Bun.spawn([emitter, "describe", component, "--out", model], { stdout: "inherit", stderr: "inherit" });
const outcomes = await Promise.all([image.exited, descriptor.exited]);
if (outcomes.some(code => code !== 0)) throw Error("The traced Note image or genuine descriptor was refused");
const receipt = Bun.spawn([process.execPath, join(import.meta.dirname, "mcp-verify-retained-note-image.ts"), model], { stdout: "inherit", stderr: "inherit" });
if (await receipt.exited) throw Error("The traced Note preparation receipt was refused");
console.log("[DEBUG] traced Note is ready", model);
