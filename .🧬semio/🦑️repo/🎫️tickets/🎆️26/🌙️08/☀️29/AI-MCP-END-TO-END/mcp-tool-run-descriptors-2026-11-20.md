# MCP ToolRun descriptor refresh

The committed Dag, Reasoning, Remodel, Trinity and WFC JSON descriptors omit the required `receiving` field on nine tool runs. The Dag, Reasoning and Trinity reorganization tools inherit the shared graph layout producer’s `ToolRunReceivingLimits::REPORTS`. Remodel’s reconstruction tool and all five WFC fill tools declare the same policy directly. The older Dag, Reasoning, Remodel and Trinity Pack files also disagree with their JSON mirror. They carry App Channel version 24 while the JSON carries version 25. The formats differ on some I/O metadata as well.

The focused ticket test failed against all five committed pairs before regeneration. Ajv rejected the absent receiving object. The test checks the exact run job roster, the receiving contract and the published policy fixture, then verifies strict JSON/Pack parity with the repository’s owned reader.

## Norm

The initial authored Norm JSON still declared `vec3` for four action parameter schemas. The focused case reproduced that failure. The current emitted model names `semio:norm` and carries App Channel 25. Its owned prototype publication binds a 242,674,717-byte component with SHA-256 `cd259d5eb1b41cab1f25210a5ebb2a9a684cae2efb42c82a505c0ed780999c13` to semantic receipt `50594c65cf339de93a9920933d1fc39343df1c57cc01ed02a0918e96a3d87068`. The matching 796-unit compiler provenance names 1,472 authored Norm files in the composition and plugin roots. All matched the current checkout byte-for-byte.

The committed source pair now copies that exact owned emitted model pair. The semantic verifier confirmed both formats, exact identity and semantic SHA after publication. Independent Ajv validation passed, and the focused Norm test passed on the committed files. The emitted descriptor retains eight apps. A separate debug emitter replay was cancelled after validation of its original model input because the unoptimized interpreter averaged about 2.5 million fuel per second. Publication used the completed, receipt-bound prototype emission.

The official MCP SDK’s live native catalog discovery admitted Norm after the source refresh. The original 12 rejected-descriptor log records fell to 10. The remaining records name Dag, Reasoning, Remodel, Trinity and WFC twice each, all for the required `receiving` field.

## WFC

The warm locked/offline WASIP2 build produced a 732,690,303-byte component with SHA-256 `622d6fb5a4658b0996a79227c402c4859660bf5d50d7a42cb3382267f636f430` after scoped optimization of the WFC owner and artifact crates. The owned interpreter admitted the 338,157,646-byte component produced by removing only optional core `name` sections. Every retained region was compared by SHA-256. The exact interpreted component hash is `bcc00c0182e742201f4943bc5760a206eee02adebb61bfe8b7e8343383bcadf1`; the registered emitter hash is `8beb45bd1b64cb6ac4755d9ceeab9275457c23a844c74c179458a7838508642a`.

The owned emitter completed `describe` and its actual `s.wfc.bitmap` codec probe. The published JSON and Pack match the emitted semantic digest `e87360531b895c739aea4c8e458e5e11f527daabd43d5b9085c9d242afd3d5cf`. The focused independent Ajv and policy-fixture check passed before and after source publication. It verified all five declared `receiving` records and the repo verifier confirmed canonical JSON/Pack parity.

## Producer dependency

The closed `PluginApp` dispatch macro expands the declared signature for typed operation result pages. Its signature names `semio_framework_value::RetainedAllocationEffect`, which requires each owning crate’s direct workspace dependency. The equivalent Note declaration exposed the missing dependency during WASI compilation. Dag, Reasoning and Trinity lacked the same direct dependency. Their existing Hub lock entries now include the matching additive edge. Remodel and WFC already declare it.

## Native source producer fallback

A ticket-local fallback is ready when the shared host build lane is free. It builds a single current owner composition and installs the bundle in the first-party native plugin runtime with the same mounted owner policy as the component. It calls the exact `describe_plugin` implementation and owned Pack schema-hash probe, then verifies kind identity and typed Pack canonicality. It writes only a scratch JSON/Pack pair and validates parity with the owned reader. The focused Ajv check can then verify that owner before source publication. Driver syntax passed. The first Dag launch resolved newer cached registry pins and stopped early. The launcher now seeds the isolated generated lock from the current Hub lock. Cargo retained the Hub pins and added only the scratch harness package. The pinned Dag build stopped during dependency compilation so the live MCP host fix could take the native lane. Its warm files were retained.

