# F6 — 🟪️stl (standard ascii) — OpText/OpBinary + DiffCodec

**Artifact**: `🟪️stl`, standard `🔖️ascii`, path `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/`

**Verdict**: **HAND-ROLL** on both sides (Diff and Mutation) — NOT the recon table's "DERIVE
(probable)" guess. The derive was attempted for real, compiled cleanly on the first try, and was
then reverted after a real `cargo test` run surfaced a genuine, previously-undocumented `dsl`
framework bug. This report exists specifically to correct the recon table's row 16 guess for future
readers.

## STEP 1 — classification, done for real (not trusted from the table)

`StlDiff`'s field tree has **zero `pub enum` nodes** and **zero `Option<Option<_>>` tri-state
fields** (`solid_name: Option<String>`, `triangles: Option<StlTrianglesDiff>` — both single-layer
`Option`). Same for `StlMutation`'s variant payloads (incl. `SetSnapshot`'s whole `StlSnapshot`).
Neither §3a (enum-in-tree) nor §3b (tri-state) of `f6-recon-report.md`'s documented decision rule
applies — by that rule alone this artifact is DERIVE-eligible, matching the recon table.

Per STEP 1's literal instruction, I added the derives for real and ran `cargo check`:

- `#[derive(dsl::DslRecord)]` on `StlTriangle`, `StlTriangleDiff`, `StlTriangleModified`,
  `StlTriangleAdded`, `StlTrianglesDiff`, `StlSnapshot`.
- `#[derive(dsl::DslDiff)]` on `StlDiff`.
- `#[derive(dsl::DslOps)]` on `StlMutation`.

**All of this compiled with zero errors** (`f64: DslField`/`usize: DslField` and the blanket
`impl<T: DslField, const N: usize> DslField for [T; N]` in
`🧰️framework/…/🗣️dsl/🦀️component.rs` cover `[f64; 3]` and, by nesting the blanket impl through
itself, `[[f64; 3]; 3]` too — `f64: DslField` ⇒ `[f64;3]: DslField` ⇒ `[[f64;3];3]: DslField`).

Then, per STEP 3/4, I wrote the two mandated roundtrip-law tests and ran them for real. Both
**failed** at runtime (not compile time):

```
parse_diff("solid-name=after triangles { removed=[ ] modified=[ index=0 diff { normal=0,0,1
vertices=5,0,0,6,0,0,5,1,0 } ] added=[ index=2 triangle { normal=-1,0,0
vertices=20,0,0,21,0,0,20,1,0 } ] }") failed: tuple expects 3 elements, found 9 at 1:108
```
```
parse_op("set-snapshot snapshot { schema=stdio.stl solid-name=renamed triangles=[ normal=0,0,1
vertices=0,0,0,1,0,0,0,1,0 normal=0,0,1 vertices=10,0,0,11,0,0,10,1,0 normal=0,0,1
vertices=20,0,0,21,0,0,20,1,0 ] }") failed: tuple expects 3 elements, found 9 at 1:113
```

## The real root cause — a third, undocumented derive blocker

Traced to `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️component.rs`:

- **Printer** (`print_shape`, the `(FieldValue::Tuple(items), Shape::Tuple(elem, _))` arm, ~line
  1900): renders EVERY tuple level as a flat, unbracketed comma-join — `sub.render(...)` for each
  item is joined with `,`, with no depth marker distinguishing "end of inner 3-tuple" from
  "continue the outer 3-tuple". `[[1,2,3],[4,5,6],[7,8,9]]` (`Shape::Tuple(Shape::Tuple(Float,3),
  3)`) prints as the indistinguishable-from-flat `"1,2,3,4,5,6,7,8,9"`.
