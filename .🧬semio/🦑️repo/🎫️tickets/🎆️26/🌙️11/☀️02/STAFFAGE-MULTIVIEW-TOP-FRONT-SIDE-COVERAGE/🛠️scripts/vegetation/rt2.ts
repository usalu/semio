import { readFileSync, writeFileSync } from "node:fs";
const f = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/english-oak/🔣️.json";
const t = readFileSync(f, "utf8"); writeFileSync(process.argv[2], JSON.stringify(JSON.parse(t), null, 2) + "\n");