## DAG source contracts

The locked/offline Dag and Reasoning build failed at the shared infinite-DAG VCS artifact. Its SQLite methods used the superseded direct return value. They now hand off through the caller-owned output slots while keeping the authored preflight budget, record projection, reconstruction and retirement. Subset validation uses the existing owner-aware shared implementation.

The existing DAG binary format uses authored ordinals. Its receiver now uses the corresponding first-party ordinal operation macro with owned DSL receiving on the aggregate, the existing fourteen mutation leaves and their nested schema fields. Property bags and expanded paths receive through the existing native owners. The Dag and Wires parent document codecs now declare their empty receiving vocabularies through the same refusal-producing ordinal contract as other empty framework lanes. The roster test now checks every existing language-agnostic wire witness through the original receiving driver as well as the binary/JSON oracle. The independent Remodel and Trinity guest build is active. A one-job pinned native run of the DAG fixture started and stopped during the shared replication prerequisites when the typed MCP host needed the lane.

## Files

- `🌎️hub/🧩️compositions/📕️norm/🧩️component/🔣️.json`
- `🌎️hub/🧩️compositions/📕️norm/🧩️component/🛂️.descriptor.semio`
- `mcp-native-guest-build.ts` in this ticket, for scoped owner selection
- `mcp-descriptor-pairs.test.ts` in this ticket, for the independent schema and exact Pack checks
- `mcp-tool-run-descriptors-2026-11-20.md` in this ticket
- `🌎️hub/🧩️compositions/🕸️dag/📦️packages/🦀️rust/Cargo.toml`
- `🌎️hub/🧩️compositions/💡️reasoning/📦️packages/🦀️rust/Cargo.toml`
- `🌎️hub/🧩️compositions/🔱️trinity/📦️packages/🦀️rust/Cargo.toml`
- `🌎️hub/Cargo.lock`
- `mcp-native-descriptor.ts` and `mcp-native-descriptor.rs` in this ticket
- `🌎️hub/🧩️compositions/🀄️wfc/🧩️component/🔣️.json`
- `🌎️hub/🧩️compositions/🀄️wfc/🧩️component/🛂️.descriptor.semio`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧪️tests/🔬️dag-direct/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🚪️io/💾️binary/🧬️mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🦀️.rs`
- `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🗂️properties/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/↔️move-node/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/✂️disconnect-nodes/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/✏️rename-node/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/➕️create-node/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/📐️resize-node/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🔀️reorder-nodes/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🔁️replace-node-kind/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🔗️connect-nodes/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🔡️change-node-abbreviation/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🔤️change-node-name/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🖼️change-node-icon/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🗃️replace-node-properties/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🗑️delete-node/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🧮️change-node-operator-kind/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🧬️schema/🖼️presentation/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🧬️schema/🖼️presentation/🗂️expanded/🦀️.rs`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`

## Trinity Jack graph generation

The first pinned Remodel and Trinity guest run reached `semio-s-artifact-trinity-jack-model` and stopped because the registered Nakagin manifest Rust source was absent. I returned the WASI compiler lane to the Note startup verification after this failure. The focused Dag native fixture test also yielded the host compiler lane to the typed host fix before the tests executed.

`@semio-tech/trinity-jack-rs:graph-generate` stalled waiting for the managed graph service. Direct package command dispatch required `SEMIO_SCRIPT_PROVIDERS`. The direct native fallback invoked the same registered graph manifest catalog reader, renderer, and publisher under `repo tools exec bun`. It rendered one supported Nakagin graph manifest, then compared all five published files byte-for-byte against the actual projection. The formerly missing Rust file is 44,715 bytes.

Canonical generated inputs restored at `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🤖️generated/`. The descriptor pair still awaits a successful current owner component build and execution.

`mcp-emit-owned-descriptor.ts` is the single-owner WASI emission runner for Dag, Reasoning, Remodel and Trinity. Bun resolved and bundled 358 modules. The runner hashes the exact compiled component and the emitter, reuses the verified optional name-section removal only at the interpreter size ceiling, and accepts only the exact owned semantic JSON/Pack pair for the selected package. Its output remains in ticket scratch pending the separate independent Ajv and live source checks.

