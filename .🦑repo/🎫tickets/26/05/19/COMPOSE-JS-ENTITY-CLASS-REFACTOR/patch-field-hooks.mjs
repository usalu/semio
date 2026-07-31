import fs from "node:fs";

const path = "c:/git/compose/compose/client/lib/react/logic/index.tsx";
let s = fs.readFileSync(path, "utf8");
const re = /useCurrentEntityField\(([^,]+),\s*\((\w+)\)\s*=>\s*\2\.(\w+)\(\)\)(?!\s*,)/g;
const matches = [...s.matchAll(re)];
s = s.replace(re, 'useCurrentEntityField($1, ($2) => $2.$3(), "$3")');
fs.writeFileSync(path, s);
console.log(`updated ${matches.length} hooks`);
