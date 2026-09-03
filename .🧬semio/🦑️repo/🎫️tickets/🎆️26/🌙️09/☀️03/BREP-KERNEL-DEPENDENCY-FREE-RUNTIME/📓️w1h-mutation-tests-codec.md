# 🧭️ W1-H — Brep Per-Mutation Tests Off `serde`

## Scope

Converted all 13 brep per-mutation fixture tests under
`B/🧬️schema/🧬️mutations/*/🧪️tests/*/🦀️.rs` (`B` = `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep`)
off `serde_json` and onto the first-party `value_derive::{ToValue,FromValue}` + `pack::json` codecs the
prior serde-elimination wave already established for `SemioBrepSnapshot`/`SemioBrepMutation`. Added
one missing codec fn (`decode_semio_brep_diff_json`) to `🔺️diff/🦀️.rs` — `SemioBrepDiff` had no JSON
decode wrapper yet, unlike snapshot/mutation. `🧬️mutations/🦀️.rs`'s own `mod tests`, and the
`📸️snapshot/🦀️.rs`/`🔺️diff/🦀️.rs` test modules, were already serde-free (only doc-comment mentions of
`serde` remained) — nothing to change there beyond the new codec fn.

## Files changed

Codec addition:
- `B/🧬️schema/🔺️diff/🦀️.rs` — added `pub fn decode_semio_brep_diff_json(text: &str) -> Result<SemioBrepDiff, String>`
  (thin `pack::from_json_str` wrapper, mirrors `decode_semio_brep_snapshot_json`/`decode_semio_brep_mutation_json`),
  in a new `//#region 🌉️ExternalCodecBridge` right after `//#endregion 🔖️HandcraftedDiffCodec`.

13 fixture test files (all under `B/🧬️schema/🧬️mutations/`):
- `✂️delete-edge/🧪️tests/removes-the-closing-edge-and-keeps-its-two-vertices/🦀️.rs`
- `➰replace-curve/🧪️tests/swaps-the-first-edges-line-for-a-circular-arc/🦀️.rs`
- `🏗️create-vertex/🧪️tests/adds-an-apex-vertex-above-the-square/🦀️.rs`
- `🐚create-shell/🧪️tests/adds-a-second-shell-that-reuses-the-face-with-flipped-sense/🦀️.rs`
- `💥delete-shell/🧪️tests/removes-the-only-shell-and-leaves-its-faces-behind/🦀️.rs`
- `📍move-vertex/🧪️tests/lifts-the-third-corner-off-the-base-plane/🦀️.rs`
- `🔗create-edge/🧪️tests/adds-a-diagonal-edge-across-the-square/🦀️.rs`
- `🔷create-face/🧪️tests/adds-an-opposing-face-over-the-same-loop/🦀️.rs`
- `🕳️delete-solid/🧪️tests/removes-the-only-solid-and-leaves-its-shell-behind/🦀️.rs`
- `🗑️delete-vertex/🧪️tests/removes-a-corner-vertex-and-cascades-into-its-two-incident-edges/🦀️.rs`
- `🗺️replace-surface/🧪️tests/swaps-the-faces-plane-for-a-cylinder/🦀️.rs`
- `🚮delete-face/🧪️tests/removes-the-only-face-and-leaves-its-loop-behind/🦀️.rs`
- `🧊create-solid/🧪️tests/adds-a-second-solid-that-treats-the-shell-as-a-void/🦀️.rs`

## Codec pattern used

Imports (per file):
```rust
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::{decode_semio_brep_diff_json, SemioBrepDiff};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{decode_semio_brep_mutation_json, SemioBrepMutation};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{decode_semio_brep_snapshot_json, SemioBrepSnapshot};
use pack::value::ToValue;
use protocol::{Mutation, MutationDiff};
```

Substitutions (every assertion's intent preserved 1:1 — fixed-point canonical-JSON check, apply/inverse
round trip, declared-outcome check — only the codec plumbing changed):

