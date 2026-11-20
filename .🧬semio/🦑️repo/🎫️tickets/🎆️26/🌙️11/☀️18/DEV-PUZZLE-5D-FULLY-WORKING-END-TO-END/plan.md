# Dev Puzzle 5D End to End — Plan

Scope: everything the dev puzzle 5D path touches.

| Lane | Owner | Report |
| --- | --- | --- |
| Dev story `✏️s/🧑‍💻dev/🧩️puzzle/📖️stories/🎭️5d-timeline` + `@semio-tech/puzzle-5d-ts` fixtures + `@semio-tech/puzzle-wasm` `puzzle5dParseDslJson` | Sonnet | `story-path.md` |
| `@semio-tech/puzzle-5d-rs` build/check/test/verify-* | Sonnet | `rust-crate.md` |
| `@semio-tech/puzzle-5d-react` test/verify-lifecycle, launch path, `@semio-tech/s-puzzle-composition-tests` | Sonnet | `react-target.md` |
| 5D project/target inventory and stale references | Haiku | `inventory.md` |

Done when every lane reports green with the command it ran, and stale references are fixed.

## Coordinator Notes

- The launch path is `repo run repo-storybook:dev-storybook-puzzle-5d`, port 6010. Its lane report goes to `storybook-launch.md`.
- The hub composition lane reports to `hub-composition.md`.
- I counted DSL rows to get the fixture counts. Concrete Forest has 1 part and 0 fasteners. Nakagin has 180 parts and 179 fasteners; the story agent's guess of 0 fasteners was wrong. Capsule Dream has 2880 parts and 2864 fasteners.
- `📚️example-counts/🔣️.json` is now the single source for these counts. The Rust `every_example_parses_into_its_fixture_counts` test reads it, and so do the Playwright `storybook-spatial` 5d cases, which loop over every example.
- At 15:02 every build run was queued behind one graph-plan inference rebuild. All 20 local runs belong to this session.
- At about 15:14 local, every `repo run` queued during the graph rebuild failed with resident-service `Error: closed` (reference `dee8ab03`; see `/run/user/1000/semio-repo/1000/host-2dee66942ec2.log`). This happened right after the sealed graph was published (971 owners). The failure looks transient, not a 5d defect, so I relaunched all runs: the wasm build, `puzzle-js:test`, 5d-react `test` and `verify-lifecycle`, with Rust and hub reruns delegated to their agents.
- `@semio-tech/puzzle-5d-react` is a compose library with no UI. The only launch path for dev puzzle 5d is Storybook (`dev-storybook-puzzle-5d`, scope `puzzle/5d`, glob `✏️s/🧑‍💻dev/🧩️puzzle/📖️stories/🎭️5d-*/🧪️.story.tsx`).
- The `Error: closed` failures have a root cause in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏛️service/🌐️host/🟨️.mjs`. A graph load runs for 30+ minutes. During that time the idle `sweep()` or the `openWorkspace` LRU evicts the workspace, so when the load completes, `ensureIndex` throws `closed` for every waiting client. The fix adds a `loads` counter per workspace and skips eviction while a load is in flight. A regression test and review are delegated; see `resident-closed-fix.md`.
- The 5d compile was blocked by framework API drift. Fixed so far:
  - `semio-s-plugin-puzzle-jobs` (`🎮️commands/📤️jobs/🦀️.rs` and its Cargo `testing` feature)
  - the 2d and 5d testkits: `maintenance_retirement_demands(body, FIXTURE_EFFECT_WORK)`
  - 5d `🧪️tests/🔬️unit` (pack_json codec) and the binary snapshot test import, both by an agent
  - the sqlite snapshot tests, by an agent, still in progress
- Hub: `semio-hub-puzzle` lives in the `🌎️hub/Cargo.toml` workspace, so run `repo cargo ... -- test -p semio-hub-puzzle --lib` from cwd `🌎️hub`. It compiled about 1320 crates, including the puzzle-5d crates, with no errors. The test binary link is slow under memory pressure. The result goes to `🗑️generated/hub-cargo.log`.
- Native storybook: the wasm module is built natively, but `pkg/semio_puzzle.js` is missing because the bindgen step failed. Storybook serves. Playwright must run under bun, not node, because of JSON imports.
- I stopped the stale first-wave `puzzle-5d-rs` repo-run shells to relieve memory pressure.
- Framework fixes from the hub surface failures:
  - `semio-plugin-decl-surface` codec-table `print_mirror` now uses the reference snapshot codec's `print_dsl` and `print_document_text_with`. A concurrent edit had required `ArtifactDsl`, and the earlier code called `.reference()` on a native codec, which always fails.
  - Reverted: changing `clock_begin` to `matches!(.., Ok(true))` broke the native-turn `birth-chain` tests. `clock_begin` only makes sure a decision exists; the verdict is checked in `grant_construction`. It is back to `.is_ok()`.
  - The real cause of the failing `puzzle{2d,3d,5d}_viewer_never_mutates` tests: the caller fixture `deadlineUs` was 1500 µs, but `OriginalSystemClock` now reads the real monotonic clock, so the cached Source deadline expired before construction ran. The fixture deadline is now 3600000000, in `plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json` and `🧾️expected.json`; see `hub-surface-fix.md`.
  - Verification is queued: the 5d lib tests, then the job native-turn and testing-caller tests, then the hub surface tests (`hubfix-1.log`).

## Halt State (2026-11-18, about 23:00, halted on request)

In flight; let these finish, then read the results:
- Full 5d lib tests: `🗑️generated/coord-5d-lib-test-3.log`, which ends with `EXIT`. It covers the `fixture_identity()` fix for the binary and text `command_envelope_round_trip` tests, the sqlite `interior_cancellation` test (its panic now names the phase), and `every_example_parses_into_its_fixture_counts`.
- Hub surface tests after the `deadlineUs` fixture fix: `🗑️generated/hubfix-3.log`, which ends with `done`. Notes are in `hub-surface-fix.md`.
- The native-turn `birth-chain` failures predate this ticket ("original return birth lacks its accepted proof" and "table never born"). An agent is on them; its report goes to `birth-chain-fix.md`.

Remaining after that:
- The official `repo run @semio-tech/puzzle-plugin:wasm`, to replace the hand-built `pkg/`.
- The `puzzle-5d-rs` repo targets.
- Delete `🗑️generated`.
- `ticket_close`.

## 2026-11-19 Progress

- The native-turn `birth-chain` tests are fixed: they had drifted from the current framework. See `birth-chain-fix.md`.
- `every_example_parses_into_its_fixture_counts` was moved off `include_str!` of the 3 MB capsule-dream DSL, which made repo cargo source-admission refuse `semio-s-artifact-puzzle-5d-v1` ("only 56364 bytes remain in its crate source boundary"). It now walks `examples()` and reads capsule-dream via `dsl_text()`, matching fixture rows by id.
- The official wasm `repo run` (`coord-wasm.log`) failed after a 72-minute graph-slice read with resident `Admitted canonical build owner differs` (ref 527c7895). Another agent created and edited `🔌️plugin/🪟️window/🎚️config/📐️owner-contract-and-mutation/📦️packages/🦀️rust/📋️project.json` mid-load. This is a build-system robustness gap: one live edit aborts the whole inference instead of re-admitting that owner. A retry is queued (`coord-wasm-2.log`).
- The 5d store close and the hub `viewer_never_mutates` both fail in store read-registry retirement and are delegated (`fix5d-tests.md`).
- Another agent's in-progress `semio-plugin-window-config-pack-source` split broke every build at about 02:50: the owner crate imported it without a dependency, and it was not a workspace member. I added the dependency to `📐️owner-contract-and-mutation/📦️packages/🦀️rust/Cargo.toml` and members in the root `Cargo.toml` and `🧰️framework/Cargo.toml`. The other agent added its `📋️project.json` at the same time.
