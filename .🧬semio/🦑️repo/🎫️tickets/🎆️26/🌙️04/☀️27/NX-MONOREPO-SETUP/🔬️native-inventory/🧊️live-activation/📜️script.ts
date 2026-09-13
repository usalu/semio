import { join } from "node:path";
const root = process.env.SEMIO_REPO_ROOT!;
const { testWgpuLiveActivation } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️live-activation/🟦️.ts"));
await testWgpuLiveActivation(root, process.env.SEMIO_TEST_ARTIFACT_DIR!);
