# World3dScene ToValue/FromValue — already landed, verified on re-check

Dispatched to add hand-written `ToValue`/`FromValue` to `World3dScene` (the last `serde_json` call
site in the puzzle-3d editor, per `📓️puzzle3d-editor-serde-json-elimination-2026-09-03.md` line
7422). On inspection the work was **already done** by a prior session and is already committed —
this is a duplicate dispatch, not new work.

## Evidence

- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🦀️scenes.rs` lines 305-362: hand-written
  `impl ToValue for World3dScene` / `impl FromValue for World3dScene`, built directly against
  `protocol::value::` via the file's own `value_push`/`value_decode*` helpers (`🔖️ValueCodecHelpers`
  region), camelCase keys matching `#[serde(rename_all = "camelCase")]` exactly (`cameraJson`,
  `meshesJson`, `selectionJson`, `domainGranularityId`, etc.), `Serialize`/`Deserialize` derives kept
  untouched (additive only).
- Nested type `World3dSnapshotLease` (`🦀️world3d_snapshot.rs` lines 20-61) also already has both
  directions, all-integer fields (`u8`/`u64`/`u16`/`u32`) routed through each field's own
  `to_value()`/`from_value()` rather than a hand-built `DslValue::Number`, so integer-vs-float
  fidelity is correct by construction. A round-trip test exists at line 517-522
  (`world3d_snapshot_lease_round_trips`).
- A dedicated integer-fidelity round-trip test for `World3dScene` itself exists at
  `🦀️scenes.rs:1866`, `world3d_scene_round_trips_dense_and_bare_and_keeps_integers_as_integers`,
  inside a `value_round_trip_tests` module (line 1852).
- `Cargo.toml` for `semio-framework-ui-scene` depends on `protocol` (semio-framework-replication)
  only, no `semio-framework-os-kernel` — confirmed by reading the file directly; no diff needed.
- `git status`/`git diff HEAD` on all three files (`🦀️scenes.rs`, `🦀️world3d_snapshot.rs`,
  `🦀️canvas2d_snapshot.rs`) is clean — this landed in the repo's latest commit `96aa4f8c12`
  (2026-09-02 17:38:16 +0200, message references "Extend additive ToValue/FromValue bridges across
  framework and OS runtime modules while preserving serde wire shapes").
- Full prior writeup already exists at
  `📓️ui-scene-value-derive-2026-09-02.md` — documents all 26 types / 45 impls in this crate
  (including `World3dScene`, `World3dSnapshotLease`, `Canvas2dSnapshotLease`, and 23 others), with
  cargo check/test results recorded there (`semio-framework-ui-scene`: 0 errors, 108/108 tests
  passing) from when that agent did the work.

## What I did this pass

Read-only verification: re-read the impls on disk, traced `World3dScene`'s and
`World3dSnapshotLease`'s fields by hand against their `to_value`/`from_value` bodies, confirmed
camelCase keys, confirmed no os-kernel dependency, confirmed no recursive bridge-through-trait
pattern (helpers build `DslValue` directly), confirmed clean `git diff`. Made no code changes — none
were needed. **Ran zero cargo commands.**

## Nested types checked

- `World3dSnapshotLease` — has both directions (pre-existing per this pass, confirmed).
- No other nested type: every other `World3dScene` field is `String`, `Option<String>`, or
  `Option<crate::World3dSnapshotLease>` — no further nested custom types to cover.

## Not verified

I did not re-run `cargo check`/`cargo test` (instructed not to run cargo). The 0-errors/108-passing
result is as reported in `📓️ui-scene-value-derive-2026-09-02.md` by the agent that did the work;
I did not independently re-execute it.
