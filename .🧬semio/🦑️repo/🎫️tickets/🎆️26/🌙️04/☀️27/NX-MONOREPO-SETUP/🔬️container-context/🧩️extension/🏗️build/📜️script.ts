import { resolve } from "node:path";
const { testExtensionHostBuild } = await import(resolve(process.cwd(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/⚙️build/🧪️tests/🧩️host-build/🟦️.ts"));
await testExtensionHostBuild(resolve(import.meta.dir, "../../../🗑️generated"));
