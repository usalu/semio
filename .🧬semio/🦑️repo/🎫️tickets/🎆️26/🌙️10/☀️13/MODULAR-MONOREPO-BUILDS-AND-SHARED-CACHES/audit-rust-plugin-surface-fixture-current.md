# Independent Surface Fixture Packaging Audit

## Scoped Verdict

HOLD on complete packaging correctness: the authored fixture has an additional missing direct first-party dependency identified below. Exact Source preservation and the recorded physical closure sizes check out. Rust remains authored/uncompiled; this audit does not certify compilation, type/privacy correctness, runtime semantics, Native/compiler/SDK readiness, publication, global resolution or task completion. The existing eleven active old-API mismatches remain genuine blockers.

Input report `rust-plugin-surface-fixture-current.md` is 149825 bytes, SHA256 `f2f59aea6a44ca14358da2671029d034b77c716187885cbfe34df8b122ec4f19`. Read the design and API-frontier documents. Independently read and hash every one of its 678 identity-table entries: all bytes and SHA256 values matched at the observation epoch. No drift was observed. Concurrent subsequent edits require a refreshed audit.

## Concrete Findings

**P1 — Missing canonical JSON owner dependency in the new fixture.** `🧪️tests/🪟️surface-fixture/📦️packages/🦀️rust/Cargo.toml` has no `semio-framework-pack-json` dependency. Its actual root supplies no `semio_framework_pack_json` alias. Privately mounted `🧪️testing/🧬️mutation-fixtures/🪟️surface/🧬️mutations/🦀️.rs` and its `📝️set-surface-count/🦀️.rs` explicitly contain `#[canonical_json(owner = semio_framework_pack_json)]`. The actual defining derive implementation, `🧰️framework/🔨️modules/🌱️value/✨️derive/🧵️canonical/🦀️.rs`, parses that owner at lines 117–121 and emits owner-qualified trait paths (including its generated impl and canonical methods). A transitive pack-json dependency does not supply a direct extern-prelude binding in this crate. Thus the 29-edge parity table validates the declared subset but misses a concrete consumed macro-expansion owner. This is a static Source/dependency finding, not a compiler diagnostic. Correct the actual manifest, local lock entry and neutral dependency obligations, then refresh bounded evidence; no legacy alias is required.

**P3 — Overbroad scalar-field wording.** The report says scalar count/diff fields remain crate-private. `SurfaceSnapshot.count` and `SurfaceDiff.count` retain their restricted visibility, but `SetSurfaceCount.value` is literally `pub value: i32` and becomes externally reachable through the newly public mutation payload type. This is codec data, not mutable execution custody, and is not evidence of authority widening. Narrow the statement to the snapshot/diff fields rather than implying every scalar codec field is private.

## Independent Preservation Evidence

The permanent neutral census records seven precut working-tree Source identities, 173 function records, 22 named laws and 186 literal assertion counts; these are the claimed working-tree baseline, not HEAD. Independently located and hashed exact current function-item byte spans for all 173 records at their recorded defining targets: every hash matched, including the relocated metadata law. The lookup enumerated actual function-name starts and candidate item-ending byte positions and compared SHA256, without executing Rust or trusting generated pass summaries. This independently confirms current bytes against the recorded baseline digests; it does not reconstruct or independently attest the historical capture provenance of precut full files. Repeated names were matched by exact digest, not name alone. No function-body rewrite was found.

The metadata law retains its recorded exact SHA256 `3923de5b52f0998d27a8d7e06e9b24ee56b01427a16e0fd05cb3da90de6199d6` and 35-assertion census obligation. Its new same-depth Source mount is a private `#[cfg(test)] mod private_child_metadata_laws` in composition-publication. The actual private input remains `pub(crate) struct PrivateChildPublicationInputParts` in publication's `🤝️group/🪆️child/🦀️.rs:35`. Root aliases are private cfg-test imports, not new public custody exports. The other 21 laws remain in the fixture closure. The 186 census count is a preserved Source obligation, not 186 executed Rust assertions.

Independently followed literal `#[path]` and literal `include!` edges from each actual root, conservatively including cfg branches: membership sets exactly equal the supplied membership records. Independently summed physical bytes and nonblank lines:

|Root|Files|Bytes|Nonblank|
|---|---:|---:|---:|
|Fixture|8|142554|1705|
|Proof owner|16|205027|2626|
|Publication|5|141596|1547|
|Facade|198|3272435|37526|

The first three remain below unchanged 32-file/262144-byte/4000-line caps. The facade remains oversized. Imported dependency Source and macro expansion are outside these physical closure sums. The missing generated-owner dependency finding illustrates why these sums alone do not prove closure completeness at the compiler level.

The actual fixture root privately mounts the original surface, wire, job-close and no-state Sources and publicly reexports only `surface::SurfaceMutation`. Its four public DTO declarations/import bindings expose codec data; Editor/Viewer/MediaOwner and natural cursor remain private. Actual facade aggregate replaces its surface path mount with `pub extern crate semio_framework_plugin_surface_fixture as surface`. Fixture has no facade dependency; facade's fixture dependency is dev-only. No fixture-to-facade cycle is introduced by that pair. No exhaustive compiler resolution or whole-workspace cycle claim is made.

The proof macro has one implementation at decl-app's `📣️proofs/🦀️.rs`, mounted by that type owner; facade `📣️macros/📜️wp2-declaration/🦀️.rs` reexports the canonical macro. Exactly six canonical `$crate::tool_contract::ArtifactBoundedFirstStepProof` references replace the old root references. Reversing those six substitutions and hashing the actual macro item yields `933cca2ca22439de997c15334a5d28a2652cd354b1ec5ab02bca3ed022ec7986`, exactly the recorded precut digest. No copied macro implementation or public authority constructor was introduced in this cut.

## Execution And Remaining Frontier

Read retained logs as historical evidence only: reported selected Source prototype/release 12 tests/4643 expectations and strict serial successes are neutral Source checks. This auditor did not rerun tests. Those checks cannot discharge the eleven active API mismatches: dispatch_typed twice, consume_media once, viewer handle_action once, MediaOwner close_step once and six media exports. In particular the missing StepContext/loan, mutable Option and changed Result<Option<Media>,OriginalTurnFailure> obligations remain present in actual Source. Preserve refusal/error specificity and paid original-owner retirement when adapting them. The new dependency finding is additional to those eleven and is not an old-call adaptation.

Only ordinary reads, text/JSON inspection and hashing were performed. No Bun/test execution, Cargo/rustc/metadata, Native/provider/browser/network or OS sampling; no production changes, git mutation, worktree, script or AGENTS edits. This report is the sole written audit output. Parent retains ticket lifecycle and subsequent correction ownership.
