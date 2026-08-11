# F6 — 🎒️zip (standard 2.0) — OpText/OpBinary/DiffCodec Report

## Summary

| Side | Path taken | Verdict source |
|---|---|---|
| Mutation (`ZipMutation` → `OpText`/`OpBinary`) | **DERIVE** (`#[derive(dsl::DslOps)]` + handcrafted `OpText`/`OpBinary` wrapper, P6) | Real `cargo check`: compiled clean once `ZipCompressionMethod` got `dsl::DslScalar` and `ZipExtraField`/`ZipEntry`/`ZipSnapshot` got `dsl::DslRecord`. |
| Diff (`ZipDiff` → `DiffCodec`) | **HAND-ROLL** (3b: tri-state) | Real `cargo check`: `error[E0277]: the trait bound std::option::Option<i64>: DslField is not satisfied` on `ZipEntryDiff::unix_mtime: Option<Option<i64>>`. |

This confirms the recon report's row 18 guess ("Likely the `unix_mtime: Option<Option<i64>>` tri-state… small, single tri-state field — cheap hand-roll") for the **Diff** side, and additionally establishes — beyond what the recon table covered — that the **Mutation** side is cleanly DERIVE-eligible (the recon table only classified Diff-side per-standard; §3 of the recon report predicts this split explicitly: "mutation payloads don't usually have tri-state… 3b rarely applies here", and zip has zero data-carrying enum anywhere in its Mutation-reachable tree — `ZipCompressionMethod` is unit-variant-only). This is the same "diff hand-rolled, mutation derived clean" split the recon report documents for gif 89a.

## STEP 1 — Classification (verified for real, not trusted from the table)

### Diff side
Added `#[derive(dsl::DslDiff)]` to `ZipDiff` and cascading `#[derive(dsl::DslRecord)]` to `ZipEntryDiff` (needed just to reach the real blocker one level deeper). `cargo check -p semio-s-plugin-stdio --lib` gave:
```
error[E0277]: the trait bound `std::option::Option<i64>: DslField` is not satisfied
  --> …/🎒️zip/…/🔺️diff/component.rs:38:28
   |
38 |     pub unix_mtime: Option<Option<i64>>,
```
Exactly the 3b failure mode `f6-recon-report.md` §3b describes: `classify_field` peels one `Option<..>` layer, leaving `Option<i64>` itself, and no `impl<T: DslField> DslField for Option<T>` exists. This is zip's *only* tri-state field and the *only* blocker on the Diff side — reverted the experimental derive attempts (`zip-diff-orig.rs` backup diffed clean against the restored file) and hand-rolled `DiffCodec` instead.

### Mutation side
Added `#[derive(dsl::DslOps)]` to `ZipMutation`. First pass surfaced only "nested struct needs its own `dsl::DslRecord`/`dsl::DslScalar`" errors (`ZipSnapshot`, `ZipEntry`, `ZipExtraField`, `ZipCompressionMethod`) — no data-carrying enum anywhere. Added the four cascading derives (`📸️snapshot/component.rs`):
- `ZipCompressionMethod` (unit-only enum `Stored`/`Deflate`) → `#[derive(dsl::DslScalar)]`
- `ZipExtraField`, `ZipEntry`, `ZipSnapshot` → `#[derive(dsl::DslRecord)]`

Re-ran `cargo check` — **zero errors from any zip file** (confirmed by grepping the output for `🎒️zip` — no `error[` lines, only pre-existing unrelated warnings). Kept `dsl::DslOps` on `ZipMutation` and wrote the standard §2 handcrafted `OpText`/`OpBinary` wrapper on top (P6: the derive never emits these, even on full success).

Note: `SetEntryTimestamps.unix_mtime: Option<i64>` (a single-layer Option, "the new value to set") is a *different* field from `ZipEntryDiff.unix_mtime: Option<Option<i64>>` (the diff tri-state) — it bound fine, exactly matching the recon report's prediction that mutation-side single Options don't trigger 3b.

## STEP 2 — Implementation

