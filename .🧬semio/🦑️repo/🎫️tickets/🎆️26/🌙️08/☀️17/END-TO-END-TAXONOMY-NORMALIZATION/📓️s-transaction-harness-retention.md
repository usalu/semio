# Transaction Harness Evidence Retention

## Safety Correction

Inspection before the next aggregate run found that both the Nx launcher and the transaction tests deleted their completed ticket-local fixture trees. The current instruction requires those temporary assets and evidence to be retained. The launcher now preserves its fixture trees, and the tests replace whole-root removal with exact ticket-parent and fixture-prefix validation. Shared template markers and their empty preparation directory are retained as well. Intentional mutations inside an isolated transaction fixture, such as replacing its test symlink or clearing its cancellation sentinel, remain part of the tested behavior.

The launcher now passes the already absolute compiled test path to Bun. It no longer derives a relative invocation path. No test title, selection filter, concurrency setting, timeout, golden fixture, or transaction operation was changed. In particular, the complete aggregate watchdog remains fourteen seconds.

## Test-First Evidence

A permanent language-neutral vector specifies the launcher, test owner, absolute-path expression, retained-root authority, and accepted/rejected parent cases. The ticket test uses the TypeScript parser to inspect actual invocation and teardown sites. Bun and TypeScript independently compile and execute the real retention function against all four path cases, preserving every payload.

Before implementation: zero passes, three failures, five assertions, 983 milliseconds. The failures identified the relative launch expression, thirty-six whole-root deletion calls, and the missing exact retention function.

After implementation: three passes, zero failures, thirty-one assertions, 2.53 seconds. This is focused harness verification, not a passing complete aggregate or a performance acceptance claim. Actual uncached Nx execution is recorded separately once run.

Retaining shared templates exposed a necessary identity correction: PID-only run names can recur after process reuse and accidentally admit an old fixture or overwrite an old registry. The launcher and standalone test default now use PID plus a fresh UUID; all shards receive the same per-run identity. PID and boundary registries carry that identity too. A new test first failed on the absent unique identity, then the expanded packet passed four tests with zero failures and thirty-five assertions in 440 milliseconds. The frozen transaction golden remains byte-identical: 403,163 bytes, SHA-256 `7b700d79e5474417f0c92ddce61f5ffdd24603af56241d0fbdc3cdd5ba560296`, 63 transaction ledgers, nine workspace ledgers, and 98 boundaries.

The actual uncached filtered target `bun nx run @semio-tech/repo-lib:test-transaction-v2 --skip-nx-cache --args='incomplete plans'` passed one test with six assertions in 1.50 seconds; its shard finished in 1.53 seconds. After process exit, a separate no-follow check found both the shared template and strict-plan fixture under run identity `51506-20039ab0-f516-4a2d-a073-d049fa743e74`, with their source payloads preserved (SHA-256 `5d8f65d2774e206bc9f7a7a4ad39ca2dc563b5c31e46ab57ef4874961237ce29`). The unique PID and boundary registries also remain. This verifies real post-run retention; the full 62-case aggregate and its timing gate remain separate.

## Scope

Changed production test infrastructure: the repo-library TypeScript `📜️script.ts` and `🧪️tests/🧪️transaction-v2/🟦️.test.ts`. New language-neutral vector: `🧫️fixtures/🧪️transaction-harness-retention/🔣️.json`. The test, execution fixtures, and this report are retained inside this ticket. No actual Compose tree or real Git state was modified.

## Fresh Bundle Allocation and Revised Retention

The latest AGENTS instructions replace blanket output retention: authored inputs and Markdown reports remain; disposable tool outputs are removed after the relevant work is done. Active recovery dependencies are not discarded prematurely.

The launcher previously still compiled into one fixed bundle directory, despite its UUID-bearing registries and templates. It now allocates a fresh ticket-owned bundle directory with `mkdtempSync` for every run. The existing language-neutral harness vector declares this authority. The permanent aggregate's existing golden/parity test uses TypeScript to locate the real allocation function; both Bun and TypeScript compile and execute it, proving four distinct directories under the exact ticket parent. Those four empty allocation-check directories are removed after checking them; no authored input is removed.

