/** 🔍️ W1: per component, committed descriptor wasm hash == dist/component-dev bytes == shared wasm-dev bytes ==
 * staged module descriptor hash, and the s activation receipt lists it. Prints one row per component. */
import { createHash } from "node:crypto";
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, join } from "node:path";

const repo = "/Users/ueli/Documents/semio";
const sha = (path: string): string => (existsSync(path) ? createHash("sha256").update(readFileSync(path)).digest("hex") : "-");
const target = join(repo, ".🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2/wasm-dev");
const modules = join(repo, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules");
const receiptPath = join(repo, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/react/dev/s/activation/🔣️receipt.json");
const receipt = existsSync(receiptPath) ? (JSON.parse(readFileSync(receiptPath, "utf8")) as { plugins: { pluginId: string; artifactSha256: string }[] }) : { plugins: [] };
const activated = new Set(receipt.plugins.map((row) => row.pluginId));
const staged = new Map<string, string>();
for (const entry of readdirSync(modules)) {
  const json = join(modules, entry, "🔣️.json");
  if (!existsSync(json)) continue;
  const doc = JSON.parse(readFileSync(json, "utf8"));
  staged.set(doc.manifest?.pluginId, doc.hashes?.wasmSha256);
}
const owners: string[] = [];
const walk = (dir: string, depth: number): void => {
  if (depth > 5) return;
  for (const entry of readdirSync(dir)) {
    if (entry === "dist" || entry === "node_modules" || entry.startsWith(".") || entry.includes("🗿️artifacts")) continue;
    const path = join(dir, entry);
    if (!statSync(path).isDirectory()) continue;
    if (existsSync(join(path, "🛂️.descriptor.semio")) && existsSync(join(path, "📦️packages/🦀️rust/Cargo.toml"))) owners.push(path);
    walk(path, depth + 1);
  }
};
walk(join(repo, "✏️s/🔌️plugins"), 0);
let bad = 0;
const rows: string[] = [];
for (const owner of owners.sort()) {
  const doc = JSON.parse(readFileSync(join(owner, "🔣️.json"), "utf8"));
  const cargo = readFileSync(join(owner, "📦️packages/🦀️rust/Cargo.toml"), "utf8").match(/^name\s*=\s*"([^"]+)"/m)![1]!;
  const file = `${cargo.replaceAll("-", "_")}.wasm`;
  const pluginId = doc.manifest?.pluginId as string;
  const committed = doc.hashes?.wasmSha256 as string;
  const dist = sha(join(owner, "📦️packages/🦀️rust/dist/component-dev", file));
  const shared = sha(join(target, file));
  const stagedHash = staged.get(pluginId) ?? "-";
  const ok = committed === dist && dist === shared && shared === stagedHash && activated.has(pluginId);
  if (!ok) bad += 1;
  rows.push(`${ok ? "OK  " : "DIFF"} ${pluginId.padEnd(34)} committed=${committed?.slice(0, 12)} dist=${dist.slice(0, 12)} shared=${shared.slice(0, 12)} staged=${stagedHash?.slice(0, 12)} activated=${activated.has(pluginId)}`);
}
console.log(rows.join("\n"));
console.log(`components=${owners.length} consistent=${owners.length - bad} diverged=${bad} receiptPlugins=${receipt.plugins.length} receipt=${receiptPath}`);
process.exit(bad === 0 ? 0 : 1);
