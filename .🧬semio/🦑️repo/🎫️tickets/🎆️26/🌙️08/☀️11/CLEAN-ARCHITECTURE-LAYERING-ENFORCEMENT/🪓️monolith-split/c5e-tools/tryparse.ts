import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { parseCargoToml, CargoTomlWorkspace } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📖️toml/🟦️.ts";
import smol from "/Users/ueli/Documents/semio/node_modules/smol-toml/dist/index.js";
const repo = "/Users/ueli/Documents/semio";
const lock = readFileSync(join(repo, "🎓️teaching/Cargo.lock"), "utf8");
const manifests = [...(await Array.fromAsync(new Bun.Glob("🧰️framework/**/📦️packages/🦀️rust/Cargo.toml").scan({ cwd: repo }))) ].slice(0, 60);
let ok = 0, bad = 0; const t0 = Date.now(); let bytes = 0;
for (const m of manifests) {
  const text = readFileSync(join(repo, m), "utf8"); bytes += text.length;
  const ctl = { signal: new AbortController().signal, maximumUnits: 50_000_000, maximumOwnedBytes: 1 << 30, maximumDepth: 64, remainingMilliseconds: () => 60_000, onProgress() {}, yieldContinuation: async () => {} };
  try { const doc = await parseCargoToml(text, new CargoTomlWorkspace(), ctl, m); const ref = smol.parse(text); const a = JSON.stringify(doc), b = JSON.stringify(ref); if (a !== b) { bad++; console.log("DIFF", m); } else ok++; } catch (e) { bad++; console.log("ERR", m, String(e).slice(0, 150)); }
}
console.log({ ok, bad, ms: Date.now() - t0, bytes });
