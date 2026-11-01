STATUS: IN PROGRESS, UNVERIFIED. Native `cargo test --no-run` of the 10 crates (root manifest, runner `C/1101-tools/testport/t.sh p2-N Cargo.toml $(cat C/1101-tools/testport2/pkgs.txt) --no-run`) is being driven to compile; no test has run yet.

## Found
- `p2-1`/`p2-A` (`C/🗑️generated/1101/testport-p2-1.txt`, `-p2-A.txt`): source admission refuses `semio-framework-plugin-runtime-ops` (feature-gated closure with `original-actor-context-testing` at the 262144 B cap; 12 copies of `built/built_with/retired` helpers). Native `--lib` check of runtime-ops alone is green (`testport2-rops.rc`=0).
## Fixed
- runtime-ops: the helpers are defined once as `pub(crate)` in `🧰️ops/🦀️.rs` and imported by typed-operations intent/response and custody manifest root/metadata/window-view (no limit raised).
- p2-2: admission passed, compile ran 60 min with zero errors, stopped by the 3576 s cargo:test timeout (queue/load); retried as p2-3.
