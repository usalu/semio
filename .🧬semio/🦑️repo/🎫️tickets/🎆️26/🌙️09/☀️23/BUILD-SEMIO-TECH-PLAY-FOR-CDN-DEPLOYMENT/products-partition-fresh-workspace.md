# Products partition in a fresh workspace

Root cause: `buildNativeTranspileV1` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🦀️native-transpile/📜️script.ts` published the tool with raw `mkdirSync`/`copyFileSync`/`renameSync` into `.cache/b/p/m/browser/<mode>/tools/native-transpile/<platform>/`. It never opened the products partition, so on a workspace without `.cache/b/p/fixed.dmg` it created a plain directory. The storage cleanup then rejected `.cache/b/p` ("neither fixed.dmg nor image.json but is not an empty shell").

Fix: the publication now runs inside `withRepositoryCacheStoreV1(workspace, "products", ...)`. That call opens or creates and mounts the image. The file is staged with `withArtifactScratchV1`, `withArtifactDestinationTaskV1`, `withRepositoryArtifactDestinationV1` and `stageRepositoryArtifacts`. This is the same path the descriptor emitter uses for `browser/tools/descriptor-emitter`.

Test: added a source-law test to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🌱️birth/🧪️tests/🟦️.ts` (bun test, 1 pass). It requires the products store and `stageRepositoryArtifacts`, and forbids raw fs writes in the script. A real end-to-end fresh-workspace run needs the full cargo and native image harness and was not run.

Clone: both files copied byte-identical (`cmp` ok) to `/Users/ueli/.sinc/play`. The plain `.cache/b/p` was removed. `repo cache gc`, run with `SEMIO_WORKSPACE_ROOT=/Users/ueli/.sinc/play`, created `.cache/b/p/{fixed.dmg,image.json}` and mounted `m` (disk43s1). `repo doctor` and `cache status` do not provision images.
