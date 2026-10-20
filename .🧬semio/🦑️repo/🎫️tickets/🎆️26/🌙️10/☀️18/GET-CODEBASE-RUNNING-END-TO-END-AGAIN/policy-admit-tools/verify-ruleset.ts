/**
 * 🛡️ Runs dependency-cruiser's own rule-set validation (including the regular expression safety check) over a dumped policy.
 *
 *   bun verify-ruleset.ts <policy.json>
 */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const root = process.env.SEMIO_ROOT ?? resolve(import.meta.dir, "../../../../../../../..");
const policy = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const base = pathToFileURL(Bun.resolveSync("dependency-cruiser", root));
const { default: assertRuleSetValid } = await import(new URL("./rule-set/assert-validity.mjs", base).href);
const started = performance.now();
assertRuleSetValid({ forbidden: policy.forbidden, options: policy.options });
console.log(`rule set valid: ${policy.forbidden.length} rules in ${Math.round(performance.now() - started)} ms`);
