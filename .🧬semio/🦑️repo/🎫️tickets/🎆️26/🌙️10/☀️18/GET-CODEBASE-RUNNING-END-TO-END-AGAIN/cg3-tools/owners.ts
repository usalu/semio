import { Universe } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪞️unmirror/🔍️resolver/🟦️.ts";
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { join, posix } from "node:path";
const repo = "/Users/ueli/Documents/semio";
const universe = new Universe(repo);
const walk = (dir: string, out: string[]): string[] => { for (const n of readdirSync(join(repo, dir))) { if (n === "node_modules") continue; const rel = `${dir}/${n}`; if (statSync(join(repo, rel)).isDirectory()) walk(rel, out); else if (n === "Cargo.toml") out.push(rel); } return out; };
for (const manifest of walk("✏️s/🔌️plugins/🏙️bim", [])) {
  const parsed = Bun.TOML.parse(readFileSync(join(repo, manifest), "utf8")) as any;
  const name = parsed.package?.name;
  if (!name?.startsWith("semio-s-artifact-")) continue;
  const dir = posix.dirname(manifest);
  const root = posix.normalize(posix.join(dir, parsed.lib?.path ?? "src/lib.rs"));
  if (existsSync(join(repo, root))) { universe.addCrate(name, dir, root); universe.root(name.replaceAll("-", "_")); }
}
for (const path of process.argv.slice(2)) {
  const crate = universe.files.get(path)?.nodes[0]?.crate;
  const manifest = crate ? readFileSync(join(repo, crate.dir, "Cargo.toml"), "utf8") : "";
  console.log(crate?.name, "has bim dep:", /^semio-s-artifact-bim-model-bim\s*=/m.test(manifest));
}