## Live workspace recheck

The six-case independent Ajv and owned pair suite was rerun against the live catalog after the Note WASI build finished. WFC and Norm passed. Dag, Reasoning, Remodel, and Trinity failed on their known missing authored receiving data. The scoped Remodel WASI build then resumed from the warmed pinned guest target while the Note model emission and live host build used the other lane.

The 21 November single-owner Remodel build reached the component crate after 910 logged compiler starts. To make link space I compressed five retired catalog logs with complete readback verification and reclaimed 33,427,942 bytes. With coordinator approval I removed 12 scratch `.rlib` and `.rmeta` files from six superseded WFC-only unoptimized package variants. Fingerprint values matched the known opt-0 package profile `16253270031121296954` and every affected package still had both outputs at the opt-1 profile `11837183133356902558`. No WFC packages were in Remodel's lock dependency closure. The six retired pairs totaled 800,282,375 bytes. The link-time filesystem moved from 2,116,632,576 to 2,917,130,240 available bytes immediately after that cleanup.

## Remodel owned emission

The pinned native WASI fallback built the current `semio-hub-remodel-component` in 64 minutes and 33 seconds. The exact linked component was 581,921,050 bytes with SHA-256 `fd44533b7f4cc491698f153265756754e1f8c965e62563178131b4af4909b2f8`. The interpreter caps inputs at 512 MiB, so the ticket runner removed only optional embedded core symbol-name sections and compared the SHA-256 of every retained region. The actual emitter consumed 230,631,358 bytes with SHA-256 `e3cf0b9a28b296ddc2dbaa31b87b2f5042080bb54b81edb41a7d7d401572eb55`. The managed owned emitter SHA-256 was `8beb45bd1b64cb6ac4755d9ceeab9275457c23a844c74c179458a7838508642a`. It executed both the descriptor export and the actual `remodeling.scene` codec probe.

The exact semantic pair SHA-256 was `55c0f50cbb77c8992e91078ba631a1b097d6bc37b0a9051e7b3e28dd455ae3b4`. Ajv accepted the current descriptor schema and `remodeling.reconstruction.run` matched the independent language-neutral receiving policy fixture. The owned verifier confirmed the canonical Pack/JSON equality and exact package identity. Staged and live source both passed all five scoped assertions and the two files matched the exact owned output byte-for-byte. I rehashed and removed only the two redundant raw WASM aliases after publication. Available filesystem space moved from 3,440,226,304 to 4,046,901,248 bytes.

Updated `🌎️hub/🧩️compositions/📸️remodel/🧩️component/🔣️.json` and `🌎️hub/🧩️compositions/📸️remodel/🧩️component/🛂️.descriptor.semio`. The six-case live suite now passes Remodel, WFC, and Norm. Dag, Reasoning, and Trinity still fail on their missing receiving declaration. The managed browser model publication, component build stages and runs, and ticket models yielded no owned Dag, Reasoning, or Trinity pair. The warmed WASI lane was handed to the next exact WFC build after the Remodel compiler exited.

## Dag prebuild gates

The exact 22 modified Rust sources parse with the repository-pinned nightly 2026-07-20 compiler in parse-only mode, without starting Cargo. Read-only verification confirmed the live shared SQL snapshot output-slot signatures, the real Dag ordinal operation binding, the receiving fixture extension, and the direct value dependency in Dag, Reasoning and Trinity manifests and Hub lock. This is a source parse and declared-contract check. The Rust fixture test and a complete Dag guest build still need a compiler lane.

The official MCP SDK also reran live discovery over the existing native gateway after Remodel publication. Initialization took 7,044 ms, and the SDK verified 28 tools, 7 resources and 5 prompts. The six remaining registry diagnostics were exactly two each for Dag, Reasoning and Trinity, all on the required `receiving` field. The SDK process exited zero.

## Dag native fixture rebuild

The pinned one-job native fixture resumed after the SourceReady host finished with at least 2.5 GiB free. It first hit E0599 in `semio-os-store-one-item-publication` for `OpBinaryReceivingCursor::close_work_demand`. The consumer imported the trait. The live shared protocol source had received the method while the already-built trait dependency was stale for that run. The same fixture command recompiled and passed publication against the current source. No overlapping edits were made to that upstream shared protocol.

