# Lowpoly Retained Command Honesty

## Scope

This packet owns the Lowpoly editor's 47 registered command routes, the ARC-CONTEXT-2 immutable request root, the distinct operation-owned mutable command workspace, and the shared synchronous mesh kernel required by those routes. It excludes Trinity, Writer, Raster, Procedural3d, and unrelated plugin behavior.

## Source State

- The official static gate accepts all 47 Lowpoly routes with 0 remaining rows and 0 scan-then-monolith rows.
- ARC1 is version 3 with a 48-byte header, context digest at bytes 32..40, workspace identity at bytes 40..48, and work state beginning at byte 48.
- Build and restore receive the identical request-owned `Arc<ArtifactOwnedToolJobContext<_>>`; checkpoint restore fails closed on context or workspace identity drift.
- Lowpoly's immutable transient root shares large mesh state by Arc across paint-begin, transform-begin, and utility-reset transitions. Its explicit serde wire shape preserves the prior language-neutral JSON schema.
- The operation workspace admits exactly 1,024 4 KiB work items and raw commands are capped at 1 MiB. Mesh roots are capped at 1 MiB each, paint layers at the fixed 1 MiB texture extent, objects/layers/selections have explicit fixed maxima, and all shape drift fails closed. The max/max+1 test constructs real approximately 4 MiB paint/mesh request-owned fixtures rather than testing numeric predicates alone.
- All 47 commands have a queued every-turn `<8 ms` law and bounded close. Runtime closure is not claimed until native and Wasm execute it.
- `engagementSubmit` is a mesh disposition and selects exact typed mesh leaves directly; it no longer reconstructs a command and invokes the generated dispatcher.
- Shared `semio-framework-3d` mesh production APIs are synchronous. A guarded span codemod removed 92 decorative async keywords and 627 stale awaits; async test harnesses remain async. The component has a compile-time synchronous signature witness.

## Static Evidence

- `lowpoly-workspace-tool-jobs-self-test-after-arc-root.txt`: `self-tests=468 clean`.
- `📊️lowpoly-workspace-tool-jobs-live-capped-2026-08-26.json`: current full census after exact caps; Lowpoly is 47 accepted, 0 remaining, 0 scan-then-monolith, with 468 self-tests. The process exits nonzero only for recorded unrelated global cohorts.
- `lowpoly-workspace-tool-jobs-self-test-source-final.txt`: current `self-tests=468 clean`; the suite includes the hostile final one-shot dispatch and ARC1 workspace-identity bypass mutations.
- `mesh-deasync-span-journal.json`: exact source offsets, before/after bytes, and hashes for the shared mesh conversion.
- `mesh-deasync-cross-plugin-cargo-impact.txt`: 13 manifests with direct `semio-framework-3d` dependency impact.
- `lowpoly-nx-project.json`: generated Nx target evidence for `@semio-tech/lowpoly-plugin:test-quick`.

## Runtime Boundary

Worker-thread loss resumes exactly while the scheduler retains the immutable request context. The work checkpoint contains cursor, digest, tool/disposition, extent, and workspace identity only. Replay reconstructs one bounded segment per turn and validates the recorded digest before new progress. A process restart without the exact request-owned transient root fails closed; large mesh and paint bytes are deliberately not embedded into the <=16 KiB checkpoint.

## Queued Serialized Commands

Compiler ownership is external. Run only after the coordinator grants the lease, using the ticket-local targets below.

```sh
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-5-DE-ASYC-REPAIR-SWEEP/target-mesh-native" cargo test -p semio-framework-3d --lib public_kernel_api_is_synchronous -- --nocapture
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-5-DE-ASYC-REPAIR-SWEEP/target-lowpoly-native" cargo test -p semio-s-plugin-lowpoly --lib retained_ -- --nocapture
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-5-DE-ASYC-REPAIR-SWEEP/target-lowpoly-native" cargo test -p semio-s-plugin-lowpoly --lib
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-5-DE-ASYC-REPAIR-SWEEP/target-lowpoly-wasm" cargo check -p semio-s-plugin-lowpoly --lib --target wasm32-wasip2
bun nx run @semio-tech/lowpoly-plugin:test-quick
```

