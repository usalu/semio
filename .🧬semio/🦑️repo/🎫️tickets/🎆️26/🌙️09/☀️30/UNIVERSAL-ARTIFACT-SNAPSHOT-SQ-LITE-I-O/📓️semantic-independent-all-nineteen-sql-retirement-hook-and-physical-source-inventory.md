# All Nineteen Held SQL Snapshot Retirement Hook Inventory

All78 distinct held provider after images were inspected. Nineteen SQL snapshot owners have actual RetireOwned definitions. Eight override the erased SQLite retirement hook through the existing common native::Owned guard; eleven inherit the store trait default `drop(self)`. No providers were edited and no runtime command was launched.

| Owner | Snapshot type | SQLite hook | Declared RetireOwned line |
|---|---|---|---:|
| 🔤️text | SemioTextSnapshot | plain inherited drop | 482 |
| 🔊️audio | SemioAudioSnapshot | plain inherited drop | 254 |
| 🎬️video | SemioVideoSnapshot | plain inherited drop | 485 |
| 📦️object | SemioObjectSnapshot | plain inherited drop | 451 |
| 🔢️value | SemioValueSnapshot | Owned override | 376 |
| 📊️table | SemioTableSnapshot | Owned override | 479 |
| 🕸️graph | SemioGraphSnapshot | Owned override | 397 |
| 🌊️flow | SemioFlowSnapshot | plain inherited drop | 371 |
| 🖼️image | SemioImageSnapshot | plain inherited drop | 400 |
| 🎞️animation | SemioAnimationSnapshot | plain inherited drop | 232 |
| 🔺️mesh | SemioMeshSnapshot | plain inherited drop | 410 |
| 🧰️kit | SemioKitSnapshot | plain inherited drop | 405 |
| 🏛️model | SemioModelSnapshot | plain inherited drop | 416 |
| 📐️cad | SemioCadSnapshot | plain inherited drop | 303 |
| 📑️document | SemioDocumentSnapshot | Owned override | 327 |
| 📽️presentation | SemioPresentationSnapshot | Owned override | 458 |
| 🖊️drawing | SemioDrawingSnapshot | Owned override | 344 |
| 🧊️brep | SemioBrepSnapshot | Owned override | 266 |
| ✉️base | SemioSnapshot | Owned override | 160 |

The18 child definition lines refer to the actual Semio artifact root; Base line160 refers to its actual schema snapshot module. The exact eleven unique trait-impl anchors and matching after template are retained in `📥️inputs/semio-all-nineteen-retirement-hook-inventory.json`, together with each held source digest. The template is identical to the existing eight overrides:

```rust
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(self));}
```

This closes the erased-hook dispatch gap using the existing declared owner, without a new adapter. It does not make relational reconstruction temporary maps paid or install guards around partial constructors. The Model seven-anchor reconstruction plan remains a separate prerequisite.

The trait method consumes self, returns void, and accepts no parent owner, progress callback, cancellation input, item grant, or byte grant. The common Owned::Drop constructs an owned retirement cursor and synchronously pumps256 items/65536 bytes until terminal empty. Those internal bounded cursor calls establish declared semantic field traversal; they are not a caller-observed one-item4096 physical settlement or parent handback. Retirement cursor boxes/stack ownership also remain outside this hook admission interface.

Actual source allocation shapes can exceed4096. NativeDecodeControl::copy_text, copy_bytes and allocate_vec admit the whole length/count backing then reserve one String/Vec allocation; cancellable64KiB copy spans do not split that backing. Default copied limits allow up to256MiB value bytes and512MiB allocation bytes. Table rows, Model elements/properties, Mesh geometry and blobs, Drawing path arrays, Document text/blob trees, and other typed vectors/strings can therefore own admitted allocations larger than4096 without changing any semantic limit. This is source/API evidence, not an observed allocator measurement for all cases.

The current generic String Bytes cursor reports truncation bytes while retaining its Vec capacity; actual capacity is deallocated in Drop after terminal-empty. The generic Vec Collection cursor likewise truncates or visits elements while retaining collection backing until Drop. A8194-byte String can therefore have small logical byte steps followed by one full-capacity physical release; byte progress alone does not establish allocator release bounded by4096. Large typed Vec backing has the same issue. Existing physical paged scratch owners do not by themselves transform the returned ordinary typed source fields into paged allocations.

Keep four claims separate:

- Semantic IO parity means Native/Text/SQLite fields and independent cells agree.
- Exact row/value/schema admission means copied semantic grants refuse one-short inputs before prohibited ownership.
- Paid retained temporary frontiers require actual admitted lookup/sort backing and refusal/cancellation guards.
- Physical source retirement requires actual observed backing release or parent transfer with caller item/byte grants, including allocations larger than4096 and zero/narrow grants.

Actionable next steps are the eleven exact hook inserts, owner-specific relational reconstruction guards/frontiers, and a separate actual source ownership settlement mechanism/law for large String/Vec backing using first-party retained retirement and parent-allocation authority. Do not claim physical4096 closure from the hook template or the existing semantic parity laws. No grant or existing assertion should be relaxed.
