# Wave E report — puzzle3d fixture groups 4/5/6 migration

Scope: migrate the mechanical actions in fixture groups 4/5/6 from `BatchOnlyPendingRewrite`
to `Migrated` for `Puzzle3dPlayApp`. Started as 53 candidates; the coordinator caught three
real problems mid-flight that changed the final count to **51 migrated / 2 held back**.

## Final state

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`:
**57 Migrated / 8 BatchOnlyPendingRewrite** (was 6/59 at wave start). All four regex-derived
source sets (`PUZZLE3D_RETAINED_TOOL_IDS`, `PUBLICATION_CONTRACTS` keys, `bounded_first_step_tool_proofs!`
`tools:` list, and the `Migrated` subset of `.action_interactive_job` pairs) are set-equal at 57 —
verified by re-running the exact regexes the TS oracle uses (see Verification).

## What changed, file:line (current source)

### `✏️editor/🦀️.rs`
- `PUZZLE3D_RETAINED_TOOL_IDS` (line 2530): added 51 ids (grouped by original fixture group,
  comments omitted, see diff for the literal list).
- `Puzzle3dRetainedCommandJobFactory::PUBLICATION_CONTRACTS` (line 6205): added 51
  `ArtifactToolPublicationContract` entries. Two lane corrections vs. the original per-group
  blanket lanes (see "Mid-flight corrections" below): `addTargetVolume` and `relocateTargetVolume`
  both declared `&[Artifact]` only (not `Config`) — their `Work`/reducer implementations
  (`Puzzle3dRelocateVolumeWork::step`, line ~5539, and the inline `addTargetVolume` arm in
  `puzzle3d_retained_reduce`, line ~2565) never emit a `config_mutations` entry.
- `bounded_first_step_tool_proofs!`'s `tools: [...]` (line 6698 block): added the same 51 ids,
  sharing the existing `resumable(8_192, 512, 1, 262_144, 7_500, 1, 1)` contract.
- `.action_interactive_job(...)` manifest block (inside `create_puzzle3d_app`, line 7120):
  flipped 51 ids from `BatchOnlyPendingRewrite` → `Migrated` (lines 7313–7377 span). Left
  `BatchOnlyPendingRewrite` on the 6 group-7 ids (agent S's scope: `engagementRepeatLast`,
  `fillBuildTick`, `registerBrushMesh`, `suggestionsTick`, `transformBegin`, `transformEnd`) and
  on the two held-back ids below.

### `🧪️publication-authority/🔣️.json`
- Merged the 16 remaining group-5 ids (`["Artifact","Config"]`) into the existing `setActiveExample`
  Migrated group.
- Merged the 34 remaining group-6 ids (`["Config"]`) into the existing `setFillCount`/`setLocale`/
  `setTerminology` Migrated group.
- Created a new Migrated `["Artifact"]` group for `addTargetVolume` + `relocateTargetVolume`
  (previously two separate BatchOnly groups with different, now-stale, lanes).
- Removed fixture groups 4/5/6 (all empty after the merges above).
- Added two new BatchOnly groups for the ids held back (see below), each with a fresh,
  accurate `blocker` string — the old "no app-owned retained preparation factory" text is
  false today (both factories exist, `build_config_store_one_item_preparation_factory` :6626 /
  `build_artifact_store_one_item_preparation_factory` :6630) and was not reused.
- Reordered the BatchOnly groups so the group with a **literal-string** first route
  (`engagementRepeatLast`) sorts before the `setFillCountStep` group, whose id is referenced only
  symbolically in source (`set_fill_count::STEP_ACTION_ID`). The audit script's hostile-activation
  test regex-matches `.action_interactive_job("<route>", ...)` literally on
  `owner.groups.find(status===BatchOnly)?.routes[0]` — with `setFillCountStep` first this test
  silently no-ops (regex never matches, `hostile === source`) instead of exercising anything. Caught
  this by actually running the hostile-mutation battery, not by reasoning about it (see Verification).

### `📦️packages/🟦️typescript/📜️script.ts`
- `PublicationAuthorityAuditScript.run` now takes `segments: string[]` and accepts an optional
  owner-name argument: `bun ./📜️script.ts publication-authority-audit [OwnerName]`.
  - No argument: identical behavior to before — audits every owner in the fixture, unweakened.
  - An owner name: audits only that owner (`ownerOracle` + its full hostile-mutation battery),
    using the exact same functions as the full run — no logic duplicated or forked.
  - An unrecognized owner name: throws immediately (`unknown owner "X"; expected one of
    Puzzle2dPlayApp, Puzzle3dPlayApp, Puzzle5dPlayApp`) rather than silently auditing nothing.
  - The closing `hostileFixtures`/Ajv-schema checks (owner-agnostic fixture-shape invariants)
    still run unconditionally regardless of the filter.
  - Added `owners=...` to the success log line so a scoped run is visibly distinguishable from a
    full run in output.

**Not registered in `📋️project.json` / `.vscode/launch.json` / `.claude/launch.json`** — see
"Noted but not acted on" below; those files are outside this wave's write-lock and two of them
are already dirty from concurrent sessions.

## Mid-flight corrections (three, all from the coordinator, all applied)

1. **`setFillCountStep` held back.** `dispatch_puzzle3d_action`'s match (line ~2449) has no arm
   for `"setFillCountStep"` — it falls through to `_ => {}` and does nothing. Confirmed by reading
   the match directly (every other one of the 51 migrated ids has an explicit arm there or a
   dedicated inline handler in `puzzle3d_retained_reduce`). Reverted from
   `PUZZLE3D_RETAINED_TOOL_IDS`, `PUBLICATION_CONTRACTS`, the proofs `tools:` list, and its
   `.action_interactive_job` (uses the symbolic `set_fill_count::STEP_ACTION_ID`, not a literal
   string — kept that call-site form). Fixture: separate new BatchOnly `["Artifact","Config"]`
   group, blocker: *"no explicit build_tool_job arm; the generic reducer does not dispatch this
   id, so a migrated route would emit nothing"*.
2. **`relocateTargetVolume` lane fixed.** Originally inherited the blanket `["Config"]` lane from
   fixture group 6. Read `Puzzle3dRelocateVolumeWork::step` (line ~5549) in full: every stage only
   ever pushes into `self.mutations: Vec<Puzzle3dMutation>` (artifact mutations); `complete()`
   (line ~5524) returns `Emit { artifact_mutations: ..., ui_scope, ..Default::default() }` with no
   `config_mutations` field set. Declaring `Config` in its publication contract would have been a
   lie the oracle's `exactContracts` check happened to not catch (it verifies *presence* of the
   declared lanes' machinery, not that a Work never uses a declared lane). Moved to `["Artifact"]`
   only, in both `PUBLICATION_CONTRACTS` and the fixture (now grouped with `addTargetVolume`).
3. **`setFixtureJson` held back.** Its argument is a whole fixture — `set_fixture_json` command
   takes the entire document as its arg payload. The Nakagin example DSL on disk is 128,755 bytes;
   its JSON form is larger. Every id sharing `bounded_first_step_tool_proofs!`'s
   `resumable(8_192, ...)` contract is capped at `max_raw_wire_bytes = 8_192` for the *incoming*
   wire payload, enforced at `🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs:643` before any
   decode/dispatch happens — any realistic `setFixtureJson` call would hit `RawWireLimit`
   unconditionally. `PUZZLE_COMMAND_RAW_BYTES = 8_192` is a fixed `raw: [u8; N]` buffer inside
   `RetainedPuzzleCommandJob`, shared by puzzle 2d/3d/5d — raising it was explicitly out of scope
   for this wave. Reverted the same four source locations as (1), plus its manifest classification.
   Fixture: new BatchOnly `["Artifact","Config"]` group, blocker: *"argument is a whole fixture
   (~129KB for the Nakagin example) against the shared 8,192-byte PUZZLE_COMMAND_RAW_BYTES
   retained-job wire cap; needs a chunked or resumable wire path, not a wiring change"*.
   Checked the two payload shapes the coordinator flagged as similarly risky:
   - `acceptSuggestion` (`🎮️commands/✅️accept-suggestion/🦀️.rs`): args are `index: u64` and an
     optional `fullId: string` (a vortex id) — tens of bytes, not a document. Safe as migrated.
   - `patchInspector` (`🎮️commands/🩹️patch-inspector/🦀️.rs`): args are `entity`, `field`,
     `ids: Vec<String>` (a selection), `value`, `delta` — bounded by selection size, not document
     size. Same shape/risk class as every other selection-based action already in this wave
     (`deleteSelection`, `translateSelection`, `rotateSelection`, `scaleSelection`,
     `duplicateSelection`, `setSelectionFlag`) — see "Noted but not acted on".

## Verification actually run

**Ran, and passing (real command output):**

```
$ cd ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript && bun ./📜️script.ts publication-authority-audit Puzzle3dPlayApp
validated Puzzle publication authority; owners=Puzzle3dPlayApp; admitted=openAddObjectDialog,worldPointerDown,closeVortexSuggestions,cycleBrushCandidate,cycleBrushCandidateBack,engagementAbort,engagementControlSelect,engagementInput,engagementSubmit,focusSelection,hoverSuggestion,openVortexSuggestions,selectSameKindSelection,setBrushPlacementOverlapBudget,setCamera,setChunkSize,setFillCount,setGridSnapEnabled,setGridSpacing,setGridVisible,setLocale,setLodAutomatic,setLodDepthVariable,setLodManual,setObjectKindWeight,setProjection,setProjectionParam,setProximityRadius,setSelectableKind,setSunAzimuth,setSunElevation,setSunIntensity,setTerminology,setTransformGumballFlag,setVortexDirection,setVortexKindWeight,setVortexShow,setVoxelDims,toggleSun,acceptSuggestion,addBrushObject,addObjectKind,createAttraction,deleteAttraction,deleteSelection,deleteTargetVolume,duplicateSelection,patchInspector,rotateSelection,scaleSelection,setActiveExample,setSelectionFlag,setTargetVolumeFlag,translateSelection,worldRelocate,addTargetVolume,relocateTargetVolume; schema=Ajv; oracle=independent
exit=0
```

This is the real `ownerOracle` (from the actual `script.ts`, not a copy) run against the real
`🔣️.json` and the real `✏️editor/🦀️.rs`, including its full hostile-mutation battery for
Puzzle3d (hostile-activation of the first BatchOnly route, missing publication contract, missing
Config/Artifact store preparation, widened terminology allowlist, stale generation-authority guard,
missing `Progress` checkpoint step) — every hostile mutation was confirmed rejected (oracle flips
to `false`) before this passing result was accepted as real.

```
$ bun ./📜️script.ts publication-authority-audit
error: Puzzle2dPlayApp publication authority diverged from the fixture
    at run (.../📜️script.ts:169:50)
