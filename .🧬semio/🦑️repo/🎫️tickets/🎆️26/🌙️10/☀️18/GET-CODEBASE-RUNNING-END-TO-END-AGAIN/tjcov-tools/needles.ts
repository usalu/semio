import { readFileSync } from "node:fs";
const [file, ...needles] = process.argv.slice(2);
const text = readFileSync(file!, "utf8");
const compact = text.replace(/\s+/g, "");
for (const n of needles) console.log(text.includes(n) ? "ok   " : compact.includes(n.replace(/\s+/g, "")) ? "SPACE" : "MISS ", JSON.stringify(n));
