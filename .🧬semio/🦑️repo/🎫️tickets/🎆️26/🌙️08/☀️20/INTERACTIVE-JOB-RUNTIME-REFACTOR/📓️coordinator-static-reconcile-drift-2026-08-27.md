# Static Reconcile Gate Drift Review

## Fresh Result

The coordinator ran `bun x nx run workspace:verify-interactivity --skip-nx-cache` with the default environment. Nx succeeded in dispatching the real task. App discovery still reports 32 descriptors, 101 app declarations, 101/101 launch-covered contexts, 4,760 action rows and 25 hostile/oracle checks. The overall gate then exits 1 with three stale live-reconcile source predicates. Full output is retained in `📓️coordinator-default-static-interactivity-2026-08-27.md`. The older green static checkpoint does not describe the current tree.

## Attribution and Actual Contract

The coordinator read the live reconcile source, reactor and strict document-surface fixture. The default now uses `SURFACE_RECONCILE_SURFACE_BYTES = 8 * 1_024 * 1_024`; aggregate bytes are that constant times four, 32 MiB. The page remains 32 KiB. The scanner still looks for an inline 2 MiB default and literal 8 MiB aggregate. The producer render error arm now cancels the exact grant and forwards its fault, instead of the earlier `Err(_) => grant.cancel()` spelling.

Read-only git inspection shows the capacity changes were committed in peer HEAD `a8d1caf41f`; the coordinator made no modifying git call. The peer ticket `2026/08/17/FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG` documents an actual 19-node document refusing at node ten under 2 MiB, followed by a fixture-pinned 8/32 MiB repair. Its later browser-proof report records a separate 128-node document capacity with an independent 32-action-binding per-node limit. Those reports claim 195 combined native UI/tracker passes after isolating the two capacities; the coordinator has read the reports but has not independently rerun that native cohort. No browser or global hard-timing proof is inferred.

## Assigned Repair

The renderer executor owns a narrow source-verifier correction that pins the actual fixed-capacity definitions and fixture, preserves reserve-before-produce, exact error cancellation/forwarding, one bounded reconcile opportunity per inner iteration and all hostile mutations. No-op source mutations must be rejected as invalid tests rather than silently counted. Neither peer implementation is to be reverted and no runtime step budget is to be widened. The full static target must run again before green is claimed.

## Adjacent Independent UI Check

After reading the peer's typed-operation continuation changes, the coordinator reran the full React suite: **506 passed in four files, exit 0, 6.86 s total and 6.74 s tests**. Full output is in `📓️coordinator-renderer-react-full-r6-2026-08-27.md`. This is a newer coherent source snapshot than the 484-test checkpoint, not a fresh Wasm or all-app runtime result.

