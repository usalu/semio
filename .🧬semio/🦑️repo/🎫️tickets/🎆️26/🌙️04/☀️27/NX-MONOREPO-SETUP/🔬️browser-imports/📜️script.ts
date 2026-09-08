import { join } from "node:path";
const { testBrowserModuleRelocation } = await import(join(process.cwd(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📜️script.ts"));
await testBrowserModuleRelocation(process.cwd());
