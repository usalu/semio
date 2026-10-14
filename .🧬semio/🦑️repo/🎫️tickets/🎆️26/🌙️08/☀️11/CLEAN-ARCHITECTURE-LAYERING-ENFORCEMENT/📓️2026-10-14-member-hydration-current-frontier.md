# Current Member Hydration Ownership Frontier

Read-only Root continuation while the graph, member-source guard and Media adopter batches are being authored. No held Store/Member source was changed, no tests or native operations were run, and no runtime/deletion correctness is claimed. The older flow-member-producer-next-source.md describes the pre-extraction trait and is historical; member-catalog-producer-frontier.md records the newer38-file completed source batch.

## Current Defining Boundary

The current MemberStoreProducer<P,M> trait owns the SnapshotOpen associated operation and exact birth/owner admission methods. InitialMemberStoreOpen<F,P,M,C> retains the actual C, original Source loan, C::SnapshotOpen and original owners. begin quotes the child operation, fixed history page and producer source extent, clips original grants, checks progress.fits and exact retained_capacity_bytes, then retains either admitted owners or refused owners/error. These are real source obligations, not fresh authority or replacement owner witnesses.

That extraction is incomplete for representation independence: InitialMemberStoreOpen still requires P:ArtifactPack+ArtifactStoreSnapshot+ArtifactCompositionFields and M:OpBinary+OpText. At BeginHydration it transfers its exact decoded snapshot, genesis Pack/digest, history, identity and original DocumentStoreOwners to RetainedPersistedDocumentHydration<P,M>. It does not pass C into that operation. The retained producer currently cannot select hydration decoding or native publication there.

RetainedPersistedDocumentHydration has ordinary struct bounds but executable/retirement impls require P:ArtifactPack+ArtifactStoreSnapshot and M:OpBinary+OpText. Its DecodePack phase directly invokes P::decode_pack at current423, and Begin directly invokes crate::io::publish_native_snapshot::<P>() at449. The owning ArtifactStorePublication<P> trait already exists, with explicit native ArtifactSnapshotPublication and ArtifactPackGenesisProducer implementations. The already-defined MemberSnapshotDecoder<P> port is used by ProducedMemberSnapshotOpen, but does not currently own this history hydration's direct Pack decode. Replacing a snapshot's nominal owner alone cannot make child artifacts deletable while these capabilities remain implicit in P.

## Next Bounded Source Work After Qualification

First qualify the current229+38-file Store/Member changes and the new Root body-bound guards through original registered commands. Do not stack another broad Rust mutation over an unqualified frontier merely to reduce textual references.

The next defining repair should retain an explicitly selected hydration producer/capability at the operation body and transfer the same selected value from actual member opening. Use the existing first-party decoder/publication contracts where their ownership semantics fit; declare any new combined capability schema-first. Handcraft actual direct hydration constructors, member generic bounds, real child producers/macros and tests together. Keep exact body size, caller Source loan, original request/digest/history identity, grant minima, retained originals on refusal, and complete paid retirement. No generic default producer, forwarding compatibility wrapper or fake neutral snapshot is acceptable.

Event decoding is a separate remaining capability: OpBinary/OpText on M still determines actual persisted mutation wire behavior. Generic publication/constructor extraction must not be misreported as event codec or artifact deletion proof. Parent Flow concrete child tests/Cargo dependencies also remain outside this bounded read-only inspection.

## Source SHA256 Observations

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🏭️member/🧬️schema/🦀️.rs`: `46922b4ec47260939b73ed02c0ee7154067830d825a6242d3557df7e00fb86b3`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs`: `341faf811bc518f31056b69b47250af077fd3dc86e7a9ba2bd68e00e382be6be`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs`: `34751774f108c20f209c98075fd9dfc6331e1f0c7b3ef58ee1423c819c279dc6`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/📣️publication/🧬️schema/🦀️.rs`: `56a1046aab163d41cd410877271083219893b2e3383439154f4981ce56f3a428`.
