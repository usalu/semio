import { existsSync, readdirSync, readFileSync, realpathSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
const root = process.cwd();
const modules = join(root, "🧰️framework/🔨️modules");
const skip = new Set(["node_modules", ".git", ".nx", ".🧬semio", "🗑️generated", "target"]);
const walk = (d: string): string[] => readdirSync(d, { withFileTypes: true }).filter(e => !skip.has(e.name)).flatMap(e => e.isDirectory() ? walk(join(d, e.name)) : e.isFile() ? [join(d, e.name)] : []);
function facade(path: string): string[] | undefined {
  if (!/[\\/]🧫️fixtures(?:[\\/][^\\/]+)*[\\/]🟦️\.ts$/u.test(path)) return undefined;
  const code = readFileSync(path, "utf8").replace(/\/\*[\s\S]*?\*\//gu, "").replace(/(^|[^:])\/\/[^\n]*/gu, "$1");
  const statements = code.split(";").map(s => s.trim()).filter(s => s.length > 0), targets: string[] = [];
  for (const s of statements) { const m = /^export\s+(?:type\s+)?(?:\*|\{[^}]*\})\s*from\s*"([^"]+)"(?:\s+with\s*\{[^}]*\})?$/u.exec(s); if (m === null) return undefined; targets.push(resolve(dirname(path), m[1]!)); }
  return targets.length > 0 && targets.every(t => existsSync(t)) ? targets : undefined;
}
const belongs = (rel: string, forbid: string[]): boolean => { const p = rel.split(/[\\/]/), f = p.indexOf("🧫️fixtures"), t = p.indexOf("🧪️tests"); return t >= 0 && t < f && !p.slice(0, f).some(x => forbid.includes(x)); };
const placedFacade = (path: string, forbid: string[]): boolean => { const t = facade(path); return t !== undefined && t.every(x => belongs(relative(modules, x), forbid)); };
const inFx = (p: string) => p.split(/[\\/]/).includes("🧫️fixtures");
const out: Record<string, string[]> = {};
const A = ["📦️packages", "🎯️targets", "🧩️implementations"], S = ["🧬️schema", ...A];
const own = (owners: string[]) => owners.flatMap(o => walk(join(modules, o))).filter(inFx);
out.value = own(["🌱️value", "⏳️async"]).filter(p => !belongs(relative(modules, p), S) && !placedFacade(p, S)).map(p => relative(root, p));
out.pack = own(["🎒️pack", "📡️replication"]).filter(p => { const t = facade(p); const factOk = t !== undefined && t.every(x => x.split(/[\\/]/).includes("🧪️tests")); const parts = relative(modules, p).split(/[\\/]/), f = parts.indexOf("🧫️fixtures"), te = parts.indexOf("🧪️tests"); const locBad = te < 0 || te >= f || parts.slice(0, f).some(x => A.includes(x)); return !factOk && locBad; }).map(p => relative(root, p));
out.actor = own(["🎭️actor", "🧵️job"]).filter(p => !belongs(relative(modules, p), A) && !placedFacade(p, A)).map(p => relative(root, p));
const owners: string[] = JSON.parse(readFileSync(join(modules, "🏃️process/🧪️tests/🧫️fixtures/🧭️fixture-placement/🔣️.json"), "utf8")).owners;
out.process = [...new Set(owners.flatMap(o => existsSync(join(root, o)) ? walk(join(root, o)) : []))].filter(inFx).filter(p => !belongs(relative(root, p), S) && !(facade(p)?.every(x => belongs(relative(root, x), S)))).map(p => relative(root, p));
const ui = join(modules, "🖱️ui");
out.ui = walk(ui).filter(inFx).filter(f => { const t = facade(f); if (t !== undefined && t.every(x => realpathSync(x).split(/[\\/]/).includes("🧪️tests"))) return false; const p = realpathSync(f).split(/[\\/]/); return !(p.indexOf("🧪️tests") > -1 && p.indexOf("🧪️tests") < p.indexOf("🧫️fixtures")); }).map(p => relative(root, p));
for (const [k, v] of Object.entries(out)) { console.log(`== ${k}: ${v.length}`); for (const p of v) console.log(p); }

const procFiles = [...new Set(owners.flatMap(o => existsSync(join(root, o)) ? walk(join(root, o)) : []))];
const inv = new Set(procFiles.filter(inFx));
const bad: string[] = [];
for (const f of procFiles.filter(f => /\.tsx?$/.test(f))) { const t = readFileSync(f, "utf8"); if (!t.includes("🧫️fixtures")) continue; for (const m of t.matchAll(/(?:import|export)[^"'`;]*?from\s*["']([^"']*🧫️fixtures[^"']*)["']|import\s*["']([^"']*🧫️fixtures[^"']*)["']/gu)) { const spec = (m[1] ?? m[2])!; const target = resolve(dirname(f), spec); if (owners.some(o => relative(root, target) === o || relative(root, target).startsWith(o + "/")) && !inv.has(target)) bad.push(relative(root, f) + " -> " + spec); } }
console.log(`== process-imports: ${bad.length}`); for (const b of bad) console.log(b);
function includes(label: string, ownerDirs: string[], base: string, inScope: (target: string) => boolean): void {
  const all = [...new Set(ownerDirs.flatMap(o => existsSync(o) ? walk(o) : []))];
  const inventory = new Set(all.filter(inFx));
  const bad: string[] = []; let count = 0;
  for (const f of all.filter(f => f.endsWith(".rs"))) for (const m of readFileSync(f, "utf8").matchAll(/include_(?:str|bytes)!\(\s*"([^"]*🧫️fixtures[^"]*)"/gu)) { const target = resolve(dirname(f), m[1]!); if (inScope(target)) { count++; if (!inventory.has(target)) bad.push(relative(root, f) + " -> " + m[1]); } }
  console.log(`== includes ${label}: checked ${count}, bad ${bad.length}`); for (const b of bad) console.log(b);
}
const top = (t: string) => relative(modules, t).split(/[\\/]/)[0]!;
includes("value", ["🌱️value", "⏳️async"].map(o => join(modules, o)), modules, t => ["🌱️value", "⏳️async"].includes(top(t)));
includes("pack", ["🎒️pack", "📡️replication"].map(o => join(modules, o)), modules, t => ["🎒️pack", "📡️replication"].includes(top(t)));
includes("actor", ["🎭️actor", "🧵️job"].map(o => join(modules, o)), modules, t => ["🎭️actor", "🧵️job"].includes(top(t)));
includes("process", owners.map(o => join(root, o)), root, t => owners.some(o => relative(root, t) === o || relative(root, t).startsWith(o + "/")));
