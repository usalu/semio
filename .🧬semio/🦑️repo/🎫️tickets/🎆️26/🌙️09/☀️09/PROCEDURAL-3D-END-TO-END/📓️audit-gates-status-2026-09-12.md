# Gates status audit — 2026-09-12 (read-only, live-run)

Every command below was actually **executed** during this audit (not read from prior reports) under
`NX_DAEMON=false`, `RUST_MIN_STACK=33554432`, no `CARGO_TARGET_DIR` override, foreground only. Raw logs:
`🗑️generated/gates-status/*.txt` (delete once this ticket closes, per the ticket-hygiene rule — this
report and its conclusions are the durable artifact).

Context read first: `📓️audit-build-restage-gates-2026-09-12.md` §4, `📓️status.md` (grep `Gate debt`,
`pre-existing`, `peer`), and every `📓️*-2026-09-12.md` lane report's "pre-existing" mentions (29 files
grepped).

## 1. Repo-wide gates

| gate | command run | result | attribution |
|---|---|---|---|
| Dependency Truth Gate | `bun ./📜️script.ts verify dependencies literal-external` | **RED**, exit 1. `target=0, current=193, oracle-conflicts=16, toolchain-owner-conflicts=2`. | Repo-wide, pre-existing. The 09-09 baseline was `current=193, oracle-conflicts=23`; the conflict *count* drifted (registry churn) but the gate's actual threshold, `current=193`, is byte-identical. No generation3d/procedural package appears among the currently-listed conflicts. |
| `verify interactivity apps --actions` | `bun ./📜️script.ts verify interactivity apps --actions` | **RED**, exit 1. `apps=8 … failures=782`. | Repo-wide, pre-existing. `grep -ci "generation3d\|procedural"` on the full output = **0**. Headed by `.vscode/launch.json: 2502 configurations exceed fixed capacity 512` and four missing `⚖️gate…` registrations. |
| `verify interactivity tool-jobs` | `bun ./📜️script.ts verify interactivity tool-jobs` | **RED**, exit 1. "Migrated declaration has 0 exact bounded reducer proofs" for 22 actions (`addGeneration`, `addWidget`, `canvasPointerDown`, …). | The file named is generation**2d**'s editor (`✏️s/…/🌀️generation2d/…/✏️editor/🦀️.rs`), not generation3d — a sibling artifact under the same `🌀️procedural` plugin, explicitly **out of this ticket's scope** (status.md: "generation2d's mirror set … out of this ticket's scope"). File is live-modified (`git status: M`, mtime 11:43, last commit `521b618cee` 10:48) — adjacent in-flight work, not this ticket's. |
| `verify taxonomy enforce` (CLI renamed: `inventory\|plan\|apply\|verify` → `report\|enforce`) | `bun ./📜️script.ts verify taxonomy enforce` | **TIMED OUT.** Ran 20+ minutes wall clock with zero bytes of output (this codepath buffers everything in memory and prints once at completion — no progress line exists for `report`/`enforce`, unlike `implementation`). Stopped it myself (`kill -TERM` on my own pid 35408, confirmed exited, EXIT_CODE=143) per the 20-minute cap. **No partial result is available** — nothing was printed before the kill. | Inconclusive — could not be measured within budget on this run. Last known landed state (09-10 11:47): procedural 650→22 violations, gen3d 328→4, gen2d 228→2, repo-wide 19464→2123, "abort cleared." |
| `verify rust-warnings native` + `verify rust-warnings --target wasm32-wasip2` (note: the brief's positional-arg form `verify rust-warnings wasm32-wasip2` is silently accepted but parses as **no** `--target`, i.e. native — both forms were run explicitly) | `bun ./📜️script.ts verify rust-warnings --target wasm32-wasip2` (and the native default) | **RED**, exit 1, on **both** targets, at the identical crate (`semio-framework-trace`), before either sweep reaches any generation3d/procedural crate. | `error: use of deprecated method 'fetch_update': renamed to 'try_update'` — already **on HEAD** (`git blame`: commit `de93f84300`, `git status` clean, this is the tip commit at session start). Repo-wide, blocks the whole target sweep. Not this ticket's file. |
| `bun nx run @semio-tech/plugin-registry:check` | as given | **RED**, exit 1 (5m35s). `duplicate playground port 6019 (assembly react and shooting react)` / `6119 (assembly wgpu / shooting wgpu)`. | Peer registration collision between the `assembly` and `shooting` playgrounds — already logged pre-existing in `📓️viewer-eval-chain-2026-09-12.md`. generation3d's own rows use 6018/6118 and are not implicated; `generate` itself succeeds. |
| `bun ./📜️script.ts verify publication-retirement-authority` | as given | **RED**, exit 1. Compile failure: `cargo test -p semio-framework-plugin --lib publication_retirement` → `error[E0624]: method 'drive_store_replacement_jobs' is private` ×3 (in `🧪️tests/🧩️composition/🦀️.rs`). | The shared `🧰️framework/…/🔌️plugin/🦀️.rs` is **currently mid-edit** (`git status: MM`, mtime 12:41, past its last commit `521b618cee` 10:48). `📓️extension-invoke-door-2026-09-12.md` §6.2 already logged 144 pre-existing failures in this exact crate from "a live peer refactor" (file mtime 04:59) — this is a continuation of that same in-flight churn, not this ticket's edit. |

## 2. Procedural-3d crate and engine gates

| target | command | result |
|---|---|---|
| `@semio-tech/procedural-plugin:test` (fundamental) | `bun nx run @semio-tech/procedural-plugin:test` | **RED**, exit 1 (10m48s). Nextest `fundamental` profile: **14 passed, 2 failed, 3 never-run** (cancelled after the 2nd failure) of 19. See §3 for the two failures — one is generation2d (adjacent), one is a real generation3d regression. |
| generation3d crate, `--lib` tests | `RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- --test-threads=1` | **Does not compile.** `error[E0255]: the name 'context' is defined multiple times` (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:362`) and `error[E0433]: cannot find 'tests' in 'super'` (`✏️editor/🧪️tests/🔬️tick-addressing/🦀️.rs:155`). Both files are **live-modified right now** (`git status: M` on both, mtime 12:03, past last commit `521b618cee` 10:48) — an in-flight, mid-rename edit inside this ticket's own generation3d editor test module (not landed). Cannot be measured against the known 366-380-passed baseline until that edit settles. |
| generation3d crate, `--test example-geometry` | `RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --test example-geometry` | **GREEN. 17 passed; 0 failed** (110 s). Matches the last landed baseline (17/17, all 8 DSL examples evaluate with the linked kernel). |
| React renderer engine, full corpus | `cd 🧰️framework/…/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react && SEMIO_TEST_LEVEL=long bun ./📜️script.ts test long` | **RED. 998 passed / 11 failed** (3 files failed / 27 passed of 30). One more red than the previously-recorded "10, all pre-existing" baseline — see §4. |

## 3. `procedural-plugin:test` — the two failures

| test | file | attribution |
|---|---|---|
| `surface_tests::generation2d_viewer_never_mutates` | panics inside `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81` ("ordered-map root must be explicitly retired before drop") | **generation2d, adjacent/out of ticket scope.** The panicking library file is old and stable (last touched 2026-09-08, before this ticket started, clean working tree) — the bug is in generation2d's usage of it, a sibling artifact this ticket explicitly excludes. |
| `close_ladder::generation3d_close_cost_is_independent_of_the_retained_session` | `✏️s/🔌️plugins/🌀️procedural/🧪️tests/🚪️close-ladder/🦀️.rs:270` (clean, committed at `521b618cee` 10:48 — **this ticket's own file**, the close-ladder-budget lane's new law) | **Real, in-scope regression — not attributable to a peer.** At landing time (`📓️close-ladder-budget-2026-09-12.md`) the fixture measured `costs=[cold:88, warm:152, long:265], ceiling=768, growth=8×` and passed (actual growth ≈3×). This run measured `costs=[cold:2, warm:39, long:84]` — all three **individually far cheaper** (consistent with the later host-reconcile-silence / wasm-hot-path-opt-level speedups), but `cold` collapsed disproportionately (88→2, a 44× drop vs. `long`'s 3.15× drop), pushing the **ratio** to 84/2 = 42×, over the 8× ceiling — while every absolute number is nowhere near the 768-turn ceiling. This is a test-calibration casualty of unrelated performance lanes landing after this law was written, not a functional close-cost blowup. Needs either a recalibrated `growth` ceiling or an absolute-only assertion. |

## 4. React engine: the 11 red tests, one by one

| test | file | attribution |
|---|---|---|
| `window fault discriminators > carries a distinct English and German label for every class` | `🧪️tests/🩺️window-fault/🟦️.ts` | **Peer churn, in flight.** The i18n bundle it scans via `?raw` import moved from `🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx` (now a 3-line re-export shim, 182 B) to `🖱️ui/🎯️targets/⚛️react/🟦️.tsx` (516 KB, the real bundle) — both paths `git status: A` (staged, uncommitted), a live taxonomy-driven file split. The `?raw` import still reads the now-empty old path, so the regex finds 0 `windowFault` blocks. Not this ticket's file. |
| 6 × `extension invocation completion ownership > classifies '…' / dispatches '…'` (`ok`, `empty-output`, `operator-fault`, `bad-request`, `dispatches 'evaluate'`, `dispatches 'manifest'`) | `🧪️tests/🔬️engine-contract/🟦️.ts:1285` | **This ticket's own, previously-undocumented regression.** `🏛️ShellHost/🟦️.tsx:1719` now calls `invoke(..., { originInstanceId, signal: cancellation.signal })` — the `signal` field was added by the **preview-eval-cancellation** lane (commit `521b618cee`, `📓️preview-eval-cancellation-2026-09-12.md`, 12:40). That lane's own vitest run was scoped to its own suite (18/18) and never re-ran the full `engine-contract` file, so it never caught that these six `toHaveBeenCalledExactlyOnceWith(...)` assertions (already updated by the earlier `extension-invoke-door` lane to expect `{ originInstanceId }`) don't yet expect `signal` too. **Needs a fix before close.** |
| `shell option locks … footer credits render the funding/partner logos, links, and locale text` | `🧪️tests/🔬️engine-contract/🟦️.ts` | `TypeError: cn is not a function` — a `♻️mit-bestand/🧺️demonstrator` footer component (demonstrator/puzzle3d playground family, not procedural). Out of ticket scope on its face. |
| `noteShellCommand > buildNoteShellCommandAction builds … targeting the given controller` | `🧪️tests/🔬️engine-contract/🟦️.ts` | Already-logged pre-existing ticket gate debt (`status.md`: "`buildNoteShellCommandAction` inverseArgs"). |
| `PluginRuntime > surface render ViewModel > binds two instances of one body to distinct surfaces…` | `🧱️elements/🔌️PluginRuntime/🟦️.tsx` (`git status: MM`, actively touched by several of this ticket's lanes) | Already-logged pre-existing ticket gate debt (the "6 🧩️package-integration worker-byte rows" family). |
| `PluginRuntime > documentPack/transaction wire adapter > readAppDocumentPack() … null when the reply carries no document frame` | `🧱️elements/🔌️PluginRuntime/🟦️.tsx` | Already-logged pre-existing ticket gate debt (`status.md`: "two 🔌️PluginRuntime in-source laws (`readAppDocumentPack` `ops`)"). |

**Net**: of 11 reds, **3 are already-logged ticket gate debt**, **6 are a genuine, previously-undocumented
regression from this ticket's own preview-eval-cancellation lane** (all one root cause: `signal` added to
`invoke()` without updating the engine-contract test doubles), and **2 are live peer/adjacent churn**
(i18n-bundle file split, unrelated demonstrator footer).

## 5. What must be green before close

1. **Fix the 6 engine-contract `signal`-arg assertions** in `🧪️tests/🔬️engine-contract/🟦️.ts:1285` to
   expect the `signal` field `🏛️ShellHost/🟦️.tsx:1719` now passes to `invoke(...)`. This is the single
   highest-value fix — it is this ticket's own regression and the largest chunk of the current react-engine
   red count.
2. **Land the in-flight generation3d test-module rename** (`✏️editor/🧪️tests/🔬️unit/🦀️.rs` +
   `✏️editor/🧪️tests/🔬️tick-addressing/🦀️.rs`, both currently mid-edit) so
   `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib
   -- --test-threads=1` compiles again, then re-measure its red count against the known baseline
   (366-380 passed / 5-6 failed, previously attributed to a peer's `🔌️plugin.rs`/`🧠️neural⚙️engine.rs`
   undo-snapshot bug — re-confirm that attribution still holds once it compiles).
3. **Recalibrate or replace** the `growth`-ratio assertion in `generation3d_close_cost_is_independent_of_
   the_retained_session` (`✏️s/🔌️plugins/🌀️procedural/🧪️tests/🚪️close-ladder/🦀️.rs:270`) — the current
   8× ceiling no longer fits post-optimization absolute numbers ([2, 39, 84] turns, all far under the
   768-turn ceiling that actually matters).
4. **Re-run `verify taxonomy enforce` with a longer budget** (it did not finish in 20 minutes this run;
   consider `--scope` to bound it to `✏️s/🔌️plugins/🌀️procedural` if that flag narrows the walk, or run it
   unattended with no timeout).
5. Re-run `SEMIO_TEST_LEVEL=long bun ./📜️script.ts test long` from the react engine target after (1) and
   confirm only the 3 already-logged, ticket-owned reds plus whatever peer churn is live at that time
   remain.
6. The remaining repo-wide gates (Dependency Truth, `verify interactivity apps`/`tool-jobs`,
   `verify rust-warnings`, `plugin-registry:check`, `verify publication-retirement-authority`) are red for
   reasons **entirely outside this ticket's files** (§1) — do not block this ticket's close on them. They
   need their own owners: `⏱️trace` deprecation fix, the `🔌️plugin.rs` `drive_store_replacement_jobs`
   visibility refactor (live peer edit), the `assembly`/`shooting` playground port collision, generation2d's
   scalar-config reducer-proof coverage, and `.vscode/launch.json`'s 512-row capacity ceiling.

Exact commands to re-run at close time:
```
bun ./📜️script.ts verify taxonomy enforce
bun nx run @semio-tech/procedural-plugin:test
RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- --test-threads=1
RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --test example-geometry
cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react && SEMIO_TEST_LEVEL=long bun ./📜️script.ts test long
```

## Files referenced (read-only; none edited)

- `📓️audit-build-restage-gates-2026-09-12.md`, `📓️status.md`, `📓️extension-invoke-door-2026-09-12.md`,
  `📓️close-ladder-budget-2026-09-12.md`, `📓️preview-eval-cancellation-2026-09-12.md`,
  `📓️viewer-eval-chain-2026-09-12.md` (read for context/attribution).
- `📜️script.ts` (read to resolve the `verify taxonomy`/`verify rust-warnings` CLI signature changes;
  not edited).
- Live-state files inspected via `git log`/`git status`/`ls -la` for attribution (none edited):
  `🧰️framework/🔨️modules/⏱️trace/🦀️.rs`, `🧰️framework/…/🔌️plugin/🦀️.rs`,
  `🧰️framework/…/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs`, `🧰️framework/…/📡️spr/🧵️channel/🦀️.rs`,
  `🧰️framework/🔨️modules/🕸️graph/🤖️generated/🗣️writer-languages/{🦀️.rs,🟦️.ts}`,
  `🧰️framework/…/🏛️ShellHost/🟦️.tsx`, `🧰️framework/…/🔌️PluginRuntime/🟦️.tsx`,
  `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/{📦️packages/🟦️typescript/🟦️.tsx,🟦️.tsx}`,
  `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs`,
  `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/…/✏️editor/🦀️.rs`,
  `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/✏️editor/🧪️tests/{🔬️unit,🔬️tick-addressing}/🦀️.rs`,
  `✏️s/🔌️plugins/🌀️procedural/🧪️tests/🚪️close-ladder/🦀️.rs`.

Raw command output: `🗑️generated/gates-status/01` through `10` (`.txt`), including the mislabeled-arg
native run (`04a-…`) and the corrected `--target` run (`04b-…`) — see the note in §1.
