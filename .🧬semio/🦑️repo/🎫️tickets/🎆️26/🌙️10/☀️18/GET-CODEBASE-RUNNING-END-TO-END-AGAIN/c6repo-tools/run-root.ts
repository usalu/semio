import { readFileSync } from "node:fs";
import { analyze } from "./ast.ts";
import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { exportMap, definerIndex } from "./exports.ts";
import { build, surfaceOf } from "./split.ts";
import { L, plan } from "./plan-root.ts";
import { repoint } from "./repoint.ts";
const S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad";
const dry = !process.argv.includes("--write");
const edges = JSON.parse(readFileSync(S + "/edges.json", "utf8"));
const old = new Set([`${L}/🟦️.ts`, `${L}/📦️packages/🟦️typescript/🟦️.ts`]);
const files = Object.entries<any>(edges).filter(([, rows]) => rows.some(([s, r]: any) => (r && old.has(r)) || s === "@semio-tech/repo-lib")).map(([f]) => f).sort();
const a = analyze(plan.source);
const built = build(plan, a);
const { map, missingStars } = surfaceOf(plan, a, built, exportMap);
const pending = new Set<string>();
const ambiguous = new Set<string>();
const resolver = (n: string) => {
  const f = map.get(n);
  if (f && existsSync(join("/Users/ueli/Documents/semio", f))) return f;
  const dirs = f ? [dirname(f)] : missingStars.map((m) => dirname(m));
  const hits = dirs.flatMap((d) => definerIndex(d).get(n) ?? []);
  if (hits.length === 1) return hits[0];
  (hits.length ? ambiguous : pending).add(n);
  return undefined;
};
const { changed, report } = repoint(files, { oldTargets: old, bare: new Set(["@semio-tech/repo-lib"]), resolver, dry, skip: (f) => built.some((b) => b.file === f) || f === plan.source || old.has(f) });
console.log("candidates", files.length, "changed", changed.length, dry ? "(dry)" : "(written)");
const byKind = new Map<string, number>();
for (const r of report) byKind.set(r.kind, (byKind.get(r.kind) ?? 0) + 1);
console.log([...byKind]);
for (const r of report) console.log(r.kind, "|", r.file.slice(-90), "|", r.detail);

if (process.argv.includes("--list")) {
  const { writeFileSync } = await import("node:fs");
  writeFileSync(S + "/root-importers.txt", changed.join("\n") + "\n");
}

console.log("pending", [...pending].join(","));
console.log("ambiguous", [...ambiguous].join(","));
