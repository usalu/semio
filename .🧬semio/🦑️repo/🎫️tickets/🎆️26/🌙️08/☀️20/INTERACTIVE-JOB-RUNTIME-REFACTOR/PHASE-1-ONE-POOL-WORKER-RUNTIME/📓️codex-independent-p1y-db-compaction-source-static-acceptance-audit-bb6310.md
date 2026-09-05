# Independent P1y DB Compaction Source/Static Acceptance Audit

Date: 2026-08-24  
Auditor: Codex, fresh read-only review  
Verdict: **RED — P1y must not be accepted.**

## Scope And Method

Read completely: repository `AGENTS.md`; Phase-1/master plan, residual/final-matrix material;
P1y caller census; P1x/P1w and P1q boundary/audit material; the P1y implementation report; live
compact, engine facade, CLI, snapshot and index paths; and the root P1y verifier/self-mutations.

Production caller census is clean for the selected cut:

`db CLI cmd_compact` → `Database::compact_document` →
`DatabaseCompactionFuture::try_submit` → one `Lane::Io` driver → retained execution.

The only non-test `db_actor::block_on` in the engine is the separate sync-hello residual. The old
`Compactor` is test-gated, so no eager **production** caller bypass was found. This does not cure
the retained-path failures below. No production source/verifier was changed. No Cargo, Nx, build,
Wasm, browser, or runtime Rust test was run.

## Blocking Findings

### A1 — Index Compaction Ignores The Operation Cancellation And Can Consume A 30-Second/65,536-Fuel Turn

P1y calls `compaction_opportunity(cancelled)` once before each kind, then creates an independent
`IndexHandle` control with `handle.operation_control(65_536)` and awaits the complete
`handle.compact` call at [db compact component.rs:1210](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:1210).

`IndexHandle::new` creates a fresh private cancellation atomic
([db index component.rs:860](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔢️index/🦀️component.rs:860)); its control clones only that atomic and assigns a 30-second deadline
([db index component.rs:864](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔢️index/🦀️component.rs:864)). `grant` observes only this private token
([db index component.rs:139](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔢️index/🦀️component.rs:139)), whereas `DatabaseCompactionFuture::cancel` writes P1y's different atomic
([db compact component.rs:1764](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:1764)). There is no live `handle.cancel()` call.

`IndexHandle::compact` loops all runs, merges them, writes one run, deletes every old run and
collects stats before it returns ([db index component.rs:1046](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔢️index/🦀️component.rs:1046)). A cancellation arriving after P1y's pre-kind opportunity is therefore not observed by the inner control. The claimed one-opportunity/less-than-8-ms cancellation property is false for a real production callee.

### A2 — Snapshot Revalidation Is A TOCTOU Check; The Stale Full Baseline Is Already Persisted

P1y checks `latest_generation` at [db compact component.rs:1274](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:1274), computes an expected successor, and separately calls `publish_retained` at [db compact component.rs:1286](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:1286).

The publication API accepts no expected generation. It rereads current latest generation and derives
its own successor ([db snapshot component.rs:1074](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📸️snapshot/🦀️component.rs:1074)), then writes it before returning
([db snapshot component.rs:1111](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📸️snapshot/🦀️component.rs:1111)).

Permitted interleaving: P1y reads generation 4 and passes its check; another publisher writes 5;
P1y's helper rereads 5 and writes its stale full baseline as 6; P1y then sees `6 != 5` and returns
`StaleGeneration` without deleting 6. The post-write comparison detects the race only after stale
state is durable. This is not generation-qualified publication; publication needs an expected-
generation CAS/fence.

### A3 — A Panic After Lease Acquire Strands The Lease, Admission, Registry And Exact Owners

`retained_compaction_execute` acquires the compaction lease at [db compact component.rs:1339](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:1339) and releases it only on ordinary return from the inner run
([db compact component.rs:1343](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:1343)).

The driver catches a panic outside that future, stores the now-unwound future in `quarantined`, and
does not construct a terminal execution/cleanup authority
([db compact component.rs:1547](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:1547)). The public future reports an error while marking itself completed
([db compact component.rs:1807](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:1807)); its `Drop` returns early and does not schedule retirement
([db compact component.rs:1822](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:1822)).

Thus any storage/index/snapshot panic after acquire bypasses the sole release expression, leaves
`core.quarantined` nonempty, and prevents terminal admission/registry release. This directly
violates the census requirement that panic still release an acquired lease with exact owners.

### A4 — “Observed Backing” Is Per-Object, Not A Ledger; Callees Still Allocate Hidden Dynamic Owners

`database_compaction_observe_backing` merely compares one passed `(items, bytes)` with an operation
maximum and retains no cumulative accounting ([db compact component.rs:835](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:835)). Snapshot collection holds both `latest_descriptor` and a second descriptor for the same generation concurrently
([db compact component.rs:1239](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:1239)); each can fit `DATABASE_COMPACTION_OPERATION_ITEMS` while their combined capacities exceed that credit. No debit/return occurs for either owner.

P1y's live `publish_retained` callee creates `Vec::with_capacity(new_pages.len())` and fills it
before writing ([db snapshot component.rs:1092](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📸️snapshot/🦀️component.rs:1092)). This dynamic hash owner is neither pre-admitted nor incrementally retired by P1y. The verifier regex-scans only the local retained region, so it cannot see that live callee allocation.

Early cancellation/error in the descriptor-page loop returns through `?` while `descriptor` and
`latest_descriptor` remain ordinary `Vec`-backed values
([db compact component.rs:1250](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:1250)). They are explicitly retired only on selected normal paths; this failure path therefore uses ordinary destructor cleanup instead of one-backing-per-opportunity close.

## Verifier False-Green

The P1y predicate requires local tokens: one opportunity helper, local descriptor capacity text,
pre/post publication strings, and `Lane::Io` submission. It does not trace the cancellation token
passed to `IndexHandle::operation_control`, its 65,536 fuel/30-second full-compaction callee, atomic
expected-generation publication, panic-to-terminal lease release, cumulative ledger debit/return,
or dynamic allocations reachable below `publish_retained`.

I applied two representative harmful source mutations in memory without changing the worktree:
replacing the index fuel with `usize::MAX`, and retaining the isolated inner cancellation control.
Both bound to live source; neither has a named P1y hostile mutation. The in-memory check also
confirmed no live `handle.cancel()` and that `publish_retained` takes no expected generation.

## Checks Performed

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p1y` | PASS, false-green against A1–A4 |
| `bun ./📜️script.ts verify interactivity p1x` | PASS, preserved static gate only |
| `bun ./📜️script.ts verify interactivity p1w` | PASS, preserved static gate only |
| `bun ./📜️script.ts verify interactivity p1q-b1-b6` | PASS, preserved static gate only; read P1q audit remains RED |
| Scoped `rustfmt --edition 2021 --check --config skip_children=true` on compact + engine | PASS |
| Scoped `git diff --check` on compact + engine + root verifier | PASS |
| Caller/wait/eager-helper `rg` census | selected route cut over; only sync-hello non-test wait |

## Required Closure

Do not accept P1y. Thread actual P1y cancellation/deadline authority through index and snapshot
operations, split or cursor-mount whole index compaction, replace snapshot check-then-publish with
an expected-generation CAS/fence that cannot write stale state, and add a panic-to-terminal path
that releases the lease before exact owner retirement. Replace per-object checks with a cumulative,
debit/returning ledger covering both descriptor owners and downstream publication allocations; every
error/cancel path must mount descriptor/page close rather than ordinary destruction. Extend hostile
laws and the root verifier with each exact interleaving and callee trace.
