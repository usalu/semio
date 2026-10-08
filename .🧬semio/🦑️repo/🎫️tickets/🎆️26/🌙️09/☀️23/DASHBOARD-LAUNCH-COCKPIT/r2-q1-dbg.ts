import { readFileSync } from "node:fs";
const pairs = JSON.parse(readFileSync(process.argv[3]!, "utf8")) as [string, string][];
const text = readFileSync(process.argv[2]!, "utf8");
for (const [i, [from]] of pairs.entries()) {
  let n = from.length;
  while (n > 0 && !text.includes(from.slice(0, n))) n--;
  console.log(i, n, from.length, JSON.stringify(from.slice(n - 5, n + 15)));
}
