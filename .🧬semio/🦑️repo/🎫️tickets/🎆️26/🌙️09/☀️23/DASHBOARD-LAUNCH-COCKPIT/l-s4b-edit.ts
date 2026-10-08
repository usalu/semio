import { readFileSync, writeFileSync } from "node:fs";

/** 📚️ Library module root shared by every table A row of slice L-S4b. */
export const L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";

/** 📦️ Owning Nx manifest of every `@semio-tech/repo-lib` gate. */
export const PROJECT = `${L}/📦️packages/🟦️typescript/📋️project.json`;

/** ✂️ Replaces exact strings that must occur exactly once, in one read-modify-write. */
export function edit(path: string, pairs: readonly (readonly [string, string])[]): void {
  let text = readFileSync(path, "utf8");
  for (const [from, to] of pairs) {
    const count = text.split(from).length - 1;
    if (count !== 1) throw new Error(`${path}: expected exactly one occurrence, found ${count}: ${from.slice(0, 120)}`);
    text = text.replace(from, () => to);
  }
  if (path.endsWith(".json")) JSON.parse(text);
  writeFileSync(path, text);
}

/** 🛡️ Declares the owner cache refusal on one `@semio-tech/repo-lib` target. */
export function refuseCache(target: string): void {
  edit(PROJECT, [[`    "${target}": {\n      "executor": "nx:run-commands",\n      "options": {`, `    "${target}": {\n      "executor": "nx:run-commands",\n      "cache": false,\n      "options": {`]]);
}
