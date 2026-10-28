/** 🐹️ Runs the Go launcher (or MCP) cases of a root with the canonical overlay plan: bun gotest.ts <root> <module-rel> <out-dir> [go test args...] */
import { spawnSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const [root, module, out, ...args] = process.argv.slice(2) as [string, string, string, ...string[]];
const { canonicalGoPlan } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🐹️go-plan/🟦️.ts"));
const plan = canonicalGoPlan(join(root, module));
mkdirSync(out, { recursive: true });
const overlay = join(out, "overlay.json");
writeFileSync(overlay, JSON.stringify({ Replace: plan.replacements }));
const run = spawnSync(process.env.LP_GO ?? "go", [...(process.env.LP_GOVERB ? [process.env.LP_GOVERB] : ["test", "-count=1"]), "-overlay", overlay, ...args, ...plan.packages], { cwd: join(root, module), encoding: "utf8", env: { ...process.env, GOTOOLCHAIN: "local", SEMIO_WORKSPACE_ROOT: root }, maxBuffer: 1 << 26 });
process.stdout.write(run.stdout.split("\n").slice(-40).join("\n") + run.stderr.split("\n").slice(-20).join("\n"));
process.exit(run.status ?? 1);
