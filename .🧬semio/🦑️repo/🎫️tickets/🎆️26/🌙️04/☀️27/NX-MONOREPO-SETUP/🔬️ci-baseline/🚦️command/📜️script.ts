import { resolve } from "node:path";
const { testCiBaselineCommand } = await import(resolve(process.cwd(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧪️tests/🚦️baseline-command/🟦️.ts"));
await testCiBaselineCommand(process.cwd(), resolve(import.meta.dir, "../../🗑️generated"));
