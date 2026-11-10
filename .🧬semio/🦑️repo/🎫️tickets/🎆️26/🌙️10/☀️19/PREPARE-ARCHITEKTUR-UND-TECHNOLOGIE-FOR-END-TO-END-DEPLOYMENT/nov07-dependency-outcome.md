# Native Dependency Recovery Outcome

The retained all-scope recovery log `generated/deps-nov06-all-current-recovery.log` ends after preservation ownership frontier 471488 with `Current completed Source binding changed` from capsule retirement. The owned command subsequently reports status 1. Its last modification time was 2026-11-06 15:57:41 UTC. This is a failure, not an all-scope current or Ready receipt. The prior root-only ensure is not an all-scope proof. No recovery producer has been launched while exact process and capacity revalidation is pending.

Retained execution handle 63310 now returns unknown process ID. That result alone does not prove orphan absence. Current source checks the endpoint link target and captured identity before and after payload operations; the observed error does not identify which identity field differed or the cause.

The root retained capsule `.pf898dc6d` has intent and completed receipt; its receipt target matches the current root Node readlink. No original captured stamp or failed endpoint was logged, so this does not identify the failing endpoint. Native Windows descriptor entry exposes an NT substitute target while captureBinding exposes a public normalized target and keeps the original entry as its identity. The retirement guard previously compared the raw entry to the public target. The existing real Windows retirement integration reproduced the same failure (exit 1); after changing the check to compare the current raw target with the captured raw identity target, it passed 1 test and 5 assertions. The initial public target qualification and full stamp comparison remain. A full suite initially passed 12 tests but failed an existing POSIX-specific churn message assertion on Windows; that assertion was made platform-neutral. A real mid-walk binding replacement test was added; the complete rerun is awaiting normal capacity admission. This is not a Ready receipt.

The complete native retirement suite passed 14 tests and 130 assertions, including real current-link replacement during traversal, retained payload, four cancellation/recovery boundaries, and schema/Ajv/SQLite validation. Root confirmed old processes absent and capacity empty before this recovery; independent review found no concrete defect. One current-source canonical all-scope synchronization was launched as retained session 79991 with normal 2048 MiB/one CPU, output under generated/deps-nov07-current-recovery.log. No all-current or terminal outcome is yet claimed.

Recovery session 79991 was admitted at its normal 2048 MiB budget and passed source capture into implementation-source/source.served. Its retained handle remains live; observed heartbeat at 270 seconds did not establish current inner phase or readiness. A separate Node read timed out under host load and succeeded on retry; no producer restart occurred.

The retained current recovery progressed through dependency store ownership (frontier 428224 observed) into `dependencies:store-stage`. Session 79991 remains live; neither phase is an all-current receipt.

Root revalidated exact writer PID 12572, birth 2026-11-07T02:43:01.3718540Z, parent 6064 and Source 35fff2… across 10.111 seconds: CPU +2125 ms, reads +592 operations/+3461 bytes, writes +5/+1964 bytes, Other +8812/+950792 bytes. Native MCP lease query showed no waiters or cycles. This is actual activity, not proof of exact stage or readiness.

Current recovery later advanced from store-stage to `dependencies:store-input-replay` at approximately 2040 seconds, with retained session live. This is a phase boundary only, not an all-current result.

Fresh retained 79991 handle poll remained live. Last semantic label was store-input-replay at approximately 2080 seconds, followed by outer heartbeats through 2270 seconds; there is no current-scope DATA or terminal outcome.

A second bounded I/O probe was cancelled while still unadmitted under normal capacity pressure; it produced no measurement. The first sample only establishes activity during its own interval. Recovery 79991 remains the single live producer.

A subsequent admitted second process sample revalidated the same PID 12572, parent, birth and image over 10.086 seconds: CPU +78.125 ms, reads zero, writes +24 operations/+11217 bytes, Other +380 operations/+560 bytes. This shows low activity during that sample and neither semantic progress nor deadlock.

The live recovery log subsequently advanced to a shared managed Bun executable resource, with its active-resource timer observed at 213 seconds. That label does not establish the provider outcome or all-current readiness.

