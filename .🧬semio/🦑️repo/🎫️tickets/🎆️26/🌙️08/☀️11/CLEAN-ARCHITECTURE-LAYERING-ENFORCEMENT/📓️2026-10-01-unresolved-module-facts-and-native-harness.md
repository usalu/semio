# Unresolved Module Facts And Native Harness Review

Read-only on 2026-10-01. No test, compilation or production change. Fixture offset slices were inspected directly; no passing runtime claim.

## Closed Module Facts

The proposed optional unresolved discriminated union is appropriate: `{code: "unsupported-attribute", attributes: string[]}` or `{code: "ambiguous-path", paths: (string|null)[]}`. Schema must close keys and require the associated array. Preserve all authored paths, including undecodable null, and deterministic attribute identities. If both defects exist, define priority or retain a list; do not let an ambiguous path hide unsupported rewriting. The authoritative module graph must skip unresolved mount/context/target grants while keeping facts observable.

`rustMetadataAttributeHead` in discovery/🟦️.ts:8319–8334 currently supports path-only and parenthesized attributes, but returns null for every name-value attribute. `rustMetadataAttributes` at8341 silently returns when head is null. Thus direct/cfg_attr `doc = ...`, `path = ...`, and `unknown = ...` all disappear from flattened items. Extend the head with an explicit name-value form and original value range; do not merely scan the first identifier and call malformed metadata inert. Unknown or malformed ranges must remain unresolved. Existing derive/provider consumers use head.arguments; retain that meaning rather than disguising name-value values as call arguments.

Actual macro_use detection should use flattened item.name === "macro_use". This catches nested cfg_attr attributes while excluding predicate names and string values. Native cases: cfg_attr(macro_use, path="leaf.rs") should not set macroUse solely from its predicate; cfg_attr(any(), macro_use) must set it from the dormant attribute; doc="macro_use" must not. Unknown conditional predicates do not prove attribute rewriting by themselves.

## Live Consumers Outside The Graph

Skipping unresolved facts only in inspectRustModuleGraph is insufficient:

- `🧹️normalization/🧬️mutation/🧾️evidence/🟦️.ts:183–187`: fallback finds a raw out-of-line module fact with pathTarget and grants an existing physical file after canonical graph lookup failed. Add unresolved refusal before this fallback grant; otherwise the rejected mount regains authority.
- `🧹️normalization/🧬️mutation/📐️structural-reachability/🟦️.ts:333–335`: raw module selection establishes a mounted direct leaf. It must require !unresolved.
- Same file:345–350: raw public child/inline-child selection follows child source/type origin. Both must require resolved mount metadata.
- `🧪️tests/⚙️native-source-ownership/🟦️.ts:29` selects raw test module facts; update expected contract to exclude unresolved metadata. Workspace-contract and finite-target-consumption declaration fixtures also describe module facts and need the new optional union.
- Execution's direct graph-facts use at source/🏃️execution/🟦️.ts:118 reads uses for alias detection, not membership, so the new module field does not itself require that code to filter module facts.

This is a bounded library consumer inspection, not a claim that every external repository consumer has been exhaustively checked.

## Raw Fixture Golden Offsets

Direct slices of authored ASCII sources confirm:

- ordinary-template-raw-invocation: templateOffset41 starts include_str, invocationOffset100 starts load and132 starts r#load.
- raw-template-and-builtin: templateOffset43 starts r#include_str, invocationOffset104 starts load and136 starts r#load.
- raw-metavariable-binding: templateOffset43 starts include_str, invocationOffset99 starts load.

Because these sources are ASCII, these indices also equal UTF16 offsets. Golden macro identity remains canonical load while original raw spelling starts at the recorded offset. Keep two invocation rows despite identical definition/template sites; paths input and escape independently appear in native dependencies/output assertions.

## Four Worker Harness And AJV

The core native loop at tests/🧱️rust-source-direction/🟦️.ts:214–231 uses a synchronously incremented shared next index before its first await, isolated row-id directories, Promise.allSettled, and rethrows rejected workers after every worker settles. This avoids simultaneous row reuse and removes temporary output only after all started workers complete. A rejected worker stops that worker but surviving workers continue consuming rows; no fullpass should be reported if any worker rejects. Assert row IDs unique in the closed corpus because mkdirSync isolation relies on that invariant.

The shared AJV report validator at test:41 is synchronous. Calls at58 have no await between validateReport(report) and reading validateReport.errors, so worker interleaving cannot replace the error state during that expression. Sharing this validator is sound for current synchronous use; save errors immediately if future code inserts awaits.

Native core expected dependencies uses a Set to collapse physical inputs while scanner metadata tests retain occurrence facts. This is appropriate: rustc dep-info proves the physical input set, while explicit scanner golden offsets prove individual provenance. Neither substitutes for the other. The unused-macro-authored-input exception intentionally has empty native dependencies while authored static literals remain inventoried; retain an explicit named assertion for that distinction.
