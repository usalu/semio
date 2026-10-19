import { readFileSync, writeFileSync } from "node:fs";
const base = "🧰️framework/🔨️modules/♾️infinite/🌍️world/🎨️r3f/";
const edits: Record<string, Array<[RegExp, string]>> = {
  "🧪️tests/🎚️config/🟦️.ts": [
    [/(\.\.\/){8}(?=🔨)/gu, "../".repeat(6)],
    [/"(\.\.\/){9}\.\."/gu, `"${"../".repeat(6)}.."`],
  ],
  "📦️packages/🟦️typescript/📋️project.json": [[/(\.\.\/){7}(?=🦑)/gu, "../".repeat(6) + "🛍️products/"]],
  "📦️packages/🟦️typescript/📜️script.ts": [
    [/(\.\.\/){8}(?=🔨)/gu, "../".repeat(6)],
    [/(\.\.\/){7}(?=🦑)/gu, "../".repeat(6) + "🛍️products/"],
  ],
  "📦️packages/🟦️typescript/package.json": [[/"(\.\.\/){8}\.\."/gu, `"${"../".repeat(6)}.."`]],
};
for (const [file, list] of Object.entries(edits)) {
  let text = readFileSync(base + file, "utf8");
  for (const [pattern, replacement] of list) text = text.replace(pattern, replacement);
  writeFileSync(base + file, text);
}
