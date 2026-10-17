// 📐️ Measures virtual variants of the generation3d-app-laws test root with the canonical reader. usage: bun measure.ts <spec.json>; spec = {crateDir, gen3dDir, variants:[{name, root, files:{relToGen3d:text}}]}. Read-only: nothing is written.
import {readFileSync} from "node:fs";
import {resolve, join} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const spec = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as {crateDir: string; gen3dDir: string; variants: {name: string; root: string; files: Record<string, string>}[]};
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const cfg = [...hostCfg, "debug_assertions", "test"];
const out: any[] = [];
for (const v of spec.variants) {
  const overrides = new Map<string, string>();
  overrides.set(join(spec.gen3dDir, "🦀️.rs"), v.root);
  for (const [rel, text] of Object.entries(v.files)) overrides.set(join(spec.gen3dDir, rel), text);
  const read = async (p: string) => { if (overrides.has(p)) return new TextEncoder().encode(overrides.get(p)!); try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
  try {
    const inputs = await readRustSourceClosureV1([join(spec.gen3dDir, "🦀️.rs")], ROOT, cfg, read, {manifestDirectory: spec.crateDir, generated: () => undefined});
    const src = inputs.filter(i => i.kind === "source"), emb = inputs.filter(i => i.kind === "embedded");
    out.push({name: v.name, files: inputs.length, bytes: inputs.reduce((s, i) => s + i.bytes, 0), lines: inputs.reduce((s, i) => s + i.activeLines, 0), srcFiles: src.length, srcBytes: src.reduce((s, i) => s + i.bytes, 0), embFiles: emb.length, embBytes: emb.reduce((s, i) => s + i.bytes, 0), paths: inputs.map(i => i.path)});
  } catch (e) { out.push({name: v.name, error: String((e as Error).message)}); }
}
console.log(JSON.stringify(out));
