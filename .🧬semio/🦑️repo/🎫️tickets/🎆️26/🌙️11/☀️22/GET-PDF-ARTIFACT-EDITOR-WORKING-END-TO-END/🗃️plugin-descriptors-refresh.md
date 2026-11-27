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

## Progress log (2026-11-26 03:00)

### Results

| Check | Result |
| --- | --- |
| Freshness law `a_committed_descriptor_is_fresh_only_when_it_equals_the_assembled_one` (`semio-plugin-extension` lib tests, fixture `🧫️descriptor-freshness`) | passes (4 cases: identical fresh, changed label stale, undecodable committed fault, undecodable assembled fault) |
| animate strict `descriptor_is_fresh` after settling | passes (`vf1`) |
| dag, note strict check | blocked by `semio-os-inference` compile break (another session editing it); settled pairs written, confirmation pending (`after-inference-quiet.sh` retries after 15 min quiet) |
| cad, demonstrator | check clean; emit blocked by the same inference break; retry pending |

### Settled pairs (JSON / pack bytes, before -> after)

| Plugin | JSON | Pack |
| --- | --- | --- |
| animate | 200,698 -> 196,029 | 60,751 -> 60,751 |
| dag | 278,967 -> 216,580 | 77,617 -> 66,872 |
| note | 248,871 -> 248,871 (already current) | 75,457 -> 75,457 |

### Phase 1 census additions

| Plugin | First error class | Owning file | Last modified | Action |
| --- | --- | --- | --- | --- |
| draw | `semio_framework_replication` unresolved in graph-manifest | `🕸️graph/🛂️manifest/📡️delta/🦀️.rs` | 11-26 02:02 | graph-manifest `Cargo.toml` dependency renamed so both spellings resolve; recheck pending |
| energy, fem | `semio-os-inference`: `driver` and `values` disagree (`publish_output_step` arity, borrow) | `💡️inference/🏁️driver`, `🧺️values` | 11-26 02:23 | another session editing; retry after quiet |
| flow | missing generated `🤖️generated/📇️registry/🦀️.rs` (gitignored build product) | `🌎️hub/🧩️compositions/🌊️flow/🦀️.rs` | n/a | official `repo run @semio-tech/plugin-registry:generate` fails: `nx:authorizers` cannot find `./🧬️schema/🔣️.json` in the process testing authorizer compile (repo infrastructure, being edited); retry later |
| forms | `ReplaceConfig` / `SetContributions: DslOwnedReceivingField` | `📋️forms/…/🎚️config/🧬️schema/🧬️mutations/📸️replace`, `🧩️set` | 11-19 | `DslOwnedReceiving` derive added to both; recheck (`ck6`) pending |
| demonstrator, dag, cad | clean | | | settled / pending as above |

Note on blanket migration: of about 5,100 `MutationLeaf` derives in the plugins nearly all lack `DslReceiving`/`DslOwnedReceiving`, yet most plugins compile, so the derive is added only where a crate errors on it.

## Progress log (2026-11-26 08:45)

### Settled descriptor pairs (native emit, committed through `settleComponentDescriptorV1`)

| Plugin | JSON bytes (before -> after) | Pack bytes (before -> after) | Strict `descriptor_is_fresh` |
| --- | --- | --- | --- |
| stdio-pdf | 4,496,151 -> 1,281,668 | 968,129 -> 299,702 | green (earlier) |
| animate | 200,698 -> 196,029 | 60,751 | green |
| note | 248,871 (already current) | 75,457 | green |
| dag | 278,967 -> 216,580 | 77,617 -> 66,872 | green |
| cad | 330,301 -> 233,279 | 90,115 -> 72,727 | pending (link race fixed, rerun queued) |
| gis | 1,831,749 -> 1,760,499 | 756,769 -> 729,968 | pending |
| procedural | 751,605 (already current) | 205,545 | fresh at emit (exit 0) |
| reasoning | 210,907 | 67,201 | pending |
| space | 566,224 | 155,877 | pending |

