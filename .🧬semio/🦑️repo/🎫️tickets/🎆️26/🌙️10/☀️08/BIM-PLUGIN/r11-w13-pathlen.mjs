import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";
const root = process.argv[2];
const prefix = "C:\git\semio\\".length;
let max = 0, worst = "", over = 0;
const walk = (dir) => { for (const name of readdirSync(dir)) { const path = join(dir, name); const length = prefix + path.replaceAll("\\", "/").split("/").join("\\").length - (process.argv[3] ? 0 : 0); if (length > max) { max = length; worst = path; } if (length > 256) over++; if (statSync(path).isDirectory()) walk(path); } };
walk(process.argv[2]);
console.log(max, over, worst);
