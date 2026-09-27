#!/usr/bin/env bun
/** 🧪️ R9 item 1: renders `.vscode/launch.json` with the registry's own generator (seed + playground registry + declared
 * project targets), reports what it registered, checks it with VS Code's own parser (`jsonc-parser`), and with `--write`
 * writes it (the same bytes `@semio-tech/plugin-registry:generate` writes for launch.json). */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import * as jsonc from "jsonc-parser";
const ROOT = "/Users/ueli/Documents/semio";
const { renderCatalogFiles } = await import(`${ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts`);
const { generateLaunchJson, declaredProjectTargets } = await import(`${ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts`);
const started = performance.now();
const { playgrounds } = renderCatalogFiles(ROOT);
const projects = declaredProjectTargets(ROOT);
const text = generateLaunchJson(ROOT, playgrounds, projects);
const errors: jsonc.ParseError[] = [];
const document = jsonc.parse(text, errors, { allowTrailingComma: false }) as { configurations: { name: string; command?: string; presentation?: { group?: string } }[]; inputs: { id: string; options?: string[] }[] };
const declared = projects.reduce((sum: number, project: { targets: string[] }) => sum + project.targets.length, 0);
const covered = new Set<string>();
for (const row of document.configurations) for (const match of String(row.command ?? "").matchAll(/nx run ([^\s:$]+):(\S+)/gu)) covered.add(`${match[1]}:${match[2]}`);
const familyInputs = new Map(document.inputs.filter((input) => input.id.startsWith("projectTarget.")).map((input) => [input.id, input.options ?? []]));
const familyRows = document.configurations.filter((row) => String(row.command ?? "").includes("${input:projectTarget."));
const byFamily = new Set<string>();
for (const row of familyRows) {
  const match = /\$\{input:(projectTarget\.[^}]+)\}:(\S+)/u.exec(String(row.command))!;
  for (const project of familyInputs.get(match[1]!) ?? []) byFamily.add(`${project}:${match[2]}`);
}
const uncovered = projects.flatMap((project: { project: string; targets: string[] }) => project.targets.map((target) => `${project.project}:${target}`)).filter((pair: string) => !covered.has(pair) && !byFamily.has(pair));
const groups = new Map<string, number>();
for (const row of document.configurations) groups.set(row.presentation?.group ?? "-", (groups.get(row.presentation?.group ?? "-") ?? 0) + 1);
const names = new Map<string, number>();
for (const row of document.configurations) names.set(row.name, (names.get(row.name) ?? 0) + 1);
console.log(JSON.stringify({ ms: Math.round(performance.now() - started), projects: projects.length, declaredTargets: declared, configurations: document.configurations.length, familyRows: familyRows.length, familyInputs: familyInputs.size, coveredByRows: covered.size, uncovered: uncovered.length, duplicateNames: [...names].filter(([, count]) => count > 1).length, vscodeParserErrors: errors.length, lines: text.split("\n").length, groups: Object.fromEntries(groups), identicalToCommitted: text === readFileSync(join(ROOT, ".vscode/launch.json"), "utf8") }));
if (uncovered.length) console.log("uncovered sample", uncovered.slice(0, 10));
if (process.argv.includes("--write")) writeFileSync(join(ROOT, ".vscode/launch.json"), text);
process.exit(errors.length === 0 && uncovered.length === 0 ? 0 : 1);
