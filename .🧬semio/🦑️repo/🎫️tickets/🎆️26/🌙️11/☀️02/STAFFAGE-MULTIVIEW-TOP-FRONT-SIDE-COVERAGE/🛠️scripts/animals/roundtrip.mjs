import fs from "node:fs";
const f = process.argv[2]; const t = fs.readFileSync(f, "utf8"); const r = JSON.stringify(JSON.parse(t), null, 2);
console.log(t === r, t === r + "\n", t.length, r.length, t.includes("\r\n"));
