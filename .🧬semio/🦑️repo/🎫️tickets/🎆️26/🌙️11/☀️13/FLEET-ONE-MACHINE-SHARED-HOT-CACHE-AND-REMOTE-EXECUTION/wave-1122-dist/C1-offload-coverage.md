# C1: Offload coverage audit (read-only)

Lane C1 of wave 1122-dist. No code edits, no git writes, no locks taken.

LIB = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`
RE = `LIB/🏗️build/🌐️remote-execution`
UC = `LIB/⚡️caching/🦀️cargo/🗃️unit-cache`

## 0. Correction on the input file

`/private/tmp/.../scratchpad/re-status.txt` is not `repo remote-execution status` output. It is a build progress log (`[script] ... build:source-admission`). The live status was taken fresh instead, saved to `/private/tmp/claude-501/-Users-ueli-Documents-semio/caf759d9-ccba-4796-9909-827125acd710/scratchpad/C1-re-status-live.txt`. A plain `curl` to the scheduler `/status` returns `Unauthorized`, `/v1/fleet/load` and `/v1/fleet/idle` also `401`. Only signed calls (`repo remote-execution status`) read the scheduler. The fleet cache `:6094/healthz` answers 200.

## 1. Where the offload decision lives (rustc wrapper, unit cache)

Flow for one rustc unit, in order (`UC/📜️script.ts`):
1. Local unit-cache hit `serve()` (script.ts ~line 110-150). A hit returns before any offload.
2. Fleet cache hit `obtainFleetUnitV1` (script.ts ~ line 190). A hit returns.
3. Per-key lock `acquireUnitClaimV1`, then `serve()` again.
4. **Offload attempt** `offloadUnitV1()` (script.ts:160-185), called at script.ts:194 `let remote=await offloadUnitV1();`. A `undefined` result means compile locally with `compile(true)`.

Eligibility, first checks in `offloadUnitV1` (script.ts:162):
- `mode==="never"` → no.
- `!unit.source.endsWith(".rs")` → no. Units with another source form never offload.
- `offloadCrateTypeRefusalV1(unit.crateTypes,unit.target)!==undefined` → no. Rule in `🏗️build/🌐️remote-execution/🚀️offload/📐️policy/🟦️.ts:15-22`:
  - Always allowed: `lib`, `rlib`, `proc-macro`, `bin`.
  - `cdylib` only when target is `wasm32-unknown-unknown` or `wasm32-wasip2` (`policy/🟦️.ts:9`, schema `wasm.targets`).
  - Refused: native `cdylib`, `dylib`, `staticlib`.
- These early refusals return `undefined` **with no event and no counter**. That is gap G3.

Gate `offloadGateV1` (`🚦️admission/🟦️.ts:112-125`, called at script.ts:167):
- `never` → no. `always` → yes (reason `always`, skips every other test).
- `!worthARoundTripV1` (recorded local cost under 2500 ms, `policy.limits.roundTripMilliseconds`) → no (`cheap`). Units with no cost record are offloaded.
- `door` (`SEMIO_OFFLOAD_PRESSURE=door`, set when the build door refused the whole build) → yes.
- `slotQueueMillisecondsV1 >= 1500` (`queueThresholdMilliseconds`) → yes (`slot-queue`).
- `sharedUnitAdmissionV1 == wait` (host-capacity verdict, `🚦️admission/🟦️.ts:50-60`) → yes (`host-wait`).
- Broker `wait` → yes (`broker-wait`).
- Otherwise local (`calm`).

So under the default `auto` mode, offload happens only under local pressure. Default mode is `auto`: schema `policy.mode.default` in `🚀️offload/🧬️schema/🔣️.json` (env `SEMIO_OFFLOAD`), read by `offloadModeV1` (`policy/🟦️.ts`).

Then `offloadRustcUnitV1` (`🚀️offload/🟦️.ts:47-120`):
- Toolchain identity from `rustc -vV` (`parseRustcIdentityV1`, `📜️action/🟦️.ts:28`). Missing → `toolchain-missing`.
- Endpoint: `resolveRemoteExecutionEndpointV1` (`🧭️endpoint`). Missing → `no-endpoint`.
- Cache endpoint `/healthz` probe (memo 15 s, `policy.limits.reachableMemoMilliseconds`).
- Decline memo 3 s (`declineMemoMilliseconds`) after `no-idle-machine` or `worker-policy`.
- `planOffloadV1` (`🚀️offload/🗺️paths/🟦️.ts:98`).
- Action built by `buildOffloadActionV1` (`📜️action/🟦️.ts`), platform by `offloadPlatformV1` (`📜️action/🟦️.ts:21-25`).
- Execute with `semio-offload=if-idle` (`policy.property.offloadValue`). The scheduler answers `DECLINED` if no other machine has headroom (`🗓️scheduler/🟦️.ts:114`). Local fallback follows.
- Outputs translated back and written into the real `--out-dir` (`🚀️offload/🟦️.ts:100-118`).

### Shipped inputs (`planOffloadV1`, `paths/🟦️.ts:98-170`)
- The root source plus files it reaches: `mod x;` files, `#[path]`, `include*!` (`reachedFilesV1`, `paths/🟦️.ts:25`). Fallback is a directory walk of the package dir (`walkFilesV1`).
- `Cargo.toml`.
- Extern `.rlib`/`.rmeta` by `--extern` path, plus `-L dependency=` directories through `dependencyClosureV1` (`🌳️inputs`). Limit 512 MiB (`limits.maximumInputBytes`).
- Everything in `OUT_DIR` (walked, `paths/🟦️.ts` near line 170-180).
- `.d` dependency files within the checkout.
- Environment: names matching `deniedNames`/`deniedPrefixes` are dropped (`policy.environment`, schema). **This includes every `SEMIO_*` name** (`deniedPrefixes`). Values containing the cwd decline.
- Not shipped: sysroot, toolchain, system libraries. The worker must have the same toolchain (`semio-toolchain` digest) and the same OS.

