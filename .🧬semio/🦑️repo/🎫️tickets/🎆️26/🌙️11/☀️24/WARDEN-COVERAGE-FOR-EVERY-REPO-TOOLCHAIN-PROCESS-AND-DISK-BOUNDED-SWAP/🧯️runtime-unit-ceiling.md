# 🧯️ Runtime Unit Ceiling: the Warden Stops a Unit Before the OS Does (2026-11-25)

Incident: a `local-unit` `rustc --crate-name semio_s_artifact_sourcing_curation` grew from 7.4 GB to 8.8 GB while the warden suspended it under `warn` pressure and, "suspended for too long", resumed it again, five times. The admission law `Policy.unitCeiling` was only checked after the unit had finished. Nothing stopped it at runtime. Now it does.

Paths below are relative to `🧰️framework/🔨️modules/🏃️process/` (`W/` = `🏗️build/🌡️host-capacity/🛡️warden/`, `A/` = `📦️artifacts/🏗️native-build/🎟️admission/`).

## 1. The runtime ceiling

Schema first, one source of truth: `A/🧬️schema/🔣️.json` `Policy.unitCeiling`.

- Every carving entry now declares `budgetMegabytes`, the budget enforced while the unit runs (at least its measured peak, at most 10 percent above it; a law test pins both and that it stays below the 60 percent cap share of the 16 GB class). Rule used: measured peak plus 5 percent, rounded up to 100 MB, with the two values named in the request fixed by hand: hub final `semio_hub_sourcing_component` 5600 (measured 5519), `semio_hub_sourcing` 4500 (measured 4273); `semio_s_artifact_sourcing_curation` 3900 (measured 3653); cad final 8500, puzzle final 6500, and so on for all 32 entries.
- `measuredMegabytes` of six entries was raised to the larger peak this ticket's own table (`🪚️sourcing-unit-carving.md` section 2) measured later (`binary_io_registry` 2925, `actor_input` 2366, `presentation_member` 2318, `model_member` 2376, `graph_editor` 2116, `animation_member` 1845); an old lower number would have killed a legitimate rebuild.
- New pure module `A/🏔️ceiling/🟦️.ts` (the admission re-exports it explicitly; nothing outside it was copied): `compileUnitOfCommandV1` reads the crate (`--crate-name`), target (`--target`, `host` without) and crate types (`--crate-type`, comma lists, `=` forms) from a compiler command line; `runtimeCeilingOfCommandV1` gives `{unit: "<crate>|<target>", kind, megabytes, listed, measuredMegabytes?}`: the library budget 1536 MB, the final (`cdylib`) budget 4096 MB, or for a unit on the carving list `max(budgetMegabytes, budget of its kind)`. A command without a crate name (`rustc -vV`) has none. The functions `unitKeyPartsV1`, `unitBudgetKindV1`, `unitBudgetMegabytesV1` moved here from the admission module (the warden must not import the heavy admission module, which imports the warden lazily).
- Warden: kinds with `ceiling: "unit"` (`remote-unit`, `local-unit`) get a `Subject.ceiling` computed from their command line. `decideHostWardenV1` cancels a subject whose megabytes exceed its ceiling at **every level, first, before caps and shedding**, build-critical or not (reason `ceiling`, new `Action`/`Alarm`/`Cancel` reason). The cancel counts as freed memory, is not limited by `maximumCancelsPerTick`, and a suspended unit over its ceiling is cancelled once as a ceiling, not again as overdue. Observe mode reports once and signals nothing.
- Test binaries, browsers and the other kinds keep their kind cap (`capMegabytes`, non-critical, cancelled at any level, e.g. test-binary 8192 MB): no per-unit budget is measured for them, so no new number was invented. The 10240 MB cap of the unit kinds remains as the backstop for an unparseable command line.
- Typed alarm (en and de), for an unlisted unit: "unit semio_plain|wasm32-wasip2 peaked at 1705 MB, above its library ceiling of 1536 MB, and it is not on the carving list: carve it along the domain taxonomy so that it fits"; for a listed one: "... above its declared carving budget of 4500 MB (measured 4273 MB): carve it further, or measure it again and declare the new budget". German: "Einheit ... erreichte ... MB und liegt über ihrer Obergrenze Bibliothek von ... MB; sie steht nicht auf der Carving-Liste: entlang der Domänen-Taxonomie schneiden, damit sie passt".
- Reaction time: the footprint probe (`observeTreeMegabytesV1`, includes compressed and swapped pages) is refreshed every tick for a unit at or above `ceilingNearBasisPoints` (7500, new policy number) of its ceiling instead of every `capMilliseconds`, and the loop keeps the tight 500 ms tick while any unit is that near.
- The code digest of the warden now covers the admission ceiling module and schema, so a changed budget restarts every warden.

