# 1108 graphenv result

## Cause
The error comes from `build()` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs` (line ~468). It fires when a stored owner is neither dirty, returned, carried nor removed.

Sealed generation 901 (`.cache/b/g/m/generations`) held the crate as a Cargo-config owner: name `semio-framework-pack-json-output-receiving`, config `Cargo.toml`. The checked-in `📋️build-owners.json` and a newly added `📋️project.json` declared the same root as project owner `@semio-tech/framework-pack-json-output-receiving-rs`.

Two owners then claimed one root with different configs and names, and the stale one was never recomputed. It is the only crate of 4963 owners with both a `project.json` and a Cargo-config owner at the same root. The sibling crates (value-testing-admission, clone-source-testing, value-paged-*) are consistently project-owned, so they are not affected.

## Fix
- Deleted `.../receiving/🫴️unique/📦️packages/🦀️rust/📋️project.json`. The crate is now Cargo-only, like the other 3104 Cargo-config owners.
- `📋️build-owners.json`: replaced the `@semio-tech/...-rs` row with `semio-framework-pack-json-output-receiving` (manifest `Cargo.toml`), placed after `semio-framework-pack-json-native-input`.

## Generator fix
`plan()` in `generations/🟨️.mjs`: when a config file is added at the root of an already-owned directory whose owner has a different config (project.json over Cargo.toml, or the reverse), the prior owner now goes into `removed`. Only same-directory configs count, so a nested crate never drops its parent. Test: `generations/🧪️tests/🔁️owner-replacement/🟦️.ts` (3 tests: project.json over module.json owner, module.json over project.json owner, nested crate keeps its parent). The synthetic world (`🏃️world/🟦️.ts`) now also reads `module.json` as a second config kind; `"primary":true` decides which config wins when a root has both. Run: `bun test` in that folder, 3 pass. With the fix removed, the two replacement tests fail (nested one still passes). All 20 generations test files: 67 pass, 2 fail (`🌊️stream` external-names and `🧹️retire` artifact-dir, neither touches plan() or the world owners).

## Stall (W-graphenv-1)
Killed leaf-first (bun eval 77077/76991, bun 35393, wrapper 35391, slot.sh 26441). The idle bun child was the repo capacity gate: "not enough free memory (need 5632 MB ... behind `@semio-tech/mit-bestand-demonstrator:test` 3.2-3.7 GB, semio-plugin-abi / decl-surface, check)". It was waiting, not wedged.

## The two red generations tests
Both were red before my change (files unchanged vs HEAD, my `.mjs` edit is outside `assemble`/retire).
- `🧹️retire`: needs `SEMIO_TEST_ARTIFACT_DIR` from the test runner; with it set, 1 pass. Not a defect.
- `🌊️stream` external-names: stale test. `assemble` now takes `externalInputs` (the test passed `external`) and an unknown name refuses with `GENERATION_UNKNOWN_EXTERNAL`. Test renamed and updated; 6 pass.

## Rerun W-graphenv-2
rc 0 (`$G/W-graphenv-2.txt`, label W-graphenv-2). Capacity wait about 15 min, run 6982 s under fleet load.
- Graph generation 902 published: 5067 projects refreshed in 791 s, fact index 734320 facts of 5068 projects. `semio-framework-pack-json-output-receiving` is now a single owner.
- `@semio-tech/ui-rs:generate` and `workspace:schema-generate` passed (2m 19s). `🔣️schema-catalog.json` regenerated: 5922 scopes, 14435 diagnostics. Many withheld-project warnings (peers' unfinished crates) are in the log; none is an owner-edge error.

STATUS: FIXED-verified W-graphenv-2
