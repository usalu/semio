import { join } from "node:path";
import { existsSync } from "node:fs";
import { ROOT } from "./targets.ts";
import { parse, type Fn } from "./module.ts";
import { walk, domainOf, rel, type Evidence } from "./walk.ts";
const memo = new Map<string, Evidence>();
export function walkFile(relFile: string, root: string): Evidence {
  const key = relFile + "|" + domainOf(root);
  const hit = memo.get(key); if (hit) return hit;
  const abs = join(ROOT, relFile);
  if (!existsSync(abs)) return { texts: [], files: [], entry: "", entryKind: "missing", owned: [], infra: [], spawns: [], writes: [], unresolved: ["MISSING FILE " + relFile] };
  const m = parse(abs);
  const fn: Fn = { name: "<file>", file: abs, start: 0, end: m.src.length, text: m.src, kind: "function" };
  const ev = walk([{ file: abs, fn }], domainOf(root));
  memo.set(key, ev);
  return ev;
}
