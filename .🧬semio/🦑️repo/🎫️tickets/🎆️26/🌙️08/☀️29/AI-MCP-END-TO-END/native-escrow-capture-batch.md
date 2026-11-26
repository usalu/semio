# Native Escrow Capture Batching

The Wake capture batches retain the existing authenticated frozen producer, header, issued history and Source checks. The capture, fixed-header validation, SRE1 validation and claim field staging bodies now have focused native laws around the exact paid code called under those public guards. Ordinary capture and validation check cancellation between microsteps. An explicitly authorized retirement turn can still continue the receipt retirement path.

The neutral fixture has a full ready 152-byte actor output header and two SRE1 rows, one open and one terminal. The native test drives identical resident bytes through a real `OriginalJobFixtureCaller` with its own producer recipient and scoped grants. It compares every captured byte and staged row word, per-window spend, and aggregate item, copy, work, capacity and release receipts. A separate test constrains copy and work to less than one full word and moves the live clock or cancellation after the first paid word.

| Phase | 512-item windows | 3-item windows | 1-item windows | Total paid items | Total copy bytes on a 64-bit machine | Total work |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Capture bytes | 3 | 24 | 71 | 71 | 1,136 | 639 |
| Validate fixed header | 1 | 7 | 19 | 19 | 140 | 19 |
| Validate SRE1 | 3 | 25 | 74 | 74 | 1,126 | 74 |
| Stage one claim | 2 | 9 | 24 | 24 | 385 | 24 |

The shared fixture expresses copy receipts as fixed bytes plus the target pointer-width coefficient. The SQLite oracle independently calculates total work, copy, items, number of funded windows, partial-stop receipts and every hold/redemption/release axis. The repo Bun run passed its one test and 23 assertions.

The initial Wake unit build found one mutable-callback declaration missing in the deadline fixture. After correction, a native relink against the exact 692 cached Cargo dependencies ran the full suite and passed all 33 tests. Both capture laws passed, including paid partial resume, real work-loan expiry and explicitly authorized retirement after cancellation. The same run passed the issued-origin regression with 72 paid probes and 762 work for nine lookups over 2,913 origins. The root post-turn relay package independently passed both capture scheduler tests.
