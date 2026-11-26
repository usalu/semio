import { readdirSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import { stageArtifacts } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts";
import { machinePathV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import { activationFilesDigest } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🔖️identity/🟦️.ts";
import { developmentRuntimeRoot } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";

const base = "semio_hub_procedural_component";
const required = [base + ".js",base + ".core.wasm","🌉️bridge.js","🟨️.js","🛂️.descriptor.semio","🔣️.json"];
const renderer = process.argv[2];
if (renderer !== "react" && renderer !== "wgpu") throw Error("Expected react or wgpu");
if (!process.argv[3]) throw Error("Expected a probed procedural browser module directory");
const source = resolve(process.argv[3]);
const workspace = process.cwd();
const runtime = process.argv[4] ? resolve(process.argv[4]) : developmentRuntimeRoot(workspace,"generation3d","prototype",renderer);
const destination = join(runtime,"components","🌀️procedural");
const controller = new AbortController();
for (const event of ["SIGINT","SIGTERM"] as const) process.once(event, () => controller.abort());
const names = new Set(required);
for (const entry of readdirSync(source)) if (/^semio_hub_procedural_component\.core[0-9]+\.wasm$/.test(entry)) names.add(entry);
const files = (root: string) => new Map([...names].map(name => {
  const file = join(root,name);
  const stat = statSync(file);
  if (!stat.isFile() || stat.size === 0) throw Error("Incomplete development package at " + file);
  return [name,file] as const;
}));
const candidate = files(source);
const identity = await activationFilesDigest(candidate,controller.signal);
await stageArtifacts(destination,"development-component:procedural",candidate,{
  leaseDirectory: machinePathV1(workspace,".cache/b/l/m/data"), signal: controller.signal,
});
if (await activationFilesDigest(files(destination),controller.signal) !== identity) throw Error("Published procedural bytes changed");
console.log("[DEBUG] Verified local procedural module",identity,destination);
