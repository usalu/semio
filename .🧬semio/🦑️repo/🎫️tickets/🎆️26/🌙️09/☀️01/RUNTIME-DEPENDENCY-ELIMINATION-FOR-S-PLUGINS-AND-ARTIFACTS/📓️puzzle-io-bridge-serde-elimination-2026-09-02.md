# 🧩️ puzzle: converting the production JSON io bridges off `serde_json`, and dropping the forced `Serialize`/`Deserialize` on `Puzzle{2,3,5}dSnapshot`

Follow-up to `📓️puzzle-serde-to-value-conversion.md` (the 290-derive-site sweep). That agent
correctly left `Puzzle2dSnapshot`/`Puzzle3dSnapshot`/`Puzzle5dSnapshot` on an **unconditional**
`#[derive(Serialize, Deserialize)]` because real production `serde_json::to_value`/`from_value`
call sites existed for those exact types. This pass finds every one of those call sites, converts
them to the first-party `ToValue`/`FromValue`/`DslValue` path, and downgrades the three Snapshot
types' serde derive to `#[cfg_attr(test, derive(Serialize, Deserialize))]` (the sanctioned
"test"-bucket end state, kept only as the differential oracle the `🧪️tests/**` fixture suite
checks against).

**Verification was not attempted** — a peer session's `cargo check --workspace` held the exclusive
build-directory lock the entire session (per the task brief), and the task explicitly forbids
running any `cargo` command. Everything below is checked by `rustfmt --check` (all touched files:
0 diffs, i.e. valid syntax) and by hand-tracing every type through the real trait/impl definitions
in `🧰️framework` and the sibling `🧱️block` plugin (which had already completed the identical
pattern for its own io bridges — used throughout as a live precedent, see "Precedent" below).

## What forced the derive, and what actually got converted

Three distinct call-site groups turned out to reference `Puzzle{2,3,5}dSnapshot` through
`serde_json::to_value`/`from_value` directly, not just the two `🚪️io` files the task background
named:

1. **The `🚪️io/📥️import`/`📤️export` JSON codec leaves** (6 files: 2d/3d/5d × import/export) — the
   ones literally named in the task background.
2. **Each artifact's `🧬️mutations/🦀️.rs`, `//#region 🔖️ValueBridge`** — `impl MutationDiff<Value>`/
   `impl Mutation<Value>` and `puzzle{2,3,5}d_document_delta_operations`, where `Value =
   serde_json::Value` is the play app's own legacy scratch-fixture boundary type. These round-trip
   through the typed Snapshot via `serde_json::from_value`/`to_value` too — same forcing effect on
   the derive, just not under `🚪️io`.
3. **Each artifact's `🧬️mutations/🦀️.rs`, `//#region 🔖️PlaySnapshot`** — `Puzzle{2,3,5}dPlaySnapshot`
   (the `ArtifactEditor::Snapshot` type for the play app) materializes/reads its typed
   `Puzzle{2,3,5}dSnapshot` via the same `serde_json::to_value`/`from_value` pattern internally
   (`Puzzle3dPlaySnapshot::new`/`value()`; for 2d/5d, indirectly through the `Value` impls in group 2
   since those `PlaySnapshot`s are bare `Value` newtypes).

All three groups, across all three artifacts (9 files total, since `🧬️mutations/🦀️.rs` covers
groups 2+3 in one file per artifact), were converted in this pass. Pattern used everywhere:
`serde_json::to_value(snapshot)` → `dsl::ToValue::to_value(snapshot)` (infallible, returns
`DslValue`) then `Value::from(dsl_value)` (existing `impl From<DslValue> for serde_json::Value` in
`🧰️framework/🔨️modules/🌱️value/🦀️.rs`) where a `serde_json::Value` still needs to come out the other
side (the `🔖️ValueBridge`/`🔖️PlaySnapshot` boundary type, deliberately **not** retyped — see
"What was NOT retyped" below). `serde_json::from_value::<Snapshot>(v)` → `dsl::FromValue::from_value
(dsl::DslValue::from(&v))` (`impl From<&serde_json::Value> for DslValue`, same file). For the
`🚪️io` leaves specifically (which bridge through stdio's own `JsonSnapshot`/`JsonValue`, not raw
`serde_json::Value`), used `dsl::json::from_dsl_value`/`.to_serde_value().into()` — see "Precedent"
below, this is copied verbatim from the already-converted `block2d`/`block3d`/`block5d` leaves.

