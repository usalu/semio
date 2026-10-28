# TypeScript Layout Diff Algebra

The TypeScript implementation now shares one immutable central applier for ordered keyed deltas, sparse field setters, positional guide/style-run rows, data-field dictionaries, and composed-child identity. Snapshot differencing, inverse diffs, ordered replay, and composition all pass through this authority. Background text editing uses an owner-supplied interface. The public Layout diff entry exports the runtime and its owned types. The owned verification router admits `verify layout-diff-algebra`.

## Verification

Executed through TypeScript compiler API and the existing installed test libraries, without shelling out around the repo-only shell rule:

- 33 portable native-JSON cases: 11 independent RFC 6902 diff vectors and 22 structural vectors; apply, inverse, transported inverse, replay, and composition matched the oracle.
- All 45 existing mutation golden diffs and their inverses matched their complete before/after snapshots.
- 500 deterministic fast-check properties proved ordered story collection diffs and exact binary64 restoration (including every possible category of IEEE words).
- Eight malformed event cases were refused without changing the canonical input.
- Strict TypeScript compiler API check reports zero diagnostics across the entire new applier/test dependency closure.
- Independent protobufjs parsing found duplicate messages in the existing diff Protobuf facet. The facet is now deduplicated; nullable setter wrappers preserve omitted/cleared/empty string as three distinct states, proven by encode/decode.

The initial cross-realm direct fixture-loading probe refused plain records before reaching semantic application; rerunning fixtures within their proper module realm succeeded. Stale module caches also required refreshing the concurrent fixture edits. These were harness issues, not skipped semantic failures. Native Rust compilation and app runtime checks remain unverified until the repo build runner is available.

## Sparse Schema Alignment

JSON Schema and TypeScript parsers now distinguish missing optional setters from explicit null. Native double-Option fields follow the same semantics. See sparse-fixtures.md for the reviewed existing golden adjustments. No compatibility reader or migration was added.

## Positional Parity Repair

Native apply_positional previously accepted repeated final indices by taking the last supplied row while TypeScript rejected them. Added two language-neutral refusal witnesses and native/TypeScript routes before updating native admission to reject duplicate indices atomically. Native lookup now indexes supplied rows in a BTreeMap rather than rescanning every supplied row for each final position. JSON-schema descriptions state the unique-position contract.

Actual rerun: 37 portable,45golden,500 generated,10 refusals pass through the TypeScript production APIs. Full strict TypeScript dependency closure including drawing composition and the new IDML intrinsic parser reports zero diagnostics. Independent Rust grammar accepts positional implementation and both native regression files; native tests remain unrun.
