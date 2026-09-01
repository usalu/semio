# Exact Children Dialect and Strict UI Bridge

Status: ticket-only read-only source inventory and diagnostic of the CURRENT whole-buffer UI cursor. No streamed decoder, canonical schema, runtime API, price, caller or native build change.

## Ninth Dialect: Exact Existing Class

The class is `RetainedUiChildIdsCursor`, not “RetainedUiNativeChildrenCursor”. Actual path:
`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts:465`.

The R211 AST census already records its eight fields; the summary table now explicitly includes:
`input, output, index, payload, retirement, failure, closing, ready`.
Constructor475 transfers an intrinsic-branded whole `BigUint64Array` with maximum1024 bytes, then creates an output array of matching length. Scalar traversal484–487 checks each exact u64 against9007199254740991 before Number conversion. Publication489–491 creates the typed children payload/root/owned link and drops input locally. Close496–504 drives the payload child, otherwise clears whole input/output references. Existing string-only fault handling/forced-pending child forwarding is not new strong fault/ledger proof.

Canonical opcode4 is different: `KernelReturnUiOperationHeader` leaves the list count and canonical ULEB-u64 items inside the original field after the node id. It does NOT produce a BigUint64Array. Preserve the safe53 typed admission rule, but do not feed paged bytes into the old constructor, reinterpret a byte backing, or claim eight fields already implement its bytewise count/item grammar. A source-bound list count/item cursor, original reader receipts, exact field EOF/trailing check, output admission and typed handoff still need their own declarations. Other eight dialects remain PACK bridge profiles; opcode9/10 have no field.

## Generic Store Decoder Is Not the Strict Structural Oracle

Store `decode_wire_value` at4644–4651 returns Null when the decoded record does not contain field1 as Value. That fallback is observable in source, but no native Store decoder was run here. It cannot establish acceptance/rejection parity for malformed UI bridge packets.

R216 uses two independent layers: a small test-only literal null/bool bridge oracle using the existing third-party canonical LEB128 decoder/reencoder plus exact field/tag/extent assertions; and the actual existing UI `RetainedUiWireValueCursor`. The literal oracle is NOT a new production PACK implementation or a complete generic decoder. It is deliberately restricted to the16 declared bridge cases below.

```json
[
  {
    "id": "null",
    "hex": "0001011112",
    "accepted": true,
    "expected": null
  },
  {
    "id": "false",
    "hex": "0001011101",
    "accepted": true,
    "expected": false
  },
  {
    "id": "true",
    "hex": "0001011102",
    "accepted": true,
    "expected": true
  },
  {
    "id": "missing-bridge-record",
    "hex": "0000",
    "accepted": false
  },
  {
    "id": "extra-bridge-field-count",
    "hex": "0002011112",
    "accepted": false
  },
  {
    "id": "wrong-bridge-id",
    "hex": "0001021112",
    "accepted": false
  },
  {
    "id": "missing-outer-tag",
    "hex": "00010112",
    "accepted": false
  },
  {
    "id": "wrong-outer-tag",
    "hex": "0001010012",
    "accepted": false
  },
  {
    "id": "nested-any-tag",
    "hex": "0001011111",
    "accepted": false
  },
  {
    "id": "trailing-byte",
    "hex": "000101111200",
    "accepted": false
  },
  {
    "id": "truncated-before-id",
    "hex": "0001",
    "accepted": false
  },
  {
    "id": "truncated-value",
    "hex": "00010111",
    "accepted": false
  },
  {
    "id": "noncanonical-symbol-count",
    "hex": "800001011112",
    "accepted": false
  },
  {
    "id": "noncanonical-field-count",
    "hex": "008100011112",
    "accepted": false
  },
  {
    "id": "noncanonical-field-id",
    "hex": "000181001112",
    "accepted": false
  },
  {
    "id": "empty-input",
    "hex": "",
    "accepted": false
  }
]
```

Actual current UI results:3 accepted (null/false/true),13 rejected, matching that independent restricted oracle. Missing field record/count, wrong id, wrong/missing outer tag, nested Any tag, trailing bytes, truncation and noncanonical symbol/count/id varints do NOT become successful null placeholders in this UI cursor.

## Actual Diagnostic and Bounds

R215 import-resolution setup failure: changing process.cwd does not change an eval module's relative import base. Nx exited1 before loading the cursor or running cases. Preserved as a setup failure, not semantic RED.

R216 corrected only the explicit local source import to the already resolved absolute workspace path. Existing renderer Nx exec selection, actual start 2026-08-28T05:14:28.135Z, terminal exit0. Five selected production/source hashes match before/after. All16 cursor instances reached explicit terminal-empty close; three valid roots and trailing-byte root closed in5 turns, other rejected cursors in4.

Advance bound is inputByteLength+5 for these no-symbol/no-container null/bool cases: every byte-consuming phase consumes at most one fixture byte, plus symbol-count-done, field-count-done, field-id-done, attach and finish. Rejection may occur sooner. This is not a universal decoder bound.

Close bound5 is the exact possible root clear, empty-symbol-table retirement creation, empty numeric retirement completion, one<=7-byte backing scrub, and terminal observation. No frame/owned container/symbol node/Surface allocation is present. It does not certify nonempty variable retirement or unknown-fault cleanup.

Exact results: valid10 advances; missing/extra record4; wrong id6; missing/wrong outer tag7; nested Any8; trailing10; truncated-before-id5; truncated-value8; noncanonical symbol/count/id2/4/6; empty input1. Raw per-step counts, phases and original failure strings are retained in the JSON.

## Evidence and Limits

`🧪️renderer-children-and-strict-bridge-r216-2026-08-28.json`:16 rows, actual results, all per-step/close outputs, five selected pre/post hashes, exact corrected command/program and prior setup failure.
`🧪️renderer-children-and-strict-bridge-r216-2026-08-28.txt`: complete returned stdout.

No generic native decoder acceptance claim, fresh native parity, new paged-reader runtime, schema mount, source ACK, output publication, global caller cutover or whole-field memory certificate follows. Future native malformed-bridge vectors must compare strict UI structural policy explicitly rather than treating generic fallback Null as a successful UI value.

