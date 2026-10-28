DOC = """🗂️ Staged batch `artdir` (lane td-misc): the owned-execution and testing/execution laws take the suite's SEMIO_TEST_ARTIFACT_DIR, or the system temporary directory in a bare `bun test ./file` (the house idiom `SEMIO_TEST_ARTIFACT_DIR??tmpdir()`), instead of failing on the missing variable; the budgeted-command law gets the explicit 60 s budget its two siblings already declare (bare it ran into Bun's 5 s default)."""
P = "🧰️framework/🔨️modules/🏃️process/"
EDITS = [
 (P + "🎛️owned-execution/🧪️tests/🟦️.ts", [
  ('import { resolve } from "node:path";\n', 'import { tmpdir } from "node:os";\nimport { resolve } from "node:path";\n'),
  ('const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "🧫️fixtures/🔣️.json"), "utf8"));\n', 'const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "🧫️fixtures/🔣️.json"), "utf8"));\n/** 🗂️ The suite\'s artifact directory, or the system temporary directory in a bare run. */\nconst artifactParent = process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir();\n'),
  ('  const artifactParent = process.env.SEMIO_TEST_ARTIFACT_DIR;\n  expect(artifactParent).toBeTruthy();\n  if (!artifactParent) return;\n  const root = mkdtempSync(resolve(artifactParent, "owned-execution-"));', '  const root = mkdtempSync(resolve(artifactParent, "owned-execution-"));'),
  ('const owner = resolve(import.meta.dir, "../🟦️.ts"), artifactParent = process.env.SEMIO_TEST_ARTIFACT_DIR!;\n  expect(artifactParent).toBeTruthy();\n', 'const owner = resolve(import.meta.dir, "../🟦️.ts");\n'),
  ('mkdtempSync(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!, "owned-groups-"))', 'mkdtempSync(resolve(artifactParent, "owned-groups-"))'),
  ('  const artifactParent=process.env.SEMIO_TEST_ARTIFACT_DIR!;expect(artifactParent).toBeTruthy();\n', ''),
 ]),
 (P + "🧪️testing/🎛️execution/🧪️tests/🟦️.ts", [
  ('import { existsSync, realpathSync } from "node:fs";\n', 'import { existsSync, realpathSync } from "node:fs";\nimport { tmpdir } from "node:os";\n\n/** 🗂️ The suite\'s artifact directory, or the system temporary directory in a bare run. */\nconst artifactParent = process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir();\n'),
  ('const output = process.env.SEMIO_TEST_ARTIFACT_DIR!; expect(output).toBeTruthy(); mkdirSync(output, { recursive: true });', 'const output = artifactParent; mkdirSync(output, { recursive: true });'),
  ('api.createTestScratchRootV1(process.env.SEMIO_TEST_ARTIFACT_DIR!, "scratch-dispose-")', 'api.createTestScratchRootV1(artifactParent, "scratch-dispose-")'),
  ('parent = process.env.SEMIO_TEST_ARTIFACT_DIR!;', 'parent = artifactParent;'),
  ('  } finally { rmSync(temporary, { recursive: true, force: true }); }\n});\n\nconst system', '  } finally { rmSync(temporary, { recursive: true, force: true }); }\n}, 60000);\n\nconst system'),
 ]),
]
