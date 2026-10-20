/**
 * 🩺 Every authored export target of every manifest must exist (build outputs are reported apart).
 *
 *   bun health.ts   (writes generated/health.json)
 */
import { existsSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { GENERATED, ROOT, posix } from "./lib.ts";
import { model } from "./model.ts";

const targetsOf = (v: unknown, out: string[] = []): string[] => {
  if (typeof v === "string") out.push(v);
  else if (Array.isArray(v)) v.forEach(x => targetsOf(x, out));
  else if (v && typeof v === "object") Object.values(v).forEach(x => targetsOf(x, out));
  return out;
};
const BUILD = /(^|\/)(dist|pkg|📤️dist|🕸️bindings|out)(\/|$)/u;
const missing: { pkg: string; packageJson: string; key: string; target: string }[] = [];
const outputs: { pkg: string; key: string; target: string }[] = [];
let total = 0;
for (const p of model.all) {
  for (const [key, v] of Object.entries(p.exports)) {
    if (!key.startsWith(".")) continue;
    for (const t of targetsOf(v)) {
      if (!t.startsWith("./") || t.includes("*")) continue;
      total++;
      if (existsSync(resolve(ROOT, p.dir, t))) continue;
      (BUILD.test(t) ? outputs : missing).push({ pkg: p.name, packageJson: p.packageJson, key, target: t } as never);
    }
  }
}
writeFileSync(resolve(GENERATED, "health.json"), JSON.stringify({ missing, outputs }));
console.log(`targets ${total} missing ${missing.length} build-outputs-absent ${outputs.length}`);
for (const m of missing) console.log(m.pkg, m.key, "->", m.target);
void posix;