Before implementation, this test failed with **0 passes, 1 failure, 1 assertion in 182 milliseconds**. Afterward it passed **1 test, 818 assertions, 0 failures in 351 milliseconds**, including the existing golden checks. No transaction golden, case title, 62-case coverage, 98-boundary matrix, or fourteen-second watchdog changed.

A fresh exploratory uncached Nx aggregate used `🧪️transaction-v2-bundle-bViV3C` and run identity `99124-c2c0ee6c-1565-46fe-ba49-54357a8ed2fb`. Shards 1, 3, and 4 completed with **5/0/869 in 9.31 seconds**, **19/0/418 in 9.39 seconds**, and **19/0/388 in 12.53 seconds** respectively. Shard 2 was stopped by the unchanged watchdog at 14.01 seconds. The complete target exited unsuccessfully; it is not a passing aggregate or performance acceptance run.

Readback of its exact boundary registry found **96 unique boundaries out of 98**, missing only the killed and recovered `transaction-attempt-canonical-published` pair. The three-identical-identity uncached performance gate remains **0/3**. This run occurred while other scoped code work continued, so even a passing result would not establish identical-identity acceptance.

## Exact One-Case Rebalance

The independent [partition review](📓️s-transaction-shard-rebalance-proposal.md) identified a minimal transfer of `recovers parent-killed transaction-attempt-canonical-published` from shard 2 to shard 1. The router now applies that exact two-string filter change. The one wave, concurrency caps `5/6/6/6`, test titles and bodies, child deadlines, PID cleanup, and fourteen-second aggregate watchdog are unchanged.

The language-neutral harness vector now declares all four exact filters and their expected test/boundary counts. The existing golden/parity test uses TypeScript to inspect the actual router and canonical test, expands all fourteen static and forty-eight generated titles, proves each bare and describe-qualified title is selected exactly once, and maps all 98 boundary identities to the unchanged golden. The new assertion first failed on the original filter strings (**0 passed, 1 failed, 1 assertion, 397 milliseconds**). After the router change, it passed **1 test, 949 assertions, 0 failures in 362 milliseconds**. The four partitions contain **6/18/19/19 tests** and **6/29/34/29 boundaries** respectively.

This static/focused success is not a timing result. No new complete aggregate has yet run with this partition, and the strict three-run gate remains **0/3**. The production transaction golden remains unchanged.

An actual uncached Nx invocation selecting only the transferred case passed **1 test, 22 assertions, zero failures in 1.05 seconds**, with its shard exiting in 1.07 seconds. Its new bundle was `🧪️transaction-v2-bundle-hqdy6I`. This proves the complete killed/recovered case executes through the real launcher; it is not an aggregate timing result. The next full exploratory aggregate is running separately.

The full exploratory run used `🧪️transaction-v2-bundle-83uhUu`, identity `69450-77469c8f-82fd-47c6-a126-f155219c0ba7`. It failed the unchanged watchdog; all four shards were stopped between **14.03 and 14.08 seconds**. The transferred case itself completed and both of its boundaries were present. Exact registry readback contained **61 unique expected boundaries, no extras**, 2,465 bytes, SHA-256 `f3c5a2d4e1d894f7c8aa0ee2823496ae31d208634d3e2783aefedbaae8b3ace6`. This result does not demonstrate a scheduling speedup. Other scoped lanes were active; no causal attribution or passing performance claim is made. The strict gate remains **0/3** and needs fresh source-identity-pinned measurements after current integration work settles.

A subsequent read-only host sample reported **10 available logical processors**, load averages **41.05 / 45.27 / 38.52**, 32 GiB total memory, and 0.95 GiB free memory. This is a later host-load observation, not an isolated benchmark or proof of the timeout's cause. No unrelated process was interrupted, stopped, or reconfigured.
