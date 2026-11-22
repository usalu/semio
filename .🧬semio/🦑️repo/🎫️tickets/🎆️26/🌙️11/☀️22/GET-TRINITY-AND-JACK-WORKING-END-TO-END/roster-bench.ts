import { join, resolve } from "node:path";
import { completedCargoClosureRosterV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/📋️inventory/🟦️.ts";

const units = Number(process.argv[2] ?? 100), sources = Number(process.argv[3] ?? 40), scripts = Math.floor(units / 4), resources = Math.floor(units / 6);
const root = resolve("G:/t/rb"), build = join(root, "build"), target = join(root, "target"), manifest = join(root, "Cargo.toml");
const digest = (value: number): string => value.toString(16).padStart(64, "0");
const observedUnits: any[] = [], buildScripts: any[] = [], buildResources: any[] = [], compilerResources: any[] = [];
for (let unit = 0; unit < units; unit++) {
  const custom = unit < scripts, id = `path+file:///s#u${unit}@0.0.0`, dir = join(root, "src", `u${unit}`), artifact = join(build, `libu${unit}.rlib`), dep = join(build, `libu${unit}.d`);
  const inputs = Array.from({ length: sources }, (_, index) => ({ path: join(dir, `m${index}.rs`), kind: "file" as const, sha256: digest(unit * 1000 + index + 1) }));
  const message = { package_id: id, manifest_path: manifest, target: { name: `u${unit}`, kind: [custom ? "custom-build" : "lib"], src_path: inputs[0]!.path }, filenames: [artifact] };
  observedUnits.push({ message, observedAtMs: 0, depInfo: [{ path: dep, sha256: digest(900000 + unit), baseDirectory: root, sources: inputs.map((row) => row.path), checksums: [] }], inputs, artifacts: [{ path: artifact, sha256: digest(800000 + unit) }] });
  if (custom) {
    const out = join(build, "build", `u${unit}`, "out");
    buildScripts.push({ package_id: id, out_dir: out });
    buildResources.push({ package_id: id, out_dir: out, path: join(out, "inputs.jsonl"), sha256: digest(700000 + unit), observedAtMs: 0, resources: [] });
  }
}
for (let index = 0; index < resources; index++) compilerResources.push({ path: join(build, "semio-compiler-resources", `r${index}`, "observation.json"), sha256: digest(600000 + index), observedAtMs: 0, producer: null, caller: null, producerUnit: observedUnits[scripts + index]!.message, callerUnit: observedUnits[scripts + index + 1]!.message, resources: [] });
const observation: any = { version: 1, manifest, cwd: root, command: "cargo", args: [], buildDirectory: build, builtAtMs: 0, status: 0, cancelled: false, units: observedUnits, buildScripts, observedAtMs: 0, invocationInputs: [{ path: manifest, sha256: digest(1) }], buildResources, compilerResourceRoot: null, compilerResources };
const origin = { version: 1, workspace: root, mode: "prototype", manifest, cwd: root, buildDirectory: build, targetDirectory: target, cargoHome: join(root, "cargo") };
const started = performance.now();
const rows = completedCargoClosureRosterV1(observation, origin);
console.log(JSON.stringify({ units, sources, rows: rows.length, ms: Math.round(performance.now() - started) }));
