/**
 * 🏗️ Bundles every existing TypeScript/JavaScript exports target of every workspace package with `Bun.build` (target bun, packages external)
 * and reports resolution errors (missing relative modules, missing named exports).
 *
 *   bun glue-tools/build-targets.ts [--only=<manifest substring>] [--sample=<n>]
 */
import { rmSync, mkdirSync } from "node:fs";
import { resolve } from "node:path";
import { ROOT, listTargets } from "./check-exports.ts";

const only = process.argv.find(a => a.startsWith("--only="))?.slice(7);
const sample = Number(process.argv.find(a => a.startsWith("--sample="))?.slice(9) ?? 0);
const out = resolve(import.meta.dir, "../🗑️generated/glue/build");
const byManifest = new Map<string, string[]>();
for (const t of listTargets()) {
  if (!t.exists || !/\.(ts|tsx|mts|js|mjs|cjs)$/u.test(t.file) || t.file.endsWith(".d.ts") || t.file.endsWith(".d.mts")) continue;
  if (only && !t.manifest.includes(only)) continue;
  (byManifest.get(t.manifest) ?? byManifest.set(t.manifest, []).get(t.manifest)!).push(resolve(ROOT, t.file));
}
let total = 0, failed = 0;
for (const [manifest, files] of byManifest) {
  const picked = sample ? files.slice(0, sample) : [...new Set(files)];
  rmSync(out, { recursive: true, force: true }); mkdirSync(out, { recursive: true });
  const result = await Bun.build({ entrypoints: picked, outdir: out, target: "bun", packages: "external", throw: false, naming: "[hash].[ext]" }).catch(e => ({ success: false, logs: [e as any] }));
  total += picked.length;
  const text = (l: any): string => Buffer.from(String(l.message ?? l), "latin1").toString("utf8");
  const viteOnly = /\?(raw|url|inline|worker|sharedworker)"|\.css:\d+$|^Could not resolve: "\/|\/(\u{1F578}\uFE0F?bindings|pkg)\//u;
  const errors = (result.logs as any[]).filter(l => l.level === "error" && !viteOnly.test(text(l) + (l.position ? ` ${l.position.file}:${l.position.line}` : "")));
  if (errors.length) { failed += picked.length; console.log(`FAIL ${manifest} (${picked.length} entries)`); for (const e of errors.slice(0, 8)) console.log("   " + text(e).slice(0, 300) + (e.position ? ` @ ${e.position.file}:${e.position.line}` : "")); }
}
rmSync(out, { recursive: true, force: true });
console.log(`built ${total} entries across ${byManifest.size} manifests, failing entries ${failed}`);
