import { planClass } from "./classsplit.ts";
const mod = await import(process.argv[2]);
const write = process.argv[3] === "--write";
const { files, sourceText } = planClass(mod.cp, write);
for (const f of files) console.log(String(f.text.split("\n").length).padStart(5), f.file.split("modules/")[1]);
console.log(String(sourceText.split("\n").length).padStart(5), "(source)");
if (process.argv[3] === "--show") console.log(sourceText.slice(0, 3000));
