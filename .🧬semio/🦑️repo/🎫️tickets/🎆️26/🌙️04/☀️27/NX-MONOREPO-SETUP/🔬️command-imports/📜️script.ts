import { resolve } from "node:path";
const { testCommandImportClosure } = await import(resolve(process.cwd(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔗️command-imports/🟦️.ts"));
await testCommandImportClosure(process.cwd(), resolve(import.meta.dir, "../🗑️generated"));
