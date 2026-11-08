# 🧭️ 1107 Runbook: from hub-ok-1107 to done (lane runbook, 2026-11-07)

STATUS: UPDATED, NOT RUN. Read-only pass: no activate, serve, lock, sync, e2e, cargo or git writes. Tags: [V] = the command, script or marker was read today (file path given); [I] = inferred from text or code, never run.

Steps: one numbered row = one command or one action. Token class: L = machine time (coordinator polls only); A = needs an agent or reasoning (fixes, triage, decisions).

## 0. Header for every Bash call [V]

```
cd /Users/ueli/Documents/semio
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1105b"
D="/Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator"
```

Background launch, detached, never from a Monitor, never `pkill -f`:
`( nohup "$C/1105b-tools/slot.sh" <LABEL> <cmd...> > "$G/<LABEL>.txt" 2>&1 < /dev/null & )`
`slot.sh` writes no `.rc`; scripts that write their own `.rc`: `chk.sh`, `value/ext.sh`, `canary/activate.sh`, `e2eready/run.sh`. Others: wrap the command in `sh -c '<cmd>; echo $? > "$G/<LABEL>.rc"'`.

Bounded wait (at most 9 min per call): `for i in $(seq 1 18); do [ -f "$G/<LABEL>.rc" ] && break; sleep 30; done; cat "$G/<LABEL>.rc" 2>/dev/null`

## 1. State at 2026-11-07 about 20:50 [V]

| Item | State | Source |
|---|---|---|
| Gate markers | `hub-ok-1107` absent, `ext-ok-1107` absent | `ls $G` |
| Hub map | `main-157` rc 1, `main-158` rc 1 (`semio-os-store-replay-operation` 2 errors, `semio-os-store-snapshot-clone` 14 errors), `main-159` running | `$G/main-15{7,8}.rc`, `main-158.txt` |
| Lock and sync | `lock-111.rc` 0, `sync-111.rc` 0; `lock-ok-1107`, `sync-done-1107`, `lock-sync-1107-ok` present | `$G` |
| Freeze | absent | `$G/freeze` |
| Slots | `slots.k` = 8, `slots.wextra` = 2 (the old runbook said 7/2; 8/2 is the value to restore); 10 slot dirs `s1`..`s10` exist, owners unchecked | `1105b-tools/slots.k`, `slots.wextra`, `$G/slots` |
| Canary | `canary-act-1` killed earlier; no `canary-go` marker (handover step 4 names one, it does not exist) | `$G` |
| `[DEBUG]` | 41 lines in 26 tracked `.rs/.ts/.tsx` outside `.🧬semio` (plain `git grep`); 6 untracked source files not grepped. Census of 14:14: 98 lines, 47 candidates, 28 need care | `git grep`, `1107-audit-debugcensus.md` |

Hub is NOT green. Nothing after step 2 may start until `gate.sh` returns 0.

## 2. Order

