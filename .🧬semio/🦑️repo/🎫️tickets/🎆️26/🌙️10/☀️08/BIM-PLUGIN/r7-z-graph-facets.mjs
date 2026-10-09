import { readFileSync, writeFileSync } from "node:fs";

const I = "C:/git/semio/✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/";
const rust = (path) => readFileSync(I + path, "utf8");

const members = (source, name) => {
  const match = source.match(new RegExp(`pub enum ${name} \\{([\\s\\S]*?)\\n\\}`));
  return match[1].split("\n").map((line) => line.trim().replace(/,$/, "")).filter((line) => /^[A-Z][A-Za-z0-9]*$/.test(line));
};

const enums = {
  OpeningIssue: members(rust("🪟️opening-frames/🦀️.rs"), "OpeningIssue"),
  DiagnosticCode: members(rust("⚠️diagnostics/🦀️.rs"), "DiagnosticCode"),
};

const snake = (text) => text.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
const edit = (path, change) => {
  const before = readFileSync(I + path, "utf8");
  const after = change(before);
  writeFileSync(I + path, after);
  console.log(path, before === after ? "unchanged" : "updated");
};

for (const [name, list] of Object.entries(enums)) {
  const prefix = snake(name);
  for (const path of ["🟦️.ts", "🪟️opening-frames/🟦️.ts", "⚠️diagnostics/🟦️.ts"]) {
    edit(path, (text) => text.replace(new RegExp(`export type ${name} = [^;]*;`), `export type ${name} = ${list.map((item) => `"${item}"`).join(" | ")};`));
  }
  edit("🔣️.json", (text) => text.replace(new RegExp(`("${name}": \\{[^\\]]*?"enum": \\[)[^\\]]*(\\])`), (_, open, close) => `${open}\n${list.map((item) => `        "${item}"`).join(",\n")}\n      ${close}`));
  edit("🔗️.graphql", (text) => text.replace(new RegExp(`enum ${name} \\{[^}]*\\}`), `enum ${name} {\n${list.map((item) => `  ${item}`).join("\n")}\n}`));
  edit("🛰️.proto", (text) => text.replace(new RegExp(`enum ${name} \\{[^}]*\\}`), `enum ${name} {\n${list.map((item, index) => `  ${prefix}_${snake(item)} = ${index};`).join("\n")}\n}`));
}
