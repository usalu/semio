# Recovery Audit — 2026-11-05

At 09:11:38, 09:11:53, 09:12:08 and 09:12:23 UTC, bounded localhost:6063 probes returned ECONNREFUSED. No app identity or browser readiness was observed.

Dependencies-only doctor at epoch 1793869785625 exited 1: probe status ok, pointers absent, scopes empty, 48 unready, 0 orphans. Leases snapshot showed 18 held, 0 waiting, 0 cycles, 0 recent order violations; machine admission was 6 granted and 1 queued. Subsequent filtered snapshot 1793869817054 identified peer JS-build holder PID 29656, workspace-edges counter 39999; it does not establish completion or staleness. No repair or ensure was performed.

Native ledger snapshot 1793869816580: historical Hub PID 53092 recorded child.begin then child.error at 1793855292874, followed by consume.end. Historical component PID 40120 latest retained mark was graph.begin; absent later marks are not proof of outcome. Live-process helper snapshot 1793869819736 returned no matching tracked processes; disappearance is not success.

Ledger snapshot 1793869885536: PID 39288 recorded cargo:test child errors at 1793855079246 and 1793855992359 on separate admitted attempts, followed by consume.end. PID 42720 recorded child.error at 1793856023763 followed by consume.end. The ledger has no error detail or lane identity; neither terminal outer result nor attribution follows from timestamps alone.

Read-only lane.json metadata identifies p53a89c as workspace scope, agent 01a1a852-623d-7313-bc62-fbc8be52e14b; p663a6f as peer WFC engine/job/oracle/solve scope; p765709 as semio image-editor scope; pc13d29 as JPG io-registry/mutations/schema-2 scope. Engine independently recovered ENAMETOOLONG uv_spawn compiler outputs in those lanes; lane timestamps alone do not bind these to ledger PIDs. The original recorded output files for Hub, component, catalog, Vitest and host contain delivered historical chunks, not their final outcomes. No descriptor or catalog refresh is established.

## Hub Lane Attribution

Lane p570fe7 lane.json names semio-hub-remodel and root agent 01a1a852-623d-7313-bc62-fbc8be52e14b. Its newest retained compiler output is semio-framework-os-store-text-io/ac6936c3ba5ca7d2/fingerprint/output-lib-semio_framework_os_store_text_io, exact content ENAMETOOLONG: name too long, uv_spawn. Its mtime 1793855290698 is 2176 ms before historical Hub captured PID 53092 child.error at 1793855292874. This is strong lane/time correlation, not a direct PID-to-output provenance record or proof of an outer terminal result.

## Descriptor Snapshot

Composition directory 🌎️hub/🧩️compositions/📸️remodel/🧩️component contains 🔣️.json (472641 bytes, SHA256 eaf0a364204e6a445d7fb9cc48c4c27c38357b052f0e10b8b490b1530cf023b4) and 🛂️.descriptor.semio (134583 bytes, SHA256 b1fc1ec23958b86f2812186ce4c989bfcaaeb6373462166bb0880fc5091c7a5c). Both mtimes 2026-11-05T06:35:56.495436Z. JSON exportQcReport semantics description in both locales still specifies remodeling-qc-report.ops, inconsistent with the current repaired .json manifest/handler. This is direct stale-JSON evidence; binary regeneration/equivalence and catalog publication remain unproved. Historical owner-root paths in older notes are absent in the current layout.

## Response-File Review

Reviewed compilerArgumentsV1 and run cleanup in repo unit-cache script and Windows nested-argv test. Contiguous generated response spans preserve ordering and keep existing top-level @ references in place, avoiding unsupported nested expansion; empty and newline arguments stay top-level. Temporary files are cleaned on child close and synchronous spawn throw. Current near-limit test checks a custom response echo and does not itself establish real rustc interpretation. Requested real-rustc coverage of embedded/leading/trailing spaces, quotes, backslashes, Unicode, empty/newline arguments, and interleaved top-level @ paths with spaces. No conclusive defect found from static review; native compiler proof and full suite remain pending.

Engine subsequently added a real-rustc comparison with interleaved response paths containing spaces, quoted Unicode/backslashes and direct-versus-wrapper empty/newline failures; 1200 repeated arguments trigger the conservative response threshold. A nonzero child test verifies files exist during child execution, are removed afterward, and exit 7 is preserved. Engine reports normal suite 9 pass, 1 expected skip, 64 assertions. Static review found no further actionable defect; this does not constitute a Remodel native result.

## Scheduler Review

Engine found that scheduler parsed raw response references and added one-level expansion for unit admission. Static review confirms expansion keeps ordering and original arguments are retained for compiler execution and thread-file handling. Asked engine to assess -Z and threads=N split across response boundaries: thread bounding currently processes top-level arguments and each response file separately, though rustc expands a single ordered argument sequence. Also noted non-UTF8 direct OS arguments now fail closed instead of previous passthrough. Reachability and native cargo test are pending.

Engine reproduced the boundary defect RED and changed bounded_argfiles to retain previous -Z state across direct and response-file spans. It now fails closed on response read/create/write errors, exclusively creates owned copies and cleans partial outputs on error. Engine reports scheduler suite GREEN 8/8, handle 88280. Re-review found no further actionable defect. Non-UTF8 argument refusal is intentional fail-closed; Remodel gates remain separate.

Final joint review: bounded copies use short system-temp paths, process-local atomic index, exclusive creation and a 256 UTF-16-unit limit. Wrapper retains async spawn error until close cleans files. Engine reports scheduler 9/9 and Windows suite 10 pass, 1 expected skip, 67 assertions, including 245-unit original path preservation and ENOENT cleanup. No further actionable defect identified in static re-review.

Engine additionally reports production broker proof GREEN using retained 🐍️scheduler-broker-proof.ts and a freshly Cargo-compiled scheduler with build-success receipt: real rustc exited 0 and real broker reported response_witness. This is admission evidence, not a complete Remodel native gate.

Bounded localhost:6063 monitoring continued at roughly 45-second intervals through 2026-11-05T09:58:24Z; all observed results remained ECONNREFUSED, without service identity.
