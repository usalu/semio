# 🔎️ 1112 Audit Activation (read-only, 2026-11-12)

Scope: read-only audit of the activation lane. No source edits, no git writes, no builds, no sub-agents. Checked: `1110-activation-checklist.md`, the `1105b-tools` folder, `🗑️generated/1105b` markers, the committed descriptor files, and a literal `[DEBUG] ` scan.

## 1. DEBUG lines

Literal `[DEBUG] ` (trailing space) found with `/usr/bin/grep -rnF --include=*.rs --include=*.ts --include=*.tsx --include=*.go --include=*.sh`, excluding `.🧬semio`, `.cache`, `node_modules`, `target`, `dist`. Result: 34 lines in 18 files. No `.go` or `.sh` hits. Full lines below (verbatim, `./path:line:` prefix).

Reconciliation: the checklist gate 6 uses `git grep -F '[DEBUG]'` (no trailing space) on tracked `*.rs *.ts *.tsx`. That gives 41 lines in 21 files. The extra hits are not temporary logs: `🧪️test/…/📝️docstring-and-debug-census/🟦️.ts` (the census checker itself, lines 152-182), a `leftover` literal in `🎓️teaching/…/🌐️site/🟦️.ts:107`, and one test string in `…/📥️consumption/🟦️.ts:41`. The checklist's "35 lines in 17 files" does not match the 34/18 seen now.

Classes seen in the list: commented-out `// [DEBUG]` block in `🌉️mcp/🔬️probe-document/🦀️.rs:15-25` (remove the whole commented block); `eprintln!("[DEBUG] …")` runtime traces in `⏪️time-travel`, `🧪️artifact-app-laws`, `🎟️registration`, `🔁️replay/🧬️supersession`, `🖥️host/📤️publication`; `[DEBUG]` inside assertion messages in `♻️cancellation`, `🧬️brep` differential tests (`KB1_TRACE` gated).