The bounded independent child-tree probe identifies Bun PID 29576, parent 12572, birth 2026-11-07T03:39:36.0646050Z, classified install. Across its ten-second sample CPU increased 125 ms and read/write bytes increased 665036, with 216 additional read operations. This is genuine child activity, not a readiness receipt; see nov07-dependency-child.md.

A governed 768 MiB native diagnostic (`nov07-installer-diagnostic.ts`, exit 0) revalidated child 29576 parent/birth/image and observed `install`, `--frozen-lockfile`, and `--ignore-scripts` in its command, without `--offline`, `--silent`, or `--verbose`. A momentary TCP state query returned no connections owned by this PID; this does not rule out network activity or identify its current await. No installer output/error was present in the recovery log at inspection. No process or connection was modified.

A later normal-budget exact-identity diagnostic at 03:58:38 UTC observed nine installer threads: one Wait:UserRequest, two Wait:EventPairLow and six Wait:Unknown; the point-in-time owned TCP state set was empty. These scheduler states do not identify handles, errors, current await or deadlock. The same diagnostic finished exit 0; no installer process was changed.

Output-route trace: `runJavascriptDependencyStoreProducerV1` invokes `captureOwnedCommandResultV1(process.execPath, args, ...)`; its implementation spawns separate stdout and stderr pipes and registers data handlers on both before awaiting the child terminal, with bounded accumulated output. Thus the proposed sequential wait-before-read mechanism is contradicted by the source route. Captured output is returned on terminal rather than streamed to the outer recovery log; absence from that log does not establish that the child emitted nothing. This source observation does not rule out event-loop starvation or other mechanisms. No output handling was changed.

A bounded read-only immediate-generation census found `j/b562d818bb7622708d0c` generation mtime 03:39:36.213 UTC, close to child birth 03:39:36.064 UTC. Its node_modules mtime was 04:03:55.111 UTC and immediate entries counted 810 at both 04:06:57 and 04:07:13. This correlation is not proof of the child working directory, and unchanged immediate entries do not exclude deeper activity. No recursive walk or modification occurred.

Recovery later advanced from managed Bun resource (~1723 seconds) through ownership traversal into store-input-replay; the retained handle remained live. Current Source checks installer terminal status and signal before its following fence, so this sequence is consistent with successful installer return, but no separate installer receipt or all-current DATA was logged.

The current producer advanced to `dependencies:workspace-edges` (3647 observed) after installer and replay. This is further declared graph traversal, not all-scope Ready.

The live producer advanced its workspace-edge traversal from 3647 to 14591. The log provides no overall denominator or all-current receipt.

Fresh session 79991 poll remained live and workspace-edge traversal reached 29055; no all-current DATA or terminal result.

The current run advanced beyond workspace edges into `javascript-installed-paths:60404`; a fresh 79991 poll remained live, with later outer heartbeat through approximately 7231 seconds. No all-current DATA or terminal result was observed.

Source trace of bare `javascript-installed-paths:60404`: it is a scan entry counter emitted after a work threshold, followed by ancestor identity checks, `control.yieldContinuation()`, another check, and resumed current directory enumeration or child traversal. After traversal, the caller can encode/seal a directory ledger and perform more fences without that marker. It does not identify the current await, imply a completed traversal, or prove a lease wait.

Root revalidated writer PID 12572 and Source identity in an admitted ten-second sample at 04:53:47–57 UTC: CPU +796.875 ms, reads +570 operations/+3341 bytes, Other +7217/+546630 bytes, writes unchanged, working set 873.9 to 875.7 MB. This establishes metadata/I/O activity only during that interval, not completion.

Issued resource control wraps `yieldContinuation()` with remaining-authority checks before and after the original invocation yield. The original process invocation synchronously records watchdog progress and renews accepted physical cleanups, then awaits `setImmediate`; this yield itself does not acquire host capacity or await loan credit. That contract does not establish where the live writer currently is.

The retained recovery subsequently reported `dependencies:store-input-replay` at about 8902 seconds after the installed-path checkpoint; the handle remained live. This establishes a new phase event, not all-current or terminal proof.

