# Non-Codec Send-But-Not-Sync Compiler Contract — Preparation

> Concrete ticket-only refinement: see [Rustdoc fixture R1](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️plugin-non-sync-rustdoc-fixture-r1-2026-08-28.md). It supersedes the unused/no_run positive helper below with executable binding construction, removes unnecessary speculative aggregate JSON/unit-test leaves, and corrects the overbroad attribution of Sync-only dependency blockers. Historical preparation is preserved; no compiler was run.

## Decision and Scope

Use a real, newly authored **Cell-backed mutation payload** and a real `ArtifactApp`, then prove both sides against the original repository APIs: a concrete non-codec `PluginBuilder::document_app` function must compile; the same app's `register_document_codec_for_app` use must be rejected for `Sync`. Do not extract or duplicate either trait, add global bounds, use unsafe auto-trait implementations, or reuse another owner's descriptor as the new leaf's metadata.

The smallest existing compiler-negative convention found is Rustdoc's paired positive/compile-fail examples (UI contract ImageBuilder1449). The Plugin has no trybuild/compiletest dependency or mounted compiler-negative harness. I propose the same **Rustdoc mechanism**, with an explicitly registered, budgeted doc-test route; not a new dependency or raw rustc driver. This is a proposal only: no schema, fixture, Rust doc mount, route or compiler invocation was authored/executed.

There is a real dependency blocker: this proof must build the actual Plugin library and OS-kernel dependency. The still-open production Sync detach ownership/await join and any concrete mandatory-descriptor fallout cannot be bypassed by the successful isolated derive AST law. No Plugin native readiness or favorable compile outcome is claimed.

## Exact Existing Sources

| Source | Observed Contract |
|---|---|
| [ArtifactApp](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:10888) | App owner requires Default+Send+'static; not Sync. |
| [Associated mutation](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:11074) | Actual protocol Mutation+PartialEq+Send+OpText+OpBinary+'static; not Sync. Snapshot separately requires Sync. |
| [Real base Mutation](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs:104) | Clone+serde and actual mandatory DESCRIPTORS/descriptor. No blanket Sync. |
| [Builder registration](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs:321) | `document_app<A: ArtifactApp>`, only additional PA:From<VcsArtifactApp<A>>. Stores the real definition and noncapturing factory; no codec request. |
| [Builder type](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs:53) | `PluginBuilder<State, PA>`; the positive witness must use **Ready as first generic argument**, not pretend PA is the sole parameter. |
| [Runtime owner](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:18391) | Actual VcsArtifactApp<A,M=NoMembers>; PluginApp impl22925 keeps A:ArtifactApp and M:Send, not Mutation:Sync. |
| [Codec boundary](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:29847) | Public `plugin_runtime::register_document_codec_for_app<A>` now has the approved method-local A::Mutation:Sync bound. |
| [Underlying codec](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:9127) | Actual ArtifactCodec::of requires Mutation:Send+Sync; only compile_dsl/print_mirror erased futures gained Send. |
| [Existing fixture pattern](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🎲️dummy/🦀️.rs:187) | Real ArtifactApp with typed mutation/command, existing NoConfig/NoDraft/NoPresence/NoTransient lanes, real render result. Its initialized VCS/factory helpers must not be run just to check this type boundary. |
| [Real leaf provenance](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs:581) | MutationLeaf resolves its original local source, adjacent descriptor, owner taxonomy and workspace token; this cannot be replaced by a copied generated descriptor. |
| [Canonical operation writer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️component.rs:352) | Existing DslVariants/variants_binary encodes format1, ordinal, actual record body. Use it, not a new ad-hoc JSON or tag-only wire protocol. |

ArtifactEditor25756/25764 and ArtifactViewer26096/26104 likewise do not globally require Mutation:Sync, but this first compiler packet should select only ArtifactApp, not claim editor/viewer coverage.

## Real Fixture Design

Proposed names are `CellSnapshot { count:i32 }`, `SetCellCount { value:std::cell::Cell<i32> }`, one-variant `CellMutation::SetCellCount`, `CellDiff`, and `CellApp`. The cell is the **actual semantic value**, not PhantomData, a sentinel, an extracted trait twin, or an unused marker. Diff reads value.get(), sets the new count; inverse records the original count. Clone/serde preserve the scalar payload. No interior alias crosses threads.

