#!/usr/bin/env bun
/** 🔎️ Z1: renders `.vscode/launch.json` from the current seed without writing it, and reports the
 * diff against the committed file plus any duplicate launch names the seed change would introduce. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const { renderCatalogFiles } = await import(`${ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts`);
const { generateLaunchJson } = await import(`${ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts`);

const { playgrounds, componentLaunchers } = renderCatalogFiles(ROOT);
const next = generateLaunchJson(ROOT, playgrounds, componentLaunchers);
const current = readFileSync(join(ROOT, ".vscode/launch.json"), "utf8");

const names = (Bun.JSONC.parse(next) as { configurations: { name: string }[] }).configurations.map((row) => row.name);
const seen = new Map<string, number>();
for (const name of names) seen.set(name, (seen.get(name) ?? 0) + 1);
const duplicates = [...seen].filter(([, count]) => count > 1);
console.log(`rendered ${names.length} configurations; duplicates: ${duplicates.length}`);
for (const [name, count] of duplicates) console.log(`  DUPLICATE x${count}: ${name}`);

const currentLines = current.split("\n"), nextLines = next.split("\n");
console.log(`current ${currentLines.length} lines, rendered ${nextLines.length} lines, identical: ${current === next}`);
await Bun.write(`${ROOT}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/z1-launch-rendered.json`, next);
