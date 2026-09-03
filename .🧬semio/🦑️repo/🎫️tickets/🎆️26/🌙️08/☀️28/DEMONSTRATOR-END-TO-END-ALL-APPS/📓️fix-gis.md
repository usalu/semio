# Fix: semio-s-plugin-gis wasm compile errors (serde → DslValue/ToValue/FromValue)

## Scope
Drove `semio-s-plugin-gis` to zero `cargo check --target wasm32-wasip2` errors, continuing a
repo-wide serde/serde_json → `dsl::DslValue` + `ToValue`/`FromValue` migration
(`26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`). Starting point (per the
assigning agent, already fixed before this pass): 33 stale `mutation::` import paths, and the
missing `semio-framework-value-derive` Cargo dependency — that took gis from 270 to 64 errors.

## Repro command
```
cd /Users/ueli/Documents/semio && CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-engines CARGO_BUILD_JOBS=4 CARGO_PROFILE_DEV_DEBUG=false RUSTFLAGS=-Awarnings CARGO_TERM_QUIET=true cargo check -p semio-s-plugin-gis --target wasm32-wasip2 --message-format=short 2>&1 | grep -E ": error" | wc -l
```

## Verified error counts (observed from real command runs, this pass)
- Start of this pass: **64**
- After the `command_from_action`/`value_to_dsl`/`gismap` bridge-function fixes: **48**
- After the schema/mutation/io serde→ToValue conversions: **1** (an `E0252` duplicate `ToValue`
  import I introduced and then fixed)
- **Final: 0 errors, exit code 0** (verified with a clean re-run of the exact repro command above,
  both with `--message-format=short | grep error | wc -l` returning `0` and a plain run returning
  exit code 0).

## Fix classes, with file:line

### 1. `E0046` — missing `Mutation::DESCRIPTORS`/`descriptor()`
The `protocol::Mutation` trait grew two required items (`const DESCRIPTORS`, `fn descriptor`)
declared at `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:148`. Copied the idiom from the
already-migrated sibling `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs`
(a "provisional" single-leaf descriptor, since presence mutations have no `👥️presence/<slug>` leaf
triad of their own).
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs:104`
  — added `Gis3dPresenceMutation`'s `DESCRIPTORS`/`descriptor()` inside its `impl Mutation<Gis3dPresence>`.

### 2. `E0053` — `command_from_action` signature drift (`serde_json::Value` → `dsl::DslValue`)
The `ArtifactEditor::command_from_action` trait method now takes `Option<&dsl::DslValue>`. Fixed
by changing only the signature and converting at the top of the fn body via the direct
`From<&DslValue> for serde_json::Value` bridge (`🧰️framework/🔨️modules/🌱️value/🦀️.rs:218`), leaving
every internal `serde_json::Value`-shaped parsing line untouched:
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:759`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:588`

### 3. `E0277 &str: FromValue` — struct with borrowed fields wrongly deriving `FromValue`
`TerrainSceneStyleJson<'a>` is write-only (built then serialized, never decoded); `&'a str` fields
can't implement `FromValue` (no owned-value lifetime to borrow from). Dropped the `FromValue`
derive, kept `ToValue`, and switched its one caller off `serde_json::to_string` onto
`dsl::os_pack::json::to_json_string`:
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:289,314`

### 4. `E0277` cluster — schema types no longer `serde::Serialize`/`Deserialize`
`GisTerrainSnapshot`/`GisTerrainMutation`/`GisTerrainDiff`, `GisMapSnapshot`/`GisMapMutation`/
`GisMapDiff`/`MapFeature`, and the framework's `MutationMessage` all already derive
`ToValue`/`FromValue` (only their serde derives were dropped upstream) — every error in this
cluster was a **call site** still routing through `serde_json`. Migrated call sites to
`dsl::os_pack::json::{to_json_string, from_json_str, object, to_string, from_dsl_value, Value}`
and the direct `DslValue <-> serde_json::Value` `From` impls
(`🧰️framework/🔨️modules/🌱️value/🦀️.rs:218,236,247,268`):
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
  — `gis_terrain_identity_report_json` (test-bridge JSON report).
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs`
  — `gis_terrain_mutation_report_json` (decode base/mutation, encode the whole report).
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/…/json/…/🦀️.rs:9`
  and the matching `📤️export/…/🦀️.rs:9` — `deserialize`/`serialize` against stdio's `JsonSnapshot`,
  now via `dsl::DslValue::from(&from.to_serde_value())` / `serde_json::Value::from(&snapshot.to_value())`.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:520,602,613`
  — `gis2d_document_json_to_svg`/`gis2d_document_json_from_dwg`.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
  — `gis_map_content_key` (tuple `ToValue` instead of a serde tuple, byte-shape-identical per the
  framework's own `(A,B,C)` `ToValue` doc comment), `enc_json`/`dec_json`, the binary
  encode/decode of `positions`/`routes`/`regions`, and `gis_map_identity_report_json`.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
  — `gis_map_mutation_report_json`.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:580,627`
  — `clone_feature`/`step`'s digest-sizing `serde_json::to_vec` → `dsl::os_pack::json::to_json_string(..).into_bytes()`.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` —
  `gis3d_bounded_serialized_bytes<T: serde::Serialize>` rewritten as `<T: dsl::ToValue>` measuring
  `dsl::os_pack::json::to_json_string(value).len()` directly (dropped the `serde_json::to_writer`
  + byte-counting `Write` shim entirely — simpler and no longer needs `std::io`), and both
  `M: serde::Serialize + Send + 'static` where-clauses (`Gis3dOneItemPreparation`'s
  `impl store::ArtifactStoreOneItemPreparation` and `begin_gis3d_preparation`) changed to
  `M: dsl::ToValue + Send + 'static`.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` — `value_to_dsl`/
  `dsl_to_value` bridge helpers switched from the generic (`ToValue`-bound, therefore inapplicable
  to `serde_json::Value` itself) `dsl::to_dsl_value`/`from_dsl_value` onto the direct
  `DslValue::from`/`Value::from` conversions.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️features/🦀️.rs:20`
  — `patch_routes_operations` no longer round-trips a bare `&str` through `serde_json::json!` +
  the (inapplicable) generic `to_dsl_value`; builds `DslValue::String` directly.

