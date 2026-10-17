// 🔎️ Finds `#[cfg(test)]` items of one crate that the files of another crate of the same artifact name.
import {readFileSync, existsSync, readdirSync} from "node:fs";
import {dirname, join, relative, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import {parseRustSourceTree, identifierSet} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
const root = resolve(process.argv[2]!);
const art = resolve(root, process.argv[3]!);
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const cfgBase = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const manifests: string[] = [];
const walk = (dir: string) => { for (const e of readdirSync(dir, {withFileTypes: true})) { if (e.name === "node_modules" || e.name === "dist" || e.name.startsWith(".")) continue; const p = join(dir, e.name); if (e.isDirectory()) walk(p); else if (e.name === "Cargo.toml") manifests.push(p); } };
walk(art);
type Crate = {name: string; entry: string; files: Map<string, string>};
const crates: Crate[] = [];
const read = async (p: string) => { try { return readFileSync(p); } catch { return null; } };
for (const m of manifests) {
  const parsed = Bun.TOML.parse(readFileSync(m, "utf8")) as any;
  if (!parsed.package?.name) continue;
  const entry = resolve(dirname(m), parsed.lib?.path ?? "src/lib.rs");
  if (!existsSync(entry)) continue;
  try {
    const inputs = await readRustSourceClosureV1([entry], root, [...cfgBase, "test"], read, {manifestDirectory: dirname(m), generated: () => undefined});
    crates.push({name: parsed.package.name, entry, files: new Map(inputs.filter(i => i.path.endsWith(".rs")).map(i => [i.path, readFileSync(resolve(root, i.path), "utf8")]))});
  } catch (e) { console.error("skip", parsed.package.name, (e as Error).message.slice(0, 100)); }
}
const owner = new Map<string, Set<string>>();
for (const c of crates) for (const f of c.files.keys()) { if (!owner.has(f)) owner.set(f, new Set()); owner.get(f)!.add(c.name); }
const gated = new Map<string, {crate: string; file: string; kind: string}[]>();
for (const c of crates) for (const [file, text] of c.files) {
  const tree = parseRustSourceTree(text);
  const visit = (items: readonly any[]) => { for (const item of items) if (item.name && item.kind !== "use" && item.kind !== "impl" && item.kind !== "macro" && item.visibility.startsWith("pub") && item.attributeTexts.some((a: string) => /cfg\(\s*test\s*\)/u.test(a))) gated.set(item.name, [...(gated.get(item.name) ?? []), {crate: c.name, file, kind: item.kind}]); };
  visit(tree.items);
  const walkMods = (mods: readonly any[]) => { for (const m of mods) { visit(m.items); walkMods(m.children); } };
  walkMods(tree.modules);
}
let hits = 0;
for (const c of crates) for (const [file, text] of c.files) {
  const ids = identifierSet(text);
  for (const [name, defs] of gated) {
    if (!ids.has(name)) continue;
    for (const d of defs) {
      if (d.crate === c.name || owner.get(file)?.has(d.crate)) continue;
      if (defs.some(x => owner.get(file)?.has(x.crate))) continue;
      hits++;
      console.log(`${name} (${d.kind}) defined in ${d.crate} @ ${relative(root, d.file).slice(-70)} used by ${c.name} @ ${file.slice(-70)}`);
    }
  }
}
console.log("crates", crates.length, "cfg(test) pub items", gated.size, "hits", hits);
