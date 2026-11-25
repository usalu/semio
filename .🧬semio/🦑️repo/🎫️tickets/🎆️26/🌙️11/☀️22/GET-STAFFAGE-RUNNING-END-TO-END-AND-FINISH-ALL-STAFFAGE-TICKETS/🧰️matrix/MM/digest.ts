/** 🔏️ The digest of asset documents as the records carry it: `bun digest.ts <🔣️.json> …`. */
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
for (const file of process.argv.slice(2)) console.log(createHash("sha256").update(JSON.stringify(JSON.parse(readFileSync(file, "utf8")).asset)).digest("hex").slice(0, 12), file.split("/").slice(-2)[0]);