### Declines inside `planOffloadV1` (`scan`, `paths/🟦️.ts:74-97`)
- `-l` (any) → `not-portable`.
- `-L` other than `dependency=` (i.e. `native=`, `all=`) → `not-portable`.
- `--sysroot`, `-o`, `--emit-dep-info`, `--emit <k>=<v>`, `--target *.json` → `unmapped-path`.
- A source outside `CARGO_MANIFEST_DIR` or no `--remap-path-prefix` for the module dir → `unmapped-path`. The module remap fix `moduleRemapArgumentsV1` (`paths/🟦️.ts:62`) is wired at `UC/📜️script.ts:92-97`.

### Worker-side validator (`🧱️actions/🟦️.ts`)
- `validateRemoteExecutionRustcActionV1` (`🧱️actions/🟦️.ts:90+`) accepts only `@tool:rustc` with an allow-list of flags (`WITH_VALUE`, `BARE`, `flag()` at ~line 30-70).
- Rejects `-l`, `-o`, `--sysroot`, `-C link-arg` except install-name, `--max-memory`, `-zstack-size` (wasm only). Rejects `cdylib` unless wasm, and any `SEMIO_*` env.
- Worker action policy default is `"rustc"` (`🔨️modules/📚️library/🏗️build/🔣️policy.json:567`). `"any"` exists (`👷️worker/🟦️.ts:235, 357`) but is only set by `SEMIO_REMOTE_EXECUTION_WORKER_ACTIONS`.

### Worker-side placement (`🗓️scheduler/⚖️placement/🟦️.ts`)
- Required platform: `Arch`, `OSFamily`, `semio-toolchain` (= sha256 of `rustc -vV`, 16 hex), `semio-target` (= requester host triple), `semio-rustc-target-<t>=1` for a non-host target, `semio-offload-format=argv-1`, plus `semio-peak-mb` for wasm linked units (6144 MB). Control properties are stripped (`REMOTE_EXECUTION_CONTROL_PROPERTIES_V1`, `🟦️.ts:76`).
- `placementCompatibleV1` (`⚖️placement/🟦️.ts:71`) excludes the requester origin. A unit never runs on the machine that asked.
- Least loaded worker wins (`planDispatchV1`, `⚖️placement/🟦️.ts:80`).

### Which unit kinds actually offload (counts from this machine's log, see section 5)
Eligible by rule: `lib`, `rlib`, `proc-macro`, `bin` (includes build-script compile, `isBuildScriptUnitV1` `🏗️buildscript/🟦️.ts:17`), wasm `cdylib`. Build-script compiles are offloaded (35 seen). Native `cdylib`, `dylib`, `staticlib` never offload.

## 2. Force-offload switch