## 2. "Suspended for too long": decision

Old: an overdue build-critical subject was resumed below `danger`. That produced the incident cycle (suspend, 120 s, resume, grow, suspend, ...).

New, one rule for the ladder (`decideHostWardenV1`): the ladder only escalates. An overdue subject (suspended for `suspendMaximumSeconds`) below calm is **cancelled**, build-critical or not, unless the level is *quiet* (held by compressed memory alone, nothing to shed); it is **resumed** only when it cannot be cancelled, when the level is calm, or when the level is quiet. So it is never resumed while the host is still tight; this is a superset of "never resume the largest suspended unit below the guard line". Reason: resuming restores exactly the pressure that caused the suspension and the unit only grows into it; the owner learns it from the typed failure and re-runs when the host is calm. The one-line consequence for the dev: under sustained OS pressure a build-critical unit that the ladder parked for two minutes is killed instead of being parked forever.

Fixture cases (oracle-written): incident (7400 MB unit without a ceiling, 1500 MB available: `cancel.overdue`), OS pressure warn with 3000 MB available (`cancel.overdue`, not quiet), compressed memory only (quiet: `resume.overdue`), calm (`resume.overdue`), critical (`cancel.overdue`). The old fixture "slice worker suspended for too long is resumed" now expects the cancel and was renamed.

## 3. Surfacing to `repo cargo`

- Cancel record (`W/🧬️schema`, `Cancel`) gained `ancestors` (the pids above the cancelled process, nearest first, at most 64), `ceiling` and `capMegabytes`. The owner of a build finds the cancels that hit its own tree by its pid, whatever the command line says (the 512 character command cut loses `--out-dir`).
- `W/🔁️loop`: `readHostWardenCancelsBelowV1(ancestor, sinceMs)`, `HostWardenCancelledErrorV1` (`code: "host-warden-cancelled"`, message = English then German, one line per cancel with kind, pid, owner, megabytes and the typed reason, then "Nothing is retried"), `requireNoHostWardenCancelBelowV1`.
- `repo cargo` launcher (`🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts`): when the cargo execution fails, `consumeExecution` calls `requireNoHostWardenCancelBelowV1(process.pid, launchedMs)`, so the admitted build ends with the typed error and its clear message instead of a bare "process didn't exit successfully (signal: 9)". No retry exists on this path (`runCargoPassAttemptsV1` already refuses to retry units with exit >= 128).
- The unit-cache wrapper (`⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts`) writes the one-line bilingual `[unit-cache] [warden] cancelled local-unit pid N (X MB): <typed reason> / [Wächter] ...` to the build log when its compiler dies from SIGKILL and a record names the pid.
- Admission broker: cancel records with reason `ceiling` no longer raise the retry prediction or double the wait of the next admission of that crate (the unit is too big, not the host busy); a rerun is cancelled again at its ceiling with the same message, never held in a loop.
- Not done: cargo finishes the units already running before it exits (it cannot be told to stop faster from here without a Rust change); the message is raised when it does.

## 4. Tests (repo bun `NODE_ENV=test bun test ./<file>`, python3 -I)

