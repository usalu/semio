import { readFile } from "node:fs/promises";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const root = process.cwd(), metadataFile = process.argv[2], rootPackage = process.argv[3], limit = 4000;
const meta = JSON.parse(readFileSync(metadataFile, "utf8"));
const byId = new Map(meta.packages.map(p => [p.id, p]));
const nodes = new Map(meta.resolve.nodes.map(n => [n.id, n]));
const start = meta.packages.find(p => p.name === rootPackage).id, seen = new Set([start]), queue = [start];
while (queue.length) for (const dep of nodes.get(queue.pop())?.deps ?? []) if (!seen.has(dep.pkg)) { seen.add(dep.pkg); queue.push(dep.pkg); }
const mod = await import(resolve(root, "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts"));
const cfg = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix"];
const rows = [];
for (const id of seen) {
  const p = byId.get(id);
  if (p.source !== null) continue;
  for (const t of p.targets.filter(t => t.kind.some(k => ["lib", "rlib", "proc-macro"].includes(k)))) {
    for (const testCfg of [[], ["test"]]) {
      try {
        const rowsIn = await mod.readRustSourceClosureV1([t.src_path], root, [...cfg, ...testCfg], f => readFile(f).catch(e => e.code === "ENOENT" ? null : Promise.reject(e)));
        rows.push({ name: p.name, test: testCfg.length > 0, lines: rowsIn.reduce((s, r) => s + r.activeLines, 0), bytes: rowsIn.reduce((s, r) => s + r.bytes, 0), files: rowsIn.length });
      } catch (e) { rows.push({ name: p.name, test: testCfg.length > 0, error: String(e.message ?? e).slice(0, 160) }); }
    }
  }
}
rows.sort((a, b) => (b.lines ?? 0) - (a.lines ?? 0));
console.log("closure crates:", rows.length / 2, "limit", limit);
for (const r of rows.filter(r => r.error || r.lines > limit * 0.9)) console.log(r.error ? "ERR" : r.lines > limit ? "OVER" : "NEAR", r.name, r.test ? "(test)" : "", r.lines ?? "", r.bytes ?? "", r.files ?? "", r.error ?? "");
