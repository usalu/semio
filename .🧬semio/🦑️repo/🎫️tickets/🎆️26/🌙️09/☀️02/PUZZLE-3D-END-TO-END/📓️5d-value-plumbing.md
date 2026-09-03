# 5d Editor Value Plumbing — dsl Conversion

File: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
(only file edited — confirmed via `git diff --name-only`, exactly one puzzle-plugin `.rs` file changed by this session: the editor file above. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🦀️.rs` — the name-colliding sibling file the coordinator warned about — shows as modified in `git status` but that is a **different, concurrent session's** change (232 insertions / 185 deletions, unrelated to anything in this ticket); this session never opened it.)

## Error-count trend (target file only, grouped by path — never totals)

| Checkpoint | count |
|---|---|
| Baseline (import swap not yet applied) | 32 |
| After import swap + json! qualification only | 32 (blocked by unrelated upstream `DirectoryStreamMessage` error in a concurrent session's file; not measurable) |
| After first (wrong) pass — blanket `serde_json::Value::` qualification everywhere | **165** — flagged by the coordinator as the wrong direction |
| After reverting to dsl `Value` + `puzzle5d_projection_value` bridge + native `parse()`/`from_json_str` + fixing `DslValue::from` seams | 38 |
| After fixing `Number::as_f64` (infallible in dsl), `target.meta` qualification, and bridging `apply_world3d_sun_action` (framework-owned, still `serde_json::Value`) | 4 |
| Final | **0** (verified: `cargo check` exit code 0, "Finished `dev` profile", no upstream blocker this run) |

`serde_json::` reference count in the file (the real acceptance signal, not the error count): peaked at **184** (coordinator's flagged number, all wrongly qualified) → **135** now. Of those 135, **29** are `serde_json::json!(` (the other owner's macro sites — see below) and **106** are legitimate remaining boundary touches, all tied to the 9 structs whose derive conversion is gated (see "Held: struct derive conversion").

## What changed

1. **Import (line 43)**: `use serde_json::{json, Value};` → `use dsl::os_pack::json::{from_json_str, object, parse, to_json_string, to_string, Object, Value};` (verbatim copy of the 🧊️3d reference).

2. **`json!` sites**: 29 bare `json!(` calls (out of the file's then-46 total) were qualified to `serde_json::json!(` so they stay greppable and syntactically untouched for the other owner. 14 sites were *already* `dsl::json!(` when this session started (a concurrent session mid-converting some of them) and were left alone. By the final measurement the concurrent session had brought `dsl::json!(` up to 18 and fixed at least one type mismatch on its own — that overlap is expected and not this session's work.

3. **`puzzle5d_projection_value<T>(value: T) -> Value where dsl::DslValue: From<T>`** — new bridge, copied in spirit from 🧊️3d's `puzzle3d_projection_value`. `Puzzle5dPlaySnapshot`'s inner `.0` (owned by `🧬️mutations/🦀️.rs`, out of scope) is `serde_json::Value` by design — this bridges it into the file's own dsl `Value` **once per function**, not once per `.get()` call. Used at ~45 call sites (all the `Work::step`/`extent`/`scan_grip`/`catalog(s)` functions that used to read `snapshot.0.get(...)` directly).

   **Asymmetry worth flagging**: 🧊️3d's `Puzzle3dPlaySnapshot` has a `.typed()` accessor (returns `&Puzzle3dSnapshot` directly, no per-call bridge needed) that 5d's `Puzzle5dPlaySnapshot` does not have. That accessor lives in `🧬️mutations/🦀️.rs`, out of this ticket's file scope — closing that asymmetry is a finding for that file, not fixed here.

4. **Functions returning `&[Value]` (borrowed, tied to a `Puzzle5dPlaySnapshot`/bridge-temporary lifetime) converted to return owned `Vec<Value>`** — the bridge produces a fresh owned tree, so a function can't hand back a `&[Value]` borrowing from a bridge temporary that dies at the end of the function. Fixed: `Puzzle5dSelectionScan::rows`, `Puzzle5dImportJob::snapshot_rows` (+ new `snapshot_kind_compatibility_rows` sibling, replacing 4 duplicated inline chains), `Puzzle5dKindWeightWork::catalog`, `Puzzle5dAddNodeWork::catalogs` (+ its `Puzzle5dAddBrushPartWork::catalogs` forwarder). `Puzzle5dPatchGripWork::compatibility_rows` stayed a borrowed `&[serde_json::Value]` since it reads `Puzzle5dDocument.kind_compatibility` directly (owned by `self`/`document`, no bridge temporary involved).

5. **Raw JSON-text parsing switched from `serde_json::from_str`/`from_value` to dsl's own `parse`** wherever the parsed value only ever gets read back with `.get()`/`.as_*()` (never handed to a `Serialize`/`Deserialize`-derived struct): `puzzle5d_decode_import_fragment` (kit:in import), `parse_brush_candidates_free`, `Puzzle5dBoardEventsWork::event`/`take_payload`. `Puzzle5dAddBrushPartWork::args` no longer bridges at all — both `self.payload` and `command.args()` are dsl `Value` now, so it's a plain `.cloned()`.

6. **`dsl::DslValue::from(x)` → `dsl::os_pack::json::to_dsl_value(x)`** at 13 sites where `x` used to be `&serde_json::Value` (the `From<&serde_json::Value> for DslValue` trait impl target) but is now this file's own dsl `Value` after the projection-bridge conversion — `DslValue` has no `From<&Value>` (dsl) impl, only the `to_dsl_value` free function.

7. **`serde_json::from_value::<[f64; 3]>(value.clone())` / `<[f64; 4]>`** (8 sites) replaced with two new small helpers, `puzzle5d_value_as_f64_3`/`_4`, reading straight off `Value::as_array()`/`.as_f64()` — dsl's `Value` has no `Deserialize`, so the serde round trip can't survive the conversion; these read the same shape natively instead.

8. **`Puzzle5dPart3d.scale`, `Puzzle5dDocument.{meta,kind_catalogs,kind_compatibility}`** stayed typed `Option<serde_json::Value>` (not bare `Value`) — required for those 4 fields' `#[derive(Serialize, Deserialize)]` to keep compiling (dsl `Value` has no `Serialize`/`Deserialize`), matching the struct-level `impl dsl::FromValue for Puzzle5dDocument`'s own doc comment, which already explained why it's hand-written instead of derived: those fields "stay the raw document JSON, matching `Puzzle5dPlaySnapshot`'s own still-`serde_json::Value` boundary."

9. **`puzzle5d_retire_json_step`** stayed typed `&mut serde_json::Value` — forced by its own differential test, which builds its probe value with `serde_json::json!({...})` (an untouched, out-of-scope macro site). Its two real serde_json callers (`Puzzle5dPasteJob.args`/`.fragment_value`) are consistent with that. The third caller (`Puzzle5dPart3d.scale`, in `Puzzle5dPasteJob::close_step`) was rewritten to a flat, single-step capacity-credit disposal instead of reusing the shared recursive stepper, matching the sibling grip/part disposal arms right next to it — `scale` is always bounded (a bare number or short per-axis array), so a recursive walk was unnecessary there.

   **Genuine gap found and reported, not papered over**: `Puzzle5dImportJob.fragment` (now dsl `Value`, built via `parse`) also needed disposal through the same close_step machinery, but dsl's `Object` type (`🎒️pack/🔤️json/🦀️.rs:169`) exposes no `remove`/`iter_mut` — only `get`/`get_mut`/`insert`/`iter`/`len`/`contains_key` — so the same byte-exact, key-at-a-time recursive descent the `serde_json::Value` version uses (pop the last array element / remove one object key per step) **cannot be replicated** for dsl `Value::Object` without adding methods to that framework type, which is out of this file. Fixed with a documented, single-step flat disposal instead (the fragment is already admission-capped at `PUZZLE5D_IMPORT_MEDIA_BYTES`/`PUZZLE5D_IMPORT_SEMANTIC_ITEMS`, so this doesn't risk an unbounded blocking step, but it is not byte-exact the way the sibling recursive disposal is). See `Puzzle5dImportJob::close_step`, the `fragment` arm.

10. **`apply_world3d_sun_action`** (framework-owned, `🔌️plugin/🦀️.rs`, still `use serde_json::Value` — out of scope) bridged at its one call site the same way as the `puzzle5d_document_delta_operations` boundary: `args.map(|value| serde_json::Value::from(&dsl::os_pack::json::to_dsl_value(value)))`.

## Held: struct derive conversion (step 3) — explicitly NOT done

Per the coordinator's gate, the 9 locally-defined structs (`Puzzle5dGrip2d`, `Puzzle5dGrip3d`, `Puzzle5dGrip`, `Puzzle5dFastener`, `Puzzle5dPart2d`, `Puzzle5dPart3d`, `Puzzle5dPart`, `Puzzle5dDocument`, `Puzzle5dPartAnchor`) **still derive `Serialize, Deserialize`**, confirmed by grep just now (`derive(Clone, Debug, PartialEq, Serialize, Deserialize...)` × 8, unchanged). No `dsl::ToValue`/`dsl::FromValue` was added to any of them. The differential test the coordinator asked for (byte-for-byte `to_json_string` vs `serde_json::to_string`, dual-derived, `skip_serializing_if`/`default`/integer-arm coverage, modeled on `value-derive`'s `flatten_nested_struct_matches_serde_json_byte_for_byte`) was **not started** — this session ran out of budget doing the plumbing-correctness work (items 1–10 above) that had to land first for the file to compile at all. This is the natural next step, not yet begun.

Note on the coordinator's name-collision warning: these 9 structs are genuinely distinct from the same-named types at `crate::artifacts::puzzle5d::{Puzzle5dGrip2d,Puzzle5dGrip3d,Puzzle5dGrip,Puzzle5dFastener,Puzzle5dPart}` (defined in the *other* file, `🗿️artifacts/🖐️5d/🦀️.rs`) — the editor file already uses both, side by side, disambiguated by path (bare name = this file's own struct; `crate::artifacts::puzzle5d::X` = the other file's, already dsl-derived, used ~19 times for the canonical artifact/mutation boundary). Converting this file's own 9 structs does not touch or collide with those.

## `.get(<numeric>)` grep (requested check)

Re-ran on the final file state:
```
grep -noE '.{40}\.get\([0-9]+\).{10}' <file>
```
4 hits, all on `Vec<Value>` (via `Value::Array` destructuring or `.as_array()`), i.e. plain `std::vec::Vec::get`, **not** `dsl::os_pack::json::Value::get` (which is object-key-only and would have needed `.as_array().and_then(|a| a.get(n))` or `.get_index(n)`). Zero genuine hazard sites. (Same result both before and after the rework — nothing shifted this from safe to unsafe or vice versa.)

## Numeric-arm sites reviewed

- `part_scale_json`/the `scaleSelection` transform-tool arm: `Some(Value::Number(value)) => [value.as_f64(); 3]` — dsl's `Number::as_f64(&self) -> f64` is **infallible** (not `Option<f64>` like serde's), so the old `.unwrap_or(1.0)` was a compile error, not a semantic choice; removed. Widening `UInt`/`Int`/`Float` to `f64` here is correct on purpose — `scale` is a continuous multiplicative factor, not a count or index, so there is no integer-identity to lose.
- `puzzle5d_value_as_f64_3`/`_4` (new): read origin/orientation/position/direction coordinate arrays — all inherently `f64` fields (never integer-typed), no UInt/Int risk.
- No site was found where a `u64` grid index, count, or id needed to survive a `Value::Number` round trip through code this session touched — the places that write such integers (`json!`/`dsl::json!` macro bodies) are the other owner's json! sites, out of scope here.

## Behaviour changes from dropping `unwrap_or_else` fallbacks

None found needing this in the code actually converted. `to_json_string`/`to_string` (dsl, infallible) were not substituted for any `serde_json::to_string(...).unwrap_or_else(...)` call in this pass — the `serde_json::to_string(&vec![InteractionTarget{...}])` call sites (×2, lines near `puzzle5d_action`/dispatch) serialize a framework `Serialize`-derived struct with no `Value` involved at all, so they were left untouched; they are not part of the `Value` plumbing this ticket covers.

## Remaining `serde_json::json!` sites (other owner's, line numbers as of this session's end)

29 `serde_json::json!(` sites remain (down from the file's original bare-`json!` count of 46 minus the 14+ the other session had already converted before this session started). Exact current line numbers:

```
183, 809, 810(x2 nested), 811, 812, 813, 819, 824, 826, 827, 837, 844, 853, 854, 861,
1568, 1622, 1631, 2248, 2249, 3601, 3603, 3889, 8452, 8496, 8497, 9498, 9758, 9809, 9855
```
(Run `grep -n 'serde_json::json!(' <file>` for the authoritative live list — the other session is actively converting these concurrently and the exact set will keep shifting.)

## Verification

`cargo check -p semio-s-plugin-puzzle` via the prescribed measurement recipe (isolated `CARGO_TARGET_DIR`, `RUSTC_WRAPPER=""`, foreground) ran to completion (exit 0, "Finished `dev` profile", 278 warnings, 0 errors) as of the last run this session made. This *is* a real green build of the whole crate at this moment — not a fluke of the earlier upstream `DirectoryStreamMessage` blocker (that error is gone from this run's output entirely, meaning the concurrent session that had it mid-flight has since fixed or committed past it). Given other sessions are editing sibling files in this same crate concurrently, this green state is a snapshot, not a guarantee it stays green — the acceptance signal that matters per the coordinator's direction is the `serde_json::` reference count in *this* file (135, trending down), not the transient compile status.
