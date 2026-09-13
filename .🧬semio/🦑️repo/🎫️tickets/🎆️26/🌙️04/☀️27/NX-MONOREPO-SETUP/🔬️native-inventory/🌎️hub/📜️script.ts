import { join } from "node:path";
const root = process.env.SEMIO_REPO_ROOT!;
const { testHubBuild } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🌎️hub/🟦️.ts"));
await testHubBuild(root);