## Open Runtime Proof

The compiler lease has not yet been granted. Mesh/UV kernel finalization and paint fill/diff finalization remain unproven against the actual native and Wasm `<8 ms` law. If the queued timing law reports a command over budget, that leaf must be split into an operation-owned microcursor; raising the budget, restoring async, or adding a compatibility bridge is not acceptable.

## R3 Pure-Root Source Repair

The r3 compiler checkpoint exposed decorative futures in shared immediate APIs and missing suspension at retained runtime boundaries. The guarded source pass made the following exact changes without running Cargo or Nx:

| Symbol / callsite class | Before | After | Exact callsite delta |
|---|---|---|---:|
| `vcs::apply_mutation` | `pub async fn` | `pub fn` | 10 stale awaits removed from shared store folds/tests |
| `ArtifactView::new` | `pub async fn` | `pub fn` | 3 stale awaits removed in the shared plugin component |
| `HistoryView::empty` | `pub async fn` | `pub fn` | 4 stale awaits removed in the shared plugin component |
| `VcsArtifactApp::snapshot` | `pub async fn` | `pub fn` | 9 stale awaits removed in shared tests |
| `VcsArtifactApp::backbone_ref` | `pub async fn` | `pub fn` | 3 stale awaits removed in shared tests |
| `AppActionRegistry::from_definition` | `pub async fn` | `pub fn` | 10 shared/plugin-builder await or `resolve_ready` sites removed |
| Lowpoly `testkit::dispatch` | `pub fn` incorrectly returning through a future | `pub async fn` | 12 `dispatch_typed` sites await the retained runtime |
| Lowpoly `testkit::render` | `pub fn` incorrectly returning through a future | `pub async fn` | 1 `PluginApp::render` and 5 helper render sites await the retained runtime |
| Lowpoly `testkit::select_face` | `pub fn` incorrectly returning through a future | `pub async fn` | 6 `handle_action` sites and 4 helper select sites await the retained runtime |
| Other Lowpoly helper/testkit calls | unawaited futures | explicit suspension | 20 helper dispatch and 2 generic async testkit sites await completion |

The exact source-span evidence is `shared-pure-roots-span-journal.json` (4 files, 45 edits) and `lowpoly-runtime-awaits-span-journal.json` (18 files, 53 edits: 3 helper signatures plus 50 await insertions). Each file row records its pre-edit hash, post-edit hash, byte offsets, exact before bytes, and exact after bytes. `rustfmt --edition 2021` and `rustfmt --edition 2021 --check` were run only over the 22-file union in those journals; the check completed with exit code 0.

Static census after formatting:

- 261 `#[semio_framework_async_macros::async_test]` entrypoints remain intact.
- 264 Lowpoly `async fn` definitions remain: the 261 harness entrypoints plus the three genuine testkit runtime helpers.
- 50 Lowpoly `.await` sites remain, all introduced at retained dispatch/render/testkit boundaries.
- 0 Lowpoly `dispatch_typed` or `handle_action` callsites remain unawaited.
- 0 shared callsites remain awaiting the five newly synchronous immediate roots. The three textual `backbone_ref().await` matches left in the shared store belong to a different async host/backbone type and were intentionally preserved.

### Exact Constructor Blocker

`plugin::testkit::{new_app,new_app_with_registry}` remain async. Their generic construction chain reaches `VcsArtifactApp::with_registry_on_bus`, whose fresh-store genesis branch calls `ArtifactStore::dispatch`. That dispatch first pumps a potentially attached backbone and then flushes outbound traffic, so it is not an immediate pure API. Making the testkit constructors synchronous in isolation would require a blocking compatibility bridge, which is forbidden. A truthful repair needs a constructor/genesis redesign that seeds validated genesis into the fresh envelope/store without entering the live dispatch I/O path, or an explicitly retained constructor job. `VcsArtifactApp::dispatch_typed` and `PluginApp::handle_action` likewise remain async because they drive retained tool work, cache refresh, recording, and completion.
