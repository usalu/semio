import { readFileSync } from "node:fs";
import { join } from "node:path";
import { readRustSourceClosureV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { crateTreeHostCfg } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const dirs: string[] = JSON.parse(readFileSync(0, "utf8"));
const host = [...crateTreeHostCfg()];
const read = async (abs: string) => { try { return readFileSync(abs); } catch { return null; } };
const out: Record<string, { prod: string[]; test: string[]; err?: string }> = {};
for (const dir of dirs) {
  try {
    const toml = readFileSync(join(ROOT, dir, "Cargo.toml"), "utf8");
    const lib = /\[lib\][^\[]*?path\s*=\s*"([^"]+)"/u.exec(toml)?.[1] ?? "src/lib.rs";
    const entry = join(dir, lib).split("/").reduce((a: string[], p) => (p === ".." ? (a.pop(), a) : p === "." ? a : (a.push(p), a)), [] as string[]).join("/");
    const run = async (cfg: string[]) => (await readRustSourceClosureV1([join(ROOT, entry)], ROOT, cfg, read, { manifestDirectory: join(ROOT, dir), generated: () => undefined })).map((i) => i.path);
    out[dir] = { prod: await run(host), test: await run([...host, "test"]) };
  } catch (e) { out[dir] = { prod: [], test: [], err: String((e as Error).message).slice(0, 120) }; }
}
console.log(JSON.stringify(out));
