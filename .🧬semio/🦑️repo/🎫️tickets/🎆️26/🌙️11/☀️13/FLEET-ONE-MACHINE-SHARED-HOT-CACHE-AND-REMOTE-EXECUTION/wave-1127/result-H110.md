# Result H110: capacity store isolates foreign grant rows
Deutsch: Ergebnis H110: Der Kapazitätsspeicher isoliert fremde Zusagezeilen.

## Root cause
A live older process wrote a grant row with inline `peakMegabytes/observations/deviationMegabytes`. `Grant` is `additionalProperties:false`, so `readGrants` threw `unrecognizedGrant` for a live owner; every admission waited ("written by a newer version", wrong relation) and `repo capacity status` crashed.

## Fix (host-capacity, schema-first)
- Format identity lives in a sidecar `<id>.format` (schema `GrantFormat`, `Protocol.grantFormat` = 1, written BEFORE the row). Not inline: an inline field would make every still-running older reader refuse all rows (the same outage in reverse); the `.stats` sidecar precedent applies.
- `readGrants`: a row is current iff its sidecar says the current format AND it validates as `Grant`. Any other row (no sidecar = "unversioned"/older code, older, newer, same format but invalid, unreadable bytes, non-object) becomes a stand-in `foreignHostGrantV1`: held reservation of its owner (pid+startMs, boot), counted at its declared `reservedMegabytes` or `Protocol.foreignGrantFallbackMegabytes` (2048) when absent/invalid; released when the owner ends (ownerless rows: stale heartbeat). Nothing throws.
- Gate: warns once per foreign grant id (en/de, `foreignGrant`) naming pid, relation, format, size and basis; admission goes on. The old `unrecognizedGrant` error and the "written by a newer version" wait are deleted (no compat).
- `repo capacity status` prints `grantForeign` lines: "older format, none recorded (this reader: 1); held by pid N, M MB counted (...)" / German. The relation is stated truthfully (older / newer / unversioned / invalid).
- A guard test pins a fingerprint of `Grant`, `Handover`, `Phase`, `GrantFormat`: changing the grant definition without raising `Protocol.grantFormat` (and recording the new fingerprint) fails.

## Tests
- `🧪️tests/🪤️foreign-row/` (fixture with 8 foreign rows incl. the inline-stats incident row, Python oracle for classification and held megabytes, status listing, release on owner end, fingerprint guard).
- `🧪️tests/🔐️ledger-update.test.ts` rewritten: unfamiliar rows are held (sqlite oracle kept), and a waiter beside a foreign live grant is admitted with a bilingual warning.
- `🧪️tests/🧾️grant-record` asserts the sidecar.

## Verification (load 50, nice 10)
- `repo typecheck` of the 6 touched source/test files: passed.
- bun: foreign-row 5/5, ledger-update 3/3, grant-record 4/4, elastic-reservation 7/7, handover 9/9, blockers 5/5, leaked-images 3/3 and every other 🧪️tests file of host-capacity passed except below (warden folder excluded: the real-tick overhead test needs minutes under this load).
- Not mine / unverified: `🤲️lend/🚪️gate` (third worker is admitted instead of waiting: lending arithmetic, no foreign rows involved; the tree has heavy peer churn so HEAD is no baseline), `👻️orphan` (broken-pipe "closed"), `🦀️cargo-grant` (python3 lacks `tomllib`), `🔬️footprint` (macOS RSS vs footprint).

## Landing
Outage-class (one older row refused every admission). I edited the live host-capacity tree directly (not through `repo land`, no stage): files `📏️decision`, `🎫️gate` (+`📖️status`, `🤝️handover`), `🧬️schema`, the repo `⌨️command`, and the tests above. Compatibility with older processes: the row bytes are unchanged and the new `<id>.format` sidecar is ignored by older readers (they list `*.json`), so older readers keep reading new rows. Older writers' rows are now isolated, never fatal.
Caveat: processes still running the pre-fix reader keep refusing on an inline-stats row until they restart; fresh processes (and the next serve/test) are immune.
