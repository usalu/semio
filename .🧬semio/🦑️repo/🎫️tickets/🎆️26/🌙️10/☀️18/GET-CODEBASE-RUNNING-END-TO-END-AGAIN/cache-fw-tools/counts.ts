import { readFileSync } from "node:fs";
import { join } from "node:path";
const load = (n: string) => JSON.parse(readFileSync(join(import.meta.dir, "../🗑️generated", n), "utf8"));
const b = load("cache-fw-census-before.json"), a = load("cache-fw-census-after.json");
const cnt = (c: any) => { const T = c.targets.filter((t: any) => t.owner === "🧰️framework"); const m: Record<string, number> = {}; for (const t of T) m[t.classProposal] = (m[t.classProposal] ?? 0) + 1; const f: Record<string, number> = {}; for (const t of T) for (const x of t.flags) f[x] = (f[x] ?? 0) + 1; const raw = { cacheTrue: T.filter((t: any) => t.cache === "true").length, cacheFalse: T.filter((t: any) => t.cache === "false").length, cacheAbsent: T.filter((t: any) => t.cache === "absent").length, inputsAbsent: T.filter((t: any) => t.inputs.declaredKind === "absent").length, outputsAbsent: T.filter((t: any) => t.outputs.kind === "absent").length }; return { total: T.length, m, f, raw }; };
console.log("before", JSON.stringify(cnt(b))); console.log("after", JSON.stringify(cnt(a)));
