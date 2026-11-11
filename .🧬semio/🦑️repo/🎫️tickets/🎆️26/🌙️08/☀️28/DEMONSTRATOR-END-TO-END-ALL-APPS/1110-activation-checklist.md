# 🚦️ 1110 Activation Go/No-Go Checklist (lane actprep, 2026-11-11 00:50)

Header for every call: `cd /Users/ueli/Documents/semio; C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1105b"; D="/Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator"`

Verdict at 00:50: NO-GO. Blocking rows are marked NO.

## Gate table (all must be GO before step 21 freeze)

| # | Gate | Check command | State now |
|---|---|---|---|
| 1 | hub-ok-1107 fresh | `bash "$C/1105b-tools/value/gate.sh" <n>`, then `cat "$G/value-<n>-gate.rc"` (want `hub=0 ext=0`) | NO: marker 11-09 12:57 is older than hub edits 11-09 15:15 (`🌎️hub/…/🪐️space/…`); hub still red per claims |
| 2 | ext-ok-1107 present and fresh | written only by gate.sh on rc 0 | NO: absent (old unversioned `ext-ok` 11-06 is not read) |
| 3 | lock/sync current | `lock-155.rc`=0 (00:03); `sync-155` running (holds javascript-dependency-build) | WAIT: `sync-155.rc` absent; prefreeze `sync-run` passes only once rc 0 exists. No Cargo.toml newer than lock-155 |
| 4 | disk-host | `df -g /Users/ueli/Documents/semio` (>= 100 GiB) | NO: 80 GiB free. Needs disk prune by coordinator (stale incremental sessions, ticket nx caches) |
| 5 | slots idle | `ls "$G/slots"` (no s*, empty queue) | NO: 10 held, 5 queued; cargo admission queue has 10 waiters (memory, 5.9 GB each, 2.9 GB headroom) |
| 6 | no [DEBUG] | `git grep -n -I -F '[DEBUG]' -- '*.rs' '*.ts' '*.tsx' ':!.🧬semio'` | NO: 35 lines in 17 files (accepted D3 only for census literals) |
| 7 | channel pin | prefreeze `channel-pin-code` | GO: 25/25/25/25 |
| 8 | descriptors | step 26/27 below, then `grep -L '"appChannelVersion": 25'` over closure descriptors must list none | NO: all 71 committed pairs still say 24 |
| 9 | graph | see "Graph" | NO: gen 902 stale, new generation required |
| 10 | e2e harness | `"$C/1105b-tools/e2eready/run.sh" --check` | GO: CHECK GREEN 8/8 (run 00:20) |
| 11 | named inputs / wasm-debug0 | prefreeze | GO |

Accepted FAILs of prefreeze (by earlier ruling): `slot-policy` (D2, window 0/1 differs from its 0/0 expectation) and `no-debug-lines` (D3, census literals only). Anything else FAIL = NO-GO. Today also FAIL: disk-host, slots, sync-run, hub-ok, ext-ok.

## Order once gates 1-6 are GO

1. `bash "$C/1105b-tools/prefreeze.sh" > "$G/prefreeze-pre.txt" 2>&1; echo rc=$?`
2. Freeze and window: `printf 1 > "$G/freeze"; printf 0 > "$C/1105b-tools/slots.k"; printf 1 > "$C/1105b-tools/slots.wextra"`
3. Descriptors BEFORE canary (decision: step 26 first, avoids a wrong-reason canary failure). Fixed `canary/describe.sh` (now pins `SEMIO_BUILD_MODE=prototype`; the driver refuses `--profile prototype` under any other mode, `driver-stages/🟦️.ts:113`; `closure` keyword = the 32 ids):
   - energy: `( nohup "$C/1105b-tools/slot.sh" W-act-150 "$C/1105b-tools/canary/describe.sh" act-150 energy > "$G/W-act-150.txt" 2>&1 < /dev/null & )`; `cat "$G/act-150.rc"`
   - all 32: `( nohup "$C/1105b-tools/slot.sh" W-act-200 "$C/1105b-tools/canary/describe.sh" act-200 closure > "$G/W-act-200.txt" 2>&1 < /dev/null & )`; `cat "$G/act-200.rc"`
   - check: `git ls-files -m -- '*🧩️component/🔣️.json' '*🛂️.descriptor.semio'`; every closure json must say `"appChannelVersion": 25`; demonstrator must carry `activationEvents` and `capabilityRequests`.
   - Descriptor files are freeze-class: after they change, rerun gate.sh (markers go stale) and wait for the graph pass.
