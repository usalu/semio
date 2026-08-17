import fs from "node:fs";

const s = fs.readFileSync("c:/git/compose/elements/client/lib/board/node_modules/react-reconciler/cjs/react-reconciler.development.js", "utf8");
const re = /([a-zA-Z0-9$]+) = \$\$\$config\.([a-zA-Z0-9]+)/g;
const keys = new Set();
let m;
while ((m = re.exec(s))) {
  keys.add(m[2]);
}
console.log([...keys].sort().join("\n"));
console.error("count", keys.size);
