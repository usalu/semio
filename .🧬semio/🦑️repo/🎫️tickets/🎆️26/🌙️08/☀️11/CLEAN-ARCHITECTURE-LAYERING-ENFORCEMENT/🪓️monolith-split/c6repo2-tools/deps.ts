import { parseFile } from "./lib.ts";
const p = parseFile(process.argv[2]);
const nm = (i: number) => `${i}:${[...p.units[i].names][0] ?? p.units[i].kind}`;
for (const a of process.argv.slice(3)) { const i = Number(a); console.log(nm(i), "->", [...p.units[i].deps].map(nm).join(" ")); }
