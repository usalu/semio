# Result lease-pin: NamespaceTicket schema pin

## Outcome
Green. The pinned `NamespaceTicket` hash in `queue.json` was re-pinned to the value the test computes. The test itself was not changed.

## Changed files
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🧫️fixtures/📜️queue.json`: `schemaOwnershipHashes.NamespaceTicket` changed from `b1c5d358f16be867b2230bc81ad9e692110d37519927a56ddeddaf6b6cfdcc3a` to `a25f5fa3d9eec40729c1d06a0bfbce80cc12db61b9ed705dd6489f3c39c94480`. One line, nothing else.
- `C/lease-pin-tools/hash.ts` (new, script): copy of the test's hashing logic (strip `$schema`/`$id`, drop the five named definitions, sorted-key canonical JSON, SHA-256).
- `C/🗑️generated/lease-pin-ticket-head.json` (generated): HEAD copy of `ticket.json`, used to confirm the old pin.

## Schema files
- Schema: `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧬️schema/📜️ticket.json`
- Pin: `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🧫️fixtures/📜️queue.json`

## Is the ticket.json change intended?
Yes. The only change against HEAD is one added definition, `AdmissionTimeout` (75 lines, additive, no other definition touched). It is the schema of `ResourceLeaseAdmissionTimeoutV1`, which the implementation attaches to admission timeouts (`🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts`, `directoryAdmission` and `queueClaimCatalog`), and it is validated by `testResourceLeaseAdmissionTimeoutV1` in `🧪️tests/🚪️admission/🟦️.ts`. Caveat: the fix8 note in `C/result-conc-leases.md` does not name `AdmissionTimeout` explicitly; it describes the admission patience/sweep work that this definition belongs to.

The other five pins (`RetirementPolicy`, `ClaimCustodyPolicy`, `ClaimRetentionPolicy`, `WriterProofCorpus`, `WriterIssuanceCorpus`) still match. Their definitions did not change.

## Hash check
- My copy of the test logic reproduces the old pin `b1c5d358…` on the HEAD `ticket.json`, so the method matches the test.
- On the working-tree `ticket.json` it gives `a25f5fa3…`, the same value the test reported as "got".

## Commands and outcomes
1. `bun C/lease-pin-tools/hash.ts <ticket.json>` (working tree and HEAD copy): as above.
2. `bun test ./…/🧪️tests/🎟️writer/🟦️.ts -t "lease authority component schema preserves every owned contract"` run directly: fails with "Original parent Script process invocation required". The test needs the Script process envelope, so a plain `bun test` cannot run it.
3. Same test through the envelope helper, `bun C/lease-proof-tools/run-test.ts ./…/🎟️writer/🟦️.ts "lease authority component schema preserves every owned contract" <scratchpad>/lease-pin`: 1 pass, 0 fail (148 expects).
4. Whole file through the same helper, `run-test.ts ./…/🎟️writer/🟦️.ts "" <scratchpad>/lease-pin`: 5 pass, 0 fail (315 expects). Artifacts went to `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-2BFGEq/a55b014a-59e5-4214-ad10-dbd1b1cb936d/scratchpad/lease-pin`, because the ticket path is too long for the lease path limit.

## Not done / open
- Nothing started by me is still running; all commands were foreground and finished.
- The test file was not run through `bun repo`; the envelope helper is the only way to run it outside the repo runner.
