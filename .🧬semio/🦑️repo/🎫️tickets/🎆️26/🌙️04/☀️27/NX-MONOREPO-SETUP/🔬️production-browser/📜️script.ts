import { resolve } from "node:path";
const root = process.cwd();
const { testProductionBrowserArtifacts } = await import(resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🟦️.ts"));
await testProductionBrowserArtifacts(root, resolve(import.meta.dir, "../🗑️generated"));
