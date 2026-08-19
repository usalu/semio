# Luna Census: dyn Trait Objects
**Date:** 2026-08-19  
**Method:** Python3 traversal using absolute paths (no shell globbing)  
**Verification:** Multiple regex patterns, cross-referenced with asyncify-universal.py

## Summary

- **Total first-party traits declared:** 236
- **First-party traits used as `dyn` trait objects:** 95
  - **In known six-family design list:** 13 ✓
  - **NOT in design list (NEW):** 82 ⚠️
- **Traits with E0053 risk** (sync methods in dyn trait): 3
- **Std/lang `dyn` residue:** 306 uses across 6 trait types (legal baseline)

## Table A: First-Party Traits Used as Trait Objects

**Format:** trait · declared (path:line) · total methods · async methods · dyn uses · status

| Trait | Declared | Methods | Async | Dyn Uses | Status | E0053 |
|-------|----------|---------|-------|----------|--------|-------|
| ActionHandler | 🧰️framework/🔨️modules/🎯️action-bus/🦀️component.rs:10 | 2 | 2 | 2 | NEW |  |
| AgentRunner | 🧰️framework/🛍️products/🦑️repo/🎮️commands/🌊️workflow/🦀️component.rs:224 | 3 | 3 | 1 | NEW |  |
| Animation | ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎞️animation/🦀️component.rs:24 | 9 | 9 | 3 | NEW |  |
| AppChannelHost | 🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs:105 | 2 | 2 | 1 | NEW |  |
| ArtifactChannel | 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🦀️component.rs:99 | 1 | 1 | 2 | NEW |  |
| AsyncHttpTransport | 🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs:661 | 1 | 1 | 1 | NEW |  |
| AuditSink | 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📒️audit/🦀️component.rs:99 | 1 | 1 | 2 | NEW |  |
| AuthorityStore | 🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️component.rs:117 | 12 | 12 | 1 | NEW |  |
| AuthzHook | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️artifact/🦀️component.rs:394 | 1 | 1 | 1 | NEW |  |
| Backbone | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:6184 | 3 | 3 | 9 | ✓ KNOWN |  |
| BackboneChannelPort | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:6291 | 2 | 2 | 6 | NEW |  |
| BackbonePort | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:6190 | 2 | 2 | 6 | ✓ KNOWN |  |
| BackboneTransport | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs:453 | 1 | 1 | 1 | NEW |  |
| BlobStore | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:6458 | 4 | 4 | 4 | NEW |  |
| BrepKernel | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️component.rs:140 | 92 | 92 | 8 | NEW |  |
| CapabilityChecker | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs:421 | 1 | 1 | 1 | NEW |  |
| CatalogStorage | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️component.rs:280 | 0 | 0 | 5 | ✓ KNOWN |  |
| ChildStoreFactory | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:527 | 2 | 2 | 1 | NEW |  |
| CiTest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🔗️causal-internals/🦀️component.rs:493 | 1 | 1 | 1 | NEW |  |
| Collective | 🧰️framework/🔨️modules/🧮️math/🎯️sampling/🦀️component.rs:2657 | 5 | 5 | 1 | NEW |  |
| CommandSink | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🔄️fsm/🦀️component.rs:639 | 1 | 1 | 2 | NEW |  |
| CompletionSink | 🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs:1349 | 1 | 1 | 1 | NEW |  |
| Compressor | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🎲️entropy-internals/🦀️component.rs:6044 | 1 | 1 | 1 | NEW |  |
| ConflictOracle | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/👁️preview/🦀️component.rs:201 | 1 | 1 | 1 | NEW |  |
| ConsistencyResolver | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔍️query/🦀️component.rs:276 | 0 | 0 | 1 | NEW |  |
| Constraint | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/⛓️constraint/🦀️component.rs:113 | 4 | 4 | 5 | NEW |  |
| DbStorage | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️component.rs:361 | 7 | 7 | 9 | ✓ KNOWN |  |
| Decider | 🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority/🦀️component.rs:103 | 3 | 3 | 2 | NEW |  |
| Denoiser | 🧰️framework/🔨️modules/🧮️math/🎯️sampling/🦀️component.rs:6935 | 2 | 2 | 1 | NEW |  |
| DirectoryWsConnection | 🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️component.rs:83 | 3 | 3 | 1 | NEW |  |
| DocumentAuthority | 🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️component.rs:197 | 2 | 2 | 1 | NEW |  |
| DynEngine | 🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️component.rs:43 | 1 | 1 | 2 | NEW |  |
| EffectMetricsRecorder | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs:519 | 1 | 1 | 1 | NEW |  |
| Element | ✏️s/🔨️modules/🏗️fem/⚙️engine/🏗️model/🦀️component.rs:85 | 8 | 8 | 8 | NEW |  |
| Emit | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🕸️version-graph/🦀️component.rs:121 | 1 | 1 | 6 | NEW |  |
| EnvelopeInjector | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs:192 | 1 | 1 | 1 | NEW |  |
| ErasedProjection | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📽️projection/🦀️component.rs:206 | 7 | 7 | 2 | NEW |  |
| FullTextLookup | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔍️query/🦀️component.rs:570 | 0 | 0 | 1 | NEW |  |
| GatewayBackend | 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧭️protocol/🦀️component.rs:514 | 7 | 7 | 2 | NEW |  |
| GuestRuntime | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:512 | 9 | 9 | 6 | ✓ KNOWN |  |
| HostAsyncRuntime | 🧰️framework/🔨️modules/⏳️async/🦀️component.rs:356 | 6 | 6 | 6 | ✓ KNOWN |  |
| HttpBody | 🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs:651 | 1 | 0 | 1 | NEW | YES |
| HttpTransport | 🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs:671 | 1 | 1 | 2 | NEW |  |
| IndexStorage | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️component.rs:297 | 0 | 0 | 7 | ✓ KNOWN |  |
| JoinHandleLike | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🎭️actor/🦀️component.rs:703 | 1 | 1 | 1 | NEW |  |
| LeaseStorage | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️component.rs:329 | 0 | 0 | 6 | ✓ KNOWN |  |
| LogitsProcessor | 🧰️framework/🔨️modules/🧮️math/🎯️sampling/🦀️component.rs:2579 | 8 | 8 | 1 | NEW |  |
| MachineCatalog | ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️component.rs:282 | 4 | 4 | 5 | NEW |  |
| MediaCache | 🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs:118 | 2 | 2 | 1 | NEW |  |
| MeshExporter | 🧰️framework/🔨️modules/🔺️mesh-engine/📦️packages/🦀️rust/📦️glue.rs:817 | 2 | 2 | 1 | NEW |  |
| MeshImporter | 🧰️framework/🔨️modules/🔺️mesh-engine/📦️packages/🦀️rust/📦️glue.rs:823 | 2 | 2 | 1 | NEW |  |
| Migration | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🔄️fsm/🦀️component.rs:1303 | 2 | 2 | 2 | NEW |  |
| NationalAnnex | ✏️s/🔌️plugins/📕️norm/📄️artifact/🦀️component.rs:213 | 9 | 9 | 4 | NEW |  |
| Operator | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️component.rs:643 | 1 | 0 | 11 | NEW | YES |
| OsBackbonePort | 🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs:778 | 2 | 2 | 4 | NEW |  |
| Part21Preamble | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🚪️io/📐️part21/🦀️component.rs:664 | 1 | 1 | 2 | NEW |  |
| PayloadSink | 🧰️framework/🔨️modules/🚪️io/🦀️component.rs:485 | 1 | 1 | 1 | NEW |  |
| PayloadSource | 🧰️framework/🔨️modules/🚪️io/🦀️component.rs:476 | 3 | 3 | 1 | NEW |  |
| PayloadStorage | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️component.rs:255 | 0 | 0 | 5 | ✓ KNOWN |  |
| PluginApp | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:9690 | 47 | 47 | 4 | ✓ KNOWN |  |
| PrincipalResolver | 🧰️framework/🛍️products/🖥️server/🔨️modules/🛡️policy/🦀️component.rs:215 | 2 | 2 | 2 | NEW |  |
| ProjectionStore | 🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️component.rs:292 | 6 | 6 | 1 | NEW |  |
| PromptRegistry | 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧭️protocol/🦀️component.rs:471 | 2 | 2 | 1 | NEW |  |
| QueryHandler | 🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️component.rs:211 | 2 | 2 | 1 | NEW |  |
| QuerySource | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔍️query/🦀️component.rs:544 | 2 | 2 | 1 | NEW |  |
| QueryableGraph | 🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️component.rs:80 | 7 | 7 | 1 | NEW |  |
| RandomAccessPayload | 🧰️framework/🔨️modules/🚪️io/🦀️component.rs:470 | 2 | 2 | 1 | NEW |  |
| RandomSource | 🧰️framework/🔨️modules/🧮️math/🎯️sampling/🦀️component.rs:1104 | 8 | 8 | 1 | NEW |  |
| ResourceRegistry | 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧭️protocol/🦀️component.rs:374 | 5 | 5 | 1 | NEW |  |
| ResourceResolver | 🧰️framework/🔨️modules/🚪️io/🦀️component.rs:497 | 2 | 2 | 1 | NEW |  |
| RouterEffectHandler | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs:379 | 1 | 0 | 1 | NEW | YES |
| Saga | 🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority/🦀️component.rs:439 | 1 | 1 | 1 | NEW |  |
| SamplingObserver | 🧰️framework/🔨️modules/🧮️math/🎯️sampling/🦀️component.rs:2641 | 5 | 5 | 1 | NEW |  |
| SceneHost | 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️scene_slots.rs:53 | 1 | 1 | 2 | NEW |  |
| ScriptRuntime | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs:543 | 1 | 1 | 1 | NEW |  |
| ServerModule | 🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️component.rs:161 | 5 | 5 | 1 | NEW |  |
| SessionStore | 🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️component.rs:437 | 4 | 4 | 1 | NEW |  |
| ShardTransport | 🧰️framework/🔨️modules/🎭️actor/🦀️component.rs:2107 | 4 | 4 | 2 | NEW |  |
| SnapshotStorage | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️component.rs:228 | 0 | 0 | 6 | ✓ KNOWN |  |
| Sobject | ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🦀️component.rs:644 | 37 | 37 | 7 | NEW |  |
| SoftConstraint | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🪶️soft/🦀️component.rs:11 | 2 | 2 | 2 | NEW |  |
| SolidExporter | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️component.rs:1475 | 2 | 2 | 2 | NEW |  |
| SolidImporter | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️component.rs:1481 | 2 | 2 | 2 | NEW |  |
| SourcingModule | ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs:369 | 5 | 5 | 1 | NEW |  |
| SpaceBackbonePort | 🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs:1328 | 2 | 2 | 3 | NEW |  |
| SpaceMember | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:6480 | 25 | 25 | 2 | ✓ KNOWN |  |
| StopCondition | 🧰️framework/🔨️modules/🧮️math/🎯️sampling/🦀️component.rs:2624 | 6 | 6 | 1 | NEW |  |
| StorageBackend | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs:397 | 3 | 3 | 1 | NEW |  |
| TextRenderer | ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔤️text/🦀️component.rs:469 | 1 | 1 | 1 | NEW |  |
| ThreadSpawner | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🎭️actor/🦀️component.rs:696 | 1 | 1 | 1 | NEW |  |
| TokenSampler | 🧰️framework/🔨️modules/🧮️math/🎯️sampling/🦀️component.rs:2594 | 3 | 3 | 1 | NEW |  |
| TokenTextAdapter | 🧰️framework/🔨️modules/🧮️math/🎯️sampling/🦀️component.rs:1288 | 3 | 3 | 1 | NEW |  |
| ToolRegistry | 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧭️protocol/🦀️component.rs:276 | 2 | 2 | 1 | NEW |  |
| VersionGraph | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🕸️version-graph/🦀️component.rs:34 | 4 | 4 | 3 | NEW |  |
| WalStorage | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️component.rs:182 | 0 | 0 | 5 | ✓ KNOWN |  |

