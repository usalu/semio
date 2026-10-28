# Native IFC4x3 Source IO Implementation

## Source Route

The new standard owns `🚪️io/step.rs`, mounted as `semio_s_artifact_stdio_ifc_4x3::io`. It directly shares the existing IFC graph model and native graph pack implementation, and uses the shared Part21 grammar/writer behind that interface. It never routes IFC4X3_ADD2 source through the IFC4 source codec, which correctly rejects that schema.

The identities remain distinct:

| Identity | Purpose |
| --- | --- |
| `IFC4X3_ADD2` | Natural source FILE_SCHEMA identifier |
| `stdio.ifc` | Shared native graph schema and graph pack envelope |
| `s.stdio.ifc@4x3/*` | Framework dialect and retained host document envelope |

Public source functions are `parse_ifc4x3_step`, `write_ifc4x3_step`, their `_controlled` variants with caller-owned progress/cancellation callbacks, and `_io` bridges accepting the original framework `IoRunControl`. Refusals carry a stable code, message and source byte offset. This source layer does not assert semantic EXPRESS or MVD conformance; imported incomplete models remain editable.

Import scans complete bounded physical records, respecting strings, doubled apostrophes, binary literals, comments and nested argument lists. It requires mandatory section framing, exactly ordered FILE_DESCRIPTION/FILE_NAME/FILE_SCHEMA records, correct header field types and IFC4X3_ADD2, final terminator and no trailing records. Each individual instance is parsed with the shared grammar inside an isolated exchange frame, preserving its complete typed/complex graph. Decimal reals are checked against finite binary64 before the existing model bridge can project them. Positive unique u64 identities, nested references, finite reals, canonical binary content and valid entity/enum/typed names are checked before publishing a snapshot.

Export performs the same graph checks and uses the shared writer on individual instance records. It retains headers, order, typed values, complex constituents, exact integer identities, Binary content and negative-zero binary64 sign. The overall source admission bound is 256 MiB, physical records are bounded to 8 MiB, and value nesting is bounded to 64 levels with framing allowance. These are explicit resource bounds rather than invented semantic restrictions.

## Actual Framework Registration

`Ifc4x3Composer` implements the existing `ArtifactComposition` and `ArtifactComposer` contracts. It accepts exactly one known-dialect source: natural UTF-8 STEP text, UTF-8 text artifact pack, or the shared native IFC graph pack. A native-dialect Binary payload first tries its graph pack representation and then valid UTF-8 source. The resulting FILE_SCHEMA is checked in either case; a foreign IFC4 pack cannot silently become a 4x3 snapshot. `Ifc4x3FromTxt` supplies the typed UTF-8 import boundary. `Ifc4x3ToTxt` implements `ArtifactSerializer` and produces a text artifact containing natural STEP source.

`entries()` contains the IFC4x3 composer and the real IFC4x3-to-text serializer entry. The root IFC aggregate `io_registry::{entries,register}` and the root IFC `declaration()` include both. The artifact definition declares exact native-write and directed foreign-export capabilities plus a source dialect. The host codec has its own exact `codec`/`codec-extension` capability for `s.stdio.ifc@4x3/*`; it does not conflate that envelope with the shared graph schema. A source layer is deliberately not masqueraded as a `NativeCodecFactory`: that factory contract constructs native document pack codecs, and the shared graph/host codec is registered separately.

An integration audit found that foreign-write composers previously claimed the target dialect exclusively, colliding with the TXT artifact. The parent authorized the smallest general capability fix. `composer_identity_claims` now preserves existing same-owner output dialect authority and gives a foreign exporter exact `composition-route` authority from each owned source to its target. The claim uses canonical UTF-8 byte length framing, validated by the same central claim constructor; a definition can claim only its own source artifact. Both declaration-builder admission and final capability requirement collection call that helper. The stdio schema validator accepts the namespace, and the IFC exporter has the exact route capability `17:s.stdio.ifc@4x3/*19:s.stdio.txt@utf-8/*`. It is included normally in the declaration, without a hidden imperative fallback. See `route-capabilities.md` for general files, tests and validation limits.

All `.ifc` physical document representation flags are now text (`is_binary: false`). The format catalog selects one representation by equal extension/MIME claims and lexical identity; leaving the existing IFC2x3 entry marked binary would continue misclassifying natural `.ifc` source even after adding IFC4x3's text flag. This correction changes metadata for the actual source representation, not native graph pack encoding.

## Tests and Evidence

The language-neutral `🚪️io/🧪️tests/native.feature` and native Rust tests were written before the implementation. Five native tests define the required contracts:

