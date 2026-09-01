# Plugin Non-Sync Rustdoc Fixture R1

## Status and Review Boundary

Ticket-only preparation, 2026-08-28. No canonical mount, production change, Cargo/Rustdoc/compiler, Nx test, source oracle, new dependency or native lease. The previously proposed unused positive helper is superseded by an ordinary executable Rustdoc example that constructs the concrete binding. Nothing here is a native PASS or an observed compiler diagnostic.

The [selected capture](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️plugin-non-sync-compiler-r1/🧪️source-capture.json) records30 current source/draft hashes, not a resolved compiler closure. [Raw hash output](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️plugin-non-sync-compiler-r1/📓️read-only-capture.md) is preserved. Plugin51fb521e, builderfc2e46b8, base77e8205a, Store7c71a7bf and Sync62f31952 remain unchanged. All authored changes are under this ticket.

## Concrete Paired Witnesses

| Draft | Exact identity and role |
| --- | --- |
| [Positive](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️plugin-non-sync-compiler-r1/🟢️positive/🦀️.rs:1) | 19lines/711bytes; SHAfe3c9be41958d2c0ee797e02a8331cce96ee29d0eecf3e530f08a9760dc2745e. Ordinary run, not no_run. |
| [Negative](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️plugin-non-sync-compiler-r1/🔴️negative/🦀️.rs:1) | 24lines/978bytes; SHAc0d7c3f16f0052118ea3847f424c434cb870d7819284168b6a668b46e736862d. Same bytes plus exactly five lines/267bytes before positive line16. |
| [Paired doc leaf](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️plugin-non-sync-compiler-r1/🦀️.rs:1) | SHA7482485cf8bab0b14bd5fe79e19c28f398d27297195174adb3ec37979523bf63. Positive snippet lines6–24, item26; negative lines31–54, item56. Negative registration is47–50. |
| [Shared owner](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️plugin-non-sync-compiler-r1/🧪️fixture/🦀️.rs:1) | SHAd0d2982d708f2ce25edc0119c1b35522f33c4e24a89da2db8ef5b9eeada39d60. Same original include for both witnesses. |
| [Aggregate](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️plugin-non-sync-compiler-r1/🧪️fixture/🧬️mutations/🦀️.rs:1) | SHA6db267e22326c336f1a861269fe4fc0d0cc9edd4bd80ae4d0378eeb214561b3a; real DslOps + Mutations derives. |
| [Leaf](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️plugin-non-sync-compiler-r1/🧪️fixture/🧬️mutations/📝️set-cell-count/🦀️.rs:1) | SHA39467076ced7d81c8f12b60799d7eaaff3df1e96196036d02f324d8a2d1a4597; real Cell<i32> semantic payload and MutationLeaf derive. |

A read-only diff displayed only this insertion; no new native/source-test driver was executed:

```rust
    plugin::app::resolve_ready(
        plugin::plugin_runtime::register_document_codec_for_app::<fixture::CellApp>(
            <fixture::CellApp as plugin::app::ArtifactApp>::DOCUMENT_SCHEMA,
        ),
    ).expect("the non-Sync shared codec must not typecheck");
```

The first15 lines are identical. Positive lines16–19 are identical to negative21–24. The exported API is called directly; this is not an extracted trait twin, an assertion of Sync alone, or a private fixture shortcut.

### Actual Positive Construction

Shared source116–131 calls the real App::builder, supplies document state, an Edit mode and Main window, uses App::try_from_builder, then executes PluginBuilder::<NeedsLabel,VcsArtifactApp<CellApp>>::new(...).label(...).version(...).document_app::<CellApp>(app). The concrete ReadyBuilder result is bound in main and later dropped. Mode/window declarations are necessary: actual AppBuilder validation5205–5224 refuses their absence.

The program does not call PluginBuilder::try_build, invoke its stored app factory, instantiate VcsArtifactApp, open a RuntimeAppCell, register a global codec, or manufacture job/disposer admission. The existing document_app body321–341 stores the noncapturing concrete factory and definition; it does not invoke the factory. A fixture-only AtomicUsize in CellApp::default and assertions before/after builder disposal detect accidental runtime construction. This scalar is a test observation, not an ownership permit or a second ledger.