### Mutation side (DERIVE + handcrafted wrapper)
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`:
- `#[derive(dsl::DslOps)]` added to `ZipMutation`.
- `#[dsl(block)]` on `SetSnapshot.snapshot` and `AddEntry.entry` (struct-valued fields, matches `BinaryMutation`/`GifMutation` precedent).
- `#[dsl(base64)]` on `SetEntryData.data: Vec<u8>` (bare `Vec<u8>`, not `Option<Vec<u8>>`, so the attribute actually takes effect per the recon report's documented derive quirk).
- Replaced the `serde_json`-based `OpText`/`OpBinary` stubs with the exact §2 handcrafted wrapper (`dsl::DslVariants::variants()`/`to_named_record`/`from_named_record` + `dsl::variants_binary::encode_op`/`decode_op`) — copied verbatim from `BinaryMutation`'s precedent.

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`:
- `#[derive(dsl::DslScalar)]` on `ZipCompressionMethod`.
- `#[derive(dsl::DslRecord)]` on `ZipExtraField`, `ZipEntry`, `ZipSnapshot`.

### Diff side (hand-rolled `DiffCodec`)
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — new `#[region 🔖️HandcraftedDiffCodec]` region, following `f6-recon-report.md` §5's template and gif 89a's actual code (not just its summary) as the literal pattern:
- **Primitives**: `hex_encode`/`hex_decode`/`hex_decode_string`, `split_top_level` (bracket-depth-aware), `strip_brackets`, `encode_option`/`decode_option` (uniform `[0]`=None / `[1,<T>]`=Some(T) tag) — copied verbatim from gif 89a's own primitive set.
- **Value codecs**: `enc_method`/`dec_method` (`ZipCompressionMethod` → single char `s`/`d`, same style as gif's `enc_disposal`), `enc_extra_field`/`dec_extra_field`/`enc_extra_list`/`dec_extra_list` (`ZipExtraField` → positional `[id,hexpayload]`), `enc_entry`/`dec_entry` (whole `ZipEntry` → 14-field positional tuple, field order matching the struct declaration).
- **Diff value codecs**: `enc_entry_diff`/`dec_entry_diff` — `ZipEntryDiff`'s 14 sparse fields as single-letter `tag:value` pairs (`N`=name, `D`=data, `M`=method, `A`=dos_date, `T`=dos_time, `U`=unix_mtime [tri-state, via `encode_option` on the inner `Option<i64>`], `F`=flags, `B`=version_made_by, `V`=version_needed, `I`=internal_attrs, `E`=external_attrs, `L`=local_extra, `C`=central_extra, `O`=comment) — same shape as `GifFrameDiff`'s own tag scheme.
- **Collection triple**: `enc_entries_diff`/`dec_entries_diff` — the one deviation from gif 89a's literal template, noted explicitly in a doc comment: gif's `frames`/`comments`/`app_extensions` are **index**-keyed (`usize` on all three sections), but `ZipEntriesDiff` is **name**-keyed on `removed`/`modified` (`String`, matching `ZipEntriesDiff`'s own field types) while `added` is still index-keyed (`usize`, matching `ZipEntryAdded::index`). Adapted the `name{[removed];[modified];[added]}` shape accordingly: `removed`/`modified` keys are hex-encoded entry names, `added` keys are the final-position index — same bracket-depth-aware `split_top_level`/`strip_brackets` machinery handles both without modification.
- **Top level**: `print_zip_diff`/`parse_zip_diff` — space-separated `name=value` tokens (`comment=<hex>`, `entries{...}`), absent token = unchanged.
- `impl protocol::DiffCodec for ZipDiff`: `print_diff`/`parse_diff` delegate to the above; `encode_diff`/`decode_diff` = the text bytes verbatim (same simplification `WriterDiff`/gif89a/svg's hand-rolled `DiffCodec`s use — satisfies every LAW without inventing a denser wire format).

No index-transport/absorb-algebra machinery was needed on the Diff side (unlike gif 89a's `absorb_indexed_collection`/`inverse_indexed_collection` generics) — `ZipDiff`'s `apply`/`absorb`/`between`/`inverse` already existed pre-F6 (name-keyed collection algebra, untouched by this ticket); only the text/binary *codec* layer (`print_diff`/`parse_diff`/`encode_diff`/`decode_diff`) was added.

## STEP 3 — Tests (added, both mandatory)

Both new tests live in `🧬️mutations/component.rs`'s existing single test module (zip's pre-existing convention — `between_roundtrip_law`/`field_sweep_covers_every_mutable_field` for `ZipDiff` were already colocated there rather than in the diff file's own test module, so the two new tests follow that convention rather than gif 89a's separate-test-module-per-file layout):

- **`op_text_binary_roundtrip_law`** (Mutation): every `ZipMutation` variant, including `SetEntryTimestamps` with both `unix_mtime: Some(_)` and `unix_mtime: None`, and the two nested-record-carrying variants (`SetSnapshot`, `AddEntry`). Asserts `!printed.contains('\n')`, `parse_op(print_op(m)) == m`, `decode_op(encode_op(m)) == m`.
- **`diff_codec_text_binary_roundtrip_law`** (Diff): reuses the existing `sweep_a()`/`sweep_b()` fixtures (already exercise every mutable field including the `unix_mtime` tri-state clear, per `field_sweep_covers_every_mutable_field`) via `ZipDiff::between()` in both directions plus the empty diff. Same three assertions as above, adapted to `print_diff`/`parse_diff`/`encode_diff`/`decode_diff`.

## STEP 4 — Verification (real, all commands actually run)

- `cargo check -p semio-s-plugin-stdio --lib` — 0 errors from any zip file at every stage (baseline, after Mutation-side derive, after Diff-side hand-roll).
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::zip"` → **40/40 passed, 0 failed** (38 pre-existing + `op_text_binary_roundtrip_law` + `diff_codec_text_binary_roundtrip_law`). Full run saved: `f6-zip-scoped-test-run.txt`.
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **1047/1047 passed, 0 failed** (0 filtered, 0 ignored). Full run saved: `f6-zip-full-crate-test-run.txt`. (Count is higher than the recon report's 1019 baseline because other concurrent F6 sibling sessions landed work on other artifacts during this session — confirmed by transient `stl`/`las` compile errors from in-progress concurrent edits that resolved themselves mid-session, per the "Concurrent Cargo Workspace Churn" pattern; polled `cargo check` until clean rather than assuming zip was at fault — see `f6-zip-mutation-derive-check.txt` for one such transient-error capture.)
- Policy check (informational, read-only, `📜️script.ts` NOT touched): `✏️s/…/🎒️zip/…/🔺️diff/component.rs` now literally contains `impl protocol::DiffCodec for ZipDiff` — satisfies `policyDiffCompletenessBreaches`'s literal-text file-level check (`📜️script.ts:3185-3205`) the same way binary/gif89a/svg already do. Did not run the full `bun ./📜️script.ts policy` command (21619-line output, informational only per the recon report — not required for this artifact's completion) but confirmed the text match directly via `grep`.

## Deviations from the recon report's §5 template

1. **Collection-triple key types**: gif 89a's `frames`/`comments`/`app_extensions` triples are uniformly index-keyed (`usize` on removed/modified/added). Zip's `ZipEntriesDiff` is name-keyed on `removed`/`modified` (`String`) and index-keyed on `added` (`usize`) — this is `ZipEntriesDiff`'s own pre-existing (F1-era, not touched by this ticket) field shape, not a codec-layer choice. Adapted the `name{[removed];[modified];[added]}` grammar shape to mixed key types; the underlying bracket-depth-aware `split_top_level`/`strip_brackets` primitives needed no change to support this.
2. **No index-transport algebra needed**: unlike gif 89a, zip's `apply`/`absorb`/`between`/`inverse` (name-keyed collection algebra, rank/unrank arithmetic equivalents) already existed before this ticket — only the `DiffCodec` text/binary codec layer was added, not any diff-algebra logic.
3. **`ZipMutation` landed on DERIVE, not hand-roll**, despite the recon table's row 18 not explicitly stating this (the table only classifies the Diff side per-standard, per its own stated scope in §8's intro: "This catches the Diff-side question precisely. The Mutation-side question… is a SEPARATE check per artifact"). Verified for real per the procedure's STEP 1b rather than assumed.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — hand-rolled `impl protocol::DiffCodec for ZipDiff` (full grammar + helper functions), module-doc-comment citing the real 3b compile error.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — `#[derive(dsl::DslOps)]` on `ZipMutation` (+ `#[dsl(block)]`/`#[dsl(base64)]` attrs), handcrafted `OpText`/`OpBinary` replacing the `serde_json` stubs, `op_text_binary_roundtrip_law` test, `diff_codec_text_binary_roundtrip_law` test.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — `#[derive(dsl::DslScalar)]` on `ZipCompressionMethod`, `#[derive(dsl::DslRecord)]` on `ZipExtraField`/`ZipEntry`/`ZipSnapshot`.
- Ticket-folder scratch (`.txt`, kept per repo rules): `f6-zip-mutation-derive-check.txt`, `f6-zip-scoped-test-run.txt`, `f6-zip-full-crate-test-run.txt`.

No shared files touched: `📦️glue.rs`, `📜️script.ts`, the `dsl`/`protocol`/`schema` framework crates, `🏪️store`, and `POLICY_DIFF_COMPLETENESS_ALLOWLIST` were all read-only for this session.
