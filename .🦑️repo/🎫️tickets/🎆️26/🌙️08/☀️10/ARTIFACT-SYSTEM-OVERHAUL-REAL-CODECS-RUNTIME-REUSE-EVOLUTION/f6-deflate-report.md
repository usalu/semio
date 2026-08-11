# F6 — 🗜️deflate (standard rfc1950) — OpText/OpBinary/DiffCodec Report

## Summary

| Side | Path taken | Verdict source |
|---|---|---|
| Mutation (`DeflateMutation` → `OpText`/`OpBinary`) | **DERIVE** (`#[derive(dsl::DslOps)]` + handcrafted `OpText`/`OpBinary` wrapper, P6) | Real `cargo check`: compiled clean once `DeflateLevelHint` got `dsl::DslScalar` and `DeflateSnapshot` got `dsl::DslRecord` — zero errors from any deflate file. |
| Diff (`DeflateDiff` → `DiffCodec`) | **HAND-ROLL** (3b: tri-state) | Real `cargo check`: `error[E0277]: the trait bound std::option::Option<u32>: DslField is not satisfied` on `DeflateDiff::dict_id: Option<Option<u32>>` (line 37). |

This confirms the recon report's row 21 guess exactly ("`dict_id: Option<Option<u32>>`-shaped, small") for the **Diff** side, and additionally establishes — beyond what the recon table covered (it only classified the Diff side per-standard) — that the **Mutation** side is cleanly DERIVE-eligible: `DeflateSnapshot`'s whole field tree has zero data-carrying enums (`DeflateLevelHint` is unit-variant-only), so `SetSnapshot`'s payload binds without needing a `FlowMutationDsl`-style mirror enum. Same "diff hand-rolled, mutation derived clean" split the recon report documents for gif 89a and (per its own report) for zip 2.0.

## STEP 1 — Classification (verified for real, not trusted from the table)

### Diff side
Temporarily added `#[derive(dsl::DslDiff)]` to `DeflateDiff` (no cascading derives needed first — the tri-state blocker is on a top-level field, `dict_id`, not nested inside another struct). `cargo check -p semio-s-plugin-stdio --lib` gave:
```
error[E0277]: the trait bound `std::option::Option<u32>: DslField` is not satisfied
  --> …/🗜️deflate/…/🔺️diff/component.rs:37:25
   |
37 |     pub dict_id: Option<Option<u32>>,
   |                         ^^^^^^^^^^^ the trait `DslField` is not implemented for `std::option::Option<u32>`
```
Exactly the 3b failure mode `f6-recon-report.md` §3b describes: `classify_field` peels one `Option<..>` layer, leaving `Option<u32>` itself, and no `impl<T: DslField> DslField for Option<T>` exists anywhere in the `dsl` crate. `dict_id` is `DeflateDiff`'s *only* tri-state field (`compression_method`/`window_bits`/`compression_level_hint`/`payload` are all plain single-layer `Option<T>` — "field changed at all", not a nullable-value tri-state) and the *only* blocker on the Diff side. Reverted the experimental derive attempt (diffed the restored file byte-for-byte against a pre-edit backup in the ticket scratchpad, confirmed clean) and hand-rolled `DiffCodec` instead.

