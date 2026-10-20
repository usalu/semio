/** 🎯 Checks that every export target of every workspace manifest named in argv (or all members) exists on disk, with the live workspace discovery of validate.ts. */
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
const root = process.cwd();
const jr = (p: string): any => JSON.parse(readFileSync(p, "utf8"));
const roots = ["package.json", ...readdirSync(root).filter(d => existsSync(resolve(root, d, "package.json")) && (jr(resolve(root, d, "package.json")).workspaces ?? []).length).map(d => `${d}/package.json`)];
const pjs = new Set<string>();
for (const r of roots) for (const w of jr(resolve(root, r)).workspaces ?? []) { const pj = resolve(root, dirname(r), w, "package.json"); if (existsSync(pj)) pjs.add(pj); }
const flat = (v: unknown): string[] => (typeof v === "string" ? [v] : v && typeof v === "object" ? Object.values(v).flatMap(flat) : []);
let missing = 0, total = 0;
const names = process.argv.slice(2);
for (const pj of pjs) {
  const m = jr(pj);
  if (names.length && !names.includes(m.name)) continue;
  for (const [k, v] of Object.entries<any>(m.exports ?? {})) for (const t of flat(v)) {
    if (t.includes("*")) continue;
    total++;
    if (!existsSync(resolve(dirname(pj), t))) { missing++; if (missing <= 400) console.log(`MISSING ${m.name} ${k} -> ${t}`); }
  }
}
console.log(`export targets ${total}, missing ${missing}`);
process.exit(missing ? 1 : 0);
