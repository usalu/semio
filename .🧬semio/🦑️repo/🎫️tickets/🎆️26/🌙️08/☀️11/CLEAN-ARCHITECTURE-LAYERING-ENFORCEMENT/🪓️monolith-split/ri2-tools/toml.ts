import { readFileSync } from "node:fs";
console.log(JSON.stringify(Bun.TOML.parse(readFileSync(process.argv[2]!, "utf8"))));