| # | Step | Command or action | Tag | Class |
|---|---|---|---|---|
| **A** | **Hub and ext gate** | | | |
| 1 | State check, read-only | `ls $G`; `cat` slots; `uptime`; `ps -axo pid,ppid,etime,command` for `[c]argo`, `[r]ustc`, `[r]epo ` | V | L |
| 2 | Hub, then 21 ext components; writes `hub-ok-1107` and `ext-ok-1107` only on rc 0 | `bash "$C/1105b-tools/value/gate.sh" <n>` (runs `hub.sh value-<n>-hub`, then `value/ext.sh <n>`, even if the hub failed) | V | L |
| 3 | Read the gate result | `cat "$G/value-<n>-gate.rc"` (`hub=0 ext=0`); logs `value-<n>-hub.txt`, `value-<n>.txt` | V | L |
| 4 | Host crates `check --tests` | `1105b-tools/hostcheck.sh`; read its header for arguments first | I | L |
| **B** | **a11y apply (after steps 2-3)** | | | |
| 5 | Dry runs, read-only | `cd "$C/1107-a11y" && python3 -I core.py; python3 -I localize.py`; expect core 88 edits / 47 files / 0 stale, localize 0 unmatched | V | L |
| 6 | Apply core first | `python3 -I core.py --write` (exit 1 on stale anchor) | V | L |
| 7 | Apply localize second | `python3 -I localize.py --write` (exit 1 on unmatched) | V | L |
| 8 | Gate | `python3 -I check.py`; exit 0 required (baseline 66 violations) | V | L |
| 9 | Hand edits core did not script: workflow record and sqlite columns, schema JSON, norm plugin, TS tests, `workflow_node_for_app` default-language leak, persisted fixtures, host label emit | owners, compile-driven; list in `1107-result-a11y.md` | V | A |
| 10 | Generator regeneration (schema-catalog, generated TS, mutation JSON, rows) | `repo run @semio-tech/framework:generate` (name from the a11y result; not found in a project.json) | I | L |
| 11 | Lock and sync for LOCK NEEDED, first one: `semio-ui-locale` to `semio-framework-pack-json` (reach.py: no cycle) | `repo dependencies lock --all`, then `sync --all` (tools `dependencies_lock` and `dependencies_sync` exist) | I | L |
| 12 | Compile the a11y closure, wasm: pack-json, ui-locale, manifest-media, app-surface, plugin builder and declaration, workflow, host, 8 pane crates, wgpu shell | `"$C/1105b-tools/chk.sh" a11y-<n>-<name> <Cargo.toml> wasm -p <crate>` per crate | I | L |
| **C** | **Toolsplit and pane smokes (en and de)** | | | |
| 13 | Toolsplit, once the store wall clears | `chk.sh toolsplit-102 Cargo.toml native -p semio-plugin-tool-machine -p semio-plugin-tool-run -p semio-plugin-tool-run-operations -p semio-plugin-tool-run-turns`; the wasm twin; `runtime/tst.sh toolsplit-103 Cargo.toml native -p semio-plugin-tool-run --lib selected_entry_required_tests settle_presentation_turn_tests`; `tst.sh toolsplit-104 ... -p semio-framework-tool-run-entities -p semio-framework-tool-run-presentation` | V | L |
| 14 | Pane smokes, one command per pane; each law loops `PANE_SMOKE_LOCALES` (en, de), fresh fixture per language: `proc/smoke.sh <label> en`, `koordinator/tst.sh`, `aggregator/smoke.sh`, `energie/go102.sh <n>` (waits for `lock-sync-1107-ok`), `aussuchen/smoke.sh`, `bearbeiten/smoke.sh`, `verfolgen/tst.sh` (exact test not recorded), `statik/smoke.sh` (`statik/retry.sh` for transient refusals) | V (scripts exist; verdicts unknown) | L (triage: A) |
| 15 | Remove `aussuchen_apply_probe` and its 4 prints; decide the no-op apply (hypothesis: `curation_add` resolves to NoOp for an id not in `stock_extra`) | owner aussuchen | I | A |
| **D** | **[DEBUG] removal, before the freeze** | | | |
| 16 | List hits | `cd /Users/ueli/Documents/semio && git grep -n -I -F '[DEBUG]' -- '*.rs' '*.ts' '*.tsx' ':!.🧬semio'`, plus the untracked loop | V | L |
| 17 | Remove class T (temporary logs) first, then class R (repo product). Keep the census checker and its fixture, and the nx parser constant. Change the nx test contract (`operation/🟦️.ts:60`, `:63`) and the custody assertion (`:9`, `:23`) before removing their markers | owners; 47 candidates, 28 need care | V (counts), I (classes) | A |
| 18 | Verify | `cd /Users/ueli/Documents/semio && bash "$C/1105b-tools/prefreeze.sh" > "$G/prefreeze-debug.txt" 2>&1`; `no-debug-lines` still fails on census literals until decision D3 | V | L |
| **E** | **Lock, sync, freeze** | | | |
| 19 | Lock and sync again, only if a Cargo.toml changed after `lock-111` | same as step 11 | I | L |
| 20 | Prefreeze evidence (2 to 5 min) | `bash "$C/1105b-tools/prefreeze.sh" > "$G/prefreeze-pre.txt" 2>&1; echo rc=$?`. Accepted FAILs: `slot-policy` (D2) and `no-debug-lines` (D3) | V | L |
| 21 | Freeze on and slot window 0/1 (only one W- run is admitted at a time) | `printf 1 > "$G/freeze"`; `printf 0 > "$C/1105b-tools/slots.k"`; `printf 1 > "$C/1105b-tools/slots.wextra"` | V | L |
| **F** | **Canary: energy only, port 6106** | | | |
| 22 | Canary activation, through a W- slot | `( nohup "$C/1105b-tools/slot.sh" W-canary-act-2 "$C/1105b-tools/canary/activate.sh" canary-act-2 > "$G/canary-act-2.slot.txt" 2>&1 < /dev/null & )`. Success: `canary-act-2.rc` 0 and `.cache/b/p/m/browser/prototype/runtime/react/energy/activation/🔣️receipt.json` | V | L |
| 23 | Canary serve | `"$C/1105b-tools/canary/serve.sh" start`. Exit 4 no receipt, 3 port busy, 5 exited early, 6 not 200, 7 Vite race | V | L |
| 24 | Canary probe, en and de | `"$C/1105b-tools/canary/probe.sh" http://127.0.0.1:6106/?plugin=energy; echo rc=$?`; `cat "$G/canary-probe/run.rc"`. On FAIL read `canary-probe/diagnostics-<lang>.json` first | V | L (triage: A) |
| 25 | Canary stop | `"$C/1105b-tools/canary/serve.sh" stop; lsof -nP -iTCP:6106 -sTCP:LISTEN` (empty = free) | V | L |
| **G** | **Descriptor refresh (section 15 of the old runbook, kept here)** | | | |
| 26 | Energy descriptor pair | `( nohup "$C/1105b-tools/slot.sh" W-act-150 env SEMIO_LANGUAGE=en SEMIO_BUILD_MODE=prototype repo run workspace:components -- describe energy --profile prototype > "$G/W-act-150.txt" 2>&1 < /dev/null & )`. The `--` form is already in `canary/describe.sh` (fixed 06:55); the `SEMIO_BUILD_MODE` pin is not | I | L |
| 27 | All 32 ids in one run, after step 26 is clean | `... repo run workspace:components -- describe demonstrator cad gis procedural process puzzle sourcing flow playbook fem energy <30 extension and module ids> --profile prototype` (full list in the old runbook, section 15) with label `W-act-200` | I | L |
| 28 | Check | `grep -L '"appChannelVersion": 25'` over the 32 committed `🧩️component/🔣️.json` (none may say 24); demonstrator must carry `activationEvents` and `capabilityRequests` | V | L |
| **H** | **activate-prototype, serve 6029** | | | |
| 29 | Prefreeze, go | `bash "$C/1105b-tools/prefreeze.sh" > "$G/prefreeze-go.txt" 2>&1` | V | L |
| 30 | activate-prototype through a W- slot | `( nohup sh -c "\"$C/1105b-tools/slot.sh\" W-act-160 env SEMIO_PROGRESS_DEADLINE_TRACE=1 SEMIO_LANGUAGE=en repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:activate-prototype; echo \$? > \"$G/W-act-160.rc\"" > "$G/W-act-160.txt" 2>&1 < /dev/null & )`. Success: rc 0, `$D/dist/♻️activation/prototype`, three lane receipts (generator, energy, fem3d). Do not use `act.sh`: it sets `SEMIO_LEASE_PRIORITY=interactive`, which the brief forbids | V | L |
| 31 | Serve 6029 | `"$C/1105b-tools/serve.sh" start; echo rc=$?`. Needs the three receipts and the union directory; success prints `ready, GET / -> 200` | V | L |
| **I** | **Browser verdict and tests** | | | |
| 32 | Harness self-check, no server | `"$C/1105b-tools/e2eready/run.sh" --check; echo rc=$?` | V | L |
| 33 | Browser verdict, 8 panes, en and de | `E2E_ACTIONS_ALL_LANGUAGES=1 "$C/1105b-tools/e2eready/run.sh" http://127.0.0.1:6029/ --panes generator,koordinator,aggregator,energie,aussuchen,bearbeiten,verfolgen,statik --languages en,de`. Output `$G/e2eready/run.log`, `run.rc`. Pane ids come from `e2eready/expect.json` (`energie` and `statik`, not `energy`/`fem3d`) | V | L (triage: A) |
| 34 | test-e2e, 50 executions (25 cases x 2 languages; baseline excludes `@behaviour`) | `( nohup sh -c "\"$C/1105b-tools/slot.sh\" W-e2e-160 env SEMIO_TICKET_DIR=\"$C\" repo run @semio-tech/mit-bestand-demonstrator:test-e2e; echo \$? > \"$G/W-e2e-160.rc\"" > "$G/W-e2e-160.txt" 2>&1 < /dev/null & )`. `test-e2e` depends on `serve-e2e` (its own server); `prepare-e2e` mutates the host store, so no other store writer runs beside it. Do not set `CI` | V | L |
| 35 | `@behaviour`: 8 cycle cases in en and de (result file: 16 executions for `@cycle`, 40 for `@behaviour`; never run) | no repo target exists. `repo playwright test --config "$D/🧪️tests/🎭️acceptance/🧭️behaviours/🎚️config/🟦️.ts" --grep @behaviour` with `PLAYWRIGHT_BASE_URL=http://127.0.0.1:6029` (command shape from the config header) | I | L (triage: A) |
| **J** | **Release build, close-out** | | | |
| 36 | Release build through a W- slot | `( nohup sh -c "\"$C/1105b-tools/slot.sh\" W-build-160 env SEMIO_LANGUAGE=en repo run @semio-tech/mit-bestand-demonstrator:build; echo \$? > \"$G/W-build-160.rc\"" > "$G/W-build-160.txt" 2>&1 < /dev/null & )`. Network needed for map tiles | V | L |
| 37 | Teardown | `"$C/1105b-tools/serve.sh" stop`; `printf 8 > "$C/1105b-tools/slots.k"`; `printf 2 > "$C/1105b-tools/slots.wextra"`; `mv "$G/freeze" "$G/freeze.done"` | V | L |
| 38 | Close-out | results summary in the ticket; remove tool output under `$G` (keep reports and inputs; resolve the AGENTS.md rule "delete the generated folder" against the handover before deleting); empty `📌️important.md`; `ticket_close` with explicit path `26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS` | V | A |

