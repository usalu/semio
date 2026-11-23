# Seed Rebind Across Source Turns

## Design

`NativeOwnedStorage` now pins the template (`NativeSeedTemplate::same`) and a monotonic part cursor, not the first part's Work binding. Each part is admitted under whichever live effect funds it: the grant admits the exact part quote and at least one work unit remains. The struct `receiver_template` and the enum seed plan therefore survive several Source turns and several Work scopes.

- `SeedState` lost its `work` field and holds `template`, `cursor`, `copied`.
- `initialize_template_at` and `release_at` no longer compare the effect to a pinned Work. The `between_source_actions` variants keep one invariant: a resident Source owner must present an effect with a Source turn.
- Kept invariants: same template (fn-pointer identity), monotonic cursor, exact part quote admitted by the grant, funded work unit, typed access only after the last part.
- Dropped: refusal of a foreign effect ("native partial template changed its original Work"). Foreign-template refusal stays.

## Law

`📦️storage/🧪️tests/template.{json,schema.json,test.ts,rs}`: `foreignWorkAccepted:false` became `rebindWorkAccepted:true`, and `fieldPlan.worksAcrossParts:2` was added. The Rust law seeds the 10-part field plan alternating two independent `EffectBank` Work scopes for every cancellation cut and releases the partial prefix under the other scope. The TS twin checks the fixture and schema.

## Receipt decision (record open)

The published per-turn receipt (`last_receipt`, the Pending progress, the failure effects) was the child spend only. The Source books the receiving scope's own control charge (about 1 item, 81 B) in the same turn, so a caller debiting the receipt under-counted. Product fix: `publish` takes the turn's start progress and publishes the physical Source delta of the whole turn (never below the semantic child receipt). Verified in the run: receipt == `since(cx.retained_progress(), start)` == observed heap on every step. The test was right.

## Other fixes

- `NativeReceivingFailure` implemented `NativeReceivingEffectError` without `cause()`, so `flat()` in the record replaced every real cause with "opening custody changed". It now delegates `cause()` to the selected domain.
- Test fixture `caller()` calls `retain_dispatch()`. Production keeps one Source dispatch across Pending; without it the decoder refuses the second Receive ("pending decoder requires original recovery").

## Files

- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📦️storage/🌱️seed/🦀️.rs`, `…/📦️storage/🦀️.rs`
- `…/📦️storage/🧪️tests/template.rs`, `template.json`, `template.schema.json`, `template.test.ts`
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/📬️outcome/🛬️receiving/⚠️failure/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🫴️record/🦀️.rs` and `🧪️tests/🦀️.rs`
- Helper: `seed-cargo.sh` (root workspace for the store crate; `SEEDROOT=…/🧰️framework` for framework crates, since `semio-framework-value-native-storage` is not a root member).

## Results

| Target | Result |
| --- | --- |
| `semio-framework-value-native-storage --lib` | 17 passed, including the new cross-Work seed law |
| `semio-framework-job --features testkit` `receiving_floor` | passes |
| `semio-framework-value-derive` | derive source unchanged; the run stops at 3 failing retirement tests (`wide_record_retires...`, `composite_retirement...`, `retained_generic_composite_fields...`) unrelated to receive |
| `semio-os-store-member-open-store --lib` | 1 passed, 3 failed |

### Record tests: remaining blocker (not seeding)

With the rebind, the dispatch fixture and the receipt fix, receiving now runs about 70 turns (struct seeds across turns, receipts exact). All three tests then fail identically (same effects 6 items, 1019 B, independent of turn size) at the same point: the derived receiver of `DrawingGroupBody` (`#[value(flatten)] base`, defaulted `isolation`, `children`) gets `NativeFieldMatch::Ambiguous` from the schema key matcher (`🧩️record/🔎️schema`, `combined()` of two non-missing matches), which the derive turns into "native receiving original type, field or phase is invalid". The identical failure under a 1x and a large grant says it is deterministic schema semantics, not turn splitting. Next step: find which key matches both the flattened `DrawingLayerBase` branch and the group's own fields (or a stale matcher frame for a nested group) in `🔎️schema/🧭walk.rs`.

Cleanup: all `[DEBUG]` prints were removed; `🗑️generated/native` (cargo target and build dirs) remains for reuse.
