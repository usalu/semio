# result-keys4-g (wave 1026g): read-oracle evidence for the withheld router rules

FOR gate-g (21:10): batch ready `C/keys4-g-tools/staged/precision-rules` (ONE rule, `framework-rs-tests` narrowed to the 8 fully evidenced targets; rebased onto the LIVE `🎯️precision/🔣️.json`, rule-id level apply/revert, no router test copy, no read-oracle sync). `python3 -I stage.py check|apply|revert|status`, laws `sh verify.sh` (graph-free, 21 pass / 0 fail, derived counts equal the evidence). Apply/revert round trip on a scratch root is byte-equal. `dashboard-cargo-tests` stays WITHHELD (no cargo-leg evidence yet, see below).

Tools `C/keys4-g-tools/` (`run-target.ts` legs under the read oracle, `declare-all.ts` declared sets with the LIVE engine, `depinfo.py` compile-time leg, `cargo-bins.sh`/`build-all.sh`, `crates-of-target.json`, `evidence/*.json` per target and leg incl. every learned path). Raw logs `C/🗑️generated/keys4-g/` (deletable).

## Method (what "evidence" means here)
- Declared set per target = files the LIVE precision engine derives for the staged rule (+ the explicit globs the target keeps), re-derived at 19:04 on the live tree (not keys2-g's 10:44 lists).
- Oracle = the staged oracle2-g module (overlay copy: Seatbelt kernel enforcement, profile split for big plans) so every plan (1.3k-3.1k files) ran KERNEL-ENFORCED, none fell back to the mirror (this also upgrades the dashboard windows bun leg that was mirror-only). Positive control: the oracle names the undeclared reads of 10 targets, so a clean verdict is not vacuous.
- Legs copied from the router classes: `bun test`, `tsc --noEmit`, python, and the cargo LIB test executables of every crate the target tests (all tests of the crate, a superset of the nextest filter), cwd = package dir, `CARGO_MANIFEST_DIR` set, run from the binaries cargo built (1989 s cold build, shared store, prototype).
- NEW compile-time leg (`depinfo.py`): rustc's own dep-info (`.d`) of every unit in the target's path-dependency closure (closure from the Cargo.toml files, independent of the engine) must be inside the declared set. This leg found a gap that no runtime leg can see.
- A target is evidenced only when EVERY leg is clean with 0 learned reads, the baseline is green and the compile leg has 0 undeclared files.

## Per-target evidence (framework-rs, 24 router-bound targets)
learned = undeclared reads the oracle had to add before the leg ran clean (0 = clean at round 0); compile = rustc dep-info files outside the declared set.
| target | runtime legs (learned) | compile | verdict |
|---|---|---|---|
| test-source-projection | bun 0 | n/a (no cargo) | EVIDENCED |
| test-directory-discovery | bun 0 | n/a | EVIDENCED |
| test-host-effect-invocation | bun 0 | n/a | EVIDENCED |
| test-artifact-kind-source | bun 0 | n/a | EVIDENCED |
| test-original-factory-admission-source | bun 0 | n/a | EVIDENCED |
| test-deflate-encoding | cargo deflate 0 | 0 (186 files read) | EVIDENCED |
| test-history-progress | cargo kernel-history 0 | 0 (217) | EVIDENCED |
| test-snapshot-sqlite-native | cargo io-sqlite-snapshot 0 | 0 (12; the oracle bin is one file, no mod/include) | EVIDENCED |
| test-staged-payload-funding | bun job 0, bun tool-run 3, cargo job 2, cargo tool-run 0 | 8 | WITHHELD |
| test-retained-pack-encoding | bun pack 8 (rust-toolchain.toml, 2 schema json, 5 `💻️os/…/🏪️store/🧵️operation-wire/*.rs`), cargo pack 0 | 0 | WITHHELD |
| test-window-utility-native | cargo semio-framework 1 (`🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts`) | 8 | WITHHELD |
| test-action-choices | cargo app-surface 0 | 8 | WITHHELD |
| test-tool-run-actions | bun 0, cargo 0 | 8 | WITHHELD |
| test-history-edit-actions | bun 0, cargo 0 | 8 | WITHHELD |
| test-mutation-inputs | bun 36 (manifest TS schema closure, `.cargo/config.toml`, `🧰️framework/Cargo.toml`), python: BASELINE RED (fixture path `🛂️manifest/🧫️fixtures/…` does not exist), cargo 0 | 8 | WITHHELD |
| test-snapshot-sqlite | bun unit 5 (plugin schema + fixture, assets README, 2 html), interop needs the oracle bin (not built), cargo 0 | 0 | WITHHELD |
| test-core-modules | hash/pixels/intrinsic-size 0, mesh-engine 9 fixtures + 1 glb (still red after 8 rounds; unsandboxed it is green 46/46) | 8 | WITHHELD |
| test-wire-retirement-source | BASELINE RED (`original fixture authority maximum_items` assertion fails unsandboxed too), 0 reads before it | n/a | WITHHELD (no evidence) |
| test-wire-retirement-native | cargo action-bus 0 | 8 | WITHHELD |
| test-package-descriptor-value-codec | cargo manifest-plugin 0 | 8 | WITHHELD |
| test-command-ingress-consumer | tsc 6, bun 1 | n/a | WITHHELD |
| test-snapshot-receiving-source | tsc 6, bun 10 | n/a | WITHHELD |
| test-argument-retirement-native | cargo mutation-inputs 0 | 8 | WITHHELD |
| test-original-factory-admission | bun 0, cargo semio-framework 1 (generated manifest mirror) | 8 | WITHHELD |
Dashboard (`dashboard-cargo-tests`): test-windows bun leg 0 learned, KERNEL-ENFORCED (3,141 declared files; the old evidence was mirror-only); cargo legs of test-windows (12 crates, features `ui-tui-terminal/native,ui-tui-unit-laws/native`) and test-unit (18 crates, all targets) NOT RUN: the 20:41 dashboard cargo build sat 14 min in the capacity queue and was cancelled by me at 20:56 on the coordinator's gate-priority rule. `rule withheld`.

## Findings that matter beyond this batch
1. COMPILE GAP (engine/rust-g): every target whose crate closure contains `semio-framework-replication` has 8 source files read by rustc that the derived set lacks: `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/{⚠️diagnostic,✂️span,🔎️key,🔺️diff,🔺️diff/🪢️sequence,🩹️patch,🩹️patch/🚚️apply}/🦀️.rs` + `🧳️retained/🦀️.rs`. Source: crate `semio-framework-replication-mutation-retained` (package dir `🧳️retained/📦️packages/🦀️rust`) whose lib `🦀️.rs` mounts `#[path = "../../🦀️.rs"] pub mod retained;` (spaces around `=`, doc comment above); the derived list contains the package dir but not the mounted owner tree. A stale hit for an edit of e.g. `🧳️retained/✂️span/🦀️.rs` on any of ~15 router targets. Hypothesis not verified: `rustSourceFiles` does not follow that attribute form (the `#[path="x"]` form inside the owner file is followed). Needs a fix in `🦀️native-manifests/🟨️.mjs` (graph, gate path) + a law against rustc dep-info before the 16 targets can be added.
2. The router's TypeScript-test closure misses runtime data: fixtures next to a module (`🧵️job/🏭️construction/🧪️tests/🧫️fixtures`), schema json, generated TS mirrors read by Rust tests, `rust-toolchain.toml`/`.cargo/config.toml`, a sibling product tree scanned by a pack test (`💻️os/…/🏪️store`). Each is a real stale-hit if the file changes; the lists are in `evidence/r1-*`/`c1-*.json` (`learned`).
3. Oracle decoding limitation (oracle2-g): the unified-log path decoder mangles names with a ZWJ or a 4-byte emoji (`🧑\040\^M💻dev`, `\U0001faaa️manifest`); the paths are readable but not usable as allow-list entries verbatim.
4. Baseline-red targets exist in the umbrella (`test-wire-retirement-source`, python leg of `test-mutation-inputs`, `test-snapshot-sqlite` interop needs a prior bin build): they never produce a cached success, so their key is moot until fixed.
5. INCIDENT 19:59 (mine, fixed): a first `bun test` leg without leading `./` was a FILTER; it scanned the checkout and held ~122k directory fds (ENFILE); coordinator killed it 20:08. `run-target.ts` now prefixes every test path with `./`; my orphaned oracle CLI (it kept starting rounds) was killed by pid. No other damage known.

## Next (needs a gate verdict first)
`sh keys4-g-tools/build-all.sh` (dashwin then dashunit builds; the fw build is done), then `bun keys4-g-tools/run-target.ts c1 dash:test-windows,dash:test-unit cargo --cargo-log <dashwin.jsonl> --cargo-log <dashunit.jsonl>` and `python3 -I keys4-g-tools/depinfo.py d3 dash:test-windows,dash:test-unit <logs>`; then stage `dashboard-cargo-tests` the same way (copy the rule to `staged/precision-rules/rules/`, refresh `expected-counts.json`).
