import { join } from "node:path";
const root = process.env.SEMIO_REPO_ROOT!;
const { testNxTooling } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️dependencies/🟦️.ts"));
await testNxTooling(root, process.env.SEMIO_TEST_ARTIFACT_DIR!);
