# Closed Guard Epoch Authority Audit 23

Read-only review of the full guard successor proposal and Interface 40 closure evidence. No archive, runtime gate, source mutation, Git mutation, or child delegation was performed.

## Receipts and Independent Observations

Proposal file SHA-256: 3dd9eed2a9d7d1cb63e8fd9da5a5d85cf91664dd97f611a54d4540f9bf09a955. Full before/inverse SHA-256: d85cf25faa7a2d846cd56c8e88e8aa87876685db7218a5313b4fb99a32232b6b. Full after SHA-256: 6451aa14649011be6feece060c700afd210615a05099660c784eea31f90c91f9. Inverse exactly equals before.

Producer independently hashes to a1eb194b97bc59feb806bde1bcc75cba5d080c9d3a3d15f11a864954220bf52a. Consumer audit stores producer fullBody (not source); fullBody was compared with the actual producer. Literal current-cut-observation prefix appears only in the writer at line 89. Consumer audit identifies ten possible basenames and zero future raw guard reads; all nine uninvoked future paths were independently absent.

Raw guard independently streamed in 1 MiB chunks with a 30-second deadline, completing in 2.32 seconds: 1,374,932,407 bytes and SHA-256 3a910f18bd62252fe5960cd3a35a14ce3c5dbfbfbcd797722db5937414ebc0aa. Path metadata was unchanged before/after: device 16777233, inode 322950471, mtimeNs and ctimeNs 1791496416987111209. No whole JSON parsing of the raw guard occurred.

Inventory and physical terminal agree on outer session 37031, exit 0, original inner exit 1, and expected Explicit General command workspace required refusal. These are receipts in evidence files; this auditor did not personally execute that historical session or independently establish global absence of active handles.

## Review Findings and Required Admission Boundaries

No regression in the proposed diff was found: exact request epoch and producer/path joins replace hardcoded 37; cases are finite 1–10 and distinct; gzip streaming, exact decoded lengths and hashes, before/publication source hashes, descriptor/path identity checks, source-text guards, cancellation and work/byte/deadline controls remain unchanged. Carrier epoch and payload/temp basenames now use the admitted row epoch. There is no source-publication or architecture acceptance authority implied.

Actionable prerequisite: the supplied Interface 40 complete inventory has no rows property, archivalExecuted property, or normalized identity field shape; it cannot directly satisfy assert.deepEqual(request.cases, inventory.rows). Author a separate exact normalized archive inventory preserving the original full evidence reference. Map dev→device, ino→inode, size→bytes explicitly while preserving decimal strings and ns fields. Do not mix outer gate exit 0 with original inner exit 1 or invent a childClosure success. Consumer producer metadata uses fullBody, not source.

Current guards.json still points to Interface 37 and lacks interfaceEpoch at observation; current schema and fixture remain the previous admission. The operator proposal alone therefore gives no Interface 40 execution authority. The new policy must pin exact 40 request, one receiving-red/pre row, its guard identity/hash, producer, custody/consumer evidence, and original expected-red failure facts. A generic numeric interfaceEpoch check alone is insufficient to prove exact 40 authority.

Native census remains restricted to Darwin/Linux ps/lsof and explicitly refuses other hosts; no Windows portability or future lease guarantee can be claimed. Historical physical closure does not replace immediate pre-publication census. Consumer absence applies to the exact receipt-bound producer body only, not future unrelated consumer code.

Inherited boundedness limit: control setup and guard readFile/exactGuards operations occur outside byte/work metering or are not abortable filesystem calls, and publication fsync/rename/journal writes are not intrinsically cancellable. The proposed diff does not expand these operations or weaken controls, but a claim that every operation has a hard interruptible deadline would exceed the implementation. Raw guard streaming itself remains bounded and cancellable.

No tests were run by this auditor. Root planned old-schema law 7 failure, successor law 8, then guards 3; none is certified by this read-only report. Broader architecture remains unaccepted.
