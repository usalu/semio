# Release Job Failure Census

Read-only audit of the semio-tech play release driver, as of 2026-11-21 about 23:40 CET.

Sources:
- Per-generation job logs: `/Users/ueli/Documents/semio/.cache/b/r/m/components/runs/<gen>/l/`
- Driver logs: `generated/native/site-build-*.log`, filtered for `"activity"` and `[capacity]` lines.

Window: driver runs since about 2026-11-20 17:40. Exit 75 is treated as a restart request. "Failed" means a non-zero exit other than 75, or 75 after the restart budget (7 attempts) is spent.

## Headline

- Only the known `surface-wasm` NoData failure blocks the latest complete generations. The latest runs (wilswhi, w3VVobu, wuWv9ic, wWhYuWQ) pass every other job they run.
- The latest generations never ran the full job set. Their jobs are canvas-runtime-assets, describe-tool, flow-wasm, font-assets, native-transpile, stdio-catalog-definition and surface-wasm.
- Seven jobs (bundle, editor-wasm, fonts, graph-wasm, map-tiles, puzzle-wasm, support) were observed passing only in generations wgqfntd and wt1bH3S (2026-11-19). They have not been re-run since. surface-wasm also passed there, but only with a 1-test suite that predates the NoData test.
- Pages publication has never been reached. The furthest staging was 1/73 components, in archived driver logs.

## Job set

| Set | Jobs | Last observed |
|---|---|---|
| Current 7-job set (wilswhi family) | canvas-runtime-assets, describe-tool, flow-wasm, font-assets, native-transpile, stdio-catalog-definition (composition), surface-wasm | 2026-11-21 23:33 |
| Full 13-job set (wgqfntd, wt1bH3S) | bundle, canvas-runtime-assets, describe-tool, editor-wasm, flow-wasm, font-assets, fonts, graph-wasm, map-tiles, native-transpile, puzzle-wasm, support, surface-wasm | 2026-11-19 |

Pass history across all generations:

| Job | Generations observed completing |
|---|---|
| canvas-runtime-assets | 23 generations |
| describe-tool | 15 generations (also several restart-then-pass cases) |
| flow-wasm | wilswhi, w3VVobu, wuWv9ic, wWhYuWQ, wuONgiG, wgqfntd, wt1bH3S |
| font-assets | 23 generations |
| native-transpile | 23 generations |
| stdio-catalog-definition | wilswhi, w3VVobu, wuWv9ic, wWhYuWQ, wuONgiG, wJerO9p |
| surface-wasm | wgqfntd and wt1bH3S only, with a 1-test suite. The current 6-test suite has never passed. |
| bundle, editor-wasm, fonts, graph-wasm, map-tiles, puzzle-wasm, support | wgqfntd and wt1bH3S only (2026-11-19) |
| pages publication | never |

## Per-generation status (latest first)

| Generation | Started (mtime) | Jobs | Status |
|---|---|---|---|
| wJerO9p | 23:34 | canvas, describe, font, native, stdio complete; flow-wasm still running (721/721 admitted at 23:37); surface not started | in progress |
| wilswhi | 23:30 | all complete except surface-wasm | surface-wasm: NoData only (known) |
| w3VVobu | 23:27 | same as wilswhi | surface-wasm: NoData only (known) |
| wuWv9ic | 23:25 | same as wilswhi | surface-wasm: NoData only (known) |
| wWhYuWQ | 23:04 | same as wilswhi | surface-wasm: NoData only (known) |
| wuONgiG | 20:58 | describe attempt 1 restarted, attempt 2 complete; flow, font, native, canvas, stdio complete | surface-wasm: NoData only (known) |
| w73S37W | 14:43 | describe restart then complete; font restart then complete; flow-wasm restarted 3 times, attempt 4 never finished | surface-wasm stopped by driver after 360 min (see F5) |
| wrsWb7f | 12:14 | canvas, describe, font, native complete | flow-wasm exit 75 on all 7 attempts (see F1) |
| w4L9aOU | 00:05 (driver 001330) | describe restart then complete | flow-wasm exit 75 on all 7 attempts, real compile errors (see F1, F4) |
| wTxgqdA | 12:06 (driver 120840) | describe failed on attempt 6, font restart | see F3 |
| wSiw5a4 | 10:35 (driver 103716) | flow-wasm failed on attempt 3 | see F2 |
| w8YmuSf | 04:04 (driver 095028) | describe failed on attempt 3 | see F2 |

Generations wtqSwCX, wwPJjvx, wH7OhJg and wti6Rh8 hold only cargo-prototype logs, not job logs. wtqSwCX also shows `E0425 cannot find value DRAW_MOUNTED_OWNER_POLICY in crate semio_hub_draw`.

Seven window generations reached the describe-tool restart limit and exited 75 for a seventh time: wvuq3J7, wAzFtU9, wGfxJi8, wjV8JUD, wcosRhk, wpe8vr9, wnxMXND (driver runs 173151 to 213537, 2026-11-20 17:31 to 21:26).

## Distinct failures (excluding the known surface-wasm NoData failure)

**F1. Restart budget exhausted while sources are being edited (exit 75, attempt 7).** Blocks a full release.
- describe-tool: 7 generations (wvuq3J7, wAzFtU9, wGfxJi8, wjV8JUD, wcosRhk, wpe8vr9, wnxMXND).
- flow-wasm: 2 generations (w4L9aOU at 00:05, wrsWb7f at 14:23).
- Each attempt ends with `Another process kept editing <file> while this command read it (one of its inputs changed). Waiting 250 ms, then starting the command again`. The file is typically a `.rs` or `Cargo.toml` under `🧰️framework`.
- Not seen in the wilswhi family, which ran with no source churn.

