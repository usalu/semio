import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "/Users/ueli/Documents/semio/node_modules/ajv/dist/ajv.js";
import { ROOT, TICKET, loadTargets } from "./targets.ts";
const { validateJsonSchemaSubset } = await import(join(ROOT, "🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts"));
const schema = JSON.parse(readFileSync(join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json"), "utf8"));
const check = new Ajv({ allErrors: true, strict: false }).compile(schema);
const applied = JSON.parse(readFileSync(join(TICKET, "🗑️generated", "uncached-outputs", "apply-actions.json"), "utf8"));
let repoFail = 0, ajvFail = 0;
for (const manifest of applied.manifests as string[]) {
  const json = JSON.parse(readFileSync(join(ROOT, manifest), "utf8"));
  const e1 = validateJsonSchemaSubset(schema, json);
  if (e1.length) { repoFail++; console.log("repo validator:", manifest.slice(-70), JSON.stringify(e1).slice(0, 300)); }
  if (!check(json)) { ajvFail++; console.log("ajv:", manifest.slice(-70), JSON.stringify(check.errors).slice(0, 300)); }
}
console.log({ validated: applied.manifests.length, repoValidatorFailures: repoFail, ajvFailures: ajvFail });
const decisions = JSON.parse(readFileSync(join(TICKET, "🗑️generated", "uncached-outputs", "decisions.json"), "utf8"));
const kind = new Map(decisions.map((d: any) => [d.id, d.resolution]));
const left: Record<string, string[]> = { contract: [], stale: [], unresolved: [] };
let resolved = 0;
for (const t of loadTargets()) {
  const id = t.project + ":" + t.target;
  const res = kind.get(id) as string;
  if (res === "contract" || res === "stale") { left[res]!.push(id); continue; }
  const def = JSON.parse(readFileSync(join(ROOT, t.manifest), "utf8")).targets?.[t.target];
  if (def && (Array.isArray(def.outputs) || def.cache === false)) resolved++; else left.unresolved!.push(id);
}
console.log({ resolvedInManifest: resolved, contractInjected: left.contract!.length, stale: left.stale!.length, unresolved: left.unresolved });
