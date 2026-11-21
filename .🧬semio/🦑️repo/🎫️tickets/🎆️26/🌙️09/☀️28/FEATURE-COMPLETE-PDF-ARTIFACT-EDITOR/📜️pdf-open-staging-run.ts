/** 🧪️ Runs focused PDF regressions under the repository's Vitest owner policy. */
import { resolve, join } from "node:path";
import { pathToFileURL } from "node:url";
const root = process.cwd();
const owner = pathToFileURL(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🧪️vitest/🟦️.ts")).href;
const { runVitest } = await import(owner);
process.env.SEMIO_TEST_ARTIFACT_DIR ??= join(import.meta.dir,"🗑️generated");
await runVitest(root, ["--maxWorkers=1", "--reporter=verbose"], join(import.meta.dir, "📜️pdf-open-staging-tests.ts"));