- All three retained official informative examples round-trip complete graph data: triangulated 35 entities, alignment 3,161 entities, structural 188 entities.
- Shared native graph pack round-trips retain FILE_SCHEMA IFC4X3_ADD2 and graph schema stdio.ifc. This directly tests that the IFC4-scoped native pack implementation is graph-generic.
- Strict rejection of duplicate/zero ids, missing nested references, non-finite reals, foreign schemas, extra headers, missing terminators and trailing records.
- Complex records, ids beyond 2^53, Binary content and negative-zero `to_bits()` preservation; caller cancellation before publication; registered import/export dialects and native-pack composer path.
- Actual erased entry invocation from a packed TxtSnapshot containing natural STEP to a native graph pack, then the actual exporter from that pack to a packed TxtSnapshot containing regenerated natural STEP. FILE_SCHEMA is checked after pack decoding. Foreign native packs with IFC4 FILE_SCHEMA are rejected by both import and export entries.

**Passed:** rustfmt parsing/format checks on `step.rs` and `native.rs`; Cargo `metadata --no-deps` resolves the 4x3 package with its added internal async dependency. All created source paths stay below the 256-character limit. Definition JSON parses and the new runtime capability claims and source representation flags are present.

**Passed regression:** the existing TypeScript STEP suite over the same three fixture inputs: 13 tests, 30 assertions. This checks the shared fixture contracts and the TypeScript implementation, not execution of the new Rust implementation.

**Independent evidence already produced by the concurrent exchange worker:** IfcOpenShell 0.8.3 reports IFC4X3_ADD2 and exactly 35/3,161/188 entities for triangulated/alignment/structural inputs and produces canonical STEP plus semantic summaries. The retained production fixture bytes/provenance remain the source of truth. The test-only third-party executable is `C:/Users/Ueli/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/python.exe`, with temporary package under ticket `🗑️generated/ifc083`; reusable input is `ifc-oracle.py`. Reports currently live in `🗑️generated/ifc-results` and parent cleanup removes them after the ticket's evidence is summarized in Markdown. These baseline oracle reports do **not** establish that unexecuted Rust output matches the third party.

**Unverified:** native Rust tests and production type checking cannot execute through the current repository compiler ingress. The previously documented fallback stops at `Original compiler ingress requires the repository launcher and authentic enforcing store`; no configuration/enforcement bypass was attempted. Cached `repo test semio-s-artifact-stdio-ifc-4x3` returns no test output. The source is formatted and statically inspected, but native registration, packed payload behavior and emitted STEP still require execution through authentic compiler infrastructure.

## Progress, Cancellation and Limits

The direct controlled API checks the caller during source scanning every 4,096 bytes, between parsed/written records, during identity/reference traversal and at individual value visits. Publication happens only after final validation and a final accepted checkpoint. `IoRunControl` bridges observe the original caller's cancellation authority at those same locations.

The existing `ArtifactComposition` and erasure registry remain synchronous and provide no caller control parameter. Their registered path therefore invokes the convenience source functions with an always-accept callback. This implementation wires a real source route but **does not claim complete retained host progress/cancellation for that synchronous path**. It must move to a genuine retained source job/controlled entry before large file UI responsiveness is proven. No fake ownership grants, background thread, fabricated control or enforcement workaround was added.

Shared grammar/writer work inside one admitted physical record remains synchronous. Graph-to-Part21 projection, per-record writer allocation and header projection are not individually stepped or budget-debited. Export checks the actual record byte limit after producing that bounded unit; a large already-owned scalar can therefore allocate before source refusal. Source string copy costs and aggregate capacity ownership are not yet charged through the original IO receiving ledger. These limits need further framework-native producer work and actual large-file cancellation/memory tests. Automatic dialect identification from arbitrary unlabelled bytes is also not added here; explicit registered target routing is provided.

No AGENTS files, build infrastructure, Git history/index or worktrees were changed. The host provider/editor under concurrent parent ownership was left untouched. Full semantic IFC4x3 conformance remains subject to the separate schema implementation report.

The IFC root's convenience selection now filters both target and supplied source dialects. The global composer registry and host router already use exact owner/counterpart import/export keys, so independent foreign sources sharing TXT do not collide. Discovery previously retained only the first entry per target and hid other foreign sources; it now unions all source dialects for that target in deterministic order. General tests cover three independent source routes to one target and distinct exact registry keys. This global integration behavior is statically inspected and awaiting execution with authentic compiler ingress.