Pinned local standard-library source explicitly gives Cell<T> Send when T:Send and a negative Sync impl (core/cell.rs317/325). It also supplies Clone for Copy and PartialEq for Copy+PartialEq. Cargo.lock pins serde/serde_core1.0.228; I read that exact version's Cell serialization/deserialization implementations (ser600/de2091), which require Copy and serialize the contained scalar. An earlier inspection of installed1.0.229 was not treated as the lockfile authority.

The leaf must use actual `dsl::MutationLeaf` and aggregate `dsl::Mutations`, with a new truthful adjacent owner/leaf descriptor and payload schema. Do not alter/reuse Dummy's authored leaf metadata. The snapshot remains ordinary Sync i32. Config/draft/presence/transient use the existing explicitly uninhabited No* mutation types; those empty rosters are legitimate only for the unused lanes, never for CellMutation.

For text/binary obligations, supply a small real DslVariants record mapping whose integer field is read from/written into the cell; use the existing parse/print and variants_binary helpers. MutationLeaf itself does not inspect the Rust field shape, but its provenance/descriptor must resolve. Do not assume DslRecord derives Cell<i32> automatically: the scalar mapping is a required fixture implementation detail, to be reviewed before mount. No always-Err or panic-only codec stubs.

CellApp implements required initial_snapshot, handle and render using actual repo types; render can reuse public `app::built_text_to_component_tree`334 and `ui_wgpu::wgpu::Label::data`, as the existing Dummy fixture does. No fake initialized RuntimeAppCell, reserved factories or document publication authority is fabricated. The compile-positive registration helper takes an already existing App argument; it does not construct/open/close a VCS instance.

## Exact Compiler Witnesses

The positive doctest imports the actual external Plugin crate and includes the **same original fixture source** used by the negative example. It typechecks:

```rust
type RuntimeOwner = plugin::app::VcsArtifactApp<CellApp>;
type ReadyBuilder = plugin::app::PluginBuilder<plugin::app::Ready, RuntimeOwner>;

fn assert_send<T: Send>() {}
fn assert_app<T: plugin::app::ArtifactApp>() {}

fn retain_non_codec_registration(
    builder: ReadyBuilder,
    app: plugin::app::App,
) -> ReadyBuilder {
    builder.document_app::<CellApp>(app)
}

assert_send::<CellMutation>();
assert_app::<CellApp>();
```

This is compiler proof of the actual concrete non-codec owner/registration function. A `no_run` example deliberately does not execute builder/App construction or retirement; it is not runtime registration or publication certification.

The negative example includes byte-identical fixture definitions and adds only an actual call, inside an async body, to:

```rust
plugin::plugin_runtime::register_document_codec_for_app::<CellApp>(
    CellApp::DOCUMENT_SCHEMA,
).await
```

It is a `compile_fail,E0277` example. Expected reason: Cell<i32> is not Sync, carried through SetCellCount/CellMutation into the exact local A::Mutation:Sync requirement. No registration future is executed. This must not be replaced with a test that merely asserts CellMutation:Sync fails; the failure must occur through the real codec API.

The pinned nightly's installed Rustdoc book confirms error-code-qualified compile-fail examples require no new feature flag. It also warns the requested code is not necessarily the only diagnostic. Therefore: run/require the positive fixture first, retain all negative compiler output available from Rustdoc, and inspect the actual local-bound chain. A missing import, missing metadata file, another fixture failure, or generic E0277 on an unrelated obligation is not this law's acceptance. If Rustdoc suppresses that diagnostic chain even with output enabled, report that evidence limit and review a diagnostic-preserving route before claiming exact-cause proof; do not silently switch to raw rustc or trybuild.

## Schema and Independent Neutral Test

Propose a **new child packet**, leaving the accepted12-site source schema/fixture unchanged. Strict schema fields:

- version, concrete fixture/owner/leaf IDs and source paths;
- scalar signed-i32 domain and semantic set/inverse vectors;
- capabilities `ownedSend` and `sharedSync`;
- boundary kind `non-codec-document-app` or `document-codec`;
- expected acceptance and precise required capability;
- compiler witness kind (positive no_run vs negative E0277), real API path, expected owner/leaf chain;
- explicit `runtimeRegistrationExecuted:false`, `globalSyncExpansion:false`, `unsafeAutoTraitImpl:false`.

Neutral boundary rows cover Send+Sync and Send-only at both boundaries, plus non-Send refusal. The Cell row declares ownedSend=true/sharedSync=false; this is a language-neutral contract input, not proof that JS inferred a Rust auto trait.

