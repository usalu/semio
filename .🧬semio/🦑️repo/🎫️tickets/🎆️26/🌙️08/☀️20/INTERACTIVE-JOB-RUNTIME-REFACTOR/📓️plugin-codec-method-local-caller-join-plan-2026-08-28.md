# Plugin Codec Method-Local Caller Join Plan

## Review Boundary

Source-only proposal, following the [R2 owned OS candidate](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️os-kernel-r2-owned-source-candidate-2026-08-28.md). No production, fixture, schema, package, launch or runner file was edited by this packet. No Cargo, native process, source oracle or test was executed. No source hold is requested. The actual OS92 compiler RED remains historical evidence, not a new compiler result.

The minimal proposed production delta is **twelve method-local qualifications in two Plugin files: eight `A::Mutation: Sync` clauses and four additions of `Sync` to existing bare `Mutation` bounds**. Preserve all current function bodies, sync/async signatures and await sites. Do not widen ArtifactApp, ArtifactEditor, ArtifactViewer, protocol::Mutation, associated Diff, PluginApp, or an enclosing impl.

Current Store SHA is **7c71a7bf09b8bac3fbfd8b420b98f3a82ae89d62ebd0c868f5e6e97d8bffc2c4**, matching the announced rejected-page cfg(test) mount. R2's codec changes remain present. Mutation's outer-sync84 and base-trait/default work, rejected-page tests, and Dag's resident release-phase candidate are separate. This report does not authorize a joint compile or any detach repair.

## Exact Generic Qualification Graph

| Existing item and source line | Proposed additional local requirement | Actual edge requiring it |
| --- | --- | --- |
| [ArtifactDeclarationBuilder::document_codec](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:3212) | `A::Mutation: Sync` | `document_codec_async::<A>` |
| [ArtifactDeclarationBuilder::document_codec_async](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:3216) | `A::Mutation: Sync` | `DocumentCodecSpec::of::<A>` |
| [ArtifactDeclarationBuilder::document_codec_bare](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:3238) | `Mutation: Sync` | `document_codec_bare_async::<Snapshot, Mutation>` |
| [ArtifactDeclarationBuilder::document_codec_bare_async](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:3246) | `Mutation: Sync` | `DocumentCodecSpec::bare::<Snapshot, Mutation>` |
| [DocumentCodecSpec::of](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:3333) | `A::Mutation: Sync` | `codec::<A> function item` |
| [DocumentCodecSpec::of::codec](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:3334) | `A::Mutation: Sync` | `ArtifactCodec::of::<A::Snapshot, A::Mutation>` |
| [DocumentCodecSpec::foreign](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:3340) | `A::Mutation: Sync` | `codec::<A> function item` |
| [DocumentCodecSpec::foreign::codec](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:3341) | `A::Mutation: Sync` | `ArtifactCodec::of::<A::Snapshot, A::Mutation>` |
| [DocumentCodecSpec::bare](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:3350) | `Mutation: Sync` | `codec::<Snapshot, Mutation> function item` |
| [DocumentCodecSpec::bare::codec](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:3355) | `Mutation: Sync` | `ArtifactCodec::of::<Snapshot, Mutation>` |
| [PluginBuilder::foreign_document_codec](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs:290) | `A::Mutation: Sync` | `DocumentCodecSpec::foreign::<A>` |
| [register_document_codec_for_app](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:29847) | `A::Mutation: Sync` | `ArtifactCodec::of::<A::Snapshot, A::Mutation>` |

Nested `codec` functions are independent generic items: an enclosing method's where clause does not qualify the nested item's separately declared `A` or `Mutation`. Both sides of each function-item assignment must be qualified. Conversely, adding only the nested requirement leaves the outer `codec::<A>`/bare instantiation unproven.

For app methods, the exact shape is `where A::Mutation: Sync`; retain `A: ArtifactApp`. For bare methods, insert `+ Sync` beside the existing `+ Send` in the existing Mutation bound. Do not introduce a new trait alias, wrapper, public marker, runtime branch or fallback constructor.