| suite | result |
|---|---|
| `W/🧪️tests/🟦️.ts` (warden) | 43 pass, 0 fail (37 before, 6 new, several extended) |
| `W/🧪️tests/⏱️overhead/🟦️.ts` | 1 pass |
| `W/🧪️tests/🐍️.py --check` | stored expectations equal the oracle (decisions 106 = 94 + 12 new, classifications 26 = 23 + 3 new) |
| `A/🧪️tests/🧢️unit-ceiling/🟦️.ts` | 11 pass (new: runtime ceiling of every corpus command line equals the independent oracle, carving budget invariants) |
| `A/🧪️tests/🧯️available`, `🎫️broker`, `♻️replay`, `🔖️scheduler` | 4, 7, 5, 7 pass |
| `🗃️unit-cache/🧪️tests/🧯️warden-cancel/🟦️.ts` (new) | 2 pass |
| `fleet-status/🧪️tests/🟦️.ts`, `remote-execution/👷️worker` suites | 28, 17 + 3 + 4 pass |

New warden tests: ceiling cancel on a calm host with typed alarm (en/de, unit, peak, ceiling, carve), carving budget (hub final 5600, hub_sourcing 4500 with measured), observe mode and tight level, typed failure by ancestor pid (found, another build not hit, older cancel not hit, bilingual message), overdue unit never resumed while tight, a MOUNTED test with a real child process over an injected tiny ceiling (killed with its tree, record carries `ancestors` and the unit, the owner finds it). Independent oracle: `W/🧪️tests/🐍️.py` implements ceilings, the cancel rules and the command parser from the schema data only; the unit-ceiling oracle `A/🧪️tests/🧢️unit-ceiling/🐍️.py` implements the same parser separately and compares it with the TypeScript on 17 command lines. The third-party `jsonschema` check is skipped (module not installed), as before.

Failures that are not from this change (environment): `A/🧪️tests/🟦️.ts`, `🔓️tokens`, `🧵️frontend-threads` need a cargo/rustup toolchain build; `🔭️probe` "a live broker answers the probe" fails because the real host is under pressure now (the probe does not inject availability); `fleet-status/🧪️tests/🛡️guard` fails on a peer's `📥️readers` using `spawnSync`; heavy-questions "event loop lag" is timing. No cargo/rustc build was started; the launcher line is verified by a transpile and by the typed functions it calls, not by an end-to-end `repo cargo`.

## 5. Live proof (this machine)

Warden restarted itself onto the changed digest: state `code 69f05e8e493053cc` equals `hostWardenCodeDigestV1()`, pid 31460, mode enforce. `live-proof-ceiling.sh` (ticket folder) starts a process whose argv0 is the pinned `rustc` (no repo ancestry), names an unlisted library unit and holds 1700 MB:

```
cancelled after about 3 s
exit status 137
[warden] cancelled local-unit pid 31964 (... 1705 MB): unit warden_ceiling_proof|wasm32-wasip2 peaked at 1705 MB, above its library ceiling of 1536 MB, and it is not on the carving list: carve it along the domain taxonomy so that it fits; host 3942 MB available ...
[Wächter] abgebrochen local-unit pid 31964 (... 1705 MB): Einheit warden_ceiling_proof|wasm32-wasip2 erreichte 1705 MB und liegt über ihrer Obergrenze Bibliothek von 1536 MB; ...
HostWardenCancelledErrorV1 host-warden-cancelled   (the typed failure the build owner gets, en and de)
```

