import { readdirSync, readFileSync } from "node:fs";
const root = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation";
for (const d of readdirSync(root)) { const t = readFileSync(`${root}/${d}/🔣️.json`, "utf8"); const o = JSON.parse(t); const r = [JSON.stringify(o, null, 2), JSON.stringify(o, null, 2) + "\n", JSON.stringify(o)].findIndex((x) => x === t); console.log(d, r, t.includes("\r\n") ? "crlf" : "lf", t.slice(0, 80).replace(/\n/g, "|")); }
