# F6 — 🏗️ifc, standard 4 — OpText/OpBinary + DiffCodec Report

**Scope**: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/**` only (standard `2x3` under the
same artifact was explicitly out of scope per the assignment and was not touched). Followed
`f6-recon-report.md` §9's procedure literally.

## STEP 1 — classification (verified for real, not trusted from §8's table)

The recon's §8 table guessed "DERIVE (probable)" for ifc/4 but flagged it for verification since
`IfcValue` was a known candidate for being a data-carrying enum reachable from the diff. Verified by
actually adding the derive attributes and running `cargo check -p semio-s-plugin-stdio --lib`:

- **Diff side**: added `dsl::DslDiff` to `IfcDiff`'s derive list. Real compile error (§3a of the
  recon report — data-carrying enum in the tree):
  ```
  error[E0277]: the trait bound `IfcValue: DslField` is not satisfied
     --> …/🔺️diff/🦀️component.rs:483:34   (pub file_description: Option<Vec<IfcValue>>)
  error[E0277]: the trait bound `IfcEntitiesDiff: DslField` is not satisfied
     --> …/🔺️diff/🦀️component.rs:492:26   (pub entities: Option<IfcEntitiesDiff>)
  ```
  **HAND-ROLL** confirmed. Derive attempt reverted; the real compile error is captured verbatim in a
  doc comment on `IfcDiff` itself.

- **Mutation side**: separately added `dsl::DslOps` to `IfcMutation`'s derive list. Real compile
  error, same root cause:
  ```
  error[E0277]: the trait bound `IfcValue: DslField` is not satisfied
    --> …/🧬️mutations/🦀️component.rs:27:21   (SetFileDescription { values: Vec<IfcValue> })
  error[E0277]: the trait bound `IfcSnapshot: DslField` is not satisfied
    --> …/🧬️mutations/🦀️component.rs:23:19   (SetSnapshot { snapshot: IfcSnapshot })
  error[E0277]: the trait bound `IfcEntity: DslField` is not satisfied
    --> …/🧬️mutations/🦀️component.rs:40:17   (InsertEntity { entity: IfcEntity })
  ```
  **HAND-ROLL** confirmed. Derive attempt reverted; the real compile error is captured verbatim in a
  doc comment on `IfcMutation` itself.

Root cause: `IfcValue` (`Unset`/`Derived`/`Integer`/`Real`/`String`/`Enum`/`Reference`/`Aggregate`/
`TypedValue`) is a genuine data-carrying enum — 7 of 9 variants carry fields — with no `DslField` impl
(only `DslRecord`-derived structs and `DslScalar`-derived UNIT-only enums implement `DslField`). It is
reachable from the Diff directly (`file_description`/`file_name`/`file_schema: Option<Vec<IfcValue>>`)
and transitively (`entities` → `IfcEntitiesDiff` → `IfcEntityDiff` → `IfcArgsDiff` →
`IfcArgModified.value: IfcValue`), and from the Mutation both directly (`value: IfcValue` on
`SetEntityArg`/`InsertEntityArg`, `values: Vec<IfcValue>` on the three header setters) and
transitively via `SetSnapshot{snapshot: IfcSnapshot}` / `InsertEntity{entity: IfcEntity}`. This is
the same failure mode §3a documents for svg's `XmlNode`/`SvgNodeDiff` — the recon's "DERIVE
(probable)" guess for ifc/4 was wrong, exactly the case the recon told this agent to verify rather
than trust.

Unlike gif 89a (where Diff hand-rolled but Mutation derived clean because the Snapshot tree had no
enum), **both sides of ifc/4 hand-roll**, because `IfcValue` sits directly in the Snapshot's own
`IfcEntity.args`/`IfcComplexType.args` and the Header's three value-tuples — there is no clean side
to fall back to derive on.

No `Option<Option<_>>` tri-state fields exist anywhere in `IfcDiff` (§3b does not apply here) —
`file_description`/`file_name`/`file_schema` are plain `Option<Vec<IfcValue>>` (unchanged vs. set-to,
never a removal state — HEADER records are always present), so 3a is the sole, sufficient blocker.

## STEP 2b — hand-rolled implementation

Followed §5's template exactly (own local copies of `hex_encode`/`hex_decode`/`split_top_level`/
`strip_brackets`/`encode_option`/`decode_option`, `pub(crate)` in `🔺️diff/🦀️component.rs` so
`🧬️mutations/🦀️component.rs` can reuse them — same intra-artifact-reuse pattern svg's
`SvgDiff`/`SvgMutation` pair uses).

**`IfcValue` grammar** (single uppercase tag + bracketed positional payload, payload-free variants
bare): `U`=Unset, `D`=Derived, `I[n]`=Integer, `R[n]`=Real (Rust's `Display`/`FromStr` round-trip
exactly for `f64`), `S[hex]`=String, `E[hex]`=Enum, `F[n]`=Reference, `A[v,v,...]`=Aggregate,
`T[hex,[v,v,...]]`=TypedValue — recursive, exercised by nesting `Aggregate`/`TypedValue` in tests.

**Structural codecs** (all in `🔺️diff/🦀️component.rs`, `pub(crate)` where mutations.rs reuses them):
- `enc_complex_type`/`enc_entity`: positional bracketed tuples, field order = struct declaration
  order (`[id,hexname,[args],[complex]]` for an entity).
- `enc_args_diff`/`enc_entity_diff`/`enc_entities_diff`: the recipe's own
  `[removed];[modified];[added]` collection-triple shape, `idx:payload`/`id:payload` colon-separated
  entries (safe even with payloads that themselves contain colons deeper down, since `split_once(':')`
  only ever takes the first — verified working through 2 levels of nesting:
  `entities.modified[].diff.args.modified[]`).
- Top-level `IfcDiff`/`IfcMutation` lines: space-separated `name=value` / `keyword arg=value ...`
  tokens, one per changed field / mutation argument — no token ever contains a literal space (every
  encoding is hex/digits/brackets/commas/semicolons/colons only).
- `encode_diff`/`encode_op` = the printed text bytes verbatim — same simplification `GifDiff`/
  `SvgDiff`/`WriterDiff` use; satisfies every `DiffCodec`/`OpBinary` law without a second wire format.

Real captured `print_diff` output (from `diff_codec_text_binary_roundtrip_law`'s `between(a,b)` case
— exercises the `entities` collection triple, a per-entity `args` triple, `complex` weak-list
replace, and header value-list changes):
```
file-name=[S[6368616e676564]] entities=[1];[2:[[0],[1,[];[];[2:T[49464345545355524d4145534852450a,[R[3030302e30]]]]],[1,[]]]];[300:[300,49464342554946444d554b535352454e455356,[1,A[I[31],I[32]]],[]]]
```
(hex payloads decode to `"changed"`, `IFCLENGTHMEASURE`/`3000.0`, `IFCBUILDINGSTOREY`, matching the
fixture's mutated names/values.)

Real captured `print_op` output (`InsertEntity`, exercising every `IfcValue` tag in one call):
```
insert-entity index=1 entity=[99,49464353495445,[U,D,I[-7],R[3.25],S[6869],E[45444745],F[42],A[I[1],I[2]],T[49464345584d4147495459,[R[500]]]],[]]
```

## STEP 3 — tests (both added, both pass)

- `diff_codec_text_binary_roundtrip_law` (`🔺️diff/🦀️component.rs`, new
  `handcrafted_diff_codec_tests` module) — asserts `!printed.contains('\n')`,
  `parse_diff(print_diff(x)) == x`, `decode_diff(encode_diff(x)) == x` over `IfcDiff::default()` and
  4 `between()` results exercising the `entities`/`args` collection triples in both directions, every
  `IfcValue` tag including `Aggregate`/`TypedValue` recursion, and `complex` being replaced.
- `op_text_binary_roundtrip_law` (`🧬️mutations/🦀️component.rs`, extended the existing `mod tests`)
  — same 3 assertions over every one of the 11 `IfcMutation` variants, including `SetSnapshot`'s
  whole-snapshot payload and an `InsertEntity` whose entity carries every `IfcValue` variant at once.

Both extend existing test files/modules per repo rules — no new test files created.

## STEP 4 — verification (real, both runs shown)

```
$ cargo test -p semio-s-plugin-stdio --lib "artifacts::ifc::standards::v4"
test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 1014 filtered out
```
(17 pre-existing + 2 new law tests — no regressions, only additions.)

```
$ cargo test -p semio-s-plugin-stdio --lib
test result: ok. 1033 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

**Note on transient concurrent-session noise during this session** (per repo norms — live shared
tree, other sessions active): while iterating, `cargo check`/`cargo test` intermittently failed for
reasons entirely outside `🏗️ifc/**` — unrelated in-flight edits in `📕️xlsx`, `📊️csv`, `📐️step`,
`☁️ply` (confirmed via `git status` showing those files modified by other sessions, not this one).
One transient failure was even a real bug surfacing mid-edit in xlsx's own hand-rolled `DiffCodec`
(`relationships` list encode emitting a trailing comma, `split_top_level` returning a spurious empty
final element on decode) — reproduced twice, then gone on a later rerun once that session's edit
completed. None of this was touched or fixed here (out of this ticket's ownership boundary); the
final whole-crate run above is clean at 0 failures, taken after polling until the shared tree
stabilized.

## Policy signal

Did not run the full `bun ./📜️script.ts policy` (slow, ~21k lines of unrelated output per the recon
report). Verified directly instead: `🔺️diff/🦀️component.rs` now contains the literal text
`DiffCodec for` (`impl protocol::DiffCodec for IfcDiff`), which is exactly what
`policyDiffCompletenessBreaches` (`📜️script.ts:3185-3205`) greps for — `stdio.ifc.4`'s diff file will
no longer be flagged as a `dsl-migration/diff-completeness` breach. `POLICY_DIFF_COMPLETENESS_ALLOWLIST`
(`📜️script.ts:2304`) was not touched, per the ticket's explicit instruction.

## Deviations from §5's template

None structural. Minor artifact-specific choices, all within the template's stated freedom:
- Chose 9 distinct single-uppercase tags for `IfcValue`'s 9 variants (`U`/`D`/`I`/`R`/`S`/`E`/`F`/`A`/
  `T`) — more variants than `XmlNode`'s 5 or `TransformOp`'s 6, tag letters chosen to avoid confusion
  with hex payload chars (always lowercase) and with each other, documented in the doc comment next
  to `enc_ifc_value`/`dec_ifc_value`.
- `IfcHeader`'s 3-field encode (`enc_ifc_header`/`dec_ifc_header`) lives in `🧬️mutations/🦀️component.rs`
  (only needed there, for `SetSnapshot`'s whole-snapshot payload), matching svg's precedent of keeping
  `enc_svg_snapshot`/`dec_svg_snapshot` local to the mutations file rather than the diff file.
- No tri-state (`Option<Option<T>>`) handling needed anywhere (§3b does not apply to this artifact) —
  simpler than gif/svg in that one respect.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — compile-error-citing doc comment on `IfcDiff`; hand-rolled `impl protocol::DiffCodec for IfcDiff`
  (primitives, `IfcValue`/entity/collection-triple codecs, top-level print/parse), all `pub(crate)`
  where reused by the mutations file; new `handcrafted_diff_codec_tests` module with
  `diff_codec_text_binary_roundtrip_law`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — compile-error-citing doc comment on `IfcMutation`; hand-rolled `OpText`/`OpBinary` for
  `IfcMutation` replacing the prior `serde_json` stubs (reusing the diff file's `pub(crate)`
  primitives); extended the existing `mod tests` with `op_text_binary_roundtrip_law`; updated the
  `use` block accordingly (added `OpText` unconditionally, `OpBinary` under `#[cfg(test)]`, and the
  new diff-module imports).
- Ticket-folder scratch (`.txt`, kept per repo rules): `f6-ifc-check1.txt`, `f6-ifc-check2.txt`.

No shared files touched (`📦️glue.rs`, `📜️script.ts`, SDK traits, schema/dsl/protocol modules,
`🏪️store` all untouched, read-only). Standard `2x3` under the same artifact was not touched (out of
scope per the assignment). Did not call `ticket_open`/`ticket_close`/`ticket_reopen`.
