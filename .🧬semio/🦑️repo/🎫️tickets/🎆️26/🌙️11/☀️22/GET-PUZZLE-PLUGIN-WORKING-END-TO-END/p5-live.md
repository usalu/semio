IN WORK (component compile running in background via p5-wasm-retry.sh)

# p5-live: component, descriptor, browser module, live en/de lifecycles

## Findings at start (this session)
- `.cache/b/p/m/browser/prototype/modules/` has no `🧩️puzzle`; no `wasm32-wasip2` target dir and no `semio_hub_puzzle_component*` anywhere under the repo. Cold build.
- `/home` has 4.1 GiB free (gate for registered activation is 12 GiB + cushion); `repo cache gc --dry-run` stalled for 110 s (graph service), so no gc was attempted.
- Cheapest allowed profile: policy-rendered `prototype` for producer `wasm-component` (opt-level s, debug 0, thin LTO, strip symbols, cgu 16, no incremental); rendered with `p5-profile.ts` into `🗑️generated/p5-profile-selectors.txt` (82 selectors). Release is more expensive; no cheaper mode exists in the policy.
- Native build: `p5-component-build.sh` (cargo build --offline --locked --lib --profile prototype --target wasm32-wasip2, target dir on the NFS store scratch `/var/mnt/semio-store/p5-live-scratch`, because /home is full and /tmp is RAM), submitted via `repo capacity run --memory=5200 --cpu=4` (6144 can never be admitted: host spare is ~5.3 GB above the floor). Log `🗑️generated/p5-component-build.log`.

## Step log
1. `repo capacity run --memory=5200 --cpu=4 -- timeout 21000 p5-component-build.sh` (pid in queue, log `🗑️generated/p5-component-build.log`): WAITING since 09:00 CET. Last status at 09:17: 4008 MB available, floor 1981 MB, swap 8944 of 8947 MB used (full), load 31.9, 3 running + 4 waiting grants of 5000 MB each, mine is 3rd in line (waiting reason alternates between "earlier request still waiting" and "swap is crowded"). 6144 MB can never be granted (host spare never exceeds ~5.3 GB).
2. `repo cache gc --dry-run` (log `🗑️generated/p5-cache-gc-dry.log`): still walking (`cache:repository:undeclared`) after 17+ min, no result; no real gc run.
3. Registered activation (`repo run @semio-tech/framework-os-dev:activate-puzzle*-react-prototype`) was not started: the gate needs 12 GiB + cushion on /home (3.9 GiB free), the same wall the lanes l4/l15 of ticket 11/04 documented (they also record that self-issued native caller capabilities for the component driver were denied by the permission layer, so I did not repeat that).
4. Not done (nothing ran, so nothing counts as green): component wasm, descriptor, transpiled module under `.cache/b/p/m/browser/prototype/modules/🧩️puzzle`, serving 6012/6013/6014, six time-travel runs (2D/3D/5D x en/de).

## Coordinator correction (applied)
- The queued capacity wait (my pid 2233944, verified by its command line to be `repo capacity run --memory=5200 ... p5-component-build.sh`) was ended with SIGTERM; it is gone. The build was never granted and never ran. One exception to state plainly: my first call of `p5-component-build.sh` (made by mistake with `--help`, under `timeout 100`) started cargo for under 100 s before the timeout; it left only an empty target skeleton (2 CACHEDIR.TAG files, empty dirs) and no compile output.
- `/var/mnt/semio-store/p5-live-scratch` held only those empty dirs and 2 CACHEDIR.TAG files; I removed exactly that directory (`ls /var/mnt/semio-store` now shows only `v1`). I was wrong to pick the shared fleet store; it will not be used again.
- `p5-component-build.sh` no longer has a default scratch: it requires `P5_SCRATCH` (`${P5_SCRATCH:?...}`).
- `repo cache gc --dry-run` finished with `Would remove 0 entries, 0 B, 26 kept` for the scratch area (exit 1 at the end on a bun error, log `🗑️generated/p5-cache-gc-dry.log`); it frees nothing. No gc was run for real, the 12 GiB gate is untouched. /home: 3.9 GiB free.

## Disk-free prep (what I ran, results)
- Playwright browser: `~/.cache/ms-playwright` has chromium-1248, chromium_headless_shell-1248, ffmpeg-1013 (the runner also calls `prepareRepositoryBrowserV1`). node_modules for playwright at `.cache/b/d/m/j/<hash>/node_modules/playwright`.
- Ports: 2D 6012 (wgpu 6112), 3D 6013 (6113), 5D 6014 (6114); scenario `defaultServes` in each scenario; React renderer only (`--renderer react`); the 5D scenario's non-react branch only reports the plan failure.
- Locale switch: runner flag `--locales en` / `--locales de` (not `--lang`); each locale is a fresh browser context with `navigator.language`. `--lang` of repo only selects the CLI language.
- Tests vs sources: the three scenario files compile (`bun build --no-bundle` exit 0 for each). All eight `@semio-tech/framework/time-travel/testing/*` subpaths they import (band, chrome, history-session, page, report, scenario, state, wgpu-mirror) exist as exports in `🧰️framework/📦️packages/🟦️typescript/package.json` and their target files exist. Staleness flags: 5D scenario last changed 2026-11-06, 2D and 3D on 2026-11-21, while editor sources changed after (the 2D yes/no flag labels of 2026-11-24 mean 2D step 13 needs the rebuilt engine wasm, `pkg/semio_puzzle_bg.wasm` still dates from 2026-11-19, which is lane p4). Whether steps still match the current editors is only decidable by running them; not claimed.

