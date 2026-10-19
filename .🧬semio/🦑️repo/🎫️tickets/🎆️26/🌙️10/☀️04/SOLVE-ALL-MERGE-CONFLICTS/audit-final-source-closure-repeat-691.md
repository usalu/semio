# Final Static Source Closure — Repeat 691

Immutable refs: base `1bae6115bc56bf5e1b01212774dd76877a5f64fe`; HEAD `36fbe9da786b71ec7b6946a1fc1cbb4e9d4d25ec`; stash `a09b7afdff001185403f0decb7cb0714a7bb839e`. Root AGENTS.md read in this audit lane. Final raw-byte snapshot taken after parent reported Store/shared/Hub checkpoint complete.

## Evidence and Scope

Compared all 8654 incoming paths and 192 overlap paths, plus all 2922 HEAD-only changes. Blob hashes use actual file bytes and Git blob framing, without CRLF normalization. An initial exploratory normalized hash count was discarded because authored CRLF blobs must retain their exact bytes. No source writes, Git mutations, builds, compiler invocation, tests, application runtime or dependency resolution occurred. Python JSON/TOML parsers were used only for static source inspection.

Incoming: 8641 stash-exact, 13 merged/divergent. Overlap: 177 merged/divergent, 14 HEAD-exact, one stash-exact. Every incoming/overlap divergence or HEAD-exact selection has an explicit literal-path preservation/relocation ledger. These are source-accounting dispositions; a ledger reference does not establish runtime correctness.

