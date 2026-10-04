/** 🗒️ Lists every UI text of the quiz client that differs between the last commit and the working tree, in both languages,
 * as a markdown table: `bun ui_text_changes.ts` (run from this ticket folder; reads `git show HEAD:<i18n module>`, changes
 * nothing). Feeds `🗒️ui-notes-for-e2e.md`. */
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "../../../../../../..");
const module = "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts";

function bundles(source: string): { readonly en: Map<string, string>; readonly de: Map<string, string> } {
  const found = { en: new Map<string, string>(), de: new Map<string, string>() };
  for (const [name, target] of [
    ["QUIZ_BUNDLE_EN", found.en],
    ["QUIZ_BUNDLE_DE", found.de],
  ] as const) {
    const start = source.indexOf(`export const ${name}`);
    const end = source.indexOf("\n};", start);
    const body = source.slice(start, end);
    const path: string[] = [];
    const token = /(\w+): \{|\},|(\w+): phrase\(\s*"((?:[^"\\]|\\.)*)"/gu;
    for (const match of body.matchAll(token)) {
      if (match[1] !== undefined) path.push(match[1]);
      else if (match[2] !== undefined) target.set([...path, match[2]].join("."), JSON.parse(`"${match[3]}"`) as string);
      else path.pop();
    }
  }
  return found;
}

const before = bundles(execFileSync("git", ["show", `HEAD:${module}`], { cwd: root, encoding: "utf8", maxBuffer: 1 << 26 }));
const after = bundles(readFileSync(join(root, module), "utf8"));
const cell = (text: string | undefined): string => (text === undefined ? "—" : text.replaceAll("|", "\\|"));
const keys = [...new Set([...before.en.keys(), ...after.en.keys()])];
const changed = keys.filter((key) => before.en.get(key) !== after.en.get(key) || before.de.get(key) !== after.de.get(key));
const rows = (kind: (key: string) => boolean): string[] => changed.filter(kind).map((key) => `| \`${key}\` | ${cell(before.en.get(key))} | ${cell(after.en.get(key))} | ${cell(before.de.get(key))} | ${cell(after.de.get(key))} |`);
const head = "| Key | English before | English now | German before | German now |\n|---|---|---|---|---|";
console.log(`### Changed (${rows((key) => before.en.has(key) && after.en.has(key)).length})\n\n${head}\n${rows((key) => before.en.has(key) && after.en.has(key)).join("\n")}`);
console.log(`\n### Removed (${rows((key) => !after.en.has(key)).length})\n\n${head}\n${rows((key) => !after.en.has(key)).join("\n")}`);
console.log(`\n### New (${rows((key) => !before.en.has(key)).length})\n\n${head}\n${rows((key) => !before.en.has(key)).join("\n")}`);
console.log(`\nKeys: ${before.en.size} before, ${after.en.size} now.`);
