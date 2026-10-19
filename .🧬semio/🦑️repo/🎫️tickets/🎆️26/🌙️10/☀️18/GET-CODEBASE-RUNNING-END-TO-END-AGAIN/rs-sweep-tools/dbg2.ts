import { initParser, findWasm, parseFile } from "./parse.ts";
import { readFileSync } from "node:fs";
await initParser(findWasm("/Users/ueli/Documents/semio")!);
const f = process.argv[2];
const m = parseFile(f, readFileSync(f, "utf8"));
const name = process.argv[3];
for (const it of m.body.items) if (it.name === name) console.log(it.kind, it.line, JSON.stringify(it.cfg));
