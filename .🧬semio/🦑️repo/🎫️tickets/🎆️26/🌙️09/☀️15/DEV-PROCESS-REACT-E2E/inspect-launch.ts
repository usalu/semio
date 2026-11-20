import { readFileSync } from "node:fs";

for (const path of process.argv.slice(2)) console.log(readFileSync(path, "utf8"));
