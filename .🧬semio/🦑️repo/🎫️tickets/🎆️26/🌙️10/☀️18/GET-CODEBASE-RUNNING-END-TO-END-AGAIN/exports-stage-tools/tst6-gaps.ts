/**
 * 🧾 tst6 dependency gaps (`tst6-tools/gaps-unresolved.json`) that the code scan of `deps.ts` does not already stage.
 *
 *   bun tst6-gaps.ts   (writes generated/tst6-gaps.json)
 */
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { GENERATED, TICKET } from "./lib.ts";
import { model } from "./model.ts";
import { bindingOf } from "./deps.ts";

const bad = (JSON.parse(readFileSync(resolve(TICKET, "tst6-tools/gaps-unresolved.json"), "utf8")) as { bad: Record<string, string[]> }).bad;
const batch = JSON.parse(readFileSync(resolve(TICKET, "exports-stage-tools/batch.json"), "utf8"));
const staged = new Set<string>(batch.entries.filter((e: any) => e.field !== "exports").map((e: any) => e.packageJson + "\0" + e.key));
const lockText = readFileSync(resolve(TICKET, "../../../../../../../bun.lock"), "utf8");
const rows: { name: string; spec: string; packageJson: string; manifest: string; declaredIn: string[]; staged: boolean; files: string[] }[] = [];
for (const [spec, files] of Object.entries(bad)) {
  if (!/^(@[a-z0-9._-]+\/)?[a-z0-9._-]+(\/[A-Za-z0-9._/-]+)?$/u.test(spec)) continue;
  const name = spec.split("/").slice(0, spec.startsWith("@") ? 2 : 1).join("/");
  const byManifest = new Map<string, string[]>();
  for (const f of files) { const m = bindingOf(f); (byManifest.get(m.packageJson) ?? byManifest.set(m.packageJson, []).get(m.packageJson)!).push(f); }
  for (const [pj, fs] of byManifest) {
    const m = model.byJson.get(pj)!;
    const declaredIn = model.all.filter(p => name in p.deps || name in p.dev || name in p.peer).map(p => p.packageJson + "#" + (name in p.deps ? "dependencies" : "devDependencies") + "=" + (p.deps[name] ?? p.dev[name] ?? p.peer[name]));
    rows.push({ name, spec, packageJson: pj, manifest: m.name, declaredIn, staged: staged.has(pj + "\0" + name) || name in m.deps || name in m.dev, files: fs });
  }
}
const inLock = (n: string): boolean => lockText.includes(`"${n}": ["${n}@`);
writeFileSync(resolve(GENERATED, "tst6-gaps.json"), JSON.stringify(rows));
for (const r of rows.sort((a, b) => a.name.localeCompare(b.name))) console.log((r.staged ? "staged  " : "MISSING ").padEnd(9), r.name.padEnd(34), r.packageJson.slice(0, 30).padEnd(30), "lock:" + (inLock(r.name) ? "y" : "n"), "decl:" + r.declaredIn.length, r.files.length);
