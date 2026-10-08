# Preparation Live Queue Progress

## Evidence

The original Puzzle native-c ended at its unchanged 1800000ms deadline (2026-10-07T13:24:12.802Z), before selected native assertions. All 16 recorded process identities and its exact selector were absent afterward.

The read-only preparation arrival queue at 13:29:04.678Z contained 18 tickets, each associated with a physically live canonical preparation process. This does not establish a stale ticket. The oldest 4693 disappeared before sampling; next 4838 was live and opened the exact preparation SQLite file. Its one-second sample showed native filesystem directory enumeration, but unsymbolized JIT frames do not identify a precise producer phase. No process, lease or ticket was changed.

## Observability Seam

Canonical prepareCargoWorkspaceInvocation synchronously retains child stdout until completion while inheriting stderr. Existing queued lease wait messages use stdout, so the managed console cannot expose those messages while that preparation child remains live. The proposed opt-in lease-wait diagnostic uses the existing strict diagnostic schema, existing SEMIO_CARGO_PREPARATION_TIMING switch, and inherited stderr. Default stdout protocol, resource queue/lease behavior, task dependencies, selection, recipes, operational budgets and caches remain unchanged.

## Schema-First Test

Extended the current diagnostic phase schema and neutral fixture first. The authored runtime test holds the real system SQLite lease, starts the existing PreparationScript against a test-owned workspace, checks an Ajv-admitted lease-wait stderr row while the protected child is still incomplete, then cancels the child and releases the parent lease. All temporary fixture owners live under the ticket generated folder. Nx managed source RED10366 is still waiting in the ordinary graph at the current observation; production has not been changed.


## First Harness Outcome

The first managed source capture10366 ended13:36:15.632Z before the test body: Nx exec without an explicit project selected the whole graph and refused a circular Value/derive dependency. This is a task-router harness failure, not a feature RED. Production remains unchanged. The corrected capture explicitly selects workspace and excludes task dependencies, matching existing pure source test controls; the queue-progress test itself and its protected-operation assertions remain unchanged.


## Executed Feature RED and Producer

Corrected Nx source11384 actually executed the protected SQLite lease scenario: 0/1, one Ajv expectation, 8.82s. Its sole failure was no live stderr lease-wait observation while the child remained blocked by the retained parent lease. Only after this actual feature RED, PreparationScript gained an opt-in onWait callback using the existing diagnostic logger and monotonic elapsed time from request admission. The shared diagnostic phase union gained lease-wait. The callback observes each existing waiter progress event; it changes no queue or lease algorithm. When the timing flag is absent, the original stdout wait behavior remains intact. The full current Cargo/TOML/fast-glob/Ajv suite is the successor rather than only the new law.


## Completed Current Contract Suite

Source14730 actuallyGREEN18/18,501 expectations,16.82s completed2026-10-07T13:50:12.096Z. New protected-operation wait law ran487.84ms and emitted visible DEBUG lease-wait while child remained incomplete; existing Cargo-tree/TOML/Ajv/membership/default-quiet laws passed unchanged. Subsequent actual native12973 console demonstrated inherited stderr wait observations followed by closure/publication; it still reached the original30-minute deadline before selected native assertions. Thus live progress is established without claiming throughput improvement or native semantics.