### Framework and artifact fixes made on the way (root causes)

| Area | Break | Fix |
| --- | --- | --- |
| `semio-plugin-instance-render` | `CommandHistory::invalidate_cache` called, never defined | defined (drops the cache) |
| `semio-plugin-time-travel-turns` | `TimeTravelReplayStep::Unfunded` not matched | arm added: `Unfunded => return Ok(())` (the variant's documented meaning) |
| `semio-plugin-runtime-ops-document` | `PLUGIN_RUNTIME_INSTANCE_SLOTS` not imported | added to the existing import |
| `semio-framework-graph-manifest` | dependency spelled `semio-framework-replication`, source spells `semio_framework_replication` | Cargo dependency renamed with `package =` |
| forms config | mutation leaves lacked `DslOwnedReceiving` / `RetainedClone`; `FormsConfig` lacked `DslOwnedReceiving` | derives added (closure tool `native-receive-closure.py`) |
| playbook examples carve | `crate::editor::playbook::terminology` not mounted | mounted from `✏️editor/🗣️terminology` |
| writer sqlite snapshot | `ArtifactSqliteSnapshotHost` native encode/decode gained `output` and `SnapshotReceivingOutcome` | migrated; post-decode `admit_values` kept through `decode_sqlite_snapshot_record_receiving` |

### Open items

| Plugin | Blocker | Owner / status |
| --- | --- | --- |
| writer | `✏️editor/⌨️typing/🔗️fold`: retained-work protocol changed (`admit_original_construction` now takes an `OriginalConstructionAdmission` plus a work quote; close steps return `RetainedWorkOutcome`; `step` takes an effect). Needs a state field and work-quote design, not a mechanical port; sequence's import analogue is stale against the same API | deferred until the framework migration lands |
| stdio, stdio-semio, draw, demonstrator, energy | `DEBUG_PHASE` static in pack document cursor | pack agent fixing |
| block, lowpoly, others with a sqlite native bridge | same `output` signature change as writer | migrate with the writer pattern, keeping post-decode admission |
| flow | emit run killed (memory warden); retry scheduled alone | `c13` |
| remaining phase 1 crates | run `ck4` still going | |


## Census snapshot (2026-11-26 09:45, runs paused at the coordinator's request)

Built from the most recent log of each component crate across all check, emit and verify runs. Caveat: the runs I stopped at 09:29 left partial logs, so `no error recorded` and some `clean (check)` rows mean the run was cut short, not that the crate is green; the authoritative green set is the table of strict checks above.

| Crate | Latest state | Run | Time |
| --- | --- | --- | --- |
| hub-animate | strict check passed | vf1 | 11-26 02:06 |
| hub-architect | fails: semio-s-artifact-architect-program-inferences | ck3 | 11-26 00:27 |
| hub-bim | fails: semio-s-artifact-bim-model-kit | ck2 | 11-26 00:24 |
| hub-block | fails: semio-s-artifact-block-2d-model, semio-s-artifact-block-3d-model, semio-s-artifact-block-5 | ck2 | 11-26 00:40 |
| hub-cad | strict check passed | vf-c7 | 11-26 08:42 |
| hub-dag | strict check passed | vf-r3 | 11-26 08:58 |
| hub-demonstrator | fails: semio-hub-demonstrator-component | em-r3 | 11-26 08:37 |
| hub-draw | fails: semio-framework-pack-record-encode-cursor | em-r3 | 11-26 08:05 |
| hub-energy | clean (check) | ck17 | 11-26 09:29 |
| hub-fem | fails: semio-plugin-time-travel-turns | ck-r2 | 11-26 06:36 |
| hub-flow | no error recorded | em-c13 | 11-26 09:29 |
| hub-forms | clean (check) | ck10 | 11-26 09:29 |
| hub-gis | fails: semio-hub-gis-component | vf-c7 | 11-26 09:07 |
| hub-imperative | fails: semio-plugin-instance-render | ck-r2 | 11-26 05:44 |
| hub-layout | fails: semio-plugin-instance-render | ck-r2 | 11-26 05:46 |
| hub-lowpoly | fails: semio-plugin-instance-render | ck-r2 | 11-26 05:47 |
| hub-mathematical | fails: semio-plugin-instance-render | ck-r2 | 11-26 05:47 |
| hub-norm | fails: semio-plugin-time-travel-turns | ck11 | 11-26 06:33 |
| hub-norm-energy | fails: semio-hub-norm-energy-component | ck-r2 | 11-26 05:50 |
| hub-norm-eurocode | fails: semio-hub-norm-eurocode-component | ck4 | 11-26 04:31 |
| hub-norm-eurocode-materials | fails: semio-plugin-time-travel-turns | ck11 | 11-26 06:37 |
| hub-norm-eurocode-structural | fails: semio-plugin-guest-contract | ck4 | 11-26 05:08 |
| hub-note | no error recorded | vf-r3 | 11-26 09:42 |
| hub-playbook | no error recorded | em-c18 | 11-26 09:29 |
| hub-procedural | strict check passed | vf-c7 | 11-26 07:35 |
| hub-process | fails: semio-plugin-instance-render | ck4 | 11-26 05:38 |
| hub-puzzle | no error recorded | em-c17 | 11-26 09:28 |
| hub-raster | fails: semio-plugin-instance-render | ck4 | 11-26 05:44 |
| hub-reasoning | emitted, committed stale at that run | vf-c7 | 11-26 07:55 |
| hub-remodel | fails: semio-plugin-instance-render | ck4 | 11-26 05:47 |
| hub-sequence | fails: semio-plugin-instance-render | ck4 | 11-26 05:47 |
| hub-shooting | fails: semio-plugin-instance-render | ck4 | 11-26 05:48 |
| hub-sourcing | fails: semio-plugin-instance-render | ck4 | 11-26 05:48 |
| hub-space | no error recorded | vf-c9 | 11-26 09:39 |
| hub-stdio-bim | fails: semio-plugin-time-travel-turns | ck4 | 11-26 06:32 |
| hub-stdio-binary | fails: semio-plugin-time-travel-turns | ck4 | 11-26 06:36 |
| hub-stdio-cad | fails: semio-plugin-time-travel-turns | ck4 | 11-26 06:38 |
| hub-stdio | fails: semio-framework-pack-record-encode-cursor | em-c9 | 11-26 08:05 |
| hub-stdio-image | fails: semio-os-store-engine-derived | ck4 | 11-26 06:52 |
| hub-stdio-media | emitted, committed stale at that run | em-c9 | 11-26 08:34 |
| hub-stdio-mesh | emitted, committed stale at that run | em-c9 | 11-26 08:52 |
| hub-stdio-office | emitted, committed stale at that run | em-c9 | 11-26 09:39 |
| hub-stdio | no error recorded | em-c14 | 11-26 09:29 |
| hub-trinity | clean (check) | ck10 | 11-26 09:23 |
| hub-vcs | fails: semio-s-artifact-vcs-vcs-editor | ck4 | 11-26 08:29 |
| hub-wfc | no error recorded | em-c16 | 11-26 09:29 |
| hub-writer | clean (check) | ck4 | 11-26 09:30 |

Never run in phase 1 or phase 2 (the plugin extension crates and the host test component): semio-hub-stdio-pdf-component, semio-plugin-host-test-component, semio-s-plugin-cad-aec-building-component, semio-s-plugin-cad-aec-energy-component, semio-s-plugin-cad-aec-structure-component, semio-s-plugin-cad-spatial-shape-component, semio-s-plugin-flow-bim-component, semio-s-plugin-flow-brep-component, semio-s-plugin-flow-dictionary-component, semio-s-plugin-flow-draw-component, semio-s-plugin-flow-list-component, semio-s-plugin-flow-logic-component, semio-s-plugin-flow-math-component, semio-s-plugin-flow-primitive-component, semio-s-plugin-flow-text-component, semio-s-plugin-imperative-control-component, semio-s-plugin-imperative-effect-component, semio-s-plugin-imperative-logic-component, semio-s-plugin-imperative-math-component, semio-s-plugin-imperative-text-component, semio-s-plugin-playbook-procedural-component, semio-s-plugin-process-concrete-component, semio-s-plugin-process-metal-component, semio-s-plugin-process-robotic-component, semio-s-plugin-process-wood-component, semio-s-plugin-sourcing-beams-component, semio-s-plugin-sourcing-slabs-component, semio-s-plugin-sourcing-windows-component.

Paused: every queued descriptor run was stopped at the coordinator's request (PDF-critical runs share the memory gate). In flight at the pause: the `r3` strict-check chain. Held and to be resubmitted afterwards: `ck17` (full recheck of the 28 earlier failures), `c9` stdio family emits, `c13`/`c14` (`flow`, `stdio-semio`), `c15`..`c18` (`puzzle`, `wfc`, `playbook`), then the 22 extension crates.

## State at the disk pause (2026-11-26 18:05, 17 GB free)

The descriptor sweep is paused at the coordinator's request after the in-flight job (`pid 50389`) exits; no new job is submitted until told. The coordinator removes the `semio-hub-*-component` build units of verified plugins to reclaim space; they rebuild on demand.

### Confirmed green by the strict `descriptor_is_fresh` (committed pair equals the assembly)

`animate`, `note`, `dag`, `cad`, `procedural`, `stdio-media`, `stdio-mesh`, `stdio-office` (the last three were already current), plus `stdio-pdf` (settled earlier).

### Settled but not yet confirmed by a later strict run

| Plugin | Committed JSON / pack bytes |
| --- | --- |
| gis | 1,760,499 / 729,968 |
| reasoning | 210,907 / 67,201 (re-emit blocked by the test-support rollout below) |
| space | 566,224 / 155,877 (same) |

### Blocked, not mine (files edited within the last 15 minutes by other sessions)

| Plugin | Blocker | Owner |
| --- | --- | --- |
| space, reasoning, trinity and the other plugins with text-mutation codecs | 47 `🚪️io/📝️text/🧬️mutations/🦀️.rs` files gained `semio_os_store_test_support::mutation_report_json` calls (uncommitted, 17:28); the crates do not depend on `semio-os-store-test-support` | case-adapter rollout in progress |
| stdio, draw, stdio-semio, demonstrator, energy | pack document-cursor `DEBUG_PHASE` static; `stdio-pdf-presence-lane` compile error | pack and PDF agents |
| flow, stdio-semio | emit killed by the memory warden, retry alone | scheduled `rs2` (held) |
| writer | typing fold: retained-work protocol migration (needs an `OriginalConstructionAdmission` state field and work quotes) | deferred |
| puzzle | `wasm-bindgen` gated to wasm32 in 55 manifests; rebuild pending | held `rs1` |

### Held, to resubmit one at a time after the pause

`rs1` remainder (`energy`, `draw`, `demonstrator`, `playbook`, `puzzle`, `wfc`), `rs2` (`flow`, `stdio-semio`), then `ck18`: the phase 1 recheck of the 28 earlier failures and the 22 extension crates plus the host test component, which were never run.

### Root-cause fixes landed and verified

- `descriptor_is_fresh` can no longer pass vacuously (law passes, 4 cases).
- `dyn_enum_close!`: the proc macro rewrites `crate::` to `$crate::` in captured signatures; `semio-plugin-tests-05` compiles with it.
- `package_descriptor_lists_its_io_mechanism_rows_with_their_native_side` and its sibling no longer depend on process-global IO registries; `semio-plugin-tests-06-2` is 27/27.