Both examples call the same semantic function. The negative does not execute at all when its intended type error occurs. The positive's runtime assertions are prospective only until the ordinary doctest actually runs.

### Real Auto-Trait and Semantic Payload

SetCellCount.value is Cell<i32>, not a phantom marker. diff reads get(), inverse stores the original count, serde serializes the scalar, and the local DslField implementation projects through a real derived CellFields record. No trait is implemented for foreign Cell; no unsafe Send/Sync implementation or hidden mutex converts it into Sync.

CellSnapshot is a normal i32 record implementing the repository ArtifactDsl/ArtifactPack APIs with actual DSL, PACK and Semio envelopes. CellMutation uses the repository DslVariants text and binary machinery; leaf codecs delegate to that same one-variant representation. Descriptor binaryTag0 names its actual ordinal0, not the format-version byte1. This does not certify a newly authored external wire vector.

The app's Command and unused config/draft/presence/transient lanes use the existing genuinely uninhabited No* mutation types. Command must be Sync under ArtifactApp11108; using CellMutation as Command would cause an unrelated positive failure. The app declares no executable commands and no tool factories. The document mutation is nonempty and has exactly one real descriptor; it is never substituted by an empty roster.

## Schema, Metadata and Original Source Identity

The [strict packet schema](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️plugin-non-sync-compiler-r1/🧬️schema/🔣️.json) was authored before Rust. Its [neutral data](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️plugin-non-sync-compiler-r1/🧪️tests/🔣️.json) pins four Send/Sync capability combinations and five signed-i32 set/inverse cases, including both extrema. The scope fields explicitly say this is an unexecuted ticket packet.

The [adjacent14-field descriptor](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️plugin-non-sync-compiler-r1/🧪️fixture/🧬️mutations/📝️set-cell-count/🔣️.json) names SetCellCount/set-cell-count and the exact proposed owner below; its [payload schema](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️plugin-non-sync-compiler-r1/🧪️fixture/🧬️mutations/📝️set-cell-count/🧬️schema/🔣️.json) accepts only one signed-i32 value. No Dummy/Txn metadata or base trait is copied or changed.

Proposed canonical root, still requiring taxonomy admission:

```text
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️codec/🧵️send/🔬️compiler/
  🦀️.rs
  🧬️schema/🔣️.json
  🧪️tests/🔣️.json
  🧪️fixture/🦀️.rs
  🧪️fixture/🧬️mutations/🦀️.rs
  🧪️fixture/🧬️mutations/📝️set-cell-count/🦀️.rs
  🧪️fixture/🧬️mutations/📝️set-cell-count/🔣️.json
  🧪️fixture/🧬️mutations/📝️set-cell-count/🧬️schema/🔣️.json
```

The two standalone positive/negative files remain ticket byte witnesses; they need no duplicate canonical mount. The earlier speculative aggregate JSON and separate unit-test source are unnecessary for this smallest pair: the real aggregate derive requires the adjacent leaf descriptor, and the positive ordinary doctest runs the five semantic cases itself.

A future reviewed doc-only mount would be exactly:

```rust
#[cfg(doc)]
#[path = "📦️codec/🧵️send/🔬️compiler/🦀️.rs"]
pub mod codec_send_compiler_contract;
```

That mount belongs in Plugin's canonical component, not in the externally included fixture and not under cfg(test). It has not been applied. Ordinary --lib/nextest never supplies proof of these doc examples.

Both snippets use this identical original-path include:

```rust
mod fixture {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../📦️codec/🧵️send/🔬️compiler/🧪️fixture/🦀️.rs"));
}
```

The shared owner explicitly includes the original aggregate, which explicitly includes the original leaf. No relative nested-module resolution, temporary Rust source copy or metadata rewriting is needed. All three derive invocations therefore have an intended original source span; **actual Rustdoc span behavior remains a compiler precondition to observe**, not a fact inferred from include syntax.

MutationLeaf581ff and Mutations1704ff require local_file(), the current workspace/taxonomy authority and exact adjacent owner. Generated includes read nx.json, root project, taxonomy and descriptor. The aggregate validates source scope/roster. Shared assert_semantics143ff additionally requires the generated leaf source_path to equal the proposed canonical original path and its owner to equal its descriptor owner. No fake provenance token is authored.