Following the replay event, a further managed Bun operation began; its active-resource timer reached 166 seconds while 79991 remained live. Source loops through fresh scopes, but the event does not identify which scope or prove all-current readiness.

Fresh 79991 poll remained live during its second managed Bun operation, timer approximately 1845 seconds at outer elapsed 11072 seconds. No scope/current DATA or terminal result; elapsed time does not establish a deadline or hang.

Root normal-budget exact child sample confirmed a new Bun child PID 3268, parent 12572, birth 05:16:49.936072 UTC, flags install, frozen lock and ignore scripts. Across 05:56:00.389–05:56:10.501 UTC it used 78.125 ms CPU, added 217 read operations and 29,787,095 read bytes, 29,787,095 write-transfer bytes with no extra write operation, and 3930 Other operations/81062 bytes; working set was approximately 69 MB. This is interval I/O evidence, not install or readiness outcome.

Fresh retained 79991 poll remained live during the second managed Bun operation at approximately 2957 seconds; no scope/current DATA or terminal result was reported.

The second managed Bun operation reached its last resource marker at approximately 3280 seconds, followed by ownership traversal and store-input-replay; session 79991 remained live. Source checks installer status before the subsequent fence, but no separate scope/current receipt or all-current proof was logged.

Fresh 79991 poll remained live after the second-install replay; latest outer heartbeat reached approximately 13323 seconds and no current DATA, explicit wait, or terminal result appeared.

Read-only scope planning: current `scopesPlan(all)` starts root manifest and examines direct nonhidden, non-output, non-collection child manifests via the root `*/package.json` owners declaration, selecting `semio.workspace` declarations. Six inspected authored top-level scopes with Bun locks are `.`, `♻️mit-bestand`, `✏️s`, `🌎️hub`, `🎓️teaching`, and `🏢️semio-tech`. Existing older bindings metadata explicitly maps them respectively to generation prefixes ccd79fc7, 34ce8540, a0549e46, 5c770cf5, 67e4f6ec, and b4e10eb2. These are historical observations, not validated current readiness. New b562d818 metadata lacks bindings; no current child or completed-scope association is claimed.

After the second installer and replay, the live producer entered another workspace-edge traversal (2687 observed). Counter resets reflect a new pass and do not establish scope completion or percentage.

The second post-install workspace-edge pass advanced from 2687 to 31423 on fresh live 79991 poll; no all-current DATA or terminal result.

The second workspace traversal advanced into `javascript-installed-paths:58843` (~27 seconds scan); 79991 remained live with no current or terminal proof.

Fresh 79991 handle remained live, last inner marker installed-paths:58843 with outer heartbeat through ~16131 seconds. No all-current DATA, explicit wait or terminal result.

After the second path checkpoint and a quiet interval, the live producer emitted `javascript-installed-paths:66039` and then `dependencies:store-ownership:readText:1232768`; these phase labels do not establish a denominator, the exact prior wait, or all-current status.

After the later installed-path and ownership events, the live producer entered another managed Bun operation (timer ~39 seconds). No scope/current DATA, completion count or all-current proof was logged.

Fresh retained 79991 poll remained live. The further managed Bun operation last reported at ~1010 seconds, followed by ownership readText:1421568 and store-input-replay, with outer heartbeat ~19073 seconds. No scope/current DATA or terminal proof; no completed-scope count inferred.

The live producer advanced from replay into another workspace-edge traversal, reaching 16895 on inspection. This is not a completed-scope count or denominator.

Fresh session 79991 remained live with the current workspace-edge pass advanced from 16895 to 35647; no scope/current DATA or terminal proof.

The later workspace pass advanced into `javascript-installed-paths:45563`; retained 79991 remained live, with no current or terminal proof.

Fresh 79991 handle remained live; last inner marker installed-paths:45563 followed by heartbeat through ~21921 seconds, with no current DATA or terminal outcome.

Fresh 79991 poll remained live at outer heartbeat ~22763 seconds; last inner marker installed-paths:45563, with no current DATA or terminal result.

The live producer advanced from installed-paths:45563 to store-input-replay around outer 23443 seconds. No current or terminal proof was emitted.

After the later replay, the live producer entered another managed Bun operation (observed timer ~312 seconds). The resource label does not prove scope identity, completed count, or all-current readiness.

