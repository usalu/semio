# 📓️ Wave 5A1 — puzzle 🖐️5d WindowConfig / Config-lane verb migration

Slice: migrate the 14 `InteractiveJobClassification::BatchOnlyPendingRewrite` verbs whose publication
lane is the addressed pane's `WindowConfig` or the shared app `Config` to honest `Migrated` retained
tools, port their handler bodies against puzzle 3d, and rename the two weight verbs into the 5d
vocabulary. Repo root `/Users/ueli/Documents/semio`. EDITOR5 =
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.

## 1. What landed

### 1.1 The 14 verbs, their route and their declared lane

| verb | work | lane | note |
|---|---|---|---|
| `setCamera` | `Puzzle5dWindowCommandWork` | `WindowConfig` | scoping bug fixed (§1.3) |
| `setCamera2d` | `Puzzle5dWindowCommandWork` | `WindowConfig` | |
| `setCamera3d` | `Puzzle5dWindowCommandWork` | `WindowConfig` | |
| `setGridFactor` | `Puzzle5dWindowCommandWork` | `WindowConfig` | absolute-or-delta + band clamp (§1.3) |
| `setGridSnapEnabled` | `Puzzle5dWindowCommandWork` | `WindowConfig` | `pressed`-or-toggle (§1.3) |
| `setLodMode` | `Puzzle5dWindowCommandWork` | `WindowConfig` | |
| `setSuggestionOffset` | `Puzzle5dWindowCommandWork` | `WindowConfig` | absolute-or-delta + band clamp (§1.3) |
| `toggleSun` | `Puzzle5dWindowCommandWork` | `WindowConfig` | |
| `setSunAzimuth` | `Puzzle5dWindowCommandWork` | `WindowConfig` | |
| `setSunElevation` | `Puzzle5dWindowCommandWork` | `WindowConfig` | |
| `setSunIntensity` | `Puzzle5dWindowCommandWork` | `WindowConfig` | |
| `setBrushPlacementContactTolerance` | `Puzzle5dWindowCommandWork` | `Config` | absolute-or-delta (§1.3) |
| `setPartKindWeight` (was `setObjectKindWeight`) | `Puzzle5dKindWeightWork` | `Config` | renamed (§1.2) |
| `setGripKindWeight` (was `setVortexKindWeight`) | `Puzzle5dKindWeightWork` | `Config` | renamed (§1.2) |

Lanes were taken from puzzle 3d's own `PUBLICATION_CONTRACTS`
(EDITOR3 `🦀️.rs:7169-7233`), not from `📓️E1` — E1 row 50 records
`setBrushPlacementContactTolerance` as `WindowConfig`, but 3d's source declares it `Config`, and 5d's
`contact_tolerance` sits in `window_ownership::shared()` (EDITOR5 `🪟️window/🦀️.rs`), i.e. the shared app
`Config`. **E1 row 50 is wrong; the source is right.**

### 1.2 Registries touched (EDITOR5 `🦀️.rs`)

Every row below carries all 14 verbs (verified by grep after the last sibling write):

- `puzzle5d_command_variants!` — `SetPartKindWeight = "setPartKindWeight"`,
  `SetGripKindWeight = "setGripKindWeight"` (renamed variants + ids).
