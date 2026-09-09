import { join } from "node:path";
const root = process.env.SEMIO_LAYOUT_REPO_ROOT;
if (!root) throw new Error("SEMIO_LAYOUT_REPO_ROOT is required");
const { testCiBaseline } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🧪️tests/🧭️baseline-selection/🟦️.ts"));
await testCiBaseline(root);
