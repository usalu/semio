import { readFileSync, writeFileSync } from "node:fs";

const spec = readFileSync(process.argv[2]!, "utf8")
  .split(/\r?\n/)
  .filter((line) => line.trim() !== "");

const byFile = new Map<string, Map<number, string>>();
for (const row of spec) {
  const match = /^(.+?):(\d+)\s+(\S+)$/.exec(row);
  if (!match) throw new Error(`bad spec row: ${row}`);
  const file = match[1]!;
  const rows = byFile.get(file) ?? new Map<number, string>();
  rows.set(Number(match[2]), match[3]!.replace(/️/g, "") + "️");
  byFile.set(file, rows);
}

for (const [file, rows] of byFile) {
  const text = readFileSync(file, "utf8");
  const eol = text.includes("\r\n") ? "\r\n" : "\n";
  const lines = text.split(/\r?\n/);
  for (const [number, emoji] of rows) {
    const line = lines[number - 1]!;
    const match = /^(\s*\/\/[\/!]\s?)(\S+)(.*)$/.exec(line);
    if (!match) throw new Error(`${file}:${number} is not a docstring: ${line}`);
    lines[number - 1] = `${match[1]}${emoji}${match[3]}`;
  }
  writeFileSync(file, lines.join(eol));
  console.log(`${file}: ${rows.size}`);
}