```
./🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🔀️boolean/🧪️tests/🔬️differential/🦀️.rs:74:    let mut rec = OpRecorder::new();if std::env::var_os("KB1_TRACE").is_some(){eprintln!("[DEBUG] case start");}
./🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🔀️boolean/🧪️tests/🔬️differential/🦀️.rs:75:    let a = build_operand(&mut body, &mut rec, &entry["a"]).map_err(|e| format!("operand a: {e:?}"))?;if std::env::var_os("KB1_TRACE").is_some(){eprintln!("[DEBUG] operand a built");}
./🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🔀️boolean/🧪️tests/🔬️differential/🦀️.rs:76:    let b = build_operand(&mut body, &mut rec, &entry["b"]).map_err(|e| format!("operand b: {e:?}"))?;if std::env::var_os("KB1_TRACE").is_some(){eprintln!("[DEBUG] operand b built");}
./🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🔀️boolean/🧪️tests/🔬️differential/🦀️.rs:83:    let outcome = boolean_solid(&mut body, a, b, op, 1e-6, &mut rec);if std::env::var_os("KB1_TRACE").is_some(){eprintln!("[DEBUG] boolean done");}
./🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🧪️tests/original-loan.rs:116: let mut actor=NativeResidentSuccessAck::<ValueError>::vacant();actor_close_phase(bank,native,&mut actor,0);let mut producer=OriginalEffectAuthority::new(PlainSuccess);let demand=RetirementDemand{depth:1,..Default::default()};assert!(matches!(bank.run(native,|control|control.receive
./🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/📦️effect/🧪️tests/retained-outcome.rs:85: let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{let _=bank.run_with_turn(&mut native,|turn,native|native.with_original_source_witness(turn.witness(),|_,_|->Result<(),ValueError>{panic!("[DEBUG] original witness loan unwind")}));}));assert!(int
./🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📤️publication/🧵️work/🧪️tests/🦀️.rs:31:  for _ in 0..1_000_000{if registry_retirement.terminal_is_empty(){break}let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:registry_retirement.next_capacity_byte_demand(64).unwrap(),maximum_release_bytes:registry_retiremen
./🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📤️publication/🧵️work/🧪️tests/🦀️.rs:65: assert_eq!(observe(||drop(decoder)),((),0,0));assert_eq!(original+births,releases,"[DEBUG] original={original} births={births} releases={releases} turns={turns}; registry={} cache={} infos={} work={}",size_of::<neural::Registry>(),size_of::<neural::NeuralCa
./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🧵️instance/📖️ref-reads/🦀️.rs:44:        if self.time_travel.closing&&!self.time_travel.terminal_is_empty(){eprintln!("[DEBUG] runtime time-travel residue {:?}",self.time_travel.residue());return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"time travel clo
./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/♻️cancellation/🦀️.rs:282:        assert!(std::thread::panicking(),"original cancellation lease requires its paid terminal cursor\n[DEBUG] ledger-lease-drop {}",std::backtrace::Backtrace::force_capture());
./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs:303:        if idle>=maximum_idle{eprintln!("[DEBUG] rb-gate settle-idle {} | close={}",app.typed_operation_diagnostic(),app.close_diagnostic());return Err(semio_framework_value::ValueError::literal(semio_fram
./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🌱️initialization/🫧️ephemeral/🧪️tests/🦀️.rs:9:fn original_ephemeral_source_same_roots_cancelled_prefixes_and_full_physical_close(){let law=law();for cutoff in std::iter::once(usize::MAX).chain(law["cancelAfter"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as usize)){let mut caller=
./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🌱️initialization/🎟️registration/🧪️tests/🦀️.rs:32: for index in 0..1048576{if index<512{eprintln!("[DEBUG] native opening turn={index}");}let(_,p)=turn(&mut caller,&mut decoder,POLICY.preparation,|cx|{if let Some(construction)=construction.as_mut(){let source=definition.original_source().un
./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🎟️birth/🧪️tests/🦀️.rs:60:            let mut paid=(0,0);let mut observed=(0,0);for _ in 0..1000000{if mounted_owner.is_some(){break}let(job,heap)=heap(||turn(&mut caller,&mut decoder,settled,|cx|match ArtifactReservedToolJob::admit_mounted(&mut source,&mut construction,business.as
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🔁️replay/🧬️supersession/🧪️tests/🦀️.rs:10: for step in 0..200000{if cutoff==Some(step){break}let source=events.get(position).map(|event|NativeSupersessionSource{event,previous:position.checked_sub(1).and_then(|at|events.get(at)),document,line:None});let no_grant=RetainedCloneGrant{maxi
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🔁️replay/🧬️supersession/🧪️tests/🦀️.rs:13: if cutoff.is_none(){assert!(complete,"[DEBUG] native supersession coroutine must finish at event={position} quote={:?}",owner.original_job_demand(4096));let actual:serde_json::Map<String,serde_json::Value>=table.iter().map(|(target,value)|(tar
./🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🌱️process-arena/🦀️.rs:32:#[cfg(test)]mod tests{use super::*;#[test]fn process_arena_returns_normally_only_after_real_source_empty(){if let Ok(mode)=std::env::var("SEMIO_MCP_PROCESS_ARENA_CHILD"){let policy=serde_json::from_str(include_str!("../🎚️config/🏛️host-driver-policy.json")).unwrap();let operation=serd
./🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔬️probe-document/🦀️.rs:15:// [DEBUG] #[cfg(test)]
./🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔬️probe-document/🦀️.rs:16:// [DEBUG] #[path = "../🏠️workspace/🪶️sqlite/🫴️native/🧪️tests/🦀️.rs"]
./🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔬️probe-document/🦀️.rs:17:// [DEBUG] pub mod probe_sqlite_original_io;
./🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔬️probe-document/🦀️.rs:19:// [DEBUG] #[cfg(test)]
./🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔬️probe-document/🦀️.rs:20:// [DEBUG] #[path = "../🏠️workspace/🪶️sqlite/🫴️semantic/🧪️tests/🦀️.rs"]
./🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔬️probe-document/🦀️.rs:21:// [DEBUG] pub mod probe_sqlite_original_semantic;
./🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔬️probe-document/🦀️.rs:23:// [DEBUG] #[cfg(test)]
./🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔬️probe-document/🦀️.rs:24:// [DEBUG] #[path = "../🏠️workspace/🪶️sqlite/🧪️tests/🦀️.rs"]
./🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔬️probe-document/🦀️.rs:25:// [DEBUG] pub mod probe_sqlite_tests;
./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🎛️operation/🟦️.ts:60: await issuance.receiveScriptProcessInvocation(process.env,async root=>{const control=issuance.createIssuedResourceControlV1(root,"fixture:graph-root-child",90000),directory=join(repository.root,".cache/b/l/m
./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🎛️operation/🟦️.ts:63: const owner="fixture:graph-root:"+randomBytes(6).toString("hex"),rows:Record<string,unknown>[]=[],prefix="[DEBUG] graph-root ";let tail="",ready!:()=>void,settled=false,failed=false;const entered=new Promise
./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts:336: if(selection.kind==="products-graph-fixture"){if(!row.producer||!row.fixture)throw Error("Current exact fixture and known producer selector required");stable(row.producer,schema.definitions.CurrentMaintenanceProtocol.const.maximumProduc
./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🧪️tests/🌊️output/👷️child/📜️script.ts:9: await receiveScriptProcessInvocation(process.env,async original=>{const control=createIssuedResourceControlV1(original,"custody:output-fixture",0),write=process.stdout.write.bind(process.stdout);let
./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🧪️tests/🌊️output/🟦️.ts:21: const crossSpawn=createRequire(import.meta.url)("cross-spawn"),independent=crossSpawn.sync(process.execPath,["-e",'const value=await Bun.stdin.json();await new Promise((accept,reject)=>process.stdout.write(JSON.strin
./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🧪️tests/🌊️output/🟦️.ts:23: const delivered=await captureOwnedCommandResultV1(process.execPath,[wrapper,"capture-program",root,entry],root,{control:control.leaseControl,output:{maximumBytes:schema.$defs.CargoPreparationControlProtocolV1.const.m
./🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy/🌐️sealed-publication-and-bundle/🟦️.ts:64:    expect(siteArtifactProblems(tree({ ...released(COURSE_PROCTOR_ORIGIN), "assets/app-AbCd1234.js.map": "{}", "assets/chunk.js": "//# sourceMappingURL=chunk.js.map\nconsole.log('[DEBUG] x')" }), COURSE_PROCTOR_ORIGIN)).toEqual(["assets/app-AbCd1234.js.map carries no co

```

