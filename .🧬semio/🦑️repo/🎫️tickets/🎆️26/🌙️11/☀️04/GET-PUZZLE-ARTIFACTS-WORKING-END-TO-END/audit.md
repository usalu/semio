# Puzzle Artifact Read-Only Audit

## Scope

Read root, s, and puzzle AGENTS.md; inspected puzzle artifact entry points, manifests, package targets, schema/test ownership and browser acceptance harnesses. All commands used `repo tools exec --light python`. No source, AGENTS, or git changes; no executable tests run in this audit.

## Ownership Map

Puzzle root: `✏️s/🔌️plugins/🧩️puzzle`.

Each artifact has its own workspace under `🗿️artifacts/◻️2d`, `🗿️artifacts/🧊️3d`, or `🗿️artifacts/🖐️5d`, separate from the plugin root workspace. Each owns `🏅️standards/🔖️1/🪆️subsets/✳️any`, including schema, mutation catalogs, viewer/editor, I/O, examples, and portable JSON fixtures. Native host implementation resides under `🔨️modules/🏠️host/🧰️owned`. Root artifact Rust files reexport many narrow owner crates. Common board scenes, jobs and retained command infrastructure belong to plugin-level `🔨️modules/🎲️board/🎬️scene`, `🎮️commands/📤️jobs`, and `🎮️commands/🧵️retained`.

Editor publication owners are Puzzle2dPlayApp, Puzzle3dPlayApp, and Puzzle5dPlayApp. Their authored route and lane fixture is `🧪️tests/🧫️fixtures/🔏️publication-authority/🔣️.json`; all current groups declare migrated status. Lanes distinguish artifact, config, window-config, window-transient, interaction, and host-only. Fixture laws include authority before decode, exact contracts, bounded progress, cancellation, freshness, incremental close, and empty terminal state.

## Verification Map

- `@semio-tech/puzzle-js:test` includes transform gesture, renderer boot variants, committed example tests, and dev diff parsers. It does not run the browser/Storybook, time-travel, or publication-authority gates.
- `@semio-tech/puzzle-js:publication-authority-audit` validates window schemas with Ajv and audits source declarations against an independent fixture, including hostile source mutations. It is a source-string oracle, so its success alone cannot establish runtime end-to-end behavior.
- Each artifact Rust package offers build/check/test, graph-generate, graph-wire-check, preview-generated, snapshot SQLite source/native tests and source type verification. 2d additionally exposes store-close and presence source/native targets. 3d/5d expose original-work construction verification; 3d also has original fill fixture source/native gates.
- Native editor tests include agent-lane execution, retained jobs, mounted presence, store envelope/replay, snapshot SQLite, mutation/inverse/fixture laws and source custody. Portable fixtures and third-party Ajv provide useful existing independent validation infrastructure.
- `🧪️tests/⏪️time-travel/🟦️.ts` is a substantial actual-app end-to-end scenario with React and wgpu, English and German, selection drag mutation editing, undo/redo, persistence reload, alternatives, peers, long replay progress/cancel, device widths and ARIA independent oracles. Run through `repo run @semio-tech/framework-os-dev:time-travel -- --scenario <scenario path> --renderer react|wgpu --locales en,de --out <ticket generated dir>`. It can start or reuse its serve. This harness is primarily Puzzle2d, so equivalent 3d/5d lifecycle coverage should be established separately.
- `🧪️tests/◻️storybook-2d/🟦️.ts` uses Playwright pointer/wheel/keyboard input and CPU session debug output for selection, zoom, deletion and brush placement. `🧪️tests/🧊️storybook-spatial/🟦️.ts` is the spatial suite. These are outside the normal puzzle-js gate.
- Renderer boot fixture covers puzzle2d, puzzle3d, 3d alias and puzzle5d app IDs.

## Concrete Inconsistencies and Risks to Investigate

1. `🗿️artifacts/◻️2d/🦀️.rs:15` unconditionally imports retained IDs from `semio_s_artifact_puzzle_2d_unit_tests`; its normal Cargo dependencies include that unit-test crate at `📦️packages/🦀️rust/Cargo.toml:47`. The cfg(test) catalog instead takes the IDs from mut_any_10. This production-to-test ownership coupling is suspicious and should be resolved at the production owner rather than hidden by an adapter. It is not yet a demonstrated runtime failure.
2. Artifact root 2d and 5d files unconditionally import framework fixture readers; 2d lists fixture reader in both regular and dev dependencies. Determine whether production actually requires fixture ownership, then move test-only ownership behind test configuration where applicable.
3. Separate workspaces and narrow owner crates mean plugin-root checks cannot establish all artifact checks passed. Validate all three artifact roots and actual mounted apps.
4. Source-string publication assertions can pass while runtime dispatch, renderer wiring, persistence or authority transfer fails. Run native mounted and browser journeys after source gates.
5. Default artifact manifests have empty node-kind catalogs. This may be intentional because authored examples supply catalogs; verify initial app boot and chosen example hydration across every renderer and variant.

No TODO, todo!, unimplemented!, or explicit not-implemented markers were found in the inspected puzzle Rust/TypeScript tree. This does not prove completeness. Existing schema/fixture/test surfaces are extensive; prioritize executing them and tracing the first real failure rather than introducing parallel contracts.
