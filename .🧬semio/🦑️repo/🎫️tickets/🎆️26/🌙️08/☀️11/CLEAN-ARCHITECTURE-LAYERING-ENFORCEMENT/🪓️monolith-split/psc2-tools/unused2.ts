/** 🧹️ For created exports with no specifier importer: counts importers that reach the facade by relative path (bootstrap entries). Usage: bun unused2.ts <plan.json>... */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { ROOT, scanSource } from "../sdkb-tools/lib.ts";
import { listSourceFiles, nfc } from "./model.ts";
const rows: { plan: string; spec: string; file: string }[] = [];
for (const p of process.argv.slice(2)) {
  const plan = JSON.parse(readFileSync(p, "utf8"));
  for (const f of plan.facades) if (f.created) rows.push({ plan: p.split("-plan-")[1].replace(".json", ""), spec: f.pkg + (f.subpath === "." ? "" : f.subpath.slice(1)), file: nfc(f.facade) });
}
const bySpec = new Map<string, number>(), byFile = new Map<string, number>();
for (const f of listSourceFiles()) {
  let text: string; try { text = readFileSync(join(ROOT, f), "utf8"); } catch { continue; }
  if (!/(?:from|import|require)\s*\(?\s*["'`]/u.test(text)) continue;
  for (const r of scanSource(f, text)) {
    if (!r.resolved) continue;
    const res = nfc(r.resolved);
    byFile.set(res, (byFile.get(res) ?? 0) + (f === res ? 0 : 1));
  }
}
const none = rows.filter(r => !(byFile.get(r.file) ?? 0));
console.log({ createdFacades: rows.length, noImporterAtAll: none.length });
for (const r of none) console.log("  NO-IMPORTER", r.plan, r.spec);
