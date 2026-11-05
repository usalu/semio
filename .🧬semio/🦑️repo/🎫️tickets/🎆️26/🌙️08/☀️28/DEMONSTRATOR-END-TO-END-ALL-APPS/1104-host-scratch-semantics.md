# Procedural Host Scratch Semantics

Active P2/P3 tick branches still await the complete funded host/provisional fold; no old synchronous branch has been silently replaced by a partial implementation. The following facts come from the actual semantic diff functions under each artifact's mutation taxonomy, which remain independent cold oracles.

- CreateWidget and ConnectSynapse clamp insertion index to the end. Generation Add differs: it rejects a supplied past-end index.
- DeleteWidget removes the first matching widget only; connected wires remain dangling and layout is not silently removed. The old diff emits an informational cascade message rather than deleting edges.
- ReplaceWidget/ReplaceSynapse require matching ID. Synapse replacement does not add the endpoint existence validation used by insertion.
- MoveWidget requires an existing widget and finite x/y. ClearWidgetLayout only addresses the layout map and can clear a dangling layout without an extant widget.
- Camera update requires finite x/y/zoom, with no additional positivity condition. Equality is a no-op. Preserve the old stored signed zero when PartialEq says equal; unconditional replacement can change canonical bytes.
- P2 schema change permits any string; P3 rejects Unicode-trim-empty input. Both have equality no-op. P3 payload name is new_schema.
- P2 ConnectSynapse ignores an equivalent existing from/from_port/to/to_port tuple after checking unique ID and endpoint existence. P3 permits that parallel tuple. Shared mechanics must not erase this intentional current domain difference.
- Slider requires finite target and an existing InputSlider. Current value equality is checked before range landing, so preserve no-op behavior even if the existing range is malformed. Use the existing bounded scalar slider policy; it computes all next values before writes and widens range exactly as the cold oracle.
- MoveNodes validates target-set invariants and finite offsets; missing or unplaced widgets are skipped when any valid target exists. A zero offset is no-op. Any nonfinite resulting position rejects the entire update, so stage all selected numeric updates before map writes.
- P3 drag/rotate/scale validate payload and target-set invariants, skip missing/mismatched operators, and apply the valid subset. No valid subset is rejected. Identity transforms are no-op. The actual compose math is bounded scalar logic, but dictionary reads and new native parameter values still require their real funded producers.
- P3 ChangeWidgetInput has eleven literal variants. Its actual admissible/landing logic also checks existing input kind/shape and inbound wire ownership. Do not call its cold widget/dictionary clone path under the original frame.

Mounted mechanical helpers: foundation/🌊️flow-source/📋️rows/🦀️.rs moves actual Vec owners through OriginalVectorCursor and a paid final backing; row writes charge sizeof(T). foundation/🌊️flow-source/🔎️rows/🦀️.rs compares original UTF8 row IDs with funded byte pairs and authenticates the held row/key/projection identity. Both have native cancellation/denial/receipt laws in both editor roots, native NOTRUN. Neutral5 row and4 ID cases pass Ajv+SQLite.

Direct provisional native intake: foundation/🌊️flow-source/📥️provisional owns the genuine immutable context alias and indexed original projections; its 🛬receive child uses NativeReceivingOriginal::from_projection under the actual original decoder recipient, retaining complete failure/effects/refusal custody. No DslValue clone, reserialization or fake retained reference. Native product typing and integrated original-caller laws are still required.

Owned layout clear now has the rightful same-engine OriginalOwnedEntry::remove extension, mounted after the shared four-law baseline. It skips the nonexistent value header and passes None into the existing UpdateCursor. Borrowed lookup/rank and removal native laws are mounted but NOTRUN; full ordered suite is required.

The actual shared four-controller native baseline passed all4 at20:47:22.979Z. This is not a product tick integration verdict.
