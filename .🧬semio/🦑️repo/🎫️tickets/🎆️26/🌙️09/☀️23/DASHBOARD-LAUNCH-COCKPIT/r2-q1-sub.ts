import { readFileSync, writeFileSync } from "node:fs";

const [path, ...rest] = process.argv.slice(2);
const pairs = JSON.parse(readFileSync(rest[0]!, "utf8")) as [string, string][];
let text = readFileSync(path!, "utf8");
for (const [from, to] of pairs) {
  const count = text.split(from).length - 1;
  if (count !== 1) throw new Error(`${count} matches for: ${from.slice(0, 70)}`);
  text = text.replace(from, () => to);
}
writeFileSync(path!, text);
