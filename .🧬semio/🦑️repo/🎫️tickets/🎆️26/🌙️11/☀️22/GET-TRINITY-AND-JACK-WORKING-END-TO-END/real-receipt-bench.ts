import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { decodeCargoProvenanceV1, encodeCargoProvenanceV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts";
import { completedCargoClosureRosterV1, completedCargoOutputClaimsV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/📋️inventory/🟦️.ts";

const time = <T>(name: string, run: () => T): T => { const start = performance.now(); const value = run(); console.log(name, Math.round(performance.now() - start) + "ms"); return value; };
const text = readFileSync(process.argv[2]!, "utf8");
const observation = time("decode", () => decodeCargoProvenanceV1(text));
console.log("units", observation.units.length, "scripts", observation.buildScripts.length, "resources", observation.buildResources.length, "compilerResources", observation.compilerResources.length, "status", observation.status);
await (async () => { const start = performance.now(); const encoded = await encodeCargoProvenanceV1(observation); console.log("encode", Math.round(performance.now() - start) + "ms", encoded.length); })();
const origin = { version: 1, workspace: resolve(observation.cwd), mode: "prototype", manifest: observation.manifest, cwd: observation.cwd, buildDirectory: observation.buildDirectory!, targetDirectory: resolve(observation.cwd, "target-x"), cargoHome: resolve("C:/Users/Ueli/.cargo") };
const rows = time("roster", () => completedCargoClosureRosterV1(observation, origin));
console.log("rows", rows.length);
time("claims", () => completedCargoOutputClaimsV1(observation, origin));
