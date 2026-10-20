# Fresh Entry Receiver Audit

Reviewed draft entry.ts a1b2113… and receiver.py1e6c1b… without commands or private effects.

## Material Defect

receiver.py:34–37 feeds untrusted tar metadata to tarfile.open(mode=r|). Python processes PAX/extended header payloads before yielding the member on which the256MiB limit is checked. Therefore that check does not bound extended-header allocation; the transport allows up to4GiB. Bound physical tar headers/extension payloads before handing them to tarfile, or use a bounded narrow receiver protocol. Add an actual oversized-PAX rejection test; Python AST parsing is not a behavioral oracle.

## Custody And Truthful Limits

entry.ts:32 supplies one stdin child, serial write callbacks/backpressure,64KiB chunk and4GiB stream ceilings, bounded combined output, issued operation and cancellation/deadline signaling, then awaited child close. Guest receiver has its own900s alarm. Client close alone does not settle guest: success additionally requires actual receiver success, matching Source manifest digest/length/rows and post-authentication. Failure controller attempts exact workload evidence and dedicated-builder stop, retains Source volume and records cleanup failure/unknown as appropriate (:34). No whole-builder deletion occurs.

The receiver checks canonical bounded paths, exclusive nofollow regular-file creation, duplicate/type/size limits, streamed content hashes, final manifest last, physical rehash and stable descriptor/named identities. Source rows are written directly into the fresh Source volume, never copied into ticket. Parent directory checks are not descriptor-relative; current safety relies on exclusively owned fresh empty Source and no concurrent guest writer. This limitation should remain explicit.

Cleanup reserve60s currently has worst declared command ceilings55s: two authenticate calls each2×5s, workload evidence5s, stop25s and final inspect5s. Each port invocation caps timeout by actual remaining budget. This leaves5s for local work; any exhaustion must remain unknown, not successful terminal. Pre-authentication before the try can fail before a receiver child exists; no guest terminal should be invented.

Creation verifies pinned image ID/Linux/amd64 and actual empty Source, Bun/Python/Compose availability, versions and Node platform. It does not prove a complete Rust/linker/compiler roster, exact version requirements, daemon readiness or successful product release. Docker data-root/exec-root/pidfile are explicitly outside/repo (:29), with Source only mounted at/repo. Fresh image digest is concrete admission, toolchain completeness is still an acceptance requirement.

Current entry.tests.ts contains13 admission cases comparing actual readAdmission with Ajv and SQLite plus Python AST parse. No actual receiver tar behavior, transport cancellation, creation cleanup, or dedicated-stop branch is exercised by these tests. These are prepared, not executed tests. Receiver-level neutral execution is required before use; no physical Docker test is implied.

No Source, frozen helper or private runtime changed. The extended-header issue was sent to Release and Root.