[ArtifactApp::Snapshot](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:11066) already supplies Clone, PartialEq, serde, Send, Sync, ArtifactDsl, ArtifactPack and 'static. [ArtifactApp::Mutation](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:11074) supplies protocol::Mutation, PartialEq, Send, OpText, OpBinary and 'static. [protocol::Mutation](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs:105) already supplies Clone, Serialize and DeserializeOwned. Thus Sync is the one missing qualification at this constructor boundary; no duplicate serde or new associated Diff requirement is proposed.

The [native_codecs<S,M> fixture](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:27494) already requires M: Send + Sync and needs no edit.

## Why The Obligation Ends At Codec Construction

Read the actual Store [ArtifactCodec fields](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:9085), [of and compile_dsl_impl](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:9124), and [print_mirror_impl](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:9145). Only compile_dsl and print_mirror now erase to Send futures. Both retain parsed P/Mutation ownership across await and share an ArtifactEnvelope across awaited printing/validation. Owned retention requires Send; sending that shared envelope reference requires Sync. Borrowed str/byte inputs already have the necessary native element traits.

No new A value is retained by the Plugin thunk; it monomorphizes an existing function item. Consequently A: Sync, PA: Sync, schema: Send, or Config/Draft/Presence/Transient mutation Sync are not justified by this edge. The declaration helpers are not being converted into erased Send futures. Their existing resolve_ready delegation and async methods remain exactly as authored; no executor/local-future fallback is added.

The erased boundary is concrete:

- [DocumentCodecSpec](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:3324) stores schema, extension, app_id, foreign and a plain `fn(String) -> ArtifactCodec`; no generic payload owner or capturing closure is added.
- [DocumentCodecSpec::codec](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:3365) invokes the already-qualified function pointer with the existing schema clone. It needs no new bound.
- [ArtifactRegistrationPlan::from_declarations](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:3983) reads the existing specs and pushes concrete ArtifactCodec values at4017/4022. It does not instantiate A or Mutation and needs no qualification.
- [PluginBuilder preflight and aggregate assembly](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs:628) consumes the same concrete specs; `try_build` and `PluginBuilder<Ready,PA>` remain unqualified.
- [NativeCodecs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:26921) and [IoDeclaration](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:26941) already store concrete ArtifactCodec. Existing subset assembly clones those concrete codec descriptors at27140/27225; it does not need a blanket subset bound.

Read [document_app](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs:321), [viewer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs:377) and [editor](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs:427) including their app-schema/factory thunks. These construct VcsArtifactApp/app metadata, not ArtifactCodec. Do not put Sync on these generic methods, their factories, or the corresponding traits merely because one caller also opts into foreign_document_codec.

[EditorApp<E>::ArtifactApp](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:26315) maps Mutation = E::Mutation at26420; [ArtifactEditor](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:25756) and [ArtifactViewer mutation](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:26104) remain Send-only for the mutation lane. A concrete codec call checks that concrete E::Mutation; no global editor/viewer restriction follows.

## Actual Upstream Caller Census

The attached [source census](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️plugin-codec-caller-join-source-census-2026-08-28.json) preserves exact paths, line numbers, matched text, observed file hashes and all twelve desired qualifications. It is a lexical census over default non-hidden Rust source, not Cargo membership, macro expansion, Rust auto-trait proof or an executed call graph. Comment-only mentions are excluded. Its nearestFunction field means nearest preceding fn text, **not** a certified enclosing function; nested helpers can intervene.

The current concrete domain calls are 31 app declaration calls, 28 bare declaration calls, two foreign calls and58 direct ArtifactCodec::of calls. These counts are source occurrences, not executed registrations or passing plugins. No generic domain wrapper was identified by this scan and the concrete caller reads below. Do not infer every concrete payload's Sync implementation from its name.

### App, Bare And Foreign Callers

