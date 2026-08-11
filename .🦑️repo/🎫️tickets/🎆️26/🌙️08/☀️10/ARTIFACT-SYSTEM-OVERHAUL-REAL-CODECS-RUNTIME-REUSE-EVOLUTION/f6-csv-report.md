# F6 — 📊️csv (rfc4180) — OpText/OpBinary + DiffCodec

Scope: `CsvMutation` (`OpText`/`OpBinary`) and `CsvDiff` (`DiffCodec`) for the `stdio.csv`
rfc4180 artifact, following `f6-recon-report.md` §9's procedure literally.

## STEP 1 — classification (verified for real, both sides land on HAND-ROLL)

The §8 recon table's guess ("DERIVE probable") was **wrong for both sides** — verified by
actually adding the derive attributes and reading real `cargo check` errors, then reverting.

### Diff side: HAND-ROLL

Added `#[derive(dsl::DslRecord)]` to every nested struct (`CsvFieldDiff`, `CsvRecordDiff`,
`CsvRecordModified`, `CsvRecordAdded`, `CsvRecordsDiff`) and `#[derive(dsl::DslDiff)]` to
`CsvDiff`. Real compile error:

```
error[E0277]: the trait bound `std::option::Option<v_rfc4180::…::CsvFieldDiff>: DslField` is not satisfied
  --> …/🔺️diff/🦀️component.rs:68:24   (pub fields: Option<Vec<Option<CsvFieldDiff>>>)
```

Root cause: `CsvRecordDiff::fields: Option<Vec<Option<CsvFieldDiff>>>`. `classify_field` peels
exactly one `Option<..>` layer, leaving `Vec<Option<CsvFieldDiff>>`; the blanket
`impl<T: DslField> DslField for Vec<T>` then requires `Option<CsvFieldDiff>: DslField`, and no
`impl<T: DslField> DslField for Option<T>` exists anywhere in the `dsl` crate. This is a
`Vec`-wrapped sibling of the recon report's documented §3b tri-state finding (`Option<Option<T>>`)
— same missing blanket impl, one collection layer removed. Reverted the derive attempt; hand-rolled
`DiffCodec` instead, following §5's template.

### Mutation side: HAND-ROLL — for a NEW, undocumented reason (a derive-macro hygiene bug)

Added `#[derive(dsl::DslOps)]` to `CsvMutation`. Real compile error:

```
error[E0308]: mismatched types
  --> …/🧬️mutations/🦀️component.rs:31:9   (record: CsvRecord,  — the InsertRecord variant field)
   | expected reference `&_`, found struct `RecordValue`
```

This is **not** §3a (no enum anywhere in `CsvMutation`'s reachable tree — `CsvSnapshot`/`CsvRecord`/
`CsvField` are all plain structs) and **not** §3b (no tri-state field in any variant). Root cause,
confirmed empirically: `InsertRecord`'s field is literally named `record`.
`dsl_derive::dsl_variants_codegen`'s generated `to_named_arms` match-arm body for each variant is:

```rust
#match_pattern => {
    let mut record = ::dsl::RecordValue::default();   // <- always named `record`
    #(#to_value_stmts_for_variant)*
    (#keyword.to_string(), record)
}
```

The `let mut record = …` accumulator is unconditionally named `record` — it shadows ANY field of
the variant that also happens to be named `record` (match ergonomics on `&self` bind
`InsertRecord { index, record }`'s `record` as `&CsvRecord` first, then the very next statement
shadows it with an unrelated `RecordValue`). The subsequent `record.fields.insert(#id,
::dsl::DslField::to_value(record))` statement generated for the `record` field then resolves to the
SHADOWING `RecordValue`, not the `&CsvRecord` binding — hence "expected `&_`, found `RecordValue`".

