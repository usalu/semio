# Graph Farm Memory and Worker Retirement

The interrupted canonical NativeDependencies owner340161 observed13416MB against8190MB reserved according to the parent's official MCP sample at04:38:05Z. The durable log reached520/560 projects. Parent governor messages sampled4106MB against4095MB and later resumed below the threshold. The actual process-by-process memory split was not captured by this agent; totals do not identify whether TypeScript programs, source capture, parent concurrency or worker overlap dominated.

## Read-only Source findings

The graph parent uses createGraphParentMemoryGovernorV1 in build/memory, charging max(rss,heapUsed+external) with zero baseline discount and refreshing the actual ledger reservation before admission. This samples only the parent process; it cannot enforce a total-lineage bound or prevent an already-running project from growing. The qualified reservation is partitioned into parent, fixed overhead and owned worker shares. No estimates, shares, floors or limits have been changed.

Project-nodes creates a shared inference environment and starts its parent governor around complete inferProject operations. The farm can adopt staged worker records; after a worker fails/lost/declines, the parent can perform genuine fallback inference. The actual farm worker retains one session environment between jobs. It reports RSS only after a successful job; configured runtime heap variables influence collection rather than representing an OS resident-page limit. Root independently owns the TypeScript program reuse and conditional/memo review; this report does not infer their cause.

## Proven retirement defect

The pool's previous retire implementation immediately removed the child and marked its slot empty before requesting graceful shutdown or observing process exit. The next pump could launch a new process into that same paid slot while the old heap remained resident. Pool close only awaited children currently assigned to slots, so it could return and release ownership while an already-retiring process remained live.

A bounded genuine test with a declared delayed-exit child port reproduced both properties, without running inference or allocating large heaps. Normal causal gate graph-farm-retirement-causal-red (PID352127) observed beforeExit2 instead of1 and closedBeforeExittrue instead offalse. The child port uses Node EventEmitter; independent SQLite expectations check the neutral JSON rows, while Ajv validates actual existing production Init/Run/Ready/Result/Shutdown messages. An initially added corpus selfshape schema was removed entirely; test corpora do not introduce schema authority. This proves the lifecycle defect, not its contribution to the13416MB historical sample.

## Repair and bounded validation

The existing production pool schema description now states replacement after exit and close after every generation exits; no new DTO or schema authority was introduced. Pool slots remain retiring until an actual exit/close event. A tracked set retains every retiring generation; close joins that set, including generations that retired before close began. The existing graceful shutdown and kill deadline remain unchanged. The retirement timer is cleared when exit is observed. Concurrent close calls receive the same settlement promise.

The first corrected neutral gate graph-farm-retirement-current (PID352898) closed0/null. Final neutral/SQLite/actual-production-protocol gate graph-farm-retirement-production-contract (PID354671) closed0/null at04:44:48.732Z. Bounded existing real-process echo/crash/timeout/recycle/cancel/identity gate graph-farm-retirement-real-workers (PID353481) closed0/null:6tests460assertions. No full graph or NativeDependencies replay has been launched; historical overrun attribution still needs evidence.

Files: pool/🧬️schema/🔣️.json, pool/🟨️.mjs, farm/🧪️tests/♻️retirement/{🔣️.json,🟦️.ts}. Every added path is within256characters.

## Capacity release remains owned through child exit

A second read-only trace found the outer farm returned a disabled pool host grant immediately in report(disabled), before worker retirement. The first probe356114 exposed a wrong test import and is not causal evidence; after correcting that import, genuine gate graph-farm-release-delayed-causal-red356330 produced1pass1fail: the host grant had already been returned before the delayed child exited. The production farm now shares one closePool settlement promise and releases each pool only after that promise settles. Disabled and declined pools still close before the overall graph completes; rejection is observed and remains reportable by outer finalization. Release is idempotent, avoiding duplicate callback invocation after final close. Existing error propagation and all host limits remain unchanged.

Final delayed replacement/close/host-release gate graph-farm-release-retained-current356625 closed0/null at04:47:01.062Z. Existing early host-release-before-overall-graph-close law graph-farm-release-existing-current357055 closed0/null at04:47:34.303Z. The earlier6real-process tests/460assertions remain accepted. The new neutral test uses the existing production protocol schema via Ajv and SQLite expected rows; there is no corpus schema. No fullgraph replay was submitted, and historical13416MB attribution remains unresolved.

