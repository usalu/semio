# Current Controlled Lease Capacity Snapshot

Core's retained `lease-diagnostic/📜️script.ts` was executed from the actual repository root with the exact workspace and ticket generated-output environment. The corrected run terminated zero in 0.126 seconds and wrote `🗑️generated/lease-capacity-current.json` and `.log`. Its Original policy is 60,000 ms. It uses the declared repository budget and genuine shared native lease-image API directly, with no repository opener's automatic retirement, exclusive probe, maintenance, reset, resize or manual removal. Bounds are 8,192 entries per directory and 64 read-only identity samples; both directories completed uncapped and had stable directory metadata.

| Measurement | Actual snapshot |
|---|---:|
| Physical filesystem bytes | 66,019,328 |
| Physical available bytes | 30,797,824 |
| Root data entries / directories | 3,994 / 1,110 |
| Root data SQLite files | 2,883 |
| Root data logical / allocated bytes | 27,811,840 / 27,811,840 |
| Claim files | 21 |
| Claim logical / allocated bytes | 65,536 / 65,536 |
| Metadata headroom reserve | 4,194,304 bytes, regular |
| Read-only identity samples | 54 |
| Samples refused before SQLite reading | 13 (bounded regular SQLite check) |

The canonical catalog was a stable regular 8,192-byte file, application ID 1,397,574,739, two 4,096-byte pages, exact resource/filename agreement. The read-only connection reported `max_page_count=1073741823`; this observed connection value is not proof of a failing writer's configured limit. No active/inactive claim classification was attempted: safe retirement eligibility remains **unknown without genuine exclusive claim probes**.

This current snapshot has substantial free physical space. It does not establish conditions at the combined suite's earlier SQLITE_FULL failure, identify its exact resource or per-database page limit, or explain the separate prior warm-recomputation failure. Root assigned Core a narrow opt-in fault-boundary diagnostic; the next authentic combined gate will preserve exact primary errors and Original limits. No speculative retention correction follows from this snapshot.

The first adapter invocation stopped on a Bun directory-close return-value assumption before publishing a receipt; Core corrected the adapter, and the above receipt comes only from the successful second invocation. No additional repeat was made for Core's later primary-error cleanup refinement.


## Combined Diagnostic Gate

After Core added the opt-in original-error boundary (without lease/retention algorithm changes), the sole authentic combined `--graph-gate` ran with `SEMIO_LEDGER_DIAGNOSTIC=1` and `SEMIO_LEASE_DIAGNOSTIC=1`, unchanged Original 120,000 ms. Session 77906 terminated zero: **97 pass / 0 fail / 2,186 assertions / 25.19 s**. Evidence: `🗑️generated/final-graph-gate-lease-diagnostic.log`.

Both real optional dataset images were published (1,813 and 1,814 bytes), their three captured facts were current with no stale row, and available graph bytes were approximately 1.848 GB. Their original one-computation warm assertions passed. No SQLITE_FULL or associated fault diagnostic was emitted. The earlier two failure causes therefore remain unestablished; this green does not retroactively explain either. No cleanup, retry suppression, expectation weakening or speculative retention algorithm was used.
