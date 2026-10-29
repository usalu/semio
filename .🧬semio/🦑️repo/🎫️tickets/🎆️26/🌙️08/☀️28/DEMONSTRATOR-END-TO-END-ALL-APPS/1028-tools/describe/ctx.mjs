import { readFileSync } from "node:fs";
const { rustTokens, rustTokenPairs } = await import(process.argv[2]);
const s = readFileSync(process.argv[3], "utf8"), t = rustTokens(s), p = rustTokenPairs(t);
const bad = t.findIndex((x, i) => ["(", ")", "[", "]", "{", "}"].includes(x.text) && !p.has(i));
console.log(bad, t[bad]);console.log(JSON.stringify(s.slice(t[bad].start-400, t[bad].start+300)));
