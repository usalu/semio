import { join } from "node:path";
const root = process.env.SEMIO_REPO_ROOT!;
const { testNativeRuntime } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️native-runtime/🟦️.ts"));
await testNativeRuntime(root, process.env.SEMIO_TEST_ARTIFACT_DIR!);
