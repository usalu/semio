# Hydration Producer Extraction Census

Read-only future planning on 2026-10-14. HEAD: `7c8af563c74abe9a1c8b0f751224c7201311dfa3`. Read Root's current hydration frontier first; preserved the earlier Source audit unchanged. No tests, runtime validation, Cargo, native operations, application edits, Git mutation, or Source schema mutation performed. Concurrent working-tree changes mean this census is not an immutable 7c8af563c74abe9a1c8b0f751224c7201311dfa3 certification.

## Defining Boundary and Constructors

The hydration defining file is `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs`. It defines the retained body at 148, `from_decoded_pack` at 207, `from_pack` at 227, private `new` at 243, executable impl at 202, retirement/demand impls at 1041/1088/1213 and Drop at 1203. Both public constructors delegate to private `Self::new`; there is also an actual test struct literal at hydration unit tests line 270. These are distinct migration surfaces, not additional runtime instances.

Two production constructor expressions were discovered: InitialMemberStoreOpen BeginHydration at operation line 798 uses `from_decoded_pack`; plugin maintenance ladder HydrateParent at line 996 uses `from_pack`. The latter's retained field is `ActiveDocumentArchiveLoad`'s document module line 1630. Its exact catalog comes from `A::build_document_store_owners(grant)`; app-level selection must be explicit too. Simply adding a C bound to member opening will leave this real parent archive entry point unresolved.

Test constructor expressions: hydration unit 27; Store unit 11649; Store supersede-replay 622/699/733; SpaceHistory sqlite snapshot tests 40; collection sqlite snapshot tests 25; space sqlite snapshot tests 25. The full source inventory below includes reexports, type fields, definitions and module mounting. The broad exact-name scan found no renamed `as` import/type alias of hydration. Public Store reexport and os_store aliases nevertheless permit qualified consumers; reference counts are not counts of constructed runtime instances or monomorphizations.

## Selected Capability Census

General decoder schema: operation/🧬️schema/📦️decoder/🦀️.rs defines static `MemberSnapshotDecoder<P>::decode_snapshot(&[u8]) -> Result<P,PackError>` and the explicit `ArtifactPackSnapshotDecoder<P>` implementation calling `P::decode_pack`. `ProducedMemberSnapshotOpen<P,D>` invokes D at operation line 191. General publication schema defines static `ArtifactStorePublication<P>::publish_native_snapshot()` and two explicit existing implementations: `ArtifactSnapshotPublication` and `ArtifactPackGenesisProducer`, both invoking the current native snapshot registration. Store reexports decoder at 74 and publication at 2612; open module reexports decoder/open types at line 8.

The real Flow producer `FlowSnapshotPackProducer` already implements both ports at Flow binary snapshot lines 62 and 70, as well as genesis encoding/decoding at 65. Its decoder reconstructs FlowHostSnapshot from authored DSL Pack conversion, and its publication registers its authored flow.host_snapshot dialect and codec. Its MemberStoreProducer impl at Flow vcs line 181 selects `ProducedMemberSnapshotOpen<FlowHostSnapshot,FlowSnapshotPackProducer>` at 187. This is an existing substantive producer suitable for explicit selection, not a neutral witness.

General DagMemberStoreProducer at Infinite dag retained line 29 selects ProducedMemberSnapshotOpen with ArtifactPackSnapshotDecoder at 34. Existing test producers are enumerated below, including unsupported Demo/Dummy opening, HashProjection Pack decoding, explicit TestMemberStoreProducer, and recursive branch streaming opening. Unsupported snapshot opening must remain deliberately unsupported; do not fabricate successful decoder/publication witnesses for these fixtures.

Specific S has no explicit `impl MemberSnapshotDecoder` or `impl ArtifactStorePublication` in the searched Rust source. Its base `semio_member_owner!` macro owns SemioMemberStoreProducer implementations at base lines 111–130. The two-argument arm selects ProducedMemberSnapshotOpen with ArtifactPackSnapshotDecoder; the explicit-open arm selects its supplied operation. Actual invocations include value, model, document, audio, image, brep, kit, graph, cad, video, text, presentation, flow, object, animation, mesh, drawing and table. S Flow explicitly selects its own SemioFlowSnapshotDecode. That streaming decoder is an operation, not an implementation of the whole-Pack MemberSnapshotDecoder port. Future hydration capability selection must author the real S methods at their defining owner rather than infer them from the opening associated operation.

## Where C Is Lost and Required Repair Shape

MemberStoreProducer's current schema owns only SnapshotOpen plus birth/catalog admission. InitialMemberStoreOpen retains `producer:C` at 332 and `Option<C::SnapshotOpen>` at 334, but its hydration field at 350 is only `RetainedPersistedDocumentHydration<P,M>`. The BeginHydration constructor transfers the exact initial snapshot, original genesis Pack and finalized digest, decoded history, expected identity, owner, schema, original admitted owner catalog, operation/generation/expiry and actor; it transfers no selected C. Hydration then dispatches directly to `P::decode_pack` at 423 and native P publication at 449. Its current executable bounds retain ArtifactPack/ArtifactStoreSnapshot and M OpBinary/OpText. The representation dependency is therefore in the actual operation, not merely trait naming.

After the current batch qualifies, define the combined required capability schema-first, explicitly retain/transfer the selected producer from member opening, and require the real archive parent app to select a producer too. Reuse the existing first-party decoder/publication methods where their semantics fit; do not add default forwarding, compatibility adapters, fake neutral snapshots, or synthetic authority. Update all public constructors, private new, struct literal tests, retained fields, retirement impls and real producer macros together. Source guard declaration/input schema and actual-body checks must then be updated intentionally to cover the new owner and dispatch, rather than accept free-floating matching text.

Preserve the original caller Source loan and request, pack bytes/digest/history identity, operation/generation/expiry checks, original grant minima and progress accounting, exact retained body quote (including any newly stored producer), retained originals on refusal, complete paid retirement and terminal emptiness before drop. from_decoded_pack must preserve its supplied original digest; from_pack must preserve bounded scan/hash phases. Keep publication after identity validation. Mutation wire decoding via M::decode_op/M::parse_op remains a separate capability frontier; this extraction alone cannot prove event codec independence or deletion correctness.

## Complete Exact-Name Hydration Rust Inventory

```text
```

## Actual Producer Impl and S Macro Invocation Inventory

```text
```

## Observation Limits

These are source expressions and authored contracts, not executed instances. No macro expansion/name-resolution validation, runtime ownership proof, passing test claim, or artifact deletion claim is made. Broad rg uses the normal tracked-workspace visibility and Rust globs; generated build artifacts were not searched or treated as defining sources. Concurrent source changes require a fresh focused read before mutation.
