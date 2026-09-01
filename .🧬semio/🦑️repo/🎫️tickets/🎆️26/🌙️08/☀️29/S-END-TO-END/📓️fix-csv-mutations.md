# Fix: csv mutations `🦀️.rs` botched-splice repair

File: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`

The corruption was one consistent pattern repeated at every damaged site: a `return Vec::new()` /
`Vec::new()` fragment from some other edit got spliced into positions where either (a) a real
`CsvMutation::…` value belonged, (b) a match-arm pattern belonged, or (c) a `mod` declaration was
missing entirely. All six damaged categories:

## 1. Missing `mod set_snapshot` declaration (the six E0433s at :45/:78/:94/:138/:155/:287)

The `//#region 🔖️Leaves` block declared `set_has_header`, `insert_record`, `remove_record`, and
`set_field` as `#[path = "…/🦀️.rs"] pub mod …;`, but not `set_snapshot`, even though every other
leaf-newtype variant needs its module in scope. A comment above the block claimed `set_snapshot` was
"declared by 📦️glue.rs" and deliberately not re-declared here — but no `glue.rs` exists anywhere
in this artifact's tree, and the leaf folder `📄set-snapshot/` (containing `🦀️.rs`, `🔺️diff/`,
`↩️inverse/`, etc.) sits right next to the other leaf folders. Compared against 8 sibling
already-migrated artifacts (tiff, jpg, xlsx, docx, pptx, ifc, step) that all declare
`#[path = "…set-snapshot/🦀️.rs"] pub mod set_snapshot;` in the same Leaves block, in enum-declaration
order (first). The comment was stale/wrong — removed it and added the real declaration:

```rust
#[path = "📄set-snapshot/🦀️.rs"]
pub mod set_snapshot;
```

## 2. `encode_op`'s match on `self` (the line-252 syntax error)

```rust
match self {
    return Vec::new() => {   // was: illegal expression in pattern position
        w.write_u8(0);
    }
    CsvMutation::SetSnapshot(...) => { w.write_u8(1); ... }
    ...
```
Ordinal 1 is already `SetSnapshot`, so this bogus arm was never meant to encode a real variant —
`NoMutation` (which used to be ordinal 0, per the doc comment in the file's own header: "`NoMutation`
was dropped: the derive requires every variant to wrap exactly one leaf payload") no longer exists
as a `CsvMutation` value. Since the match is now exhaustively over the 5 remaining variants
(ordinals 1–5), this arm doesn't correspond to anything and was deleted outright.

## 3. `decode_op`'s ordinal-0 arm (part of the same corruption, not separately flagged by cargo)

```rust
let mutation = match ordinal {
    0 => return Vec::new(),   // was: type mismatch, Result<CsvMutation, ProtocolError> vs Vec<_>
    1 => CsvMutation::SetSnapshot(...),
    ...
```
Same reasoning as above — ordinal 0 has no corresponding `CsvMutation` value anymore, so decoding it
should fail like any other unknown ordinal. Deleted the `0 =>` arm; ordinal 0 now falls through to
the existing `other => Err(ProtocolError::Malformed { … "unknown ordinal {other}" … })` catch-all.

## 4. `parse_csv_mutation`'s `"no-mutation"` special case (the E0308 at :148)

```rust
if line == "no-mutation" {
    return Ok(return Vec::new());   // Result<CsvMutation, String> vs Vec<_>
}
```
Same root cause: no `CsvMutation` value represents "no mutation" anymore. Deleted the whole
`if` block; `"no-mutation"` as input now falls through to the normal `keyword` match and hits the
existing `other => Err(format!("csv mutation: unknown keyword {other:?}"))` arm, which is honest
(there is genuinely no such op anymore).

## 5. `agg_inverse`'s two "index out of range" arms (not flagged by cargo — this shape happens to
   typecheck because `return X` has the never-type, but it's the same splice pattern and just as
   broken conceptually)

```rust
None => vec![return Vec::new()],
```
in both the `RemoveRecord` and `SetField` arms of `agg_inverse` (`-> Vec<CsvMutation>`). Functionally
this early-returns an empty `Vec` from the whole function — which happens to be the same *value* a
plain `None => Vec::new()` would produce as that match arm's result — but wrapping a `return` inside
a `vec![]` literal is exactly the splice signature described in the task, and is definitely not
intended code. Simplified both arms to `None => Vec::new(),`.

## 6. Five spliced test-vector list literals

In `mutation_diff_law`, `inverse_law`, `op_text_binary_roundtrip_law`, `ops_grammar_conformance_law`,
and `kinds_match_enum_and_catalog`, the `vec![...]` / array literal listing one sample per
`CsvMutation` variant had a bogus extra leading element:

```rust
let variants = vec![
    return Vec::new(),                              // bogus — deleted
    CsvMutation::SetSnapshot(...),
    CsvMutation::SetHasHeader(...),
    CsvMutation::InsertRecord(...),
    CsvMutation::RemoveRecord(...),
    CsvMutation::SetField(...),
];
```
Each of these lists, once the bogus line is removed, has exactly 5 elements/variant-samples — matching
the 5 real `CsvMutation` variants (`kinds_match_enum_and_catalog`'s `samples` array is asserted
directly against `KINDS`, which has exactly 5 entries, confirming this count). There is no
replacement value for the deleted line (no `NoMutation`-equivalent exists), so it was deleted, not
substituted.

## Verification

`cargo check -p semio-s-plugin-stdio --target wasm32-wasip2 --lib --message-format=short 2>&1 | grep -E '📊️csv|could not compile'`
— see status appended by the closing agent turn. Only an `E0046 missing DESCRIPTORS, descriptor`
line for this csv file is expected/acceptable per the task brief (that's being handled by another
agent); no other error should mention this file.