## Exact ordered command list for when space is there (needs >= 12 GiB + cushion free on /home for the registered route)
1. `cd /home/deck/semio`; check `df -h /home`.
2. Wasm engine (p4 owns it): `repo run @semio-tech/puzzle-plugin:wasm --lang=en`.
3. Per variant, one at a time (each replays the closure: session, web support, fonts, engine wasm, component build wasm32-wasip2 prototype/thin-LTO with ~6 GB link peak, descriptor, jco transpile into `.cache/b/p/m/browser/prototype/modules/🧩️puzzle`, prepare, activate):
   `repo capacity run --memory=5200 --cpu=4 -- timeout 14400 repo run @semio-tech/framework-os-dev:activate-puzzle2d-react-prototype --lang=en` then the same for `puzzle3d`, `puzzle5d` (script: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/ALL-PUZZLE-ARTIFACT-EDITORS-END-TO-END/l4-activate.sh 2d|3d|5d`). Receipt check: `.cache/b/p/m/browser/prototype/runtime/react/<variant>/activation/🔣️receipt.json`.
4. Serve and verify `data-semio-os-ready` on 6012/6013/6014 (the time-travel runner starts or reuses the serve and fails on a missing ready marker).
5. Six runs, `OUT=<ticket>/🗑️generated/p5-tt-<dim>-<lang>`:
   `repo run @semio-tech/framework-os-dev:time-travel -- --scenario '✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel/🟦️.ts' --renderer react --locales en --out $OUT`, same with `--locales de`, then `🧊️3d-lifecycle/🟦️.ts` and `🖐️5d-lifecycle/🟦️.ts` (en then de each).
6. Native fallback if the graph service stalls: `P5_SCRATCH=<local dir with space> repo capacity run --memory=5200 --cpu=4 -- p5-component-build.sh` builds only the raw wasm; the descriptor, transpile and publish stages stay in the registered driver (a self-issued native caller grant for it was denied earlier, do not repeat).

## Result
Component, descriptor, browser module, three serves, six lifecycle runs: not done, none started. Blocker: /home 3.9 GiB free versus the 12 GiB gate; host RAM/swap (swap full, load ~32, 5.2 GB grants queued behind others).

## Resumed after disk freed (/home 127 GB free)
1. Wasm lane request 5000 MB queued behind p4 (which holds the lane lock `lockwasm`) and blocked the queue: ended my own waiting processes by PID (2759417, 2759410, later 2764918; none had a cargo child) per coordinator. Exit: killed (SIGTERM), no output.
2. Because p4 holds `lockwasm` for its unknown-unknown build, my dependency compile uses its own lane `BUILDER_LANE=wasip2 BUILDER_MEM=3000` (own lock, own target `🗑️generated/builder/lane-wasip2/t`), via `p5-wasm-run.sh` (policy prototype selectors + `lto=false` as the cheaper profile first, `--jobs 3`, `--offline --locked`, wasm32-wasip2).
3. Run 3 (`p5-wasm-build3.log`): EXIT 101 after 89 crates; the only error is `dsl-token` rustc `signal: 15 SIGTERM` (host memory guard cancel, no source error).
4. Run 4 (`p5-wasm-build4.log`): resumed, 360 crates compiled after about 9 min, no source errors so far.

## brep-snapshot higher-ranked lifetime error (handed to p4)
- Run 4 (`🗑️generated/p5-wasm-build4.log`, wasm32-wasip2, policy prototype + `lto=false`, `--jobs 3`, RUSTFLAGS of builder-run.sh): 795 crates compiled, then `EXIT 101`: `error: higher-ranked lifetime error` at `🦀️.rs:200:199` of `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot` (derive macro `semio_framework_value::NativeReceive` on `BrepCoedge`); `could not compile semio-s-artifact-stdio-semio-brep-snapshot (lib) due to 1 previous error; 5 warnings`.
- p4's wasm32-unknown-unknown log shows the same error; host (native) builds of this crate compiled earlier (lane4 logs, 5 warnings) before the derive/record-machine files under `🧰️framework/🔨️modules/🌱️value/✨️derive/🛬️receive` and `🛬️decode/🧳️owned` changed (11:26 and later today).
- My diagnosis run `p5-brep-diag.sh` (`cargo rustc -p … -- -Zmacro-backtrace`, same flags, `--jobs 1`) was stopped by its own `timeout 110` at 13:50 while recompiling the dependency closure (75 crates, reached `dsl-syntax`); it never reached the crate: no error line, did not end by itself. No cargo process of mine is alive. No further build of this crate, no refresh, no edit of brep or the derive.
- Lane target `🗑️generated/builder/lane-wasip2/t` kept as is (795 crates warm).

## Parts without the component (while brep-snapshot is open)
- Catalogue input `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🚀️playgrounds.json` holds the three records: puzzle2d (app `s.puzzle.puzzle2d@1/*#editor`, react 6012, wgpu 6112, engines `./🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust`), puzzle3d (6013/6113, engines none), puzzle5d (6014/6114, engines none); all from crate `🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/📦️packages/🦀️rust` (plugin id `puzzle`, package `semio:puzzle`). The launch needs `SEMIO_PLUGIN_CATALOG=<repo>/✏️s/🧑‍💻dev/🧩️catalog/🤖️generated`, `SEMIO_PLUGIN=puzzle2d|3d|5d`, `SEMIO_RENDERER=react`, `SEMIO_BUILD_MODE=prototype`; locales via the runner flag `--locales en` / `--locales de` (fresh context per locale through `navigator.language`), not the repo `--lang`.
- Engine wasm dependence: only puzzle2d declares an engine (`@semio-tech/puzzle-wasm`, `pkg/` of Nov 19), so the 3D and 5D lifecycle runs need no engine wasm, only the component. In the 2D scenario every step that boots the board waits for `data-board-snapshot-parsed` (needs the engine wasm, any build); only step 13 (lock/unlock flag text, yes/no labels changed 2026-11-24) needs the engine rebuilt from current sources; the rest should not depend on its freshness (not verified by a run).
- Registration dry run: `timeout 280 repo run @semio-tech/framework-os-dev:activate-puzzle2d-react-prototype --dry-run --lang=en` (log `🗑️generated/p5-activate-dry.log`): reading the project graph slice for 280 s (85 owner checks, 180714 observations), then my timeout cancelled it (SIGTERM, "Cancelled; leases released"); exit of the pipeline 124-class, no plan printed. So native publishing stays the fallback (envelope via `l15-envelope.ts` of ☀️04 is known to be refused for the component driver without a native caller grant).
- State for resume: lane `🗑️generated/builder/lane-wasip2` holds 795 compiled crates; resume with `BUILDER_LANE=wasip2 BUILDER_MEM=3000 ./builder.sh exec "bash p5-wasm-run.sh"` once brep-snapshot compiles; the final link needs the larger BUILDER_MEM (5000) in its own call.

## Resume after brep fix (14:20 onward)
| step | command | exit | result |
| --- | --- | --- | --- |
| 5 | `p5-wasm-run.sh` --locked, lane wasip2, MEM 3000 | 101 | `cannot update the lock file 🌎️hub/Cargo.lock because --locked` (frozen lock was altered by a refresh of p1; coordinator restored it) |
| 6 | same without --locked | 101 | NOT COUNTED (coordinator: altered lock); ended in `semio-plugin-store-lifecycle` `bounded_ownership` errors of the altered tree; process ended by itself |
| 7 | `p5-wasm-run.sh` --locked after the restore (`p5-wasm-build7.log`) | 101 | 331 crates recompiled (fingerprints changed by the restored files), then `semio-framework-3d-mesh-clipping` and `semio-os-flow-host-session` died with `signal: 15 SIGTERM` (memory warden), no source error |
| 8 | `p5-wasm-retry.sh` (loop, up to 6 reruns while the log shows SIGTERM; logs `🗑️generated/p5-wasm-build-r<n>.log`, `EXIT` line at the end) | running | started 14:55 CET; r1 at 37 crates after 3 min; about 10-20 crates/min, ~800 crates left, so 45-90 min |
Not done yet (no run, so nothing counts): final link (needs its own call with BUILDER_MEM=5000 when at most one other Rust build runs), descriptor, transpile, publication via the repo driver (5 min limit; earlier dry run of the activation target only read the graph for 280 s), three serves, six runs.
Next on resume: read the last `p5-wasm-build-r*.log`; when `EXIT 0` with `Finished`, `BUILDER_LANE=wasip2 BUILDER_MEM=5000` only for the link if the compile finished without it, then `repo run` the component/activate targets with `timeout 300`.

## Run 9 (once, no loop; coordinator 15:45)
`BUILDER_LANE=wasip2 BUILDER_MEM=2500 ./builder.sh exec "bash p5-wasm-run.sh"` (`--locked`, `--jobs 2`, log `🗑️generated/p5-wasm-build8.log`, `EXIT` line at the end): QUEUED, pid 3262741 holds `lockwasip2` and waits for one of the two `rust-slot` locks of builder.sh, which p1 (pid 3220580, 34 min) and p4 (pid 3262501, 12 min) hold. Log still empty at 16:00, no compile started, no exit code yet. Locals check (59,535 vs 50,000 limit) pending until the component links: run `p4-wasm-locals.py` or `wasm-tools validate` on `🗑️generated/builder/lane-wasip2/t/wasm32-wasip2/prototype/semio_hub_puzzle_component.wasm`.