4. Canary (energy, 6106): `( nohup "$C/1105b-tools/slot.sh" W-canary-act-2 "$C/1105b-tools/canary/activate.sh" canary-act-2 > "$G/canary-act-2.slot.txt" 2>&1 < /dev/null & )`; rc in `$G/canary-act-2.rc`; then `canary/serve.sh start` (exit 3 busy, 4 no receipt, 5 early exit, 6 not 200, 7 vite race), `canary/probe.sh "http://127.0.0.1:6106/?plugin=energy"; cat "$G/canary-probe/run.rc"`, `canary/serve.sh stop; lsof -nP -iTCP:6106 -sTCP:LISTEN` empty.
5. `bash "$C/1105b-tools/prefreeze.sh" > "$G/prefreeze-go.txt" 2>&1`
6. `( nohup sh -c "\"$C/1105b-tools/slot.sh\" W-act-160 env SEMIO_PROGRESS_DEADLINE_TRACE=1 SEMIO_LANGUAGE=en repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:activate-prototype; echo \$? > \"$G/W-act-160.rc\"" > "$G/W-act-160.txt" 2>&1 < /dev/null & )`. Target verified: dependsOn prepare-prototype (9 prepare-*-react-prototype + 4 wasm) and activate-lanes-prototype (energy, fem3d, generator). Success: rc 0, `$D/dist/♻️activation/prototype`, three receipts under `.cache/b/p/m/browser/prototype/runtime/react/{generator,energy,fem3d}/activation/🔣️receipt.json`.
7. `"$C/1105b-tools/serve.sh" start; echo rc=$?` (needs the 3 receipts + union dir; exit 4 missing, 7 vite race).
8. `"$C/1105b-tools/e2eready/run.sh" http://127.0.0.1:6029/ --panes generator,koordinator,aggregator,energie,aussuchen,bearbeiten,verfolgen,statik --languages en,de` with `E2E_ACTIONS_ALL_LANGUAGES=1`; rc in `$G/e2eready/run.rc`.
9. test-e2e (50): runbook step 34 verbatim. Then teardown (runbook step 37).

## Graph (repo MCP `graph`, `leases_status`)

- Log evidence: generation 899 (1310 s, 4963 projects), 900 (428 s), 901 (437 s, 512 refreshed), 902 published (791 s, 5067 projects; `W-graphenv-2`, 11-09 19:21). 902 is the latest published generation seen; no 903 yet.
- Freeze-class edits AFTER gen 902: four `📋️project.json` (11-10 03:14 and 03:30: remodel TS package, os/dev TS package, repo caching, repo selection/scope), 3 modified `Cargo.lock`, and new crate `semio-framework-plugin-reactor-staged-forms` (🔌️plugin/⚛️reactor/🔄️turn/📥️receiving/📝️forms Cargo.toml) plus lock-154/155. So a new pass is REQUIRED: PENDING EDITS = YES.
- `graph` MCP with `graph_policy=current` failed at 00:40 (exit 1, 27 min): every test project depending on `semio-framework-plugin-reactor-staged-forms` is "withheld: depends on projects this snapshot does not contain" (snapshot lacks the new crate, needs a rebuild), and the report writer throws `Declared build report name required` (`📊️reporting/🟦️.ts:13`; the `file` argument is not a declared report name, do not pass `file`). Rerun without `file` after the next generation.
- Duration: 7 to 13 min for a 5000-project pass on an idle host (428/437/791 s), 22 min under load (1310 s), up to 90 min in the handover worst case. Memory admission is the limiter: 10 cargo waiters queued 4-13 min, headroom 2.9 GB vs 5.9 GB per grant, plus `sync-155` (javascript-dependency-build, exclusive) holding the namespace. Plan 30 to 90 min after the LAST freeze-class edit; any further edit restarts it.
- `leases_status` at 00:44: no cycles, no waiters on leases; only `dependencies sync --all` (pid 76634) holds image/namespace leases (etaMinutes 2). Violations listed are old (not enforced).

## Descriptor refresh state (step 26/27)

Not run to completion: `slot.sh W-act-150` waited 9 min for a slot (10 held, 5 queued ahead), stopped by my bounded wait; ticket removed, nothing changed. No descriptor file changed by this lane. Compile reds are not yet reachable; once a slot is granted the compile of energy needs hub/ext green. Owners of the red parts (from claims): store (replay-operation, snapshot-clone: store3/4/5), instance-render and instance-dispatch (gen3dfinal, gen3dfinal2), reactor-wit-bridge 102 errors (reactorturn), energy engine (paneB/energie), value-retirement arc-custody (closeshared, FIXED 23:53).
