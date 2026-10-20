// Writes a Go overlay and an alternate go.mod that compile the live cli module with the staged launcher integration, using the repository's own canonical planner. Usage: bun cli-overlay.ts <ov-root> <out-dir>
import { readFileSync, readdirSync, writeFileSync, mkdirSync, existsSync } from "node:fs";
import { join, resolve } from "node:path";
import { canonicalGoPlan } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🐹️go-plan/🟦️.ts";
const live = resolve(import.meta.dir, "../../../../../../../..");
const [ov, out] = [resolve(process.argv[2]!), resolve(process.argv[3]!)];
const M = "🧰️framework/🛍️products/🦑️repo/🔨️modules";
const cli = join(live, M, "⌨️cli/📦️packages/🐹️go");
const plan = canonicalGoPlan(process.env.MODULE ? join(live, M, process.env.MODULE) : cli);
const replace: Record<string, string> = { ...plan.sourceReplacements, ...plan.testReplacements };
for (const [virtual, source] of Object.entries(replace)) { const relative = source.startsWith(live + "/") ? source.slice(live.length + 1) : undefined; if (relative && existsSync(join(ov, relative))) replace[virtual] = join(ov, relative); }
replace[join(cli, "🐹️.go")] = join(ov, M, "⌨️cli/📦️packages/🐹️go/🐹️.go");
replace[join(cli, "🚪️entrypoint_unix.go")] = "";
replace[join(cli, "🚪️entrypoint_windows.go")] = "";
const launcher = join(ov, M, "🚀️launcher/📦️packages/🐹️go");
let n = 0;
for (const entry of readdirSync(launcher, { withFileTypes: true })) if (entry.isDirectory() && entry.name !== "📦️main" && existsSync(join(launcher, entry.name, "🐹️.go"))) replace[join(launcher, `zz_${n++}.go`)] = join(launcher, entry.name, "🐹️.go");
mkdirSync(out, { recursive: true });
writeFileSync(join(out, "overlay.json"), JSON.stringify({ Replace: replace }));
let mod = readFileSync(join(ov, M, "⌨️cli/📦️packages/🐹️go/go.mod"), "utf8");
mod = mod.replace(/=> (\.\.\/[^\n]*)/g, (_m, rel) => `=> ${resolve(cli, rel)}`).replace("repo/launcher => " + resolve(cli, "../../../🚀️launcher/📦️packages/🐹️go"), "repo/launcher => " + launcher);
writeFileSync(join(out, "cli.mod"), mod);
writeFileSync(join(out, "cli.sum"), existsSync(join(cli, "go.sum")) ? readFileSync(join(cli, "go.sum")) : "");
console.log(Object.keys(replace).length, "overlay entries");