### Mutation side
Added `#[derive(dsl::DslOps)]` to `DeflateMutation` directly (no experimentation needed — `DeflateSnapshot`'s only enum, `DeflateLevelHint`, is unit-variant-only, so the 3a blocker structurally cannot apply). First pass surfaced only "nested struct needs its own `dsl::DslRecord`/`dsl::DslScalar`" — added:
- `DeflateLevelHint` (unit-only enum `Fastest`/`Fast`/`Default`/`Maximum`) → `#[derive(dsl::DslScalar)]`
- `DeflateSnapshot` → `#[derive(dsl::DslRecord)]` (added alongside, not replacing, the existing hand-rolled `store::ArtifactDsl`/`store::ArtifactPack` — same treatment `BinarySnapshot` got in the pilot: `DslRecord` only gives `DeflateSnapshot` a `DslField` impl so it can be embedded as `SetSnapshot`'s payload, it does not touch the artifact's own honest hex-text/raw-binary envelope format)

Re-ran `cargo check` — **zero errors from any deflate file** (confirmed by grepping the full-crate check output for `🗜️deflate` — no `error[` lines, only pre-existing unrelated warnings from other artifacts). Kept `dsl::DslOps` on `DeflateMutation` and wrote the standard §2 handcrafted `OpText`/`OpBinary` wrapper on top (P6: the derive never emits these, even on full success).

Note: `SetPresetDictionary.dict_id: Option<u32>` (a single-layer Option, "the new value to set/clear") is a *different* field from `DeflateDiff.dict_id: Option<Option<u32>>` (the diff tri-state) — it bound fine on the derive path, exactly matching the recon report's §3's prediction that mutation-side single Options don't trigger 3b.

## STEP 2 — Implementation

### Mutation side (DERIVE + handcrafted wrapper)
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`:
- `#[derive(dsl::DslOps)]` added to `DeflateMutation`.
- `#[dsl(block)]` on `SetSnapshot.snapshot` (struct-valued field, matches `BinaryMutation`/`GifMutation`/`SpaceMutation` precedent).
- `#[dsl(base64)]` on `SetPayload.payload: Vec<u8>` (bare `Vec<u8>`, not `Option<Vec<u8>>`, so the attribute actually takes effect per the recon report's documented derive quirk).
- Replaced the `serde_json`-based `OpText`/`OpBinary` stubs with the exact §2 handcrafted wrapper (`dsl::DslVariants::variants()`/`to_named_record`/`from_named_record` + `dsl::variants_binary::encode_op`/`decode_op`) — copied verbatim from `BinaryMutation`/`GifMutation`'s precedent.
- Added a new `#[cfg(test)] mod tests` block (this file had none before) with `op_text_binary_roundtrip_law`.

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`:
- `#[derive(dsl::DslScalar)]` on `DeflateLevelHint`.
- `#[derive(dsl::DslRecord)]` on `DeflateSnapshot`, `#[dsl(base64)]` added to its `payload: Vec<u8>` field.

### Diff side (hand-rolled `DiffCodec`)
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — new `//#region 🔖️HandcraftedDiffCodec` region, following `f6-recon-report.md` §5's template and gif 89a's actual code as the literal pattern (this artifact needed none of gif's enum-tag or collection-triple machinery — `DeflateSnapshot` has zero data-carrying enums and zero keyed collections, just five scalar/weak fields):
- **Primitives**: `hex_encode`/`hex_decode`, `parse_u8`/`parse_u32`, `split_top_level` (bracket-depth-aware, kept even though nothing here nests brackets today — shared grammar contract per the recon report), `strip_brackets`, `encode_option`/`decode_option` (uniform `[0]`=None / `[1,<T>]`=Some(T) tag) — copied verbatim from gif 89a's own primitive set.
- **Value codecs**: `enc_level_hint`/`dec_level_hint` (`DeflateLevelHint` → single char `f`/`a`/`d`/`m`, same style as gif's `enc_disposal`/`dec_disposal`).
- **Top level**: `print_deflate_diff`/`parse_deflate_diff` — space-separated `name=value` tokens (`compression-method=`, `window-bits=`, `level=`, `dict-id=` [tri-state via `encode_option`/`decode_option`], `payload=` [hex]), absent token = unchanged.
- `impl protocol::DiffCodec for DeflateDiff`: `print_diff`/`parse_diff` delegate to the above; `encode_diff`/`decode_diff` = the text bytes verbatim (same simplification `WriterDiff`/gif89a/svg's hand-rolled `DiffCodec`s use — satisfies every LAW without inventing a denser wire format).
- Doc comment on the region cites the real `cargo check` error verbatim (line/column-accurate) per the citation-style convention set by `GifFrameDiff`/`SvgDiff`.

No index-transport/absorb-algebra machinery was needed (unlike gif 89a's `absorb_indexed_collection`/`inverse_indexed_collection` generics) — `DeflateDiff`'s `apply`/`absorb`/`between`/`inverse` (plain scalar LWW, per the recipe's "no strong entities" rule) already existed pre-F6, untouched by this ticket; only the text/binary *codec* layer (`print_diff`/`parse_diff`/`encode_diff`/`decode_diff`) was added.

## STEP 3 — Tests (added, both mandatory)

Both required per §9's STEP 3, added to their respective files' existing test modules (no new test files created, per repo rules):

- `op_text_binary_roundtrip_law` — new `#[cfg(test)] mod tests` in `🧬️mutations/🦀️component.rs` (this file had no test module before). Exercises all 5 variants incl. `SetSnapshot` (struct payload), both `SetPresetDictionary` arms (`Some`/`None`), and an empty-`Vec<u8>` `SetPayload`. Asserts `!print_op().contains('\n')`, `parse_op(print_op(x)) == x`, `decode_op(encode_op(x)) == x` for each.
- `diff_codec_text_binary_roundtrip_law` — added to `🔺️diff/🦀️component.rs`'s existing `#[cfg(test)] mod tests` (which already had `field_sweep`/`mutation_diff_law`/`inverse_law`/`absorb_law`/`between_roundtrip_law`/`codec_retention_law` from prior waves). Reused the module's existing `sweep_a()`/`sweep_b()` fixtures (already engineered to differ in every field, `sweep_a.dict_id = None` / `sweep_b.dict_id = Some(0xDEAD_BEEF)`) — `between(a,b)` exercises the tri-state `Some(Some(_))` arm, `between(b,a)` exercises `Some(None)`. Also covers the empty diff and two synthetic edge diffs (`dict_id` explicitly cleared, empty `payload`). Asserts the same 3 laws as above for `print_diff`/`parse_diff`/`encode_diff`/`decode_diff`.

## STEP 4 — Verification (real, both this session)

| Check | Result |
|---|---|
| `cargo check -p semio-s-plugin-stdio --lib` (full crate, after implementation) | 0 errors from any `🗜️deflate` file (grep-confirmed); crate-wide compiled clean (0 errors total once sibling F6 sub-waves' own in-flight edits — pptx/png/gltf, confirmed as their own uncommitted changes via `git status`, not this session's fault — finished landing) |
| `cargo test -p semio-s-plugin-stdio --lib "artifacts::deflate"` | **19/19 passed, 0 failed** (17 pre-existing + 2 new law tests) |
| `cargo test -p semio-s-plugin-stdio --lib` (whole crate) | **1049/0** (all passed, 0 failed) |

## Deviations from §5's grammar conventions

None of substance. `deflate` needed no enum-tag scheme (no data-carrying enum anywhere) and no collection-triple scheme (no keyed collection anywhere), so the hand-rolled codec is a strict subset of gif 89a's template — only the `Primitives` region, one `ValueCodecs` pair (`enc_level_hint`/`dec_level_hint`, analogous to `enc_disposal`/`dec_disposal`), and the `TopLevel` region were needed. `split_top_level` is technically unused by any real bracket-nesting case in this artifact's current field set (only `decode_option`'s own `[0]`/`[1,<v>]` shape uses it, and its payload never itself contains a `,`) but was kept per the shared-primitive-set convention rather than hand-pruned, matching how the recon report frames these primitives as a reusable, not project-specific, template.

## Files touched (real, live, not reverted)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — `dsl::DslScalar` on `DeflateLevelHint`, `dsl::DslRecord` + `#[dsl(base64)]` on `DeflateSnapshot`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — hand-rolled `impl protocol::DiffCodec for DeflateDiff` (primitives + value codecs + top-level print/parse, doc-comment-cited compile error), + `diff_codec_text_binary_roundtrip_law` test added to the existing test module.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — `dsl::DslOps` on `DeflateMutation` (derived clean), `#[dsl(block)]`/`#[dsl(base64)]` attributes, handcrafted `OpText`/`OpBinary` replacing the `serde_json` stubs, + new `#[cfg(test)] mod tests` with `op_text_binary_roundtrip_law`.
- Ticket-folder scratch (kept per repo rules): `f6-deflate-check1.txt`, `f6-deflate-check2.txt`, `f6-deflate-test1.txt`, `f6-deflate-full-crate-test.txt`, `deflate-diff-backup.rs` (pre-edit backup used to verify the reverted experimental-derive file diffed clean).

**No shared files touched**: `📦️glue.rs`, `📜️script.ts`, the `dsl`/`protocol`/`schema` framework crates were all read-only for this session. `POLICY_DIFF_COMPLETENESS_ALLOWLIST` (`📜️script.ts:2304`) was not touched — the goal (this artifact's diff file now has a real `DiffCodec` impl, so it drops out of the live `dsl-migration/diff-completeness` policy check on its own) is met without any allowlist edit.