Consequently these ticket drafts intentionally are not a standalone compilable package: their include paths/metadata describe the proposed canonical destination. Canonical admission/mount must happen atomically before a future compiler attempt. A missing include, authority mismatch or derive diagnostic is a fixture/setup failure, not the negative Sync law.

## Proposed Exact Diagnostic Acceptance

The required observation is:

1. The positive's same shared source compiles and runs; the real binding and five semantic cases execute, zero runtime-app construction assertions pass.
2. The negative compile_fail example reports expected E0277.
3. Retained actual compiler diagnostics identify Cell<i32> as not Sync, SetCellCount and CellMutation as the containing chain, and the local A::Mutation:Sync qualification on register_document_codec_for_app at actual Plugin29847.
4. At least one primary error span is the added concrete registration expression (negative17–20 / doc47–50), not a helper, metadata declaration or unrelated type.
5. Any E0432/E0433/E0603/E0046, derive/const-provenance failure, missing source, link/runtime failure, or unrelated E0277 prevents exact-cause acceptance. Do not assert one E0277 count: Rust may repeat the same required-bound diagnostic around an async call.

Stock compile_fail,E0277 checks a requested code, not the absence of other errors. Its passing stderr visibility under the pinned Rustdoc invocation has not been verified. If it suppresses the diagnostic chain, the pair can only receive the narrower observed positive/expected-code scope; **exact-cause diagnostic proof stays open**. Do not fabricate stderr or add a permanently failing unfiltered doc item. Any extra diagnostic-preserving harness needs its own explicit review/registered route; none is authored or requested for immediate execution here.

## Existing Routing and Dependency Ordering

Current Plugin package script250d72c1 routes ordinary :test through runCargoTestBudgeted with --lib/nextest, or explicit --no-run through Cargo inventory. The registered :test-codec-send-source is the previous12-site source law, not this pair. Neither is an existing --doc route. Do not pass --doc through the nextest branch or claim the12-site oracle covers these new files.

Proposed taxonomy-owned domain/package command remains test-codec-send-compiler, with argument-refusal, positive-before-negative and one finite packet deadline verified by inert routing tests. Use the existing exported runTestBudgeted/runCmdStatus family, not raw unbudgeted rustc or a new library. The Cargo argument data would select the real Plugin --doc package and the two item names separately; actual Rustdoc list names/counts must be retained before executing. Keep the same master target/jobs2/coverage0/build ceiling. No new route is mounted, no command is run, no lease is inferred.

There is an important correction to the prior broad blocker wording. Actual OS glue260–262 gates Store Sync on feature sync. Plugin's own direct OS-kernel edge, framework's edge, schema's edge and UI's declarative wgpu→dsl edge inspected today do **not** request sync; OS default is deflate. Thus the still-present SyncSession::detach await900 and cfg(test) Demo missing descriptors3850 cannot automatically be called blockers for the proposed Plugin --doc command. Demo is also dependency cfg(test), which Plugin doc compilation does not execute. Those remain real separately owned source obligations, not permission to repair/ignore them here.

The full feature/dependency closure, including dev dependency unification, still needs an actual authorized metadata/capture boundary. No readiness is inferred from this partial source census. Plugin12 and base mandatory source joins remain unchanged; additional unconditional Plugin fixture/API errors, if encountered, must be reported before the pair. No broad OS rerun, feature disabling, stack override or fixture default is proposed.

## Independent Oracle and Next Stop

The future registered source-only command should validate the strict packet and leaf payload schema with the already installed Ajv, compare five reducer/inverse rows using Lodash cloneDeep/set against expected values, and check exact positive/negative deletion inverse plus extracted doc bytes and original source includes. These are required planned third-party checks, **not executed results**. Only the real Rust compiler supplies the Cell auto-trait boundary; JavaScript capability rows cannot prove it.

Safe stopping boundary reached: concrete eight-file canonical proposal plus paired ticket bytes and source capture are ready for root review. There is no native process, source hold, production mount, metadata request execution or follow-on command. The next resident scheduling request can proceed independently.
