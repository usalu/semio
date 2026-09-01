# OS-Kernel R17 Schema-First Native Packet

## Current Boundary

Six native laws are mounted under the actual Directory native module and Store test module. Production implementation remains unchanged: the wrong Directory import, the two non-Send erased codec slots, and the unsafe detach ordering are still present. No Cargo, Rust test, source oracle, or new runtime behavior was executed during this packet. Scoped git diff --check returned exit0. A final process inspection found no cargo, rustc, or cargo-nextest process.

The prior actual WGPU R17 compiler failure remains the only compiler evidence: four OS-kernel library diagnostics, zero native tests executed. Resident R7 remains a separate pre-Cargo taxonomy-discovery failure; these source additions do not repair it.

Read in full before edits: mutation ticket 📓️store-r17-codec-backbone-coordination.md and master 📓️os-kernel-r17-owned-repair-proposal-2026-08-28.md. FreshFieldDecoder, FreshVcsAuthority, registry/envelope grammar, and unrelated fixture contracts remain excluded.

## Authored Native Tests

| Selector | Count | Real source and assertion | Present execution status |
| --- | ---: | --- | --- |
| directory_native_runtime_identity_ | 2 | Actual UreqStreamingHttpTransport constructor must accept services-owned TokioHostRuntime; real transport keeps the original runtime/compute Arcs and scope across 1/2/3-worker injected pools. A separately constructed runtime over the same pool is not the original Arc. Cleanup closes the unused scope and shuts down the original pool before assertions. | Uncompiled/unexecuted; current wrong import still blocks the library. |
| document_codec_native_send_ | 2 | require_send borrows the actual returned compile_dsl/print_mirror future from ArtifactCodec::of<DemoSnapshot,DemoMutation>. Each then awaits the real codec, roundtrips null/i32MIN/4/i32MAX, and checks native serde/DSL/PACK values. | Uncompiled/unexecuted; current erased Future contracts intentionally fail Send checking once reached. |
| backbone_detach_refusal_ | 2 | Actual initialized Store, exact original MemoryBackbone queue with 257 bytes, original Vec pointer/capacity/contents, full real displaced-owner reservation or u64MAX generation. catch_unwind retains Store and returned-root destination outside the callback. After observation, the exact reservation is released, fixture generation reset only for cleanup, shared peer released while the local queue owner remains, returned backbone typed-retired if any, and Store explicitly closed before outcome assertions. | Uncompiled/unexecuted. Expected full-destination descriptor loss and generation-overflow panic are source predictions, not new actual RED results. |

Native leaves are canonical 🧪️tests/🦀️.rs beneath each domain listed below. Include mounts add only5 lines in Directory/client/🦀️component.rs and8 lines in Store/🦀️component.rs. No production imports, slots, thunks, methods, limits, pools, or behavior were changed. Native error or cleanup failure must be reported distinctly if observed; cleanup is not proof merely because it was authored.

The detach fixture lists five additional boundaries explicitly as pending, not executed coverage: zero/short physical grant, shell allocation refusal, construction unwind, commit-transfer unwind, and exact-one session detach request. No allocation admission claim is inferred from the initial two refusal tests.

## Canonical Source Registration Request

All commands belong to existing @semio-tech/framework-os-kernel, take zero arguments, and execute only the corresponding lazy domain oracle. Parent is coordinating taxonomy ownership of package 📜️script.ts, 📋️project.json, and launch rows. Those files were not edited here.

| Command | Domain script relative to OS package | Export |
| --- | --- | --- |
| test-directory-runtime-source | ../../🔨️modules/📇️directory/🔌️client/🪪️runtime/📜️script.ts | testDirectoryRuntimeIdentityFixture |
| test-codec-send-source | ../../🔨️modules/🏪️store/📦️codec/🧵️send/📜️script.ts | testNativeCodecSendFixture |
| test-backbone-detach-source | ../../🔨️modules/🏪️store/🔗️backbone/✂️detach/📜️script.ts | testBackboneDetachFixture |

Each domain owns 🧬️schema/🔣️.json and 🧪️tests/🔣️.json, authored before its Rust tests. Directory uses strict Ajv plus independent Lodash reference identity, including equal-looking foreign owners. Codec uses strict Ajv, Lodash clone/semantic equality and Buffer i32 bounds; this cannot prove Rust Send. Detach uses strict Ajv, Lodash identity and Buffer's exact u64 maximum to check the two refusal models; this cannot prove real allocation or Store ownership. Source-only outputs name their scope and pending native execution. Scripts are not yet executed or registered by this lane. Existing scalar-wire routing is untouched.

## Native Routing Boundary