- `PUZZLE5D_RETAINED_TOOL_IDS` — 14 ids appended.
- `PUZZLE5D_WINDOW_TOOL_IDS` — the 12 one-turn window/scalar ids appended; the const grew a docstring
  stating that it is the routing set for `Puzzle5dWindowCommandWork` (it now also carries the one
  `Config`-lane scalar, exactly as puzzle 3d's own `Puzzle3dWindowCommandWork` arm does).
- `build_tool_job` — the kind-weight arm is now `"setPartKindWeight" | "setGripKindWeight"`.
- `Puzzle5dKindWeightWork::{section, weights, step}` — the three `tool_id == "setObjectKindWeight"`
  discriminators renamed.
- `PUBLICATION_CONTRACTS` — 14 new rows (11 `WindowConfig`, 3 `Config`).
- `bounded_first_step_tool_proofs!` `tools:` — 14 ids added.
- `.view_action` labels EN+DE — `Set Part Kind Weight` / `Teileart-Gewicht festlegen`,
  `Set Grip Kind Weight` / `Griffart-Gewicht festlegen`.
- `.action_interactive_job(…, InteractiveJobClassification::Migrated)` — 14 rows flipped.
- `dispatch_puzzle5d_action` — the weight arm matches the new ids.

### 1.3 Handler fixes ported from puzzle 3d

- `🎮️commands/🧲️set-grid-snap-enabled/🦀️.rs` — **real bug**: the arm read `enabled` and fell back to
  `false`, so a `WindowMeasure::Toggle` press (which dispatches `pressed`) was an unconditional "off"
  and an argument-less invocation silently turned snap off. Ported 3d's
  `🎮️commands/🧲️set-snap-enabled/🦀️.rs:6` shape: `pressed`, else flip the current state. The two grid
  option files that slice 5E landed while this slice ran
  (`🎭️modes/✏️edit/🪟️windows/{◻️2d,🧊️3d}/☑️options/🌐️grid/🦀️.rs:40-41`) both bind a `Toggle` with
  `puzzle5d_action("setGridSnapEnabled", None)`, i.e. exactly the `pressed` payload this fix now reads.
- `🎮️commands/📐️set-grid-factor/🦀️.rs` — took only an absolute `value` and wrote it unclamped; the
  window schema states `gridFactor` as `exclusiveMinimum: 0`, so `0` was a persistable state the
  partition refuses. Now `puzzle5d_absolute_or_delta` + `clamp(PUZZLE5D_GRID_FACTOR_MIN,
  PUZZLE5D_GRID_FACTOR_MAX)`, mirroring 3d's `↔️set-spacing`.
- `🎮️commands/🚧️set-brush-placement-contact-tolerance/🦀️.rs` — absolute-only → absolute-or-delta
  (3d's `🚧️set-brush-placement-contact-tolerance/🦀️.rs:10`), clamp kept.
- `🎮️commands/🧭️set-suggestion-offset/🦀️.rs` — absolute-only (`distance`|`value`) → absolute-or-delta
  on `value`, clamp kept. The settings panel a sibling slice landed
  (`📌️panels/⚙️settings/🦀️.rs:74`) binds a `NumberStepper` to this verb, which is a `delta` payload;
  before this port every stepper press was a silent no-op. The `distance` alias is gone (greenfield).
- `🎮️commands/🎥️set-camera/🦀️.rs` — **per-window scoping**, the named port. It picked the flat vs the
  volume camera from the payload shape alone (`camera.position.is_none()`), so any world-pane pose that
  omitted `position` wrote `camera2d`, which `Puzzle5dWorldWindowConfig` then dropped silently. It now
  honours an explicit `surfaceId` (`board2d::SURFACE_ID` / `world3d::SURFACE_ID`, the real constants —
  the fixture's old `"puzzle5d-board"` matched neither) and otherwise scopes by `ctx.window_kind`,
  which `handle_action_impl` derives from `view.window_id` through
  `window_ownership::kind_for_view`.
- New shared helper `puzzle5d_absolute_or_delta` (EDITOR5 `🦀️.rs`, next to the existing
  `puzzle5d_resolve_number_edit`), the twin of 3d's `puzzle3d_absolute_or_delta`.

### 1.4 Retained factory contract raised to the 3d band

`ToolExecutionContract::resumable(8_192, 512, …)` → `resumable(262_144, 16_384, 1, 262_144, 7_500, 1, 1)`,
expressed as two named consts `PUZZLE5D_RETAINED_RAW_BYTES` / `PUZZLE5D_RETAINED_DECODED_ITEMS` so the
proof block and the factory cannot drift:

- `Puzzle5dRetainedCommandJobFactory` gained a `contract` field built in `new()` (mirrors
  `Puzzle3dRetainedCommandJobFactory`, EDITOR3 `🦀️.rs:7105-7117`);
- `execution_contract()` returns `self.contract` instead of the shared
  `crate::retained_command::puzzle_command_contract()`;
- `create_job_from_wire_pages_with_payload` admits against `self.contract.max_raw_wire_bytes` instead of
  the shared `PUZZLE_COMMAND_RAW_BYTES`.

Coordination note for 5A2: this is the shared contract line the slice brief said "whoever gets there
first sets to the 3d values" — it is done, and both proof blocks now reference the consts.

The retained-jobs fixture `capacities` block is deliberately **not** raised with it: the shared law
`language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle`
(`✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🧪️tests/🔬️unit/🦀️.rs:148-158`) builds its expectation
from the crate-shared `PUZZLE_COMMAND_*` constants (8 192 / 512), and puzzle 3d carries exactly the same
intentional asymmetry — wide factory contract (`PUZZLE3D_IMPORT_RAW_BYTES = 262_144`), shared-constant
fixture capacities.

**Repair after the outage:** a sibling had propagated the widened band into the 5d fixture
(`capacities.rawBytes` 262 144, `capacities.decodedItems` 16 384, and the `max`/`maxPlusOne` vectors at
262 144/262 145 while their asserted `fingerprint` stayed `8193:…`). That is red against the shared law
for the whole 5d crate and internally self-contradictory, so this slice restored the four numbers to the
shared constants, matching 3d exactly. The underlying limitation is worth a coordinator decision: the
fixture has no way to state a per-artifact admission band today, because `expected()` in the shared law
is artifact-agnostic (§4.6).

### 1.5 Fixtures

- `…/🖐️5d/…/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json`
  - `toolIds` re-synced to the exact, ordered `PUZZLE5D_RETAINED_TOOL_IDS` (65 ids — the union of
    every slice's additions as of this write); the oracle law asserts exact array equality, so this
    file has to be re-synced by whoever writes the registry last (see §4).
  - `evidenceToolIds` widened to the union (no id removed).
  - `semanticCursors` / `hostileSourceMutations` / `vectors` renamed off the object/vortex vocabulary.
  - The five aspirational hostile rows that named symbols 5d never had
    (`Puzzle5dScalarConfigWork`, `Puzzle5dConfigMutation::Set{Camera2d,GridFactor,Sun,ContactTolerance}`)
    now name the real route anchors (`PUZZLE5D_WINDOW_TOOL_IDS` routing arm,
    `window_ownership::{addressed_config,config_from_runtime,shared}`, `apply_world3d_sun_action`).
  - This slice's 12 scalar vectors converted to the `expectedWindowConfigField` /
    `expectedAppConfigField` convention the sibling slices adopted in the same file, and their inputs
    corrected to the payloads the real handlers read (`pressed`, `value`, the real `surfaceId`).
- `✏️s/🔌️plugins/🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json` — the 11 window-config verbs and
  the 3 config verbs moved from their `batch-only-pending-rewrite` groups into the `migrated`
  `window-config` / `config` groups. The now-empty batch-only `config` group was removed (the TS
  oracle refuses a group with no routes). `focusSelection` was left behind in the batch-only
  `window-config` group for slice 5A2, who has since migrated it — as of the end of this slice the
  `Puzzle5dPlayApp` owner has **zero `batch-only-pending-rewrite` groups left**. The 3d owner's
  `setObjectKindWeight`/`setVortexKindWeight` rows were **not** renamed: that is 3d's own vocabulary.

### 1.6 Laws (EDITOR5 `🧪️tests/🔬️unit/🦀️.rs`, region `🎚️WindowAndConfigLaneVerbs`)

Six new laws, all dispatching through the real registry-backed app
(`app_with_registry()` → `dispatch_typed` → `Puzzle5dRetainedCommandJobFactory`) and settling the
typed operation, then reading the published state back typed through
`artifact_app_laws::capture_fixture_window_config::<Owner, _, _>`:

1. `window_and_config_lane_verbs_are_registered_migrated_retained_tools` — every verb is in
   `PUZZLE5D_RETAINED_TOOL_IDS`, carries a `Migrated` row and no `BatchOnlyPendingRewrite` row, and its
   declared lanes are exactly `[WindowConfig]` or exactly `[Config]`; plus a hostile assertion that the
   old object/vortex verb ids appear nowhere in production source.
2. `retained_factory_contract_matches_its_declared_proof_band` — the factory's live
   `execution_contract()` equals the declared proof band and is wider than the shared puzzle default.
3. `camera_verbs_publish_only_the_addressed_window_config` — board pane `setCamera` → `camera2d`,
   world pane `setCamera` → `camera3d`, `setCamera2d`/`setCamera3d` per pane; no document mutation, app
   config pack byte-identical before/after.
4. `grid_verbs_publish_the_clamped_board_window_config` — `pressed:false` persists false, an
   argument-less press flips back to true, absolute/delta factor edits, and both band clamps.
5. `lod_and_suggestion_offset_publish_the_board_window_config` — LOD mode round trip; offset clamped at
   both band ends and nudged by a stepper `delta`.
6. `sun_verbs_publish_the_world_window_config` — `toggleSun` flips the world pane's own environment,
   the three scalars land, the document is untouched.
7. `config_lane_verbs_publish_the_shared_app_config_only` — the contact tolerance and both renamed
   weight verbs each change the app config pack, leave the addressed pane's window-config generation
   at 0 and never touch the document.
8. `kind_weight_group_normalization_keeps_the_group_summing_to_one` — the pure core both weight verbs
   share (5d had no test for `puzzle5d_normalize_kind_weight_group` at all): the changed kind keeps
   exactly the requested value, the group sums to 1, equal siblings stay equal, a skewed group
   redistributes the remainder in proportion, and a one-kind group is always the whole distribution.
   Value semantics live here so the retained-route law above only has to prove the lane, which keeps
   it independent of how the config envelope happens to serialize.

Two pre-existing laws were updated for the rename and the widened routing set:
`kind_weight_route_is_cursorized` (new verb ids) and `engagement_submit_route_is_cursorized` (its
hard-coded one-line `PUZZLE5D_WINDOW_TOOL_IDS` literal became an iteration over the const itself, so it
no longer breaks every time a sibling slice adds a window verb).

## 2. Commands run — verdicts

All outputs are under `TICKET/🗑️generated/5A1/`. Every cargo invocation carried `CARGO_INCREMENTAL=0`.

| # | command | verdict | evidence |
|---|---|---|---|
| 1 | `cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly` | **GREEN, exit 0** | `check-native.txt:1787` `Checking semio-s-artifact-puzzle-5d` (a real from-scratch compile, not a replay), `:1813` `generated 2 warnings`, `:1814` `Finished dev profile … in 15m 06s`, `EXIT=0`. `grep -c '^error'` = **0**. |
| 2 | `cargo check … --target wasm32-wasip2` | **GREEN, exit 0** | `check-wasm32-wasip2.txt`, 0 errors, same 2 warnings, `Finished … in 3.50s`, `EXIT=0`. Cargo reported every unit fresh, so the warnings are replayed — under `checksum-freshness` that still means the CURRENT 5d source is what produced those artifacts, but this verdict is a cache hit, not a compile I drove. |
| 3 | `cargo test … --lib -- verbs lod_and_suggestion retained_factory_contract kind_weight` | **RED — not mine, owed to integration** | `test-5A1.txt`, `EXIT=101`, "could not compile (lib test) due to 14 previous errors". **Zero of those errors are in a file this slice owns** — see §3. My laws therefore compiled but the crate's test binary could not link, so none of the 8 laws has been *executed* yet. |
| 4 | `cargo check … --message-format=short` (the single gated re-check the 19:55 addendum asks for) | **INCONCLUSIVE — never got the lock** | `check-final.txt` stops after 24 lines of framework-dependency warnings, still `Blocking waiting for file lock on artifact directory` behind 9 concurrent peer invocations. Per the 19:55 addendum I did not wait it out. Its purpose was only to re-confirm command 1 against the sibling edits that landed after it; command 1 is the authoritative green, and everything this slice owns was re-verified unchanged by grep afterwards (all 14 verbs × 4 registries, all 8 laws, all 6 handlers). |
| 5 | `bun ./📜️script.ts publication-authority-audit` | **RED — not mine** | blocked in `validateWindowOwnershipSchemas` before it ever reaches an owner oracle: first `Puzzle5dBoardWindowConfig … missing gridVisible/selectableKinds` (5E), then `Puzzle2dWindowConfig … missing areaBrushWidth/areaBrushHeight` (2F), then `Puzzle5dWorldWindowConfig … missing voxelDims` (5G). Each time a slice adds a window-config field it must also extend the hard-coded `cases` list in `📦️packages/🟦️typescript/📜️script.ts`. |
| 6 | `bun TICKET/🗑️generated/5A1/owner-oracle-5d.ts` (stand-in for the half of the audit the schema cases block) | **ALL PASS (10/10)** | re-run after both outages: manifest rows == fixture routes; classifications match; `PUZZLE5D_RETAINED_TOOL_IDS` == migrated routes; publication contracts == migrated routes; proof tools == migrated routes; declared lanes == fixture lanes; all 14 of this slice's verbs Migrated + retained + contracted + proved; the old weight vocabulary is gone; the retained-jobs fixture `toolIds` equals the Rust registry in order. |
| 7 | `rustfmt --edition 2024 --emit stdout` on all 8 edited Rust files | parses clean | syntax gate used between the outages while the artifact-directory lock was contended. |

Warning counts: the 5d crate emits **2** warnings on both targets, both pre-existing
`unused_qualifications` (`✏️editor/🦀️.rs:9412` and `✏️editor/🪟️window/🦀️.rs:276`) and neither in code this
slice touched.

Build-environment note: the fleet kept 6–18 concurrent `cargo` invocations on this one crate all
afternoon (load average peaked at 129). Command 1 waited ~40 min on the artifact-directory lock before
it started. The gate in the 15:15 addendum never opened for me (it read 6–9 throughout), so per the
19:55 addendum I stopped waiting and took exactly one invocation per milestone.

## 3. Not verified by this slice / owed to integration

**The 8 laws have never been executed.** The 5d `lib test` target does not compile, and every error is in
another slice's file. From `test-5A1.txt` (14 errors, paths relative to
`🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/`):

| file | errors | owner |
|---|---|---|
| `✏️editor/🧪️tests/🔬️target-volumes/🦀️.rs` | 10 (`E0433` unresolved `world3d`/`board2d`, `E0425` `puzzle5d_labels`, `E0599` `handle_action` on `Puzzle5dTestApp`) | 5G — the file imports `crate::editor::puzzle5d::*` but the window modules are not re-exported there, and it calls the framework-reserved `handle_action` the test harness deliberately replaced with `dispatch` |
| `✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs:111` | 1 (`E0063` missing `voxel_dims`) | 5G — they added the field to `Puzzle5dWorldWindowConfig` without updating their own round-trip fixture |
| `✏️editor/🧠️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs:477` | 2 (`E0599` `is_number` on `dsl::JsonValue`) | 5B |
| `🧬️schema/💡️inferences/**/🧪️tests/🔬️unit/🦀️.rs` | 2 (`E0063` missing `target_volumes`) | 5G |

I deliberately did not fix any of them (19:55 addendum §2, and rule 17). **Integration owes:** once the
crate's test target links, run
`CARGO_INCREMENTAL=0 cargo test -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib -- verbs lod_and_suggestion retained_factory_contract kind_weight`
— that filter selects exactly this slice's 8 laws plus the two it amended.

- No Nx / activate / serve / Playwright run — the coordinator owns those. The verbs are proven live at
  the **registry + retained-factory + publication** level, not against a running browser.
- `setLodMode` accepts any string; it is not validated against the declared LOD tier list
  (`semio_framework_os_infinite::puzzle_2d_lod_scale_json()`). Puzzle 3d has no single `setLodMode` to
  port a rule from (it has the `setLodAutomatic`/`setLodDepthVariable`/`setLodManual` trio), and slice
  5E owns the LOD trio for 5d — left as declared product behaviour, flagged here.
- The plugin descriptor `✏️s/🔌️plugins/🧩️puzzle/🔣️.json` still carries `setObjectKindWeight` /
  `setVortexKindWeight` for puzzle5d. It is a generated manifest; Wave 2's descriptor regen must
  regenerate it. Hand-edit deliberately avoided.

## 4. Hand-offs

1. **5E (window options) — `setSuggestionOffset` is dropped in the world pane.** The brush Utility
   Options group is declared at MODE level (`🎭️modes/✏️edit/☑️options/🖌️brush/🦀️.rs`) and is collected
   by BOTH panes' `window_measures()` (`🪟️windows/🧊️3d/🦀️.rs:67-69`), but `suggestion_offset` lives only
   on `Puzzle5dBoardWindowConfig`. `window_ownership::addressed_config` therefore builds a
   `Puzzle5dWorldWindowConfigMutation::Snapshot` that does not carry the field, so dragging the offset
   slider in the 3D pane publishes a no-op. The verb itself is migrated and correct; the fix is one
   field on `Puzzle5dWorldWindowConfig` + its four `🪟️window/🧬️schema` twins + the
   `Puzzle5dWorldWindowConfig` case in
   `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts` (`validateWindowOwnershipSchemas`,
   whose expected key list is hard-coded). Not done here because 5E was mid-refactor on exactly that
   struct while this slice ran (it grew 11 fields during the slice) and a concurrent edit would have
   been a lost update. The same applies to `lod_mode`, which is board-only, if the LOD select is ever
   surfaced in the world pane.
2. **5A2** — the retained factory contract line is already at the 3d band (§1.4); do not re-edit it.
3. **Whoever writes the 5d registries last** — re-sync
   `…/🧫️fixtures/🗄️retained-jobs/🔣️.json:"toolIds"` to the exact ordered `PUZZLE5D_RETAINED_TOOL_IDS`.
   The oracle law compares the two arrays with `assert_eq!`, so any registry append without a fixture
   append is a red test for everyone. Verified in sync at 70 ids at the end of this slice (check 6 in
   §2 asserts exactly this, so re-running that script is a one-second way to confirm it).
4. **Coordinator** — `📓️E1-3d-feature-inventory.md` row 50 records
   `setBrushPlacementContactTolerance` as `WindowConfig`; 3d's source declares `Config`.
5. **Whoever raised the 5d fixture `capacities`** — it is back at the shared `PUZZLE_COMMAND_*`
   constants (§1.4). The widened band is real but it lives on the FACTORY
   (`PUZZLE5D_RETAINED_RAW_BYTES`), not in the fixture, exactly as in 3d. Raising the fixture block
   turns `language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle` red for the
   whole 5d crate.
6. **Coordinator — the retained-jobs fixture cannot express a per-artifact admission band.**
   `expected()` (`🎮️commands/🧵️retained/🧪️tests/🔬️unit/🦀️.rs:148-158`) compares `capacities` against
   the shared constants for all three artifacts, while 3d and now 5d both run a wider factory contract.
   Either the law should read the artifact's own factory contract, or the fixture should carry both
   numbers. Out of this slice's remit (shared test file, all three artifacts).
7. **5E (projection) — `setCamera`/`setCamera3d` parse the WHOLE `Puzzle5dCamera3d`.** After 5E added
   `up` and `projection` to that struct (`🎚️config/🦀️.rs:85-104`), a host orbit payload carrying only
   `{position,target,zoom}` re-parses the camera with `#[value(default)]` for the two new fields, i.e.
   it resets the pane's projection. This slice deliberately did **not** diverge from puzzle 3d here:
   3d's `📷️set-camera/🦀️.rs:6-11` does the identical whole-struct parse into a `Puzzle3dCamera` that
   carries the same two fields, so either both are fine (the host always sends the full camera) or both
   need the same merge — that is a decision for whoever owns `setProjection`/`setProjectionParam`, and
   inventing a 5d-only second mechanism would violate the port-from-3d rule.

8. **Integration (both crate integrators)** — the 5d `lib test` target is red on four other slices'
   files (§3, exact list with error codes). Nothing in this slice is blocking it; the production `lib`
   compiles clean on both targets.

## 5. State at hand-off

- **Production source: green.** `cargo check` native (a real from-scratch compile of the 5d crate) and
  `--target wasm32-wasip2` both exit 0 with 0 errors and the same 2 pre-existing warnings. The one
  re-check the 19:55 addendum asks for never got the artifact-directory lock (§2 row 4); integration
  should treat the first native compile of the 5d `lib` it manages to run as the confirmation.
- **Publication authority: consistent.** All ten owner-oracle invariants pass; `Puzzle5dPlayApp` now has
  zero `batch-only-pending-rewrite` groups, and the 14 verbs this slice owns are `Migrated` with exact
  lanes in every registry.
- **Laws: written, compiled, not yet executed.** The crate's test binary cannot link because of four
  sibling files. This is the one thing this slice could not verify by running, and it is stated as such
  rather than claimed.
- **Nothing left half-done inside this slice.** Every verb, registry row, handler port, schema-facing
  fixture row, label and law named in the brief is landed; the two items I chose not to do
  (`suggestion_offset` on the world pane, the whole-camera projection reset) are deliberate,
  argued hand-offs to the slice that owns those structs, not omissions.
