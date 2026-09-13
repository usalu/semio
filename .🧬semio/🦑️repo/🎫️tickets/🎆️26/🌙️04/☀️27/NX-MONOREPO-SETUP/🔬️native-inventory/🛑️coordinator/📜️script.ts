import { join } from "node:path";
const root = process.env.SEMIO_REPO_ROOT!;
const { testNxCoordinator } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🛑️coordinator/🟦️.ts"));
await testNxCoordinator(root);
