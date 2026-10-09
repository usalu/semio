# r11-baseline execution report

Status: **WRITTEN BUT UNVERIFIED** (build impossible at report time, see Blocker). No pass/fail counts exist yet.

S = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`

## Scan of `#[path]` targets
All 292 `#[path = ...]` declarations under S were checked (shell loop, relative to the declaring file). Exactly the six reported
targets were missing; no other missing target. All six now exist.

## Files created (all real tests, no stubs, no `#[ignore]`)
| File (under S) | Tests | What it checks |
|---|---|---|
| `🧬️schema/💡️inferences/📋️schedules/🧪️tests/🔬️unit/🦀️.rs` | 7 | cell text/number/display, natural order + cell order, filter ops, candidate scope (category/storey/phase), wall schedule via `ModelInference::infer` (items = walls, one group row per wall type, grand total = sum of items), determinism + filter -> 0 items + `table_json`, `dependency` Null/changes with definition |
| `✏️editor/🎭️modes/✏️edit/🪟️windows/🧮️schedule/✏️edit/🧪️tests/🔬️unit/🦀️.rs` | 6 | `apply` yields sparse `SetSchedule` (only changed list), column move/total/heading, filter add/op/value/remove, storey/phase/itemize toggles, refusal codes, preset `created`/`presets` |
| `✏️editor/📌️panels/🌳️outliner/🖼️views/🧪️tests/🔬️unit/🦀️.rs` | 5 | kind -> group map, `groups` order (kind, storey level, name), per-building filter + `views_row` none/some, `scale_text`, rendered outliner en+de |
| `✏️editor/🧩️entities/🕰️phasing/🧪️tests/🔬️unit/🦀️.rs` | 4 | phase choices en/de, `write_phase` -> `SetElementPhase`, storey choices order + building suffix with several buildings, `write_storey` |
| `🚪️io/📤️export/🏗️ifc/🔲️ceilings/🧪️tests/🔬️unit/🦀️.rs` | 5 | level ceiling = `IFCCOVERING`/CEILING with extrusion depth = layer sum, type + layer-set usage, `Qto_CoveringBaseQuantities`, sloped = no extrusion, degenerate skipped + determinism (`document(&m) == document(&m)` needs `Part21Document: PartialEq`; replace by a tag/count comparison if not) |
| `🚪️io/📤️export/📊️csv/🧪️tests/🔬️unit/🦀️.rs` | 5 | `number_text`, schedule table (CRLF, header tokens, total label), quoting round trip via the decoder, diagnostics records (both languages), report sections + determinism |

## File changed
`🧬️schema/💡️inferences/🧊️element-solids/🛤️railings/🧪️tests/🔬️unit/🦀️.rs`: removed POST_SIZE/RAIL_WIDTH/RAIL_DEPTH; new helper
`sections(snapshot, id)` returns `(extents_of(post_profile), extents_of(profile))` from the railing's authored profiles; the analytic
volume, low-railing and post/rail-top assertions use those (post_w*post_d*(height-rail_depth) + rail_w*rail_d*length).

## Other compile errors fixed
None (could not compile far enough to see lib/lib-test errors in the BIM crate).

## Commands and results
`GATE_SLOTS=4 T/🚦️gate.sh r11-baseline -- cargo check --manifest-path ✏️s/.../🏢️model/Cargo.toml -p semio-s-artifact-bim-model --lib --tests --message-format=short`
(4 attempts, outputs in `T/🗑️generated/r11-baseline/check{1..4}.txt`):
1. exit 101: `semio-framework-os-kernel` store file had conflict markers (`store/🦀️.rs:20365 mismatched closing delimiter`).
2. exit 101: `Cargo.lock` write failed (os error 1224, user-mapped section), transient.
3-4. exit 101: `semio-framework-os-kernel` (store) does not compile.

## Blocker (not my files; r11-merge territory: "framework store")
`🧰️framework/🔨️modules/🏪️store/**` references items that no longer exist: `SnapshotRetirementStep`, `SnapshotReadLeaseRefusal`,
`SnapshotReadLeaseRegistry`, `snapshot_registry_alias_demands`, `advance_returned_snapshot_read`, `snapshot_registry_alias_close_step`,
`artifact_retirement_box_byte_demand`, `semio_framework_value::retirement::owned_retirement`, `semio_framework_value::ControlledRetirement`,
trait method `ArtifactEnvelopeVcsFieldAuthority::next_close_byte_demand` (about 40 errors; e.g. `store/♻️retirement/📦️backing/🦀️.rs:54`, `store/🦀️.rs:5470`).
Looks like the framework `value` crate (`retirement` module) and the store came out of the stash pop from different sides.
`r11-exec-merge.md` did not exist after ~45 min of waiting.

## Failing tests table
Not available (no test run possible). wasm32-wasip2 check: not run (same blocker).

## Next steps once `os-kernel` compiles
1. `GATE_SLOTS=4 T/🚦️gate.sh r11-baseline -- cargo test ... -p semio-s-artifact-bim-model --lib > T/🗑️generated/r11-baseline/test.txt`
2. Likely first fixes in my files: `table_json` import if r11-merge moves it into `io/text/inferences/📋️schedules`; `Part21Document: PartialEq`;
   `read_records` header semantics in the csv tests; exact `Qto` areas in the ceilings test; whether the outliner opens the Views node by default.
3. `cargo check ... --target wasm32-wasip2` for the lib.
