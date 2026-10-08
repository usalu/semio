import { readFileSync, renameSync, writeFileSync } from "node:fs";

const root = "C:/git/semio";
const ui = "🧰️framework/🔨️modules/🖱️ui";
const dashboard = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard";
const table: Record<string, Array<[number, string]>> = {
  [`${ui}/⌨️tui/📝️text/🦀️.rs`]: [[200, "🧭"], [239, "🔢"], [247, "🔁"], [263, "🔂"], [317, "🪚"], [365, "🧵"], [440, "🧿"], [471, "🔭"]],
  [`${ui}/⌨️tui/🔲️cell/🦀️.rs`]: [[95, "🪢"]],
  [`${ui}/⌨️tui/🚇️pty/🦀️.rs`]: [[328, "🛬"], [999, "🧯"], [639, "🔧"], [1005, "🧰"], [1034, "⚠️"]],
  [`${dashboard}/🌀️daemon/✉️ipc/🦀️.rs`]: [[354, "🌐"], [362, "🧭"], [377, "📍"], [410, "🪪"]],
  [`${dashboard}/🎮️registry/🦀️.rs`]: [[949, "🧮"], [1541, "🤝"]],
  [`${dashboard}/🧪️tests/🧭️journeys/🏗️workspace/🦀️.rs`]: [[44, "📂"], [63, "🧱"], [23, "🧪"], [84, "📄"]],
  [`${dashboard}/🧪️tests/🧭️journeys/🖥️terminal/🦀️.rs`]: [[63, "🎹"], [79, "🔤"], [93, "🖼️"], [102, "📐"], [108, "🔡"], [116, "⏲️"]],
};
const docLine = /^(\s*(?:\/\/\/|\/\/!)\s+)(\S+)(\s.*)$/;

for (const [file, edits] of Object.entries(table)) {
  const path = `${root}/${file}`;
  const original = readFileSync(path, "utf8");
  const lines = original.split("\n");
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

const debugPath = `${root}/${dashboard}/🌀️daemon/🧪️tests/🧊️integration/🦀️.rs`;
const source = readFileSync(debugPath, "utf8");
const kept = source.split("\n").filter((line) => !line.includes('println!("[DEBUG] a batch file stopped with'));
writeFileSync(`${debugPath}.q1a-tmp`, kept.join("\n"), "utf8");
renameSync(`${debugPath}.q1a-tmp`, debugPath);
console.log("assigned");