### Precedent: `🧱️block`'s io leaves were already done, and confirm the pattern

`✏️s/🔌️plugins/🧱️block/🗿️artifacts/{◻2d,🧊️3d,🖐️5d}/…/🚪️io/{📥️import,📤️export}/…/🔣️json/…/🦀️.rs` and
the matching `Block{2,3,5}dSnapshot` derives were **already fully converted** (found mid-session,
not by me) — `#[derive(…, dsl::ToValue, dsl::FromValue, dsl::DslRecord, ArtifactSchema)]` +
`#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]`, io leaves reading:
```rust
let raw = dsl::ToValue::to_value(snapshot);
Ok(JsonSnapshot::from_value(dsl::json::from_dsl_value(&raw)))
```
and
```rust
let raw: dsl::DslValue = from.to_serde_value().into();
let snap: Block3dSnapshot = dsl::FromValue::from_value(raw).map_err(...)?;
```
This is copied verbatim onto all 6 puzzle io leaves. Block's `🧬️mutations/🦀️.rs` has **no**
`ValueBridge`/`PlaySnapshot` region at all (no legacy play-app scratch-`Value` baggage), so it
didn't need groups 2/3 — puzzle does, hence the larger scope here.

## Two compile-blocking bugs found and fixed along the way (not part of the original task, but
## directly in the files this pass had to touch)

Both are casualties of the prior 290-site mechanical sweep treating a locally-opaque `Value`
(`use serde_json::Value;`) as if it were a local, convertible type. Neither was in the task
background; both are real, confirmed (by reading the actual trait/impl definitions, not
speculation) blockers in the exact region this pass converts.

1. **`ArtifactChild<S>`'s `Serialize`/`Deserialize` is `#[cfg_attr(test)]`-only** (see
   `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2648`, its own doc comment explains why —
   `local_owner`/`_snapshot` fields aren't `derive`-able generically). `Puzzle5dSnapshot.kind_catalogs:
   Option<store::ArtifactChild<SemioKitSnapshot>>` under an *unconditional* `#[derive(Serialize,
   Deserialize)]` (the pre-existing "kept dual-derive" state) could never have compiled outside test
   builds — confirmed no other unconditional `Serialize`/`Deserialize` impl for `ArtifactChild<S>`
   exists anywhere. Fixed as a side effect of this pass's own downgrade (Puzzle5dSnapshot's serde is
   now `cfg_attr(test)` too, so this stops mattering in production builds) — noted in the struct's
   doc comment.

2. **`Puzzle{2,3,5}dPlaySnapshot` needs `ToValue + FromValue`** (`ArtifactEditor::Snapshot: … +
   protocol::ToValue + protocol::FromValue + …`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`,
   3 occurrences of that exact bound; `Puzzle{2,3,5}dPlayApp` binds `type Snapshot =
   Puzzle{2,3,5}dPlaySnapshot`) but:
   - `Puzzle2dPlaySnapshot`/`Puzzle5dPlaySnapshot` are `struct X(pub Value)` (bare
     `serde_json::Value` field) and had `#[derive(Clone, Debug, value_derive::ToValue,
     value_derive::FromValue)]` — the derive macro needs `Value: ToValue + FromValue` for its
     generated body, and **no `impl ToValue for serde_json::Value` exists anywhere in this
     codebase** (searched exhaustively — the only `impl ToValue for Value` hits are unrelated local
     `enum Value` types in `🧠️neural/⚙️engine` and `🧮️math/🎯️sampling`). This could not have compiled.
   - `Puzzle3dPlaySnapshot` (a named-field struct wrapping `Arc<Puzzle3dSnapshot>` +
     `OnceLock<Arc<Value>>`) had **no** `ToValue`/`FromValue` impl at all — missing entirely, not
     even a broken derive attempt.
   - Same root cause broke `Puzzle{2,3,5}dPlaySnapshot`'s `store::ArtifactPack::encode_pack_with`/
     `decode_pack_with`, which called the generic `dsl::to_dsl_value(&self.0)`/`dsl::from_dsl_value
     (value).map(Constructor)` helpers (`T: ToValue`/`FromValue`-bound) against the same bare
     `serde_json::Value`.

   Fixed by hand-writing `impl dsl::ToValue`/`impl dsl::FromValue` for all three `PlaySnapshot`
   types, bridging through `DslValue`'s own `serde_json::Value` `From` impls (the same bridge
   `ArtifactChild<S>` itself uses as precedent for "hand-written, not derived" in `🏪️store`), and
   rewriting `encode_pack_with`/`decode_pack_with` to call those `DslValue::from(...)`/
   `Value::from(...)` conversions directly instead of the generic trait-bound helpers.