## 2. Checklist status (preconditions on disk now, nothing run)

Verified on disk at audit time. "Not verified" means the check needs a run or the repo MCP, which is down in this session (`repo` MCP: CONNECTION_CLOSED).

| # | Gate (checklist row) | Precondition as written | State now | Verdict |
|---|---|---|---|---|
| 1 | hub-ok-1107 fresh | `value-<n>-gate.rc` shows `hub=0 ext=0` | `hub-ok-1107` still dated 11-09 12:57. Newest gate rc: `value-124-gate.rc` = `hub=1 ext=1`; `value-1115.rc` = `1`; `value-1115-gate.rc` empty (no verdict). Hub edits date (11-09 15:15) not re-checked. | NO |
| 2 | ext-ok-1107 present | written by gate.sh on rc 0 | absent. Only old unversioned `ext-ok` (11-06, 0 bytes) exists. | NO |
| 3 | lock/sync current | `lock-155.rc`=0, `sync-155.rc`=0 | Changed since 00:50: `sync-155.rc` = 0 (11-11 00:18). Also `lock-156.rc` = 0 and `sync-156.rc` = 0 (11-11 01:54). Cargo.toml files with mtime newer than `lock-155.txt`: 873 (the checklist claimed none). Likely replica-copy mtimes, not content edits; confirm with `git diff --stat` on Cargo files before deciding. | NO (lock/sync rc green, but Cargo.toml changed after lock-155 per the file count; re-run lock/sync after the last edit) |
| 4 | disk-host (>= 100 GiB) | `df -g` | 68 GiB available on `/System/Volumes/Data` (checklist said 80). | NO |
| 5 | slots idle | `ls slots` no s*, empty queue | `slots/s1`..`s5` present (5 held), `slots/queue` empty. Checklist said 10 held and 5 queued; that changed. | NO |
| 6 | no [DEBUG] | `git grep` count 0 | 34 literal lines (section 1); 41 no-space hits (checklist form). Accepted only for census literals (D3). | NO |
| 7 | channel pin | `prefreeze.sh` channel-pin-code 25/25/25/25 | `prefreeze-pre.txt` and `prefreeze-go.txt` absent; needs a prefreeze run. | NOT VERIFIED |
| 8 | descriptors appChannelVersion 25 | all closure `🔣️.json` and `🛂️.descriptor.semio` say 25 | `git ls-files` finds 142 tracked files: 1 with `"appChannelVersion": 25`, 70 with `24`, 71 with neither key string. `act-150.rc` and `act-200.rc` absent (descriptor refresh never ran). | NO |
| 9 | graph generation current | new pass after freeze-class edits | Checklist: gen 902 latest, freeze-class edits after it. Repo MCP `graph` and `leases_status` cannot be queried in this session. | NO (per checklist; not re-verified) |
| 10 | e2e harness | `e2eready/run.sh --check` green 8/8 | `run.sh`, `expect.json`, `probe.ts`, `tsconfig.json` present. Green status not re-run. | NOT VERIFIED (files present) |
| 11 | named inputs / wasm-debug0 | prefreeze | Not verified (prefreeze not run). | NOT VERIFIED |