- **Parser** (`parse_shape`, the `Shape::Tuple(elem, len)` arm, ~line 845): loops on commas,
  recursively parsing `elem` for each item, and only checks `items.len() != expected_len` AFTER
  the comma-chain is exhausted — it never stops early. For a nested `Tuple(Tuple(Float,3),3)`, the
  FIRST recursive call (for the outer tuple's "item 1") itself loops on ALL remaining commas
  (nothing bounds it to 3), consuming all 9 available floats, then fails its OWN arity check (`3 !=
  9`) — exactly the `"tuple expects 3 elements, found 9"` error captured above.

This is a real bug in the shared `dsl` grammar engine (nested fixed-arity `Shape::Tuple` has never
been print/parse-round-trip-safe; no test anywhere in the framework's own test suite exercises a
`[T; N]` whose `T` is itself `[U; M]`). It is out of this artifact's ownership boundary to fix
(`🗣️dsl` is a shared framework module — `📜️script.ts`/`glue.rs`/SDK traits/schema/dsl/protocol
modules/`🏪️store` are explicitly off-limits per this ticket's rules). I considered and rejected:

- `#[dsl(coord)]`: only rewrites a *single*-level `[f64;N]` field to `Shape::Coord(3)` (hardcoded
  arity 3, unrelated mechanism); doesn't touch nested-tuple composition and the codegen for
  `FieldKind::Scalar` still calls `DslField::to_value`/`from_value` on the Rust type regardless —
  shape/value would mismatch.
- Restructuring `vertices` to a flatter Rust type (`[f64; 9]` or `Vec<[f64;3]>`): forbidden —
  this ticket's brief is explicit that Snapshot/Diff/Mutation SHAPE must not change, only the
  OpText/OpBinary/DiffCodec codecs.

So: reverted every `dsl` derive/attribute added in STEP 1, and hand-rolled both sides instead,
citing this exact bug in doc comments on `StlTriangle` (`📸️snapshot::component`), `StlDiff`'s
file header (`🔺️diff::component`), and `StlMutation`'s file header (`🧬️mutations::component`).

## STEP 2b — hand-rolled grammar (both sides)

Followed §5's primitive set (`hex_encode`/`hex_decode`/`split_top_level`/`strip_brackets`), added
as `pub(crate)` in `🔺️diff::component.rs` (reused by `🧬️mutations::component.rs`, same
intra-artifact-reuse pattern `svg`'s `SvgMutation` uses over `SvgDiff`'s primitives). The one
addition beyond §5's stock primitives: `enc_vec3`/`dec_vec3` (one `[f64;3]` level, always
`[...]`-bracketed) and `enc_vertices`/`dec_vertices` (the outer `[[f64;3];3]` level, wrapping 3
bracketed `enc_vec3` calls in one more `[...]`) — this explicit bracket-per-level is exactly what
sidesteps the framework bug: `split_top_level`'s bracket-depth-aware comma split recovers nesting
unambiguously where `dsl`'s own flat-comma `Shape::Tuple` printer cannot.

- `StlDiff`'s `DiffCodec`: `solid_name=<hex>` / `triangles{[removed];[modified];[added]}`
  space-separated sparse tokens (unchanged fields omitted), same shape gif89a's hand-roll uses.
- `StlMutation`'s `OpText`/`OpBinary`: `<keyword> arg=value ...` (all args always present, unlike
  the Diff's sparse tokens). `encode_op`/`encode_diff` = the text bytes verbatim (same
  `WriterDiff`/gif89a/svg simplification — satisfies every `DiffCodec`/`OpBinary` law without a
  denser wire format).

## STEP 3 — tests added

- `diff_codec_text_binary_roundtrip_law` (in `🧬️mutations::component`'s existing `mod tests`,
  extending the file per CLAUDE.md rather than creating a new test file/module elsewhere): 3 cases
  (`StlDiff::default()`, `between(sweep_a, sweep_b)`, `between(sweep_b, sweep_a)`) exercising
  `solid_name` plus all three `triangles` triple sections in both directions, incl. the
  doubly-nested `vertices` field. Asserts `!printed.contains('\n')`,
  `parse_diff(print_diff(x)) == x`, `decode_diff(encode_diff(x)) == x`.
