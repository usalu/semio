import { resolve } from "node:path";
const { testExtensionPackage } = await import(resolve(process.cwd(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/⚙️build/🧪️tests/📦️package/🟦️.ts"));
await testExtensionPackage(resolve(import.meta.dir, "../../../🗑️generated"));
