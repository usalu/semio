import { appendFileSync, mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const args = process.argv.slice(2), profile = args[args.indexOf("--profile") + 1], manifest = args[args.indexOf("--manifest-path") + 1];
if (args.includes("--unit-graph")) { process.stdout.write(JSON.stringify({ version: 1, units: [], roots: [] })); process.exit(0); }
const packages = args.flatMap((arg, index) => args[index - 1] === "-p" ? [arg] : []), root = manifest.slice(0, -"/Cargo.toml".length);
appendFileSync("C:\\git\\semio\\.🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️04\\SOLVE-ALL-MERGE-CONFLICTS\\🗑️generated\\receipts\\f-KqmaKn\\calls.jsonl", JSON.stringify({ at: Date.now(), pid: process.pid, profile, manifest, jobserver: process.env.CARGO_MAKEFLAGS ?? null, wrapper: process.env.RUSTC_WRAPPER ?? null, unification: process.env.CARGO_RESOLVER_FEATURE_UNIFICATION ?? null }) + "\n");
if (process.env.STUB_MODE === "hang") await new Promise(() => setInterval(() => {}, 1000));
const linked = process.env.STUB_MODE === "omit-last" ? packages.slice(0, -1) : packages;
for (const name of linked) {
  await new Promise((accept) => setTimeout(accept, 400 / packages.length));
  const directory = join(process.env.CARGO_TARGET_DIR, "wasm32-wasip2", profile), wasm = join(directory, name.replaceAll("-", "_") + ".wasm"), source = join(root, name, "lib.rs");
  mkdirSync(directory, { recursive: true });
  writeFileSync(wasm, Buffer.concat([Buffer.from([0,97,115,109,13,0,1,0]), Buffer.from(name + profile)]));
  process.stdout.write(JSON.stringify({ reason: "compiler-artifact", package_id: "path+file://" + root + "/" + name + "#" + name + "@0.1.0", manifest_path: join(root, name, "Cargo.toml"), target: { kind: ["cdylib"], crate_types: ["cdylib"], name, src_path: source }, profile: { test: false }, features: [], filenames: [wasm], executable: null, fresh: false }) + "\n");
}
appendFileSync("C:\\git\\semio\\.🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️04\\SOLVE-ALL-MERGE-CONFLICTS\\🗑️generated\\receipts\\f-KqmaKn\\calls.jsonl", JSON.stringify({ end: Date.now(), profile, manifest }) + "\n");
process.exit(linked.length === packages.length ? 0 : 101);
