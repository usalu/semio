import fs from "node:fs";
let s = fs.readFileSync("apply.mjs", "utf8");
s = s.replace('import { join, dirname } from "node:path";', 'import { join, dirname } from "node:path";\nimport { fileURLToPath } from "node:url";');
s = s.replace('const here = dirname(new URL(import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, "$1"));', 'const here = dirname(fileURLToPath(import.meta.url));');
fs.writeFileSync("apply.mjs", s);