The retry then stopped in the required test-only `semio-os-space-testing-sqlite-native-caller` with 13 compile diagnostics. That harness still passes a bare allocation callback where the original loan now requires `NativeSendAllocationLoan`. Its old one-argument retirement calls also lack a paid effect and do not use the current decode poll or encode reply protocol. The current SQLite companion demonstrates the real allocation event authority, Source-bound work and construction recipients, and distinct paid close drivers. Preserving the harness's observer and denied-grant assertions requires all three migrations. The existing Dag fixture remains intact in the unit-test target, which directly depends on that harness and has no separate registered fixture target.

The native lane subsequently ran the same owned Dag production library as a single-job build, separate from the still-blocked fixture. The WASI compiler remained available to the end-to-end Note/WFC work.

## Dag stock retirement owners

The pinned production Dag library build has verified the actual `semio-framework-artifact-infinite-dag-vcs` owner and its migrated SQLite snapshot and original ordinal receiving implementations. The thin `semio-framework-artifact-infinite-dag` wrapper then failed on 14 diagnostics, all in three hand-written delegates using obsolete retirement grants and return values. The current no-draft producer already constructs these stock shared and owned retirement factories directly through the admitted document source constructor.

Dag now registers the real `SharedValueRetirementFactory<DagSnapshot>`, `OwnedValueRetirementFactory<DagSnapshot>`, and `OwnedValueRetirementFactory<DagMutation>` through that same admitted source shape. It computes the admission birth quote from the selected concrete factory types. The existing neutral fixtures and the independent serde_json oracle remain. The retirement assertions now exercise issued original snapshot storage, the actual paid stock replies, bounded closure, preserved payload and alias identity under a denied close, exact measured allocation receipts, and full physical conservation. The original three denied birth-grant axes remain explicit.

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🧪️tests/🔬️unit/🦀️.rs`

A subsequent Cargo verification is pending while the native and WASI compilers are reserved for Note and the corrected host. The exact Dag unit fixture still requires a live migration of the unrelated SQLite test-native-caller harness.

## Native descriptor runner readiness

The 24 exact Dag changed Rust sources parse with the pinned nightly after the stock retirement factory change. The source comparison verified the actual installed Dag, Reasoning and Trinity native bundle functions and `INTERACTIVE_COMPONENT` policy against their live Wasm component export entries. The managed emitter's documented first-owned declared artifact codec gate matches the native runner check. The Hub composition production sources have no platform-specific authored-app compile gates.

The initial native runner attempt was stopped after it resolved registry dependencies from an unseeded work manifest. It selected a different Syn and Serde. The second attempt seeded the lock from the authored Hub workspace, added the actual `semio-plugin-describe` package and was preempted during proc-macro compilation. The saved resolved lock has no registry package/version outside the current Hub lock. Neither attempt reached a plugin owner or emitted a descriptor. Each subsequent native emitter invocation should seed the current Hub lock and run the actual Bundle/codec producer for one owner in the warm one-job target, when the corrected host releases it.

## Trinity Rewriting source gate

The Rewriting snapshot decode still returned its original as a function result. The current shared SQLite host and native record decoder require a caller-owned output slot and a mutable three-argument constructor. A portable source check reproduced the three missing contract points, then passed all three after both Rewriting methods forwarded the original output and bound the same declared envelope and record constructor. This is a source gate pending a complete Trinity build.

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs`

During the one-owner native Dag descriptor build, the coordinator authorized removal of the 13 remaining superseded opt-0 WFC-only scratch pairs. I checked both exact profiles and the intact optimized pair for each owner and removed only their 26 old library/metadata files, totaling 437,983,795 bytes. APFS available space immediately moved from 2,104,934,400 to 2,580,254,720 bytes. Current Dag artifacts and accepted Note/WFC models were preserved.

## Plugin window export ownership

