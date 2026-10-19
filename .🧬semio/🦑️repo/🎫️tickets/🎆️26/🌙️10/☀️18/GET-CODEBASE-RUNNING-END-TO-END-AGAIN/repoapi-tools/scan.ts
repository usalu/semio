import { existsSync, readFileSync, statSync } from "node:fs";
import { resolve } from "node:path";
import { GENERATED, ROOT, edgesOf, fileList, isFrozen, libTargets, writeJson, isOutsidePackage, type Edge } from "./lib.ts";

/** One pass over every source file: edges whose specifier resolves into `known`. Frozen importers are reported apart. */
export function scanEdges(known: Set<string>, include: (file: string) => boolean, refresh = false): { edges: Edge[]; frozen: Edge[]; files: number } {
  const files = fileList(refresh);
  const edges: Edge[] = [], frozen: Edge[] = [];
  for (const file of files) {
    if (!include(file)) continue;
    let text: string;
    try { text = readFileSync(resolve(ROOT, file), "utf8"); } catch { continue; }
    if (!text.includes("..")) continue;
    for (const e of edgesOf(file, text, known)) (isFrozen(file) ? frozen : edges).push(e);
  }
  return { edges, frozen, files: files.length };
}

/** Edges into the library from outside it: reuses a scan younger than `maxAgeMs`, else scans the tree. */
export function ensureScan(maxAgeMs = 15 * 60_000): { edges: Edge[] } {
  const file = resolve(GENERATED, "edges.json");
  if (existsSync(file) && Date.now() - statSync(file).mtimeMs < maxAgeMs) return JSON.parse(readFileSync(file, "utf8"));
  const r = scanEdges(libTargets(), isOutsidePackage);
  writeJson(file, { generatedAt: new Date().toISOString(), edges: r.edges, frozen: r.frozen });
  return { edges: r.edges };
}

if (import.meta.main) {
  const t0 = Date.now();
  const r = scanEdges(libTargets(), isOutsidePackage, process.argv.includes("--refresh"));
  writeJson(resolve(GENERATED, "edges.json"), { generatedAt: new Date().toISOString(), edges: r.edges, frozen: r.frozen });
  console.log(`files ${r.files}, edges ${r.edges.length}, importers ${new Set(r.edges.map(e => e.file)).size}, targets ${new Set(r.edges.map(e => e.target)).size}, frozen-importer edges ${r.frozen.length}, ${Date.now() - t0} ms`);
}
