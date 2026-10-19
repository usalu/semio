import { readFileSync } from "node:fs";
import { join } from "node:path";
import { ROOT, projectFiles } from "./lib.ts";
let n = 0; const bad: string[] = [];
for (const f of projectFiles) { const j = JSON.parse(readFileSync(join(ROOT, f), "utf8")); for (const [k, t] of Object.entries<any>(j.targets ?? {})) { n++; if (t.executor !== "repo:script" && t.executor !== "repo:noop") bad.push(j.name + ":" + k + " " + t.executor); } }
console.log(n, bad.length, bad.slice(0, 40));
