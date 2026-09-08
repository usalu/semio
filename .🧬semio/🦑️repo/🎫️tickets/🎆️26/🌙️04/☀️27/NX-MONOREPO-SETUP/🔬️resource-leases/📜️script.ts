import { mkdirSync } from "node:fs";
import { resolve } from "node:path";
const output = resolve(import.meta.dirname, "../🗑️generated");
mkdirSync(output, { recursive: true });
const { testResourceLeases } = await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔒️leases/🧪️tests/📜️script.ts"));
await testResourceLeases(output);
