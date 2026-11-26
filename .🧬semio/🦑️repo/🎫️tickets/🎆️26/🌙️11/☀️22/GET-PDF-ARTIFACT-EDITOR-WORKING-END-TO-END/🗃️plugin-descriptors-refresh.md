# 🗃️ Plugin descriptor refresh (2026-11-26, in progress)

Follows `🗂️pdf-descriptor-action-sets.md`: the framework change (reserved window actions on the app roster, plugin-level `actionSets`) makes every plugin's committed descriptor pair stale. PDF was settled already and is not touched here.

## Route

- Official generator: `repo run workspace:components -- describe <id>|--all` (wasm build per plugin, refused here: 30+ min and memory heavy).
- Official check: every component's `plugin_exports!` generates the native test `descriptor_is_fresh` (`🔌️plugin/🧪️tests/🧬️generated-test-contracts/🦀️.rs`).
- Native describe route (new): with `SEMIO_DESCRIBE_EMIT_DIR=<dir>` that same test writes the assembled pair to `<dir>/<plugin-id>/` through `describe::emit_semantic_descriptor_pair` (`🔌️plugin/🛂️describe/🦀️.rs`), the bytes the component emitter writes. `settle-plugin-descriptors.ts` then writes each pair into its owner through the emitter's own `settleComponentDescriptorV1`.
- Ticket scripts: `check-plugin-components.sh` (phase 1, `cargo check --tests --lib --keep-going`, 25 GB disk guard), `refresh-plugin-descriptors.sh` (phase 2, emit test), `settle-plugin-descriptors.ts`, `check-census.sh`, `native-receive-closure.py`, `plugin-descriptors-components.txt` (75 component crates).

## Root cause: the freshness check was vacuous

`descriptor_is_fresh` compared canonical bytes only inside `if let (Some(a), Some(b)) = (canonical(assembled), canonical(committed))`. A committed pair that no longer decodes as the current descriptor shape (exactly what a stale pair is) gave `None`, so the comparison was skipped and the test passed. Observed: `note` passed with a pair 81,269 / 284,313 bytes against the described 75,457 / 248,871 (pack / JSON).

Fix, test first:
- Law `a_committed_descriptor_is_fresh_only_when_it_equals_the_assembled_one` (`🛂️describe/🧪️tests/🔬️unit/🦀️.rs`): identical pair fresh; one changed label byte stale; undecodable committed pair a fault; undecodable assembled pair a fault.
- `describe::descriptor_freshness_fault(assembled, committed)`: `None` only when both decode and re-encode to the same bytes; both `descriptor_is_fresh` macros panic on any fault.
- Status: written, not yet compiled or run (the plugin test build is blocked, see animate below).

## Framework fixes made on the way (root causes)

| Area | Break | Fix |
| --- | --- | --- |
| `PluginApp` dispatch macro | `dyn_enum_close!` copies trait signatures into callers; `take_typed_operation_result_page` named `semio_framework_value::...` by crate path, so every plugin without that dependency failed | bare `RetainedAllocationEffect` in the trait, re-exported by the reactor app crate and `plugin_app_close_prelude` |
| architect tests-2 carve | dead `pub use semio_s_artifact_architect_program_architect::editor::architect::*` (would be a dependency cycle) | line removed; crate checks clean |
| bim model diff | `MutationDiff::apply/absorb` and `DiffAlgebra::inverse` moved to `ColdMutationDiff` / `ColdDiffAlgebra` | impls migrated like trinity jack |
| animate presentation io | closure `\|[r, g, b, a]\|` needs a type | `: [f32; 4]` |
| architect registers | `AdjacencyKind` lacked `NativeReceive` | derive added |
| cad add-object | `protocol::ActorId` now wraps `SharedUtf8` | `seed.as_str().into()` |

## Status per plugin

| Plugin | Phase 1 (`cargo check --tests`) | Descriptor |
| --- | --- | --- |
| stdio-pdf | n/a | settled earlier (1,281,668 JSON / 299,702 pack bytes) |
| note | clean | emitted (75,457 pack / 248,871 JSON), not yet settled |
| animate | clean (ck3 exit 0) | emit blocked: `semio-plugin-testing-laws` fails on `semio_os_kernel::os_vcs::io::binary::entity_identity::testing` (feature `testkit` gate edited 00:59 by another session; retry later) |
| architect | fails | see census |
| bim | fails | see census |
| block | fails | see census |
| cad | `E0308` fixed, recheck pending | pending |
| other 66 | phase 1 `ck4` queued | pending |

## Error census (first error class per plugin, owning file, last modified)

| Plugin | First error class | Owning file | Last modified |
| --- | --- | --- | --- |
| architect | `E0117` orphan rule: diff impls of foreign traits for foreign types in the inferences carve (`🔺️diff/🦀️.rs` lines 164-199) | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/…/🧬️schema/🔺️diff/🦀️.rs` | 11-19 16:17 |
| bim | `kit` carve crate lacks `bim_window_config!`, five `Bim*WindowConfigOwner` types and a `vocabulary` crate | `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/…/✏️editor/🧰️kit/🦀️.rs` | 11-26 00:34 (another session; retry later) |
| block | `Block2dSnapshot` / `Block3dSnapshot` / `Block5dSnapshot: RetireOwned` unsatisfied; unresolved `crate::snapshot` in the sqlite mount | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/*/…/🧬️schema/📸️snapshot/🦀️.rs` and `🚪️io/🪶️sqlite` | 11-19 |
| cad | `E0308` `ActorId` string type (fixed) | `✏️s/🔌️plugins/📐️cad/…/🧵️retained-command-tools/➕️add-object/🦀️.rs` | 11-26 (this session) |
| all with dev-deps | `semio-plugin-testing-laws`: `entity_identity::testing` missing without feature `testkit` | `🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🦀️.rs` | 11-26 00:59 (another session) |
| remaining 66 | pending `ck4` | | |

The census generalizes one theme: the framework moved (diff traits, native receive, retirement, shared string types, carve mounts) and many artifacts were never brought along, so most plugins do not compile natively at HEAD, and a wasm `describe` would hit the same errors.

## Disk

The first sweep (full test builds) filled the volume; the disk guard in the scripts now prunes `lane-*/b/debug/build/*/*` units older than 60 minutes whenever fewer than 25 GB are free.
