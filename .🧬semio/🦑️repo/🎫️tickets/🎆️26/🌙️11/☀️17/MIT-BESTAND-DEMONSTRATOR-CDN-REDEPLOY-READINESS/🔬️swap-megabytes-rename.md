# Owner-pool swap thresholds: basis points to megabytes per second

Renamed `swapActiveBasisPointsPerSecond` / `swapOutBasisPointsPerSecond` to `swapActiveMegabytesPerSecond` / `swapOutMegabytesPerSecond` (absolute MB/s, defaults 1 and 64, no `/10000` scaling) in the owner-pool schema, source, shared-budget test, fixtures and Python oracle. No aliases. A grep over `🧰️framework` and `✏️s` finds no other consumer of the old owner-pool names.

Fixture cases (16 GB host, 8 children): 33 MB/s gives 3 children, 64 and 200 MB/s give 0. TS and oracle agree on all 30 fixture tests.

## Open problem outside this change

The working tree of the host-capacity migration keeps flipping back to the old names. Observed more than once during this session:
- `🏗️build/🔣️policy.json` swap block went back to `activeBasisPointsPerSecond` / `outBasisPointsPerSecond` / `shortBasisPoints` after being set to megabytes.
- host-capacity `🧬️schema/🔣️.json`, `📏️decision/🟦️.ts` and the `📐️scaling`, `🔼️upgrade` oracles still carry the old names (HEAD has the new schema).

Result: `repo show projects` fails with `Closed host capacity Policy required: /swap/activeBasisPointsPerSecond is required`, so the repo tool could not run tests. Tests were run with the bun binary directly. `🧮️shared-budget` fails on `policy.hostCapacity.swap.activeMegabytesPerSecond` being undefined until policy.json carries the new names again.
