# Record Fields Codegen Pair Propagation

Read-only current mounted source; no Cargo or generated-expansion runtime verdict.

Shared controlled encoding generator at DSL schema derive722 and OS derive1483 emit `record.insert(#id,field)?; control.step()?`. Actual record constructors shared801/OS1563,1660 and tagged variant constructors shared977/OS1910 reserve exactly the number of generated encoding statements and hold EncodedRecord until take. Replacement/refusal retirement remains owned by EncodedRecord rather than generated ordinary field mutation. Shared generator additionally annotates field projection errors with under(key); OS omits that annotation, an existing diagnostic difference unrelated to insertion propagation.

Shared/OS first-party RecordFields source facets are byte-identical. Sorted unique IDs provide canonical equality and field iteration, and absent-default ordinary parsing now uses explicit membership/insert; no record entry shim remains. Other `.entry` uses in macro metadata indexes and Pack symbol/classification maps have different owners and are not record backing compatibility.

Reader insert71 in both facets now establishes DecodedFieldOwner before checked capacity arithmetic and allocation. This repairs the earlier incoming-value overflow cleanup concern. Each growth charges a complete fresh Vec frontier before transferring existing fields; prior storage release does not refund cumulative owned bytes. Retire helper itself still has allocating pending Vecs, so source guard correctness does not prove allocator-refused cleanup for arbitrary deep fields.

Pack shared record_slots2027 and OS2026 match: checked address-space count, checked `count * size_of<(u16,FieldValue)>`, full materialization charge, exact Vec reserve with typed AllocationFailed, replacement ownership transfer. The old per-record tuple plus128 estimate is absent from these reviewed constructor/record_slots paths; unrelated metadata/index estimates are not covered by that conclusion.

No new concrete propagation gap found in this bounded source review. Full generated expansions, current five-owner compiler namespace ports and genuine DSL whole83 remain Root's execution authority. The allocator single law still measures its authored payload-free cardinalities and full requests; source equality and parser receipts alone do not establish those runtime results.