The first completed native descriptor compiler attempt reached `semio-plugin` and stopped with E0432. Three existing public functions remained listed on the old app component import. The directly depended-on window transient crate defines all three, and Procedural still calls their public `semio_plugin` paths. The public export now points directly at their window owner. A focused source check resolved each existing call to its public owner before restarting the same pinned native Dag command.

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`

## Native image builder metadata check

The warm Dag retry encountered E0599 for `ImageBuilder<HasAlt>::try_source_body` in the Window Transient crate. Its direct dependency is the native `semio-ui-contract` facade, which re-exports the real builder. The live method is part of the unconditional typestate implementation. The newest dependency metadata at that earlier failure had been built before the live method appeared.

The canceled next retry had rebuilt both native variants. Pinned Rustc checked the exact Window Transient builder expression directly, including the `ImageBuilder<HasAlt>` type and the source carrier call, against the refreshed umbrella metadata `5dcda933f0985128` and `61449c5787995912`. Both passed. Their actual unconditional public builder dependencies are `e99d815504f4fb7c` and `af9cf71cb2e058ae`. The older preserved native umbrella `ede7773fb5466ad5` with builder `eeb078c314ebae49` produced the same E0599 on the same probe. The three facade metadata records contain empty library feature lists. The independent metadata check used no Cargo process and wrote its small inputs and proof under ticket scratch.

## Later catalog proof

The fresh WFC build's owned emitter again probed `s.wfc.bitmap` and returned semantic SHA-256 `e87360531b895c739aea4c8e458e5e11f527daabd43d5b9085c9d242afd3d5cf`. An independent owned verifier recomputed that digest on its current model. Both emitted files equal the live committed WFC JSON and Pack byte-for-byte.

The Dag retirement fixture now observes real allocation and release through the registered allocator test dependency. Pinned Rustc typechecked the exact fixture's paid-retirement helpers and shared alias steps against the built Dag VCS metadata and its own value crate dependency hash `4c862c545305fb53`. Metadata compilation succeeded without Cargo. The unit-test target remains pending because it needs the stale SQLite test harness migration.

The same pinned Rustc metadata gate then compiled the exact complete live production Dag crate root, including the stock retirement owner and all other production modules. The real selected dependencies were Dag VCS `f650f5bfc4801cf3`, OS kernel `9075c555c1310f73`, value `4c862c545305fb53`, directed presentation `665264eddcccf6ea`, and graph manifest `f0bff07603d1a4b3`. It exited zero. This establishes source type compatibility while the complete native bundle and runnable fixture remain pending.

## Warm Dag bundle status

The updated native graph compiled Window Transient and the public plugin facade. The first long attempt then failed in Terrain after its Cargo process had captured a graph without the new Pack JSON edge. The live Terrain manifest and both authored locks already held that exact edge. The freshly seeded retry resolved it, compiled the Terrain crate, and went on to compile both the Dag VCS and complete Dag production library with Cargo.

After 596 reported compiler starts the retry reached the aggregate Stdio Semio package. Available APFS bytes fell from 813,248,512 to 596,795,392 over a short interval. I stopped only the exact native descriptor runner with exit 130 before that late aggregate allocation. Its warmed compiler outputs remain available. Dag, Reasoning, and Trinity descriptor emission remain pending.

## Remaining owner preparation

The current Hub lock closure for Reasoning adds eleven package names over Dag, including ten Wires packages. Trinity adds 101 package names, including 23 Trinity artifact packages. These are lock graph upper bounds for a native compile. Both checked Hub manifests declare the generated dispatch value dependency. The Reasoning Wires and Trinity Jack tools both derive their actual layout-run receiving policy from the shared `REPORTS` producer.

The Reasoning editor declares `graph.wires` with schema `reasoning.wires.identity.snapshot`. Trinity's two editors declare `graph.trinity` with schema `trinity.graph` and `text.rewriting` with schema `trinity.rewrite.rule`. The exact native emitter checks at least one declared, nonzero document codec hash and performs canonical JSON/Pack equality before the independent Ajv fixture can admit a publication.

Jack's five generated files are present. Rewriting's registered graph manifest projected five files and its two native Rust includes were missing. The actual catalog reader and graph renderer generated the two required Rust files, and an independent readback confirmed their exact projection bytes.

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🤖️generated/📇️registry/🦀️.rs`, 413 bytes, SHA-256 `3354987420c826d526fb8a8c36cc45b205ac6c2fa6a58e3cdcd6789a71e04bb0`.
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🤖️generated/🫲️rewrite-lhs/🦀️.rs`, 11,258 bytes, SHA-256 `3f199da1a43c7c068d27c05743395ed0c82b9872c21ad4870f428986fe486ed8`.
