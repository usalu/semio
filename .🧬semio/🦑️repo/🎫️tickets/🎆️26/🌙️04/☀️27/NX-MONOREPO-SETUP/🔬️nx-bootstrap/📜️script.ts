import { mkdirSync } from "node:fs";
import { join } from "node:path";
const { testNxBootstrap } = await import(join(process.cwd(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🚀️bootstrap/🟦️.ts"));
const output = join(import.meta.dirname, "../🗑️generated");
mkdirSync(output, { recursive: true });
await testNxBootstrap(process.cwd(), output);
