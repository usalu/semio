# Taxonomy Generator Contracts

Completed `cad-geometry-bindings` and `spatial-kernel-session-bindings` in `🔣️taxonomy.json`:
- `previewTarget` set to `<project>:preview-generated`.
- `inputPatterns` sorted by UTF-16 code units (validator uses default `Array.sort`); the `**/*.rs` glob now precedes the `📦️packages/...` entries.
- Each owner project got a `preview-generated` target (`repo ./📜️script.ts preview-generated`, cwd = owner root) and a `PreviewGeneratedScript` command that reports the binding files the wasm build would write under `🕸️bindings` without writing.

Verification: plugin discovery no longer reports `Invalid taxonomy schema`; `repo show projects` now stops later at the unsealed whole-workspace graph.

Open, separate defect (not taxonomy): graph owner inference fails for both projects with `Generated task cannot consume its own output` (target `wasm`). The owner `📜️script.ts` hosts both `wasm` and the browser/source tests that import `🕸️bindings`. Flow solved this by moving the wasm producer to a separate build script (`🏗️build/📜️script.ts`). The same split is needed here.

## Producer split (coordinator decision)

Mirrors Flow: the `wasm` producer moved out of each owner `📜️script.ts` into `<module>/🏗️build/📜️script.ts` (CAD: `✏️s/🧑‍💻dev/📐️cad/🏗️build`, session: `.../🌊️session/🏗️build`), wired as `repo "../../🏗️build/📜️script.ts" wasm`. The `wasm` target now has explicit inputs (build script, Cargo files, `**/*.rs`, project.json, `cargoToolchain`, `sharedGlobals`) instead of `default`, which matched the whole project root including the test scripts that import `🕸️bindings` and caused the self-consumption error. `preview-generated` stays in the package script (read-only, taxonomy-routed). Taxonomy `inputPatterns` now name the build script instead of the package script, still lexically ordered. Original formatting of the existing `📋️project.json` lines was restored.

Verification: no new `Generated task cannot consume its own output` failures in the service log since the change (last ones predate it). The whole-workspace graph read is very slow here, so `repo run <project>:preview-generated` did not complete within 10 minutes and end-to-end success is unconfirmed.
