import { existsSync, readFileSync, statSync, readdirSync } from "node:fs";
import { join } from "node:path";

/** 🧾️ Settles every natively emitted plugin descriptor pair (`🗑️generated/described/<plugin-id>/`, written by `refresh-plugin-descriptors.sh`) into its committed owner through the settle step `repo run workspace:components -- describe` runs after its wasm build.
 * Usage from the workspace root: `bun settle-plugin-descriptors.ts <ticket directory> <write|verify>`; prints one `plugin|status|before|after` row per component.
 * @see ./🗃️plugin-descriptors-refresh.md */

const root = process.cwd();
const [ticket, mode] = process.argv.slice(2) as [string, "write" | "verify"];
const describe = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe");
const { componentStageTargetV1, settleComponentDescriptorV1 } = await import(join(describe, "🧩️component-stage/🟦️.ts"));
const { verifySemanticDescriptorPairBytesV1 } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts"));
const rows = readFileSync(join(ticket, "plugin-descriptors-components.txt"), "utf8").split("\n").filter(Boolean).map((line) => line.split("|") as [string, string]);
for (const [crate, directory] of rows) {
  if (crate.includes("stdio-pdf-component")) continue;
  const target = componentStageTargetV1(root, join(directory, "📦️packages/🦀️rust/Cargo.toml"));
  const emitted = join(ticket, "🗑️generated/described", target.pluginId);
  if (!existsSync(emitted)) { console.log(`${target.pluginId}|NOT-EMITTED|-|-`); continue; }
  const jsonBytes = readFileSync(join(emitted, "🔣️.json")), packBytes = readFileSync(join(emitted, "🛂️.descriptor.semio"));
  const pair = verifySemanticDescriptorPairBytesV1(jsonBytes, packBytes);
  if (pair.pluginId !== target.pluginId) throw new Error(`${emitted} describes ${pair.pluginId}`);
  const size = (name: string): number => { const path = join(target.ownerRoot, name); return existsSync(path) ? statSync(path).size : 0; };
  const before = `${size("🔣️.json")}/${size("🛂️.descriptor.semio")}`;
  if (mode === "write") {
    const drift = settleComponentDescriptorV1(root, target, { profile: "prototype", directory: emitted }, { jsonBytes, packBytes }, "write");
    console.log(`${target.pluginId}|${drift.length ? "rewrote" : "current"}|${before}|${size("🔣️.json")}/${size("🛂️.descriptor.semio")}`);
  } else {
    const same = Buffer.compare(jsonBytes, readFileSync(join(target.ownerRoot, "🔣️.json"))) === 0 && Buffer.compare(packBytes, readFileSync(join(target.ownerRoot, "🛂️.descriptor.semio"))) === 0;
    console.log(`${target.pluginId}|${same ? "equal" : "STALE"}|${before}|${jsonBytes.byteLength}/${packBytes.byteLength}`);
  }
}
