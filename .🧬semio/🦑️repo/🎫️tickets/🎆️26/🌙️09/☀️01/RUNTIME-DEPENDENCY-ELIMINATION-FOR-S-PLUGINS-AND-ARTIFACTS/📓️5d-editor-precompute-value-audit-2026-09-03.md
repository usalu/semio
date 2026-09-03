# 🖐️5d `editor/🦀️.rs` + `editor/🧠️precompute/**` — serde_json/DslValue audit and fixes

## Scope given
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (~28 errors estimated)
- `.../✳️any/✏️editor/🧠️precompute/**` (~5 errors estimated) — actually lives at
  `.../✳️any/✏️editor/🧠️precompute/🦀️.rs` (the brief's path omitted the `✏️editor/` segment).

Ran **zero** cargo commands. The error-capture file at
`…/scratchpad/puzzle-5d-errors.txt` was present but empty (0 bytes, no `DONE`) the whole session, so
worked from source, cross-referenced against this ticket's own prior research notes (which turned out
to be extensive and load-bearing — see below).

## Critical finding: the top-level brief's translation table target was wrong for this file
The brief said convert to `dsl::DslValue` via `dsl::FromValue::from_value`. The actual architecture
(confirmed by reading `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs` directly, and cross-checked against
this ticket's own `📓️puzzle-5d-2d-commands-serde-to-value-2026-09-03.md` and
`📓️puzzle-91-to-0-2026-09-03.md`) has **three** distinct value types in play here, not two:
1. `serde_json::Value` — still genuinely live: `Puzzle5dPlaySnapshot(pub Value)`
   (`🧬️schema/🧬️mutations/🦀️.rs`, out of my scope, still unconverted, documented as intentional
   remaining work) IS a real `serde_json::Value`. Every read of `snapshot.0` / `doc.snapshot.0`, the
   `Puzzle5dCommand::args: Option<Value>` plane, and this file's own local "structural twin" mirror
   types (`Puzzle5dDocument`, `Puzzle5dPart`, `Puzzle5dFastener`, `Puzzle5dGrip{,2d,3d}` — defined
   *in this file*, distinct from the canonical `crate::artifacts::puzzle5d::*` types of the same
   name) are correctly still `serde_json::Value`-typed. **Left all of this alone** — it's correct,
   not a leftover.
2. `dsl::os_pack::json::Value` (aka `dsl::json::Value`) — the in-house serde_json drop-in
   (`Value::get`/`as_str`/`as_array`/etc., plus a `json!` macro at `pack::json!`). This is what every
   `editor/🎮️commands/*` handler function (already converted by a concurrent session, confirmed by
   reading several of them: `set-camera`, `patch-part`, `add-brush-part`, …) actually takes as its
   `args: Option<&Value>` parameter — **not** raw `DslValue`.
3. `dsl::DslValue` (`protocol::value::DslValue`) — the typed-struct round-trip target for
   `ToValue`/`FromValue` derives on the *canonical* `crate::artifacts::puzzle5d::*` types.

`dispatch_puzzle5d_action` (already fixed, pre-existing on disk, not touched by me) bridges (1)→(2)
once at the single call boundary into the command handlers:
`raw_args.map(|value| dsl::json::from_dsl_value(&dsl::DslValue::from(value)))`. This — plus
`command_from_action`'s `args.map(Value::from)` using the framework's pre-existing
`impl From<&DslValue> for serde_json::Value` (`🧰️framework/🔨️modules/🌱️value/🦀️.rs:218`) — are the
file's two sanctioned, unavoidable boundary crossings, not bridges I added.

## What was already done (by prior sessions in this same ticket, uncommitted, on disk before I started)
Extensive: local twin struct `value_derive` derives already stripped (kept plain `Serialize`/
`Deserialize`), ~15 `crate::artifacts::puzzle5d::*` typed conversions already using
`<T as dsl::FromValue>::from_value(dsl::DslValue::from(&value))`, `command_from_action` and
`dispatch_puzzle5d_action` already bridging correctly, `parse_example_dsl` already using
`dsl::json::to_json_string`. Verified all of this by reading the actual current file content, not by
trusting the notes — the notes describe a highly volatile multi-session churn (94→110→156→134 error
swings on the whole crate from a concurrent peer session rewriting the same struct region), so I
treated "what's on disk right now" as ground truth over any note's narrative.

## Fixes made this session

### `✏️editor/🧠️precompute/🦀️.rs` (3 sites, all genuine — confirmed `BrushPlacePayload`/`Fixture`
only derive `value_derive::ToValue/FromValue` in production, serde is `#[cfg_attr(test, ...)]`-gated
only, so `serde_json::to_string`/`from_str` on them could never have compiled outside tests):
- `fixture_outcome_json`: `Ok(serde_json::to_string(&fixture)?)` → `Ok(dsl::json::to_json_string(&fixture))`
  (infallible — dropped the `?`).
