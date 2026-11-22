import { blake3 } from "@noble/hashes/blake3.js";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { CargoProvenanceSessionV1, sharedProvenanceCustodyV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts";
import { cargoProvenancePhysicalControlV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts";

const units = Number(process.argv[2] ?? 100), sources = Number(process.argv[3] ?? 40), base = process.env.PROV_BENCH_DIR ?? "G:/t/prov";
mkdirSync(base, { recursive: true });
const root = mkdtempSync(join(base, "b-")), build = join(root, "build"), src = join(root, "src");
mkdirSync(build, { recursive: true });
mkdirSync(src, { recursive: true });
const manifest = join(root, "Cargo.toml");
writeFileSync(manifest, '[workspace]\n[package]\nname="synthetic"\nversion="0.0.0"\n');
const list = [];
for (let unit = 0; unit < units; unit++) {
  const dir = join(src, `u${unit}`);
  mkdirSync(dir, { recursive: true });
  const files = [];
  for (let index = 0; index < sources; index++) {
    const path = join(dir, `m${index}.rs`), bytes = Buffer.from(`pub const V${index}: u32 = ${unit * 1000 + index};\n`.repeat(20));
    writeFileSync(path, bytes);
    files.push({ path, bytes });
  }
  const artifact = join(build, `libunit${unit}.rlib`);
  writeFileSync(artifact, Buffer.alloc(200_000, unit % 251));
  writeFileSync(join(build, `libunit${unit}.d`), `${artifact}: ${files.map((file) => file.path).join(" ")}\n${files.map((file) => `# checksum:blake3=${Buffer.from(blake3(file.bytes)).toString("hex")} file_len:${file.bytes.length} ${file.path}`).join("\n")}\n`);
  list.push({ message: { reason: "compiler-artifact", package_id: `path+file:///synthetic#unit${unit}@0.0.0`, manifest_path: manifest, target: { name: `unit${unit}`, kind: ["lib"], src_path: files[0]!.path }, filenames: [artifact] }, evidence: { version: 1, kind: "discovery", paths: [], producer: null } });
}
const invocation = { manifest, cwd: root, command: "cargo", args: ["build"], buildDirectory: build, builtAtMs: Date.now(), status: 0, cancelled: false, units: list, buildScripts: [] };
const control = () => cargoProvenancePhysicalControlV1({ cancelled: () => false, onProgress: () => undefined });
const custody = sharedProvenanceCustodyV1(root, control);
const session = new CargoProvenanceSessionV1(custody);
const started = performance.now(), cpu0 = process.cpuUsage();
await session.write(join(root, "r", "receipt.json"), invocation as any, new Map(), join(root, "cargo-home"));
const written = performance.now();
await session.close();
console.log(JSON.stringify({ units, sources, cpuMs: Math.round((process.cpuUsage(cpu0).user + process.cpuUsage(cpu0).system) / 1000), writeMs: Math.round(written - started), closeMs: Math.round(performance.now() - written) }));
