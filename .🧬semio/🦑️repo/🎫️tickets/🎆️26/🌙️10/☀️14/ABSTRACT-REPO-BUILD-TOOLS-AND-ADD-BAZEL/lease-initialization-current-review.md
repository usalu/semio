# Current Lease Initialization Failure Investigation

The current combined original-120 graph suite (session 79676) completed 96 pass / 1 fail / 2,174 assertions / 26.43 s. The distinct graph dataset concurrency test failed at the actual resource lease initialization COMMIT with SQLITE_FULL. Its preceding first NP publication existed (1,813 bytes), all three Source facts held, and the graph filesystem had about 1.848 GB available. This failure is different from the earlier warm-computation-count mismatch; neither cause is inferred from the other.

The existing `lease-capacity.md` and real 4-test/36-assertion canonical catalog, original timeout, held-claim and close-before-unlink protocol were read before proposing any algorithm change. Current reclamation selects only bounded owned claim databases under canonical catalog exclusion. It is not authority to remove ordinary root resource databases or files held by other processes.

Root authorized one at-most-60-second genuine shared native lease-image diagnostic with no automatic queue GC, maintenance, retirement, exclusive probes, deletion or resize. The ticket adapter uses the actual repository budget and native platform owner directly, preserving an inherited original wire if present and otherwise issuing a genuine original finite envelope. It refuses unbounded or greater-than-60-second remaining authority. Root/claim metadata is capped at 8,192 entries per directory; SQLite identities at 64 bounded regular files. Directory and database stability are reported. Inactive retirement eligibility stays UNKNOWN.

The initial adapter refused because Bun's directory close returns void and the adapter incorrectly attached `.catch`. The ticket-only correction uses awaited close in try/catch, tolerates the already-closed state and preserves a prior body error through directory and image cleanup. The subsequent exact rerun exited 0 in 126 ms and wrote `🗑️generated/lease-capacity-current.json`.

Current read-only result: lease data has 3,994 entries including 2,883 SQLite files, approximately 27,811,840 allocated file bytes; claims has 21 files / 65,536 allocated bytes. The actual metadata reserve is 4 MiB. Available bytes are 30,797,824 of 66,019,328. The canonical catalog is a valid 8-KiB, two-page owned database. There are 54 sampled identity results and 13 bounded-regular-file refusals. The read-only connection's `max_page_count` is a current connection value, not proof of a historical writer cap. This snapshot does not demonstrate that the historical failure occurred on a full physical lease volume or was caused by claim retention.

To distinguish a connection page cap, another private filesystem, transient physical exhaustion or another actual resource, Root released minimal temporary opt-in logging at the existing lease error boundary. `SEMIO_LEASE_DIAGNOSTIC=1` emits bounded DEBUG context only for SQLITE_FULL: exact path/resource and UTF-8 size, initialization phase, configured cap, current connection pages, logical/allocated file bytes, device, filesystem size/available bytes and signal state. Best-effort observation never replaces the original exception. It runs after the existing rollback attempt, so its filesystem/page values are post-rollback observations, not a reconstructed fault peak. No retention algorithm, control policy, image size or quota changed.

High owns the sole current combined gate, session 77906, `🗑️generated/final-graph-gate-lease-diagnostic.log`, original 120 seconds with both `SEMIO_LEDGER_DIAGNOSTIC=1` and `SEMIO_LEASE_DIAGNOSTIC=1`. Aggregate terminal evidence is pending. Source is held during that run; no capacity fix or public runtime acceptance is claimed.

Instrumented lease Source SHA-256: `ed3de9284eff3f367b1fd894a490572b95376d84105fa1116059207f69319f0c`.

## Combined Gate and Final Removal

High's authorized combined session 77906 exited 0: **97 pass / 0 fail / 2,186 assertions / 25.19 s**. Both genuine dataset NP receipts were published and all captured Source facts held. No SQLITE_FULL/fault record occurred. The historical physical-full or page-cap cause remains unknown; there is no evidence for a speculative retention change.

Root then instructed removal of only this temporary production diagnostic. Core removed the exact phase callback/state/calls and opt-in DEBUG block, restoring the existing functional lease initialization/acquisition code. No retention, capacity, quota, timeout, signal or primary-error behavior changed. The authored bounded ticket diagnostic and reports remain. The above instrumented hash is historical, not the final Source hash. Pure diagnostic removal was reviewed without another full-suite execution, per Root's explicit instruction. High was released for one actual harmless public exec after Source held.

Current restored lease Source SHA-256: `9c30f9d5bb1ff1c0914dee648d0de3a259c68a173d709304dd424d89417202e5`.
