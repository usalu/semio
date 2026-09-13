import { join } from "node:path";
const root = process.env.SEMIO_REPO_ROOT!, output = process.env.SEMIO_TEST_ARTIFACT_DIR!;
const { testProductionBrowserArtifacts } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🟦️.ts"));
await testProductionBrowserArtifacts(root, output);
