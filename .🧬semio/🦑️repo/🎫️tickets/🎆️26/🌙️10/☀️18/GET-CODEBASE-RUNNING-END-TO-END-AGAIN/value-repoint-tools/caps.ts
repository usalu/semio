import { readFileSync } from "node:fs";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { readRustSourceClosureV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const os = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os", workspace = "/Users/ueli/Documents/semio";
const read = (p: string) => readFileSync(resolve(os, p), "utf8");
const corpus = JSON.parse(read("🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json"));
const cuts = JSON.parse(read("🧪️tests/🧫️fixtures/📦️crate-boundaries/🧬️cuts.json"));
const cfg = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix", "test"];
for (const law of cuts.cuts) {
  const producer = corpus.crates.find((r: any) => r.package === law.producer);
  const inputs = await readRustSourceClosureV1([resolve(os, producer.entry)], workspace, cfg, async (p: string) => await readFile(p).catch((e: any) => (e.code === "ENOENT" ? null : Promise.reject(e))));
  const b = inputs.reduce((s: number, r: any) => s + r.bytes, 0), l = inputs.reduce((s: number, r: any) => s + r.activeLines, 0);
  if (b > 262144 * 0.9 || inputs.length > 28 || l > 3600) console.log(law.producer, inputs.length, b, (b / 262144).toFixed(3), l);
}
{
  const G = `${import.meta.dir}/../🗑️generated/value-repoint`;
  const ledger: Record<string, string> = JSON.parse(readFileSync(`${G}/backup/ledger.json`, "utf8"));
  const law = cuts.cuts.find((l: any) => l.producer === "semio-framework-os-store-one-item-batch");
  const producer = corpus.crates.find((r: any) => r.package === law.producer);
  const inputs = await readRustSourceClosureV1([resolve(os, producer.entry)], workspace, cfg, async (p: string) => await readFile(p).catch((e: any) => (e.code === "ENOENT" ? null : Promise.reject(e))));
  let delta = 0, n = 0;
  for (const i of inputs as any[]) {
    const p = i.path ?? i.relativePath;
    const abs = p.startsWith("/") ? p : resolve(workspace, p);
    const h = ledger[abs];
    if (h) { delta += readFileSync(abs).length - readFileSync(`${G}/backup/${h}`).length; n++; }
  }
  console.log("closure files", inputs.length, "edited by codemod", n, "delta bytes", delta);
}