- [Flow declaration](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️component.rs:380) calls document_codec::<EditorApp<FlowPlayApp>> at386. Its [concrete editor implementation](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:1551) maps Mutation = FlowMutation. Other app declarations likewise name concrete editor types; none requires a new generic app-wide bound in this proposal.
- [Energy declaration](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️component.rs:307) calls bare EnergyModelSnapshot/EnergyModelMutation at313. It has no A and receives only the existing bare constructor's local Mutation requirement.
- [PDF declaration](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🦀️component.rs:28) independently supplies its1.7 and1.4 concrete snapshot/mutation pairs at42/43. Preserve both schemas, order and registration descriptors; do not merge them or infer a shared generic bound.
- [Procedural module_plugin_bundle](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs:808) passes concrete ModuleApp at809. Its ArtifactApp implementation698–700 selects ModulePayloadMutation. That mutation's payload200 is ModuleRenderPayload107 (String fields, bool and DslValue); [DslValue](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🦀️component.rs:17) is the existing scalar/String/Vec tree. This source shape is compatible with the proposed bound; no compiler execution is claimed.
- [Space plugin](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🦀️component.rs:556) passes concrete SpaceApp at571. [SpaceApp implementation](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️component.rs:574) selects WorkflowMutation. Its [actual aggregate](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🧬️schema/🧬️mutations/🦀️.rs:66) contains eighteen concrete leaf types, not a generic mutation parameter. No blanket Workflow/Space trait edit is proposed; full leaf auto-trait validation remains a compiler obligation.

Repository searches for register_document_codec_for_app found its definition/body and documentation but **no authored invocation** in the scanned source. Existing declaration docs suggesting that each declaration calls this function are not the actual path: DocumentCodecSpec constructs an erased codec, then the registration plan commits it. Retain the existing register method and give it the local qualification; do not delete it or claim its async body has executed.

### Direct Concrete Codec Calls — Correction To Earlier Census Scope

The earlier R2 paragraph listing Store/Sync/MCP/Plugin callers was not a complete repository-wide constructor inventory. The expanded census additionally finds58 direct constructor sites under domain subset IO/schema code. This report supersedes that paragraph's implied completeness, without rewriting earlier evidence.

For example, [Dag io](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs:20) constructs NativeCodecs with concrete DagSnapshot/DagMutation at56. The nested entries helper at28 is not the enclosing constructor scope. Writer/Forms/Block/Puzzle/etc. use the same concrete NativeCodecs boundary. These sites need real concrete auto-trait checks, not generic wrapper qualifications.

Some direct registration sites contain pre-existing dropped async registration futures, e.g. [MP4 register](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🦀️component.rs:46) uses `let _ = store::register_document_codec(...)` at49 without await. The same source shape exists in the concrete [MCP Probe registration](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️component.rs:237). These are separate existing completion/registration issues, not additional Sync bounds and not fixed by this packet. No executed registration credit is inferred.

The searched DSL derive/framework Rust and TypeScript sources yielded no additional emitted DocumentCodecSpec/document_codec/foreign_document_codec wrapper. This remains a source search, not a claim about every possible macro expansion.

## Minimal Schema-First Desired-Law Packet — Proposed, Not Mounted

Proposed canonical domain: Plugin/📦️codec/🧵️send, with `🧬️schema/🔣️.json`, `🧪️tests/🔣️.json`, `🧪️tests/🦀️.rs` and its sole `📜️script.ts`. Coordinate its exact taxonomy/registered source route before mounting. Reuse the existing Plugin native test runner for future owner-crate laws; do not add a raw Cargo driver or change the OS six-law selection.

1. Author a language-neutral contract for four construction routes (app-owned, foreign, bare, direct registration), the twelve exact obligation sites, and invariants: only document Mutation gains shared-read qualification; erased codec identity/schema and sync/async ownership stay unchanged; no new global trait/Diff bounds; only the existing two erased futures require Send. No Rust syntax is used as a substitute for the neutral access/qualification model.
2. Use the existing dev-only Ajv2020+Lodash pattern from [Store codec Send oracle](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧵️send/📜️script.ts:1): strict schema validates authored rows; an independently expressed Lodash set relation computes required sites from route edges and shared-payload access, compared with the implementation's explicit traversal. Reject duplicate/missing/extra sites and any broad trait qualification. This validates the neutral model only, not Rust Send/Sync.
3. Separately inspect the **actual twelve source headers**, anchored to their containing item, and compare the extracted local qualifiers with the authored requirement set. Current source should report twelve missing local qualifications; do not manufacture a native compiler error count. Guard the exact associated Mutation declarations on ArtifactApp/Editor/Viewer and the unchanged other two codec slots. A bounded selector must reject duplicate/missing anchors and ignore comments/strings; it is a source-contract check, not a Rust parser or compiler proof.
4. Source TDD hostiles remove each required qualification independently; place a qualification only on the outer/nested item; widen a global mutation trait or associated Diff; remove either Send slot; replace the plain fn pointer with a capturing closure; change a parse/print body or await. Every hostile must actually change inspected source/model input and be rejected, not merely assert a fixed expected number.
5. Preserve selected original function bodies and spec field assignments byte-for-byte after excluding only reviewed header edits. Preserve original schema strings, schema ownership, both PDF registrations, registry preflight/conflict behavior and whole-codec structure. Do not use whole main/protocol file pinning to reject concurrent unrelated owner changes.

