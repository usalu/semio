import { parseFile } from "./lib.ts";
import { unwrapDescribeText } from "./split.ts";
import { readFileSync } from "node:fs";
const file = process.argv[2], name = process.argv[3];
const u = name ? unwrapDescribeText(readFileSync(file, "utf8"), name) : undefined;
const p = parseFile(file, u?.text);
for (const x of p.units) {
  const t = p.text.slice(x.start, x.start + 130).replace(/\s+/g, " ");
  const title = x.effect ? t.replace(/^test\(|^it\(/, "").slice(0, 100) : [...x.names].join(",");
  console.log(`${x.id}\tL${x.startLine}\t${x.lines}\t${x.effect ? "T" : "h"}\t${title}`);
}