- `op_text_binary_roundtrip_law`: every one of the 7 `StlMutation` variants, incl.
  `SetSnapshot`/`InsertTriangle` (struct payloads) and `SetTriangleVertices` (the doubly-nested
  array payload). Same three assertions via `print_op`/`parse_op`/`encode_op`/`decode_op`.

Both tests reuse existing fixtures already in the file (`tri`, `base_snapshot`, `sweep_a`,
`sweep_b`) rather than inventing new ones, per §STEP-3's instruction.

## STEP 4 — verification (real, both runs)

```
cargo test -p semio-s-plugin-stdio --lib "artifacts::stl"
  → 23 passed; 0 failed (was 21 before this session's 2 new tests)

cargo test -p semio-s-plugin-stdio --lib   (whole crate)
  → 1047 passed; 0 failed
```

`cargo check -p semio-s-plugin-stdio --lib` is clean (0 errors) both before and after the
derive-then-revert — no leftover dangling derive attributes, no duplicate trait impls.

No stl-specific new warnings were introduced (`grep`-verified against the check output): the only
stl-adjacent warnings present (`unused import: StlDiff` in `⚙️engine::component.rs`, an
"unnecessary qualification" at `🧬️mutations::component.rs:69`, a "hidden lifetime parameters"
note in `🎹️composer::component.rs`) are all in code this session never touched (pre-existing).

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
  — doc comments only (no derive added — the bug's root-cause citation lives here, on
  `StlTriangle`, since that's the type carrying the doubly-nested field).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — hand-rolled `impl protocol::DiffCodec for StlDiff` (new `HandcraftedDiffCodec` region: 6
  `pub(crate)` primitives + value/diff codecs + top-level print/parse), updated file-header doc
  comment.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — replaced the old `serde_json`-stub `OpText`/`OpBinary` with a hand-rolled grammar (`enc_snapshot`/
  `dec_snapshot` + `print_stl_op`/`parse_stl_op`, reusing `🔺️diff`'s primitives), added
  `op_text_binary_roundtrip_law` and `diff_codec_text_binary_roundtrip_law` tests, updated file-header
  doc comment.
- This report: `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/f6-stl-report.md`.
- Ticket-folder scratch (`.txt`, kept per repo rules): `f6-stl-check1.txt` (derive attempt,
  compiled clean), `f6-stl-test-scoped.txt` (derive attempt, 2 real test failures — the bug
  citation source), `f6-stl-check2.txt`/`f6-stl-check3.txt` (post-revert checks), `f6-stl-test-scoped2.txt`
  (post-revert, 23/23 passed), `f6-stl-full-crate-test.txt` (whole-crate, 1047/0 failed).

No shared files touched: `glue.rs`, `📜️script.ts`, the `dsl`/`protocol`/`schema` framework crates
were all read-only this session (the `dsl` bug is documented, not fixed, per the ownership
boundary). `StlSnapshot`/`StlDiff`/`StlMutation`'s SHAPE (fields, types, variants) is byte-for-byte
unchanged from before this session — only OpText/OpBinary/DiffCodec bodies changed, as scoped.

## Deviations from the brief

1. **Recon table correction**: row 16 (`🟪️stl`/`ascii`) said "DERIVE (probable)". The real
   verdict is HAND-ROLL, for a reason outside the documented §3a/§3b decision rule entirely (a
   third blocker: doubly-nested fixed-size array / `Shape::Tuple`-of-`Shape::Tuple`). Flagging this
   for any future agent that still trusts that table row, and for whoever eventually triages the
   `dsl` framework bug itself (any other artifact with a `[[T; N]; M]`-shaped field — nested
   fixed-arity arrays — will hit the identical failure; `stl` is likely not the only one long-term,
   though a grep of the other 30 standards for `; *[0-9]+\s*\]\s*;` was not run as part of this
   artifact's scope).
2. Time spent on a derive attempt that was ultimately reverted was not wasted per STEP 1's own
   instruction ("classify for real ... don't trust the table blindly") — the derive's clean
   compile plus the subsequent real test failure IS the classification evidence this report cites.
