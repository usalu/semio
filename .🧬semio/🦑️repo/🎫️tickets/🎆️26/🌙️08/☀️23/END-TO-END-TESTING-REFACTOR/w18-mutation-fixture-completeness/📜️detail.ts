import { readdirSync, readFileSync, existsSync } from "node:fs";
import { join, relative } from "node:path";
const repoRoot = process.cwd();
const dirs = (p: string): string[] => { try { return readdirSync(p, { withFileTypes: true }).filter((e) => e.isDirectory()).map((e) => e.name); } catch { return []; } };
const manifests: string[] = [];
const walk = (p: string, d: number): void => { if (d > 12) return; let es; try { es = readdirSync(p, { withFileTypes: true }); } catch { return; }
  for (const e of es) { if (["node_modules","target",".git","temp"].includes(e.name)) continue; const f = join(p, e.name);
    if (e.isDirectory()) walk(f, d + 1); else if (e.name.endsWith("component.json") && f.includes("oracle")) manifests.push(f); } };
walk(repoRoot, 0);
const out: any[] = [];
for (const m of manifests) {
  let parsed: any; try { parsed = JSON.parse(readFileSync(m, "utf8")); } catch { continue; }
  if (!Array.isArray(parsed.mutationCatalogs)) continue;
  const owner = relative(repoRoot, m).split("/").slice(0, -2).join("/");
  for (const c of parsed.mutationCatalogs) {
    const kinds: string[] = c.kinds ?? [];
    const vectors: any[] = c.vectors ?? [];
    const vecIds = new Set(vectors.map((v) => v.mutationId));
    const srcRoot = join(repoRoot, owner, "🧬️schema", "🧬️mutations");
    const physicalSrc = dirs(srcRoot);
    const missing = kinds.filter((k) => !vecIds.has(k));
    const strayVec = vectors.map((v)=>v.mutationId).filter((id) => !kinds.includes(id));
    out.push({ owner, catalog: c.id, capability: c.capability, kinds: kinds.length, vectors: vectors.length, missingCount: missing.length, missing, strayVec, physicalSrcDirs: physicalSrc.length, srcRootExists: existsSync(srcRoot) });
  }
}
out.sort((a,b)=>b.missingCount-a.missingCount);
console.log(JSON.stringify(out, null, 1));
