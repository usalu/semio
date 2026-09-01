# Scalar Declaration Hardening R187–R193

This release is ticket/schema/model only. Production, imported renderer tests, actor callers, prices and capacities are unchanged. Proposed decoder992 plus payload8 and reader16 remains unchanged.

## Actual Executions

R187 desired schema negatives:8 missed rejections (Nx not involved; inline declaration diagnostic exits1). R188 rejects both accepted/expected contradictions, duplicate IDs and each of five missing profiles;43 original scalar values are byte-for-byte unchanged. Unique projected IDs are an explicit semantic validator alongside strict JSON Schema, not a false claim about uniqueItems.

R189 stopped at strict Ajv union-type schema compilation. R190 stopped in the test oracle because an Immer assignment arrow returned a value while mutating. These are declaration/harness failures, not decoder failures. Both were corrected without relaxing strict mode. R191 executes48 closed traces/984 rows with literal assertions and a plain transition evaluator versus Immer.

R192 is the actual desired work-admission RED: with64 bytes the old one-byte read declaration advanced, while the new law required blocked before any receipt mutation. R193 executes **50 traces /993 rows**, strictAjv2 and993 matching Immer transitions after explicit65-byte read/parse grants. Complete raw logs R187–R193 are retained beside this report.

## Mandatory Original Reader Interface (Proposed Only)

```ts
payload.beginReader(builder, consumer: OwnedUiScalarDecoder, receipt: OwnedUiScalarReadReceipt, grant)
OwnedUiResidentPayloadReader.bindScalar(reader, consumer, receipt, payload, grant)
OwnedUiResidentPayloadReader.matchesScalarBinding(reader, consumer, receipt, payload)
reader.prepareScalar(consumer, receipt, grant)
reader.advanceScalar(consumer, receipt, grant)
reader.settleScalar(consumer, receipt, grant)
reader.cancelScalar(consumer, receipt, witness: OwnedUiScalarRetirement, grant)
OwnedUiScalarDecoder.matchesReaderConstruction(consumer, payload, reader, receipt)
OwnedUiScalarDecoder.matchesReceiptApplied(consumer, reader, receipt)
OwnedUiScalarDecoder.matchesReceiptDiscarded(consumer, reader, receipt, witness)
```

No optional consumer or old advance(grant) overload is proposed. Reader construction must install the exact consumer/receipt before finalization/public exposure using the original payload scalar/reader roots and current admission slot. Matchers inspect private original identities/state directly; no callbacks/getter inference/structural proof. Their close identity remains usable after revocation. No existing production method changed.

The one original receipt has the already inventoried eight words. Source-owned scalar calls expose that preinstalled current result instead of allocating a returned byte object. An actual maintenance child result is forwarded unchanged, without updating receipt or appending wrapper work; original-state observation is a later grant. Unknown after-maintenance mutation is quarantined, not cleanly retried.

## Exact Per-Byte Transaction

| Phase | Granted logical work bytes | Source cursor |
| --- | ---: | --- |
| Prepare original receipt/check u64 serial |64|unchanged|
| Read one byte into receipt, fixed receipt bookkeeping |65 (1+64)|unchanged|
| Commit original cursor and receipt phase |64|advance once|
| Decoder observes receipt and latches byte |64|unchanged|
| Apply one byte and fixed scalar bookkeeping |65 (1+64)|unchanged|
| Source settles exact applied receipt |64|unchanged|
| Decoder observes exact settlement |64|unchanged|

All65-byte grants are preflighted before byte access or scalar mutation;64 bytes refuses without changing state. The second grant—not an appended free wrapper—commits offset/consumed. A source-read after-return fault leaves its original receipt value owned and cursor unchanged. A cursor-commit after-return fault leaves the cursor advanced exactly once and forbids retry. Parser after-mutation faults preserve parsed offset and the first exact fault; they never silently skip or reapply a byte.

Normal scalar work is7b+3 turns and450b+192 bytes including explicit start/publication/consume; actual child maintenance/admission/close is excluded and must be composed separately. This supersedes the first report's4b+3 draft.

The closed traces exercise both faults around read/commit/observe/parse/settle, every transaction cancellation prefix, replay, foreign consumer, u64 serial overflow, stale result serial, short grants, and page/seal backpressure versus truncation at each prefix of1/2/4/8/10-byte scalar units. The model's parsed byte count is an ownership observation, not a new arithmetic parser implementation. Original43 independent scalar oracle values remain separate.

## Remaining Close Work

The original source/cursor/latch/settlement declaration is coherent for review now. Full close-grant composition and child-prefix sums are the next ticket-only refinement; existing close names are not yet credited as an executed runtime bound. No runtime source transaction, intrinsic reader binding, metadata admission, real-window split, typed output or ACK is claimed by this release.