**It exists already**: `SEMIO_OFFLOAD=always` (`policy.mode.environment`, `offloadModeV1`). Under `always`:
- The gate returns `always` (`🚦️admission/🟦️.ts:113`).
- Every eligible unit (section 1) is offered to the fleet.

It is not enough for "fully distributable":
- **G2 (important).** Even under `always`, the platform still carries `semio-offload=if-idle` (`📜️action/🟦️.ts:22`). The scheduler answers `DECLINED` when no other machine has headroom (`🗓️scheduler/🟦️.ts:114`) and the unit compiles locally. There is no queue-and-wait mode. The requester also gives up after `timeouts.queueMilliseconds` = 8000 ms (`🔌️client/🟦️.ts:51`, `queue-timeout`).
  - Proposal: a third value `semio-offload=queue` (new `policy.property.offloadValue` variant, `SEMIO_OFFLOAD=always` maps to it). Scheduler: skip the decline branch for `queue`. Client: use a long queue limit for `always`.
  - Files: `🚀️offload/🧬️schema/🔣️.json` (`property`, `timeouts`), `📜️action/🟦️.ts:21-25`, `🗓️scheduler/🟦️.ts:114`, `🔌️client/🟦️.ts:27,51`.
- **G3.** Eligibility refusals (`script.ts:162`) are silent. Coverage can't be measured for `always`.
  - Proposal: emit `offload-declined` with reason `kind:<crate-type|source|mode>` before returning `undefined` at script.ts:162.

## 3. Component finals and non-rustc steps

**Not remote-executable today.** Only rustc units go through REAPI (`offloadRustcUnitV1`, the only `buildOffloadActionV1` caller).

- jco transpile and component passes: local `spawn` (`📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts:652, 897`). Stage cache is local (`work.cache.run`, `🔌️plugin/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts:124, 139, 165`).
- wasm-opt: `runCmdStatus` (`🔌️plugin/🌐️browser-bundle/🏗️materialization/🪶️wasm-opt/🟦️.ts:36`), local.
- Probes, native transpile, tsc, cmake: local (`🏃️process/🎛️owned-execution/🟦️.ts:115` spawn).
- Bazel is the only other REAPI client: `🔧️tools/🏗️bazel/🏃️execution/🟦️.ts` (`bazelExecutionFlagsV1` ~line 67-77 adds `--remote_executor`). It only applies to tasks that pass `bazelHermeticDecisionV1` (same file, ~line 100+: hermetic declaration, a command the runner interprets, declared outputs). Those tasks are not the component chain.
- Worker side is ready for generic actions (`actions:"any"`, `👷️worker/🟦️.ts:357`, the darwin-sandbox workers report `confinement: full`), but no client builds them.

Proposal (G7): an `offloadCommandV1` client next to `offloadRustcUnitV1`, with a closed allow-list of tools (`jco`, `wasm-opt`, `wasm-tools`, probes) identified by pinned digest (`🧰️tools` pins). Action validator: an `any`-policy variant limited to those digests and `confinement: full` workers. Files: new `🚀️offload/🧰️command/🟦️.ts`, `🧱️actions/🟦️.ts`, `🪶️wasm-opt/🟦️.ts`, `🧩️component-pass/🟦️.ts`.

Build-script RUN (G6): not a rustc invocation. Cargo executes the compiled `build-script-build` binary itself, so `RUSTC_WRAPPER` never sees it. There is no clean hook. Options:
- a. Cache the run result: key on the compiled binary digest + env + inputs, store stdout + `OUT_DIR` in the fleet cache, and offload that as a generic action (needs G7).
- b. Leave build-script runs local (they are cheap in the live log: the 35 offloaded build-script compiles are the expensive part).

## 4. Running a whole build on a specific worker (g1-m12)

**No command exists that runs a build on a chosen fleet machine.**
- `repo scheduler run` (catalogue `⚙️operations/🔣️catalog.json` ~line 1716) is the host-local fair queue, not fleet placement.
- `repo fleet status` reads only.
- `repo remote-execution workers --join <url> [--properties name=value]` (catalogue ~line 1817) registers a worker, it does not dispatch.

