# result-td-ucache (lane td-ucache, mirror prefix `tu`, wave 1027b, 10-27)

FOR coordinator: batch `C/td-ucache-tools/staged/scratch` READY (5 edited test files + 3 new files: helper, its JSON table, its law; all test-only, no library code; base = live 15:00; stage.py from `xplat5-tools/stage.template.py`, passes `xplat5-tools/tests/test_stage_mode.py`). NOTHING applied live, NO official run.
- apply `python3 -I C/td-ucache-tools/staged/scratch/stage.py apply` · verify `sh C/td-ucache-tools/staged/scratch/verify.sh /Users/ueli/Documents/semio` (~12 min; the jobserver pool file alone is ~175 s per mode) · revert `python3 -I C/td-ucache-tools/staged/scratch/stage.py revert`.
- rebuild from live if an anchor drifted: `python3 -I C/td-ucache-tools/build.py` (exit 2 names the anchor).
- FOR coordinator (proof): no product run needed (tests only). Proof = the difflaw/gate test runs that set `SEMIO_TEST_ARTIFACT_DIR` under `C/🗑️generated/...`: the 14 unit-cache failures below disappear from the failure set; `bun repo run caching:test-cargo-unit-cache-processes` stays green (its artifact dir is the sandbox outside the checkout).

## Classification (every test file of `🗃️unit-cache/**`, 23 files / 133 tests, mirror `tu1` = live 14:45)
| mode | result |
| --- | --- |
| bare `bun test ./file` (live and mirror) | ALL GREEN (main suite 26/0 on live, 94 s) |
| `SEMIO_TEST_ARTIFACT_DIR` inside the checkout (the gate's/difflaw mode, `C/🗑️generated/...`) | 14 FAIL in 4 files: main suite 8 (exactly the 8 of `cf-art2/leaseorder-suites.set` and `cargofix/ord-b-base.set`), `🎰️jobserver/🧪️tests/🚦️pool` 4, `🎰️jobserver/🧪️tests` 1, `🧵️lane/📤️retention/🧪️tests/🧾️provenance-equality` 1 |
| envelope | not needed: no file of the area asks for the parent Script invocation |

`🏊️pool` (pool-model 5, pool-claims 6, sessions 1), `🧵️lane/📤️retention` (6 files, 27 tests), `📖️admission` (key-stability 4, foreign 2), `🔏️key` (8 + parts 1), `💾️store` 23, `🧹️gc` (5 files, 33 tests), `🪟️windows` 5, `🌱️incremental` 1: green in BOTH modes.

ONE root cause, a stale test assumption (not a product bug): these files create their scratch Cargo workspaces below `SEMIO_TEST_ARTIFACT_DIR ?? tmpdir()` and claim "every Cargo child runs with the scratch workspace as its working directory, so no repository `.cargo/config.toml` can leak in". Cargo merges every `.cargo/config.toml` from the working directory up to `/`, and attaches a package to the nearest enclosing `Cargo.toml`; rustup reads the nearest `rust-toolchain.toml`. With the artifact dir inside the checkout, the scratch builds ran with the repository's `rustc = …ingress/📜️script.ts`, `rustc-wrapper = "repo"`, its target/build dirs (the lane `d` of the REAL unit cache when run on live), `-Z threads=0`, the nightly toolchain, and the repo workspace. Evidence per failure (`🗑️generated/tu/base*/art-*.txt`): "Original compiler ingress requires the repository launcher and authentic enforcing store" (exit 101), "current package believes it's in a workspace when it's not … workspace: /Users/ueli/Documents/semio/Cargo.toml", "failed to load manifest for workspace member …", and the counters that read 0 because the wrapper chain never ran ("sccache … Expected 4 Received 0", "focused crate … Expected 1 Received 0", "cleanup … Expected > 0"). The "three sessions … 180 s timeout" seen on live is the same leak: the repo ingress waits for admission.

## Fix (batch `scratch`)
- NEW `🗃️unit-cache/🧪️tests/🏖️scratch/🟦️.ts`: `cargoAncestorFileV1(dir)` names the first `.cargo/config.toml`, `.cargo/config`, `Cargo.toml`, `rust-toolchain.toml` or `rust-toolchain` at or above `dir` (Cargo's HOME configuration excluded: Cargo reads it wherever it runs); `cargoScratchBaseV1()` returns the first candidate (artifact dir, then `tmpdir()`) nothing reaches, else refuses naming every file. Table `🏖️scratch/🔣️.json` (language-agnostic: files, kinds, plants, expected evidence).
- The 5 Cargo-starting test files use `const base=cargoScratchBaseV1();` (main suite, jobserver, jobserver pool, provenance-equality, and cargo2's `🌱️incremental`, which hard-coded `tmpdir()` for the same reason); docstrings corrected; unused imports dropped. No assertion changed.
- NEW law `🏖️scratch/🧪️tests/🟦️.ts` (9 tests, 1.6 s): table rules + refusal + home exclusion, and the third-party oracle = Cargo and rustup themselves: for each of the 5 files planted above an artifact dir, `cargo build` below it fails with the planted evidence (leak wrapper / foreign workspace / leak toolchain) and below the chosen base succeeds. Negative control (helper blind): 8 of 9 RED.

## Verification (mirror `tu1` = live 14:45, batch applied; `🗑️generated/tu/verify1.txt`)
- `verify.sh` GREEN: stage check; scan.ts 7/7 parse clean; tsc --strict 0 errors in the staged files (proof it type-checks: a planted `const probe:number=cargoScratchBaseV1()` is reported; the one other error is pre-existing in `🏃️process/🧪️testing/🟨️bun/🚧️operands`); every Cargo-starting file bare AND with `SEMIO_TEST_ARTIFACT_DIR` inside the checkout:
  law 9/0 · main suite 26/0 (base inside: 18/8) · 🌱️incremental 1/0 · jobserver 4/0 (base inside: 3/1) · jobserver pool 4/0 (base inside: 0/4) · provenance-equality 1/0 (base inside: 0/1). Bare: unchanged, all green.
- Law RED on base (helper absent) and with a blind-helper negative control (8/9 fail, incl. all 5 Cargo/rustup oracle rows).
- Official envelope (`red-g-tools/mrun.sh tu1 env`): law 9/0, main suite 26/0.
- `cachingTestCoverageV1()` with the batch: new law routed by the `cargo-unit-cache-processes` tree, no unrouted/duplicated/stale helper (one pre-existing `missing` row outside the mirror closure).
- Longest new path 135 chars (163 absolute). Live still at base at 15:25 (`stage.py check` exit 0).

## Notes for other lanes
- Any test elsewhere that builds scratch Cargo workspaces below `SEMIO_TEST_ARTIFACT_DIR` has the same exposure when that variable points into the checkout. In `cf-art2` only unit-cache testcases carry the leak signatures ("believes it's in a workspace", "Original compiler ingress requires…"); the helper stays inside unit-cache (test-only, no framework API).
- Mirror `tu1` dropped at the end.
