import { readFileSync } from "node:fs"; import { path } from "./sy.ts";
const a = JSON.parse(readFileSync(path(process.argv[2]),"utf8")).asset;
for (const v of process.argv[4].split(",")) for (const s of a.representations[process.argv[3]].geometry[v]) if (new RegExp(process.argv[5]).test(s.id)) console.log(v, JSON.stringify(s));
