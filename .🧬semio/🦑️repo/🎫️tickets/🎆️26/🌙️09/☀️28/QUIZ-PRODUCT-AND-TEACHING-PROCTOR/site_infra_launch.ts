/** 🚀️ Renders `.vscode/launch.json` from the seed exactly as `@semio-tech/plugin-registry:generate` does, prints which launch rows
 * appear or vanish against the current file, and writes it with `--write` (without regenerating the registry catalog). */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { renderCatalogFiles } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts";
import { declaredProjectTargets, generateLaunchJson, LAUNCH_OUTPUT_REL_PATH } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts";

const root = join(import.meta.dir, "../../../../../../..");
const path = join(root, LAUNCH_OUTPUT_REL_PATH);
const current = readFileSync(path, "utf8");
const next = generateLaunchJson(root, renderCatalogFiles(root).playgrounds, declaredProjectTargets(root));
const names = (text: string): Map<string, string> => new Map((Bun.JSONC.parse(text) as { configurations: { name: string }[]; compounds: { name: string }[] }).configurations.concat((Bun.JSONC.parse(text) as { compounds: { name: string }[] }).compounds).map((row) => [row.name, JSON.stringify(row)]));
const before = names(current), after = names(next);
for (const [name, row] of after) if (!before.has(name)) console.log(`[DEBUG] + ${name} ${row.slice(0, 200)}`);
for (const [name] of before) if (!after.has(name)) console.log(`[DEBUG] - ${name}`);
for (const [name, row] of after) if (before.has(name) && before.get(name) !== row) console.log(`[DEBUG] ~ ${name}`);
console.log(`[DEBUG] rows ${before.size} -> ${after.size}; identical=${current === next}`);
if (process.argv.includes("--write")) writeFileSync(path, next);
