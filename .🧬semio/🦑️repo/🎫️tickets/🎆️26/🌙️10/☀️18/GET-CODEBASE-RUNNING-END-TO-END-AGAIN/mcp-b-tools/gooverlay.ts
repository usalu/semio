import { spawnSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { canonicalGoPlan, writeCanonicalGoWorkspaceOverlay } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🐹️go-plan/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../../..");
const [verb, out, ...rest] = process.argv.slice(2);
const moduleRoot = join(root, process.env.GO_MODULE ?? "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp");
const plan = canonicalGoPlan(moduleRoot);
const overlay = join(import.meta.dir, "../🗑️generated/mcp-b/overlay.json");
writeCanonicalGoWorkspaceOverlay(overlay, [moduleRoot]);
const merged = JSON.parse(readFileSync(overlay, "utf8"));
Object.assign(merged.Replace, plan.replacements);
for (const [virtual, source] of Object.entries(merged.Replace)) if (String(source).includes("hermetic-root-guard")) delete merged.Replace[virtual];
writeFileSync(overlay, JSON.stringify(merged));
const env = { ...process.env, GOWORK: join(root, "go.work") };
const args = verb === "build" ? ["build", `-overlay=${overlay}`, "-o", out, "."] : ["test", `-overlay=${overlay}`, ...rest, ...plan.packages];
console.log("[go]", args.join(" ").slice(0, 300));
process.exit(spawnSync("go", args, { cwd: moduleRoot, env, stdio: "inherit" }).status ?? 1);
