import {readFileSync} from "node:fs";
for (const p of process.argv.slice(2)) { try { Bun.TOML.parse(readFileSync(p, "utf8")); } catch (e) { console.log("BAD", p, String((e as Error).message).slice(0, 80)); } }
console.log("checked", process.argv.length - 2);
