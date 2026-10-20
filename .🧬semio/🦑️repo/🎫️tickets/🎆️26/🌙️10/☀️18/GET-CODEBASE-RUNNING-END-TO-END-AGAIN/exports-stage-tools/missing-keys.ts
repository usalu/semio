/**
 * 🗝️ Bare workspace imports whose export key does not exist in the target manifest (`bun missing-keys.ts`, writes generated/missing-keys.json).
 */
import { writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { GENERATED } from "./lib.ts";
import { scan } from "./scan.ts";
import { model } from "./model.ts";
const { bare } = scan();
const matches = (pkgExports: Record<string, unknown>, sub: string): boolean => {
  if (sub in pkgExports) return true;
  for (const k of Object.keys(pkgExports)) { if (!k.includes("*")) continue; const [a, b] = k.split("*"); if (sub.startsWith(a!) && sub.endsWith(b!) && sub.length >= (a! + b!).length) return true; }
  return false;
};
const miss = new Map<string, { pkg: string; sub: string; files: Set<string> }>();
const noPkg = new Map<string, Set<string>>();
for (const b of bare) {
  const p = model.byName.get(b.pkg);
  if (!p) { (noPkg.get(b.pkg) ?? noPkg.set(b.pkg, new Set()).get(b.pkg)!).add(b.file); continue; }
  const sub = b.spec.split("?")[0]!.slice(b.pkg.length);
  const key = sub ? "." + sub : ".";
  if (matches(p.exports, key)) continue;
  const k = b.pkg + key;
  (miss.get(k) ?? miss.set(k, { pkg: b.pkg, sub: key, files: new Set() }).get(k)!).files.add(b.file);
}
writeFileSync(resolve(GENERATED, "missing-keys.json"), JSON.stringify({ miss: [...miss.values()].map(m => ({ ...m, files: [...m.files] })), noPkg: [...noPkg].map(([k, v]) => ({ pkg: k, files: [...v] })) }));
console.log("missing keys:", miss.size, "importer files", new Set([...miss.values()].flatMap(m => [...m.files])).size);
for (const m of [...miss.values()].sort((a, b) => b.files.size - a.files.size).slice(0, 60)) console.log(String(m.files.size).padStart(3), m.pkg + m.sub.slice(1 - 1 + (m.sub === "." ? 1 : 0)));
console.log("unknown packages:", [...noPkg].map(([k, v]) => `${k}(${v.size})`).join(" "));