No desired-law model, source oracle, hostile case, source RED or native law described above has run yet. The current report/census are research artifacts only.

### Subsequent Native Proof, Separately Authorized

Propose a small owner-crate fixture that references all four public construction routes and the three private spec routes using exact existing Sync-capable authored fixture types; verify schema/extension/app_id/foreign fields and inert staging before registry commit. Fully await async methods/registration rather than treating a dropped future as a passing test. A generic compile probe must explicitly carry only A: ArtifactApp and A::Mutation: Sync (or the unchanged bare bounds plus Sync), not an extra A/PA/Diff bound that hides the real requirement.

A separate non-codec fixture should use a concrete Send-but-not-Sync document mutation (e.g. an owned Cell scalar with exact authored mutation/codec metadata) and demonstrate that the base ArtifactApp contract remains implementable without constructing a codec. Do not repurpose shared TestApp modes or weaken metadata to obtain that fixture. Source-only absence of a Sync bound does not itself execute this negative boundary; compiler acceptance of that concrete fixture is required later.

Retain the two existing OS `document_codec_native_send_*` laws unchanged; they exercise real parse/print output and compiler Send requirements. A Plugin dependency compile cannot execute OS cfg(test). Any actual future failure involving an associated Diff or a concrete domain payload must be attributed from diagnostics before adding bounds; this plan does not preauthorize a blanket cascade or a wider native graph.

## Observed Core Inputs And Remaining Blockers

Captured read-only at 2026-08-28T02:53:14.209Z; no freeze/atomic compiler closure is claimed:

```json
[
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs",
    "sha256": "2ad816977def25ded3175c87c0f7d03344f1bae57549689b17388adf871736ca"
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs",
    "sha256": "10c85d56f64e5b2b7ab81276365aa6f57516fac2a459c200a5c8d1683b351868"
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs",
    "sha256": "7c71a7bf09b8bac3fbfd8b420b98f3a82ae89d62ebd0c868f5e6e97d8bffc2c4"
  },
  {
    "path": "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs",
    "sha256": "e5f2f9ce74cc305bcbc23c0d99ab70cc2af54cf299a561f7910d56a7dbbd8385"
  },
  {
    "path": "🧰️framework/🔨️modules/🌱️value/🦀️component.rs",
    "sha256": "9abe77b58b04e36db58309a6e21827e3c372fc82595068abd6bc8af0191f6d46"
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧵️send/📜️script.ts",
    "sha256": "2924cbb556b457da9e53d98b6d2f2f2f3c03ea97085d8f85023154411907d669"
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts",
    "sha256": "c98da5ce13ef320d2bc14da17cea5550096a92066fd2ec0185311e528f7a0ac6"
  },
  {
    "path": "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️component.rs",
    "sha256": "354bd610a6c6b3d6d92798f846334537de83d94472256e5128873d49e14622b6"
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🧬️schema/🧬️mutations/🦀️.rs",
    "sha256": "e90fc0e21fd022e08d16f32d63dc049bd40ed1f01cd986aa42b27ac2bafbd113"
  }
]
```

The census's121-file observations and these core observations are provenance only. No selected compiled inputs, native selection count or pass result is claimed.

Still separate and unresolved: original-parent Store detach/SyncSession forwarding, Mutation's84 outer-sync fixture repairs, actual concrete plugin auto-trait/registration execution, and funded Opening/private parent receiver. The current resident phase candidate does not supply that missing Store parent API. No compatibility layer, ignored returned backbone, cloned disposer, extra pool, local-future fallback, unsafe Send, new feature or budget change is proposed.

**Coherent review boundary:** the twelve method-local edits and the schema/source-law packet above are ready for parent review. Production remains unchanged by this plan; native lane idle.

