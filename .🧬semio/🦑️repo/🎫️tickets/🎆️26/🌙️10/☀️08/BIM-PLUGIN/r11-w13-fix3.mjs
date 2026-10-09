import { readFileSync, writeFileSync } from "node:fs";
let t = readFileSync("r11-w13-csv-ui.ts", "utf8");
const a = t.indexOf("const windows = join(");
const b = t.indexOf("const scheduleWindow");
t = t.slice(0, a) + 'const descend = (parent: string, ...suffixes: string[]) => suffixes.reduce((path, suffix) => join(path, find(path, suffix)), parent);\nconst windows = descend(editor, "modes", "edit", "windows");\n' + t.slice(b);
writeFileSync("r11-w13-csv-ui.ts", t);
