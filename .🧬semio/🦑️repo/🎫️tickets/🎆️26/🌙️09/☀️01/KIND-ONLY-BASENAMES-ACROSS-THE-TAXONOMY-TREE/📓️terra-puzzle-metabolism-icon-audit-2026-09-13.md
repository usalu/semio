# Puzzle Metabolism Icon Acceptance Audit — 2026-09-13

## Status

**Accepted within the stated native and cache limits.** The move, taxonomy contexts, parent mount, 29-asset closure, bounded native output proof, and exact 32-source collector are correct.

## Scope

This audit covers only the move of the Puzzle 2d metabolism SVG lookup:

- Removed source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔣️icons/🧩️metabolism.rs`.
- Current owner: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔣️icons/🌱️metabolism/🦀️.rs`.
- Parent mount: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔣️icons/🦀️.rs`.
- Rust project and input declaration: `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📋️project.json`.

It excludes full Puzzle rendering, canvas behavior, browser execution, WGPU, and an ordinary Puzzle Nx build/test run.

## Current source and taxonomy

The old file is absent. The current anonymous Rust leaf has 29 distinct `include_str!` calls, and the control's 29 declared assets resolve one-for-one to those calls; every resolved asset exists. The leaf exposes only the catalogue lookup. The parent has exactly one `#[path = "🌱️metabolism/🦀️.rs"]` mount and delegates once to it. Board-host imports `icons::puzzle_themed_icon_lookup`, and the existing icon unit test imports that public parent lookup rather than the private leaf.

The current taxonomy registry gives `engine-icons` the `engine` owner and `🔣️icons` member. It gives `metabolism-icon-catalog` the `engine-icons` owner and `🌱️metabolism` member. This establishes the requested nested context; it does not retain the former generic `members-of-engine` classification.

## Behaviour evidence

The root-owned control at `📋️puzzle-metabolism-controls/🔣️.json` fixes the surface to 29 keys/assets and 35 queries, including trimming variants, unknown keys, empty input, and case mismatch. Its schema fixes the cardinalities, the `metabolism-icon-catalog` member kind, and the actual icon unit-test module as a source-data input.

Root's completed `before-result.json`, `after-result.json`, and `parent-result.json` under `🗑️generated/coordinator/puzzle-metabolism-source` each record 35/35 native-harness results and 35/35 installed-lodash results. Before the move, the old source was present, the destination absent, the classification was `members-of-engine`, and the kind-only basename finding existed. After the move and through the public parent, the old source is absent, the destination is present, the classifications are `engine-icons` and `metabolism-icon-catalog`, no basename finding remains, and the parent is bound. All 29 asset digests match across phases for a total of 695,606 bytes.

This is a suitable direct owner and independent data-oracle proof. It proves lookup values and the parent mount, not rendering or browser behaviour.

## Nx input and cache boundary

The Puzzle Rust project's `default` input covers every Puzzle Rust source plus `nativeAssets`. `nativeAssets` covers the exact metabolism SVG subtree and excludes generated SVGs. Thus the moved owner is within the Rust-source glob and its 29 source-as-data assets are declared inputs. The project has a cached `wasm` target; the audit has not inferred a passing cached run from this static closure. The `test` targets do not declare `cache: true` here.

The repaired real source collector lists exactly 32 present files: the moved owner, parent, existing icon unit-test leaf, and 29 SVGs. Its current receipt records `tracked: 32`, `expectedTracked: 32`, `exactSet: true`, `oldExcluded: true`, and `allPresent: true` after 2.731 seconds including taxonomy load. It also rejects the metabolism member below each wrong parent context: `engine`, `members-of-engine`, `artifacts`, and `modules` all resolve to `null`. This closes both source-data and nested-context admission.

## Limits

I did not rerun the Rust harness, an Nx target, a Cargo build, canvas tests, or a browser test. The native and lodash results above are attributed to the root's retained controls. No full-Puzzle, WGPU, or browser outcome is claimed.
