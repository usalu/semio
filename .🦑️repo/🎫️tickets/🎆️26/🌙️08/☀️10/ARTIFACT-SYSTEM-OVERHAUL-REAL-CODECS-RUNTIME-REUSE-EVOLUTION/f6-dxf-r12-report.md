# F6 — 🖊️dxf (standard r12): OpText/OpBinary + DiffCodec

**Scope**: implement `protocol::DiffCodec` for `DxfDiff` and `protocol::OpText`/`protocol::OpBinary`
for `DxfMutation`, per `f6-recon-report.md`'s §9 procedure. Ownership boundary respected:
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/**` + this report only. No shared files touched
(`📦️glue.rs`, `📜️script.ts`, SDK traits, `dsl`/`protocol`/`schema` framework crates, `🏪️store`,
`POLICY_DIFF_COMPLETENESS_ALLOWLIST`).

## Step 1 — real classification (verified, not trusted from the recon table)

The recon report's own sweep guessed **HAND-ROLL (3a only, zero tri-state)** for dxf. Verified for
real by temporarily adding the derive and running `cargo check -p semio-s-plugin-stdio --lib`,
then reverting:

**Diff side** (`#[derive(dsl::DslDiff)]` added to `DxfDiff`, cascaded with
`#[derive(dsl::DslRecord)]` on `DxfEntityModified` to surface the root cause directly):
```
error[E0277]: the trait bound `DxfEntityDiff: DslField` is not satisfied
   --> …/🖊️dxf/…/🔺️diff/🦀️component.rs:906:60
906 | pub struct DxfEntityModified { pub index: usize, pub diff: DxfEntityDiff }
```
`DxfEntityDiff` is a data-carrying enum (`Replace{entity}` + one variant per typed entity kind) —
no `DslField` source exists for it, derivable or otherwise (recon §3a). Confirmed **HAND-ROLL**.
Zero `Option<Option<_>>` anywhere in the diff tree — 3b does not apply, matching the table's
"zero tri-state" note.

**Mutation side** (`#[derive(dsl::DslOps)]` added to `DxfMutation`):
```
error[E0277]: the trait bound `v_r12::…::DxfEntity: DslField` is not satisfied
   --> …/🧬️mutations/🦀️component.rs:57:42
 57 |     InsertEntity { index: usize, entity: DxfEntity },
```
(also fails identically at `SetEntity`, and independently at `SetSnapshot{snapshot:DxfSnapshot}`
since the snapshot tree itself contains `DxfEntity`). Confirmed **HAND-ROLL**, independent
blocker, same root cause (3a) as the diff side.

Both temporary derive attempts were reverted before writing the real hand-rolled implementation —
`DxfDiff`/`DxfMutation`'s derive lists are unchanged from before this session (still
`Clone, Debug, Default, PartialEq, Serialize, Deserialize[, ArtifactSchema]`).

Full compiler transcripts kept in this ticket folder: `f6-dxf-diff-derive-check1.txt`,
`f6-dxf-diff-derive-check2.txt`, `f6-dxf-mutation-derive-check1.txt`.

## Step 2b — hand-rolled implementation

### `🔺️diff/🦀️component.rs` — `impl protocol::DiffCodec for DxfDiff`

New region `HandcraftedDiffCodec`, following §5's grammar template exactly:

- **Primitives**: `hex_encode`/`hex_decode`/`enc_str`/`dec_str` (hex for strings/bytes),
  `enc_f64`/`dec_f64` (bare decimal — Rust's `f64` `Display`/`FromStr` round-trip losslessly, no
  hex needed for floats), `split_top_level`/`strip_brackets`, `encode_option`/`decode_option`
  (`[0]`/`[1,x]`), and a new `enc_list`/`dec_list` generic core (self-bracketing plain list) that
  every `Vec<T>` field's grammar (group codes, vertices, entities, header vars, tags, …) is built
  on top of.
- **Value codecs**: `DxfValue` (data-carrying enum, the file's own second root-cause example of
  3a) tag-prefixed `S[hex]`/`I[digits]`/`D[float]`/`P[x,y,z]`; `DxfEntity` (the whole entity, not
  its diff) tag-prefixed `L`/`C`/`A`/`W`/`T`/`S`/`I`/`O` for
  Line/Circle/Arc/Polyline/Text/Solid/Insert/Other.
- **Item codecs**: self-bracketing positional tuples for every full (non-diff) struct type touched
  — `DxfHeaderVar`/`DxfLayer`/`DxfStyle`/`DxfLinetype`/`DxfBlock`/`DxfTag`/`DxfOtherTable`/
  `DxfTables`/`DxfSnapshot` (the last needed because `DxfMutation::SetSnapshot` carries the whole
  snapshot — exercised by the Mutation side, not the Diff side).
- **Diff-value codecs**: sparse positional tuples (`encode_option` per field) for
  `DxfHeaderVarDiff`/`DxfLayerDiff`/`DxfStyleDiff`/`DxfLinetypeDiff`/`DxfBlockDiff`/
  `DxfTablesDiff`, and a tag-prefixed `DxfEntityDiff` (`R`=Replace carrying a whole tagged
  `DxfEntity`, else one letter per kind matching `enc_dxf_entity`'s own tags).
- **Collection triples**: two NEW generic cores, `enc_name_triple`/`dec_name_triple` and
  `enc_index_triple`/`dec_index_triple` — mirrors this file's own pre-existing
  `DxfNamedElem`/`DxfIndexElem` generic structural-diff cores one level up (string grammar instead
  of diff algebra), used by all 6 collections (`header_vars`/`layers`/`styles`/`linetypes` name-keyed,
  `blocks`/`entities` index-keyed — the latter reused verbatim for a block's own nested
  `entities`, exactly like the structural diff algebra already does).
- **Top level**: `header-vars=… tables=… blocks=… entities=…` space-separated tokens, absent
  token = unchanged (same shape `GifDiff`/`SvgDiff` use). `encode_diff`/`decode_diff` =
  `print_diff().into_bytes()` (same simplification `WriterDiff`/`GifDiff`/`SvgDiff` use).

All helpers touched by the Mutation side's hand-rolled codec are `pub(crate)` (matches svg's
pattern of the diff file's primitives being reused by its mutations sibling).

### `🧬️mutations/🦀️component.rs` — `impl protocol::OpText`/`OpBinary for DxfMutation`

Replaced the old `serde_json`-stub `OpText`/`OpBinary` with a real hand-rolled `keyword arg=value
...` grammar (space-separated, same shape the derive's own handcrafted-wrapper convention uses),
reusing `🔺️diff`'s `pub(crate)` primitives (`enc_str`/`dec_str`, `enc_header_var`/`dec_header_var`,
`enc_layer`/`dec_layer`, `enc_style`/`dec_style`, `enc_linetype`/`dec_linetype`, `enc_block`/
`dec_block`, `enc_dxf_entity`/`dec_dxf_entity`, `enc_dxf_snapshot`/`dec_dxf_snapshot`) rather than
duplicating them a second time — one match arm per of the 19 non-`NoMutation` variants.
`encode_op`/`decode_op` = `print_op().into_bytes()`, same simplification as the diff side.

## Step 3 — tests (both added, both pass)

- `diff_codec_text_binary_roundtrip_law` (in `🔺️diff`'s new `handcrafted_diff_codec_tests` mod):
  a single `DxfDiff` exercising every collection triple simultaneously (name-keyed
  removed+modified+added on `header_vars`/`layers`; index-keyed on `blocks`/`entities`, including
  a NON-`Replace` kind-preserving `Line` patch, a `Replace` (kind-change) entry, and a nested
  block-level `entities` sub-diff — the same `DxfEntitiesDiff` grammar reused at two tree depths).
  Asserts `!printed.contains('\n')`, `parse_diff(print_diff(d)) == d`,
  `decode_diff(encode_diff(d)) == d`, determinism, and the empty-diff fixed point.
- `op_text_binary_roundtrip_law` (in `🧬️mutations`'s existing `tests` mod, reusing the file's own
  `variants()` fixture — all 20 variants including `SetSnapshot` against `sweep_b()`, which
  exercises `other_tables` raw retention and a nested block's own entities inside the whole-snapshot
  grammar). Same four assertions per variant.

## Step 4 — verification (real, this session)

| Check | Result |
|---|---|
| `cargo check -p semio-s-plugin-stdio --lib` (dxf files only, isolating from concurrent sibling-artifact churn) | clean, 0 dxf-related errors |
| `cargo test -p semio-s-plugin-stdio --lib "artifacts::dxf"` | **15/15 passed**, including both new law tests |
| `cargo test -p semio-s-plugin-stdio --lib` (whole crate) | **1075/0** (baseline before this session's work: 1073; +2 new law tests) |

**Note on transient full-crate failures observed mid-session**: two full-crate runs during this
session showed unrelated failures — a stale-manifest error citing a `semio-s-3d`/`✏️s/🔨️modules/…`
path that doesn't exist anywhere in the live tree (confirmed by grep; resolved itself on retry —
filesystem/cache glitch from a concurrent writer, not a real dependency), and one run where
`artifacts::docx::…::diff_codec_text_binary_roundtrip_law` failed with a tri-state assertion
message, then passed cleanly on the very next run with no code changes in between. Both are
another F6 sub-agent's (docx) or unrelated (3d) transient mid-edit states in this same live, concurrently-worked
tree — not caused by, and not touched by, this session. The final `1075/0` full-crate run above is
clean. `git diff --stat` confirms only the two owned dxf files changed by this session (977
insertions / 19 deletions across `🔺️diff/🦀️component.rs` and `🧬️mutations/🦀️component.rs`).

## Deviations from §5's grammar conventions

- Added a new `enc_list`/`dec_list` generic core (self-bracketing plain list) not present in
  svg's/gif's own primitive sets — a straightforward generalization of the repeated
  `format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))` pattern that recurs for
  every one of dxf's many `Vec<T>` fields (group codes, vertices, entities, header vars, tags).
  Kept file-local (`pub(crate)`), not exported — same "reuse within this file only" rule the
  file's own pre-existing `DxfIndexElem`/`DxfNamedElem` cores already follow.
- Added `enc_name_triple`/`dec_name_triple` and `enc_index_triple`/`dec_index_triple` as NEW
  generic cores (svg/gif wrote each collection's triple codec concretely, since they only had 1-2
  collections each). dxf has 6 collection triples (4 name-keyed, 2 index-keyed, one reused at two
  tree depths), so generalizing was the direct, in-spirit continuation of this file's own
  established generic-core style (`DxfIndexElem`/`DxfNamedElem`/`generic_apply`/`generic_between`/
  `generic_absorb_pair`/`named_apply`/`named_between`/`named_absorb_pair` already do the identical
  thing one layer up, for the structural diff algebra rather than the string grammar).
- Floats are encoded as bare decimal (`enc_f64`/`dec_f64` via Rust's own `Display`/`FromStr`, which
  round-trip losslessly for finite `f64`) rather than hex — §5 only prescribes hex for
  strings/bytes; svg/gif had no float fields to set a precedent either way. This keeps the grammar
  human-legible for the frequent point/radius/angle fields DXF entities carry.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — hand-rolled `impl protocol::DiffCodec for DxfDiff` (full grammar + ~50 helper functions,
  many `pub(crate)` for mutations-side reuse), + `handcrafted_diff_codec_tests::diff_codec_text_binary_roundtrip_law`
  test. Module doc updated with the real compiler citation.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — hand-rolled `OpText`/`OpBinary` for `DxfMutation` (reusing the diff file's `pub(crate)`
  primitives) replacing the old `serde_json` stubs, + `op_text_binary_roundtrip_law` test. Module
  doc updated with the real compiler citation.
- Ticket-folder scratch (`.txt`, kept per repo rules): `f6-dxf-diff-derive-check1.txt`,
  `f6-dxf-diff-derive-check2.txt`, `f6-dxf-mutation-derive-check1.txt`,
  `f6-dxf-diffcodec-check1.txt`, `f6-dxf-mutcodec-check1.txt`, `f6-dxf-mutcodec-check2.txt`,
  `f6-dxf-poll-1.txt`, `f6-dxf-poll-2.txt`, `f6-dxf-test1.txt`, `f6-dxf-full-crate-test1.txt`.

**No shared files touched**: `📦️glue.rs`, `📜️script.ts`, `dsl`/`protocol`/`schema` framework crates,
`🏪️store` were all read-only for this session. `POLICY_DIFF_COMPLETENESS_ALLOWLIST` untouched (0
stdio entries before and after — `stdio.dxf.diff` now has a real hand-rolled `DiffCodec` impl in
its own file, satisfying `📜️script.ts`'s literal-text `diff-completeness` check without any
allowlist edit, same mechanism the recon pilot used for binary/gif89a/svg).
