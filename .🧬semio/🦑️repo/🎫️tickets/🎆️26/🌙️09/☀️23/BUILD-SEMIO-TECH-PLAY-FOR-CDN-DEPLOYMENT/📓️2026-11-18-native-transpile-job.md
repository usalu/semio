# native-transpile release job

Problem: `📜️stage.ts` shelled out to `repo cargo ...`, a nested Repo request that the native build driver refuses (`Undeclared original Repo build selection`).

Change (under `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🦀️native-transpile`):
- `📜️stage.ts` deleted; `📜️script.ts` added: `ScriptRouter` with a `build` command, `receiveScriptProcessInvocation`, builds `semio-native-transpile` via `buildAdmittedCargoArtifactsV1` (same provider as the font-assets tool; owner policy/admission through `scriptCompilerAdmissionControlV1`), verifies the Cargo observation inside `withCapturedCargoArtifactsTaskV1`, then copies the captured executable atomically to `.cache/b/p/m/browser/release/tools/native-transpile/<platform>/<name>` (unchanged destination, `nativeTranspileBinaryNameV1`).
- `📋️project.json` build command: `repo ./📜️script.ts build`.
- `🟦️.ts` docstring and error hint point to the new script.
- `🏢️semio-tech/🎡️play/📋️project.json` job `native-transpile` command: `["repo","./📜️script.ts","build"]`.
- No other release pipeline (root, mit-bestand, teaching) references the job.

Validation: see ticket run log in chat summary.

Validated: `repo capacity run --memory=3000 -- repo native-build-launch.ts @semio-tech/semio-tech-play:build <root> repo ./📜️script.ts build` (SEMIO_BUILD_MODE=release) exited 0 with the genuine build-caller envelope; binary staged at the unchanged destination.
