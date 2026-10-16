# NESTED23311 Timeout: Bounded Source Investigation

This investigation read only the retained closed log and repository Source. It did not execute a backend, open a cache/image/database/store, inspect a live process, change Source or raise the fixed 120000ms limit. Exact nofollow descriptor/named pre/post stamps, Source hashes and comparison with the historical 462-file capture are retained in `🗑️generated/nested-timeout-source-investigation.json`.

The backend, source-closure capturer, Bazel storage owner and cache-store owner inspected here all match their hashes in `bazel-aggregate-source-readiness-67458.json`, the receipt named by the actual attempt’s preflight. The fixture, dispatch owner and `.bazelrc` were not entries in that receipt, so their current inspected bytes are not claimed as a historical capture proof.

## What the Actual Log Establishes

The stable log is 13030 bytes/168 lines, SHA256 `b74145ec77ffa1a5b9cff6f844d9237e6ffa9d79804690208fd2fad8a9356c54`, matching the earlier independent proof. The warmup build completed in 11.176s and printed its first execution. The outer nested target was analyzed, action-birth authorization was reached, and Bazel reported `fixture:nested` running with a last build heartbeat at 30010ms. The parent then reported `build:bazel:nested stopped: timeout 120000ms`. There is no nested completion or test aggregate. This is not the earlier SQLITE_FULL refusal.

The received Original had policy maximumElapsedMilliseconds=0 and a null wire deadline. Dispatch separately funds the real Bun test child for 120000ms; the runtime test also declares 120000ms. Internal zero grants retain that Original and do not give the nested operation another 120 seconds. Reported build elapsed times exclude prior storage admission, source capture, captured planning and the separate Bazel info invocation. Their durations are not recoverable from repeated stage messages, and no attribution below is inferred from message counts.

## Actual Nested Command and Lock Domains

The fixture first calls `runBazelTasksV1` for fixture:build, checks count=1, removes the product, then calls it for fixture:nested. Nested is an uncached command `bun ./📜️script.ts nested`, with no dependency and no declared outputs. Its generated script receives its real process Original and calls `runBazelTasksV1` again for fixture:build, parallel=2, under the existing zero-grant phase `bazel:nested-fixture`. The inner build is expected to restore the warmup cache hit: final count must remain1 and product must be FIRST. It is not intended to execute the compile body twice.

The exact Source namespace hashes are:

| Selection | Workspace Namespace | Workspace EX Resource | Schedule Resource |
| --- | --- | --- | --- |
| fixture:build | 234823c9f31c8317 | bazel-workspace-234823c9f31c8317 | bazel-task-schedule-689f4ba8c4e82f34 |
| fixture:nested | 2d97587694e1e15b | bazel-workspace-2d97587694e1e15b | bazel-task-schedule-dc0efbb20f8c2d15 |

Workspaces are canonical products root `/bazel/workspaces/<namespace>`. The warmup workspace EX is released before the outer nested request begins. The outer request holds the nested workspace EX while its child requests the distinct build workspace EX. Schedule leases default to SH here and also use distinct workspace hashes. The selection chain is extended with the outer namespace; selecting build from nested is not the same namespace and passes the explicit recursion guard. No self-cycle is visible in these resource names.

Normal build/info commands use the same output_user_root but distinct workspace cwd and no explicit output_base. The observed warmup base `cd1a25dd86037cc77309d5cd6c1918f1` equals MD5 of the derived build workspace path. Applying that observed relationship to the nested workspace gives `591f75f34d73366d15a2b25794712692`; that nested base was not recorded as an actual observation. A shared output root alone therefore does not prove a same-output-base deadlock. Current `.bazelrc` has no output-base override. Dedicated explicit output_base is used for continuous launchers, which this fixture does not select.

Common module setup, cache-charge and native-storage ledger writers are short scoped calls completed before the action runs; the stores retain image readers rather than those charge writers through the action. Snapshot restore EX is released before executing a dependent command. The normal store opener shares/reopens readers rather than requiring whole-image EX. These inspected scopes show no necessary parent-waits-child/child-waits-parent cycle. An unrelated peer wait remains possible; this closed log does not name a contended resource or prove one existed.

## Concrete Repeated Work in the Source

Every runBazelTasksV1 call re-enters tools→dependencies→products storage, executable verification and module readiness. It then captures the full backend Source closure before publishing graph data to a fresh captured planner process. The captured planner opens its own store lifetimes, validates/rechecks the immutable images and emits the workspace graph. The backend separately runs batch `bazel info bazel-bin` before batch `bazel build`.

A complete nested law therefore structurally includes three backend preparations: warmup build, outer nested build, and the inner build request, even when the inner build is a task cache hit. Each publication calls captureBuildSourceClosureV1 directly without a cross-request memo. That capturer rereads each included file with nofollow ancestry checks, descriptor/named checks, sibling schema enumeration and system transpiler import scanning. Historical preflight had 462 Source files/2926112 bytes; it does not measure capture duration. Planner/action loaders also validate images and schemas, and action birth adds an actual owned captured child, rather than directly spawning the tiny fixture body. Cache hits avoid task body execution but do not currently skip this orchestration preparation.

This is a concrete duplicate-work path and a plausible contributor to the 120s total. It is not a measured explanation of the timeout. The outer log contains two completed preparation sequences; inner action stdout/stderr is managed by Bazel and is not streamed as a separate parent-side phase trace. Absence of inner stage lines therefore does not establish that the inner script never entered. Birth has a 10000ms handshake bound and termination receipts have 5000ms bounds, but the closed log contains no handshake/receipt error identifying either as the stall.

## Next Diagnostic for Root

Keep the permanent NESTED command and its 120000ms limit unchanged. During the next separately authorized owned attempt, record monotonic timestamps on actual received parent log lines and take bounded read-only observations of that attempt’s validated descendant PIDs at a few declared checkpoints. Retain full owned argv/cwd, elapsed/CPU state, and—if one Bun descendant remains active—a short owned-process stack sample. This can distinguish source/loader CPU or filesystem work, SQLite wait loops, an inner Bazel startup or an actual output-base wait without guessing from stage counts. No process should be signaled or sampled unless its ownership is proved by the real attempt’s spawn ancestry.

After actual terminal/drainage, Root can inspect only that attempt’s exact authenticated action stdout/stderr or batch command log, if retained, to recover the inner stages that Bazel buffered. Do not read a guessed unrelated output base or mutate cache state. If the same-input observations identify repeated capture/loader work as dominant, design a separate schema-first optimization retaining immutable Source rechecks and genuine ownership; do not bypass capture, shorten validation or increase the cap based only on this Source review.

No concrete deadlock is proved by the available closed evidence. The missing measurement is the nested owned child’s actual stage and wait/CPU state while the outer action was running.
