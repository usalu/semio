# Added Package Configuration Review

2026-10-28. Read-only audit coordinated with Rust and mutations owners. No compile, build, graph or browser run and no production edits. Seven owners confirmed by both writers and the retained build-owner rows.

## Seven Exact Owners

- semio-s-artifact-layout-layout-editor-tests
- semio-s-artifact-layout-layout-engine-scene-tests
- semio-s-artifact-layout-layout-engine-flow-tests
- semio-s-artifact-layout-layout-diff-plan
- semio-s-artifact-layout-layout-sqlite-member-tests
- semio-s-artifact-layout-layout-mutations-decisions
- semio-framework-os-store-replay-authority-tests

Parsed their actual Cargo manifests with the managed Bun TOML parser. Each has exactly one build-owners name row; its root matches its Cargo manifest directory. Each lib.path resolves to an existing source file. Each resolves its authored package.workspace to an existing workspace and appears exactly once in that workspace members array. The six Layout owners belong to the nested Layout artifact workspace; replay-authority-tests belongs to repository-root Cargo.toml, not the framework workspace. The decisions path also appears as a workspace dependency, which is not duplicate membership.

Every direct path dependency and inherited workspace dependency was resolved from its correct owning manifest directory and its target Cargo.toml exists. Path-backed dependency counts are editor14, scene19, flow5, diff-plan2, SQLite5, decisions9 and replay-authority5. No missing workspace dependency edge or missing file found. No new per-package project manifest is required: these are precise Cargo owner rows in the existing build-owner registry, while the existing artifact router owns the native suite selection.

## Test Roster and Fixes

The retained fresh canonical inventory names14unique selected test packages. Every recorded source/fixture input exists in the current workspace. Editor remains selected separately from editor-tests so its private args bridge laws execute. Engine remains selected separately from scene/flow companions so the four font laws execute. Earlier native-coverage-review.md documents no duplicated executable test source; the shared PDF structure file contains oracle helpers only.

Fresh-read confirms editor testkit forwards semio-framework-plugin/artifact-app-testing. This fixes the SDK test API dependency when consumed alone; it does not require the companion feature union to supply that edge. Fresh-read also confirms scene companion cfg(all(test,not(target_arch=wasm32))), keeping actual native filesystem/font APIs out of a wasm test projection. Flow remains platform-neutral. The retained companion-review inventory establishes scene native23inputs/218391bytes and wasm test1root/0active lines. No production wasm font path was removed.

## Path and Platform Boundary

All seven new absolute manifest paths are below256characters even counting UTF16code units for Windows. Maximum is decisions189UTF16units (176Unicode code points); replay authority141, editor167, scene177, flow176, diff-plan179 and SQLite179. New companion root source/schema filenames are shorter than those manifest paths in their same domain roots. Paths are repo-relative with ordinary Cargo separators rather than host absolute dependencies; no checked-in source copies were put under ticket directories.

This validates declared wiring and source presence at this workspace location. It is not a Windows/Linux/macOS compiler run or a promise about runtime generated Cargo cache directory depths. No concrete wiring defect found after the two companion corrections.
