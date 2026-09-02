# 🪵️ Sourcing plugin: 11 → 0 errors (curate artifact serde→ToValue/FromValue finish)

Baseline measured myself: `cargo check -p semio-s-plugin-sourcing --message-format short` → **11
errors** (confirmed both by `grep -cE ': error(\[|:)'` counting the 9 distinct located
diagnostics, and by rustc's own `due to 11 previous errors` summary line — the 2-error gap between
those two counts is a short-format dedup quirk on identical-looking `E0277` lines, not a
miscount; rustc's own tally is authoritative and was used to confirm baseline=11 and final=0).

## Root cause (confirmed, not assumed)

`ArtifactChild<SemioKitSnapshot>` embeds `SemioKitSnapshot`, whose `Serialize`/`Deserialize` were
correctly downgraded to `#[cfg_attr(test, …)]` by this ticket's stdio wave (stdio itself checks at
0 errors, unaffected). Sourcing's own `CurateArtifact` (editor-facing) and `CurateSnapshot`
(persisted) schema types still carried unconditional `#[derive(…, Serialize, Deserialize)]` and
`#[serde(…)]` attributes alongside `#[value(…)]`/`ToValue`/`FromValue` — the unfinished half of
the same conversion.

## Fixes made

1. **`🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`** — `CurateArtifact`:
   dropped `Serialize, Deserialize` from the derive list and the `#[serde(rename_all = …)]`
   attribute (kept `#[value(…)]`). Not used through `serde_json` anywhere in this crate (verified
   by grep), so no cascading fix needed here.

2. **`🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`** —
   `CurateSnapshot`: same derive/attribute drop. This type IS oracled through JSON by the
   `mutate-curate-1` cross-language differential (`../../../🧪️tests/mutate-curate-1/🦀️.rs`), whose
   Rust subject runs inside a **normal (non-`cfg(test)`) generated test-host binary** — a plain
   `#[cfg_attr(test, derive(Serialize, Deserialize))]` retention would NOT survive into that
   binary (it links the plugin crate as an ordinary path dependency, not through `cargo test`'s
   unit-test compilation unit), so the file's own pre-existing doc comment on
   `encode_curate_snapshot_json`/`decode_curate_snapshot_json` already explained this constraint.
   Fix: rewrote both bridge functions to use `dsl::json::to_json_string`/`dsl::json::from_json_str`
   (the `ToValue`/`FromValue`-based JSON bridge at `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:1404`,
   explicitly built for this exact ticket) instead of `serde_json::to_string`/`from_str`. The
   `mutate-curate-1` adapter compares through `semio_repo_test_host::parse_json` (semantic JSON
   equality), not raw bytes, so this is safe regardless of key-ordering differences between the two
   codecs.

3. **`🗿️artifacts/🗂️curate/🦀️.rs:203`** (`catalog_child_handle`) — hashed
   `serde_json::to_string(&catalog.types)` (where `catalog.types: Vec<SemioKitType>`, whose
   `Serialize` is also test-only now) → `dsl::json::to_json_string(&catalog.types)`. `SemioKitType`
   already derives `value_derive::ToValue`/`FromValue` unconditionally in stdio, so this compiles
   with no further changes.

4. **`✏️editor/🎮️commands/📄️set-artifact-json/🦀️.rs`** — cascading break from #2:
   `serde_json::from_str::<CurateSnapshot>(&payload.json)` (production `handle()`) no longer
   compiles once `CurateSnapshot` drops `Deserialize`. Replaced with the crate's own
   `decode_curate_snapshot_json` bridge (import added:
   `crate::artifacts::curate::schema::snapshot::decode_curate_snapshot_json`). Also fixed the
   sibling `#[cfg(test)]` unit test in the same file (`serde_json::to_string(&expected)` →
   `dsl::json::to_json_string(&expected)`) for consistency — not itself one of the 11, and not
   caught by plain `cargo check`, but left broken it would fail `cargo test`.

5. **`✏️editor/🦀️.rs`** (the two `E0053` errors) — `ArtifactEditor::command_from_action`/
   `host_configuration_mutation` now take `Option<&dsl::DslValue>` (this crate aliases
   `semio_framework_os_kernel` as both `dsl` and `protocol`; followed the file's own existing
   convention of `protocol::…`). Mirrored 📐️cad's already-converted twin
   (`cad_command_from_action`/`CadPlayApp`'s trait impl) exactly:
   - `sourcing_curate_command_from_action` (free fn) and the two trait-impl overrides retyped to
     `Option<&protocol::DslValue>`.
   - `serde_json::Value::as_str/as_f64/as_bool` → `protocol::DslValue::as_str/as_f64/as_bool`.
   - The `text_of`/number-formatting closure: `serde_json::Value::Number(number) => number.to_string()`
     → matched on `protocol::Number::{UInt,Int,Float}` (no blanket `Display` on `Number`, same
     pattern cad uses).
   - `json_field`'s non-string fallback: `other.to_string()` (serde_json's `Display`) →
     `protocol::json::to_json_string(other)`.
   - Three `#[cfg(test)]` call sites that constructed args via `serde_json::json!({…})` updated to
     `protocol::DslValue::from(&serde_json::json!({…}))` — `DslValue: From<&serde_json::Value>` is
     a sanctioned bridge impl in the value crate itself, the same pattern cad's tests use; this is
     NOT one of the 11 either, but left unfixed it would break `cargo test` for a change that ships
     right next to it.

## Rule compliance

- No `serde_json::Value` bridging introduced anywhere in production code paths (rule: don't bridge
  through `serde_json::Value` to keep it linked). The two remaining `serde_json` uses in this file
  cluster (`✏️editor/🦀️.rs`'s other ~8 hits, `📸️snapshot/📝️text/🦀️.rs:53`'s `ObjectKind` fixture
  parse, `TypologyNode`/`ObjectKind`'s own derives in `🧬️schema/🦀️.rs`) are untouched — they don't
  reference `ArtifactChild<SemioKitSnapshot>` or `SemioKitType`, aren't part of the 11 errors, and
  weren't broken by this change.
- No serde re-added to `SemioKitSnapshot`/stdio; stdio confirmed still 0 errors after this change
  (see verification below).
- No dual derives left anywhere touched.
- Did not touch `🧪️oracle/`, `🧪️tests/`, `🔬️probes/`, `🏭️generator/`, `🧫️fixtures/`.

## Verification

Ran foreground, no Monitor, no sub-agents:

```
cargo check -p semio-s-plugin-sourcing --message-format short   # 11 → 0
cargo check -p semio-s-plugin-stdio --message-format short      # stayed 0 (no regression)
```

Exact before/after counts are in the closing chat summary (both commands' real output was
captured, not assumed).
