# Terrain Archive Mutation Wire Witness

The frozen 8,934-byte browser archive has SHA-256 `35c7af2c336e32530bb0375d17d437d91584976a900283959a9d5252b2a85c81`. Independent CRC-32C checks passed for the parent SPK at carrier offset 37 and the parent SPR at carrier offset 2,260. The first-party native SPR decoder recovered one `importElevationGridFile` edit and two transitions.

| Exact history payload | Bytes | SHA-256 |
| --- | ---: | --- |
| Forward operation | 2,726 | `8c35dbdeceb2d02b599ba56d3775be656010d77992dad0bbbdbd1b845ff5de8e` |
| Inverse operation | 3,371 | `9f81224922cdcdd631c8b43f2486b896785588c0b4639ff0f4b5b5539d99dcb5` |

Both operations begin with the same container-less intrinsic record and spell `ReplaceElevationSource` in that record. They contain the historical `newElevationSource.rows` sample field. Neither payload contains an SPK or SPR envelope. Pretending that either operation starts with a Pack header makes the checksum reader read ASCII `tion` as the purported CRC at offsets 20–23. Its little-endian value is `6e6f6974`, while an independent CRC-32C of the purported header's first 20 bytes gives `1495db0a`.

The browser export calls `VcsArtifactApp::document_archive`, then `print_document_pack`, `print_document_spr`, `history_edit_from_edit`, and `history_op_payloads`. The last function takes the registered mutation's `OpBinary::encode_op` bytes verbatim. SPR stores both byte strings without interpreting their mutation schema. During hydration, `step_native_mutation` passes each exact binary payload to its native mutation receiver with range `(0, payload.len())`.

The Terrain wire implementation in commit `3d62b2ef22c` used `encode_wire_value` on the whole externally tagged value. Its untagged `original_pack_value_operation_receiving!` declaration selected the complete Pack document parser. The native witness re-encoded both frozen binary operations byte-for-byte with that historical cold writer. At both 32 KiB and 48 KiB copy grants, the historical native receiver reproduced the browser's `checksum mismatch in header at offset 20`.

Commit `b4927d2c311` pairs Terrain's authored tagged OpBinary writer with the native receiver's `tagged_key` declaration. The matching current writer prepends format `01` and replacement tag `02`. The receiver consumes both before handing the remaining intrinsic body to the record reader. The first-party cold codec round-tripped separately authored exact-millimetre 2×2 and 33×33 fixture shapes under the current tag grammar. The standalone retained shape stand-in reported zero progress with the cached fixture driver, including on the 2×2 value, so this witness does not establish mounted receiving for the new browser build. The full GIS operations libraries did not link against the available cached transitive metadata, and a later probe relink also lost matching metadata for the cached SPR, DSL, and Pack facade libraries.

The browser acceptance must export a fresh archive from the rebuilt registered GIS component and reopen that archive. The frozen historical capture remains the reproduction for the earlier CRC fault. No compatibility path or production source change was added.

The reproducible native inputs are `native-archive-mutation-witness.rs` and `run-native-archive-mutation-witness.py`. The runner reuses the ticket's pinned native probe libraries and checks the outer CRCs independently.