**Confirmed by experiment** (not just inferred): temporarily renamed the field to `csvrec`,
re-ran `cargo check` with the same `#[derive(dsl::DslOps)]` attempt — the mismatched-types error
disappeared completely (only an unrelated, expected `E0425: cannot find value 'record'` remained,
from a leftover unrenamed call site in the same throwaway experiment, itself proof the rename was
the fix). Reverted the rename (renaming would change the Mutation enum's wire shape, out of scope)
and reverted the derive attempt. Hand-rolled `OpText`/`OpBinary` instead.

**This is a new, third failure mode beyond the recon report's §3a/§3b** — worth flagging to a future
closer-level pass: any stdio artifact with a Mutation variant field literally named `record` will
hit this same bug if it ever attempts `#[derive(dsl::DslOps)]`. Grepped the rest of `🗄️stdio` for
other `record:`-named fields — none found, so no other artifact is currently affected. Not fixed at
the framework level (`dsl_derive`'s `🦀️component.rs` is out of this ticket's ownership boundary).

## STEP 2b — hand-rolled implementation (both sides)

Followed §5's template exactly, in `🔺️diff/🦀️component.rs`'s new `HandcraftedDiffCodec` region
(`pub(crate)` primitives + value codecs, reused by `🧬️mutations/🦀️component.rs`):

- **Primitives** (copied verbatim from the gif89a/svg template): `hex_encode`/`hex_decode`,
  `split_top_level` (bracket-depth-aware), `strip_brackets`, `encode_option`/`decode_option`
  (uniform `[0]`=None / `[1,<T>]`=Some(T) tag).
- **Value codecs**: `enc_str`/`dec_str` (hex — `CsvField.value` may legally contain any byte
  including this grammar's own separators `,`/`[`/`]`/space, so hex sidesteps escaping entirely;
  this artifact's own RFC4180 `⚙️engine` codec doesn't need hex since RFC4180 is already its own
  text grammar with its own quoting rule, but THIS diff/op grammar is a different, simpler one),
  `enc_field`/`dec_field` (`[value,quoted]` positional pair), `enc_record`/`dec_record`
  (`[f1,f2,...]`).
- **Diff value codecs**: `enc_field_diff`/`dec_field_diff` (single-letter `V`/`Q` tag pairs, same
  convention as gif89a's `GifFrameDiff`), `enc_record_diff`/`dec_record_diff` (an
  `encode_option`-tagged bracketed list, one entry per field position — the direct grammar
  counterpart of `Option<Vec<Option<CsvFieldDiff>>>`), `enc_records_diff`/`dec_records_diff`
  (`records{[removed];[modified];[added]}` collection triple, same shape as gif89a's
  `enc_collection_triple`, hand-instantiated since csv only has one collection).
- **Top level**: `print_csv_diff`/`parse_csv_diff` — space-separated `name=value` tokens
  (`has-header=0/1`, `records{...}`), absent token = unchanged field.
- `impl protocol::DiffCodec for CsvDiff`: `encode_diff`/`decode_diff` = `print_diff().into_bytes()`
  verbatim, same simplification `WriterDiff`/gif89a/svg use.

In `🧬️mutations/🦀️component.rs`: `enc_csv_snapshot`/`dec_csv_snapshot` (positional
`[schema,has-header,[records...]]`), `print_csv_mutation`/`parse_csv_mutation` (`keyword
arg=value ...`, one match arm per variant, reusing the diff file's `pub(crate)` primitives —
same intra-artifact reuse pattern svg's mutations file uses against its own diff file). `impl
OpText for CsvMutation` + `impl protocol::OpBinary for CsvMutation` (binary = text bytes
verbatim).

Real captured `print_diff` output (from this pilot's own test run, exercising `has_header` +
all three collection-triple sections of `records` at once — via `eprintln!` temporarily added to
the test, run with `-- --nocapture`, then removed):
```
has-header=0 records{[2];[0:[1,[[1,[V:6e65772d61,Q:1]],[1,[V:6e65772d62,Q:0]]]],1:[1,[[1,[V:78]],[1,[V:79]]]]];[2:[[6272616e64205b6e65775d,1]]]}
```
and the reverse direction (`b`→`a`, exercising `removed`/`modified` instead of `added`):
```
has-header=1 records{[2];[0:[1,[[1,[V:6e616d65,Q:0]],[1,[V:6e6f74652c207769746820636f6d6d61,Q:1]]]],1:[1,[[1,[V:61]],[1,[V:62]]]]];[2:[[78,0],[79,0]]]}
```

Real captured `print_op` output (same technique, every `CsvMutation` variant):
```
no-mutation
set-snapshot snapshot=[737464696f2e637376,0,[[[6e65772d61,1],[6e65772d62,0]],[[737461626c65,0]],[[6272616e642d6e6577,1]]]]
set-snapshot snapshot=[737464696f2e637376,0,[[[612c20747269636b79205b76616c75655d,1],[706c61696e,0]]]]
set-has-header has-header=1
set-has-header has-header=0
insert-record index=1 record=[[6e65772c205b747269636b795d,1]]
remove-record index=0
set-field record-index=1 field-index=0 value=6368616e676564 quoted=1
set-field record-index=0 field-index=2 value=776974682c20636f6d6d61205b616e645d20627261636b657473 quoted=0
```
(the last `set-field` line's `value` hex-decodes to `"with, comma [and] brackets"` — proof the
grammar's own reserved separator characters round-trip cleanly inside a hex-encoded value.)

## STEP 3 — tests added

- `handcrafted_diff_codec_tests::diff_codec_text_binary_roundtrip_law` (in `🔺️diff/🦀️component.rs`)
  — exercises `CsvDiff::default()` plus both directions of `between(a, b)` where `a`/`b` differ in
  `has_header` and exercise all three of `records`' removed/modified/added, including field
  values containing the grammar's own reserved characters (`,`, `[`, `]`) to prove hex-encoding
  makes escaping unnecessary. Asserts `!printed.contains('\n')`, `parse_diff(print_diff(x)) == x`,
  `decode_diff(encode_diff(x)) == x`.
- `tests::op_text_binary_roundtrip_law` (in `🧬️mutations/🦀️component.rs`) — exercises every
  `CsvMutation` variant including a `SetSnapshot` payload whose record fields contain the
  grammar's reserved separator characters. Same three assertions as above, for `OpText`/`OpBinary`.

## STEP 4 — verification

Full stdio crate has other sessions actively editing it concurrently (confirmed: unrelated,
in-progress `ifc`/`xlsx`/`ply`/`step` compile errors appeared and disappeared across repeated
`cargo check` runs during this session, and `⚙️engine/🦀️component.rs` plus a txt deserializer
file inside this artifact's own directory tree show unstaged diffs this session never made — a
different concurrent session actively editing files adjacent to, but not overlapping, this
session's scope). Polled rather than chased per the standing "Concurrent Cargo Workspace Churn"
guidance: a `ply` `E0432` compile error (unresolved `STDIO_PLY_DOCUMENT_SCHEMA` import) blocked
`--lib` test-target compilation for ~15 minutes mid-session and cleared on its own once that
concurrent session's edit landed.

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::csv"` → **19/19 passed**, including the
  two new tests (`diff_codec_text_binary_roundtrip_law`, `op_text_binary_roundtrip_law`). Zero
  csv-scoped failures at any point in this session.
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **1032 passed, 1 failed** (out of
  1033 total). The **one** failure is
  `artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::diff::component::handcrafted_diff_codec_tests::diff_codec_text_binary_roundtrip_law`
  — entirely inside the `📕️xlsx` artifact (a different F6 fan-out agent's in-progress work on a
  DIFFERENT artifact this session never touched; confirmed via repeated `cargo check` runs during
  this session that also showed `XlsxOpcDiff: DslField`/`XlsxWorkbookDiff: DslField` errors
  appearing and clearing as that other session iterated). Verified via `grep -c "^test .*
  artifacts::csv.* \.\.\. ok$"` that every single `artifacts::csv::*` test line in this
  whole-crate run reads `ok` — zero csv regressions. Per the ticket's own baseline framing ("count
  only goes up, never down"), 1032 > the recon's 1019 baseline, consistent with other F6 agents'
  work having landed in the interim; the 1 failure is not a regression this session introduced or
  is responsible for fixing (outside `🗿️artifacts/📊️csv/**`).

## STEP 5 — files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — doc comment on `CsvRecordDiff` citing the real derive-blocker error; new `HandcraftedDiffCodec`
  region (primitives, value codecs, diff value codecs, top-level print/parse,
  `impl protocol::DiffCodec for CsvDiff`); new `handcrafted_diff_codec_tests` module with
  `diff_codec_text_binary_roundtrip_law`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — doc comment on `CsvMutation` citing the real derive-macro hygiene-bug error; replaced the
  `serde_json`-backed `OpText`/`OpBinary` stub impls with a hand-rolled grammar
  (`enc_csv_snapshot`/`dec_csv_snapshot`, `print_csv_mutation`/`parse_csv_mutation`); new
  `op_text_binary_roundtrip_law` test.

## Deviations from §5's template

- The Mutation-side hand-roll reason (a derive-macro hygiene bug triggered by a field literally
  named `record`) is not one of the recon report's documented §3a/§3b failure modes — flagged
  above as a new finding for a future closer-level pass, not fixed at the framework level (out of
  this ticket's ownership boundary: `dsl_derive`'s `🦀️component.rs` is a shared file).
- No enum anywhere in this artifact's reachable type tree (`CsvSnapshot`/`CsvRecord`/`CsvField`/
  `CsvFieldDiff`/`CsvRecordDiff`/`CsvRecordsDiff` are all plain structs) — the recon table's §8 row
  guessed "DERIVE probable" for csv; both sides actually hand-roll, for reasons the table's
  enum/tri-state grep heuristic couldn't see (a `Vec`-wrapped Option nesting on the diff side, a
  macro hygiene bug on the mutation side).
- Did not add csv to `POLICY_DIFF_COMPLETENESS_ALLOWLIST` (`📜️script.ts:2304`) — not touched,
  per instructions; the new `DiffCodec` impl should make csv drop out of the
  `dsl-migration/diff-completeness` breach list on its own, same as binary/gif89a/svg did.
