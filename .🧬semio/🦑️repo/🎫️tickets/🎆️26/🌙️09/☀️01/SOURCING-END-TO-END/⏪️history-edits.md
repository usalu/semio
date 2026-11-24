# ⏪️ History edits (undo/redo) end to end, root causes (2026-11-23)

Crate `semio-s-artifact-sourcing-curation`, law `history_edits_end_to_end` (and `conflict_history_edits_end_to_end`).

## First fault (the cleanup refusal masked it)

Instrumented the `faulted=true` sites of `OriginalJsonReceiving`: the first fault was
`original JSON operation wire header differs` on the wire `01 00 01 0d "plate-steel-8" ...`.
Persisted history operations of `SourcingMutation` are binary (`format 1 | ordinal tag | record body`,
`artifact_persisted_binary_operation!` + `original_ordinal_operation_receiving!`), but `with_original_receiving()` (the JSON receiver,
header `op`) was installed. It faulted on the first byte; every later `demands()` call returned
`faulted original JSON receiving requires cleanup`, which is what the driver reported.

## Root causes and fixes

1. Wrong receiver family: `🗄️artifact-store-preparation/🦀️.rs` now installs `with_original_ordinal_binary_receiving()`.
2. Replacement/edit comparison re-encodes the original typed mutation and compares it with the stored bytes. The default wire source is
   canonical JSON, so a variants-binary vocabulary hit `operation output payload extent exhausted` (re-encoding 32 bytes vs 28 stored).
   Added generic `ArtifactOperationWire::with_original_ordinal_binary_receiving()` (one-item-publication `🧳️receiving/🦀️.rs`): it installs the
   borrowed variant (`BorrowedDslVariants::projected_borrowed_variant_identity`, tag = ordinal, body = the record) as the
   `ArtifactPreparedOperationSource::Pack` source, i.e. exactly what `variants_binary::encode_op` writes, plus the binary receiving.
3. `install_document_store_owners_exact` merged only target/receiving/message of the catalog wire into the store log's existing wire and dropped the
   declared `source`, so a catalog-declared codec never reached replay. It now adopts `owners.operation_wire.source` too.
4. Masking fix (generic): `OriginalJsonReceiving` retains its first fault (`fault: Option<ValueError>`, captured in `advance` from
   `NativeReceivingFailure::Primary`) and `demands()` reports that original cause instead of the generic cleanup refusal. Cleanup path unchanged.

## Files changed (all compiled into the sourcing wasm)

- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗄️artifact-store-preparation/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/📥️lifted/🧵️operation-wire/🧳️receiving/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧱️store-components/📥️lifted/🏪️store/🏗️store-owners-retained-reads-and-retirements/🦀️.rs`
- `🧰️framework/🔨️modules/🎒️pack/🔤️json/📥️decode/🧳️receiving/🦀️.rs`

Browser: live undo/redo uses the same wire, so the sourcing wasm needs a rebuild.

## Result
Curation lib `--test-threads=1`: 97 passed, 0 failed (previously 96 passed, 1 failed; the total changed from 98 to 97 between my runs, not by my edits).