Warden overhead at the check: p50 105 ms, p95 218 ms, resident 77 MB (higher than before because the machine is busy with peers' builds).

## Caveats and decisions for the lead

- Enforced on every host, whatever its memory: the fleet law is "fits the 16 GB worker", so a unit that is only fine on a big machine would be a hidden failure. A unit above 1536 MB (library) or 4096 MB (final) that is not on the carving list is now cancelled the first time it runs; the curation unit of the incident (3900 MB budget) can no longer build until it is carved.
- The budgets are measured peaks plus 5 percent; peaks of the libraries vary with the granted threads (`binary_io_registry` 1650 to 2925). If a legitimate rebuild is cancelled, re-measure and raise `budgetMegabytes` in the one schema file (a warden restarts itself on it).
- Overdue build-critical subjects are now cancelled under non-quiet pressure after `suspendMaximumSeconds` (120 s).
- Nothing committed; no `AGENTS.md` touched.

## 6. Follow-up: kernel `warn` alone no longer sheds; escalation only on a measured signal (2026-11-25)

Evidence (coordinator, `warden.log`): a legitimate 75-minute admitted build lost `semio_hub_sourcing_component` (budget 5600, 1.5 to 3.0 GB) after about five suspend/resume cycles, every one triggered by `pressure warn` alone with 3.5 to 4.0 GB available (guard 1638), headroom 11.7 to 12.4 GB (guard 4096) and swap-out 0 MB/s; the last was a cancel "suspended for too long" at 2983 MB with 3921 MB available. **This section supersedes the overdue rule of section 2** (cancel below calm unless quiet).

Schema `Policy` (new numbers) and `Subject` (new optional `suspensions`, `resumedSeconds`, set by the loop, not by classification); law in `decideHostWardenV1` (`W/🟦️.ts`); oracle `W/🧪️tests/🐍️.py`.

1. **Measured signal.** The level is *quiet* when no measured signal is at or below its line: available memory at or above the guard line, headroom at or above its guard line, swap-out rate below `swapActiveMegabytesPerSecond`, and OS pressure not `critical`. `warn` (and compressed or swapped memory alone) is then only felt by the kernel: the level stays guard with trigger `pressure`, so new admissions are held, but nothing is suspended, nothing is cancelled and a suspended subject is resumed (one per tick, the dearest first). `critical` pressure still sheds.
2. **Overdue.** A subject suspended for `suspendMaximumSeconds` is cancelled only when the level is not calm and not quiet (a measured signal at or below guard), for every kind, build-critical or not. In a quiet or calm level it is resumed (cannot-cancel subjects always). So the 2983 MB final at 3921 MB available is resumed, never cancelled.
3. **Victim order** (`order`, used for suspend, danger cancel, keepRunning and resume): not build-critical first, then the least work lost per megabyte freed (elapsed wall seconds divided by megabytes, exact by cross multiplication; CPU time is not in the process table, wall time is the proxy), then the cost rank, then the pid. A young unit or pool child goes before a long running final; the dearest (last) is the one `keepRunning` protects and the one resumed first.
4. **Damping.** The loop counts suspends per subject id: a subject suspended `flapMaximumSuspensions` (2) times within `flapWindowSeconds` (1800), or resumed less than `resumeHoldSeconds` (120) ago, is not suspended again, unless the level is severe danger (then cancel and suspend as before). Forgotten when the subject ends.

Tests: oracle fixtures 117 decisions (11 new: the coordinator incident holds and resumes, warn plus swap-out above its line sheds the young unit first and keeps the long final, warn plus available below guard, warn plus headroom below guard, overdue under a measured signal is cancelled, victim order, flapping twice and once, hysteresis 30 s versus 200 s, severe danger ignores damping; 12 reworded or re-expected for the new order and the warn rule); new TS tests for the loop damping (a victim is suspended exactly twice over 7 ticks) and the warn-only decision. The mounted tests got deterministic ages (children started 1.3 s apart) because rank no longer decides, and the mounted pinned-rustc test now suspends the non-critical test binary and keeps the build-critical rustc. Warden suites: 46 pass, 0 fail (twice in a row, including the overhead test); `🧢️unit-ceiling` 11, fleet-status 28, remote worker 17, unit-cache cancel 2 pass.

Live: the warden restarted onto digest `a5e21d49117559db` (pid 28036, enforce) and is running. Note for whoever owns it: the ceiling rule in the current sources only *reports* a unit over its ceiling below danger and cancels it at danger (changed in the tree after section 1 was written, with matching oracle, fixtures and tests); sections 1 and 5 above still describe the earlier cancel-at-once behaviour, and `live-proof-ceiling.sh` now ends with the proof process reported, not killed, on a calm host.
