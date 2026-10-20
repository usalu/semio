// Writes the canonical Go compiler overlay (virtual sources and tests) of the overlay copy of the repo MCP module.
// Usage: bun overlay-plan.ts <overlayRoot> <overlayJsonOut>
import { writeFileSync } from "node:fs";
import { join } from "node:path";
const [overlayRoot, out] = process.argv.slice(2);
if (!overlayRoot || !out) throw new Error("usage: overlay-plan.ts <overlayRoot> <overlayJsonOut>");
const repo = process.cwd();
const { canonicalGoPlan } = await import(join(repo, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🐹️go-plan/🟦️.ts"));
const module = join(overlayRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp");
const plan = canonicalGoPlan(module);
writeFileSync(out, JSON.stringify({ Replace: plan.replacements }, null, 1) + "\n");
console.log(JSON.stringify({ packages: plan.packages, sources: Object.keys(plan.sourceReplacements).length, tests: Object.keys(plan.testReplacements).length }));
