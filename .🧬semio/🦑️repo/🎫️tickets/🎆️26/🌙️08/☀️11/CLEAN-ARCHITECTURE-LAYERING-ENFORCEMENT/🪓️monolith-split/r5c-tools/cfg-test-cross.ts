// 🔍️ Finds cfg(test) items (and pub items of test-only files) of one planned crate that sources of another planned crate name. Usage: <pkg> <lift>
import {readFileSync, existsSync} from "node:fs";
import {resolve} from "node:path";
import {planArtifactCrates} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
import {parseRustSourceTree, identifierSet} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const [pkg, lift] = process.argv.slice(2);
const plan = await planArtifactCrates(ROOT, pkg!, {mutationFamilies: (process.env.MF as any) ?? "auto", lift: (lift ?? "").split(",").filter(Boolean), tests: {fixtures: true, aggregate: true, hoist: true, detach: true}});
const overlay = new Map<string, string>([...plan.overlay.edits, ...plan.overlay.creates]);
const text = (rel: string): string | null => { const abs = resolve(ROOT, rel); const o = overlay.get(abs); if (o !== undefined) return o; return existsSync(abs) ? readFileSync(abs, "utf8") : null; };
const short = (n: string) => n.replace(/^semio-s-artifact-stdio-gltf-/, "");
type Decl = {name: string; file: string; why: string};
const declsOf = new Map<string, Decl[]>();
const idsOf = new Map<string, Set<string>>();
for (const c of plan.crates) {
  const lib = new Set(c.lib?.paths ?? []);
  const decls: Decl[] = [];
  const ids = new Set<string>();
  for (const rel of c.test?.paths ?? []) {
    if (!rel.endsWith(".rs")) continue;
    const t = text(rel); if (t === null) continue;
    for (const i of identifierSet(t)) ids.add(i);
    const tree = parseRustSourceTree(t);
    const testOnlyFile = !lib.has(rel);
    const visit = (items: any[], mods: any[]) => {
      for (const it of items) if (it.name && it.kind !== "use" && it.kind !== "impl") {
        const gated = it.attributeTexts.some((a: string) => /^cfg\s*\(\s*test\s*\)$/u.test(a));
        if ((gated || testOnlyFile) && it.visibility.startsWith("pub")) decls.push({name: it.name, file: rel, why: gated ? "cfg(test) item" : "pub item of a test-only file"});
      }
      for (const m of mods) {
        const gated = m.conditionalTest;
        if ((gated || testOnlyFile) && m.visibility.startsWith("pub") && m.name !== "tests") decls.push({name: m.name, file: rel, why: gated ? "cfg(test) module" : "pub module of a test-only file"});
        visit(m.items, m.children);
      }
    };
    visit(tree.items, tree.modules);
  }
  declsOf.set(c.name, decls); idsOf.set(c.name, ids);
}
let found = 0;
for (const a of plan.crates) for (const d of declsOf.get(a.name)!) {
  if (d.name.length < 5 || ["tests", "unit"].includes(d.name)) continue;
  for (const b of plan.crates) if (b.id !== a.id && idsOf.get(b.name)!.has(d.name)) { found++; console.log(`${short(a.name)} -> ${short(b.name)}: ${d.name} (${d.why}) @ ${d.file.split("/").slice(-3).join("/")}`); }
}
console.log("pairs", found);
