#!/usr/bin/env bun
/** [DEBUG] P6: remove duplicate DocumentCodec/OpCodec / Handcrafted*Codec regions (keep first). */
import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join, relative, dirname } from "path";
import { fileURLToPath } from "url";

const ticket = dirname(fileURLToPath(import.meta.url));
const repo = "/Users/ueli/Documents/semio";
const PLUGINS = join(repo, "✏️s", "🔌️plugins");

function walk(dir, out=[]) {
  for (const n of readdirSync(dir)) {
    if (n==="target"||n==="node_modules") continue;
    const p=join(dir,n); const st=statSync(p);
    if (st.isDirectory()) walk(p,out);
    else if (n.endsWith(".rs") && p.includes("/🗿️artifacts/")) out.push(p);
  }
  return out;
}

const report = { files: 0, removedRegions: 0, details: [] };

function dedupeFile(abs) {
  let t = readFileSync(abs, "utf8");
  const rel = relative(repo, abs);
  // Collect codec regions of interest
  const regionRe = /\/\/#region 🔖️(DocumentCodec|OpCodec|HandcraftedDocumentCodecs|HandcraftedOpCodecs)\n[\s\S]*?\/\/#endregion 🔖️\1\n/g;
  const regions = [];
  let m;
  while ((m = regionRe.exec(t)) !== null) {
    const body = m[0];
    const impls = [...body.matchAll(/impl (?:[\w:]+::)*(DocumentDsl|DocumentPack|OpText|OpBinary) for ([A-Za-z0-9_]+)/g)].map(x => `${x[1]}:${x[2]}`);
    regions.push({ start: m.index, end: m.index + m[0].length, kind: m[1], impls, text: body });
  }
  if (regions.length === 0) return;

  // For each trait:type, keep earliest region that provides it; drop later regions that only duplicate
  const seen = new Set();
  const drop = new Set();
  for (const r of regions) {
    const novel = r.impls.filter((k) => !seen.has(k));
    if (novel.length === 0 && r.impls.length > 0) {
      drop.add(r.start);
    } else {
      for (const k of r.impls) seen.add(k);
      // If region is partially novel, keep whole region (safer)
    }
  }

  // Special case: if both Handcrafted* and DocumentCodec/OpCodec cover same types, drop DocumentCodec/OpCodec (ours) and keep Handcrafted*
  const byType = new Map();
  for (const r of regions) {
    for (const k of r.impls) {
      if (!byType.has(k)) byType.set(k, []);
      byType.get(k).push(r);
    }
  }
  for (const [k, list] of byType) {
    if (list.length < 2) continue;
    // prefer Handcrafted* region
    const hand = list.find((r) => r.kind.startsWith("Handcrafted"));
    if (hand) {
      for (const r of list) {
        if (r !== hand) drop.add(r.start);
      }
    } else {
      // keep first, drop rest
      for (let i = 1; i < list.length; i++) drop.add(list[i].start);
    }
  }

  if (drop.size === 0) return;

  // Remove from end to start
  const toRemove = regions.filter((r) => drop.has(r.start)).sort((a, b) => b.start - a.start);
  for (const r of toRemove) {
    t = t.slice(0, r.start) + t.slice(r.end);
    report.removedRegions++;
    report.details.push({ rel, kind: r.kind, impls: r.impls });
  }
  writeFileSync(abs, t);
  report.files++;
}

for (const f of walk(PLUGINS)) dedupeFile(f);

writeFileSync(join(ticket, "🧪p6-dedupe-report.json"), JSON.stringify(report, null, 2));
console.log(JSON.stringify({ files: report.files, removedRegions: report.removedRegions, sample: report.details.slice(0, 20) }, null, 2));
