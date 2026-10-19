/** ✅ Validates every ✏️s manifest against the neutral project schema (repo validator) and checks that only intended properties changed versus the pre-write backup. */
import { readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import { ROOT, manifests } from "./lib.ts";
import { validateJsonSchemaSubset } from "../../../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";

const schema = JSON.parse(readFileSync(join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json"), "utf8"));
const backup = JSON.parse(readFileSync(join(import.meta.dir, "..", "🗑️generated", "cache-s-backup.json"), "utf8")) as Record<string, string>;
let invalid = 0, checked = 0, unintended = 0;
for (const f of manifests()) {
  const text = readFileSync(join(ROOT, f), "utf8");
  const json = JSON.parse(text);
  const errors = validateJsonSchemaSubset(schema, json);
  checked++;
  if (errors.length) { invalid++; console.log("INVALID", f, errors.slice(0, 3)); }
  if (!backup[f]) continue;
  const old = JSON.parse(backup[f]);
  for (const k of new Set([...Object.keys(old), ...Object.keys(json)])) if (!["targets", "namedInputs"].includes(k) && JSON.stringify(old[k]) !== JSON.stringify(json[k])) { unintended++; console.log("UNINTENDED top-level", f, k); }
  for (const n of new Set([...Object.keys(old.targets ?? {}), ...Object.keys(json.targets ?? {})])) {
    const a = old.targets?.[n], b = json.targets?.[n];
    if (!a || !b) { unintended++; console.log("UNINTENDED target add/remove", f, n); continue; }
    for (const k of new Set([...Object.keys(a), ...Object.keys(b)])) {
      if (["cache", "outputs", "inputs", "executor"].includes(k)) continue;
      if (k === "options") { const oa = { ...a.options }, ob = { ...b.options }; delete oa.cwd; delete ob.cwd; if (JSON.stringify(oa) !== JSON.stringify(ob)) { unintended++; console.log("UNINTENDED options", f, n); } continue; }
      if (JSON.stringify(a[k]) !== JSON.stringify(b[k])) { unintended++; console.log("UNINTENDED", f, n, k); }
    }
  }
}
console.log(JSON.stringify({ checked, invalid, unintended, backups: Object.keys(backup).length }));
