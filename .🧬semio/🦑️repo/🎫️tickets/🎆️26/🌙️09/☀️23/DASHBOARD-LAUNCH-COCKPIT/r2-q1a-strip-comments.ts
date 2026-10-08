import { readFileSync, writeFileSync } from "node:fs";

const ui = "C:/git/semio/🧰️framework/🔨️modules/🖱️ui";
const files = ["⌨️tui/⚙️engine/🦀️.rs", "⌨️tui/📟️vt/🧱️screen/🦀️.rs", "⌨️tui/📟️vt/🖥️pane/🦀️.rs"];
const marker = /^\s+\/\/\s*#(end)?region\b/;
const text = (line: string | undefined) => (line ?? "\u0000").replace(/\r$/, "");
const blank = (line: string | undefined) => line !== undefined && text(line).trim() === "";

for (const file of files) {
  const path = `${ui}/${file}`;
  const lines = readFileSync(path, "utf8").split("\n");
  const kept: string[] = [];
  const gaps: number[] = [];
  for (const line of lines) {
    if (marker.test(text(line))) {
      gaps.push(kept.length);
      continue;
    }
    kept.push(line);
  }
  const drop = new Set<number>();
  for (const gap of gaps) {
    for (const index of [gap - 1, gap]) {
      if (!blank(kept[index])) continue;
      const before = kept[index - 1];
      const after = kept[index + 1];
      if (blank(before) || text(before).trimEnd().endsWith("{") || text(after).trim() === "}" || blank(after)) {
        drop.add(index);
        break;
      }
    }
  }
  const result = kept.filter((_, index) => !drop.has(index));
  writeFileSync(path, result.join("\n"), "utf8");
  console.log(`${file}: removed ${gaps.length} markers, ${lines.length - result.length} lines`);
}
