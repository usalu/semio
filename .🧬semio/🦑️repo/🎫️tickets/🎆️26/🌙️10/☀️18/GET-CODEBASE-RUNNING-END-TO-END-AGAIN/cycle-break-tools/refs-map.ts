/** 🗺️ Old-to-new maps of the cycle-break delta: net file moves of the direct parts and the staged part, glue entries to module entries, removed keys to new keys. */
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { ROOT, model, posix } from "./lib.ts";
type Op = { op: string; path: string; to?: string };
const parts = resolve(import.meta.dir, "delta-parts");
const direct: Op[] = ["d1", "d1b", "d3", "d5", "d5b", "d6", "d7"].flatMap(n => JSON.parse(readFileSync(resolve(parts, n + ".json"), "utf8")).fileOps as Op[]);
export const delta = JSON.parse(readFileSync(resolve(import.meta.dir, "../cycle-break-delta.json"), "utf8")) as { manifestOps: { op: string; packageJson: string; package: string; field: string; key: string; value: string }[]; newFiles: { path: string; content: string }[]; edits: { file: string; from: string; to: string; mode: string }[]; fileOps: Op[] };
export const net = new Map<string, string>();
for (const o of direct.concat(delta.fileOps)) {
  if (o.op !== "move") continue;
  const origin = [...net].find(([, to]) => to === o.path)?.[0] ?? o.path;
  if (origin === o.to) net.delete(origin); else net.set(origin, o.to!);
}
export const deleted = new Set([...direct, ...delta.fileOps].filter(o => o.op === "delete").map(o => o.path));
export const dirMap = new Map<string, string>();
{
  const cand = new Map<string, Set<string>>();
  for (const [a, b] of net) { const da = dirname(a), db = dirname(b); if (a.split("/").pop() !== b.split("/").pop()) continue; (cand.get(da) ?? cand.set(da, new Set()).get(da)!).add(db); }
  for (const [da, s] of cand) if (s.size === 1) dirMap.set(da, [...s][0]!);
}
export const specMap = new Map<string, string>();
for (const e of delta.edits) if (e.mode === "specifier" && e.from.startsWith("@semio-tech/")) specMap.set(e.from, e.to);
const newPkgs = new Map<string, { dir: string; exports: Record<string, unknown> }>();
for (const nf of delta.newFiles) if (nf.path.endsWith("package.json")) { const j = JSON.parse(nf.content); newPkgs.set(j.name, { dir: posix(dirname(nf.path)), exports: j.exports ?? {} }); }
const entryOf = (spec: string): string | undefined => {
  const m = /^(@[^/]+\/[^/]+)(\/.*)?$/u.exec(spec); if (!m) return undefined;
  const key = m[2] ? "." + m[2] : ".";
  const P = newPkgs.get(m[1]!) ?? (model.byName.get(m[1]!) ? { dir: model.byName.get(m[1]!)!.dir, exports: model.byName.get(m[1]!)!.exports as Record<string, unknown> } : undefined);
  let v = P?.exports[key]; if (v && typeof v === "object") v = (v as Record<string, unknown>).default ?? (v as Record<string, unknown>).import;
  if (!P || typeof v !== "string") return undefined;
  return posix(resolve("/", P.dir, v)).slice(1);
};
export const pkgDirOf = (spec: string): string | undefined => { const m = /^(@[^/]+\/[^/]+)/u.exec(spec); if (!m) return undefined; return newPkgs.get(m[1]!)?.dir ?? model.byName.get(m[1]!)?.dir; };
export const entryOfSpec = entryOf;
export const entryMap = new Map<string, string>();
export const keyMap = new Map<string, string>();
for (const o of delta.manifestOps) {
  if (o.op !== "remove" || o.field !== "exports") continue;
  const spec = o.package + o.key.slice(1), to = specMap.get(spec);
  const oldEntry = posix(resolve("/", dirname(o.packageJson), o.value)).slice(1);
  if (!to) continue;
  keyMap.set(spec, to);
  const e = entryOf(to); if (e) entryMap.set(oldEntry, e);
}
export const exists = (p: string): boolean => existsSync(resolve(ROOT, p));
if (import.meta.main) {
  console.log("net moves", net.size, "dir maps", dirMap.size, "deleted", deleted.size, "entries", entryMap.size, "keys", keyMap.size);
  for (const [a, b] of dirMap) console.log("  dir", a.replace("🧰️framework/", "FW/"), "->", b.replace("🧰️framework/", "FW/"));
}