Other state on disk:
- `🗑️generated/1105b/freeze` absent (freeze not on).
- `1105b-tools/slots.k` = `8`, `slots.wextra` = `2` (the teardown values from runbook step 37, not the window 0/1 from step 21).
- `canary-act-2.rc`, `W-act-160.rc`, `W-e2e-160.rc`, `W-build-160.rc`, `serve-6029.pid`, `e2eready/run.rc`, `prefreeze-go.txt` all absent: no activation or serve has started.
- Tools present: `prefreeze.sh`, `serve.sh` (port 6029 confirmed, pid file `serve-6029.pid`), `slot.sh`, `value/gate.sh`, `value/ext.sh`, `hub.sh`, `hostcheck.sh`, `canary/{describe,activate,serve,probe}.sh`, `e2eready/run.sh`. `canary/describe.sh` pins `SEMIO_BUILD_MODE=prototype` (confirmed in the file).
- Runbook `1107-runbook.md` is the source for steps 16-38 (checklist references "runbook step 34" and "step 37" for test-e2e and teardown).

## 3. Commands for the coordinator, in order

Header (from the checklist, every call):
```
cd /Users/ueli/Documents/semio; C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1105b"; D="/Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator"
```

Order: (a) debug cleanup, (b) hub/ext gate, (c) lock/sync if Cargo.toml changed, (d) graph generation, (e) prefreeze-pre and freeze, (f) descriptor refresh, gate re-run, graph pass, (g) canary, (h) prefreeze-go, (i) activate-prototype, (j) serve 6029, (k) e2e, (l) release build, teardown. Steps (b), (c), (d) and the re-run in (f) are required by the checklist's gate table and freeze notes even though the checklist's numbered order skips them.

**a. Debug cleanup** (runbook 16-18; owners remove lines, then verify)
```
cd /Users/ueli/Documents/semio && git grep -n -I -F '[DEBUG]' -- '*.rs' '*.ts' '*.tsx' ':!.🧬semio'
bash "$C/1105b-tools/prefreeze.sh" > "$G/prefreeze-debug.txt" 2>&1
```
Expected: `no-debug-lines` still fails on census literals (accepted D3).

**b. Hub and ext gate** (checklist gate 1-2; writes `hub-ok-1107` and `ext-ok-1107` only on rc 0)
```
bash "$C/1105b-tools/value/gate.sh" <n>
cat "$G/value-<n>-gate.rc"
```
Want `hub=0 ext=0`.

**c. Lock and sync** (runbook 19; only if a Cargo.toml changed after the last lock)
```
repo dependencies lock --all
repo dependencies sync --all
```
Tools `dependencies_lock` and `dependencies_sync` exist per runbook step 11 (via repo MCP, currently down). Check `$G/sync-<n>.rc` afterwards.

**d. Graph regeneration** (checklist "Graph")
The checklist gives no shell command. Use repo MCP `graph` with `graph_policy=current`, and do NOT pass `file` (the report writer throws without a declared report name, `📊️reporting/🟦️.ts:13`). Watch with `leases_status`. Plan 30 to 90 min after the last freeze-class edit; any further edit restarts it. Pass ends with `graph: generation N published`.

**e. Prefreeze and freeze** (checklist order 1-2)
```
bash "$C/1105b-tools/prefreeze.sh" > "$G/prefreeze-pre.txt" 2>&1; echo rc=$?
printf 1 > "$G/freeze"; printf 0 > "$C/1105b-tools/slots.k"; printf 1 > "$C/1105b-tools/slots.wextra"
```

