# Post-RED Independent P1y Source/Static Re-Audit

Date: 2026-08-24  
Auditor: Codex, independent read-only re-audit  
Verdict: **RED — P1y must not be accepted.**

## Scope And Method

Read completely: the repository and `os` `AGENTS.md` instructions; the P1y caller census and its 2026-08-24 RED-remediation clarification; the earlier independent RED report; Sol's updated P1y implementation report; live `db_compact`, `db_index`, `db_snapshot`, engine facade, DB CLI, and root P1y verifier source.

The selected production graph remains:

`db CLI cmd_compact` → `Database::compact_document` → `DatabaseCompactionFuture::try_submit` → retained `Lane::Io` driver.

The old `Compactor` and its eager helpers are test-only. The one non-test engine `db_actor::block_on` remains the separate sync-hello residual. No production source or root verifier was modified. No Cargo, Nx, build, Wasm, browser, or runtime Rust test was run.

## A1 And A2 Re-Audit

A1's former private child authority is closed on the production index path. Each live index kind creates its control through [retained control](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:1270), where the child constructor preserves that exact cancellation `Arc` ([index control](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔢️index/🦀️component.rs:869)). Fuel/deadline retry, cancellation normalization, handle drop, and index-document ledger return all occur before `stats?` ([compact loop](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:1268)).

A2's former check-then-write TOCTOU is also closed. The atomic claim is held before latest observation, the expected generation is refused before building/writing, and the exact body is returned on both success and refusal ([expected publication](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📸️snapshot/🦀️component.rs:1221)). The retained builder streams page hashes from fixed page slots and has no hidden hash `Vec` ([retained generation builder](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📸️snapshot/🦀️component.rs:769)).

## Blocking Finding: A3 Release Recovery Treats A Failed Retry As A Released Lease

`DatabaseCompactionLeaseRecovery::release_future` marks the lease released and permanently drops its fence regardless of whether the backend release succeeded:

1. It awaits `release(...)` into `result` at [compact component.rs:1516](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:1516).

2. It then unconditionally removes the only fence and writes `released = true` at [compact component.rs:1517](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:1517), before returning the possibly-`Err` result.

3. A release-poll panic is caught, quarantined, and retried ([compact component.rs:1774](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:1774)). When that retry resolves `Poll::Ready(Err(_))`, the same branch sets `callback_close` ([compact component.rs:1779](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:1779)).

4. Terminal cleanup accepts the false `released` witness, empties quarantine, and releases registry/admission ([compact component.rs:1863](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:1863)). Public polling then returns the panic fault once `panic_retired` is set ([compact component.rs:2095](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:2095)).

Faithful source-level reproduction: acquire a real lease, inject a panic in the first lease-release poll, and make the retained retry return a normal `Err` (for example, an injected release transport fault). The first panic reaches the existing catch/retry branch. The retry's `Err` follows steps 1–4: it loses the fence, records `released = true`, drains the quarantines and registry, and permits public fault completion despite there being no successful lease-release witness. A backend that leaves the lease held on that error is therefore stranded; recovery can no longer retry or expose the exact fence.

This violates P1y's A3 requirement that a panic during lease release cannot complete publicly or strand the lease, and that release retry/quarantine remain discoverable until exact close. The previous RED's ordinary post-acquire panic is repaired, but the release-panic-plus-failed-retry interleaving is not.

## A4 Re-Audit

The former independently checked descriptor/page debit is repaired for the examined ordinary, cancel, stale, and publication-fault paths: the ledger has checked cumulative add/sub ([compact component.rs:843](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:843)), descriptors retire before their debit returns ([compact component.rs:898](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:898)), and page owners are closed one step at a time before their retained page debit is returned ([compact component.rs:1372](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs:1372)). No additional A4 source/static counterexample was found. The A3 failure nevertheless prevents a GREEN verdict because it discards the lease recovery witness on a reachable fault route.

## Verifier Mutation Reproduction And Checks

The isolated P1y verifier was rerun; its live-source hostile mutation matrix executed and accepted none of its mutations. I also re-traced the repaired A1/A2 production callees above, rather than accepting the verifier's token counts. The A3 counterexample is not one of the matrix mutations: the verifier checks that a retry function is named/called, but not the semantic condition that only `Ok(())` may consume the fence and set `released`.

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p1y` | PASS — hostile source mutations clean; does not cover the A3 failed-release retry trace |
| `bun ./📜️script.ts verify interactivity p1x` | PASS — preserved |
| `bun ./📜️script.ts verify interactivity p1w` | PASS — preserved |
| `bun ./📜️script.ts verify interactivity p1q-b1-b6` | PASS — preserved |
| Scoped `rustfmt --edition 2021 --check` on compact/index/snapshot/engine/CLI | PASS |
| Scoped `git diff --check` on P1y source, root verifier, and census scope | PASS |

## Required Closure

Keep the recovery fence and `released = false` after every release `Err`; retain a typed retry future (or a typed terminal, still-discoverable release-failure authority) rather than allowing public completion. Only a successful `Ok(())` release may consume the fence and enable quarantine, admission, and registry retirement. Add an actual hostile law for: execution/release panic → release retry returns `Err` → no public completion and no registry/admission release while the lease witness remains retryable.