## What was NOT retyped (deliberately, to keep blast radius bounded)

- **`Value` (`serde_json::Value`) stays the `🔖️ValueBridge`/`🔖️PlaySnapshot` boundary type in all
  three artifacts.** Only the *internal* round-trip through the typed Snapshot was converted to go
  via `DslValue`; every external signature (`fn apply(&self, projection: &Value) -> …<Value>`,
  `Puzzle{2,3,5}dPlaySnapshot(pub Value)`/`.value() -> &Value`, `puzzle{2,3,5}d_document_delta_
  operations(before: &Value, after: &Value)`) is byte-for-byte unchanged. Retyping these to
  `DslValue` would cascade into the ~2000 other `serde_json::` call sites across editor
  commands/windows/brush code the prior agent already identified and explicitly deferred as its own
  wave — out of scope here, and far too large to attempt without compiler feedback.
- **Field-level dual-derive on the ~84 non-`ArtifactSchema` "prod"-bucket types** (`Puzzle2dCamera`,
  `Puzzle2dNode`, `Puzzle2dEdge`, `Puzzle2dMeta`, `Puzzle5dKindCatalogsExtra`, `Puzzle5dPart`, …) is
  untouched — still unconditional `Serialize + Deserialize + ToValue + FromValue`. Now that their
  *root* Snapshot no longer needs serde for JSON i/o, this dual-derive is inert extra weight (safe,
  additive, never a compile risk per the original sweep's own reasoning) but could in principle be
  downgraded too if no other independent serde_json call site reaches them directly. Not checked
  field-by-field in this pass — spot-checked (all of `Puzzle2dCamera`/`Node`/`Edge`/`Meta`/
  `KindCompatibility`/`KindCatalogs`/`CatalogWireKind` and the 3d/5d equivalents actually used by the
  Snapshot fields I touched) to confirm the derive I *added* to the Snapshot roots is safe, nothing
  more.
- `normalize_kind_catalogs_for_snapshot_value` (5d `🧬️mutations/🦀️.rs`) — a `Value -> Value` helper
  that internally still uses `serde_json::from_value`/`to_value` on `Puzzle5dKindCatalogs`/
  `Puzzle5dKindCatalogsExtra`/the composed `ArtifactChild<SemioKitSnapshot>` handle. Left exactly
  as-is; it operates on different types than the Snapshot itself and its output still gets routed
  through the new `DslValue::from(...)` bridge at every call site.
- 🏪️`store`'s `pack_rt` bridge (`json_values_equal`, used in every `PlaySnapshot`'s `PartialEq`) —
  untouched per the task's explicit instruction; not a `serde_json::to_value`/`from_value` call
  anyway (structural JSON-value equality, different concern).

## Found but NOT fixed — flagging for the next pass (same bug class as #2 above, different files)