**f. Descriptor refresh** (checklist step 3; `canary/describe.sh` is the checklist's form; runbook step 26/27 is the older direct form)
```
( nohup "$C/1105b-tools/slot.sh" W-act-150 "$C/1105b-tools/canary/describe.sh" act-150 energy > "$G/W-act-150.txt" 2>&1 < /dev/null & )
cat "$G/act-150.rc"
( nohup "$C/1105b-tools/slot.sh" W-act-200 "$C/1105b-tools/canary/describe.sh" act-200 closure > "$G/W-act-200.txt" 2>&1 < /dev/null & )
cat "$G/act-200.rc"
git ls-files -m -- '*🧩️component/🔣️.json' '*🛂️.descriptor.semio'
```
Check: every closure json says `"appChannelVersion": 25`; demonstrator carries `activationEvents` and `capabilityRequests`. Then re-run `value/gate.sh <n>` (descriptor changes stale the markers) and wait for the graph pass.

**g. Canary, energy, port 6106** (checklist step 4)
```
( nohup "$C/1105b-tools/slot.sh" W-canary-act-2 "$C/1105b-tools/canary/activate.sh" canary-act-2 > "$G/canary-act-2.slot.txt" 2>&1 < /dev/null & )
cat "$G/canary-act-2.rc"
"$C/1105b-tools/canary/serve.sh" start
"$C/1105b-tools/canary/probe.sh" "http://127.0.0.1:6106/?plugin=energy"; cat "$G/canary-probe/run.rc"
"$C/1105b-tools/canary/serve.sh" stop; lsof -nP -iTCP:6106 -sTCP:LISTEN
```
Serve exit codes: 3 busy, 4 no receipt, 5 early exit, 6 not 200, 7 Vite race. Last `lsof` must be empty.

**h. Prefreeze go** (checklist step 5)
```
bash "$C/1105b-tools/prefreeze.sh" > "$G/prefreeze-go.txt" 2>&1
```

**i. activate-prototype** (checklist step 6; runbook 30)
```
( nohup sh -c "\"$C/1105b-tools/slot.sh\" W-act-160 env SEMIO_PROGRESS_DEADLINE_TRACE=1 SEMIO_LANGUAGE=en repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:activate-prototype; echo \$? > \"$G/W-act-160.rc\"" > "$G/W-act-160.txt" 2>&1 < /dev/null & )
```
Success: `W-act-160.rc` = 0, `$D/dist/♻️activation/prototype`, receipts under `.cache/b/p/m/browser/prototype/runtime/react/{generator,energy,fem3d}/activation/🔣️receipt.json`. Do not use `act.sh`.

**j. Serve on 6029** (checklist step 7)
```
"$C/1105b-tools/serve.sh" start; echo rc=$?
```
Exit 4 = missing receipts, 7 = Vite race. Pid file `$G/serve-6029.pid`.

**k. e2e (checklist step 8, then runbook 32-34)**
```
"$C/1105b-tools/e2eready/run.sh" --check; echo rc=$?
E2E_ACTIONS_ALL_LANGUAGES=1 "$C/1105b-tools/e2eready/run.sh" http://127.0.0.1:6029/ --panes generator,koordinator,aggregator,energie,aussuchen,bearbeiten,verfolgen,statik --languages en,de
( nohup sh -c "\"$C/1105b-tools/slot.sh\" W-e2e-160 env SEMIO_TICKET_DIR=\"$C\" repo run @semio-tech/mit-bestand-demonstrator:test-e2e; echo \$? > \"$G/W-e2e-160.rc\"" > "$G/W-e2e-160.txt" 2>&1 < /dev/null & )
```
Output of the browser verdict: `$G/e2eready/run.rc`. The 50 executions (25 cases x 2 languages) are the `test-e2e` run; it starts its own server (`serve-e2e`) and mutates the host store, so no other store writer runs beside it. Do not set `CI`. `@behaviour` (runbook 35) has never run and is optional triage.

**l. Release build** (runbook 36) and teardown (runbook 37)
```
( nohup sh -c "\"$C/1105b-tools/slot.sh\" W-build-160 env SEMIO_LANGUAGE=en repo run @semio-tech/mit-bestand-demonstrator:build; echo \$? > \"$G/W-build-160.rc\"" > "$G/W-build-160.txt" 2>&1 < /dev/null & )
"$C/1105b-tools/serve.sh" stop; printf 8 > "$C/1105b-tools/slots.k"; printf 2 > "$C/1105b-tools/slots.wextra"; mv "$G/freeze" "$G/freeze.done"
```
Needs network for map tiles.

No `repo` command is given in the checklist for the descriptor pass beyond `describe.sh`; no release command beyond runbook 36 exists in the tickets (`1107-audit-release.md:109` notes no release build log exists).
