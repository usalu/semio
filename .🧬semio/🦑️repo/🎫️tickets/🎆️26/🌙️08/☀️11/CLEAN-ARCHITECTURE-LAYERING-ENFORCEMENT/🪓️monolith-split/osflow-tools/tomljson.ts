// 🧾️ Prints a Cargo.toml as JSON (Bun TOML) for the python tools. usage: bun tomljson.ts <file>
import {readFileSync} from "node:fs";
console.log(JSON.stringify((Bun as any).TOML.parse(readFileSync(process.argv[2]!, "utf8"))));
