import { join, resolve } from "node:path";
const { testBunDependencies } = await import(join(process.cwd(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📜️script.ts"));
await testBunDependencies(process.cwd(), resolve(import.meta.dir, "../../🗑️generated"));
