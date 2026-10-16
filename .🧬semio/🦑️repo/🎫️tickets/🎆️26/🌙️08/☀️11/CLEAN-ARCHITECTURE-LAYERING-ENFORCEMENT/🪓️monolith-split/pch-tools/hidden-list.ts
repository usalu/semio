// 🙈️ Read-only: lists cargo members (from a manifest list) that the launcher inventory does not know.
import {readFileSync} from "node:fs";
import {join} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const lib = await import(join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts"));
const known = new Set(lib.cargoRepositoryPackages(ROOT).map((p: any) => p.manifest));
for (const m of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) if (!known.has(m)) console.log(m);
