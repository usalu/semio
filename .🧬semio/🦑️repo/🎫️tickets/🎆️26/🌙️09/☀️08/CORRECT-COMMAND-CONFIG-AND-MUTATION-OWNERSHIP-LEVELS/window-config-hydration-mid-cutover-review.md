# Window Config Hydration Mid-Cutover Review

Root inspected the active, unfinished `RetainedConfigStoreHydration` implementation during its declared API cutover on 2026-09-13. These are source findings to resolve before acceptance; no new native result is claimed and this is not a review of a completed implementation.

The file is `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs`.

1. Its current operational and retirement impl bounds again include ArtifactPack, although runtime adoption was earlier split specifically to avoid requiring document/archive capability. The retained owner catalog is still DocumentStoreOwners. Generic config initialization/replay must use the appropriate config/Store lifecycle capabilities without importing document-member grammar or an unrelated document factory catalog.
2. The Begin phase clones the whole initial snapshot, clones cursor ID vectors, constructs an envelope, bulk-reserves the conflict vector, and clones another current snapshot in one call. `maximum_bytes` is not consulted for those transitions. The generic P has no enforced small fixed physical or logical bound in this file.
3. Operation decoding calls generic whole M::decode_op/parse_op. Replay applies an arbitrary generic diff in one opportunity, and cursor/applied/redo phases search the full edit vector. These need retained typed decode/preparation/replay owners or an actually enforced fixed small contract with correct admission; merely naming phases is insufficient.
4. Finish allocates the boxed Store and clones checkpoint/actor identity before transferring. That transition also needs the correct admission/custody boundary. Rejection/close already retain several pending owners, but ordinary snapshot/operation allocations and transfers must participate in the same accounting.

The execution agent has been sent these exact findings. It must distinguish compile coherence and exact-inner-identity functional acceptance from full retained hydration acceptance. Production-host adoption remains required after this loader and its every-cut refusal/cancellation/physical-close laws pass. An independent Terra review should follow the coherent bounded milestone.