## Actual failed-job retention and bounded repair

A bounded census of the original340161 log found zero worker recycling messages. The first eight reported worker failures all said Input closure staging path exceeds256characters (2d-compute-rs, compute-ownership-runner, 2d-regions-rs, framework-tool-run-rs, 2d-retire-rs, infinite-world-r3f, framework-editor-rs, framework-diagnostic-rs). Reporting intentionally stops after eight failures, so this does not count all failures. One worker was later lost/exited.

The actual worker protocol measured RSS only for successful results. The pool incremented served and applied both memory- and project-count-based recycling only to success. A repeatedly failing worker could therefore retain all session/parser/cache memory without either existing recycle criterion being evaluated. Parent fallback additionally recomputed those same projects. This is a concrete mechanism on the observed failure path, though the historical total has no per-process allocation breakdown.

The production Failure protocol now requires measured rssMegabytes, supplied by the same actual worker loop for ordinary and captured roles (the captured role literally imports that worker). The pool records successful and failed work time/RSS, counts every completed job, and applies the unchanged recycling policy to either outcome. No ratio, floor, estimate, limit or external runtime dependency changed. The language-neutral delayed-exit matrix covers result/failure crossed with memory/project retirement; Ajv checks actual production messages and SQLite independently checks outcomes. Causal gate358487 failed because a failed job kept its first generation. Corrected358902 passed. Final359493 also launched two actual worker processes whose real failing jobs each reported measured RSS and recycled/retired under the existing test policy; it passed3tests with no full graph.

## Portable closure publication cause and repair

The precise staging refusal was in build/inputs/MJS publishInputClosureV1. Its temporary was the full final hash filename plus tilde and a36-character UUID. A69-character final hash filename therefore incurred37 additional characters. Genuine physical filesystem gate360959 reproduced refusal of an otherwise valid final path exactly256UTF16 units long.

After overlap clearance from Root, publication now uses join(dirname(path), "~" + the existing randomUUID()). Same-directory exclusive wx creation, atomic rename through the existing portable interface, original final hash filename, verified bytes and finally cleanup remain. Both final and temporary paths are explicitly checked against the unchanged256bound, including scalar and UTF16 lengths;257is refused before file creation.

Production limits schema documentation names this invariant; no test corpus schema was introduced. Genuine physical gate361310 passed2tests: public compactSourceInputs published at exactly256with authentic Node/Bun SHA256/final token/JSON proof,257refused, and actual partial-write/rename/cancellation effects removed their exact temporary files. Existing publication entry-replacement, hardlink and corrupt-byte laws361975 passed after supplying the new join/dirname test ports.

Cleanup/retention audit: the publisher's finally block removes its exact local temporary; worker completion records only explicitly published final leaves, and farm retirement/reclamation removes its whole owned staging directory under the unchanged farm-directory prefix. The closure verifier and reference-retention scanner admit only64hex.json final names; both previous hash.json~UUID and current~UUID temporaries are ignored. No old-filename-specific temporary matcher was found, and no alias or relaxed path exception was added.

The bounded rerun of six existing real-worker regressions362151 closed0/null at04:55:04.343Z (6tests460assertions), after the failure protocol and publication changes. No canonical full graph or NativeDependencies replay was launched after measured-overrun cancellation.

## Fleet check before any replay

Official capacity/fleet observations around04:57UTC showed only two grants: course Dev10950 (5766MB reserved,3377MB observed) and the schema discovery target362816 (6318MB reserved, about2911MB observed), with no waiters. The third fleet run entry was the existing Dev Vite child66263. A bounded read-only proc command census matched only the schema launcher/logger/issuer/captured-child lineage362779→362780→362809→362816→362847 for discovery-boundaries; no graph-farm-worker role or previous NativeDependencies issuer was present. No additional declared fullgraph run was found. Raw observations are generated/graph-post-repair-capacity.json, graph-post-repair-fleet.json and graph-post-repair-process-census.json. This is a point-in-time observation, not a future liveness claim. Root is preparing an independent parent/worker RSS/PSS sampler. The schema agent owns its genuine canonical gate; this agent submitted no replay or new Source edit after the accepted bounded checkpoint.