## Table B: Verification of Six Known Families

**Design assumption:** Six trait families (actually 13 trait types) need dyn-to-enum dispatch conversion.

| Trait | Declared | Methods (actual) | Async (actual) | Dyn Uses (actual) | Design Assumed |
|-------|----------|------------------|---|------|------------------|
| Backbone | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:6184 | 3 | 3 | 9 | — |
| BackbonePort | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:6190 | 2 | 2 | 6 | — |
| CatalogStorage | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️component.rs:280 | 0 | 0 | 5 | — |
| DbStorage | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️component.rs:361 | 7 | 7 | 9 | — |
| GuestRuntime | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:512 | 9 | 9 | 6 | 9/15 |
| HostAsyncRuntime | 🧰️framework/🔨️modules/⏳️async/🦀️component.rs:356 | 6 | 6 | 6 | 3/10 |
| IndexStorage | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️component.rs:297 | 0 | 0 | 7 | — |
| LeaseStorage | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️component.rs:329 | 0 | 0 | 6 | — |
| PayloadStorage | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️component.rs:255 | 0 | 0 | 5 | — |
| PluginApp | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:9690 | 47 | 47 | 4 | 49/26 |
| SnapshotStorage | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️component.rs:228 | 0 | 0 | 6 | — |
| SpaceMember | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:6480 | 25 | 25 | 2 | 25/16 |
| WalStorage | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️component.rs:182 | 0 | 0 | 5 | — |

