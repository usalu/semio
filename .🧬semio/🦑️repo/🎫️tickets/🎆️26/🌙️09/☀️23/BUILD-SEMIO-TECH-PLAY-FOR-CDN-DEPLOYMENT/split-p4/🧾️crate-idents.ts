#!/usr/bin/env bun
/**
 * 🧾️ Lists, per crate root of a cut config, the crate-path identifiers its cfg-selected sources name (`ident::`, `extern crate
 * ident`, `use ident`): `lib` for the native lib cfg, `test` for what only the test cfg adds. Comments and strings are ignored.
 *
 * usage: bun 🧾️crate-idents.ts <cut.json> > idents.json
 */
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { readRustSourceClosureV1, selectRustSourceV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const REPO = resolve(import.meta.dir, "../../../../../../../..");
const NATIVE = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix", "debug_assertions", "panic=unwind"];
const config = JSON.parse(await readFile(process.argv[2]!, "utf8"));
const rows = [...config.crates, { package: config.aggregator, root: config.root }];
const strip = (text: string) => text.replace(/\/\*[\s\S]*?\*\//g, " ").replace(/\/\/[^\n]*/g, " ").replace(/r#*"[\s\S]*?"#*/g, '""').replace(/"(?:\\.|[^"\\])*"/g, '""');
const idents = (text: string) => {
  const out = new Set<string>();
  const t = strip(text);
  for (const m of t.matchAll(/(?<![\w:.])([a-z_][a-z0-9_]*)\s*::/g)) out.add(m[1]!);
  for (const m of t.matchAll(/extern\s+crate\s+([a-z_][a-z0-9_]*)/g)) out.add(m[1]!);
  for (const m of t.matchAll(/\buse\s+([a-z_][a-z0-9_]*)\s*;/g)) out.add(m[1]!);
  return out;
};
const result: Record<string, { lib: string[]; test: string[] }> = {};
const read = async (path: string) => await readFile(path).catch(error => error.code === "ENOENT" ? null : Promise.reject(error));
for (const row of rows) {
  const entry = resolve(REPO, row.root);
  const collect = async (cfg: string[]) => {
    const all = new Set<string>();
    for (const input of await readRustSourceClosureV1([entry], REPO, cfg, read)) {
      if (input.kind !== "source") continue;
      const text = selectRustSourceV1(new TextDecoder().decode((await read(resolve(REPO, input.path)))!), cfg);
      for (const id of idents(text)) all.add(id);
    }
    return all;
  };
  try {
    const lib = await collect(NATIVE), test = await collect([...NATIVE, "test"]);
    result[row.package] = { lib: [...lib].sort(), test: [...test].filter(id => !lib.has(id)).sort() };
  } catch (error) {
    result[row.package] = { lib: [], test: [], error: String(error).slice(0, 200) } as any;
  }
}
console.log(JSON.stringify(result, null, 1));
