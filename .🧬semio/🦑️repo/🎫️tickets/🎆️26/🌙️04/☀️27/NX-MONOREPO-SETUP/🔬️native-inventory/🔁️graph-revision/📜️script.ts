import { join } from "node:path";
const root=process.env.SEMIO_REPO_ROOT!;
const { testGraphRevision } = await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔁️graph-revision/🟦️.ts"));
await testGraphRevision(root,process.env.SEMIO_TEST_ARTIFACT_DIR!);
