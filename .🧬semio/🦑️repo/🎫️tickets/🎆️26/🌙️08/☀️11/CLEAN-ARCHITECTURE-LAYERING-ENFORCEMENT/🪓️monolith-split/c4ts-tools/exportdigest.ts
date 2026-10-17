// usage: bun exportdigest.ts <out.json> <module>... : imports the modules, merges their exports (asserting no duplicate names) and digests every export:
// zero-arity functions are called (sync or async) and their JSON is hashed; other functions record name and arity; values are hashed.
import { createHash } from "node:crypto";
import { writeFileSync } from "node:fs";
const [out, ...modules] = process.argv.slice(2);
const sha = (text: string) => createHash("sha256").update(text).digest("hex").slice(0, 16);
const norm = (_k: string, v: unknown): unknown => {
  if (v instanceof Set) return { set: [...v].map((x) => JSON.stringify(x, norm)).sort() };
  if (v instanceof Map) return { map: [...v.entries()].map(([k, x]) => [JSON.stringify(k, norm), JSON.stringify(x, norm)]).sort() };
  if (typeof v === "function") return `fn/${v.length}`;
  if (typeof v === "bigint") return `${v}n`;
  return v;
};
const merged: Record<string, unknown> = {};
for (const m of modules) {
  const ns = (await import(m)) as Record<string, unknown>;
  for (const [k, v] of Object.entries(ns)) {
    if (k in merged) throw new Error(`duplicate export ${k} (${m})`);
    merged[k] = v;
  }
}
const rows: Record<string, string> = {};
const noArg = new Set((process.env.NOARG ?? "").split(",").filter(Boolean));
const catalogArg = new Set((process.env.CATALOGARG ?? "").split(",").filter(Boolean));
const catalog = typeof merged.loadVizCatalog === "function" ? (merged.loadVizCatalog as () => unknown)() : undefined;
for (const name of Object.keys(merged).sort()) {
  const v = merged[name];
  if (typeof v === "function") {
    if (v.length === 0 && !/^(verify|generate|write|with|run)/.test(name) || noArg.has(name) || catalogArg.has(name)) {
      try { const r = await (v as (c?: unknown) => unknown)(catalogArg.has(name) ? catalog : undefined); rows[name] = "call:" + sha(JSON.stringify(r, norm) ?? "undefined"); } catch (e) { rows[name] = "throws:" + String((e as Error).message).slice(0, 80); }
    } else rows[name] = `fn/${v.length}`;
  } else rows[name] = "value:" + sha(JSON.stringify(v, norm) ?? "undefined");
}
writeFileSync(out, JSON.stringify(rows, null, 1));
console.log(`${Object.keys(rows).length} exports, ${Object.values(rows).filter((r) => r.startsWith("call:")).length} called, ${Object.values(rows).filter((r) => r.startsWith("throws:")).length} threw`);