- Both `apply_brush_placement_rust` (native cfg block and wasm-bindgen cfg block — duplicated body):
  `serde_json::from_str(payload_json).map_err(Puzzle3dError::from)?` →
  `dsl::json::from_json_str(payload_json).map_err(Puzzle3dError::from)?`. `Puzzle3dError` already
  has `impl From<dsl::ValueError> for Puzzle3dError` (`🗿️artifacts/🧊️3d/🦀️.rs:38`, read-only
  reference, not edited), so the `.map_err(...)` target didn't need to change.
- File now has zero `serde_json` references.

### `✏️editor/🦀️.rs` (1 site)
- Added a **hand-written** `impl dsl::FromValue for Puzzle5dDocument` (local twin type, right after
  its struct definition, ~line 378) that does
  `serde_json::from_value(Value::from(&value)).map_err(|error| dsl::ValueError::new(error.to_string()))`.
  Needed because `🎮️commands/🛍️set-fixture-json/🦀️.rs` (a concurrent session's already-completed
  work, confirmed by reading it) calls
  `dsl::os_pack::json::from_json_str::<Puzzle5dDocument>(json_text)`, which requires
  `Puzzle5dDocument: dsl::FromValue` — a bound the type didn't have (prior session's diff had
  stripped the derive because it can't expand over the struct's `Option<serde_json::Value>` fields,
  and at the time nothing in the crate called `FromValue` on it; that's since changed). Did **not**
  re-derive `value_derive::FromValue` (would still fail to expand over the `Option<Value>` fields);
  used the framework's pre-existing `From<&DslValue> for serde_json::Value` bridge instead, same
  pattern the file's own `command_from_action` already uses one function away.
  This is the one intentional use of `From<&DslValue>` I added — it mirrors an already-accepted
  pattern in the same file rather than inventing a new bridge style, and only exists because the
  type's `Option<serde_json::Value>` fields make a derive-based `FromValue` structurally impossible.
- Did not need `ToValue` for `Puzzle5dDocument` — grepped every `🎮️commands/**` reference to it;
  only `set-fixture-json` needed the trait, and only `FromValue`.
- Extensively audited but found no other genuine mismatches in this file: every remaining
  `serde_json::` call site (73 total, ~70 untouched) targets either a local twin type (still
  correctly `Serialize`/`Deserialize`-only), `Puzzle5dPlaySnapshot.0` itself, or test-only code under
  `#[cfg(test)]` (where the canonical types' `#[cfg_attr(test, derive(Serialize, Deserialize))]`
  makes `serde_json` valid). No stray `#[value(...)]` attributes remain on de-valued structs. No
  `.get(<integer>)` array-index trap (`.get(1)`/`.get(2)` hits are all plain `Vec<Value>::get`, not
  `Value::get`). No stale `unwrap_or_else` beside my new infallible `to_json_string` calls.

## Not converted / explicitly left alone
- All ~70 other `serde_json` sites in `editor/🦀️.rs` — confirmed correct, not leftovers (see above).
- `🎮️commands/**` (including `world-relocate/🦀️.rs`, which I noticed still does
  `serde_json::from_value::<[f64;3]>` on what should now be a `dsl::os_pack::json::Value` arg — a
  real-looking mismatch, but out of my slice; another session owns that directory).
- `Puzzle5dPlaySnapshot`, `Puzzle5dCommand`'s JSON-args design, `OpBinary` impl — all pre-existing,
  documented, intentional boundaries; not touched.

## Verification performed (no cargo)
- Re-read every changed region from disk after editing (both files, in full for precompute).
- `grep -n '\.get([0-9]'` on both files — 3 hits, all `Vec::get`, not the `Value::get` trap.
- `grep -n serde_json` on precompute — zero hits.
- `grep -n 'from_dsl_value'` on both files — zero hits (no accidental bridging).
- `grep -n unwrap_or_else` beside `to_json_string`/`from_json_str` in both files — zero hits.
- Confirmed `impl From<&DslValue> for serde_json::Value` exists at
  `🧰️framework/🔨️modules/🌱️value/🦀️.rs:218` before relying on it (read-only).
- Confirmed `dsl::ValueError` is a valid path by finding its existing use in
  `✏️s/…/🧊️3d/🦀️.rs:13,38` (read-only reference).
- Confirmed `BrushPlacePayload`/`Fixture` derives by reading
  `✏️s/…/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` directly (read-only reference).

## Files edited
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️.rs`

No other files touched. No cargo commands run at any point.
