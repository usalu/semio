# Controlled Original receiving for ordinary PDF edits

The staged base PDF format-1 receiver handles `SetPageRotation`, `SetInfo` and `SetLanguage` through the same `PdfMutation` Original cursor used by the Image Samples and COS object lanes. It keeps the canonical format byte, tag, varint length and compact JSON. The receiver retains the frame's original source anchor until close. Root and info field positions are identified one record entry at a time. The typed leaf checks exact integer widths and optional-field semantics, takes existing string storage, and places the typed value in the shared retained slot. An admitted Info extras sequence uses the existing paid COS scalar placer. The same close phases retire partial typed or grammar state.

## Canonical fixture proof

The independent Python oracle reads the repository's original before and after PDFs with pypdf 5.9.0. Google CRC32C and Python's SHA-256 reproduce these six compact format-1 frames. The exact repository codec and focused receiver reproduce the same bytes and use the actual rotation, info and language Rust leaf declarations. The focused state test runs their actual cold inverse expressions over the fixture observations. The concrete `PdfSnapshot` apply and restore test is staged in the registered testkit.

| Fixture edit | Direction | Frame bytes | CRC32C | SHA-256 | Turns to Ready | Copied items | Copied bytes | Balanced cancel release |
|---|---|---:|---|---|---:|---:|---:|---:|
| Page rotation, 0° → 90° | Forward | 57 | `2c1af6e0` | `bc9b5bd1f311dd96c43de029ae10bc4fc3bc7ed2507fd7c20f88d741f98222a0` | 192 | 193 | 40,252 | 12,784 |
| Page rotation, 0° → 90° | Inverse | 56 | `a6c49a09` | `26c0f3845ff83219afc5e72b2155a6711773763e0b03cbb3da84c726a3c020ab` | 190 | 191 | 39,948 | 12,784 |
| Document info title replacement | Forward | 98 | `f627cf69` | `7be7038d412064ed0f323602698201721ff93ceb2df64c247d75cceb3098156f` | 331 | 332 | 75,263 | 18,487 |
| Document info title replacement | Inverse | 107 | `a428ddd2` | `b7684abae58988b6341ef1f1bd15a3f85ade2b406a0eb9a546d982b6b83f6237` | 358 | 359 | 79,376 | 18,496 |
| Document language, absent → `de-CH` | Forward | 48 | `b0de4d8a` | `c2a8c98ef1cefa94ac5ba976ff0103d91bdcfa572c1894d94b90adb56e8312aa` | 159 | 160 | 32,800 | 12,584 |
| Document language, absent → `de-CH` | Inverse | 45 | `3b431afa` | `e7ded009d8a9fa4bffcc925cb325a3e2bd527434ad257e107a3d0059d6b0a264` | 143 | 144 | 30,363 | 12,547 |

The six fixture edits each require zero additional typed allocation after grammar custody. A second Info shape exercises signed date offsets, defaults, a string donation and one COS integer extra. It adds exactly 96 bytes of typed backing under the inherited stock, requires 624 items and 124,137 copied bytes through Ready, and releases all 39,777 retained bytes when canceled immediately before Ready. The recorded byte counts are logical receiver receipts on this host and include cursor and grammar work where applicable.

All 17 focused tests passed. They compare the cold decoder and Original receiver on 15 declared invalid inputs, including overflow, floats in exact integer fields, missing required fields, duplicate fields, wrong types and a wrong binary tag. Three additional valid shapes cover omitted optional language, ignored ordinary extension keys and typed date/COS metadata. A source subrange stays anchored inside a larger byte buffer. Typed work and copy grants one unit below demand leave the cursor unchanged. A 95-byte capacity grant at the 96-byte Info extra reservation does the same. An occupied output receives zero work. Cancellation before the last typed step leaves the destination empty and exactly balances accumulated capacity with retirement in all seven trials.

## Registered replay handoff

The Python stage composes the object and Image overlays into an 11-path hash receipt at `🗑️generated/archive-leaves/receipt.json`. It verifies every prior staged SHA, current product preimage SHA and target path length. The ticket stage only creates generated files. The path preserving base-testkit mirror directly mounts the existing aggregate mutation unit module with a Cargo `#[cfg(test)]`. After applying this receipt from its exact preimages, the shortest registered gate for the three leaves is:

```sh
repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-stdio-pdf-testkit --lib original_archive_ordinary_leaves
```

That filter includes `original_archive_ordinary_leaves_apply_and_restore_real_pdf_fixtures`, which decodes the committed PDFs through the normal testkit, obtains the production `mutation_inverse`, and applies both received values through `apply_pdf_mutation`. The app-level proof separately requires the staged [VT viewer replay issuer](vt-archive-load.md) and its genuine committed SPR test. No product source was edited and no governed build was launched for this slice.

The typed Info scope covers two date records of up to 24 fields and up to 16 simple COS scalar extras. Larger records, larger extras sequences and nested or otherwise unsupported COS retain the original cold Bind behavior. This leaves complex metadata and the other PDF mutation tags outside the staged bounded Bind guarantee. A future paid typed receive for general COS would remove that Info fallback without changing the wire, while the remaining mutation leaves require their own typed placements.

## Reproduction

- `archive-leaves-laws.json` provides the language-neutral frames and validation cases.
- `archive-leaves-oracle.py --verify` checks the committed PDF observations against the corpus.
- `archive-leaves-stage.py . --verify` checks the staged product candidates and current preimages.
- `set-image-donation-check.py --probe-only` with `SEMIO_TICKET_DONATION_OVERLAY` pointing at `🗑️generated/archive-leaves` and `SEMIO_TICKET_DONATION_LIBRARIES` pointing at `🗑️generated/set-donation` compiles and runs the focused real codec/Original cursor proof against the existing cached value/model libraries.
