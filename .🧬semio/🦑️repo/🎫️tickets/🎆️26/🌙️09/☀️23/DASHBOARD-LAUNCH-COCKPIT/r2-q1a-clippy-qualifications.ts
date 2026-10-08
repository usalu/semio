import { readFileSync, writeFileSync } from "node:fs";
import { posix } from "node:path";

const log = readFileSync(process.argv[2], "utf8").replaceAll("\\", "/").split(/\r?\n/);
const target = process.argv[3];
const hits = new Map<string, Array<[number, number]>>();
for (const line of log) {
  const match = /^(.*?):(\d+):(\d+): warning: unnecessary qualification/.exec(line);
  if (!match || !match[1].includes(target)) continue;
  const file = posix.normalize(match[1]).replace(/^.*\/🖱️ui\//, "");
  hits.set(file, [...(hits.get(file) ?? []), [Number(match[2]), Number(match[3])]]);
}
const ui = "C:/git/semio/🧰️framework/🔨️modules/🖱️ui";
for (const [file, spots] of hits) {
  const path = `${ui}/${file}`;
  const lines = readFileSync(path, "utf8").split("\n");
  for (const [number, column] of spots.sort((a, b) => b[1] - a[1])) {
    const line = lines[number - 1];
    const chars = [...line];
    const start = chars.slice(0, column - 1).join("").length;
    if (!line.startsWith("tables::", start)) throw new Error(`${file}:${number}:${column} ${line.slice(start, start + 20)}`);
    lines[number - 1] = line.slice(0, start) + line.slice(start + "tables::".length);
  }
  writeFileSync(path, lines.join("\n"), "utf8");
  console.log(file, spots.length);
}
