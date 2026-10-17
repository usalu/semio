// 🔍️ Same scan as cfg-test-cross.ts over the applied crates of a directory tree. Usage: <dir under repo> <name-prefix>
import {readFileSync, existsSync} from "node:fs";
import {dirname, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import {parseRustSourceTree, identifierSet} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const [dir, prefix] = process.argv.slice(2);
const files = execFileSync("git", ["-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "ls-files", "-co", "--exclude-standard", "-z", "--", dir!], {cwd: ROOT, maxBuffer: 1 << 28}).toString().split("\0").filter(p => p.endsWith("/Cargo.toml"));
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string) => { try { return readFileSync(p); } catch { return null; } };
type Decl = {name: string; file: string; why: string};
const crates: {name: string; decls: Decl[]; ids: Set<string>; own: Set<string>}[] = [];
for (const rel of files) {
  const m = resolve(ROOT, rel); const parsed = Bun.TOML.parse(readFileSync(m, "utf8")) as any;
  const name = parsed.package?.name as string | undefined; if (!name || !name.startsWith(prefix!)) continue;
  const entry = resolve(dirname(m), parsed.lib?.path ?? "src/lib.rs"); if (!existsSync(entry)) continue;
  const lib = await readRustSourceClosureV1([entry], ROOT, [...hostCfg, "debug_assertions"], read, {manifestDirectory: dirname(m), generated: () => undefined});
  const test = await readRustSourceClosureV1([entry], ROOT, [...hostCfg, "debug_assertions", "test"], read, {manifestDirectory: dirname(m), generated: () => undefined});
  const libSet = new Set(lib.map(i => i.path)); const decls: Decl[] = []; const ids = new Set<string>(); const own = new Set<string>();
  for (const i of test) {
    if (!i.path.endsWith(".rs")) continue;
    const t = readFileSync(resolve(ROOT, i.path), "utf8"); for (const id of identifierSet(t)) ids.add(id);
    const tree = parseRustSourceTree(t); const only = !libSet.has(i.path);
    const collect = (items: any[], mods: any[]) => { for (const it of items) if (it.name) own.add(it.name); for (const mm of mods) { own.add(mm.name); collect(mm.items, mm.children); } }; collect(tree.items, tree.modules);
    const visit = (items: any[], mods: any[]) => {
      for (const it of items) if (it.name && it.kind !== "use" && it.kind !== "impl") { const g = it.attributeTexts.some((a: string) => /^cfg\s*\(\s*test\s*\)$/u.test(a)); if ((g || only) && it.visibility.startsWith("pub")) decls.push({name: it.name, file: i.path, why: g ? "cfg(test) item" : "pub item of a test-only file"}); }
      for (const mm of mods) { const g = mm.conditionalTest; if ((g || only) && mm.visibility.startsWith("pub") && mm.name !== "tests") decls.push({name: mm.name, file: i.path, why: g ? "cfg(test) module" : "pub module of a test-only file"}); visit(mm.items, mm.children); }
    };
    visit(tree.items, tree.modules);
  }
  crates.push({name, decls, ids, own});
}
let n = 0;
for (const a of crates) for (const d of a.decls) { if (d.name.length < 5) continue; for (const b of crates) if (b !== a && b.ids.has(d.name) && !b.own.has(d.name)) { n++; console.log(`${a.name} -> ${b.name}: ${d.name} (${d.why}) @ ${d.file.split("/").slice(-3).join("/")}`); } }
console.log("crates", crates.length, "pairs", n);
