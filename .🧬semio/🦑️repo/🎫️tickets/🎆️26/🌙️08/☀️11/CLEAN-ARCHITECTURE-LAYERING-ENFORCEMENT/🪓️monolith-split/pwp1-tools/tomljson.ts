// 🧾️ Parses a TOML file with Bun and prints it as JSON (python 3.9 has no tomllib).
import { readFileSync } from "node:fs";
const path = process.argv[2];
console.log(JSON.stringify(Bun.TOML.parse(readFileSync(path, "utf8"))));