| old (`serde_json`) | new (first-party) |
|---|---|
| `serde_json::from_str::<SemioBrepSnapshot>(text)` | `decode_semio_brep_snapshot_json(text)` |
| `serde_json::from_str::<SemioBrepMutation>(text)` | `decode_semio_brep_mutation_json(text)` |
| `serde_json::from_str::<SemioBrepDiff>(text)` | `decode_semio_brep_diff_json(text)` (new fn) |
| `serde_json::to_value(&x)` / `serde_json::Value` reparse | `pack::json::from_dsl_value(&x.to_value())` vs `pack::json::parse(text)` |
| `serde_json::Value::as_str` | `pack::json::Value::as_str` (same signature, `pack_json`'s `Value` mirrors `serde_json::Value`'s read API) |

`pack::json::{parse, from_dsl_value, Value}` and `pack::from_json_str`/`pack::value::ToValue` come from
the framework `pack` crate (`🧰️framework/🔨️modules/🎒️pack`, dependency name `pack` in stdio's
`Cargo.toml`) — `pack::json::Value` is the drop-in `serde_json::Value` replacement (`PartialEq`,
`as_str`/`get`/etc. all mirror `serde_json`'s), and `from_dsl_value(&DslValue) -> Value` is the
`serde_json::to_value`-equivalent bridge from a `ToValue`-derived type's `DslValue` tree. This is the
exact "canonical-JSON comparison pattern" already used at
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🧬️schema/📸️snapshot/🦀️.rs`
(`pack::json::from_json_str::<PdfSnapshot>`) and the stdio crate root's own `impl_serde_op_codec!` macro
(`pack::json_to_dsl_value`/`pack::parse_json` aliases of the same fns).

## Verification (foreground, this session)

```
RUSTC_WRAPPER="" cargo check -p semio-s-plugin-stdio --tests --message-format short 2>&1 | tee "TICKET/🗑️generated/w1h-check.txt" | grep -c '✳️brep.*error'
```
→ `20`

Full brep-matching error lines (`grep '✳️brep' TICKET/🗑️generated/w1h-check.txt | grep error`):
```
.../✳️brep/🧬️schema/💡️inferences/🏷classification/🦀️.rs:568:29: error[E0425]: cannot find type `PointSolidClassification` in this scope
.../✳️brep/🧬️schema/💡️inferences/🏷classification/🦀️.rs:570:13: error[E0433]: cannot find type `PointSolidClassification` in this scope
.../✳️brep/🧬️schema/💡️inferences/🏷classification/🦀️.rs:571:13: error[E0433]: cannot find type `PointSolidClassification` in this scope
.../✳️brep/🧬️schema/💡️inferences/🏷classification/🦀️.rs:572:13: error[E0433]: cannot find type `PointSolidClassification` in this scope
.../✳️brep/🧬️schema/💡️inferences/🏷classification/🦀️.rs:583:17: error[E0425]: cannot find function `classify_point_on_solid` in this scope
.../✳️brep/🧬️schema/💡️inferences/🌳bounding-volume/🦀️.rs:381:73: error[E0277]: Vec<&&str> cannot be built from iterator over &&&str
.../✳️brep/🧬️schema/💡️inferences/🏷classification/🦀️.rs:603:21: error[E0061]: function takes 3 arguments but 4 supplied
.../✳️brep/🧬️schema/💡️inferences/🏷classification/🦀️.rs:623:21: error[E0061]: function takes 4 arguments but 5 supplied
.../✳️brep/🧬️schema/💡️inferences/🧩tessellation/🦀️.rs:1205:21: error[E0061]: function takes 4 arguments but 5 supplied
.../✳️brep/🧬️schema/💡️inferences/🧩tessellation/🦀️.rs:1232:21: error[E0061]: function takes 4 arguments but 5 supplied
.../✳️brep/🧬️schema/💡️inferences/🧩tessellation/🦀️.rs:1250:21: error[E0061]: function takes 4 arguments but 5 supplied
.../✳️brep/🧬️schema/💡️inferences/🧩tessellation/🦀️.rs:1260:21: error[E0061]: function takes 4 arguments but 5 supplied
.../✳️brep/🧬️schema/💡️inferences/🧩tessellation/🦀️.rs:1294:21: error[E0061]: function takes 3 arguments but 4 supplied
.../✳️brep/🧬️schema/📸️snapshot/🕸️topology/🦀️.rs:752:42: error[E0277]: Body: serde::Serialize not satisfied
.../✳️brep/🧬️schema/📸️snapshot/🕸️topology/🦀️.rs:753:26: error[E0277]: Body: serde::Deserialize<'de> not satisfied
.../✳️brep/🧬️schema/⚙️engine/🦀️.rs:1522:25: error[E0502]: cannot borrow `*self` as mutable, also borrowed as immutable
.../✳️brep/🧬️schema/⚙️engine/🦀️.rs:1525:25: error[E0502]: (same)
.../✳️brep/🧬️schema/⚙️engine/🦀️.rs:1528:25: error[E0502]: (same)
.../✳️brep/🧬️schema/⚙️engine/🦀️.rs:1531:25: error[E0502]: (same)
.../✳️brep/🧬️schema/⚙️engine/🦀️.rs:1534:25: error[E0502]: (same)
```

All 20 are in files I never touched: `💡️inferences/🏷classification`, `💡️inferences/🌳bounding-volume`,
`💡️inferences/🧩tessellation`, `📸️snapshot/🕸️topology`, `⚙️engine` — every one owned by concurrent
kernel-layer workers (W1-A/EngineRep removal, W1-F classify/mass/validate, W1-G tessellation) per
`TICKET/📓️status.md`'s fleet table, not by W1-H's slice. Filtering for MY files specifically
(`grep '✳️brep' | grep -E '🧪️tests|🔺️diff/🦀️\.rs|🧬️mutations/🦀️\.rs|📸️snapshot/🦀️\.rs' | grep error`)
returns **zero** matches — none of the 13 fixture tests, the diff root, the mutations root, or the
snapshot root have any error.

`grep -rl serde_json B/🧬️schema/🧬️mutations` finds only one hit, a doc-comment in a non-Rust spec file
(`💾️binary/📡️.protocol.semio`, historical note, not compiled) — no `.rs` file in the mutations tree
references `serde_json` any more.

Total non-brep errors in the same run: 1740 − 20 = **1720**, spread across many other subsets (peers'
own serde-elimination slices still in flight — not this ticket's concern).

## Runtime semantics verification

`TICKET/📓️h0-harness.md` (STATUS: READY) confirms its isolated harness is deliberately KERNEL-scope
only: it does **not** mount `📸️snapshot/🦀️.rs`'s `SemioBrepSnapshot` root, `🧬️mutations/🦀️.rs`,
`🔺️diff/🦀️.rs`'s top-level `SemioBrepDiff`, or any `🧪️tests/` fixture directory (verified directly —
`grep -n '🧪️tests\|SemioBrepMutation\|decode_semio_brep' TICKET/🔬️harness/lib.rs` returns nothing).
So these 13 fixture tests and the new `decode_semio_brep_diff_json` codec are **compile-verified only**
in this session — `cargo check` confirms every type/trait/fn resolves and every codec call typechecks,
but the actual test bodies cannot execute until the whole `semio-s-plugin-stdio` test binary links,
which needs every other subset's serde-elimination slice to also finish (peers' concurrent work, not
blocked on W1-H).

## Status

Done. 0 errors attributable to this slice; 20 pre-existing brep errors and 1720 other-subset errors are
all peer-owned, left untouched per the ticket's "don't touch non-brep subsets, small hunks, other
workers concurrently edit snapshot/mutations" instruction.
