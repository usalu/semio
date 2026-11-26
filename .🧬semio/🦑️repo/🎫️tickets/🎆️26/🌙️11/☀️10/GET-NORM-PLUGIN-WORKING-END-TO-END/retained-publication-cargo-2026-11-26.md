# Retained Publication, Browser and Cargo Grants

The ticket's graph-free native launcher now books a retained reservation when it opens its own exact grant. Native publication uses the authored 2,048 MiB and one-core command budget. Browser serving uses the authored 6,144 MiB and four-core end-to-end budget. On the inherited path, size, cores and retention must all be present in the live same-ID ledger row. The launcher repeats the same check on every replacement after a source-custody restart. The guarded Play workflow applies this requirement on admitted entry and before each logged child. Its top-level entry still uses the standard exact capacity command.

Fleet-delegated Cargo opens a fixed 1,536 MiB local orchestration floor for the duration of its body. That specific branch now marks the reservation retained and verifies that an inherited grant provides the same durable floor before delegating work. The ordinary Cargo forecast branch still uses learned sizing and remains eligible for ordinary forecast behavior. Component-pass per-unit allowances continue to come from the live parent ledger and their own broker.

Before the wrapper change a live ordinary forecast of sufficient size launched the mock publication. Before the Cargo change an equally sized forecast admitted pinned orchestration, and its self-acquired row lacked the retention field. Both cases now refuse before their bodies, while an adequate retained reservation is accepted. SQLite independently checks the mock wrapper admission predicate. The native launcher and sweep suites passed 16 tests and 81 assertions. The Cargo scaling suite passed 16 tests and 127 assertions, including unchanged ordinary nested Cargo, adequate and inadequate inherited grants, and cancellation with the retained orchestration grant held until the body drains.

The separate [durable graph reservation integration](durable-graph-grant-integration-2026-11-26.md) proves that a retained ledger row prevents an unrelated process from borrowing its unspent commitment after the forecast ramp. All wrapper and Cargo proofs used short test bodies. No compiler, component publication, or real browser ran.

## Exact native replay route

A read-only source trace confirmed the normal replay command can be launched beneath the standard exact producer:

```sh
repo capacity run --memory=2663 --cpu=1 -- repo cargo --offload=never --store=shared --configuration=prototype --parallelism=1 -- test --offline --locked -p semio-s-artifact-norm-en1990-mutations --test replay_preparation_physical
```

The outer producer reserves one job and one CPU, with 2,663 MiB as both its requested memory and exact minimum. It verifies the live retained ledger row before spawning. The ordinary nested Cargo admission resolves that same live ID before the queue path and runs under the inherited plan without creating a second memory grant. The explicit single-worker arguments remain in force. Cargo takes one independent compiler CPU lease and creates a one-token jobserver. The compiler broker reads the actual parent reservation, limits units by the smaller of its physical memory share and parent allowance, and deducts non-compiler memory. The fleet path stays disabled. This trace verified the current code route. Deployment of compatible ledger readers and the live machine's ability to admit the exact reservation are separate preconditions. No replay was submitted.
