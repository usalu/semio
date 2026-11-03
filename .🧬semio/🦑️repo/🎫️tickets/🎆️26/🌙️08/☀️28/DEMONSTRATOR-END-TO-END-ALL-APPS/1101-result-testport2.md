STATUS: PAUSED, UNVERIFIED. Native test build of the 10 crates compiled with zero errors in two 60-min runs (testport-p2-2/p2-3, killed only by the 3600 s cargo:test timeout) but no test has run. Halted by coordinator until act16.status has an exit line (still none at 18:57).

## Found
- `p2-1`/`p2-A` (`C/🗑️generated/1101/testport-p2-1.txt`, `-p2-A.txt`): source admission refuses `semio-framework-plugin-runtime-ops` (feature-gated closure with `original-actor-context-testing` at the 262144 B cap; 12 copies of `built/built_with/retired` helpers). Native `--lib` check of runtime-ops alone is green (`testport2-rops.rc`=0).
## Fixed
- runtime-ops: the helpers are defined once as `pub(crate)` in `🧰️ops/🦀️.rs` and imported by typed-operations intent/response and custody manifest root/metadata/window-view (no limit raised).
- p2-2: admission passed, compile ran 60 min with zero errors, stopped by the 3576 s cargo:test timeout (queue/load); retried as p2-3.
- p2-2 and p2-3 (--no-run, all 10): zero compile errors in 60 min, both stopped by the 3600 s cargo:test timeout; switched to batches (C/1101-tools/testport2/batches.sh, logs testport-p2-b1..b4) that run the tests.
- Coordinator halt: stopped my batches (pids 92368/92418/92420) before any batch result; no cargo until act16.status has an exit line.
