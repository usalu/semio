import { readFileSync, renameSync, writeFileSync } from "node:fs";
import { patch } from "./r2-q1a-edit.ts";

const ui = "C:/git/semio/🧰️framework/🔨️modules/🖱️ui";
const dashboard = "C:/git/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🪟️windows/🦀️.rs";

patch("⌨️tui/📟️vt/🖥️pane/🦀️.rs", [
  ["    pub screen: VtScreen,", "    pub screen: Box<VtScreen>,"],
  ["Self { screen: VtScreen::new(size, scrollback_cap),", "Self { screen: Box::new(VtScreen::new(size, scrollback_cap)),"],
]);
patch("⌨️tui/🖥️chrome/🦀️.rs", [["    Window(WindowState),\n}", "    Window(Box<WindowState>),\n}"]]);
patch("🧱️elements/🌳️Tree/🎯️targets/⌨️tui/🦀️.rs", [
  ["    index: FilterIndex,", "    index: Box<FilterIndex>,"],
  ["Self { index: FilterIndex::new(&keys),", "Self { index: Box::new(FilterIndex::new(&keys)),"],
]);

const patterns = /if let|let (NodeContent|ChromeState)|=> |matches!|\) = &|else \{/;
function wrapConstructions(path: string, only = /./) {
  const original = readFileSync(path, "utf8");
  const crlf = original.includes("\r\n");
  const lines = original.replaceAll("\r\n", "\n").split("\n").map((line) => {
    if (patterns.test(line) || !only.test(line)) return line;
    const token = "ChromeState::Window(";
    const at = line.indexOf(token);
    if (at < 0) return line;
    const start = at + token.length;
    let depth = 1;
    let end = start;
    while (end < line.length && depth > 0) {
      if (line[end] === "(") depth++;
      if (line[end] === ")") depth--;
      end++;
    }
    const argument = line.slice(start, end - 1);
    return `${line.slice(0, start)}Box::new(${argument}))${line.slice(end)}`;
  });
  const output = lines.join("\n");
  const staged = `${path}.q1a-tmp`;
  writeFileSync(staged, crlf ? output.replaceAll("\n", "\r\n") : output, "utf8");
  renameSync(staged, path);
}

for (const file of ["⌨️tui/🧪️tests/🔬️unit/🦀️.rs", "⌨️tui/🧪️tests/🔬️render-equivalence/🦀️.rs", "⌨️tui/🧪️tests/🔬️pointer-routing/🦀️.rs", "⌨️tui/🧪️tests/🔬️engine/🦀️.rs", "⌨️tui/🧪️tests/🔬️chrome/🦀️.rs"]) wrapConstructions(`${ui}/${file}`);
wrapConstructions(dashboard, /WindowState::new\(/);
console.log("variants boxed");
