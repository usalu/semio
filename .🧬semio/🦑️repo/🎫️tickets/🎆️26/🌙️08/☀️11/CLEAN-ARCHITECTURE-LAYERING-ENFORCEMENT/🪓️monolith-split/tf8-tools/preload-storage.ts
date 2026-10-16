import { mock } from "bun:test";
import { mkdirSync } from "node:fs";
import { join } from "node:path";
const storage = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/💾️storage/🟦️.ts";
const real = await import(storage);
const root = process.env.TF8_ARTIFACT_ROOT!;
for (const dir of ["", "tmp", "cache"]) mkdirSync(join(root, dir), { recursive: true });
mock.module(storage, () => ({
  ...real,
  repositoryTestArtifactRootV1: () => root,
  repositoryTestArtifactPathV1: (_workspace: string, ...parts: string[]) => join(root, ...parts),
  repositoryTestArtifactEnvironmentV1: (_workspace: string, base: NodeJS.ProcessEnv = process.env) => ({ ...base, SEMIO_TEST_ARTIFACT_DIR: root, SEMIO_DIAGNOSTIC_DIRECTORY: join(root, "tmp"), TMPDIR: join(root, "tmp"), TMP: join(root, "tmp"), TEMP: join(root, "tmp"), XDG_CACHE_HOME: join(root, "cache") }),
}));