Fresh 79991 poll remained live during the later managed Bun operation, resource timer ~1202 seconds; no scope/current DATA or terminal proof. Elapsed time is not treated as a deadline.

Fresh 79991 poll remained live. A later managed Bun resource label ended at ~1999 seconds, followed by ownership traversal/replay and another workspace-edge pass (1023 observed). No scope/current DATA or terminal proof; no completed-scope count inferred.

Fresh79991 poll remained live, current workspace-edge pass advanced from 1023 to 34047; no scope/current DATA or terminal proof.

Fresh 79991 poll remained live; workspace-edge pass advanced from 34047 to 41087 without current DATA or terminal proof.

Fresh 79991 poll remained live; workspace-edge pass advanced from 41087 to 51519 without current DATA or terminal proof.

Fresh79991 poll remained live; workspace-edge pass advanced from 51519 to 63039 without current DATA or terminal proof.

The later workspace pass advanced to `javascript-installed-paths:53335`; 79991 remained live with no current or terminal proof.

Fresh 79991 poll remained live; last inner marker installed-paths:53335, with heartbeat ~30471 seconds and no current DATA or terminal result.

Fresh 79991 poll remained live; last inner marker installed-paths:53335, with heartbeat ~31224 seconds and no current DATA or terminal result.

Fresh 79991 poll remained live; last inner marker installed-paths:53335, with heartbeat ~32024 seconds and no current DATA or terminal result.

Root bounded identity-checked writer sample at 11:39:30–40 UTC showed CPU +390.625 ms, reads +596 operations/+3526 bytes, writes +2/+2538 bytes, Other +3600/+287762 bytes, and working set 768.3→768.5 MB over 10.1004 seconds. This shows activity in that interval only, not path advancement or readiness.

Fresh 79991 poll remained live; last inner marker installed-paths:53335 followed by heartbeat ~33021 seconds, with no current DATA or terminal result.

Fresh 79991 poll remained live; last inner marker installed-paths:53335, heartbeat ~33741 seconds, no current DATA or terminal result.

The producer advanced from installed-paths:53335 to store-input-replay around outer 34175 seconds; no current or terminal proof was emitted.

Fresh79991 poll remained live; last inner event store-input-replay around 34175 seconds followed by heartbeat ~34945 seconds, without current DATA or terminal outcome.

Fresh 79991 handle remained live; last inner event store-input-replay around 34175 seconds, heartbeat ~35735 seconds, no current DATA or terminal result.

Fresh 79991 poll remained live; last inner marker store-input-replay around 34175 seconds, heartbeat ~36565 seconds, no current DATA or terminal result.

Fresh79991 poll remained live; last inner event store-input-replay ~34175 seconds, heartbeat ~37341 seconds, no current DATA or terminal result.

Root fresh native registry inspection found only PID 12572 holders across three registries and zero lease waiters, waitFor edges, cycles or violations; prior capacity snapshot showed the sole 2048 MiB JS grant and no waiters. These observations do not prove finite progress.

The live producer advanced from replay into another managed Bun operation (observed resource timer ~74 seconds). No scope/current DATA or terminal result was reported.

Fresh 79991 poll remained live during the later managed Bun operation, timer ~852 seconds. No scope/current DATA or terminal proof.

Fresh 79991 poll remained live during the later managed Bun operation, timer ~1575 seconds, with no scope/current DATA or terminal proof.

Fresh79991 poll remained live during the later managed Bun operation, timer ~2300 seconds, outer ~40325 seconds; no scope/current DATA or terminal proof.

Fresh79991 remained live; latest managed Bun resource label ended ~2723 seconds, followed by ownership traversal and a new workspace-edge pass at 2495. No scope/current DATA or terminal proof, and no completed scope count inferred.

Fresh79991 poll remained live; current workspace-edge pass advanced from 2495 to 39167 without current DATA or terminal proof.

Fresh 79991 poll remained live; workspace traversal advanced to `javascript-installed-paths:48341`, followed by heartbeat ~42416 seconds. No scope/current DATA or terminal proof.