Options that work today:
1. Run on the machine itself. The replica copies the same tree within ~1 s, so: `ssh ueli-workstation-mac-mini-m4-16gb-256gb-07-lan.fritz.box 'cd /Users/ueli/Documents/semio && repo run <project>:<target>'`. Host name from the live status (origin g1-m12 ↔ that host). Builds there use its own local cache plus the fleet cache. Confirm the SSH alias with the user before use.
2. Pin via property (needs code, G15): add `semio-pin=<origin>` as a required platform property from an env `SEMIO_OFFLOAD_PIN` in `offloadPlatformV1` (`📜️action/🟦️.ts:21-25`), and start the target worker with `repo remote-execution workers --join http://192.168.178.70:8981 --origin g1-m12 --properties semio-pin=g1-m12`. Caveat: placement excludes the requester origin (`⚖️placement/🟦️.ts:71`), so a pin cannot target the requesting machine.

## 5. Live numbers

### Scheduler (`repo remote-execution status`, saved to `scratchpad/C1-re-status-live.txt`)
- Counters: queued 0, executing 6, completed 1594, cached 5, failed 24, cancelled 5, retried 34, deduplicated 2, refused 21, offloadRejected 10, queue wait p50 0 ms, p90 0 ms, max 752 ms.
- Endpoint: gRPC 192.168.178.70:8980 (api 2.1 and 2.3), cache answers true, worker API 8981. 11 workers, all `semio-toolchain 312cc5b2932fbb31`, all `darwin-sandbox`, confinement `full`, `network denied`.

| origin | headroom | running | refused | offloadedIn |
|---|---|---|---|---|
| g1-server | yes | 0 | 0 | 271 |
| g1-m01 | yes | 1 | 6 | 51 |
| g1-m02 | no | 0 | 1 | 2 |
| g1-m03 | no | 0 | 7 | 207 |
| g1-m06 | yes | 4 | 0 | 453 |
| g1-m07 | no | 0 | 1 | 13 |
| g1-m08 | yes | 1 | 2 | 22 |
| g1-m10 | no | 0 | 0 | 1 |
| g1-m11 | no | 0 | 0 | 1 |
| g1-m12 | no | 0 | 0 | 506 |
| g1-m13 | yes | 0 | 4 | 115 |

Note: `g1-m12` shows `headroom: false` at this moment and `g1-m02/03/07/10/11` also `false`. Reported as observed.

### This machine's offload log (`.cache/b/r/m/u/x/offload.jsonl`, 4707 lines, 2026-11-14 to 2026-11-22)
- Kinds: `offload-run` 184, `offload-declined` 2590, `offload-local-fallback` 1933.
- Declined/fallback reasons, cumulative: `unmapped-path` 2354 (1752 "no remap covers the module directory", 601 "source outside CARGO_MANIFEST_DIR"), `infrastructure` 1537 (806 cache endpoint unreachable, 369 memo, 324 content store timeout, 25 connect), `worker-policy` 203 (185 `CARGO_*` env values, 18 `--check-cfg`), `queue-timeout` 159, `incomplete-inputs` 151 (`can't find crate for semio_f…`), `no-idle-machine` 47, `unreadable-input` 39, `inputs-too-large` 21, `not-portable` 9 (8 are `-L native`), `path-dependent-output` 3.
- Last hour (20:31 cutoff, now is later): 11 declines (7 `not-portable`, 3 `unmapped-path`, 1 `inputs-too-large`). Last 15 min: no events. The big `unmapped-path` bucket is historical; the module-remap fix (`UC/📜️script.ts:92-97`) is in the current code.
- Offloaded crates (top): `build_script_build` 35, `e5probe` 7, `rustc_hash` 5, `bit_vec`, `cursor_icon`, `color_quant`, `libm`, `siphasher`, `unicode_script`, `itoa`, `heck` (2-4 each).
- Refused by name, not-portable: `objc_sys`, `semio_os_directory_client`, `semio_s_artifact_stdio_*` (several), `semio_s_artifact_puzzle_3d_precompute_brush`, `semio_s_artifact_procedural_3d_model`.

## 6. Gap list and fixes

