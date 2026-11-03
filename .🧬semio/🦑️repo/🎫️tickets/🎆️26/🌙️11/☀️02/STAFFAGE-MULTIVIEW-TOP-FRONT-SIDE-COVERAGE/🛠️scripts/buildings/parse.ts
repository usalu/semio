import { readFileSync } from "node:fs"; import { path } from "./sy.ts";
for (const id of process.argv.slice(2)) { const a = JSON.parse(readFileSync(path(id),"utf8")).asset; console.log(id, "ok poses", a.poses.length, "clips", a.clips.length); }
