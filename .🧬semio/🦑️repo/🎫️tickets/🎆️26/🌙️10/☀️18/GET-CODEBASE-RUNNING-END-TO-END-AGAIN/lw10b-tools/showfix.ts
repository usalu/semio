import { readFileSync } from "node:fs";
const p = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const pre = process.argv[3]!;
for (const f of p.fixes) if (f.from.includes(pre)) console.log(f.from.split("/").slice(-4).join("/"), "|", f.specifier.slice(-60), "->", f.replacement);
