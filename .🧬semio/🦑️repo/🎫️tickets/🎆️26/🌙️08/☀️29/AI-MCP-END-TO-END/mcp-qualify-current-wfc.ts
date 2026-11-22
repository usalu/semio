import { mkdir, readFile, stat } from "node:fs/promises";
import { join, resolve } from "node:path";
import { stripCoreNamesV1 } from "./mcp-native-wasm-names.ts";

const generated = resolve(import.meta.dirname, "🗑️generated");
const [log, emitter, directory = "m-pure", linkedComponent] = process.argv.slice(2);
if (!log || !emitter || !/^m-[a-z-]+$/u.test(directory)) throw Error("Specific WFC compiler log, descriptor emitter, and generated model directory required");
const deadline = Date.now() + 90 * 60_000;
while (true) {
  const state = await readFile(log, "utf8");
  if (/^error:/m.test(state)) throw Error("The WFC compiler reported a failure");
  if (/^    Finished .+ target\(s\)/m.test(state)) break;
  if (Date.now() >= deadline) throw Error("The WFC compiler did not finish within the observation window");
  await Bun.sleep(6_000);
}
const raw = linkedComponent ? resolve(linkedComponent) : join(generated, "native-guests", "wasm32-wasip2", "debug", "semio_hub_wfc_component.wasm");
const oracle = Bun.spawn([process.execPath, join(import.meta.dirname, "mcp-native-preparation-oracle.ts"), raw], { stdout: "inherit", stderr: "inherit" });
if (await oracle.exited) throw Error("Exact linked WFC still needs import-free preparation");

const componentFolder = join(generated, directory, "browser", "prototype", "components", "f0caf2db1142c1ba93f50ac6");
await mkdir(componentFolder, { recursive: true });
const wasm = join(componentFolder, "📦️component.wasm");
const maximum = 512 * 1024 ** 2;
const removed = (await stat(raw)).size > maximum ? await stripCoreNamesV1(raw, wasm, maximum) : 0;
if (!removed) await Bun.write(wasm, Bun.file(raw));
console.log("[DEBUG] removed", removed, "verified optional WFC names");
const preparation = "mcp-core-preparation-law" + (process.platform === "win32" ? ".exe" : "");
const image = Bun.spawn([join(generated, preparation), "--ignored", "--nocapture"], {
  env: { ...process.env, SEMIO_PREPARATION_COMPONENT: wasm, SEMIO_PREPARATION_IMAGE: join(componentFolder, "📦️actor.semio") },
  stdout: "inherit", stderr: "inherit",
});
const descriptor = Bun.spawn([emitter, "describe", wasm, "--out", componentFolder], { stdout: "inherit", stderr: "inherit" });
const outcomes = await Promise.all([image.exited, descriptor.exited]);
if (outcomes.some(code => code !== 0)) throw Error("Exact WFC descriptor or prepared image was refused");
console.log("[DEBUG] exact WFC image and descriptor produced", componentFolder);
