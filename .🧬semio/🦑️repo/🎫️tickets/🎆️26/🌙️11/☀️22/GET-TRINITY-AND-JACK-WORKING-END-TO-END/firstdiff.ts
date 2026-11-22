import { readFileSync } from "node:fs";

const text = readFileSync(process.argv[2]!, "utf8");
const again = JSON.stringify(JSON.parse(text), null, 2) + (text.endsWith("\n") ? "\n" : "");
let index = 0;
while (index < text.length && text[index] === again[index]) index++;
console.log(JSON.stringify({ length: [text.length, again.length], at: index, original: text.slice(Math.max(0, index - 60), index + 80), rewritten: again.slice(Math.max(0, index - 60), index + 80) }, null, 1));
