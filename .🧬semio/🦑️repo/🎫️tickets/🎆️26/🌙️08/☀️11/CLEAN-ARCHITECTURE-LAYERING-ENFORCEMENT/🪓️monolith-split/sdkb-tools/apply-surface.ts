/** 🧱️ Writes the planned SDK facades and package `exports`. Usage: bun apply-surface.ts <plan.json> [--write]. Dry-run by default. */
import { readFileSync, writeFileSync, existsSync, mkdirSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { ROOT, SDK_DIR } from "./lib.ts";

const [planPath, flag] = process.argv.slice(2);
const write = flag === "--write";
const plan = JSON.parse(readFileSync(planPath, "utf8"));
const pkgPath = join(ROOT, SDK_DIR, "package.json");
const pkgText = readFileSync(pkgPath, "utf8");
const pkg = JSON.parse(pkgText);

function emojiOf(seg: string): string { const m = seg.match(/^(?:\p{Extended_Pictographic}(?:️|‍\p{Extended_Pictographic}️?)*)/u); return m ? m[0] : "🔌️"; }
function relImport(facade: string, target: string): string {
  const r = relative(join(ROOT, dirname(facade)), join(ROOT, target)).normalize("NFC");
  return r.startsWith(".") ? r : "./" + r;
}

let created = 0, extended = 0, added = 0;
const touched: string[] = [];
const collisions: string[] = [];
for (const f of plan.facades) {
  if (!f.adds.length) continue;
  const abs = join(ROOT, f.facade);
  const exists = existsSync(abs);
  const lines: string[] = [];
  const seen = new Map<string, string>();
  let prior = exists ? readFileSync(abs, "utf8") : "";
  for (const a of f.adds) {
    const from = relImport(f.facade, a.from);
    const values = a.names.filter((n: any) => n.value).map((n: any) => n.name).sort();
    const types = a.names.filter((n: any) => !n.value).map((n: any) => n.name).sort();
    for (const n of [...values, ...types]) { if (seen.has(n) && seen.get(n) !== a.from) collisions.push(`${f.subpath}: ${n} from ${seen.get(n)} and ${a.from}`); seen.set(n, a.from); }
    for (const j of a.json) { if (seen.has(j.exportName)) collisions.push(`${f.subpath}: ${j.exportName} duplicate`); seen.set(j.exportName, j.file); }
    if (values.length) lines.push(`export { ${values.join(", ")} } from "${from}";`);
    if (types.length) lines.push(`export type { ${types.join(", ")} } from "${from}";`);
    for (const j of a.json) lines.push(`export { default as ${j.exportName} } from "${relImport(f.facade, j.file)}"${j.attr ? ` with { type: "${j.attr}" }` : ""};`);
  }
  if (exists) {
    const missing = lines.filter(l => !prior.includes(l));
    if (!missing.length) continue;
    const next = prior.replace(/\n*$/, "\n") + missing.join("\n") + "\n";
    if (write) writeFileSync(abs, next);
    extended++; touched.push(f.facade);
  } else {
    const segs = f.facade.slice(SDK_DIR.length + 1).split("/");
    const leaf = segs[segs.length - 2] ?? segs[0];
    const emoji = emojiOf(leaf);
    const doc = `/** ${emoji} SDK entry \`@semio-tech/framework${f.subpath.slice(1)}\`: explicit public surface of the framework module that plugins build on. */\n`;
    if (write) { mkdirSync(dirname(abs), { recursive: true }); writeFileSync(abs, doc + lines.join("\n") + "\n"); }
    created++; touched.push(f.facade);
  }
}

const exportsOut: Record<string, string> = { ...pkg.exports };
for (const f of plan.facades) {
  if (f.subpath === ".") continue;
  const target = "./" + f.facade.slice(SDK_DIR.length + 1);
  if (!(f.subpath in exportsOut)) { exportsOut[f.subpath] = target; added++; }
  else if (exportsOut[f.subpath] !== target) collisions.push(`exports ${f.subpath}: ${exportsOut[f.subpath]} vs ${target}`);
}
const existingKeys = Object.keys(pkg.exports);
const newKeys = Object.keys(exportsOut).filter(k => !existingKeys.includes(k)).sort();
pkg.exports = Object.fromEntries([...existingKeys.map(k => [k, exportsOut[k]]), ...newKeys.map(k => [k, exportsOut[k]])]);
if (write) writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + (pkgText.endsWith("\n") ? "\n" : ""));
console.log(`${write ? "WROTE" : "DRY"}: created=${created} extended=${extended} newExports=${added} collisions=${collisions.length}`);
for (const c of collisions) console.log("COLLISION", c);
