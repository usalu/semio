import { join } from "node:path";
const root = process.env.SEMIO_REPO_ROOT!;
const { testRuntimeComponents } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts"));
await testRuntimeComponents(root);
