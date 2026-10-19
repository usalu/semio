import { readFileSync } from "node:fs";
import { join } from "node:path";
import { readRustSourceClosureV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { crateTreeHostCfg } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const [manifestDirArg, modeArg = "both", ...extra] = process.argv.slice(2);
const manifestDir = manifestDirArg!, mode = modeArg;
const toml = readFileSync(join(ROOT, manifestDir, "Cargo.toml"), "utf8");
const libPath = /\[lib\][^\[]*?path\s*=\s*"([^"]+)"/u.exec(toml)?.[1] ?? "src/lib.rs";
const entry = join(manifestDir, libPath).split("/").reduce((a: string[], p) => (p === ".." ? (a.pop(), a) : p === "." ? a : (a.push(p), a)), [] as string[]).join("/");
const verbose = process.env.V === "1";
const host = [...crateTreeHostCfg(), ...extra.map((f) => `feature="${f}"`)];
const read = async (abs: string) => { try { return readFileSync(abs); } catch { return null; } };
for (const m of mode === "both" ? ["prod", "test"] : [mode]) {
  const cfg = m === "test" ? [...host, "test"] : host;
  try {
    const inputs = await readRustSourceClosureV1([join(ROOT, entry)], ROOT, cfg, read, { manifestDirectory: join(ROOT, manifestDir), generated: () => undefined });
    let b = 0, l = 0;
    console.log(`== ${m}`);
    for (const i of inputs) { b += i.bytes; l += i.activeLines; if (verbose) console.log(String(i.bytes).padStart(7), String(i.activeLines).padStart(5), (i as any).path ?? (i as any).relativePath ?? JSON.stringify(Object.keys(i))); }
    console.log(`TOTAL files=${inputs.length} bytes=${b} (${(b/262144).toFixed(3)}) lines=${l} (${(l/4000).toFixed(3)}) files/32=${(inputs.length/32).toFixed(3)}`);
  } catch (e) { console.log(m, "ERR", String((e as Error).message).slice(0, 300)); }
}
