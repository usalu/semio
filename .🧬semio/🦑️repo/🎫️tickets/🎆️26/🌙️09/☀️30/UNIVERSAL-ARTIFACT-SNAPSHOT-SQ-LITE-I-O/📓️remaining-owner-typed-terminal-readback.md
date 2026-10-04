# Remaining Owner Typed Terminal Readback

Read-only source inspection on 2026-10-03. No Cargo, tests, runtime, Git mutations, production or test edits. This report identifies source signature inconsistencies, not fabricated compiler receipts. Concurrent files may subsequently change.

## Shared Producer Contract

`🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🦀️.rs` now returns `ValueError` from SqliteRow integer/text/optional_text (31/33/35), control check_rows/check_value_bytes/check_database/checkpoint (149/150/162/168). Artifact Projection new/insert/insert_key/check_rows/checkpoint/finish return ValueError at artifact module 67/76/83/101/107/113. Reconstruction new/text/scalar return ValueError at 227/239/243. FloatRow new/real and IEEE insertion helpers likewise return ValueError. Capability methods still use String: preserve typed failures throughout internal helpers, explicitly convert only at that actual capability terminal. No global From<ValueError> for String and no message classification.

## Bounded Scope

All paths below are under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/<owner>/🏅️standards/<standard>/🪆️subsets/<subset>/🧬️schema/📸️snapshot/🪶️sqlite/`. Counts are lexical producer-family references in inspected production files, not diagnostic/error counts; row counts include helper calls with matching names.

| Owner | Standard / Subset | Files | Control/Projection references | Row/accessor references |
|---|---|---:|---:|---:|
| Binary | raw / any | 1 | 10 | 4 |
| CSV | rfc4180 / any | 1 | 17 | 10 |
| TSV | iana / any | 1 | 17 | 10 |
| BMP | v3 / any | 1 | 10 | 9 |
| Deflate | rfc1950 / any | 1 | 3 | 10 |
| GLTF | 2.0 / any | 12 | 17 | 74 |
| SVG | 1.1 / base | 1 | 1 | 1 |
| PNG | 1.2 / any | 1 | 3 | 26 |
| LAS | 1.0 / header | 1 | 19 | 32 |

Every cohort retains String-returning owner SQLite implementations with at least one bare typed producer call. All nine require terminal/internal boundary review after the shared producer change.

## Exact Prerequisites

- Binary SQLite root to_sqlite_database 7–20 and from_sqlite_database 24–38: bare control checkpoint/check_rows/check_value_bytes/check_database; row.integer at 35–36 and document text/accessors. These cannot propagate ValueError with `?` into String.
- CSV root to_sqlite_database 8–45 and from_sqlite_database 49–85: same control mismatch; record_ids collect at 59 receives typed row.integer errors; reconstruction grouping integer calls at 63/65/72/73. TSV equivalents are 8–46, 50–86, collect 61 and integer grouping 65/67/74/75.
- BMP byte/word/unsigned/signed helpers 5–8 have bare row.integer under String. Projection/reconstruction control sites 17–20, 24/30/35, 41/53/60/69 likewise. Convert helpers to typed internal errors and keep capability conversion last.
- Deflate to_sqlite_database 7: Projection::new at 9, out.insert 10–11, direct out.finish 12. from_sqlite_database 15: control check_database 17, row integer 19–21/26/28, checkpoint 26–27, document.text 28. Direct finish return additionally differs in result error type without `?`.
- GLTF Write::new/insert/insert_key/floats/float_key/check/finish at 30–38: direct shared Result<ValueError> returned as Result<String>. Read::new/checkpoint/consume/ensure_keys/rows/text/optional_text/scalar/word/boolean/finish at 56–88: typed control/row/Reconstruction results. Read::text 78, optional_text 79, scalar 80, finish 88 have direct incompatible result tails. Internal document, buffer, camera, mesh, node, material, texture, skin, animation and extras modules must remain typed through this chain; FloatRow::new/real are not String producers. Owner capability entry points at 101–103 are actual String terminals.
- SVG validate_sqlite_snapshot_subset 18/20 uses typed control and row.text. Its actual return is IoResult, so preserve typed failure through existing IoError mapping. to/from_sqlite_database 25–26 forward XML owner producer calls; coordinate XML producer signatures rather than invent ordinary fallback.
- PNG byte/word/unsigned/boolean/singleton/entities 6–12 have bare typed row accessors and database row validation. to_sqlite_database 18 and from_sqlite_database 63 retain String; reconstruction check_database 65 and tick closure 71 use typed control; row accesses 78–107 require typed helper chain.
- LAS identity/document/u8_at/u16_at/u32_at/boolean/ordered_refs/components 11–23 have typed row/control producers under String. preflight 26 directly returns NativeEncodingBound::finish typed result; validation 28 returns IoError and must preserve kind there. to/from_sqlite_database 29–53 include control and FloatRow typed producers; gps closure 52 currently needs typed ValueError propagation, not String annotation.

## Closed Logical Native Discipline

Prior GLTF/LAS/Deflate controlled parser prerequisites were repaired in the separate controlled API report. This inspection confirms Deflate Encoder admit/checkpoint now return ValueError. Its decode/encode and public compress_zlib/decompress_zlib still convert native control failures to String inside helpers, before the owner capability terminal; that is typed-kind loss, even where explicit mapping makes a call compile. Binary/CSV/TSV/BMP native helpers similarly map ValueError::into_message internally. Do not count those source conversions as category-preserving terminal laws or propose ordinary native fallback. These are closed logical native records and require typed internal owned producer chains, explicit literal/variant InvalidValue errors, and retained allocation/cancellation categories through the actual terminal.

The prior staged GLTF23, SVG10, PNG14, LAS14 counts are authored candidates, not current runtime passes. Root owns compilation and exact target receipts.