Totals: 38 steps. Inferred (I): steps 4, 10, 11, 12, 15, 17, 19, 26, 27, 35 = 10.

## 3. Traps

- Graph verification restart: a pass ends with `graph: generation N published` (seen as 899, 900, 901 in logs) [V]. Any edit in the freeze classes during a pass restarts it (about 90 min per the handover). Freeze classes: A `Cargo.toml`, `Cargo.lock`, `package.json`, `bun.lock`, `*.patch`, `project.json`; B `🧰️framework/🛍️products/🦑️repo/**` and framework modules it imports; C sources of the 32 closure components and `🌎️hub/**`; E the 32 descriptors. Wait for `generation N published` after the last such edit. The cause of restarts from `project.json` edits is not in code [I]. After one clean pass, `SEMIO_BUILD_GRAPH_POLICY=current` is legitimate (handover, not read in code).
- Markers: `gate.sh` never deletes an old `hub-ok-1107` or `ext-ok-1107`. `prefreeze.sh` `marker_gate` fails when a marker is older than the last edit under `🌎️hub` or a plugin `🧩️extensions/`. Rerun `gate.sh` after any such edit (a11y, descriptor).
- `ext.sh` writes `value-<n>.rc` as a bad-count flag (0 = all pass), not an exit code.
- `core.py` says "apply only after hub-ok-1107" in its docstring only; the code does not check. The order is yours to keep.
- Order risk: canary activation (step 22) runs before the descriptor refresh (step 26). If activation also rejects channel 24, the canary fails for the wrong reason. Decision for the coordinator: run step 26 first (section 15 step 1) [I].
- Slot window 0/1: non-W labels get `K = 0` and wait. `slot-policy` FAIL is accepted (D2).
- Load and limits: never raise limits (handover). Activation peaked near 8 GB in the canary log. Check `uptime` and the 10 slot dirs before step 22.
- Hang guard: `slot.sh` kills only cargo test binaries (`.../deps/<name>-<16 hex>`) that print nothing for `SLOT_HANG_SECS` (default 1200 s, x3 when the log is a pipe), TERM then KILL after 10 s. Pass a larger value by env on the slot call when a long test is expected. Proven on a fake process only, not on a real `cargo test`.
- Monitor-launched builds die when the monitor ends. Use the double fork. Kill by pid tree, never `pkill -f` (kills wrappers, orphans cargo).
- `act.sh` is stale (sets `SEMIO_LEASE_PRIORITY`). Not used.
- Vite race (serve exit 7): stop by pid, empty the `viteCacheDirectory` named in `🏗️builder/🌐️vite/🟦️.ts` (line not re-read today), start once more.
- Dependency receipt stale (`An installer input was replaced`): every run re-verifies for about 45 min. Fix with `dependencies sync --all`, `lock --all` before the freeze, not during.
- Hub is red now: `main-158` fails in `semio-os-store-replay-operation` and `semio-os-store-snapshot-clone`. Owners: store3/store4/store5 (claims). Step 2 waits for them.