Fresh79991 poll remained live; last inner marker installed-paths:48341 followed by heartbeat ~43071 seconds, without current DATA or terminal result.

Fresh 79991 poll remained live; last inner marker installed-paths:48341 followed by heartbeat ~43642 seconds, without current DATA or terminal result.

Fresh79991 poll remained live; last inner marker installed-paths:48341, heartbeat ~44427 seconds, no current DATA or terminal result.

The producer advanced from installed-paths:48341 to store-input-replay around outer 44977 seconds; no current or terminal proof was emitted.

The live producer advanced from replay into another managed Bun operation (observed resource timer ~367 seconds); no scope/current DATA or terminal proof.

Fresh 79991 poll remained live during the later managed Bun operation, timer ~1035 seconds; no scope/current DATA or terminal proof.

Fresh 79991 poll remained live during the later managed Bun operation, timer ~1676 seconds; no scope/current DATA or terminal proof.

Fresh 79991 poll remained live during the later managed Bun operation, timer ~2336 seconds; no scope/current DATA or terminal proof.

Fresh 79991 poll remained live during the current managed Bun operation, timer ~2969 seconds; no scope/current DATA or terminal proof.

The producer progressed beyond the later managed Bun operation into store-input-replay around outer 48367 seconds; no current DATA or terminal proof.

The live producer advanced from replay into another workspace-edge pass, observed at 20479. The per-pass counter is not a scope completion count or current proof.

Fresh79991 poll remained live; current workspace-edge pass advanced from 20479 to 43583 without current DATA or terminal proof.

Fresh79991 poll remained live; workspace-edge pass advanced from 43583 to 54975 without current DATA or terminal proof.

The current workspace pass advanced beyond 66431 edges into `javascript-installed-paths:44302`; 79991 remained live with no current DATA or terminal proof.

- Retained recovery 79991 remained live beyond the installed-paths 44,302 marker and advanced to checking declared package providers (latest observed 2,492 edges). This is activity, not a terminal or all-current receipt.
- Retained 79991 later advanced through javascript-preserving-source:323 into dependencies:store-input-replay; outer heartbeat ~52,738 seconds. No terminal or all-current receipt observed.
- Retained 79991 advanced through store-ownership frontier:524416 to another javascript-preserving-source:2 marker; outer heartbeat ~53,362 seconds. No terminal or all-current receipt observed.
- Retained 79991 advanced from replay to preservation-ownership:850432, heartbeat ~59,668 seconds; no terminal or all-current receipt observed.
- Retained 79991 advanced from preservation ownership to dependencies:collect:10496 (shared, operation elapsed ~1,495 seconds). No terminal or all-current receipt observed.

## Recovery terminal

Retained 79991 exited 0 and logged `repo` stage complete. Its DATA first reported obsolete generations, then published epoch 2 generations for all six declared scopes (`.`, `♻️mit-bestand`, `✏️s`, `🌎️hub`, `🎓️teaching`, `🏢️semio-tech`). Normal collection reported six removed and six kept. This establishes successful publication, not a separate all-current or Ready receipt.

## Post-success verification cancellation

One normal all-scope verification 1356 was launched after successful 79991, using installed repo with 2048 MiB/1 CPU. Its captured Source was 35fff2ced85c1fb351c3e6451b210b175ad2e025bd7a2ed0db113b9a8e15aeb0. It advanced through current-input, replay, ownership and preservation checks without current/stale DATA. Root directed normal cancellation to free capacity for the required proctor build, not because of a timeout, hang or stale result. The owned exec received Ctrl+C and returned outer exit 1. The bounded pre-interrupt chain and fresh post-check are in generated logs; the complete post-check saw none of the captured PIDs. The interrupted command may have written material; no all-current result is inferred. Capacity and lease release remain separately subject to registry verification.

Root post-cancellation capacity snapshot 1794085028743 reported no grants and no 1356 session, with proctor and infrastructure waiters. Default metadata lease snapshot 1794085034627 showed zero waiters, wait-for edges, cycles or violations, but still had two queue-sourced shared records for PID 240 despite its absence from CIM. Therefore full registry cleanup is not established; canonical normal retirement/recheck owns stale metadata. No manual cleanup was performed.
