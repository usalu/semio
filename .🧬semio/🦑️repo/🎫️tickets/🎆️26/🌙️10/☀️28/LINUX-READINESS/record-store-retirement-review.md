# Record Store Retirement Review

Read-only review of current records implementation, schema, adversarial fixture tests, and production optional publication path. No tests or compiler launched by this reviewer.

No concrete blocker found in the reviewed delta. Rewrite freezes the requested key sequence before retrying, snapshots both existing and absent old leaves, and retries only its dedicated identity-change error within the schema-owned three-attempt bound. Each retry recomputes live values and physical retired bytes; absence contributes zero. Nonordinary or multiply linked leaves refuse. Newly published peer pack names are outside the captured retirement set and are not removed.

Peak accounting includes the exact emitted index UTF-8 bytes plus payload bytes. The retained delta subtracts only snapshot-existing leaves. The prewrite check executes inside the publication callback before temporary bytes are created. The actual graph observation callback delegates through withBuildGraphOptionalPublicationsV1 to scope.ledger.publish, whose exclusive withGate callback spans recheck, publication, and retirement. Thus cooperative production writers cannot change retirement identities between that check and unlink. This gate is essential to the accounting argument; a custom publication callback that permits concurrent mutations would not establish the same guarantee.

Descriptor LRU membership now uses a unique symbol per opened handle, rather than a pack name shared by multiple stores. Eviction closes the corresponding local descriptor and deletes only its own global membership. Same-pack readers therefore consume separate bounded slots. Reused local descriptors refresh their own slot; close releases the store’s own descriptors.

The new tests cover stale snapshots, peer compaction inside the publication admission boundary, continuous change refusal, declined publication, nonfile refusal, exact observed physical delta, and same-pack readers. Independent SQLite event projection and Ajv/fast-glob layout checks remain present. This review relies on the owner’s reported focused green results and does not claim canonical runtime acceptance beyond them.
