/**
 * ↩️ Reverts the applied specifier rewrites that introduce a violation of another boundary rule or a new import cycle.
 *
 *   bun revert-regress.ts <plan.json> <beforeDir> <afterDir> <out.json> [apply]
 */
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";

const [planPath, outPath, mode] = process.argv.slice(2) as [string, string, string | undefined];
const ROOT = "/Users/ueli/Documents/semio";
const policy = require(`${ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs`);
const plan = JSON.parse(readFileSync(planPath, "utf8"));
const edits = (plan.edits as any[]).filter(e => !plan.unresolved?.some((u: any) => u.file === e.file && u.after === e.after));

const arr = (v: unknown): string[] => (v === undefined ? [] : Array.isArray(v) ? v : [v as string]);
const rx = (p: string): RegExp => { try { return new RegExp(p, "u"); } catch { return new RegExp(p); } };
const rules = (policy.forbidden as any[]).filter(r => !r.name.startsWith("no-cross-package-relative-") && r.name !== "no-circular" && !r.to?.dependencyTypes && !r.to?.dependencyTypesNot && !r.to?.couldNotResolve && !r.to?.circular && !r.to?.orphan && !r.from?.orphan && !r.module && r.name !== "not-to-unlisted").map(r => ({
  name: r.name,
  fromPath: arr(r.from?.path).map(rx), fromNot: arr(r.from?.pathNot).map(rx), toPath: arr(r.to?.path).map(rx), toNot: arr(r.to?.pathNot).map(rx),
}));
const violates = (from: string, to: string): string[] => rules.filter(r => (!r.fromPath.length || r.fromPath.some(x => x.test(from))) && !r.fromNot.some(x => x.test(from)) && (!r.toPath.length || r.toPath.some(x => x.test(to))) && !r.toNot.some(x => x.test(to))).map(r => r.name);

const resolveNew = (e: any): string | null => {
  try {
    const r = e.after.startsWith(".") ? resolve(dirname(resolve(ROOT, e.file)), e.after) : Bun.resolveSync(e.after, dirname(resolve(ROOT, e.file)));
    return relative(ROOT, r).split("\\").join("/");
  } catch { return null; }
};

const reverted = new Map<any, string>();
const info = edits.map(e => ({ e, newTo: resolveNew(e), oldTo: (e.to as string) }));
for (const x of info) {
  if (!x.newTo) { reverted.set(x.e, "unresolvable"); continue; }
  const now = violates(x.e.file, x.newTo), was = new Set(violates(x.e.file, x.oldTo));
  const fresh = now.filter(n => !was.has(n));
  if (fresh.length) reverted.set(x.e, "rule:" + fresh.join(","));
}
const byReason: Record<string, number> = {};
for (const r of reverted.values()) byReason[r.replace(/^rule:.*/u, "rule")] = (byReason[r.replace(/^rule:.*/u, "rule")] ?? 0) + 1;
const ruleCounts: Record<string, number> = {};
for (const r of reverted.values()) if (r.startsWith("rule:")) for (const n of r.slice(5).split(",")) ruleCounts[n] = (ruleCounts[n] ?? 0) + 1;
let done = 0, missing = 0;
if (mode === "apply") {
  const byFile = new Map<string, any[]>();
  for (const e of reverted.keys()) (byFile.get(e.file) ?? byFile.set(e.file, []).get(e.file)!).push(e);
  for (const [file, list] of byFile) {
    const abs = resolve(ROOT, file);
    let text = readFileSync(abs, "utf8");
    for (const e of list) {
      const q = [`"${e.after}"`, `'${e.after}'`, `\`${e.after}\``].find(s => text.includes(s));
      if (!q) { missing++; continue; }
      text = text.replace(q, q[0] + e.before + q[0]);
      done++;
    }
    writeFileSync(abs, text);
  }
}
writeFileSync(outPath, JSON.stringify({ reverted: [...reverted].map(([e, r]) => ({ file: e.file, before: e.before, after: e.after, reason: r })), byReason, ruleCounts, done, missing }, null, 1));
console.log(JSON.stringify({ edits: edits.length, reverted: reverted.size, byReason, ruleCounts, done, missing }));
