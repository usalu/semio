import { join } from "node:path";
const root = process.env.SEMIO_REPO_ROOT!, output = process.env.SEMIO_TEST_ARTIFACT_DIR!;
const { testStylingOutputs } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎨️styling-outputs/🟦️.ts"));
await testStylingOutputs(root, output);
