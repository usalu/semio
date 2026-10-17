// 🧰 bd9: per-crate Rust source closure (canonical reader) for the self-testkit crates; prints JSON {crate: {manifestDir, files:[...]}}
import {readFileSync, writeFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {dirname, resolve} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
const list = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as [string, string, string][];
const out = process.argv[3]!;
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(value => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));
const readFile = async (path: string): Promise<Uint8Array | null> => {
  try { return readFileSync(path); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
};
const result: Record<string, {manifestDir: string; lib: string; files: string[]; error?: string}> = {};
for (const [manifest, name] of list) {
  const dir = resolve(root, dirname(manifest));
  const toml = Bun.TOML.parse(readFileSync(resolve(root, manifest), "utf8")) as {lib?: {path?: string}; test?: {path?: string}[]};
  const lib = resolve(dir, toml.lib?.path ?? "src/lib.rs");
  const files = new Set<string>();
  let error: string | undefined;
  const roots = [lib, ...(toml.test ?? []).map(t => resolve(dir, t.path ?? ""))];
  for (const entry of roots) {
    for (const test of [false, true]) {
      try {
        const cfg = [...hostCfg, "debug_assertions", 'feature="testkit"', ...(test ? ["test"] : [])];
        const inputs = await readRustSourceClosureV1([entry], root, cfg, readFile, {manifestDirectory: dir, generated: () => undefined}) as unknown as {path?: string; file?: string}[];
        for (const input of inputs) files.add(resolve(root, (input.path ?? input.file) as string));
      } catch (e) { error = String((e as Error).message ?? e).slice(0, 200); }
    }
  }
  result[name] = {manifestDir: dir, lib, files: [...files].sort(), error};
}
writeFileSync(out, JSON.stringify(result, null, 1));
console.log(Object.entries(result).map(([n, v]) => `${n} ${v.files.length} ${v.error ?? ""}`).join("\n"));
