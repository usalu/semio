import { readFileSync, writeFileSync } from "node:fs";

const ui = "C:/git/semio/🧰️framework/🔨️modules/🖱️ui/⌨️tui";
const files = ["📝️text/🔤️tables/🦀️.rs", "📝️text/🦀️.rs"];

for (const file of files) {
  const path = `${ui}/${file}`;
  const text = readFileSync(path, "utf8");
  const lines = text.split("\n").map((line) => {
    if (/^\s*\/\/[\/!]?/.test(line)) return line;
    return line.replace(/\bZWJ\b/g, "Zwj").replace(/\bLVT\b/g, "Lvt");
  });
  writeFileSync(path, lines.join("\n"), "utf8");
}
console.log("renamed");