These six laws live in semio-framework-os-kernel cfg(test). The source-released WGPU test-native route compiles OS-kernel as a dependency library and cannot execute these tests. The existing OS-kernel :test router currently calls raw cargo test --manifest-path Cargo.toml --lib plus supplied arguments, not the central budgeted nextest route. Parent/taxonomy must schedule the correct owner-crate gate and preserve the sync+ureq features already requested by the actual WGPU dependency; no feature removal or new runtime is proposed. No new native command has been dispatched.

After actual test-first diagnostics, the smallest Directory production repair is moving only TokioHostRuntime from the async import to the already-imported services module. Send repair is limited to compile_dsl/print_mirror and their nested thunks, with concrete generic bounds determined by compiler errors. ActorTurnFuture stays Send; edit_text/apply_ops contracts are not broadened without their own consumer evidence.

## Complete Textual Caller Census

Repository-wide tracked/nonhidden Rust text was scanned using rg for the four exact codec invocation patterns, detach_backbone, and awaited detach. This is a source-text census, not a whole-program call graph. The two production erased Send consumers remain FolderEndpoint::read1263 and ::write1277. edit_text invocation was found only in a Store fixture; apply_ops_binary has a distinct production Plugin host5142 consumer and stays outside this contract change. Codec identity comparison uses all four function-pointer slots and must remain coherent.

The additional detach forwarding scope must be coordinated before a production cutover:

- Store detach15687 and Space meta wrapper18214 currently expose the original Option<Backbones>.
- SyncSession detach896 sends a request before Store refusal, then discards command/event owners. No .detach().await production call was found by the exact textual pattern.
- OS host785 forwards and discards the return. Its surrounding workflow adapter also contains separate older API shapes; no unrelated adapter repair is authorized by this packet.
- PluginApp trait11677 and VcsArtifactApp24445 use async void, clear cache, and ignore Store return; runtime29866 uses resolve_ready then returns Ok. Those authored signatures/callers cannot remain as compatibility paths after the retained transition.
- ProgramBridge285/529 and Shell2981 are outer detach transport paths; their error and original instance authority must survive the eventual authored caller join, not be silently treated as a Store-local change.
- Existing Store detach23791 / Space host24511 / Plugin35227 tests must be migrated coherently after actual retained API design, not suppressed.

### Exact Admission And Ownership Design Constraints

The existing displaced-retirement root is the destination; no second spill queue. Its reserve_owner_slots already binds slot/generation/count, but its current Box<dyn ErasedSnapshotRetirement> insertion moves an owned Box by value. A preparation owner must remain structurally in the original Store while obtaining the exact empty backbone retirement shell and target slot, before the original backbone or descriptor moves. Construction refusal/unwind must leave the original Store root reachable. The accepted transfer must move into the preinstalled shell, with a separate bounded shell/page release after typed descendants.

Current replace_backbone_retained allocates Box only after replacing the source. Current detach clears the descriptor before bump; bump reserves cursor/revision slots, increments generation unchecked, clones cursor strings and may allocate/reconcile history. Merely changing detach to call this helper or moving bump earlier does not establish transactional refusal or bounded work. Detach does not change semantic cursor contents: a narrower staged generation/descriptor transition must preserve the existing revision meaning and original read-lease authority, and avoid whole cursor cloning. Exact descriptor String retirement belongs with the backbone transition, not a dropped local.

Original parent funding is still a concrete dependency, not supplied by a byte integer: ArtifactStoreInitializationOwnerCatalog::admitted_bytes observes retained capacities after allocation and is not an affine resident permit. The current displaced root preallocates1024 boxed-owner slots but does not itself prove additional shell allocation funding. Therefore no new grant-shaped API is mounted here and no automatic allocation admission is claimed. Parent/destination identity, real shell funding, and bounded physical move/release accounting must be joined before live detach admission.

SyncSession must retain its cmd_tx/events and an exact unsent request under its own installed operation owner until the Store transition succeeds and the actor request is accepted. Clearing a receiver may drop queued ArtifactEvents; it cannot be the cleanup implementation. The eventual Plugin/host/Space forwarding must return the typed progress/refusal/fault and drive the same original parent-owned transition. No raw original backbone return, async-void ignore, resolve_ready discard, cloned disposer, unsafe Send, local executor fallback, or generic whole-payload Drop is an acceptable fix.

## Exact Source Hashes