| ID | Gap | Evidence | Fix (files) | Sev |
|---|---|---|---|---|
| G1 | Default `auto` offloads only under local pressure; a calm machine compiles everything locally | `UC/📜️script.ts:167`, `🚦️admission/🟦️.ts:112-125`, schema `mode.default` | Use `SEMIO_OFFLOAD=always` for distributable runs; make it the default for the `distributable` build profile | H |
| G2 | `always` still declines when no other machine is idle; 8 s queue timeout | `📜️action/🟦️.ts:22`, `🗓️scheduler/🟦️.ts:114`, `🔌️client/🟦️.ts:51` | Add `semio-offload=queue` (no decline) and a longer queue limit under `always` | H |
| G3 | Eligibility refusals are silent | `UC/📜️script.ts:162` | Emit `offload-declined` with `kind:` reason | M |
| G4 | Native `cdylib`, `dylib`, `staticlib` never offload | `🚀️offload/📐️policy/🟦️.ts:15-22`, validator crate-type check `🧱️actions/🟦️.ts` (crate type and wasm-link lines) | Extend crate types for native linked finals once `-l` is handled (G5) | H |
| G5 | `-l` and `-L native=` decline (`not-portable`) | `🚀️offload/🗺️paths/🟦️.ts:74-97` (scan), `🧱️actions/🟦️.ts` (`-l` not allowed) | Ship native search dirs as virtual dirs like deps; allow macOS system libs (`-liconv`, `-lobjc`, `-lSystem`) by allow-list; add `-l`/`-L native` to the validator | H |
| G6 | Build-script RUN is local (not rustc) | cargo runs the binary, no wrapper | Cache run result keyed by binary digest + env + inputs in the fleet cache; offload later via G7 | M |
| G7 | jco, wasm-opt, wasm-tools, probes, component passes are local only | `🧩️component-pass/🟦️.ts:652,897`, `🪶️wasm-opt/🟦️.ts:36`, `🚀️commands/🟦️.ts:124,139,165`, `🎛️owned-execution/🟦️.ts:115` | New `offloadCommandV1` client, tool allow-list pinned by digest; `any`-policy validator for those digests only, on `confinement: full` workers | H |
| G8 | wasm linker args limited to `--max-memory`, `-zstack-size` | `policy.wasm.linkArguments` in schema; `🧱️actions/🟦️.ts:48` | Add wasm-ld flags with a closed list, or move link to a G7 command | M |
| G9 | `SEMIO_*` env dropped on the worker: remote compile may differ from local | `deniedPrefixes` in schema; `paths/🟦️.ts` env loop (`deniedName`) | Carry `SEMIO_*` names that the compile reads (declared list), or fail closed when the unit's key includes one | M (correctness) |
| G10 | Dependency closures over 512 MiB decline | `inputs-too-large` 21; `limits.maximumInputBytes` (schema) | Raise the limit or ship only referenced rlibs | M |
| G11 | Worker refuses `CARGO_*` env values (185) and `--check-cfg` (18) | `worker-policy` in log; validator env branch and `flag()` `--cfg`/`--check-cfg` charset (`🧱️actions/🟦️.ts`) | Reproduce from one refused trace (`x/offload.jsonl` `trace`), then widen the validator to the exact safe set | H |
| G12 | `incomplete-inputs` (`can't find crate for semio_f…`) for 151 units | log detail; extern closure in `planOffloadV1` | Ship transitive rlib/rmeta for every `--extern` (`dependencyClosureV1`), add a test | H (correctness, falls back) |
| G13 | Cache endpoint unreachable (806 + 369 memo) | `🚀️offload/🟦️.ts:55-60` (healthz) | Historical; `healthz` answers 200 now. Keep memo, log the cause | L |
| G14 | No way to run a build on a chosen worker | catalogue has no such command; `placementCompatibleV1` excludes requester | Use `ssh <host>` + `repo run` (manual), or G15 | M |
| G15 | No pin option | `📜️action/🟦️.ts:21-25`; worker `--properties` (catalogue ~line 1817) | `SEMIO_OFFLOAD_PIN` → required `semio-pin=<origin>` property, worker `--properties semio-pin=<origin>` | M |

### Tests to add (per CLAUDE.md: test-driven, one language-agnostic test per feature)
- G2: scheduler test that `semio-offload=queue` is queued, not declined (`🗓️scheduler/🧪️tests`).
- G3: unit test that a `cdylib`-native unit emits `offload-declined kind:crate-type`.
- G5: a unit with `-l iconv` is planned with a mapped system dir and validates.
- G11: validator tests with the exact refused env value and `--check-cfg` text from the log.
- G12: a unit whose extern has a transitive rlib ships it.

## 7. Open items
- The `CARGO_*` env rejection (G11) is not reproduced yet. The log only has a prefix (`environment value of CARGO_PKG_DE…`). The validator code path that fires is not confirmed.
- The SSH alias for g1-m12 is not confirmed. The host name is in section 4.
- Nothing was run to offload a unit with `SEMIO_OFFLOAD=always` in this audit. Numbers are from the existing log and status, not a new build.
