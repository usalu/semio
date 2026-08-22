# P8yv Layout Bounded Disposal

## Verdict

**RED pending app-owned snapshot retirement.** The Layout-owned renderer/export buffers now have explicit cursor-bounded disposal, and both concrete reserved owners implement the required `ArtifactReservedJob::close_step`. The shared framework now splits the job lease from an `ActiveMediaExport` runtime retention, so Layout can release its request share without becoming the last owner while that Active record remains live. This is not yet sufficient for `PluginCloseStep::Complete`: normal terminal/cancel removal still needs to transfer the retention into app-owned retirement authority, command-cache refresh can replace the store's exact old snapshot root, and eventual old-root destruction has no bounded cursor/quarantine. Layout therefore retains its job lease and returns structural `Blocked` at `SnapshotOwner`.

## Layout Source Changes

- `LayoutExportJob` implements `ArtifactReservedJob` through an explicit close-stage cursor.
- `LayoutMediaExportJob` implements `ArtifactReservedJob`, delegates all inner cleanup, verifies the segmented queue is empty, discards at most one completion value through the framework close seam, and releases its completion clone before returning complete.
- The production Layout media builder forwards the framework-provided snapshot-close lease into the concrete export job.
- The direct `ArtifactReservedToolJob::new(LayoutExportJob { ... })` test owner now satisfies the required trait. Its unwitnessed snapshot intentionally fails closed rather than asserting constant-time final destruction.

The close cursor covers:

1. dynamic JSON validation stack;
2. typed validation and package JSON stacks, including owned node strings/byte/path buffers;
3. planned rectangles;
4. raw and encoded chunk ropes, one at-most-4,096-byte chunk per step;
5. base64 tail and PNG row logical byte slices;
6. PDF offsets;
7. ZIP entries and current entry name;
8. request page, preflight, parent-document, and canonical-revision strings;
9. shared segmented output chunks through `ArtifactOutputChunks::close_take_chunk`;
10. media output credit;
11. request snapshot authority;
12. media completion through `ArtifactMediaExportCompletion::close_take`.

Empty-container deallocation happens only after explicit cursor exhaustion. Zero item or insufficient byte credit returns exact `Pending { released_items: 0, released_bytes: 0 }` without advancing the owner/cursor. No blanket/default disposal implementation, worker handoff, whole-container clear, or terminal flatten was added.

## Focused Source Fixtures

- A retained-state fixture seeds both JSON cursors, rectangle state, two raw chunks, two encoded chunks, a base64 tail, an 8 KiB PNG row, PDF offsets, an 8 KiB ZIP name, current ZIP name, an 8 KiB preflight value, and a shared output chunk. It asserts every successful close slice reports exactly one item and at most 4,096 bytes, drains every owner, and then rejects the unwitnessed unique snapshot.
- A zero-budget fixture asserts zero item credit and zero byte credit preserve the exact output close stage and retained chunk.
- A snapshot-retirement fixture asserts the final Layout cursor remains structurally blocked and never claims complete before an app-owner transfer exists.
- Existing cancellation/generation fixtures remain unchanged; the interactive step still checks cancellation before and after each unit and rejects stale operation/generation authority.

The Rust fixtures were not executed because Cargo/native/Wasm commands were prohibited.

## Gates

| Gate | Result |
| --- | --- |
| Layout two-file `rustfmt --edition 2021 --check` | PASS |
| Scoped `git diff --check` | PASS |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS: 55 clean |
| `bun ./📜️script.ts verify interactivity` | PASS: DENY clean; one recorded test-only blocking bridge |
| `bun ./📜️script.ts verify interactivity tool-jobs` | Expected fail-closed exit 1: 0 admitted, 884 remaining, ten failure classes |

No applicable Layout Bun/Nx project target exists for these Rust-only fixtures. No Cargo, Rust compiler, native test, Wasm/component generation, browser, or runtime command was run.

## Exact Residuals

1. Transfer `ActiveMediaExport`'s runtime retention into app-owned snapshot retirement authority before every normal terminal/cancel Active removal.
2. Keep cache-refresh-replaced old snapshot roots and the VCS/store's eventual last `LayoutSnapshot` owner in bounded cleanup or quarantine. Layout cannot solve those app-store owners from the concrete export job.
3. Add deterministic shared-framework race, cache-refresh, and last-owner negative fixtures for the split lease/runtime-retention protocol and eventual retirement.
4. Prove in the shared completion fixture that `close_take` only drops bounded metadata plus the already-drained identical chunk authority. Layout now enforces queue-empty-before-completion and rejects a present completion value unless this job recorded successful completion.
5. Run Rust compile/tests and native/Wasm/runtime acceptance when the no-Cargo constraint is lifted.

## Files

- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️component.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- this report

The repository MCP goal/ticket tools were unavailable in this agent context. Work therefore remained inside the already-active `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR/EVERY-TOOL-INTERACTIVE-JOB-MIGRATION` ticket and no ticket metadata/lifecycle operation was performed.