Same root cause (`Option<Value>`/`Value` field + `value_derive::ToValue`/`FromValue` derive, no
`ToValue for serde_json::Value` impl exists) in **5 more structs**, none of which are
`Puzzle{2,3,5}dSnapshot`/`PlaySnapshot` or reachable from the io bridges — they're the play apps'
own `Fixture`/`Document` scratch types (clipboard/media-import helpers), a separate concern from
this task's assigned scope. Confirmed by reading each struct; NOT fixed here (out of the
"io bridges + friends" scope this ticket named, and each needs its own review of what "empty"
should mean for that field — not a pure mechanical swap like the ones above):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:188`
  — `Puzzle3dObject.scale: Option<Value>` (this is the editor's own **Fixture**-local
  `Puzzle3dObject`, distinct from the schema's `Puzzle3dObject` in `🗿️artifacts/🧊️3d/🦀️.rs:180`,
  which is fine — that one has no bare `Value` field).
- same file `:227` — `Puzzle3dFixtureMeta.kind_catalogs: Option<Value>` and
  `.kind_compatibility: Option<Value>` (2 fields, same struct).
- same file `:241` — `Puzzle3dTargetVolume.scale: Option<Value>` (editor-local variant, distinct
  from the schema's `Puzzle3dTargetVolume` in `🗿️artifacts/🧊️3d/🦀️.rs:256`, which is fine).
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:365`
  — `Puzzle5dPart3d`, one `Option<Value>` field.
- same file `:408` — `Puzzle5dDocument`, one `Option<Value>` field.

Recommended fix (same shape used for `ArtifactChild<S>`/`Puzzle{2,3,5}dPlaySnapshot` above, but
scoped to just the one field instead of hand-writing the whole struct): use `value_derive`'s own
documented `#[value(serialize_with = "path", deserialize_with = "path")]` field attributes (see
`🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs`'s header doc, "Supported field attributes") to swap
just that field's codec for a small hand-written `fn(&Option<serde_json::Value>) -> DslValue` /
`fn(DslValue) -> Result<Option<serde_json::Value>, ValueError>` pair (bridging through
`DslValue::from`/`serde_json::Value::from` same as everywhere else in this doc), rather than
hand-writing `ToValue`/`FromValue` for the whole struct (which would mean re-deriving every other
field's `rename`/`rename_all`/`default` behavior by hand — much higher risk without compiler
feedback). `Serialize`/`Deserialize` on all 5 structs are still unconditional and fine (native
`serde_json::Value: Serialize + Deserialize`) — only the added `ToValue`/`FromValue` half is broken.

## Files touched

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
  (all three: added `dsl::ToValue, dsl::FromValue` to the derive list; downgraded `Serialize,
  Deserialize` container + all per-field `#[serde(...)]` to `#[cfg_attr(test, ...)]`; dropped the
  now-dead `use serde::{Deserialize, Serialize};` import, matching the fully-qualified
  `serde::Serialize`/`serde::Deserialize` convention already used repo-wide for this exact
  end-state)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/{📤️export,📥️import}/…/🔣️json/🔖️rfc8259/✳️any/🦀️.rs`
  (2 files, 2d) — converted to the `dsl::ToValue`/`dsl::json`/`dsl::FromValue` pattern
- same for `🧊️3d` and `🖐️5d` (4 more io files, 6 total)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/{◻2d,🧊️3d,🖐️5d}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
  (3 files) — `ValueBridge` region's internal `serde_json::to_value`/`from_value` calls converted;
  `PlaySnapshot` region's derive replaced with hand-written `ToValue`/`FromValue` (2d/5d) or added
  from scratch (3d); `ArtifactPack::encode_pack_with`/`decode_pack_with` rewritten to avoid the
  broken generic `dsl::to_dsl_value`/`dsl::from_dsl_value` calls (2d/5d) or the equivalent (3d)

12 files touched total. No `Cargo.toml` changes, no framework changes, no deletions.
