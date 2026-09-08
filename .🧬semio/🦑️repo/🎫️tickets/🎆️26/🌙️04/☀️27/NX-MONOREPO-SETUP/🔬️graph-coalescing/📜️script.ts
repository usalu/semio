import { readFileSync } from "node:fs";
import { join } from "node:path";
const root = process.cwd();
const { testGraphCoalescing } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🕸️daemon/📜️script.ts"));
await testGraphCoalescing(root, process.argv[2] ? readFileSync(process.argv[2], "utf8") : undefined);