Semantic examples include0→7,7→−3 and i32 endpoints; require exact inverse recovery and original source-value preservation. A small reference reducer via existing Lodash immutable clone/set is compared to the native fixture's actual MutationDiff/apply/inverse. Serde JSON scalar round-trip can be compared with JavaScript JSON values. These third-party checks validate the semantic fixture/model; only Rust compiler execution proves the type boundary. No invented native result or wire-vector claim is made now.

Proposed positive unit selector: `codec_send_noncodec_owner_semantics`. It would exercise only the small mutation/diff values and serde/text/binary round trips, not VCS construction. No hidden global store registration or generic retained-owner Drop is needed.

## Proposed Canonical Paths and Mount

All names below require taxonomy admission; none exists as a new mounted packet from this work:

```text
Plugin/📦️codec/🧵️send/🔬️compiler/
  📜️script.ts
  🦀️.rs                         # paired named Rustdoc witnesses
  🧬️schema/🔣️.json
  🧪️tests/🔣️.json
  🧪️tests/🦀️.rs                 # small semantic unit law
  🧪️fixture/🦀️.rs               # shared real snapshot/diff/app/command
  🧪️fixture/🧬️mutations/🦀️.rs
  🧪️fixture/🧬️mutations/🔣️.json
  🧪️fixture/🧬️mutations/📝️set-cell-count/🦀️.rs
  🧪️fixture/🧬️mutations/📝️set-cell-count/🔣️.json
  🧪️fixture/🧬️mutations/📝️set-cell-count/🧬️schema/🔣️.json
```

The doc witnesses should be attached through one deliberate `#[cfg(doc)]` module mount in Plugin source, with separately named doc items `codec_send_noncodec_owner` and `codec_send_rejects_non_sync_mutation`. This cfg is **doc-test ownership**, not suppression of a native law: the approved route must explicitly execute rustdoc, and ordinary --lib/nextest success must never be counted as its execution. The semantic unit module uses cfg(test). Do not export a test fixture through production API.

Use original-path include! for the common fixture so the macro's local_file() still resolves to its canonical owner; no source rewriting/copy into a temp crate. That exact span/provenance behavior is a precondition to verify before accepting either compiler outcome.

## Runner and Dependency Needs

Existing Plugin `:test` uses nextest and --lib; it does not execute Rustdoc. Do **not** push --doc through runCargoTestBudgeted's nextest path. Proposed separate package route `:test-codec-send-compiler` should be source-first registered by taxonomy, with the corresponding domain command and launch entries. No target is created now.

The route should use the existing exported budgeted process helper (runTestBudgeted/runCmdStatus, reviewed shared index1281/1744/1751) to invoke Cargo's **real Plugin --doc** test path. No new runtime/development library is needed. The two named witnesses are run serially, positive first; stop at its failure. Exact doc names must first be confirmed from the real rustdoc list after source coherence rather than guessed from line numbers. The eventual planned Cargo argument families are:

```text
test -p semio-framework-plugin --doc codec_send_noncodec_owner -- --nocapture
test -p semio-framework-plugin --doc codec_send_rejects_non_sync_mutation -- --nocapture
```

This is only proposed command data, not an invocation. Source-oracle routing/extra-argument refusal/positive-before-negative selection needs inert metadata TDD. Preserve same jobs2/master target/coverage0/pinned toolchain/build ceiling; apply the single existing finite packet deadline across serial phases rather than granting a new full budget to every retry. No arbitrary target, Cargo example intentionally breaking all-targets, feature fallback, ignored compiler failure, unsafe Send, or blanket Sync expansion.

The real Plugin dependency graph includes framework, OS kernel, schema, dispatch, UI contract/runtime, and others in its current Cargo manifest; it is materially broader than the completed isolated derive test. The proof cannot run until that actual library source is coherent. Known Store/Sync detach remains separately owned and unresolved; this packet does not propose touching it.

## Preserved Source and Next Review Boundary

Read-only [selected preparation capture](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️plugin-non-sync-compiler-preparation-2026-08-28.json) records12 exact source/config hashes. Current Plugin51fb521e, builderfc2e46b8 and base77e8205a match the released endpoints; no canonical bytes changed. This is selected evidence, not a compile closure.

The next review should decide the child path/doc-test route and truthful new leaf descriptor, then inspect the shared fixture and both witnesses **before any canonical mount or compiler lease**. Remaining uncertainties are original-span resolution under Rustdoc include!, actual negative diagnostic visibility, and the real Plugin library's current dependency coherence. They must become explicit observed outcomes, not source-inferred GREEN.
