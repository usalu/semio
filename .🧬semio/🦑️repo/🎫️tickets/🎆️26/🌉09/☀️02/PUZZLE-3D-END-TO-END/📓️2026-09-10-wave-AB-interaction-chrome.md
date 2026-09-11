# 2026-09-10 W-AB — interaction chrome

Assigned ticket: `26/09/02/PUZZLE-3D-END-TO-END` under `🌉09`. Historical #43 readout lives in `🌙️09` of the same slug.

No serve kill. No wasm rebuild. No modifying git. `brush_live_target` fallback left in place. Leftover/gumball edits not touched.

## #44 import apply

Landed leftover `importFixture` on `Puzzle3dWindowCommandWork` and proved the distinct-payload fold with a native law. Same-file reimport identity no-op stayed a store no-op.

### Root cause (b), not (a)

Fold was never empty for a distinct payload. Native empty→payload already emitted operations. The #43 Concrete leftover settle was:

- `utility.publish action=importFixture` with the 9177 B distinct JSON
- `performInvocation settled` `historyCursor=null` `historyUpserts=0` `effects=0`
- census stuck at one live object

`importFixture` is `InteractiveJobClassification::Migrated`. `build_tool_job` fell through to `BoundedFirstStepCommandWork` (one-shot Complete on turn 1). A leftover Complete republishes an empty `CommitCandidate` and does not re-run `completion.complete`, so the handler's leftover mutations never commit.

Other leftover document verbs already use `Puzzle3dWindowCommandWork`: Scene Progress → Sync → Dispatch (`completion.complete` / fold) → Complete.

`map_hit=false` on `utility.publish` is not the dying hop. Both restore and distinct apply logged `map_hit=false` and still folded.

Did not bump shared `PUZZLE_COMMAND_RAW_BYTES` (8192). Leftover no longer re-decodes a truncated 9177 B wire as the apply path; the native payload is the live projection string.

### Fix

`build_tool_job` now matches `"importFixture" => Puzzle3dWindowCommandWork::new(tool_id)` before `addTargetVolume`. Handler parse/assign and `puzzle3d_action_document_intent` already included `importFixture`. Temporary `[DEBUG] puzzle3d.import.apply ops={} after_objects={}` sits after the fold.

### Law

`import_fixture_of_a_distinct_two_object_json_against_a_one_object_live_fixture_emits_operations` in the puzzle-3d editor unit suite (`@semio-tech/puzzle-3d-rs` / `semio-s-artifact-puzzle-3d`, feature `component-app-assembly`).

Setup does **not** use `undo` after the second `addObjectKind` — that dispatch is a store no-op here (live count stayed 2). The one-object live fixture is restored by importing the captured one-object projection, then `importFixture` of the two-object JSON.

Pins the leftover source route:

`"importFixture" => Box::new(Puzzle3dWindowCommandWork::new(tool_id))`

Ran via `bun ./📜️script.ts test import_fixture_of_a_distinct_two_object_json_against_a_one_object_live_fixture_emits_operations -- --nocapture` in `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust`.

**PASS** (2.006s). Debug:

| hop | apply |
| --- | --- |
| restore 2→1 | `ops=1 after_objects=1` |
| distinct 1→2 | `ops=1 after_objects=2` |

After-snapshot length 2 and contains the distinct object id. `history_patch.is_some()`.

### Identity no-op stayed correct

`import_fixture_of_the_live_document_records_whether_identical_content_is_an_edit` **PASS**. Debug: `ops=0 after_objects=1`. `history_patch` none. Ingress still delivers payload+name; the store does not rewrite cores.

`import_fixture_reproduces_the_exported_document` still folds (`ops=21 after_objects=1`) and then fails its **undo** assertion (`left: 1 right: 0`). Same undo-pump family W-G3 already owns. Not an import-apply regression and not unblocked here.

### Browser leftover

Native `dispatch` already folded before this wave. #43 leftover spawn is what dropped the commit. Source now leftover-commits through WindowCommandWork. Needs the coordinator's **#44 wasm** before Concrete leftover can show `historyUpserts>0` and a `probe-distinct-*` census. Not rebuilt in this run.

### Rebuild / restart flags

- No serve kill.
- No wasm rebuild.
- No modifying git.
- `brush_live_target` fallback unchanged.