```text
972e8d8b2186c35a3a8e9c4ab131ef71380934a07282bb9e93faff54767acb92  🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🪪️runtime/🧬️schema/🔣️.json
552af8b4eac227e8d1771037bb5f238add3aa6c7527faf5e03536b11cb4298b2  🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🪪️runtime/🧪️tests/🦀️.rs
e2b0e4c8162e20e509dbb96ea4aaf124bf7d8a3aea4847b2e4bc4eb6e6db6b67  🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🪪️runtime/🧪️tests/🔣️.json
689362361df4fb5d706de21a18bb27c3cf7afa2ff2a316d6b045e0faf5f9a1ab  🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🪪️runtime/📜️script.ts
dc571710d6ca6e17475a86e444457e7c3da818bbc26e57e740169d32f019ff23  🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧵️send/🧬️schema/🔣️.json
ca6a052972a28a0d70618e19b0fa188a07b99321f755f6b2dedc98fcd537459e  🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧵️send/🧪️tests/🦀️.rs
f0e965e2d8f292371b893329de306c875750932510f63ba6632024b24040f560  🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧵️send/🧪️tests/🔣️.json
2924cbb556b457da9e53d98b6d2f2f2f3c03ea97085d8f85023154411907d669  🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧵️send/📜️script.ts
6e727ae7851d23a2606f8b1bb5750c4588da05c899a2d6cfac3d82d59f096a3a  🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/✂️detach/🧬️schema/🔣️.json
3e714650739cce80d3bd2c0f8cdcbf9951ba4a94b0eb683a7af8340bd290a977  🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/✂️detach/📜️script.ts
88492fcc126c50628d67ebbd80e39bd766c1204729d0ef987c60dfecacddfc65  🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/✂️detach/🧪️tests/🦀️.rs
7620dffca847c9ff585cfc0bff838006a65c3e01118b9ed7e36e33e4ddf46079  🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/✂️detach/🧪️tests/🔣️.json
95ce0b165cfa422189765e4adc5974ffccdfd0a2fedb157a57829fc3ce51921e  🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️component.rs
7450f9d6837055d0766a55c5fc98aae22d068ac813acda09c1385a1df48d4c9c  🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs
```

## Raw Caller Inventory

```text
./🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs:785:        pub fn detach_backbone(&mut self) {
./🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs:786:            self.inner.detach_backbone();
./🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs:2304:    //!    `ArtifactHost::close(&id)`, then `store.detach_backbone()` /
./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs:285:    pub fn detach_backbone(_instance_id: u32) -> Result<(), String> {
./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs:529:    pub fn detach_backbone(&self, instance_id: u32) -> Result<(), String> {
./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs:531:            ProgramBridgeBackend::Wasm { .. } => wasm_program_exchange::detach_backbone(instance_id),
./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs:2981:                let _ = plugin.detach_backbone(channel.instance_id);
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:900:        self.store.detach_backbone().await;
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:1263:                    let (pack_files, _dsl_mirror) = (codec.compile_dsl)(&text_files.dsl, &text_files.ops).await.map_err(|error| error.to_string())?;
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:1277:                    let mirror = (codec.print_mirror)(pack, spr).await.map_err(|error| error.to_string())?;
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4728:                        let (pack_files, _dsl_mirror) = (codec.compile_dsl)(dsl_text, ops_text).await.unwrap_or_else(|error| panic!("fixture {} compile_dsl: {error}", fixture.name));
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:9262:        && std::ptr::fn_addr_eq(left.compile_dsl, right.compile_dsl)
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:9263:        && std::ptr::fn_addr_eq(left.print_mirror, right.print_mirror)
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:9264:        && std::ptr::fn_addr_eq(left.edit_text_from_envelope, right.edit_text_from_envelope)
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:9265:        && std::ptr::fn_addr_eq(left.apply_ops_binary, right.apply_ops_binary)
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:15687:    pub fn detach_backbone(&mut self) -> Result<Option<Backbones>, VcsError> {
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:18214:    pub async fn detach_backbone(&mut self) -> Result<Option<Backbones>, VcsError> {
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:18215:        self.meta.detach_backbone()
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:23791:        store_a.detach_backbone();
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:23821:        let (pack_files, dsl_mirror) = (codec.compile_dsl)(&text_files.dsl, &text_files.ops).await.expect("codec compile_dsl");
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:23824:        let mirrored = (codec.print_mirror)(&pack_files.pack, &pack_files.spr).await.expect("codec print_mirror");
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:23843:        let edit_text = (codec.edit_text_from_envelope)(&op_envelope).await.expect("codec edit_text_from_envelope");
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:24511:        host_a.detach_backbone().await;
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧵️send/🧪️tests/🦀️.rs:18:        let future = (codec.compile_dsl)(&text.dsl, &text.ops);
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧵️send/🧪️tests/🦀️.rs:35:        let future = (codec.print_mirror)(&files.pack, &files.spr);
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/✂️detach/🧪️tests/🦀️.rs:29:        match store.detach_backbone() {
./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:5142:        match (codec.apply_ops_binary)(&self.pack, &self.spr, &ops).await {
./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:11677:        async fn detach_backbone(&mut self);
./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:24445:        async fn detach_backbone(&mut self) {
./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:24446:            self.store.detach_backbone();
./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:29866:            resolve_ready(instance.app.detach_backbone());
./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:35227:            app.detach_backbone().await;
```
