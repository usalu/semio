import { join } from "node:path";
const root = process.env.SEMIO_REPO_ROOT!;
const { testNativeDependencies } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️native-dependencies/🟦️.ts"));
await testNativeDependencies(root, process.env.SEMIO_TEST_ARTIFACT_DIR!);
