import { readFileSync, writeFileSync } from "node:fs";

const ui = "C:/git/semio/🧰️framework/🔨️modules/🖱️ui";
const el = (name: string) => `🧱️elements/${name}/🎯️targets/⌨️tui/🦀️.rs`;

const table: Record<string, Array<[number, string]>> = {
    "⌨️tui/📝️text/🦀️.rs": [[225, "🧮"]],
};

const docLine = /^(\s*(?:\/\/\/|\/\/!)\s+)(\S+)(\s.*)$/;
const startsWithEmoji = (token: string) => (token.codePointAt(0) ?? 0) > 0x2000 && !/^\p{L}/u.test(token);

for (const [file, edits] of Object.entries(table)) {
  const path = `${ui}/${file}`;
  const lines = readFileSync(path, "utf8").split("\n");
  for (const [number, emoji] of edits) {
    const raw = lines[number - 1];
    const cr = raw.endsWith("\r") ? "\r" : "";
    const body = cr ? raw.slice(0, -1) : raw;
    const doc = docLine.exec(body);
    if (!doc) throw new Error(`${file}:${number} is not a docstring: ${body}`);
    lines[number - 1] = startsWithEmoji(doc[2]) ? `${doc[1]}${emoji}${doc[3]}${cr}` : `${doc[1].replace(/\s+$/, " ")}${emoji} ${doc[2]}${doc[3]}${cr}`;
  }
  writeFileSync(path, lines.join("\n"), "utf8");
}
console.log("assigned");
