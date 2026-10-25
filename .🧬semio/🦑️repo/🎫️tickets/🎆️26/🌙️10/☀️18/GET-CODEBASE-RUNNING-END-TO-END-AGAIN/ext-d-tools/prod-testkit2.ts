import { readFileSync, writeFileSync } from "node:fs";
import { execSync } from "node:child_process";
import { parse } from "/Users/ueli/Documents/semio/node_modules/smol-toml/dist/index.js";
const root = "/Users/ueli/Documents/semio";
const files = execSync("git ls-files -z -- '*Cargo.toml'", { cwd: root, maxBuffer: 1 << 28 }).toString().split("\0").filter(Boolean).filter((f) => !f.startsWith(".🧬semio/"));
const re = /(^|[-_])(test|tests|testing|testkit)([-_]|$)/;
const testOwned = (path: string): boolean => path.split("/").slice(0, -1).some((seg) => ["🧪️tests", "🧫️fixtures", "📚️examples", "🔮️oracles", "🔬️probes", "🏭️generator"].includes(seg));
const nameOf = new Map<string, string>();
const parsed = new Map<string, any>();
for (const f of files) { try { const t: any = parse(readFileSync(`${root}/${f}`, "utf8")); parsed.set(f, t); if (t.package?.name) nameOf.set(t.package.name, f); } catch {} }
// crates with test-only optional deps (per the law's rule)
const lawTestOnly = (t: any): string[] => {
  const feats: Record<string, string[]> = t.features ?? {};
  const live = new Set(Object.keys(feats).filter((n) => !re.test(n)));
  for (let grew = true; grew;) { grew = false; for (const n of [...live]) for (const i of feats[n]!) if (i in feats && !live.has(i)) { live.add(i); grew = true; } }
  const named = (ns: string[]) => ns.flatMap((n) => feats[n]!.filter((i) => (!(i in feats) || i.startsWith("dep:")) && !i.includes("?/")).map((i) => i.replace(/^dep:/, "").split("/")[0]!));
  const prod = new Set(named([...live]));
  const explicit = new Set(Object.values(feats).flat().filter((i) => i.startsWith("dep:")).map((i) => i.slice(4).split("/")[0]!));
  return [...new Set(named(Object.keys(feats).filter((n) => !live.has(n))).filter((n) => explicit.has(n) && !prod.has(n)))];
};
const crates: Record<string, string[]> = {};
for (const [f, t] of parsed) { const only = lawTestOnly(t); const opt = only.filter((d) => t.dependencies?.[d]?.optional === true || Object.values<any>(t.target ?? {}).some((x) => x.dependencies?.[d]?.optional === true)); if (opt.length && t.package?.name) crates[t.package.name] = opt; }
const leaks: Record<string, string[]> = {};
for (const [f, t] of parsed) {
  if (testOwned(f)) continue;
  const tables: any[] = [t.dependencies, ...Object.values<any>(t.target ?? {}).map((x) => x.dependencies)].filter(Boolean);
  for (const table of tables) for (const [key, spec] of Object.entries<any>(table)) {
    const target = typeof spec === "object" && spec.package ? spec.package : key;
    if (!crates[target]) continue;
    const feats: string[] = typeof spec === "object" ? spec.features ?? [] : [];
    if (feats.some((x) => re.test(x))) (leaks[target] ??= []).push(f.slice(-100));
  }
  for (const [fname, items] of Object.entries<string[]>(t.features ?? {})) if (!re.test(fname)) for (const i of items) { const dep = i.split("/")[0]!.replace(/\?$/, ""); if (i.includes("/") && crates[dep] && re.test(i.split("/")[1]!)) (leaks[dep] ??= []).push(`${f.slice(-80)} feature ${fname}`); }
}
console.log("crates with law-test-only optional deps:", Object.keys(crates).length);
console.log("of which a production manifest enables a test-class feature:", Object.keys(leaks).length);
for (const [c, who] of Object.entries(leaks).slice(0, 15)) console.log(c, crates[c]!.join(","), "<-", who.slice(0, 2).join(" | "));
writeFileSync(process.argv[2]!, JSON.stringify({ crates, leaks }));
