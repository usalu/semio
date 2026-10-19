import { readFileSync } from "node:fs";
import { clauses } from "./ct.ts";
const LAW = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage";
const files = [...new Bun.Glob("**/🟦️.ts").scanSync({ cwd: LAW, absolute: true })].filter((f) => !f.includes("/🧪️tests/"));
const defs = new Map<string, string>();
for (const f of files) for (const m of readFileSync(f, "utf8").matchAll(/^export function (toolJob\w+)\(/gm)) defs.set(m[1]!, f);
(globalThis as any).__trace = async (name: string, argsFn: () => unknown[]) => {
  const t = Date.now();
  try {
    const args = argsFn();
    const file = defs.get(name);
    if (!file) { console.log("[TRACE]", name, "NO-DEF"); return; }
    const r = clauses(file, name, args, { quiet: true });
    console.log("[TRACE]", name, r.total, "clauses", r.bad.length, "bad", (Date.now() - t) + "ms");
    for (const b of r.bad) console.log("[TRACE-BAD]", name, b.slice(0, 420));
  } catch (e: any) { console.log("[TRACE]", name, "TRACER-ERR", String(e?.message ?? e).slice(0, 200)); }
};
const m = await import(process.argv[2]!);
const lanes: any = new Proxy({}, { get: () => () => 0 });
const r = await m.toolJobCoverageRun(process.cwd(), lanes);
console.log("[CENSUS] failures", r.failures.length);
for (const f of r.failures) if (!/bounded reducer|Migrated declaration/.test(f)) console.log("[FAILURE]", f.slice(0, 300));
