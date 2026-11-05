# Static Pack Example Destination

Borrowing canonical authored Pack assets and copying their bytes under original grants would avoid the producer parse→typed→encode roundtrip. It is a useful architecture candidate, but the current LoadDocument destination is not a verified bounded typed receiver.

## Real destination

The actual plugin docs staging maintenance ladder, HydrateParent branch (🪜️maintenance-ladder/🦀️.rs:243), moves parent_pack into RetainedPersistedDocumentHydration::from_pack. Later turns call owner.step(cx, grant with maximum_items1).

The store document/history/hydration hydration-step/pack-scan-and-fold module:30 implements step_decode_pack by calling P::decode_pack on the entire retained Pack, storing the typed result, then cx.consume_fuel(1). This branch receives no native receiver or retained grant. The earlier ScanPack stage only hashes bounded chunks; it does not stage the actual typed decode. OriginalSnapshotRecordReceiving is not used in this load path. No current RegistryCodec symbol was found in framework or stdio source; the named destination owner above is the concrete authority.

Additionally, InstanceStores::stamp_load_document_identity synchronously resolve_ready calls stamp_document_spr_identity using the live store envelope identity. The existing load_example_effect constructs pack plus empty_document_spr(schema,schema), while this stamp supplies the required composition identity before archive hydration. A shared source worker must retain the same identity contract and finance history creation/stamping, rather than treating an arbitrary static SPR as valid for every live document.

## Decision boundary

Do not implement a static Pack worker with a claim of funded end-to-end loading yet. It would remove the source-side cold parse and pack writer but still feed the cold destination P::decode_pack. The cleaner shared architecture can use canonical Pack bytes once the destination hydration owns a genuine persistent typed receiver (and corresponding paid partial-close path) for each actual snapshot. This still requires real typed receive capability, including Text nested types and DOCX retained OPC/XML owners; no ToValue bridge or old cold decoder fallback.

Suggested TDD: exercise the real LoadDocument destination with canonical Pack and exact live identity under one genuine original caller. HeapWitness around DecodePack must first expose the current unquoted allocation. Then replace that phase with the actual staged receiving owner and test tiny fuel, all five denied currencies, malformed source, cancellation mid-typed receive, unchanged caller stock, paid close, exact snapshot identity/content, and one delivery. Neutral schema/corpus should distinguish producer source-byte custody from destination typed decode; a staged hash must never count as staged parse. Keep the existing supported cold SDK codec as an output oracle only, outside the measured production turn.

No production edit or native execution was performed for this research. Canonical binary assets remain frozen pending actual SDK encoder evidence.