### 5. `E0061`/`E0308` — derive-macro `serialize_with`/`deserialize_with` calling-convention mismatch
The `ToValue`/`FromValue` derive's `#[value(serialize_with = "...")]`/`#[value(deserialize_with =
"...")]` call a single-argument `fn(&T) -> DslValue` / `fn(DslValue) -> Result<T, ValueError>` —
not serde's `fn(&T, Serializer) -> Result<S::Ok, S::Error>` two-argument shape.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔺️diff/🦀️.rs:8-20`
  — `Gis2dConfigDelta.layer_stroke_scale` no longer needs a bridge at all on the `ToValue` side:
  `BTreeMap<String, Option<f64>>` already has a blanket `ToValue`/`FromValue`
  (`🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs:296,301`, `Option<T>` at :201/:209). Removed the
  `#[value(serialize_with = "serialize_scales")]` attribute; kept the serde-only
  `#[cfg_attr(test, serde(serialize_with = "serialize_scales"))]` (the finiteness check is
  redundant defense-in-depth — `Gis2dConfigDelta::apply_into` already rejects non-finite scales on
  the way in).
  - **Deliberately NOT fixed / NOT ported**: the pre-existing serde-only `serialize_scales`
    finiteness validator function stays serde-shaped and is now called only under `cfg(test)`; it
    was never something the `ToValue` path can express (infallible `to_value`), so it wasn't
    ported rather than skipped due to time/risk.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️编辑/🎚️config/🧬️schema/🧬️mutations/👁️set-layer-visibility/🦀️.rs:16-18`
  — `SetLayerVisibility.visible: Option<bool>`'s `#[value(deserialize_with =
  "super::super::required_nullable")]` removed outright: the derive's own codegen already treats
  every field as required (a struct-level/field-level `#[value(default)]` is what would make it
  optional, and none was set), so the field is already "required, nullable" under the plain
  `Option<bool>: FromValue`/`ToValue` blanket impls — the serde-only helper was compensating for
  serde's *own* implicit `Option`-optionality, which this derive never has. Kept the
  `#[cfg_attr(test, serde(deserialize_with = "..."))]` half untouched.

## Shared framework files touched
**None.** Every fix above is confined to `✏️s/🔌️plugins/🌍️gis/**`.

## Skipped / deliberately left as-is
- Nothing was skipped for "another dev's dirty file" reasons — `git status --porcelain` on every
  file this pass touched showed them clean before editing (the only pre-existing dirty file under
  gis was `📦️packages/🦀️rust/Cargo.toml`, already modified by the prior pass per the assignment,
  not touched further here).
- `serialize_scales` (gismap `🔺️diff/🦀️.rs`) — left as a serde-only, `cfg(test)`-only helper; see
  class 5 above.
- Every `#[cfg_attr(test, derive(Serialize, Deserialize))]`/`#[cfg_attr(test, serde(...))]` pair
  across the touched files was left in place untouched — these only compile under `cfg(test)` (not
  reached by a plain `cargo check`) and are the repo's own "second-implementation oracle" pattern
  (CLAUDE.md: cross-validate against a third-party library), not legacy debt.

## Verification
Final command run (fresh, not reused from an earlier count) and its exact output:
```
$ CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-engines CARGO_BUILD_JOBS=4 CARGO_PROFILE_DEV_DEBUG=false RUSTFLAGS=-Awarnings CARGO_TERM_QUIET=true cargo check -p semio-s-plugin-gis --target wasm32-wasip2 --message-format=short
EXIT CODE: 0
$ grep -c ": error" <output>
0
```
No `error[...]` diagnostics remain; only pre-existing framework-level `warning:` noise (dead code /
unnecessary qualification in `🧰️framework`, unrelated to gis) appears in the raw output.
