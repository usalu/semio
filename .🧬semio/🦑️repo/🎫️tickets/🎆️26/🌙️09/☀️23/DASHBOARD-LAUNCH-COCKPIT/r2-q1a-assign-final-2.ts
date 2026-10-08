import { readFileSync, renameSync, writeFileSync } from "node:fs";

const root = "C:/git/semio";
const ui = "🧰️framework/🔨️modules/🖱️ui";
const dashboard = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard";
const table: Record<string, Array<[number, string]>> = {
  [`${dashboard}/🧪️tests/🔬️unit/🦀️.rs`]: [[30, "🪓"]],
};
const docLine = /^(\s*(?:\/\/\/|\/\/!)\s+)(\S+)(\s.*)$/;

for (const [file, edits] of Object.entries(table)) {
  const path = `${root}/${file}`;
  const lines = readFileSync(path, "utf8").split("\n");
  for (const [number, emoji] of edits) {
    const raw = lines[number - 1];
    const cr = raw.endsWith("\r") ? "\r" : "";
    const doc = docLine.exec(cr ? raw.slice(0, -1) : raw);
    if (!doc) throw new Error(`${file}:${number} is not a docstring`);
    lines[number - 1] = `${doc[1]}${emoji}${doc[3]}${cr}`;
  }
  const staged = `${path}.q1a-tmp`;
  writeFileSync(staged, lines.join("\n"), "utf8");
  renameSync(staged, path);
}
console.log("assigned");
