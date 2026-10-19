import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { Package, loadPackage } from "../rs-sweep-tools/cargo.ts";
import { buildTree } from "../rs-sweep-tools/tree.ts";

/** 🗺️ Maps every Rust source file to the manifest directories of the packages whose module trees mount it. */
export function buildFileMap(manifests: string[], cache: string, refresh: boolean): Map<string, string[]> {
  if (!refresh && existsSync(cache)) return new Map(Object.entries(JSON.parse(readFileSync(cache, "utf8"))));
  const map = new Map<string, Set<string>>();
  for (const m of manifests) {
    const pkg = loadPackage(dirname(m));
    if (!pkg) continue;
    for (const t of pkg.targets) {
      if (t.kind === "build" || !t.path || !existsSync(t.path)) continue;
      const tree = buildTree(pkg, t, `${pkg.name}#${t.kind}:${t.name}`);
      for (const raw of tree.files) {
        const f = resolve(raw);
        (map.get(f) ?? map.set(f, new Set()).get(f)!).add(pkg.dir);
      }
    }
  }
  const out: Record<string, string[]> = {};
  for (const [f, s] of map) out[f] = [...s];
  writeFileSync(cache, JSON.stringify(out));
  return new Map(Object.entries(out));
}
export type { Package };
