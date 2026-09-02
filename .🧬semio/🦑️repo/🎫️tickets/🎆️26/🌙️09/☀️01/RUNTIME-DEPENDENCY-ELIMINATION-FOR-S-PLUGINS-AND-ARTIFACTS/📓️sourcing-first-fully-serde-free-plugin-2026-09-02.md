# 🪵 `semio-s-plugin-sourcing` — first fully serde-free s plugin (0 refs, 0 manifest lines)

Proof-of-recipe crate for the other 28. Measured myself, foreground, no Monitor, no sub-agents.

## Baseline (measured before touching anything)
- `cargo check -p semio-s-plugin-sourcing --message-format short` → 0 errors (clean baseline from
  the prior "11 → 0" wave).
- Production serde-ref count via the project recipe (strip `//` comments, strip `#[cfg(test)] mod`
  blocks by brace matching, match `use serde|serde::|serde_json|#[serde(|derive(… Serialize|
  Deserialize …)`, exclude `_serde::`, `Error::(Serialize|Deserialize)`, `VcsError::`,
  `cfg_attr(test`) → **42**, all inside 9 files. `Cargo.toml [dependencies]` still carried
  `serde.workspace = true` and `serde_json.workspace = true`.

## The 42, exactly as categorized, and what each became

1. **7 dual-derive types in `🗿️artifacts/🗂️curate/🦀️.rs`** (`GeometryRecipe`, `ObjectKind`,
   `SortDirection`, `TableSort`, `Filters` — 5 field-level `default` pairs, `CuratedItem`,
   `ObjectKindExtra`) — each already derived `dsl::ToValue, dsl::FromValue` alongside a `Dsl*`
   derive (`DslEnum`/`DslRecord`/`DslScalar`) AND already carried a matching `#[value(…)]` twin next
   to its `#[serde(…)]`. Per the dsl-derive-vs-value-attribute-trap doc: since `ToValue`/`FromValue`
   IS present, `#[value(…)]` was already correct — verified every twin's arguments matched before
   deleting the serde half. Dropped `Serialize, Deserialize` from each derive list and deleted the
   9 `#[serde(…)]` lines (rename_all ×7, field `default` ×5 — wait, counted precisely: 7×rename_all
   + 5×default = 12 attribute lines, all removed, all with a pre-existing `#[value(…)]` twin).
2. **`TypologyNode`** (`🧬️schema/🦀️.rs`) — same pattern: `#[value(rename_all, default,
   skip_serializing_if)]` twin already existed; dropped the serde half.
3. **`SourcingMutation`** (`🧬️mutations/🦀️.rs`, derives `dsl::Mutations`) and the three
   `MutationLeaf` payloads `DeleteCuratedItem`/`CreateCuratedItem`/`ChangeCuratedItemCount` — same
   pattern, twins already present.
4. **8 × `use serde::{Deserialize, Serialize};`** — 5 were unconditional (curate/🦀️.rs, schema/🦀️.rs,
   mutations/🦀️.rs, and the 3 MutationLeaf files count as 3 more = matches the 8 total once
   `⚙️operations/🦀️.rs` has none) and got deleted outright. **2 were legitimate `#[cfg(test)]`/
   `#[cfg_attr(test, …)]`-gated retentions for genuine differential-oracle tests** —
   `💡️inferences/🦀️.rs` (no oracle actually used `CurateInference` via serde_json anywhere in the
   crate — confirmed by grep — so this one WAS unnecessary cruft and was deleted) and
   `👥️presence/🦀️.rs` (`SourcingCuratePresence` genuinely IS round-tripped through
   `serde_json::from_value`/`to_value` against a committed `🧪️retirement.json` fixture inside
   `#[cfg(test)] mod retirement_tests` — kept, but see the landmine below).
5. **3 production `serde_json::from_str` call sites** → `dsl::json::from_json_str::<T>(…)`:
   - `⚙️operations/🦀️.rs`'s `decode_sourcing_mutation_json` (target: `SourcingMutation`).
   - `🧬️schema/🦀️.rs`'s two calls inside `contributed_sourcing_modules` (targets: `TypologyNode`,
     `Vec<ObjectKind>`). All three targets already derived `FromValue` (category 1/2 above).

## Landmine found and fixed: a `use serde` import can be "production" even when `#[cfg(test)]`-gated