HEAD-only: 56 paths have deliberate integration changes; zero missing HEAD-only paths and zero exact base reversions. 53 already have literal-path ledger references. Three remaining HEAD-only files were independently read in their complete HEAD-to-working diffs here:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/📦️packages/🦀️rust/Cargo.toml`: adds the value snapshot-read dependency for relocated weaker-bound Store source. Existing HEAD manifest content retained.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🗄️stage/📦️packages/🦀️rust/Cargo.toml`: adds DSL record and Store canonical-JSON dependency seams for relocated stage edits; existing HEAD rows retained.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🏭️factory/📥️lifted/🏪️store/🦀️.rs`: preserves current extracted codec owner while reducing Clone/FromValue generic bounds in four source declarations and adopting the kernel Store snapshot trait path. No owner revival or whole-file reversion. This is a recorded source disposition, not an API validity claim.

Renderer ledger now explicitly records Runtime scope/channel/turn/contract/invocation owners, three topic test owners, and mailbox apply source-contract adjustment. Parent additionally records FrameOriginalRecovery enum/impl at the mailbox runtime owner, diagnostic imports at invocation leaf and current host trace qualifier. Store/source audit runs independently; later edits require rechecking this snapshot.

## Static Syntax and Path Inspection

Across union of incoming paths and HEAD changes, excluding ticket evidence: 2367 strict JSON documents and 214 TOML documents parsed. Only known authored Pack borrowed-document cursor schema failed strict JSON at EOF character630 (630 bytes); preserved per explicit instruction. JSONC devcontainer excluded from strict JSON. No anchored conflict-marker lines found in the source/config extension scan, so there are no newly introduced marker lines requiring source-source comparison. New files not present at HEAD had no absolute Windows path exceeding256 UTF-16 code units.

No unexplained incoming/overlap path omission was found. No missing or reverted HEAD-only path was found. This finite audit does not assess compilation, semantic completeness, runtime behavior or generated output availability.

## Incoming and Overlap Exception Ledger

| Path | Kind / snapshot | Existing disposition report |
| --- | --- | --- |
| `.vscode/launch.json` | overlap / merged-or-other | `launch-preservation-repeat-691.md` |
| `.vscode/🧩️launch.seed.jsonc` | overlap / merged-or-other | `launch-preservation-repeat-691.md` |
| `Cargo.lock` | overlap / merged-or-other | `domain-result-repeat-691.md`, `framework-inventory-repeat-691.md`, `global-cargo-preservation-repeat-691.md` |
| `Cargo.toml` | overlap / merged-or-other | `audit-repo-files-repeat-691.md`, `domain-divergences-repeat-691.md`, `domain-result-repeat-691.md`, `framework-inventory-repeat-691.md`, `global-cargo-preservation-repeat-691.md`, `plugin-result-repeat-691.md` |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🟦️.ts` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🟦️.ts` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🟦️.ts` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🟦️.ts` | overlap / head-exact | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/⚙️systems/🟦️.ts` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️corpus/🟦️.ts` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/⚡️model/🟦️.ts` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📜️script.ts` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📜️script.ts` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔺️ldlt/🧪️tests/📚️paged-ldlt/🦀️.rs` | incoming / merged-or-other | `domain-result-repeat-691.md` |
| `✏️s/🧑‍💻dev/🏗️fem/📦️packages/🟦️typescript/📜️script.ts` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `🌎️hub/Cargo.lock` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `global-cargo-preservation-repeat-691.md` |
| `🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/🚢️shipped-fleet/🪶️sqlite/🟦️.ts` | overlap / head-exact | `framework-inventory-repeat-691.md` |
| `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📜️script.ts` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md` |
| `📜️script.ts` | overlap / head-exact | `audit-independent-repo-globals-repeat-691.md`, `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md`, `domain-divergences-repeat-691.md`, `domain-inventory-repeat-691.md`, `domain-result-repeat-691.md`, `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md` |
| `🧪️tests/🧪️diff-only-law-gate/🟦️.ts` | overlap / merged-or-other | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md` |
| `🧪️tests/🧪️diff-only-law-gate/🫧️lanes/🟦️.ts` | overlap / merged-or-other | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md` |
| `🧰️framework/Cargo.lock` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `global-cargo-preservation-repeat-691.md` |
| `🧰️framework/📦️packages/🟦️typescript/package.json` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/📦️packages/🟦️typescript/📋️project.json` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/📦️packages/🟦️typescript/📜️script.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🔨️modules/🏃️process/📜️script.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🔨️modules/📡️replication/🎮️mutation/♻️retirement/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📋️project.json` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌈️Surface/🟦️.tsx` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔣️Icons/🟦️.tsx` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🟦️.tsx` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🚗️UiDriver/🟦️.tsx` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🔨️modules/🛂️manifest/🖼️view-model/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🔨️modules/🧵️job/⏱️budget/🧪️tests/🔬️tool-job-microsecond-budget/🟦️.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🧪️tests/🔬️flow-selected-copy/🟦️.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📨️diagnostics/🦀️.rs` | incoming / merged-or-other | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/🦀️.rs` | incoming / merged-or-other | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🔁️replay/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🔁️replay/🧪️tests/🔬️unit/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🤝️authority/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧪️tests/🔬️unit/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧵️planning/🎒️pack/🦀️.rs` | incoming / merged-or-other | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧵️planning/📐️budget/🦀️.rs` | incoming / merged-or-other | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧵️planning/📤️source/🦀️.rs` | incoming / merged-or-other | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧵️planning/🔗️bound/🦀️.rs` | incoming / merged-or-other | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧵️planning/🦀️.rs` | incoming / merged-or-other | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧵️planning/🧷️member/🦀️.rs` | incoming / merged-or-other | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧵️planning/🪪️decision/🦀️.rs` | incoming / merged-or-other | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/📇️index/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/🧬️mutations/🌿️create-space-alternative/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/🧬️mutations/🎯️set-active-space-alternative/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/🧬️mutations/📌️commit-space-checkpoint/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/🧬️mutations/🔀️switch-space-alternative/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/🧬️mutations/🗑️remove-space-checkpoint/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/🧬️mutations/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/🧬️mutations/🧹️remove-space-alternative/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️canonical-error-progress/🟦️.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️store-canonical-edit-sealer/🟦️.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🧪️tests/🔬️unit/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `renderer-rust-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📮️runtime-mailbox-core/🎟️apply/🧪️tests/🟦️.ts` | incoming / merged-or-other | `renderer-tests-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🎨️presentation-raster/🌐️native-asset-stream/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `renderer-rust-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🎨️presentation-raster/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `renderer-rust-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `renderer-rust-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🔬️native-socket-probe/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🖥️frame-presentation/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🔁️runtime-apply/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `renderer-rust-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `renderer-rust-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🧠️app-runtime/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🧾️frame-build/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `renderer-rust-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🧾️frame-build/🧮️frame-transaction/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `renderer-rust-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `renderer-rust-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/♿️editable-controls/🟦️.tsx` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎟️original-turn/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `renderer-rust-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx` | overlap / head-exact | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` | overlap / head-exact | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧩️shell-core-state/🟦️.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎛️prefs/🗨️chrome-overlays-and-tour/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `renderer-rust-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧪️wgpu-fault-notices/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `renderer-rust-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `plugin-runtime-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🔩️plumbing/🟦️.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🧵️instance/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🧵️instance/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/♻️cancellation/🦀️.rs` | incoming / merged-or-other | `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎞️media/🛫️intrinsic/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎮️command/📥️admission/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👥️peer/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🏛️reserved/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🛬️lifted/🔄️full-operation/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔔️wake/📃️policy/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🪶️snapshot/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️runtime-metrics-publisher/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🎟️grant/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧪️tests/🔬️unit/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🗃️document/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪆️child/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🦀️.rs` | overlap / head-exact | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🪪️identity/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🪪️metadata/🦀️.rs` | overlap / head-exact | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🦀️.rs` | overlap / merged-or-other | `domain-divergences-repeat-691.md`, `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/♻️publication-retirement-authority/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🟦️.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-artifact-fixed-registry/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/📨️owned-child-dispatch/🪪️admission/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🪟️mounted-owned-child/📦️driver/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-coverage/🟦️.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-factory-proof-join/🟦️.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️tool-run-member/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📥️staging/🔒️close-ladder/🦀️.rs` | overlap / head-exact | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/📨️dispatch/🦀️.rs` | overlap / head-exact | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-relocations-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🚪️ingress/🦀️.rs` | overlap / head-exact | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-relocations-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🤝️group/🦀️.rs` | overlap / head-exact | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-relocations-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/🦀️.rs` | overlap / head-exact | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-relocations-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/🛠️tool-machine/🦀️.rs` | overlap / head-exact | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-relocations-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/🦀️.rs` | overlap / head-exact | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-relocations-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-scalar-config-cohort/🟦️.ts` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪆️child/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md`, `plugin-inventory-repeat-691.md`, `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪜️maintenance-ladder/📥️staging/🗝️envelope-ladder/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs` | overlap / merged-or-other | `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/📋️project.json` | overlap / merged-or-other | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md`, `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/📜️script.ts` | overlap / merged-or-other | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md`, `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🟦️.ts` | overlap / merged-or-other | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md`, `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧪️tests/🎛️dashboard-only/🟦️.ts` | incoming / merged-or-other | `audit-repo-files-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧪️tests/🧩️extension/🟦️.ts` | overlap / merged-or-other | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md`, `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts` | overlap / merged-or-other | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md`, `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts` | overlap / merged-or-other | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md`, `domain-divergences-repeat-691.md`, `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts` | overlap / merged-or-other | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md`, `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` | overlap / merged-or-other | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md`, `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs` | overlap / merged-or-other | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md`, `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🟦️.ts` | overlap / merged-or-other | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md`, `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json` | overlap / merged-or-other | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md`, `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | overlap / merged-or-other | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md`, `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts` | overlap / merged-or-other | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md`, `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts` | overlap / merged-or-other | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md`, `framework-inventory-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥾️cross-platform-bootstrap/🟦️.ts` | overlap / merged-or-other | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md`, `framework-inventory-repeat-691.md` |

