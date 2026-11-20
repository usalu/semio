# Source admission speed (2026-11-18)

Bench: `native-source-admission-bench.ts <root> <dir>...` runs the real `admitCargoSourceBoundariesV1` over the 144 workspace crates of `🧰️framework` (942 files, 21.8 MB, 1709 reads) with a stub control, `COPIES=n` repeats every crate as n units.

## Where the time goes (idle machine, no-op progress sink)
| port | 144 units | per unit |
|---|---|---|
| plain fs read | 3.8 s | 27 ms (closure tokenising/cfg selection = ~88 %) |
| physical no-follow read (lease check, ancestry lstat, double read, rechecks) | 5.2 s | 36 ms (+9 ms, ~1 ms per read) |
| progress publication (no-op sink) | negligible | 4 publishes/unit |
| `rustc --print cfg` | 40 ms per distinct arg set, cached by the port | |

So the dominant cost is closure parsing, repeated for every unit even when a package has several units (lib/test/build/feature/profile variants) with the same entry, cfg and environment (3791 units over ~805 packages in the logged pass). The loop itself also recomputed `used` bytes O(files) per read.
A 20 min pass is ~10x slower than this idle model (machine under 6-job memory pressure, real progress writer, 36 MB log); that residual could not be reproduced here.

## Fix
`admitCargoSourceBoundariesV1` keeps a per-call closure cache keyed by (entry, root, manifest, cfg, support, compilerEnvironment). A hit reuses inputs, probed paths and derived environment (no read, no parse, still publishes cfg/complete and feeds `memo.remember`), but only if the cached run fits this unit's own files/bytes/active-line limits; otherwise the unit is read fully so refusals stay exact. Only successful closures are cached. Running byte total replaces the per-read O(n) sums. Lease/control checks, end recheck and `noteScriptProcessActivityV1` are unchanged.

## After (`COPIES=4`, 576 units, same cfg per copy vs distinct cfg per unit)
| case | reads | time |
|---|---|---|
| distinct cfg (no cache hits, = before) | 6836 | 25.0 s |
| identical cfg (cache hits) | 1709 | 5.3 s (4.7x faster, 4x fewer reads) |

Gain scales with the duplicate-key rate; units with distinct cfg (test vs lib, features, host vs wasm) still parse separately.

## Tests
`🎟️admission/🧪️tests/🟦️.ts`: new test (shared closure read once, identical sourceInputs, tighter-boundary unit still refuses `source.boundary-exceeded`); progress oracle now gives each unit a distinct cfg. 5/6 pass under bun; the first test needs a process invocation envelope and fails identically without my change.
