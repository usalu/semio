# Binary OpText and Cartesian Repair Source Audit

Captured: 2026-09-09T15:47:21+02:00

This is a read-only source audit. No Cargo command, build, test, formatter, or production-source edit was run. The Binary native retry remains pending, so these findings are not runtime acceptance.

## Source conclusion

The two repairs are coherent at source level. One targeted regression remains absent: the operation-text suite covers canonical positive round trips, but does not assert rejection of the old aggregate keyword or an ambiguous prefix.

### Canonical leaf dispatch

In `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`, `BinaryMutation::parse_op` now obtains each variant's `RecordSpec`, matches only its `spec.keyword` followed by either end of line or one space, parses with that leaf spec, and calls `from_named_record` with the aggregate table key.

For `BinaryMutation::ReplaceByteRange`, the aggregate `DslVariants` key is `replace-byte-range`, while the leaf at `…/🧬️mutations/✂️replace-byte-range/🦀️.rs` declares `#[dsl(keyword = "splice")]`. The derive implementation in `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs` delegates a newtype variant's spec to the leaf but reconstructs through the aggregate key. Therefore the repaired path is:

`splice …` → leaf `RecordSpec` → aggregate key `replace-byte-range` → `BinaryMutation::ReplaceByteRange`.

The exact delimiter check rejects `splice-extra …`, `spliceanything …`, and the obsolete internal `replace-byte-range …` before parsing. A bare `splice` reaches the leaf parser and must still satisfy its required fields; it is not accepted as a successful mutation merely by dispatch.

### Cartesian law and strict domain

`…/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` now evaluates all 16 ordered pairs, calls `absorb` only after the second sequential diff applies, and asserts exactly 15 valid compositions and one rejected composition. The excluded pair is the intended domain failure: `SetSnapshot([9, 9])` followed by the representative splice at offset 1 with removal length 2. The latter cannot apply to a two-byte snapshot.

This preserves strict production behavior in `…/🧬️schema/🔺️diff/🦀️.rs`: `BinaryDiff::apply` still validates the range before applying, and `BinaryDiff::absorb` remains an unconditional call to `absorb_splices`. The existing diff unit law still asserts an invalid splice fails without mutating the base. The Cartesian test excludes only a composition outside that precondition; it does not relax `apply` or `absorb`.

## Concrete missing regression

Add direct negative `parse_op` assertions for both `replace-byte-range …` and `splice-extra …`. The current `op_text_binary_roundtrip_law` proves only printed canonical forms for the four demo variants. Those two checks would lock the intended distinction between the aggregate reconstruction key and accepted wire keyword, and protect the exact prefix boundary.

## Runtime status

The earlier Binary native suite had two failures. This audit does not establish that either repair passes at runtime. Acceptance requires the pending Binary retry to finish with terminal evidence.
