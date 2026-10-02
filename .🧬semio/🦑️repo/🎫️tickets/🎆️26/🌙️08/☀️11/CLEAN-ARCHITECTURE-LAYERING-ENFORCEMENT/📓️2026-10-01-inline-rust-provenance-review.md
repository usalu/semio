# Inline Rust Provenance Review — 2026-10-01

Read-only inspection of the live anchored-inline change. No tests/compiler were launched by this reviewer. Root reported prepatch 11-pass/1-fail with 231 assertions and a successful first native oracle before module-provenance failure; this report does not claim a final green result. Applicable root/Repo instructions remain in effect; source/fixture/schema/test changes were read without altering them.

## Verdict

The new narrow lexical anchor is directionally correct and does not invent Cargo ownership. Six accepted native fixture shapes exercise orphan explicit `.`, nested explicit, nested default after anchor, conventional mounted file, explicit mounted file, and include-file-origin preservation. Two rejection fixtures preserve unanchored orphan context obligations and workspace containment. No concrete new target-base misresolution was found for these six shapes by source reasoning.

One material **existing physical-provenance blind spot is now inherited by inlineBase**: normalizing `segment/..` before no-follow census erases components that the actual Rust/OS path traversal must visit. Add a fail-closed physical-prefix vector before claiming symlink-closed physical authority. Optional inlineBase field/API coupling is also not fully closed. Neither issue was independently reproduced with rustc by this reviewer; the implementation behavior and erased-prefix mechanism are directly visible in source.

## Anchor Semantics and Limits

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:6460` adds optional `inlineBase` to compile-reference facts. Scanner `:6514-6545` derives it only from a source-root inline explicit path, propagating it through explicit child mounts and implicit child module names. Unanchored nested inline paths still need graph contexts. This matters: an explicit path on an inline module nested beneath an unanchored parent is relative to that parent's module directory; it cannot simply re-root to the physical file. Current `modulePath.length === 0` guard keeps that obligation.

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🟦️.ts:42-45` selects this lexical relative base only for `kind === "path"` references with a modulePath. `include!`, `include_str!`, and `include_bytes!` remain file-origin based unless a manifest/generated base is explicitly proven. Scanner's `add` omits inlineBase from includes. Corpus `anchor-keeps-include-file-origin` specifically proves this distinction: mounted child source is `general/specific/leaf.rs`, while included text is `specific/fixture.txt`.

Explicit directory mount references are emitted separately with `directory:true`. A top-level `#[path="."]` therefore contributes its physical current directory without granting a manifest; nested explicit mounts contribute their own directories. Implicit inline module directory names are composed into childBase but are not emitted as physical directory inputs. Native compilation requires traversable intermediate directories when a later file path references them; the accepted nested-default fixture deliberately supplies `.marker` to create that directory.

The nonportable-inline guard rejects absolute Unix mounts, drive-qualified mounts, and backslash mounts. Multiple inline path attributes and inline `cfg_attr` path attributes fail closed; supporting these without conditional/context proof is outside this narrow feature. An explicit cfg gate alone remains all-configurations scanned. Root manifest fallback behavior is untouched: inline base never becomes `manifestPaths`, and each native corpus case still asserts a CARGO_MANIFEST_DIR include fails without manifest provenance.

## Physical Prefix Erasure — Concrete Additional Vector

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:6527` normalizes childBase; `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🟦️.ts:43,47-49` joins/normalizes final targets; `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts:15-34` then lstats only normalized path segments. Source such as:

```rust
#[path="missing/.."] mod anchored {
    #[path="leaf.rs"] mod leaf;
}
```

at `general/source.rs`, with `general/leaf.rs` present but `general/missing` absent, produces canonical directory `general` and leaf `general/leaf.rs` under current logic. Census can accept both despite actual path traversal needing the missing intermediate directory. Equivalent direct path normalization existed previously; new anchored metadata carries it forward.

A symlink intermediate is more serious: `#[path="node_modules/.."]` or a multi-component explicit mount may pass through an ignored symlink directory, then normalize it away. Whole-tree walk excludes ignored directories before lstat, and target census sees only the normalized target. Actual symlink/parent traversal can differ from lexical normalization. Keep raw mount components/ordered base provenance for physical census, and reject any encountered link, missing component, or non-directory *before* resolving parent navigation. Do not solve by rejecting all `..`: legitimate actual cases use `../specific` and deeper neutral helper mounts. Add independent rustc negative/census vectors for absent intermediate and ignored symlink-parent traversal, including nested default-after-anchor.

This is a physical-authority blocker if the task claims complete no-follow compile-input authority; it is not evidence that the six accepted new cases have incorrect target values.

## Public Metadata and Schema Coupling

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️rust-source-direction/🔣️.json:87-90` currently accepts any nonempty inlineBase string on a reference, without requiring path kind, nonempty modulePath, or forbidding manifest/generated base. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🟦️.ts:43` trusts provided inlineBase and takes it ahead of proven graph contexts. Scanner-generated facts satisfy a narrower contract, but the exported API can be fed a manufactured relative inlineBase to bypass an unanchored context obligation. Add explicit coupling/invariants at the reference boundary: inlineBase iff permitted anchored path fact; canonical portable relative locator form; no ambient manifest authority; graph-consistent provenance where both are available. A branded internal proof object or scanner-owned construction can prevent caller metadata from being mistaken for inferred authority, while the language-neutral fixture schema must describe the actual transferable evidence.

Absolute/drive/backslash arbitrary inlineBase are not validated in rustSourceTargets itself (only source-derived mounts are checked). Containment guard checks the eventual normalized target; that protects common `../..` escapes, but does not validate the metadata's original source proof. Add hostile direct API reference vectors and field-combination schema rejections if references are intended to be accepted across an API boundary.

## Corpus/Test Coverage

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction/🟦️.ts:141-165` runs actual rustc dep-info independently, compares expected lexical targets under an every-input direction rule, confirms .rs/.txt targets appear in native dependencies, and runs input census for every accepted case. It also verifies orphan Cargo authority remains unavailable and rejects the two negative cases. The test's 45-second law limit follows the existing native-source oracle pattern. It does not yet cover anchor conditional/duplicate/nonportable rejections, unanchored-parent/explicit-child context preservation, path-prefix erasure, or hostile manually manufactured inlineBase metadata. Those are meaningful additions; implementation-mirroring extra tests are unnecessary.

The assertion that expected native inputs are a subset of rustc dep-info is appropriate for this feature's mounted-source window, but does not prove whole-crate dependency completeness. Directory mount targets are correctly checked by input census, since compiler dep-info generally records files rather than inline mounting directories.

## Report Counters and Error Granularity

Executor `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts:84,91-92` continues dropping all compile references from a source when one unsupported expression throws; a later unresolved target stops all edge/census evaluation for that file. Thus references excludes otherwise parseable dependencies in errored files. Scanner counts represent complete **successfully parsed** reference facts, not every authored compile input. Inline directory facts intentionally increase references and can create extra path edges; files count is unchanged. Preserve typed red failure and report this limitation explicitly, or introduce per-reference problem collection before calling the result a complete typed census.

No production edit made. Required follow-up is bounded evidence closure, preserving the six native semantic proofs and the unanchored-context/Cargo nonauthority tests.