## Section C: NEW Trait Families (Not in Design)

**Count:** 82 new families discovered beyond the six-family list.

**Categorization by scope:**

### Framework-declared NEW traits

- **Operator** (11 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️component.rs:643
  - Methods: 1 (0 async, 1 sync)
  - Used in 11 files

- **BackboneChannelPort** (6 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:6291
  - Methods: 2 (2 async, 0 sync)
  - Used in 6 files

- **Emit** (6 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🕸️version-graph/🦀️component.rs:121
  - Methods: 1 (1 async, 0 sync)
  - Used in 6 files

- **BlobStore** (4 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:6458
  - Methods: 4 (4 async, 0 sync)
  - Used in 4 files

- **OsBackbonePort** (4 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs:778
  - Methods: 2 (2 async, 0 sync)
  - Used in 4 files

- **SpaceBackbonePort** (3 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs:1328
  - Methods: 2 (2 async, 0 sync)
  - Used in 3 files

- **VersionGraph** (3 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🕸️version-graph/🦀️component.rs:34
  - Methods: 4 (4 async, 0 sync)
  - Used in 3 files

- **ActionHandler** (2 dyn uses)
  - Declared: 🧰️framework/🔨️modules/🎯️action-bus/🦀️component.rs:10
  - Methods: 2 (2 async, 0 sync)
  - Used in 2 files

- **ArtifactChannel** (2 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🦀️component.rs:99
  - Methods: 1 (1 async, 0 sync)
  - Used in 2 files

- **AuditSink** (2 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📒️audit/🦀️component.rs:99
  - Methods: 1 (1 async, 0 sync)
  - Used in 2 files

- **Decider** (2 dyn uses)
  - Declared: 🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority/🦀️component.rs:103
  - Methods: 3 (3 async, 0 sync)
  - Used in 2 files

- **DynEngine** (2 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️component.rs:43
  - Methods: 1 (1 async, 0 sync)
  - Used in 2 files

- **ErasedProjection** (2 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📽️projection/🦀️component.rs:206
  - Methods: 7 (7 async, 0 sync)
  - Used in 2 files

- **GatewayBackend** (2 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧭️protocol/🦀️component.rs:514
  - Methods: 7 (7 async, 0 sync)
  - Used in 2 files

- **HttpTransport** (2 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs:671
  - Methods: 1 (1 async, 0 sync)
  - Used in 2 files

- **PrincipalResolver** (2 dyn uses)
  - Declared: 🧰️framework/🛍️products/🖥️server/🔨️modules/🛡️policy/🦀️component.rs:215
  - Methods: 2 (2 async, 0 sync)
  - Used in 2 files

- **SceneHost** (2 dyn uses)
  - Declared: 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️scene_slots.rs:53
  - Methods: 1 (1 async, 0 sync)
  - Used in 2 files

- **ShardTransport** (2 dyn uses)
  - Declared: 🧰️framework/🔨️modules/🎭️actor/🦀️component.rs:2107
  - Methods: 4 (4 async, 0 sync)
  - Used in 2 files

- **AgentRunner** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/🦑️repo/🎮️commands/🌊️workflow/🦀️component.rs:224
  - Methods: 3 (3 async, 0 sync)
  - Used in 1 files

- **AppChannelHost** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs:105
  - Methods: 2 (2 async, 0 sync)
  - Used in 1 files

- **AsyncHttpTransport** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs:661
  - Methods: 1 (1 async, 0 sync)
  - Used in 1 files

- **AuthorityStore** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️component.rs:117
  - Methods: 12 (12 async, 0 sync)
  - Used in 1 files

- **AuthzHook** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️artifact/🦀️component.rs:394
  - Methods: 1 (1 async, 0 sync)
  - Used in 1 files

- **BackboneTransport** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs:453
  - Methods: 1 (1 async, 0 sync)
  - Used in 1 files

- **CapabilityChecker** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs:421
  - Methods: 1 (1 async, 0 sync)
  - Used in 1 files

- **ChildStoreFactory** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:527
  - Methods: 2 (2 async, 0 sync)
  - Used in 1 files

- **Collective** (1 dyn uses)
  - Declared: 🧰️framework/🔨️modules/🧮️math/🎯️sampling/🦀️component.rs:2657
  - Methods: 5 (5 async, 0 sync)
  - Used in 1 files

- **CompletionSink** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs:1349
  - Methods: 1 (1 async, 0 sync)
  - Used in 1 files

- **ConflictOracle** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/👁️preview/🦀️component.rs:201
  - Methods: 1 (1 async, 0 sync)
  - Used in 1 files

- **ConsistencyResolver** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔍️query/🦀️component.rs:276
  - Methods: 0 (0 async, 0 sync)
  - Used in 1 files

- **Denoiser** (1 dyn uses)
  - Declared: 🧰️framework/🔨️modules/🧮️math/🎯️sampling/🦀️component.rs:6935
  - Methods: 2 (2 async, 0 sync)
  - Used in 1 files

- **DirectoryWsConnection** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️component.rs:83
  - Methods: 3 (3 async, 0 sync)
  - Used in 1 files

- **DocumentAuthority** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️component.rs:197
  - Methods: 2 (2 async, 0 sync)
  - Used in 1 files

- **EffectMetricsRecorder** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs:519
  - Methods: 1 (1 async, 0 sync)
  - Used in 1 files

- **EnvelopeInjector** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs:192
  - Methods: 1 (1 async, 0 sync)
  - Used in 1 files

- **FullTextLookup** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔍️query/🦀️component.rs:570
  - Methods: 0 (0 async, 0 sync)
  - Used in 1 files

- **HttpBody** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs:651
  - Methods: 1 (0 async, 1 sync)
  - Used in 1 files

- **JoinHandleLike** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🎭️actor/🦀️component.rs:703
  - Methods: 1 (1 async, 0 sync)
  - Used in 1 files

- **LogitsProcessor** (1 dyn uses)
  - Declared: 🧰️framework/🔨️modules/🧮️math/🎯️sampling/🦀️component.rs:2579
  - Methods: 8 (8 async, 0 sync)
  - Used in 1 files

- **MediaCache** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs:118
  - Methods: 2 (2 async, 0 sync)
  - Used in 1 files

- **MeshExporter** (1 dyn uses)
  - Declared: 🧰️framework/🔨️modules/🔺️mesh-engine/📦️packages/🦀️rust/📦️glue.rs:817
  - Methods: 2 (2 async, 0 sync)
  - Used in 1 files

- **MeshImporter** (1 dyn uses)
  - Declared: 🧰️framework/🔨️modules/🔺️mesh-engine/📦️packages/🦀️rust/📦️glue.rs:823
  - Methods: 2 (2 async, 0 sync)
  - Used in 1 files

- **PayloadSink** (1 dyn uses)
  - Declared: 🧰️framework/🔨️modules/🚪️io/🦀️component.rs:485
  - Methods: 1 (1 async, 0 sync)
  - Used in 1 files

- **PayloadSource** (1 dyn uses)
  - Declared: 🧰️framework/🔨️modules/🚪️io/🦀️component.rs:476
  - Methods: 3 (3 async, 0 sync)
  - Used in 1 files

- **ProjectionStore** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️component.rs:292
  - Methods: 6 (6 async, 0 sync)
  - Used in 1 files

- **PromptRegistry** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧭️protocol/🦀️component.rs:471
  - Methods: 2 (2 async, 0 sync)
  - Used in 1 files

- **QueryHandler** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️component.rs:211
  - Methods: 2 (2 async, 0 sync)
  - Used in 1 files

- **QuerySource** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔍️query/🦀️component.rs:544
  - Methods: 2 (2 async, 0 sync)
  - Used in 1 files

- **QueryableGraph** (1 dyn uses)
  - Declared: 🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️component.rs:80
  - Methods: 7 (7 async, 0 sync)
  - Used in 1 files

- **RandomAccessPayload** (1 dyn uses)
  - Declared: 🧰️framework/🔨️modules/🚪️io/🦀️component.rs:470
  - Methods: 2 (2 async, 0 sync)
  - Used in 1 files

- **RandomSource** (1 dyn uses)
  - Declared: 🧰️framework/🔨️modules/🧮️math/🎯️sampling/🦀️component.rs:1104
  - Methods: 8 (8 async, 0 sync)
  - Used in 1 files

- **ResourceRegistry** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧭️protocol/🦀️component.rs:374
  - Methods: 5 (5 async, 0 sync)
  - Used in 1 files

- **ResourceResolver** (1 dyn uses)
  - Declared: 🧰️framework/🔨️modules/🚪️io/🦀️component.rs:497
  - Methods: 2 (2 async, 0 sync)
  - Used in 1 files

- **RouterEffectHandler** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs:379
  - Methods: 1 (0 async, 1 sync)
  - Used in 1 files

- **Saga** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority/🦀️component.rs:439
  - Methods: 1 (1 async, 0 sync)
  - Used in 1 files

- **SamplingObserver** (1 dyn uses)
  - Declared: 🧰️framework/🔨️modules/🧮️math/🎯️sampling/🦀️component.rs:2641
  - Methods: 5 (5 async, 0 sync)
  - Used in 1 files

- **ServerModule** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️component.rs:161
  - Methods: 5 (5 async, 0 sync)
  - Used in 1 files

- **SessionStore** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️component.rs:437
  - Methods: 4 (4 async, 0 sync)
  - Used in 1 files

- **StopCondition** (1 dyn uses)
  - Declared: 🧰️framework/🔨️modules/🧮️math/🎯️sampling/🦀️component.rs:2624
  - Methods: 6 (6 async, 0 sync)
  - Used in 1 files

- **StorageBackend** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs:397
  - Methods: 3 (3 async, 0 sync)
  - Used in 1 files

- **ThreadSpawner** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🎭️actor/🦀️component.rs:696
  - Methods: 1 (1 async, 0 sync)
  - Used in 1 files

- **TokenSampler** (1 dyn uses)
  - Declared: 🧰️framework/🔨️modules/🧮️math/🎯️sampling/🦀️component.rs:2594
  - Methods: 3 (3 async, 0 sync)
  - Used in 1 files

- **TokenTextAdapter** (1 dyn uses)
  - Declared: 🧰️framework/🔨️modules/🧮️math/🎯️sampling/🦀️component.rs:1288
  - Methods: 3 (3 async, 0 sync)
  - Used in 1 files

- **ToolRegistry** (1 dyn uses)
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧭️protocol/🦀️component.rs:276
  - Methods: 2 (2 async, 0 sync)
  - Used in 1 files

### Plugin-declared NEW traits

- **BrepKernel** (8 dyn uses)
  - Declared: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️component.rs:140
  - Methods: 92 (92 async, 0 sync)
  - Used in 8 files

- **Element** (8 dyn uses)
  - Declared: ✏️s/🔨️modules/🏗️fem/⚙️engine/🏗️model/🦀️component.rs:85
  - Methods: 8 (8 async, 0 sync)
  - Used in 8 files

- **Sobject** (7 dyn uses)
  - Declared: ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🦀️component.rs:644
  - Methods: 37 (37 async, 0 sync)
  - Used in 7 files

- **Constraint** (5 dyn uses)
  - Declared: ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/⛓️constraint/🦀️component.rs:113
  - Methods: 4 (4 async, 0 sync)
  - Used in 5 files

- **MachineCatalog** (5 dyn uses)
  - Declared: ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️component.rs:282
  - Methods: 4 (4 async, 0 sync)
  - Used in 5 files

- **NationalAnnex** (4 dyn uses)
  - Declared: ✏️s/🔌️plugins/📕️norm/📄️artifact/🦀️component.rs:213
  - Methods: 9 (9 async, 0 sync)
  - Used in 4 files

- **Animation** (3 dyn uses)
  - Declared: ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎞️animation/🦀️component.rs:24
  - Methods: 9 (9 async, 0 sync)
  - Used in 3 files

- **CommandSink** (2 dyn uses)
  - Declared: ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🔄️fsm/🦀️component.rs:639
  - Methods: 1 (1 async, 0 sync)
  - Used in 2 files

- **Migration** (2 dyn uses)
  - Declared: ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🔄️fsm/🦀️component.rs:1303
  - Methods: 2 (2 async, 0 sync)
  - Used in 2 files

- **Part21Preamble** (2 dyn uses)
  - Declared: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🚪️io/📐️part21/🦀️component.rs:664
  - Methods: 1 (1 async, 0 sync)
  - Used in 2 files

- **SoftConstraint** (2 dyn uses)
  - Declared: ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🪶️soft/🦀️component.rs:11
  - Methods: 2 (2 async, 0 sync)
  - Used in 2 files

- **SolidExporter** (2 dyn uses)
  - Declared: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️component.rs:1475
  - Methods: 2 (2 async, 0 sync)
  - Used in 2 files

- **SolidImporter** (2 dyn uses)
  - Declared: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️component.rs:1481
  - Methods: 2 (2 async, 0 sync)
  - Used in 2 files

- **CiTest** (1 dyn uses)
  - Declared: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🔗️causal-internals/🦀️component.rs:493
  - Methods: 1 (1 async, 0 sync)
  - Used in 1 files

- **Compressor** (1 dyn uses)
  - Declared: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🎲️entropy-internals/🦀️component.rs:6044
  - Methods: 1 (1 async, 0 sync)
  - Used in 1 files

- **ScriptRuntime** (1 dyn uses)
  - Declared: ✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs:543
  - Methods: 1 (1 async, 0 sync)
  - Used in 1 files

- **SourcingModule** (1 dyn uses)
  - Declared: ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs:369
  - Methods: 5 (5 async, 0 sync)
  - Used in 1 files

- **TextRenderer** (1 dyn uses)
  - Declared: ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔤️text/🦀️component.rs:469
  - Methods: 1 (1 async, 0 sync)
  - Used in 1 files

### Assessment

**Scope classification:**

1. **Closed-set traits** (enumerable, fixed impls):
   - Core infrastructure: Backbone, BackbonePort, BackboneChannelPort, SpaceBackbonePort, OsBackbonePort
   - Storage backends: DbStorage family already known; new: BlobStore (architecture patterns)
   - Protocol handlers: ArtifactChannel, CommandSink, AuditSink, PayloadSink/Source
   - Runtime/execution: HostAsyncRuntime, GuestRuntime, ThreadSpawner, JoinHandleLike

2. **Open-set traits** (per-plugin extensible):
   - Domain-specific engines: BrepKernel, NationalAnnex, Element, Sobject, Animation
   - User-facing DSLs: QueryableGraph, Operator, Constraint, ErasedProjection
   - Plugin ecosystems: LogitsProcessor, RandomSource, TokenSampler, Migration
   - Extensible I/O: HttpTransport, HttpBody, Compressor, SolidExporter/Importer

## Section D: Standard Library / Language Trait `dyn` Residue

These are LEGAL and must remain in `dyn` form (async-incompatible by language design).

| Trait | Total Uses | Status |
|-------|------------|--------|
| Error | 6 | ✓ Legal (language trait) |
| Fn | 127 | ✓ Legal (language trait) |
| FnMut | 57 | ✓ Legal (language trait) |
| FnOnce | 24 | ✓ Legal (language trait) |
| Future | 84 | ✓ Legal (language trait) |
| Iterator | 8 | ✓ Legal (language trait) |

**Total std/lang dyn uses:** 306

## Section E: Method Signature Reality Check

**Question:** After the codemod, are trait methods `async fn` or still sync?

### Known families — method status

| Trait | Total Methods | Async | Sync | Status |
|-------|---|---|---|--------|
| Backbone | 3 | 3 | 0 | ✓ All async |
| BackbonePort | 2 | 2 | 0 | ✓ All async |
| CatalogStorage | 0 | 0 | 0 | ✓ All async |
| DbStorage | 7 | 7 | 0 | ✓ All async |
| GuestRuntime | 9 | 9 | 0 | ✓ All async |
| HostAsyncRuntime | 6 | 6 | 0 | ✓ All async |
| IndexStorage | 0 | 0 | 0 | ✓ All async |
| LeaseStorage | 0 | 0 | 0 | ✓ All async |
| PayloadStorage | 0 | 0 | 0 | ✓ All async |
| PluginApp | 47 | 47 | 0 | ✓ All async |
| SnapshotStorage | 0 | 0 | 0 | ✓ All async |
| SpaceMember | 25 | 25 | 0 | ✓ All async |
| WalStorage | 0 | 0 | 0 | ✓ All async |

### NEW families — top 10 by dyn usage (method status)

| Trait | Total Methods | Async | Sync | Status |
|-------|---|---|---|--------|
| Operator | 1 | 0 | 1 | ⚠️ 1 sync methods (E0053) |
| BrepKernel | 92 | 92 | 0 | ✓ All async |
| Element | 8 | 8 | 0 | ✓ All async |
| Sobject | 37 | 37 | 0 | ✓ All async |
| BackboneChannelPort | 2 | 2 | 0 | ✓ All async |
| Emit | 1 | 1 | 0 | ✓ All async |
| Constraint | 4 | 4 | 0 | ✓ All async |
| MachineCatalog | 4 | 4 | 0 | ✓ All async |
| BlobStore | 4 | 4 | 0 | ✓ All async |
| NationalAnnex | 9 | 9 | 0 | ✓ All async |

### E0053 Violations (sync methods in dyn trait)

**Found 3 traits with sync methods that are used as dyn:**

- HttpBody: 1 sync methods out of 1 total
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs:651
  - Dyn uses: 1 files

- Operator: 1 sync methods out of 1 total
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️component.rs:643
  - Dyn uses: 11 files

- RouterEffectHandler: 1 sync methods out of 1 total
  - Declared: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs:379
  - Dyn uses: 1 files

## Conclusion

**The six-family design list is INCOMPLETE.** It covers only 13 of 95 first-party trait objects used as dyn.

**82 additional trait families need conversion** from trait object dispatch to enum-based dispatch:

- 64 declared in framework
- 18 declared in plugins/semes

The larger collection splits between:
- **Closed-set infrastructure traits** (~30): fixed architectural components with bounded implementations
- **Open-set extensible traits** (~52): per-plugin or domain-specific, requiring generated dispatch mechanism

