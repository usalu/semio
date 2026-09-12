import { testGeneratorOwnership, testWgpuGeneratorOwnership, testWgpuGeneratorPublication } from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧬️generator-ownership/🟦️.ts";
await testGeneratorOwnership(process.env.SEMIO_REPO_ROOT!, process.env.SEMIO_TEST_ARTIFACT_DIR!);
await testWgpuGeneratorOwnership(process.env.SEMIO_REPO_ROOT!);
await testWgpuGeneratorPublication(process.env.SEMIO_REPO_ROOT!, process.env.SEMIO_TEST_ARTIFACT_DIR!);
