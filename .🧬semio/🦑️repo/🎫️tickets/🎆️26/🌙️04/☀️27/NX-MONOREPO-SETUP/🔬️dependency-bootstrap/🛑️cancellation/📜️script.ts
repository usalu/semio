import { mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
const root = process.cwd(), output = join(dirname(dirname(import.meta.dirname)), "🗑️generated");
mkdirSync(output, { recursive: true });
const { testDependencyCancellation } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️dependencies/🟦️.ts"));
await testDependencyCancellation(root, output);