## 4. Stale items removed or corrected

- Old 6a describe tool bug (`Unexpected build argument before --`): fixed in `canary/describe.sh`; the 31 sequential describes (old 6b) are replaced by one run (step 27).
- Old slot restore 7/2: now 8/2, as read today.
- Old lock-105 and sync-105: now lock-111 and sync-111, both rc 0.
- Old e2e pane ids `energy`, `fem3d`: now `energie`, `statik` (expect.json).
- Old `act.sh` activation path: removed (forbidden env var).
- Old `canary-go` and "resume canary lane": removed (no such marker).
- Old `[DEBUG]` list of 73 lines: replaced by the census figures (47 candidates, 28 careful) and today's count.
- Old a11y order "localize, then core": reversed to core then localize (a11y result, ruling).
- Added: toolsplit (step 13), behaviour run (step 35), lockfix note (below), a11y hand edits (step 9).
- Lockfix is done: both corrupt `Cargo.lock` files repaired and the writer hardened (`1107-result-lockfix.md`). No step.

## 5. Token-light vs agent steps

- Machine time (L, the coordinator only polls): every step marked L in the table. Largest: step 2 (hub plus ext), 12 (compile), 14 (pane smokes), 27 (32 describes), 30 (activation, 6 to 9 h at 2 jobs per act22), 34 (test-e2e, 30 to 60 min), 36 (build).
- Needs an agent (A): 9 (a11y hand edits), 14 (triage of pane reds), 15 (aussuchen), 17 (debug owners), 24, 33 and 35 (probe triage), 38 (close-out). Batch these after the machine steps.
- Coordinator-only (per handover, the lane never runs them): lock and sync (11, 19), canary serve (23), activate-prototype (30), serve 6029 (31).
- No sub-agents in this lane (brief). Pane fixes are owner lanes, not part of this runbook.