The counting recipe only strips `#[cfg(test)] mod { … }` **blocks**; a standalone
`#[cfg(test)]\nuse serde::{Deserialize, Serialize};` sitting at module scope (not inside a `mod`) is
NOT stripped and still counts, because it pollutes the file's own namespace even though it compiles
to nothing outside `cfg(test)`. The sanctioned fix (precedent: `🧰️framework/🛍️products/💻️os/🔨️modules/
🏪️store/🦀️.rs:2090`, `🌊️flow` plugin, `🌿️vcs/🦀️.rs`) is to delete the standalone `use` and
fully-qualify the derive instead:
```rust
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
```
Applied to `👥️presence/🦀️.rs`'s `SourcingCuratePresence` (the genuine oracle). Its sibling
`SourcingCuratePresenceMutation` carried the identical `cfg_attr(test, derive(Serialize,
Deserialize))` + `cfg_attr(test, serde(rename_all))` pair with **no actual oracle test anywhere** —
confirmed by grepping every use of `SourcingCuratePresenceMutation` in the crate — so that one lost
serde entirely rather than being requalified. Rule of thumb: a `cfg_attr(test, …)` gate on a derive
is not by itself proof the oracle is real; grep for an actual `serde_json` call against that type
before deciding whether to requalify or delete.

## `serde_json::Value` bridge found and closed: `sourcing_catalog_fragment`

`💡️inferences/🦀️.rs` had a real, unconditional production function returning `serde_json::Value`,
built with the `json!` macro, called from `✏️editor/🦀️.rs`'s `export_media` (`catalog:out` media
port, mirroring puzzle's `kit.catalog` shape). This is exactly the "don't bridge through
serde_json::Value" trap. Converted:
- Return type `Value` → `dsl::DslValue`; `json!({…})` → `dsl::DslValue::object([(String, DslValue)…])`
  / `dsl::DslValue::Array(…)` literal construction (no `array()` helper exists on `DslValue`, use the
  variant directly).
- The call site (`editor/🦀️.rs:929`) changed from `.to_string()` (serde_json::Value's `Display`) to
  `dsl::json::to_json_string(&…)` (`DslValue` has no `Display` impl).
- The in-file `#[cfg(test)]` unit test indexed the old `Value` with `fragment["key"]` — `DslValue`
  has no `Index` impl, only `.get(key) -> Option<&DslValue>` (object-key only, confirmed) and
  `.as_array()`/`.as_str()`. Rewrote the test's assertions to `.get(…).and_then(|v| v.as_str())` /
  `.and_then(|v| v.as_array())` chains. This is a same-file `#[cfg(test)] mod` cascading fix (not a
  file under `🧪️tests/`), same category the prior "11→0" wave already fixed cascading breaks in.

## What was deliberately NOT touched (rule compliance)

- `🧪️tests/removes-the-clt-panel-from-the-curation/`, `🧪️tests/appends-a-steel-plate-to-the-
  curation/`, `🧪️tests/raises-the-glulam-beam-count-to-20/` (48 raw serde refs total, 16 each) —
  each is `#[path]`-mounted behind an OWN `#[cfg(test)]` gate in `📦️packages/🦀️rust/🦀️.rs`
  (confirmed by reading the mod tree, not assumed), so none of it is part of a plain
  `cargo check -p semio-s-plugin-sourcing` (no `--tests`) compilation unit — consistent with that
  command reading 0 errors both before and after this session's edits despite these fixtures still
  calling `serde_json::from_str::<CurateSnapshot>` etc. directly (`CurateSnapshot` has carried no
  serde derive, not even `cfg_attr(test)`, since the prior "11→0" wave — this is PRE-EXISTING
  breakage under `cargo test`/`--tests`, not something this session introduced, and it is out of
  scope per the standing "do not touch 🧪️tests/" rule).
- `mutate-curate-1/🦀️.rs` (the cross-language differential subject, `#[cfg(feature = "sut")]`) —
  untouched; already fixed in the prior wave via `decode_curate_snapshot_json`/
  `encode_curate_snapshot_json`.
- `🧬️mutations/🦀️.rs`'s own `#[cfg(test)] mod structural_correspondence_tests` (reads
  `🔣️oracle.json` via `serde_json::Value`) and `📸️snapshot/📝️text/🦀️.rs`'s
  `demo_stock_example_preserves_authored_content_against_json_oracle` test — both genuine
  differential oracles inside `#[cfg(test)] mod` blocks, correctly stripped by the counting recipe,
  left untouched.
