import { readFileSync, writeFileSync } from "node:fs";

const path = process.argv[2]!;
const text = readFileSync(path, "utf8");
const blocks = text.split(/\n(?=  Scenario: )/);
const drop = new Set(process.argv.slice(3));
const kept = blocks.filter((block) => ![...drop].some((title) => block.startsWith(`  Scenario: ${title}\n`) || block.startsWith(`  Scenario: ${title}`)));
writeFileSync(path, kept.join("\n").replace(/\n+$/, "\n"));
console.log(blocks.length - kept.length, "removed");
