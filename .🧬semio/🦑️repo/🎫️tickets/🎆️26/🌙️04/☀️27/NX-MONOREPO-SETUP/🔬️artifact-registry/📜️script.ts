import { join } from "node:path";
import { mkdirSync } from "node:fs";
const workspace = process.cwd();
const output = join(import.meta.dirname, "../🗑️generated");
mkdirSync(output, { recursive: true });
const { testArtifactRegistry } = await import(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📇️artifacts/📜️script.ts"));
await testArtifactRegistry(workspace, output);