**F2. Source admission budget breached (exit 1).** Blocks any run that hits it.
- describe-tool, w8YmuSf (attempt 3): `RepoError: breach process.source.boundary-exceeded {"crate":"semio-plugin-interpreter","files":19,"maxFiles":32,"bytes":262956,"maxBytes":262144}`. Also `refused process.source.read-bytes-exceeded` on `plugin/interpreter/.../invocation/host/🦀️.rs`.
- flow-wasm, wSiw5a4 (attempt 3): `breach process.source.boundary-exceeded {"crate":"semio-framework-dsl-record-model","files":26,"maxFiles":32,"bytes":265731,"maxBytes":262144}`. Also `refused process.source.read-bytes-exceeded` (observed 4887 bytes, maximum 1300 bytes).
- Not seen after 2026-11-21 10:35. Whether the cause was fixed is not verified.

**F3. Missing inherited Cargo dependency (exit 1).** Blocks describe-tool.
- describe-tool, wTxgqdA (attempt 6, 12:06): `error: Missing inherited Cargo dependency: .../engine/bundle/.../Cargo.toml semio-framework-pack-json`.
- describe-tool completes in later generations, so this is likely fixed. Not verified.

**F4. Real compile errors in source (transient or fixed).**
- w4L9aOU flow-wasm attempt 6: `E0425 cannot find type WorldTerrainStyle in this scope` (semio-os-infinite-world-scene).
- w4L9aOU flow-wasm attempt 7: `E0502 cannot borrow *scene_state as mutable because it is also borrowed as immutable` (semio-os-infinite-world-scene, lib). This is the first real error at exit 75 on attempt 7.
- wuONgiG describe-tool attempt 1: `E0599 no associated function MAGIC / HEADER_BYTES for OriginalJobSourceWireHeader` (semio-plugin-actor-input). This was caught during source churn, and attempt 2 completed.
- w73S37W describe-tool attempt 2: `E0599 no method return_scoped_alias for OrderedMap<V>` and `could not compile semio-framework-manifest-view-model` at line 4535. The job still carries a `complete` stage marker. Treat as unverified.
- flow-wasm in wilswhi (23:32) completes, so the infinite-world-scene errors are resolved in the latest source.

**F5. Driver wall-clock cap for surface-wasm.** Blocks a run where surface-wasm has no declared cost.
- Driver run 204743 (w73S37W, about 20:41): `surface-wasm job: native:driver stopped: the maximum duration of 360 min derived from its declared cost (0 unit(s)) is reached`.
- The surface-wasm job never wrote a log in that generation. The driver stalled in `claim-retirement-scan:128`.
- Probable cause: surface-wasm declares zero cost units, so the driver sets a 360 minute cap.

**F6. Pages publication and component staging never reached.** Blocks a full release.
- The furthest observed staging is 1/73 components, in archived driver logs `site-build-233713.log.gz` and `site-build-154625.log.gz`. Every plain driver log in the window shows 0/73.
- No job or log for pages publication exists in any run directory or driver log.

**F7. Vite play:site build failure (Nov 19).** Superseded, but not re-run in the window.
- Driver run 050048: `error: Vite build failed for play:site`, `OwnedCommandFailure: vite:play:site failed (1)`. The underlying cause is not extracted.
- The bundle job completed in generations wgqfntd and wt1bH3S on Nov 19.

**F8. Cargo configuration-publication lease refusals (Nov 18 and 19).** Superseded, not seen in the window.
- Driver run 115030 (2026-11-18): `LeaseOrderErrorV1: Refusing to wait: cargo:configuration-publication (exclusive) unavailable for 15 s`.
- Driver run 115545 (2026-11-19): `map-tiles: Finite telemetry sample ceiling exceeded`.

## Superseded (outside the window, later passing)

- native-transpile exit 1 (driver runs 115121 to 115604, Nov 18). Later generations complete native-transpile 23 times.
- describe-tool exit 75 and exit 1 across Nov 18 to Nov 20. Later generations complete describe-tool.
- flow-wasm "Original Script process terminated" (SIGTERM from owned execution). This is a cancellation, not a failure.

## Blockers for a full release (excluding surface-wasm NoData)

1. F1: describe-tool and flow-wasm restart budget exhausted under source churn. Recurred 9 times in the window, most recently 2026-11-21 14:23.
2. F2: per-crate source admission budget (maxFiles 32, maxBytes 262144, read-bytes 1300). Recurred in 2 runs on Nov 21 morning. Verify the budget values and the fix.
3. F5: surface-wasm declares 0 cost units, so the driver caps it at 360 min. Declare a real cost.
4. F6: pages publication and 73-component staging have never been reached.
5. Jobs needing a re-run on current source (passed only on 2026-11-19): bundle, editor-wasm, fonts, graph-wasm, map-tiles, puzzle-wasm, support.
6. F3 and F4: verify the Cargo dependency fix and the compile errors in a clean generation.
7. F7: rerun the vite play:site build to confirm it passes on current source.

## Jobs never observed passing

- pages publication (never run).
- Component staging to 73/73 (max 1/73).
- surface-wasm with the 6-test suite (5 pass, 1 fail in all 10 runs with 6 tests).
- editor-wasm, fonts, graph-wasm, map-tiles, puzzle-wasm, support, bundle are observed passing only on 2026-11-19 and not re-run since. They are not "never passing", but are unverified on current source.
