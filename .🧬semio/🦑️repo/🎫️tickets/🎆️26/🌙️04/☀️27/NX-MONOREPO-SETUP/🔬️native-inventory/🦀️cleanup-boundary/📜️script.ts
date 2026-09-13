import { join } from "node:path";
const root = process.env.SEMIO_REPO_ROOT!;
const { testCargoCleanupBoundary } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🦀️cleanup-boundary/🟦️.ts"));
await testCargoCleanupBoundary(root, process.env.SEMIO_TEST_ARTIFACT_DIR!);
