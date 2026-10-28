import { writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { canonicalGoPlan } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🐹️go-plan/🟦️.ts";
const [moduleRoot, out] = process.argv.slice(2);
const plan = canonicalGoPlan(resolve(moduleRoot));
writeFileSync(out, JSON.stringify({ Replace: plan.replacements }));
console.log(Object.keys(plan.replacements).length, "replacements", plan.packages.length, "packages");