```

Confirms the default (no-argument) gate still audits every owner and is **not** weakened — it
still fails, and still fails on `Puzzle2dPlayApp` specifically, which is another session's
in-flight puzzle2d migration (its Rust source already has ~35 ids flipped to `Migrated` with a
live `bounded_first_step_tool_proofs!` block; its fixture JSON still records only 3 routes, all
`BatchOnlyPendingRewrite` — confirmed via `git status` showing dozens of modified files under
`🗿️artifacts/◻️2d/`). Not this wave's code; never reached Puzzle3d's branch.

```
$ bun ./📜️script.ts publication-authority-audit NotARealOwner
error: unknown owner "NotARealOwner"; expected one of Puzzle2dPlayApp, Puzzle3dPlayApp, Puzzle5dPlayApp
```

Confirms the "unknown owner name fails loudly" requirement.

**What did NOT run: any Rust compiler.** `cargo check -p semio-s-plugin-puzzle` (foreground,
`RUSTC_WRAPPER="" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-p3d-agentE`) was started and
left running ~6 minutes before the coordinator flagged that `semio-s-plugin-puzzle` is gated
behind `semio-s-plugin-stdio`, which a live peer session is mid-migrating (`✳️base` → `🧱️base`,
directories-first-then-references, ~69 unresolved mounts a few minutes prior per a peer's static
gate) and therefore cannot presently reach a real pass/fail for puzzle at all. **Killed it on that
instruction** (PID 49588 + its live rustc children) rather than let it keep burning CPU. Its
captured output at kill time (152 lines, in
`/private/tmp/claude-501/.../scratchpad`, not reproduced in full here) never got past building
base dependencies (`tokio`, `futures-util`, `semio-framework-math`, etc.) and had not yet reached
either `semio-s-plugin-stdio` or `semio-s-plugin-puzzle`'s own code — the only diagnostics it had
emitted were two pre-existing, unrelated lint warnings (`unnecessary_qualification` in
`puzzle/📦️packages/🦀️rust/build.rs:28` and in `semio-framework-value-derive`'s derive macro, plus
one `dead_code` warning in `semio-framework-os-kernel-dsl-derive`). **Zero real errors were
captured, and none of the 51 migrations were reached, let alone verified, by rustc.** This is
consistent with the coordinator's framing: the check cannot presently pass or fail meaningfully
for puzzle at all, so its absence here is not a regression signal.

**None of the 51 migrations in this wave are compiler-verified.** The publication-authority
oracle (schema + `fixtureOracle` + `ownerOracle` + full hostile-mutation battery, run for real,
scoped to Puzzle3d) is the only verification that ran to completion. It proves: the four
regex-derived source sets agree with each other and with the fixture; every migrated route's
`.action_interactive_job` classification matches its fixture status; every migrated route has a
`PUBLICATION_CONTRACTS` entry whose lanes exactly match its fixture group's lanes; the two
Config/Artifact store preparation factories exist and are wired; and the oracle rejects each of
the hostile mutations above. It does **not** prove: that any of the 51 `Work`/reducer
implementations actually compiles, that their `step()` state machines terminate correctly, that
their emitted mutation shapes are accepted by the real store preparation `advance()` logic at
runtime, or that their argument decoding matches what the UI actually sends.

## Command sequence for a later session once stdio goes green

```sh
cd /Users/ueli/Documents/semio
RUSTC_WRAPPER="" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-p3d-agentE cargo check -p semio-s-plugin-puzzle
RUSTC_WRAPPER="" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-p3d-agentE cargo check -p semio-s-plugin-puzzle --tests
RUSTC_WRAPPER="" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-p3d-agentE cargo test -p semio-s-plugin-puzzle --lib -- puzzle3d
cd ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript
bun ./📜️script.ts publication-authority-audit Puzzle3dPlayApp   # re-run in isolation first
bun ./📜️script.ts publication-authority-audit                   # then the full gate, once Puzzle2d's fixture syncs
```
If `cargo check` surfaces errors, first check whether the failing file is under
`✏️s/🔌️plugins/🗄️stdio/` or `🧱️base`/`✳️base` — that is still someone else's migration, not this
wave's. Errors inside `✏️editor/🦀️.rs` outside the write-lock regions listed at the top of this
report are not this wave's either (another agent owns the `Work` `step` bodies at ~5935-6135 and
the engagement/noop `Work` impls).

## Noted but not acted on

- **Selection-payload wire-size risk is systemic, not just `setFixtureJson`'s.** `deleteSelection`,
  `translateSelection`, `rotateSelection`, `scaleSelection`, `duplicateSelection`,
  `setSelectionFlag`, and `patchInspector` all take an `ids: Vec<String>` (or rely on the live
  selection) against the same shared 8,192-byte `max_raw_wire_bytes` cap. A selection of a few
  hundred object ids (UUIDs are ~36 bytes each once JSON-quoted-and-commaed, so ~230 ids already
  exceeds 8,192 bytes) would hit the identical `RawWireLimit` rejection `setFixtureJson` was pulled
  for — just less obviously, since it depends on how many objects a user has selected rather than
  on a fixed payload. I did not attempt to bound real-world selection sizes against this (would
  need the actual id string lengths and a realistic max-selection-count from the UI, neither of
  which I traced). This deserves a dedicated check before calling this wave's `Work` set fully load-
  bearing at runtime.
- **`Puzzle3dPatchInspectorWork`'s and other bespoke `Work`s' `extent()`/budget numbers were not
  individually re-derived against real fixture sizes.** The recipe doc flags this as a
  per-bespoke-Work spot-check item; I verified only that each id has *a* dedicated `Work` or the
  generic fallback wired into `build_tool_job`'s match (confirmed by reading the match arms
  directly, line ~6698 region), not that each one's `extent()` bound is tuned correctly for large
  documents (e.g. Nakagin/Concrete Forest fixtures).
- **The 10 generic-fallback ids** (`addTargetVolume`, `deleteAttraction`, `deleteSelection`,
  `deleteTargetVolume`, `duplicateSelection`, `openVortexSuggestions`, `selectSameKindSelection`,
  `setSelectionFlag`, `setTargetVolumeFlag` — `setFixtureJson` was the 10th and is now held back)
  route through `BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce,
  puzzle3d_retained_extent)`. I confirmed `dispatch_puzzle3d_action` (line ~2449) has a real,
  non-empty arm for every one of these — but did not trace `puzzle3d_retained_extent`'s bound
  logic for each, which the recipe doc explicitly calls out as unconfirmed ("the shared reducer/
  extent functions were evidently not written with these ids in mind yet").
- **`📋️project.json` / `.vscode/launch.json` / `.claude/launch.json` do not register
  `publication-authority-audit` at all** (checked: `grep -rl "publication-authority-audit"
  --include="📋️project.json" .` finds nothing anywhere in the repo, including puzzle's own
  `📋️project.json`, which has only a `test` target). These three files are outside this wave's
  write-lock, and `.vscode/launch.json` / `.claude/launch.json` are already modified by concurrent
  sessions per the session-start `git status`, so I did not touch them. The repo convention for
  this kind of subcommand (seen in
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📋️project.json`, e.g. its `test-fixture-audit`
  target) is:
  ```json
  "publication-authority-audit": {
    "executor": "nx:run-commands",
    "options": {
      "cwd": "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript",
      "command": "bun ./📜️script.ts publication-authority-audit",
      "forwardAllArgs": true
    }
  }
  ```
  added next to puzzle-js's existing `test` target in its `📋️project.json`. `forwardAllArgs: true`
  is required for the new owner-name argument to reach the script through `nx run`.
- **Puzzle2d's fixture desync is out of scope** but blocks the unscoped `publication-authority-audit`
  from ever reaching Puzzle3d until it's fixed by whoever owns that migration — flagged here so it
  isn't rediscovered from scratch.