## HEAD-Only Integration Ledger

| Path | Existing disposition |
| --- | --- |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/⌨️commands/🟦️.ts` | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/👥️space-member/📥️lifted/🏭️operation/🦀️.rs` | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🏭️generate/🟦️.ts` | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🦀️.rs` | `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📃️typed-operation-pages/🟦️.tsx` | `renderer-tests-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🧷️handle-contract/🟦️.ts` | `plugin-runtime-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🟦️.ts` | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧰️test-support/🦀️.rs` | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts` | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🗺️file-analysis/🟦️.ts` | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/📥️lifted/🚪️open/🦀️.rs` | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🌳️tree-providers/🟦️.ts` | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🗣️dialog-origin/🟦️.ts` | `renderer-tests-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧼️clean-mechanism/🟦️.ts` | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/⚙️data-models/🟦️.ts` | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📨️dispatch/📥️staging/⌨️typed-command/🦀️.rs` | `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/📦️packages/🦀️rust/Cargo.toml` | Independently reviewed and disposition recorded above |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🎩️globals/🟦️.ts` | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🔎️code-lens/🟦️.ts` | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🗄️artifact-store/🟦️.ts` | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🚧️authority-bans/🟦️.ts` | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📨️dispatch/📥️staging/📤️emit/🦀️.rs` | `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏃️runtime/📥️lifted/🏪️store/🦀️.rs` | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/👥️space-member/📥️lifted/🏪️store/🦀️.rs` | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🎞️constants/🟦️.ts` | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts` | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs` | `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📨️dispatch/📥️staging/🎟️producers/🦀️.rs` | `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🔒️lifecycle/🟦️.ts` | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏃️runtime/📥️lifted/🔁️replay/🦀️.rs` | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🏬️uri-resolution/🟦️.ts` | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🏠️space/📥️lifted/🏪️store/🦀️.rs` | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/👥️space-member/📥️lifted/🧾️document/🦀️.rs` | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/📦️packages/🦀️rust/Cargo.toml` | `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🏪️stores/📥️staging/🧾️typed-operations/🦀️.rs` | `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🗄️stage/📦️packages/🦀️rust/Cargo.toml` | Independently reviewed and disposition recorded above |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts` | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/⌛️queries/🟦️.ts` | `audit-repo-files-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🔗️channel-adapter/🟦️.ts` | `plugin-runtime-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/📜️wire/📥️lifted/🗄️durable-group/🦀️.rs` | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🧵️turn-dispatch/🟦️.ts` | `plugin-runtime-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🪢️handle-scope/🟦️.ts` | `plugin-runtime-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/♻️retirement/🦀️.rs` | `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/📥️lifted/🏪️store/🦀️.rs` | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🖼️render/📥️staging/🛠️tool-machine/🦀️.rs` | `plugin-result-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧱️core/📥️lifted/🧾️document/🦀️.rs` | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/📥️lifted/🏪️store/🦀️.rs` | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🏭️factory/📥️lifted/🏪️store/🦀️.rs` | Independently reviewed and disposition recorded above |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🎼️utilities/🟦️.ts` | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/📥️invocation-frames/🟦️.ts` | `plugin-runtime-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📤️publication/🟦️.ts` | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🎞️live/📥️lifted/🏪️store/🦀️.rs` | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧱️core/📥️lifted/🏪️store/🦀️.rs` | `store-relocations-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎫️shell-holds/🟦️.ts` | `renderer-tests-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🟦️.ts` | `audit-repo-files-repeat-691.md`, `audit-repo-preservation-repeat-691.md` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🗄️stage/📥️lifted/🗄️durable-group/🦀️.rs` | `store-relocations-repeat-691.md` |
