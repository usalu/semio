# Intrinsic Owned Octets

## Decision And Scope

The full PDF quick suite passed 546 of 547 laws but the real bachelor fixture exceeded even 32 million tokens: each decoded image byte became a separate numeric dynamic-value node. The owner model already distinguishes intrinsic octets. An explicit `DslValue::Bytes(Vec<u8>)` now preserves that distinction through owned payload text and Pack, without altering PDF's handcrafted SQLite entity/relationship tables or introducing a whole-document carrier.

Text spells an intrinsic byte node as `bytes64("RFC4648 padded base64")`. Pack reuses its existing inline/chunked byte tags. JSON boundaries remain arrays of integers from 0 through 255. Explicit owner field mappings reconstruct native Bytes or the canonical JSON array; optional octets distinguish absent from empty. There is no field-name inference.

PDF maps COS strings/streams, text codes, font programs, CID maps, sampled functions, mesh shadings, images, inline images, embedded files, color profiles, JBIG2 globals, output intents and the exact two document-ID strings. COS decimal/reference tagged records retain their existing flattened logical fields. PDF's complete SQLite schema remains 127 authored tables.

## Actual Evidence

- Initial native TDD red: missing `DslValue::Bytes`. Subsequent compile reds identified exact consumers rather than introducing wildcard match arms.
- Registered kernel primitive law passed: `a4f7b083-9abe-4b32-a844-c24fc63ee4e8`, 1 law, 0.437 seconds assertions, 1 minute 5 seconds uncached task. Six neutral octet vectors are checked by Bun's independent Node Buffer base64 implementation, native text parsing/printing, Pack and serde JSON projection. Seven malformed base64 spellings are rejected. A 128 KiB byte node fits eight text tokens/one node; caller input bounds reject the small allowance. Pack refuses the same payload with `max_total_alloc=64` before ownership copying.
- TypeScript's registered `@semio-tech/value-bytes:test` first reproduced missing implementation, then passed. Final strict-source/runtime gate: 7.3 seconds uncached; six neutral vectors, 12 independent Buffer comparisons, seven malformed spellings, two allocation limits and four cancellation paths. Callback gaps are at most 4096 input units. Validation precedes decoded result allocation. Strict TypeScript compilation uses only the owning primitive's public types.
- Kernel compilation in the primitive gate included the directory and neural prerequisite arms. Directory credential wiping zeros bytes and counts them; event-page JSON mirror validation rejects intrinsic Bytes; neural Atom conversion rejects Bytes. Root owns their focused regression tests and the graph/UI/stdio-contract exhaustive consumers.
- The first clone filter against the OS kernel selected **zero** tests; it is not clone evidence. The actual framework replication gate passed all six selected laws (310 filtered), 0.30 seconds assertions and 3 minutes 54 seconds uncached. The new intrinsic law exercises copying grants, exact cancellation/source return and before-copy byte limits.
- After consolidating all octet literal helpers into the canonical framework base64 codec, the fresh TypeScript route passed in 10.6 seconds uncached: six neutral vectors, 12 independent Buffer comparisons, eight malformed spellings (including Unicode), two allocation limits, four cancellation paths and strict source compilation. Native controlled encode/decode APIs are authored in the same canonical codec and reexported through replication; their new native law is still pending the full DSL route.
- The physical chunk admission law genuinely failed before repair: a corrupt referenced chunk was inspected before the caller's decoded-byte allowance. The repair reads the declared decoded chunk length and rejects excessive cumulative octets before payload integrity/decompression work. Its fresh green run is pending.
- Current full DSL/PDF fresh attempts stopped before compilation at a shared Nx graph reference to a removed composition-laws package. A subsequent decoder-law retry uses an isolated ticket-owned graph directory; no retired package or alias was restored.

## Owned Files

- `🧰️framework/🔨️modules/🌱️value/🦀️.rs`: intrinsic variant and JSON projection.
- `🌱️value/🔁️codec/🦀️.rs`: explicit value shape.
- `🌱️value/🧬️bytes/{🦀️.rs,🟦️.ts,🧬️schema/🔣️.json,📜️script.ts,📋️project.json,🧪️tests/🧬️base64/🟦️.ts}`: owned primitive, explicit byte/optional mappings, schema, TypeScript oracle route.
- `🌱️value/🧬️clone/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`: capacity-accounted incremental octet copying and exact cancellation/source return law.
- Framework Pack JSON projection and OS Pack value module: explicit JSON projection, existing wire byte tags and before-copy allocation bounds.
- `🧰️framework/🔨️modules/🚪️io/🔤️base64/{🦀️.rs,🟦️.ts}` and replication codec exports: one canonical standard codec with controlled output admission and bounded checkpoints.
- Framework Pack physical format: public decoded chunk length inspection before chunk payload work.
- OS DSL schema and its intrinsic-bytes neutral fixture/schema/native test: strict base64 literal, one dynamic byte node.
- OS store's test serializer, canonical edit value projection/test oracle and retirement: honest JSON array view and byte retirement.
- PDF1.7 snapshot type/owned octet module/RecordSpec/preflight and PDF1.4/1.7 snapshot grammar assets: explicit owner mappings and compact allocation estimate.

## Limits And Remaining Verification

The TypeScript intrinsic primitive exposes output-byte bounds plus chunked cancellation. Native DSL/Pack owner entry points retain their existing bounded parse/decode controls and explicit SQLite native-encoding admission; this report does not claim a general interruptible streaming DSL printer. Native source-owning clone supports bounded grants and cancellation. Full PDF quick execution and complete shared regressions must pass before compact PDF payload fidelity is claimed. No universal artifact completion is inferred from this primitive.
