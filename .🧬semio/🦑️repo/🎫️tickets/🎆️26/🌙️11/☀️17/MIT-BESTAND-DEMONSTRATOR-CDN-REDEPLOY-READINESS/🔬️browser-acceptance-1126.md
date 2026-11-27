# Browser acceptance 2026-11-26 (TypeScript/UI layer)

## Unit tests
`repo test @semio-tech/mit-bestand-demonstrator --test_level quick`: 105 passed in 5 files after three fixes.
1. `native-build/🟦️.ts` (framework, concurrent edit) used a constructor parameter property (`private readonly resolverLock`). Node strip-only mode refuses it and the whole activation suite failed to import. Replaced with a declared field and an assignment.
2. `demonstratorRuntimeBuildVariants` and `demonstratorActivationLanes` tests held the pre-carve roster. The catalog now yields `cad, puzzle3d, aussuchen, gis2d, energy, fem3d` and lanes `generator + those six`. Tests updated.
3. `activate-lanes-prototype` (project.json, compile-closure fixture `cases.json`, README) only depended on energy/fem3d/generator. It now orders all seven lane tasks (aussuchen, cad, energy, fem3d, generator, gis2d, puzzle3d).

## Site build
`MEM=4096 native-wasm.sh site` fails at the Vite writeBundle: `ENOENT .cache/b/p/m/browser/release/modules/🗄️stdio/.semio-artifact.json`. The Stdio component is not built yet (owner: rebuild agents, Stdio). Everything else (Vite bundling, 12 s) passes. The acceptance below ran against the previous candidate `.cache/b/p/m/vite/release/5dc1b077e86de2aa/out` (built Nov 25 14:36, predates the menu renderer fix and all pending WASM).

## Size and deploy root
Candidate: 2,103 files, 813,072 KiB (826,856,511 B), limit in `package-site.py` is 1,000,000,000 B. Largest dirs: plugin-modules 448 MB, extension-modules 156 MB, assets 99 MB, vt 70 MB. Headroom is about 170 MB; Stdio and the refreshed components will add to it, so re-measure after the rebuild. Vite `base` is `./`, CNAME is `demonstrator.entwerfen.mit-bestand.de`, so root deployment works and relocatability is not required. No unreferenced media was found worth dropping.

## Browser run (static origin cdn.localhost, 1 worker, retries 0)
Runner: `browser-suites.sh <site> <port> both`; summary: `summarize-browser-suites.py`. Logs: `🗑️generated/bs-baseline.log`, `bs-behaviours.log`, JSON `bs-*.json`. Each stale-guest failure burns about 2 min, so the 96 cases take hours; the run was still going when this was written (20 of 52 baseline executed).

Baseline (en) so far, 10 pass / 10 fail:
- pass: pane roster, overview chrome en/de, overview scroll focus, language picker, language switch, aggregator boot, energie boot, verfolgen boot, statik boot.
- fail, all component-owned (no TS/UI/test fault found):
  - koordinator boot, language switch live (opens koordinator): CAD guest never answers `Set Active Example` (stale CAD WASM).
  - generator boot, generator undo-redo: stale Procedural guest emits `[DEBUG] follow unit` / `[DEBUG] gen-preview` logs that are absent from source, preview scene has 0 meshes (stale Procedural and standalone Flow engine).
  - aussuchen boot, bearbeiten boot, aussuchen undo-redo: console shows `[DEBUG] big work quote ...` and `unsupported backtrace`; the string exists nowhere in source, so the shipped standalone Sourcing component is stale.
  - aussuchen preview: selecting a pool row never reaches the Preview window. The authored descriptor (`🪵️sourcing/🧩️component/🔣️.json`, sourcing-preview) already declares `interactions: ["rows"]`; the shipped one is stale.
  - energie run-cancel: stale Energy guest panics `terminal_is_empty` in `🧵️run-job`, wasm trap.
  - statik undo-redo: Results window stays at `Analysis queued`; FEM component not rebuilt (FEM release failed in composition library).
- Test-side ID faults: none remaining; Statik and Energie tests already address the real controls.
- Menu renderer empty-option fix: landed (fixture `empty-selection.json`), not in the candidate site yet.

## Attribution
Rebuild agents: Stdio (missing, blocks the build), Sourcing (Aussuchen/Bearbeiten), Procedural + Flow engine, CAD, Energy, FEM, Playbook. After they land: rerun `native-wasm.sh site`, then `accept-site.sh`.
