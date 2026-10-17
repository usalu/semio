// 🧮️ c4p13 gate census: live source closure of the eight FEM engine crates lib/test through the canonical counter (files / bytes / active lines; budget 32 / 262,144 / 4,000).
import {readFileSync} from "node:fs";
import {dirname, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split";
const index = JSON.parse(readFileSync(F + "/🗑️generated/c4p13-manifest-index.json", "utf8")) as Record<string, string>;
const names = ["semio-s-fem-elements2d", "semio-s-fem-sparse-subspace", "semio-s-fem-sparse-pcg", "semio-s-fem-sparse-ldlt", "semio-s-fem-sparse", "semio-s-fem-mesh", "semio-s-fem-analyses-assembly", "semio-s-fem-elements3d"];
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const readFile = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const count = async (manifest: string, withTest: boolean) => {
  const cfg = [...hostCfg, "debug_assertions", ...(withTest ? ["test"] : [])];
  const dir = dirname(manifest);
  const parsed = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
  const entry = resolve(dir, parsed.lib?.path ?? "src/lib.rs");
  const inputs = await readRustSourceClosureV1([entry], ROOT, cfg, readFile, {manifestDirectory: dir, generated: () => undefined});
  return `${inputs.length}/${inputs.reduce((s: number, i: any) => s + i.bytes, 0)}/${inputs.reduce((s: number, i: any) => s + i.activeLines, 0)}`;
};
for (const name of names) {
  const manifest = resolve(ROOT, index[name]!);
  console.log(JSON.stringify({name, lib: await count(manifest, false), test: await count(manifest, true), target: true}));
}
