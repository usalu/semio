# Controlled InsertObject and RemoveObject Archive Receiving

The format-1 Original receiver now selects paid object binding after its existing length, JSON duplicate-member and tag checks for tags 6 and 7. It discovers the root fields in paid steps, validates the exact `u32` object number and `u16` generation, receives the COS leaf and optional insertion index, and settles stream-role rows one field at a time. The parsed tree and partially typed mutation stay resident in the existing Original retirement slots. Strings transfer their existing backing. The typed result stays private through the last field and an occupied recipient takes zero work.

This controlled scope covers null, boolean, signed integer, text, name and reference COS statements, up to 16 removed stream-role indices and up to four added or modified roles per lane. The controlled role rows cover metadata and calculator text with empty graph paths and dependencies. Compound COS objects, other stream roles and unrelated mutation tags stay on the prior archive path. These lanes still need controlled binding before the generic receiver meets the bounds for all ordinary histories.

## Wire and inverse state

The Original receiver borrows the exact 141-byte forward with `removed:[0]` and explicit empty `modified` and `added` lists. The current writer omits those empty lists, so its 115-byte encoding is semantically equal while the original archive bytes remain untouched. The typed inverse restores the real prior metadata role at its original index and re-encodes to the exact 241-byte format-1 RemoveObject frame.

| Frame | Bytes | CRC32C | SHA-256 |
| --- | ---: | --- | --- |
| Original InsertObject | 141 | `24492326` | `902f84f9a1ba639a21592599a3994ce118bc75d926c79c8ee3a44d401189c58e` |
| Canonical InsertObject | 115 | `56c1ca4b` | `eb3b0b10373004497e9d16837f53a9704bcb1e97d2cde2f7626530fc35793c16` |
| Canonical RemoveObject | 241 | `5b12409a` | `24c1d83b16b1e3d0e7d5bdbc942de8198d446c7250688fb7b3ed87b193094c81` |

Thirteen isolated tests passed using the actual PDF model, exact InsertObject and RemoveObject declarations, unchanged aggregate codec bodies and the real Original work helper. The three-arm executable uses the same value derives as the full aggregate. The production object-diff application restored the prior indirect-object lane, and the production indexed-diff apply and inverse restored the stream-role lane at its exact ordinal. Independent Python JSON and Google CRC32C reproduce the frame bytes, checksums and role ordering. The existing noncanonical varint discrepancy is retained and tested. The cold byte reader accepts a redundant terminal zero and the Original receiver requires the canonical varint.

## Mounted policy evidence

| Operation | Typed allocation | Cumulative copy through Ready | Copied items | Balanced cancellation release |
| --- | ---: | ---: | ---: | ---: |
| InsertObject | 8 bytes | 93,280 bytes | 490 | 34,244 bytes |
| RemoveObject | 608 bytes | 143,448 bytes | 820 | 58,072 bytes |

The per-turn and cumulative grants are unchanged. Each one-byte-short allocation loan records zero work and keeps its quote. A same-bytes replacement source fails the original address check. An occupied output keeps its current value and the receiver publishes into an empty slot on the subsequent attempt. The 12 frame refusals and seven field refusals cover truncated or excessive lengths, duplicate members, missing or mismatched tags, unsigned generation overflow, wrong numeric kinds and malformed inverse roles. Cancellation after the typed allocation closes the parsed source and partial owner without publication.

`archive-object-stage.py` verifies ten candidate paths under the existing Pack JSON, PDF mutation and PDF testkit owners. It stages a full registered `PdfSnapshot` forward and inverse test with the real PDF mutation bridge. The full package runner and persisted VT replay remain separate gates. The live source and pending integrated native run were untouched.
