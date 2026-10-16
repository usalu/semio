// 🧾 Dumps TOML files to JSON: bun toml2json.ts out.json file...
import {readFileSync, writeFileSync} from "node:fs";
const [out, ...files] = process.argv.slice(2);
const res: Record<string, unknown> = {};
for (const f of files) res[f] = (Bun as any).TOML.parse(readFileSync(f, "utf8"));
writeFileSync(out, JSON.stringify(res));