- `🗄️stdio`, `🔺️mesh-engine` — not part of this crate, not touched.

## The payoff step

`Cargo.toml [dependencies]` — removed `serde.workspace = true` and `serde_json.workspace = true`
outright (NOT moved with a comment first, then compiled — compiled the CODE first per the standing
rule, confirmed 0 errors, THEN edited the manifest, THEN re-checked). Both crates re-added to
`[dev-dependencies]` (tests need them as oracles), with a docstring mirroring 🔺️mesh-engine's exact
precedent wording.

```
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/8eb2ad71-482d-46b0-b299-0f4ef6f1479d/scratchpad/iso3
export RUSTC_WRAPPER=""
cargo check -p semio-s-plugin-sourcing --message-format short
```
Ran TWICE: once right after the code changes (serde still in `[dependencies]`) → `Finished` profile,
63 warnings, **0 errors**. Ran again after the manifest edit (serde/serde_json now ONLY in
`[dev-dependencies]`) → `Finished` profile, same warning set, **0 errors**. Neither run printed any
`error[` or `error:` diagnostic line.

Production serde-ref count re-measured with the exact same recipe after all edits: **0** (the only
remaining 48 raw hits repo-wide inside this crate are the three excluded `🧪️tests/` fixture files
above, 16 each).

## Replication recipe for the other 28 plugins

1. Baseline: `cargo check -p <crate> --message-format short`, record error count via
   `grep -cE ': error(\[|:)'` (never `^error`, it undercounts).
2. Baseline production serde-ref count with the strip-comments / strip-cfg-test-mod-blocks /
   pattern-match / exclude recipe above. **Do this per-crate before editing** — it tells you exactly
   how many of the 8/`N` `use serde` lines, dual-derives, `#[serde(…)]` attrs, and raw
   `serde_json::from_str`/`Value` call sites you're dealing with, mirroring this crate's 42 → exactly
   the categories in the ticket brief.
3. For every dual-derive type: check whether it derives `ToValue`/`FromValue` (→ `#[value(…)]` is
   correct, drop the serde half) vs. ONLY a `Dsl*` macro (→ the attribute is `#[dsl(…)]`, `#[value(…)]`
   is a hard error) vs. hand-written `impl ToValue`/`FromValue` (→ no container attribute at all).
   See `📓️dsl-derive-vs-value-attribute-trap.md`.
4. For every `use serde::{Deserialize, Serialize};`: if unconditional, delete outright once every
   consumer derive line is stripped. If `#[cfg(test)]`-gated standalone (not inside a `mod`), first
   grep whether ANY test in the crate actually round-trips that exact type through `serde_json` — if
   yes, keep the capability but delete the standalone `use` and fully-qualify
   (`#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]`); if no, delete entirely.
5. For every raw `serde_json::from_str`/`to_string`/`Value` production call site: confirm the target
   type already derives `FromValue`/`ToValue` (do NOT add serde back to satisfy it — grep the derive
   macro's own source first, see `📓️mutations-codegen-does-NOT-require-serde.md`), then swap for
   `dsl::json::from_json_str`/`dsl::json::to_json_string`. For a bare `serde_json::Value` bridge (no
   named type), rebuild the value shape with `dsl::DslValue` literals (`DslValue::object([…])`,
   `DslValue::Array(vec![…])`, `DslValue::String/Null/…`) — there is no `array()` helper, use the
   variant. Fix cascading same-file `#[cfg(test)] mod` breaks (index syntax, `Value` type
   annotations) as part of the same edit; leave any file under `🧪️tests/` (a directory, not an inline
   `mod`) untouched.
6. Recompile, confirm 0 errors, THEN edit `Cargo.toml`: delete `serde`/`serde_json` from
   `[dependencies]`, re-add both to `[dev-dependencies]` if any `#[cfg(test)]`/`cfg_attr(test)` oracle
   survived (with a docstring — mirror 🔺️mesh-engine's wording), omit entirely if none did.
   Recompile again. Never clear a manifest line you have not compiled past first.
7. Re-run the production serde-ref count one final time; it must read 0 (excluding `🧪️tests/`
   fixture directories, which the recipe and the standing rule both leave out).

## Files touched this session

- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🦀️.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-curated-item/🦀️.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-curated-item/🦀️.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-curated-item-count/🦀️.rs`
- `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/Cargo.toml`
