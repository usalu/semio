# Store Producer Current Read-Only Audit

Static read-only observation; no initializer, compiler, tests, formatter, Git mutation or source edit executed. Other agents actively modify these sources: line numbers and hashes identify this observation, not a frozen repository or validation result.

## Required Owning Contracts

- Store root `new<C:ArtifactStorePublication<P>>` (2995) and `construct<C>` (3005) require explicit selected publication and invoke `C::publish_native_snapshot` after validation. Main implementation P bounds no longer require ArtifactPack. This is a neutral receiving seam.
- General `ArtifactSnapshotPublication` implements the contract for any P:ArtifactStoreSnapshot and forwards General IO publication; this is a blanket producer whose behavior comes from P's native registration. It is not an actual Flow producer injection.
- VCS `ArtifactGenesisProducer<P>` is selected by reference at `born` and `from_stored_pack`; General also defines blanket ArtifactPackGenesisProducer. The trait is independent of ArtifactPack but default convenience consumers still choose the blanket producer.
- General MemberSnapshotDecoder<P> defines static decode. ProducedMemberSnapshotOpen<P,D> operation requires only P:RetireOwned and D:MemberSnapshotDecoder<P>, admitting exact request authority before calling selected decoder. General ArtifactPackSnapshotDecoder is a separate blanket implementation P:ArtifactPack.
- MemberStoreOwner requires SnapshotOpen:MemberSnapshotOpenOperation<Snapshot=Self>; it does not require a pack codec. Flow's associated type explicitly selects ProducedMemberSnapshotOpen<Self,FlowSnapshotPackProducer> at Flow VCS 187.

## Actual Injection and Residual Coupling

- Flow initializer at Flow VCS 268–271 selects FlowSnapshotPackProducer for genesis and publication. Flow snapshot producer implements all three contracts and publishes actual Flow dialect + codec.
- General create_member_store/open_member_store (Store root 648/671) still require ArtifactPack + ArtifactStoreSnapshot and select ArtifactSnapshotPublication internally. Creation calls P::decode_pack; opening uses General parse_decoded_document_spr. These APIs do not receive selected genesis/decoder/publication producers.
- General install_unscheduled_catalog/install_funded_member_store_owners (root 686/698) carry ArtifactPack + ArtifactStoreSnapshot bounds even though their receiving concern is ownership catalog installation.
- General document convenience creation chooses ArtifactPackGenesisProducer at document 29. Store text parser chooses the same at text 1355. VCS text FromValue for AdmittedArtifactGenesis<P> is a blanket P:ArtifactPack implementation hardwiring ArtifactPackGenesisProducer. Caller-facing Flow convenience creation therefore still uses this hidden blanket even where publication is explicitly Flow.
- ArtifactStoreSnapshot extends ArtifactPack (General Store IO 19), and its optional native registration and sqlite methods join encoding and application registration capabilities. Generic codec/replay helpers in Store root retain these required bounds and hardwire default publication. These are mandatory generic wire paths, not literal imports of the Flow crate.
- No exact `envelope_from_value`/`envelopeFromValue` identifier was present in searched Rust module sources. No claim about other languages or generated APIs.
- No direct General-to-Flow literal edge is established by these inspected definitions; residual coupling is to blanket pack/default publication and required native snapshot bounds. Actual Flow constructor/member seam is caller-specific. Deletion resilience of the full compiled crate was not executed or established.

## Static Call-Site Hazard

Observed unrelated wasmtime::Store::new::<ArtifactSnapshotPublication> at Plugin describe descriptor-emission 415 and interpreter test sources following broad constructor replacement. This selects a method generic on an API that does not accept this producer. Root and Flow ownership agent notified immediately; this audit did not change those files. Existing Store test sources also retain old one-argument ArtifactStore::new/new_with_actor; whether those modules compile is outside this static observation.

## Exact SHA-256 at Report Write

Observed UTC: 2026-10-14T18:25:19.742799+00:00

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`: `eab48515b9cbe5df3d0958727caee84c0fda02cea04243bf6de03a6531687311`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/📣️publication/🧬️schema/🦀️.rs`: `3390878a388f3574c4439ae684c971bd36432769f70ca0a48fede655e1818ce4`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🦀️.rs`: `731a94f6f59c73255462df472227808f9f2c6ec8b8fbaad810aa6ed40d3a542e`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/🦀️.rs`: `346d282e9591d8d52d4aee5569945197f0e4ee343e0814100493dfc5e960cfed`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/🦀️.rs`: `aa9b2a4cb3e83576d20f807d3ee218bbb300a61dab69f80c7d69130ff3f17a5f`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/📝️text/🦀️.rs`: `d4edfbeaad147ad826b8f4b0f2e23a517231f833b8905035216b83dbc31790d0`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs`: `95e260108918ae0dd6291d9b80ff57ee93e8fd02993bdb5b29f688be91f44813`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🧬️schema/📦️decoder/🦀️.rs`: `20ab2cc6e1c6b531054f1d456d0ddef6a03fb065677c04024db7c7f173a0914b`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🌱️genesis/🦀️.rs`: `0ec7b0a4b25a65345c3bc310be4aebf8f7d4eab57ecf5a50726cab60d3bcab1d`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/📝️text/🌱️genesis/🦀️.rs`: `cf0723d75b4f802c162ec8059e1c06eaff180133efb3eb4cdeead7d15d49ddef`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs`: `59e1172170812315e91e67d23e67963cfdc687746e925d0f3ef033e96a2854d6`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🚪️io/💾️binary/📸️snapshot/🦀️.rs`: `3864b79bc45cbc2e52d0c87a920c38275ce9960a0240ca0557357ba185ec50e8`