---

## Report (lane runbook, 2026-11-07)

Read today: `1107-HANDOVER.md`, `1107-result-{descriptor,actinfra2,panede,behaviours,a11y,lockfix,toolsplit}.md`, `1107-audit-{acttargets,debugcensus,paneready,a11ycheck}.md`, `1105b-tools` scripts (`slot.sh`, `hub.sh`, `value/gate.sh`, `value/ext.sh`, `prefreeze.sh`, `chk.sh`, `serve.sh`, `canary/*`, `e2eready/run.sh`, `panes.sh`, `act.sh`), `1107-a11y/*.py`, the demonstrator `project.json` targets, `e2eready/expect.json`, and the state of markers, logs and slots. Nothing executed.

Findings that change the old runbook:
- Hub is red (`main-157`, `main-158` rc 1; `main-159` running). The runbook starts at step 2.
- `act.sh` sets a forbidden env var; `gate.sh` runs ext even when hub is red; `ext.sh` leaves `ext-ok-1107` to `gate.sh` only.
- The canary order (activation before descriptor refresh) may fail for the wrong reason (trap).
- `e2eready` pane ids are `energie` and `statik`.
- No repo target exists for `@behaviour`; it runs by Playwright config.
- Pane smokes: none green since the shared fixes (paneready). `panes.sh` covers only 5 of 8 panes and runs no en/de loop of its own; the law does the en/de loop.

Steps total: 38. Inferred: 10.
