/** 🧲 Turns the paths a command script reads into project-level source globs that the build graph appends to `nativeSources` (Cargo owners) or `default` (script owners). */
import { reach, type Reach } from "./reach.ts";
import { relative } from "node:path";
import { ROOT, strip } from "./lib.ts";

const TYPED = "{ts,tsx,json,semio,wit,sql}";
const isFile = (rel: string) => /\.[a-z]+$/u.test(rel.split("/").at(-1) ?? "");

export function family(rel: string): string {
  const segs = rel.split("/");
  const art = segs.indexOf("🗿️artifacts");
  if (segs[0] === "✏️s" && art >= 0 && segs.length > art + 1) return segs.slice(0, art + 2).join("/");
  if (segs[0] === "✏️s") return segs.slice(0, 3).join("/");
  if (segs[0] === "🧰️framework") {
    const mod = segs.indexOf("🔨️modules");
    if (mod >= 0 && segs.length > mod + 1) return segs.slice(0, mod + 2).join("/");
    return segs.slice(0, 3).join("/");
  }
  return segs.slice(0, 2).join("/");
}

const inside = (rel: string, owner: string) => rel === owner || rel.startsWith(owner + "/");

export function extras(projectRoot: string, owner: string, scope: "native" | "script"): { globs: string[]; reach: Reach } {
  const r = reach(projectRoot, owner);
  const globs = new Set<string>();
  const add = (rel: string, typedOnly: boolean) => globs.add(`{workspaceRoot}/${rel}/**/${typedOnly ? "*." + TYPED : "*"}`);
  for (const rel of r.dirs) {
    if (scope === "script" && inside(rel, owner)) continue;
    if (rel === projectRoot) continue;
    const dir = isFile(rel) ? rel.split("/").slice(0, -1).join("/") : rel;
    if (scope === "native" || !inside(dir, owner)) add(dir, false);
    const fam = family(rel);
    if (!(scope === "script" && inside(fam, owner))) add(fam, true);
  }
  if ((r.indirect || r.unresolved.length) && scope === "native") add(owner, true);
  void relative; void ROOT; void strip;
  return { globs: [...globs].sort(), reach: r };
}
