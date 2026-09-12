# Audit A6 — `[DEBUG]` Trace Inventory For Close-Out Sweep

Read-only inventory (no source edits, no state-modifying git). Session boundary: waves B0-B50 
started 2026-09-11 14:00 CEST. Pre-session commit = `46c3cb9de0` (2026-09-11 12:39:02 +0200, the 
commit `git log --before='2026-09-11 14:10'` resolves to). Three auto-commits landed mid-session:

| Commit | Timestamp |
|---|---|
| `46c3cb9de0` | 2026-09-11 12:39:02 +0200 — **pre-session baseline** |
| `de93f84300` | 2026-09-12 00:35:59 +0200 |
| `989582baab` | 2026-09-12 01:07:58 +0200 |
| `521b618cee` | 2026-09-12 10:48:12 +0200 |
| *(working tree)* | now (2026-09-12, uncommitted) |

## 0. Methodology

1. **Inventory**: `rg -n --fixed-strings '[DEBUG]'` across the 7 named areas, then dropped any line whose 
   path contains `🧪️tests`/`🧪️test`/`.test.`/`.spec.`/`__tests__` (non-test filter). **419** production hits remained.
2. **Since (pre-session vs session-added)**: for every hit, fetched `git show 46c3cb9de0:<file>` and checked 
   whether the exact (whitespace-trimmed) line text exists verbatim in that pre-session blob. Present → 
   `pre-session` (untouched by rule 2 of the prior sweep's attribution method, `📓️2026-09-10-wave-Z-debug-sweep.md`). 
   Absent, and the file itself didn't exist at `46c3cb9de0` → `new-file(added-this-session)`. Absent otherwise 
   → `added-this-session`. **138** of 419 are session-added; **281** are pre-session.
3. **Which commit segment added it**: for each session-added file, ran `git diff <a> <b> -- <file>` across the four 
   boundaries above (plus `git diff 521b618cee -- <file>` for the open worktree) and matched each `+[DEBUG]` line 
   against the inventory. Cross-referenced segment windows against wave-report **mtimes** (`stat -f %Sm`) to give an 
   approximate wave-range per segment: seg0 ≈ B0-B26, seg1 ≈ B27, seg2 ≈ B28-B40, seg3 ≈ B41-B50 + everything still 
   uncommitted. **Caveat**: waves are not individually committed, so a single exact wave number per line is not 
   recoverable from git alone; exact-phrase matches against wave report text (§ per-item table, `wave-hint` column) 
   corroborate specific waves where the report happens to quote the literal, but many hits have no unique quote and 
   are labeled by segment/wave-range only. Do not over-trust a single-wave pin where the hint column shows several 
   candidates — those are files nearly every wave touched (ShellHost, plugin-bridge, PluginRuntime).
4. **Gated**: ±25-line window above each hit scanned for `runtime_diagnostics_enabled`, `runtimeDiagnosticsEnabled`, 
   `import.meta.vitest`, `cfg(test)`.
5. **Peer**: ±20-line window scanned for the literal ticket slug `PROCEDURAL-3D-END-TO-END` (the sibling ticket that, 
   per wave-Z, owns a large parallel diagnostics facility in these same files).
6. **Recommendation**: `KEEP-gated` if gated (regardless of peer marker — a peer's own gate is still a legitimate gate); 
   else `PEER` if a peer-ticket marker sits in the window; else `REMOVE`. Production files carry no test-law one-shot 
   exception (that exception in wave-Z applied only to `🧪️tests/` files, which are out of scope here by construction).

## 1. Per-area summary

| Area | Pre-session (untouched, out of scope) | Session-added: REMOVE | Session-added: KEEP-gated | Session-added: PEER | Total |
|---|---:|---:|---:|---:|---:|
| 1. Puzzle 3D guest (✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/) | 27 | 6 | 0 | 0 | 33 |
| 2. Plugin host (🔌️plugin/, ⚛️reactor, 📦️packages/🟦️typescript) | 34 | 12 | 14 | 4 | 64 |
| 3. Renderer host (📺️renderer/🧑‍🎨engine/🧱️elements/*) | 197 | 76 | 8 | 12 | 293 |
| 4. ui-react elements / 🎯️targets/⚛️react | 5 | 0 | 0 | 0 | 5 |
| 5. Actor package (🎭️actor/) | 16 | 5 | 0 | 1 | 22 |
| 7. Dev package (🧑‍💻dev/) | 2 | 0 | 0 | 0 | 2 |
| **TOTAL** | **281** | **99** | **22** | **17** | **419** |

Areas 6 (⏱️trace/🧵️job) and 7 (🧑‍💻dev) have **zero** session-added production `[DEBUG]` lines. Area 6 has zero 
production hits at all (only `🧪️tests/⏱️budget/🦀️.rs`, filtered as test code). Area 7's 2 hits are both pre-session.
Area 4 (ui-react) has 5 pre-session hits (in `🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts` and 
`🖱️ui/🧠️runtime/📦️packages/🦀️rust/📜️script.ts`) and zero session-added — untouched by this session, out of scope.

## 2. Session-added occurrences — full itemization (138)

Columns: `file:line` | literal (≤88 chars) | since-segment | gated | wave-hint (exact-phrase match against wave/audit 
reports, empty = no unique literal quote found) | recommendation.

### Area 1 — Puzzle 3D guest (✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/) (6 session-added)

| file:line | literal | since | gated | wave-hint | recommendation |
|---|---|---|---|---|---|
| `🎮️commands/📥️import-fixture/🦀️.rs`:11 | "[DEBUG] puzzle3d.import.ingress args={} payload_len={}", | seg0 (≤2026-09-12 00:36, earliest waves B0-B26ish) | ungated |  | REMOVE |
| `🎮️commands/📥️import-fixture/🦀️.rs`:40 | eprintln!("[DEBUG] puzzle3d.import.parsed objects={} before={}", fixture.objects.len(),  | seg0 (≤2026-09-12 00:36, earliest waves B0-B26ish) | ungated |  | REMOVE |
| `🎮️commands/🔖️set-selection-flag/🦀️.rs`:23 | eprintln!("[DEBUG] puzzle3d.flag.explicit entity={entity} flag={flag} value={value} ids= | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🎮️commands/🔖️set-selection-flag/🦀️.rs`:31 | "[DEBUG] puzzle3d.flag.selection entity={:?} flag={flag} value={value} objects={} vortic | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `✳️any/✏️editor/🦀️.rs`:3476 | eprintln!("[DEBUG] puzzle3d.import.apply ops={} after_objects={}", operations.len(), sce | seg0 (≤2026-09-12 00:36, earliest waves B0-B26ish) | ungated |  | REMOVE |
| `✳️any/✏️editor/🦀️.rs`:7693 | eprintln!("[DEBUG] puzzle3d.clipboard.copy marks={} resolved={} objects={}", marks.selec | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |

### Area 2 — Plugin host (🔌️plugin/, ⚛️reactor, 📦️packages/🟦️typescript) (30 session-added)

| file:line | literal | since | gated | wave-hint | recommendation |
|---|---|---|---|---|---|
| `🌐️browser-bundle/🏗️materialization/🟦️.ts`:81 | /** 🗣️ Preview2's default `cli.js` console.errors every `write()` token. One logical gue | seg3 (untracked file, never committed; uncommitted work) | ungated |  | REMOVE |
| `🌐️browser-bundle/🏗️materialization/🟦️.ts`:100 | else if (text.startsWith("[DEBUG]")) console.debug(text); | seg3 (untracked file, never committed; uncommitted work) | ungated |  | REMOVE |
| `🌐️browser-bundle/🏗️materialization/🟦️.ts`:375 | // \`[DEBUG]\` line the guest's Rust hot path prints was unreachable from a browser sess | seg3 (untracked file, never committed; uncommitted work) | ungated |  | PEER [peer-marker] |
| `🌐️browser-bundle/🏗️materialization/🟦️.ts`:393 | console.debug("[DEBUG] guest runtime diagnostics armed through wasi:cli/environment"); | seg3 (untracked file, never committed; uncommitted work) | ungated |  | PEER [peer-marker] |
| `🌐️browser-bundle/🏗️materialization/🟦️.ts`:396 | console.warn(\`[DEBUG] guest runtime diagnostics could not be armed: \${error}\`); | seg3 (untracked file, never committed; uncommitted work) | ungated |  | PEER [peer-marker] |
| `🌐️browser-bundle/🏗️materialization/🟦️.ts`:631 | if (retryableLifecycle) console.log(\`[DEBUG] shard worker: retryable lifecycle deadline | seg3 (untracked file, never committed; uncommitted work) | ungated |  | REMOVE |
| `🏪️store/📥️installation/🟦️.ts`:327 | console.log(`[DEBUG] extension store installed ${manifest.extensionId}@${manifest.versio | seg3 (untracked file, never committed; uncommitted work) | ungated |  | REMOVE |
| `🏪️store/📥️installation/🟦️.ts`:349 | console.log(`[DEBUG] extension store uninstalled ${extensionId}`); | seg3 (untracked file, never committed; uncommitted work) | ungated |  | REMOVE |
| `📇️registry/🗿️taxonomy-validation/🟦️.ts`:637 | console.log("[DEBUG] registry-rust-mounts capture=" + capture); | seg3 (untracked file, never committed; uncommitted work) | ungated |  | REMOVE |
| `📇️registry/🗿️taxonomy-validation/🟦️.ts`:684 | console.log("[DEBUG] registry-rust-mounts case=" + row.id + " rustc=" + status + " refer | seg3 (untracked file, never committed; uncommitted work) | ungated |  | REMOVE |
| `📇️registry/🗿️taxonomy-validation/🟦️.ts`:725 | console.log("[DEBUG] registry-plugin-root case=" + row.id + " reference=" + JSON.stringi | seg3 (untracked file, never committed; uncommitted work) | ungated |  | REMOVE |
| `🔌️plugin/🖥️host/🦀️.rs`:510 | /// SAME `[DEBUG]`/`[BUDGET]` lines the native in-process suites print, now observable f | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🔌️plugin/🖥️host/🦀️.rs`:1683 | /// second switch and no build-profile cfg — the guest's `[DEBUG]` sites stay compiled i | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | PEER [peer-marker] |
| `🔨️modules/🔌️plugin/🦀️.rs`:17610 | eprintln!("[DEBUG] actual typed ingress pre-admitted slot {VACANT} under 63 live foreign | seg0 (≤2026-09-12 00:36, earliest waves B0-B26ish) | ungated |  | REMOVE |
| `🔨️modules/🔌️plugin/🦀️.rs`:17649 | eprintln!("[DEBUG] actual {ACTIONS} storm actions each released their typed-operation sl | seg1 (00:36-01:08) | gated(cfg(test)) |  | KEEP-gated |
| `🔨️modules/🔌️plugin/🦀️.rs`:17679 | eprintln!("[DEBUG] actual settled, latest-wins-replaced and cancelled typed operations e | seg1 (00:36-01:08) | gated(cfg(test)) |  | KEEP-gated |
| `🔨️modules/🔌️plugin/🦀️.rs`:17816 | eprintln!("[DEBUG] actual one mutating typed operation kept its turn runnable through al | seg1 (00:36-01:08) | gated(cfg(test)) |  | KEEP-gated |
| `🔨️modules/🔌️plugin/🦀️.rs`:17844 | eprintln!("[DEBUG] actual a status-only host call finished its typed operation in {spent | seg1 (00:36-01:08) | gated(cfg(test)) |  | KEEP-gated |
| `🔨️modules/🔌️plugin/🦀️.rs`:17892 | "[DEBUG] actual the admitting call handed back the {lane:?} lane ({body:?}), {} completi | seg1 (00:36-01:08) | ungated |  | REMOVE |
| `🔨️modules/🔌️plugin/🦀️.rs`:18462 | eprintln!("[DEBUG] recursive replacement diagnostic: cancellation requested for operatio | seg3 (10:48-now, uncommitted; waves ~B41-B50) | gated(cfg(test)) |  | KEEP-gated |
| `🔨️modules/🔌️plugin/🦀️.rs`:18531 | "[DEBUG] recursive replacement diagnostic: member open rejected {diagnostic:?} at ordina | seg3 (10:48-now, uncommitted; waves ~B41-B50) | gated(cfg(test)) |  | KEEP-gated |
| `🔨️modules/🔌️plugin/🦀️.rs`:18550 | eprintln!("[DEBUG] recursive replacement diagnostic: opened member identity differed at  | seg3 (10:48-now, uncommitted; waves ~B41-B50) | gated(cfg(test)) |  | KEEP-gated |
| `🔨️modules/🔌️plugin/🦀️.rs`:18627 | eprintln!("[DEBUG] recursive replacement diagnostic: member factory refused one admitted | seg3 (10:48-now, uncommitted; waves ~B41-B50) | gated(cfg(test)) |  | KEEP-gated |
| `🔨️modules/🔌️plugin/🦀️.rs`:18682 | eprintln!("[DEBUG] recursive replacement diagnostic: closure completed with {members} me | seg3 (10:48-now, uncommitted; waves ~B41-B50) | gated(cfg(test)) |  | KEEP-gated |
| `🔨️modules/🔌️plugin/🦀️.rs`:18690 | eprintln!("[DEBUG] recursive replacement diagnostic: closure rejected {diagnostic:?}"); | seg3 (10:48-now, uncommitted; waves ~B41-B50) | gated(cfg(test)) |  | KEEP-gated |
| `🔨️modules/🔌️plugin/🦀️.rs`:21136 | "[DEBUG] recursive replacement diagnostic: publication guard rejected closing={closing}  | seg3 (10:48-now, uncommitted; waves ~B41-B50) | gated(cfg(test)) |  | KEEP-gated |
| `🔨️modules/🔌️plugin/🦀️.rs`:21152 | eprintln!("[DEBUG] recursive replacement diagnostic: app publication authority rejected  | seg3 (10:48-now, uncommitted; waves ~B41-B50) | gated(cfg(test)) |  | KEEP-gated |
| `🔨️modules/🔌️plugin/🦀️.rs`:23100 | crate::plugin_runtime::debug_runtime_line(format_args!("[DEBUG] interaction-store snapsh | seg0 (≤2026-09-12 00:36, earliest waves B0-B26ish) | ungated |  | REMOVE |
| `🔨️modules/🔌️plugin/🦀️.rs`:34116 | /// runtime's `[DEBUG]` diagnostics already use — no wire, no protocol frame, and nothin | seg1 (00:36-01:08) | gated(cfg(test)) |  | KEEP-gated |
| `🔨️modules/🔌️plugin/🦀️.rs`:34135 | eprintln!("[DEBUG] typed-operation slots instance={instance} live={live}/{} peak={peak}" | seg1 (00:36-01:08) | gated(runtime_diagnostics_enabled) |  | KEEP-gated |

### Area 3 — Renderer host (📺️renderer/🧑‍🎨engine/🧱️elements/*) (96 session-added)

| file:line | literal | since | gated | wave-hint | recommendation |
|---|---|---|---|---|---|
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:198 | onActorTrap: (actorId, message) => console.error(`[DEBUG] wgpu plugin-bridge: actor ${ac | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:200 | console.error(`[DEBUG] wgpu plugin-bridge: shard ${shardIndex} lost, restoring actors: $ | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:508 | if (stashed > 0) console.log(`[DEBUG] wgpu-bridge lifecycle deferred effects instance=${ | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:580 | if (elapsed > 1000) console.log(`[DEBUG] boot-phase ${name} ${elapsed.toFixed(0)} ms${ex | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:741 | for (const fault of scan.faults) console.error(`[DEBUG] wgpu-bridge typed-operation faul | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:770 | if (this.#pages > 0) console.log(`[DEBUG] wgpu-bridge typed-operation ${phase} instance= | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:886 | if (!actorId \|\| closingInstances.has(instanceId)) throw new Error(`[DEBUG] program ${p | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:891 | if (!client) throw new Error(`[DEBUG] program ${pluginId}: no channel for instance ${ins | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:934 | console.log(`[DEBUG] wgpu-bridge renderSurface surface=${surfaceId} turn=${opportunity}  | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:1000 | if (terminal === "fault") throw new Error(`[DEBUG] plugin ${pluginId}: command ingress f | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:1001 | if (terminal === "backpressure") throw new Error(`[DEBUG] plugin ${pluginId}: command in | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:1006 | if (terminal !== "command-complete") throw new Error(`[DEBUG] plugin ${pluginId}: comman | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:1022 | if (leftoverFriendly.length) console.log(`[DEBUG] wgpu-bridge effects leftover ${leftove | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:1062 | if (leftover.length > WGPU_TYPED_OPERATION_EFFECT_CAPACITY) throw new Error(`[DEBUG] wgp | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:1070 | if (outcome.stopped === "budget") console.warn(`[DEBUG] wgpu-bridge typed-operation drai | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:1117 | console.log("[DEBUG] wgpu-bridge extension completion submitted", { instanceId, req: Str | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:1207 | console.log("[DEBUG] wgpu-bridge extension request answered", { pluginId, capability, re | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:1213 | console.log("[DEBUG] wgpu-bridge invokeExtension dispatch", { pluginId, instanceId, exte | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:1233 | console.warn("[DEBUG] wgpu-bridge invokeExtension faulted", { extensionId, capability, i | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:1254 | console.log("[DEBUG] contributions push", { plugin: pluginId, app: appId, skipped: "empt | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:1260 | console.log("[DEBUG] contributions push", { plugin: pluginId, app: appId, active: true,  | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:1262 | throw new Error(`[DEBUG] contributions pack ingress ${ingress.ingressPages} pages exceed | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:1264 | console.log("[DEBUG] contributions slim view", JSON.stringify({ keys: Object.keys(slimVi | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:1266 | console.log("[DEBUG] contributions installed", { plugin: pluginId, app: appId, effects:  | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:1273 | console.log("[DEBUG] contributions rearm", { plugin: pluginId, action: dispatch.action,  | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:1276 | console.warn("[DEBUG] contributions rearm failed", dispatch.action, error instanceof Err | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:1298 | console.log(`[DEBUG] wgpu-bridge createApp open start plugin=${pluginId} instance=${inst | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧊️wgpu/🐚️plugin-bridge/🟦️.ts`:1315 | console.log(`[DEBUG] wgpu-bridge createApp open leave plugin=${pluginId} instance=${inst | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧱️elements/🌐️World3dHost/🟦️.tsx`:5026 | console.log("[DEBUG] puzzle3d.brushPreview.bind", { published: published?.length ?? 0, s | seg1 (00:36-01:08) | ungated |  | REMOVE |
| `🧱️elements/🌐️World3dHost/🟦️.tsx`:5664 | console.warn(`[DEBUG] fillBuildTick hop armed utility=${activeUtility} tool=${leftoverAr | seg1 (00:36-01:08) | ungated |  | REMOVE |
| `🧱️elements/🌐️World3dHost/🟦️.tsx`:6058 | console.info("[DEBUG] gumball pose delta skipped", { | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🎯️targets/🧊️wgpu/🦀️.rs`:1349 | /// 🌍️ `[DEBUG] ` trace of what the World3d pass this step just pushed actually carries  | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🎯️targets/🧊️wgpu/🦀️.rs`:1384 | debug_log(&format!("[DEBUG] world3d surface={} pane={:?} bounds={}x{} {geometry} {payloa | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🏛️ShellHost/🔀️surface-switch/🟦️.ts`:154 | return `[DEBUG] ${drop.code} dropped ${drop.what} for ${drop.instance}${drop.detail ===  | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:1752 | console.warn("[DEBUG] extension invocation refused", JSON.stringify({ extensionId, capab | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | PEER [peer-marker] |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:1985 | console.warn("[DEBUG] leftover forceReload", JSON.stringify({ keys })); | seg1 (00:36-01:08) | ungated |  | REMOVE |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:2034 | console.warn("[DEBUG] leftover InteractionView", JSON.stringify({ selectedIds: overlay.i | seg1 (00:36-01:08) | ungated |  | REMOVE |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:3532 | console.warn(`[DEBUG] ShellHost: ${ours.pluginId}#${ours.instanceId} died with shard ${s | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | PEER [peer-marker] |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:3534 | console.error(`[DEBUG] ShellHost: could not re-establish ${ours.pluginId} after a worker | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | PEER [peer-marker] |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:3693 | console.warn(`[DEBUG] hot-swap ${pluginId} retained its committed handle; predecessor re | seg1 (00:36-01:08) | ungated |  | REMOVE |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:4508 | * with zero `[DEBUG] contributions …` lines (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).  | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | PEER [peer-marker] |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:4529 | console.error("[DEBUG] contributions scoped from published examples", JSON.stringify({ p | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:4533 | console.error("[DEBUG] contributions push skipped unresolved document operators", JSON.s | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:4539 | if (scopedContributionsJson === "[]") console.error("[DEBUG] contributions push refused  | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:4540 | else console.error("[DEBUG] contributions scoped pack", JSON.stringify({ chars: scopedCo | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:4553 | console.log("[DEBUG] contributions push", JSON.stringify({ plugin: pluginEntry.handle.pl | seg2 (01:08-10:48, waves ~B27-B40) | gated(runtimeDiagnosticsEnabled) |  | KEEP-gated |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:4566 | console.error("[DEBUG] setContributions deferred effects", JSON.stringify({ plugin: plug | seg2 (01:08-10:48, waves ~B27-B40) | gated(runtimeDiagnosticsEnabled) |  | KEEP-gated [peer-marker] |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:4625 | void applyHostEffects(deferredEffects, target, { kind: "full" }, owner).catch((error) => | seg0 (≤2026-09-12 00:36, earliest waves B0-B26ish)/seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:4628 | if (outcome.status === "installed" \|\| outcome.status === "failed") console.error("[DEB | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:4785 | if (runtimeDiagnosticsEnabled()) console.warn("[DEBUG] refreshUi sections", JSON.stringi | seg2 (01:08-10:48, waves ~B27-B40) | gated(runtimeDiagnosticsEnabled) |  | KEEP-gated [peer-marker] |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:4909 | if (owedEffects) void applyHostEffectsRef.current(owedEffects.effects, owedEffects.sessi | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | PEER [peer-marker] |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:4913 | if (runtimeDiagnosticsEnabled()) console.warn("[DEBUG] refreshUi lane", JSON.stringify({ | seg2 (01:08-10:48, waves ~B27-B40) | gated(runtimeDiagnosticsEnabled) |  | KEEP-gated [peer-marker] |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:5123 | if (!quiet.settled) console.warn(`[DEBUG] surface-switch draining ${retiring.pluginId}#$ | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:5152 | trace: (step, detail) => console.warn(`[DEBUG] surface-switch ${step} ${detail}`), | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:5155 | (retired, retireError) => console.warn(`[DEBUG] switchToPluginApp: predecessor ${retired | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:5157 | console.warn(`[DEBUG] surface-switch outcome ${outcome.status} ${appId} pending=${outcom | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:5595 | console.error("[DEBUG] invokeExtension requester missing", JSON.stringify({ requested: b | seg0 (≤2026-09-12 00:36, earliest waves B0-B26ish) | ungated |  | REMOVE |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:5709 | if (runtimeDiagnosticsEnabled()) console.warn("[DEBUG] applyHostEffects refresh", JSON.s | seg2 (01:08-10:48, waves ~B27-B40) | gated(runtimeDiagnosticsEnabled) |  | KEEP-gated |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:5712 | if (runtimeDiagnosticsEnabled()) console.warn("[DEBUG] applyHostEffects skipped refresh: | seg2 (01:08-10:48, waves ~B27-B40) | gated(runtimeDiagnosticsEnabled) |  | KEEP-gated |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:5745 | if (runtimeDiagnosticsEnabled()) console.warn("[DEBUG] completion apply", JSON.stringify | seg1 (00:36-01:08) | gated(runtimeDiagnosticsEnabled) |  | KEEP-gated |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:5747 | void applyHostEffects(completion.requestedEffects, target, refresh, owner).catch((error) | seg1 (00:36-01:08) | gated(runtimeDiagnosticsEnabled) |  | KEEP-gated |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:6166 | console.warn("[DEBUG] extension requests aborted by surface cancel", JSON.stringify({ ac | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | PEER [peer-marker] |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:6308 | console.warn(`[DEBUG] setActiveTool hop leftover tool=${next ?? ""} utility=${next === " | seg1 (00:36-01:08) | ungated |  | REMOVE |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:7409 | useCallback(() => void switchToSessionRole("editor").catch((switchError) => console.erro | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:7416 | useCallback(() => void switchToSessionRole("viewer").catch((switchError) => console.erro | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:8948 | onClick={() => void switchToSessionRole(role).catch((switchError) => console.error(`[DEB | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:9546 | console.warn("[DEBUG] leftover Inspection refresh", JSON.stringify({ epoch: leftoverInsp | seg1 (00:36-01:08) | ungated |  | REMOVE |
| `🧱️elements/🏛️ShellHost/🟦️.tsx`:9597 | console.warn("[DEBUG] leftover brushPreview refresh", JSON.stringify({ hover: liveHover, | seg1 (00:36-01:08) | ungated |  | REMOVE |
| `🧫️fixtures/🔀️surface-switch/🔣️.json`:966 | "text": "[DEBUG] shell.surface-switch.sealed-instance dropped action for procedural#2 (s | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🧫️fixtures/🔀️surface-switch/🔣️.json`:972 | "text": "[DEBUG] shell.surface-switch.sealed-instance dropped host effects for procedura | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🎯️targets/🧊️wgpu/🦀️.rs`:2407 | /// 🐛️ Last `[DEBUG] ` dock-plan line, so the trace prints on CHANGE rather than once pe | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | PEER [peer-marker] |
| `🎯️targets/🧊️wgpu/🦀️.rs`:3153 | Self::debug_log(&format!("[DEBUG] wgpu-shell surface fault surface={surface_id} body={bo | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🎯️targets/🧊️wgpu/🦀️.rs`:3403 | "[DEBUG] setContributions deferred effects {}", | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🎯️targets/🧊️wgpu/🦀️.rs`:3408 | Err(error) => Self::debug_log(&format!("[DEBUG] setContributions command failed {} {erro | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🎯️targets/🧊️wgpu/🦀️.rs`:4198 | eprintln!("[DEBUG] wgpu shell recursive document archive decode failed: {error}"); | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🎯️targets/🧊️wgpu/🦀️.rs`:4204 | Err(error) => eprintln!("[DEBUG] wgpu shell load_app_document_archive failed: {error}"), | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🎯️targets/🧊️wgpu/🦀️.rs`:4959 | Self::debug_log(&format!("[DEBUG] wgpu-shell command {} settled effects={} mutations={}" | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🎯️targets/🧊️wgpu/🦀️.rs`:6780 | Self::debug_log(&format!("[DEBUG] wgpu-shell deferred action {id} failed: {error}")); | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🎯️targets/🧊️wgpu/🦀️.rs`:6789 | Self::debug_log(&format!("[DEBUG] wgpu-shell deferred chain exhausted {SHELL_DEFERRED_CH | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | PEER [peer-marker] |
| `🎯️targets/🧊️wgpu/🦀️.rs`:6806 | Self::debug_log(&format!("[DEBUG] wgpu-shell invokeExtension dispatch extension={extensi | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | PEER [peer-marker] |
| `🎯️targets/🧊️wgpu/🦀️.rs`:6809 | Self::debug_log(&format!("[DEBUG] wgpu-shell invokeExtension answered req={req} extensio | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | PEER [peer-marker] |
| `🎯️targets/🧊️wgpu/🦀️.rs`:6814 | Self::debug_log(&format!("[DEBUG] wgpu-shell invokeExtension mutations req={req} failed: | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | PEER [peer-marker] |
| `🎯️targets/🧊️wgpu/🦀️.rs`:6817 | Err(error) => Self::debug_log(&format!("[DEBUG] wgpu-shell invokeExtension failed req={r | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | PEER [peer-marker] |
| `🎯️targets/🧊️wgpu/🦀️.rs`:10908 | Self::debug_log(&format!("[DEBUG] wgpu-shell dock plan canvas={}x{} windows={} {plan}",  | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧱️elements/🔌️PluginRuntime/🟦️.tsx`:413 | console.error(`[DEBUG] PluginRuntime: ${PLUGIN_ACTOR_INSTANCE_LOST_FAULT} — ${lost.map(( | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧱️elements/🔌️PluginRuntime/🟦️.tsx`:418 | console.error("[DEBUG] PluginRuntime: an instance-lost listener threw", error); | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧱️elements/🔌️PluginRuntime/🟦️.tsx`:436 | console.error(`[DEBUG] PluginRuntime: releasing lost instance ${record.pluginId}#${recor | seg3 (10:48-now, uncommitted; waves ~B41-B50) | ungated |  | REMOVE |
| `🧱️elements/🔌️PluginRuntime/🟦️.tsx`:1048 | console.warn(`[DEBUG] wireEffectToFriendly: send-message to "${wireSendMessageTargetTag( | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🧱️elements/🕸️NodeGraph/🟦️.tsx`:2297 | * every invalidation (`[DEBUG] dag draw lod=detail zoom=1.784` repeating) while `renderF | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🧱️elements/🕸️NodeGraph/🟦️.tsx`:2353 | console.warn("[DEBUG] node-graph canvas unpresentable surface=%s retiring flow surface=% | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🧱️elements/🕸️NodeGraph/🟦️.tsx`:2442 | console.log("[DEBUG] node-graph host unmount surface=%s", surfaceId); | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🧱️elements/🕸️NodeGraph/🟦️.tsx`:2486 | console.log("[DEBUG] node-graph fit on open surface=%s %s", surfaceId, JSON.stringify(op | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🧱️elements/🕸️NodeGraph/🟦️.tsx`:2547 | console.log("[DEBUG] node-graph refit after graph change surface=%s %s", surfaceId, JSON | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🎯️targets/🧊️wgpu/🦀️.rs`:1218 | document_debug_log(&format!("[DEBUG] ui-doc reconcile fault window={window_id} fault={fa | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🎯️targets/🧊️wgpu/🦀️.rs`:1248 | document_debug_log(&format!("[DEBUG] ui-doc paint fault window={window_id} phase={:?} no | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |
| `🧱️elements/🗣️Interpreter/🟦️.tsx`:1437 | console.log("[DEBUG] puzzle3d.brushPreview.assemble", { recordKey: record.key, spinePrev | seg1 (00:36-01:08) | ungated |  | REMOVE |

### Area 5 — Actor package (🎭️actor/) (6 session-added)

| file:line | literal | since | gated | wave-hint | recommendation |
|---|---|---|---|---|---|
| `🎭️actor/🖼️wire-turn/🟦️.ts`:39 | throw new Error(`[DEBUG] coerceWireBytes: unsupported payload ${JSON.stringify(raw)?.sli | seg3 (untracked file, never committed; uncommitted work) | ungated |  | REMOVE |
| `🎭️actor/🖼️wire-turn/🟦️.ts`:483 | * branches on; an effect kind with no case here degrades to an honest `[DEBUG]`-logged d | seg3 (untracked file, never committed; uncommitted work) | ungated |  | REMOVE |
| `🎭️actor/🖼️wire-turn/🟦️.ts`:552 | console.warn(`[DEBUG] wireEffectToFriendly: send-message to "${wireSendMessageTargetTag( | seg3 (untracked file, never committed; uncommitted work) | ungated |  | REMOVE |
| `🎭️actor/🖼️wire-turn/🟦️.ts`:556 | console.warn(`[DEBUG] wireEffectToFriendly: unmapped effect "${effect.tag}" dropped — un | seg3 (untracked file, never committed; uncommitted work) | ungated |  | REMOVE |
| `🧫️fixtures/📨️effect-wire-routes/🔣️.json`:2 | "why": "Who owns a `send-message` effect once a turn hands it back. Ticket 26/09/09/PROC | seg3 (untracked file, never committed; uncommitted work) | ungated |  | PEER [peer-marker] |
| `🎭️actor/🧵️shard-runtime/🟦️.ts`:50 | * `new Worker(...)` in the repo goes through this so the guest's own `[DEBUG]` trace sit | seg2 (01:08-10:48, waves ~B27-B40) | ungated |  | REMOVE |

## 3. Pre-session occurrences (281) — compact, out of scope

Verbatim at `46c3cb9de0` per attribution rule 2 (`📓️2026-09-10-wave-Z-debug-sweep.md` §0): untouched, whether 
peer-owned or this ticket's own earlier work already adjudicated by that sweep. Listed by file with counts and 
gating breakdown only (full line-by-line detail was already produced by the wave-Z sweep for the areas it covered; 
re-itemizing 281 pre-existing lines here would just restate that audit).

### Area 1 — Puzzle 3D guest (✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/) (27 pre-session)

| file | count | gated breakdown |
|---|---:|---|
| `🪟️windows/🧊️main/🦀️.rs` | 8 | ungated:8 |
| `✳️any/✏️editor/🦀️.rs` | 8 | ungated:8 |
| `🎮️commands/🔓️open-vortex-suggestions/🦀️.rs` | 4 | ungated:4 |
| `🎮️commands/⏱️suggestions-tick/🦀️.rs` | 4 | ungated:4 |
| `✏️editor/⏳️precompute/🦀️.rs` | 2 | ungated:2 |
| `🎮️commands/🗂️open-import-fixture/🦀️.rs` | 1 | ungated:1 |

### Area 2 — Plugin host (🔌️plugin/, ⚛️reactor, 📦️packages/🟦️typescript) (34 pre-session)

| file | count | gated breakdown |
|---|---:|---|
| `🔨️modules/🔌️plugin/🦀️.rs` | 19 | ungated:15, gated(runtime_diagnostics_enabled):3, gated(cfg(test)):1 |
| `⚛️reactor/🔄️turn/🦀️.rs` | 6 | ungated:2, gated(runtime_diagnostics_enabled):4 |
| `🧫️fixtures/🌐️wasi-activation/🔣️.json` | 2 | ungated:2 |
| `🌐️browser-bundle/🌐️wasi/🟦️.ts` | 2 | ungated:2 |
| `📦️packages/🦀️rust/📜️script.ts` | 1 | ungated:1 |
| `🔌️plugin/🧵️retained-command/🦀️.rs` | 1 | ungated:1 |
| `🔌️plugin/🚪️lifetime/🦀️.rs` | 1 | gated(cfg(test)):1 |
| `🔌️plugin/⚛️reactor/🦀️.rs` | 1 | gated(runtime_diagnostics_enabled):1 |
| `🧵️shard/🚚️process-transport/🦀️.rs` | 1 | ungated:1 |

### Area 3 — Renderer host (📺️renderer/🧑‍🎨engine/🧱️elements/*) (197 pre-session)

| file | count | gated breakdown |
|---|---:|---|
| `🧱️elements/🏛️ShellHost/🟦️.tsx` | 106 | ungated:104, gated(runtimeDiagnosticsEnabled):2 |
| `🎯️targets/🧊️wgpu/🦀️.rs` | 36 | ungated:36 |
| `🧱️elements/🕸️NodeGraph/🟦️.tsx` | 12 | ungated:12 |
| `🧱️elements/🌐️World3dHost/🟦️.tsx` | 10 | ungated:10 |
| `🧱️elements/🔌️PluginRuntime/🟦️.tsx` | 7 | ungated:7 |
| `🎯️targets/🧊️wgpu/🦀️.rs` | 6 | ungated:6 |
| `🧊️wgpu/⌨️native-entrypoint/🦀️.rs` | 4 | ungated:4 |
| `👥️presence-scope/🌐️browser/🟦️.tsx` | 4 | ungated:4 |
| `🧊️wgpu/🎞️frame-worker/🟦️.ts` | 3 | ungated:3 |
| `🧱️elements/🖌️Paint2dHost/🟦️.tsx` | 2 | ungated:2 |
| `🧱️elements/🐚️Shell/🟦️.tsx` | 2 | ungated:2 |
| `🧊️wgpu/🌐️browser-worker/🦀️.rs` | 2 | ungated:2 |
| `🎯️targets/🧊️wgpu/🦀️.rs` | 1 | ungated:1 |
| `🧱️elements/✏️TextEditor/🟦️.tsx` | 1 | ungated:1 |
| `🧊️wgpu/⏱️turn-budget/🟦️.ts` | 1 | ungated:1 |

### Area 4 — ui-react elements / 🎯️targets/⚛️react (5 pre-session)

| file | count | gated breakdown |
|---|---:|---|
| `📦️packages/🦀️rust/📜️script.ts` | 3 | ungated:3 |
| `📦️packages/🦀️rust/📜️script.ts` | 2 | ungated:2 |

### Area 5 — Actor package (🎭️actor/) (16 pre-session)

| file | count | gated breakdown |
|---|---:|---|
| `🎭️actor/📮️shard-client/🟦️.ts` | 16 | ungated:16 |

### Area 7 — Dev package (🧑‍💻dev/) (2 pre-session)

| file | count | gated breakdown |
|---|---:|---|
| `📦️packages/🟦️typescript/📜️script.ts` | 1 | ungated:1 |
| `🔨️modules/🧑‍💻dev/🟦️.ts` | 1 | gated(import.meta.vitest):1 |

## 4. Hot-path flood check (per-continuation / per-hover / per-render, no `[DEBUG]` prefix required)

Source: `T/🗑️generated/probe-2026-09-12T16-24-25.md`, `## console tail` section (lines 1148-2350, one browser
probe run, ~1150s of interaction). 1197 lines in the tail carry `[DEBUG]`. Top families by normalized prefix:

| Rank | Count | Prefix | Emitter found in current source? | Note |
|---:|---:|---|---|---|
| 1 | 148 | `warning: [DEBUG] command ingress settled status` | **No** — `rg --fixed-strings '[DEBUG] command ingress lane'` / `'[DEBUG] performInvocation'` return **zero** hits repo-wide (excluding `⚡️cache`/`node_modules`/`target`/`🤖️generated`/`🗑️generated`) | Already removed/renamed since this 16:24 capture — **not a live flood**. |
| 2 | 147 | `warning: [DEBUG] command ingress lane` | No (same check) | Same — stale capture, not reproducible against current tree. |
| 3 | 109 | `warning: [DEBUG] performInvocation settled` | No | Same. |
| 4 | 108 | `warning: [DEBUG] performInvocation` | No | Same. |
| 5 | **100** | `log: [DEBUG] puzzle3d.brushPreview.assemble` | **Yes** — `🧱️elements/🗣️Interpreter/🟦️.tsx:1437`, ungated `console.log` with a 7-key object (record key, 3 lengths, preview bytes, a boolean, a lane-name array) | **Live flood.** Fires once per brush-preview assemble; 100 times in one probe run is per-render/per-hover cadence. REMOVE (matches this session's added-this-session bucket, area 3). |
| 6 | 74 | `log: [DEBUG] puzzle3d.brushPreview.bind` | **Yes** — `🧱️elements/🌐️World3dHost/🟦️.tsx:5026`, ungated `console.log` | **Live flood**, same family as #5 — bind+assemble = 174 of 1197 tail lines (14.5%) from two ungated per-hover taps. REMOVE. |
| 7 | 62 | `debug: [DEBUG] puzzle3d.utility.publish action` | Not checked against source directly (guest `debug_log`/eprintln pattern, area 1/2) | Fires once per action dispatch that touches a utility map — not obviously per-frame; lower priority than #5/#6. |
| 8 | 45 | `warning: [DEBUG] leftover forceReload` | **Yes** — `🏛️ShellHost/🟦️.tsx:1985`, ungated `console.warn`, session-added (§2, REMOVE) | Fires per forced reload of the leftover-key set; moderate, not per-frame. |
| 9 | 38×4 | `spawn-job routed` / `job done` / `job-completed leftover job` / `job-completed leftover frame` | Not traced to a specific file in the 7 areas (likely job/reactor plumbing) | Once per completed job; scales with job count in the probe, not with frame/hover count. |
| 10 | 33 | `warning: [DEBUG] leftover InteractionView` | **Yes** — `🏛️ShellHost/🟦️.tsx:2034`, ungated `console.warn`, session-added (§2, REMOVE) | Per selection/hover-target change. |

**Verdict**: the two largest families (#1-#4, 512 of 1197 lines, 43% of the tail) are **stale** — the literal text
no longer exists anywhere in the working tree, so whatever emitted them during the 16:24 capture has already
been removed or reworded by a later wave. The **live, currently-reproducible flood** is
`puzzle3d.brushPreview.{bind,assemble}` (#5+#6, 174 lines, 14.5% of the tail) — two ungated `console.log`s with
multi-key object payloads on what the probe's own per-hover/per-hover-storm steps exercise. Neither is behind
`runtimeDiagnosticsEnabled()`. Recommend REMOVE for both (already flagged in §2 area 3, `🌐️World3dHost/🟦️.tsx:5026`
and `🗣️Interpreter/🟦️.tsx:1437`) — this is the same class of flood B4/B14 already cut once (main-thread-refresh
tap); it recurred via a different pair of call sites.

## 5. Ticket-owned temporary files under `T/`

Per CLAUDE.md: scripts/configs are **inputs**, kept; markdown reports/audits are kept; only compiler/tool-output
files (logs, jsonl, generated probe artifacts) get deleted, and only from `🗑️generated` — which this audit does
not touch.

| Kind | Glob | Count | Disposition |
|---|---|---:|---|
| Probe helper scripts | `T/🔍️*.ts`, `T/🔍️*.py` | 33 | **KEEP** (input scripts, per-wave browser/verification probes) |
| Repair/build scripts | `T/🔨️*.sh`, `T/🔨️*.py`, `T/🔨️*.ts` | 11 | **KEEP** (input scripts) |
| Ticket notes/audits/reports (`📓️*.md`, `📋️*.md`) | `T/📓️*.md`, `T/📋️*.md` | ~150 | **KEEP** (this audit adds one more) |
| `🗑️generated/` contents | — | not touched | Not enumerated here — this audit does not read/modify that folder's disposition; owning wave/close-out decides. |

Full script inventory (44 total, all recommended KEEP as ticket inputs):

**🔍️ probe helpers (33)**: b20-click-add.ts, b20-click-paths.ts, b20-dialog-controls.ts, b20-dialog-inspect.ts,
b20-inspect-engagement.ts, b20-quick-inspect.ts, b20-rich-inspect.ts, b22-mesh-probe.ts, b32-completion-tap.ts,
b33-brush-pollution.ts, b36-pollution.ts, b4-main-thread-probe.ts, b44-mutation-latency.ts, b44-switch-inspect.ts,
b46-selection-nakagin.ts, b48-selection-refresh.ts, b50-camera-lane.ts, b6-lane-probe.ts, browser-probe.ts,
diagnose-publication-authority.ts, isolate-puzzle3d-tests.py, probe-fill-audit-mutations.ts,
probe-fill-p4e-findings.ts, probe-wave-G2-puzzle-interactivity-clauses.ts, scan-value-derive-gaps.py,
trace-publication-authority.ts, verify-retained-jobs-fixtures.ts, verify-wave-B-batched-publication.ts,
verify-wave-T-publication-authority.ts, verify-wave1-puzzle3d-hostile-mutations.ts,
verify-wave1-puzzle3d-oracle.ts, wp5-world-lane-probe.ts, ws2-scene-latency-probe.ts.

**🔨️ repair/build scripts (11)**: canonicalize-puzzle3d-mutation-fixture-floats.py, check-p3d.sh,
check-plugin-host.sh, complete-puzzle-value-derive.py, rebuild-until-ok.sh,
repair-scalar-config-cohort-fixture.ts, restore-crate-root-leaf-ownership.py, restore-emoji-corruption.py,
restore-leaf-ownership.py, seed-targets.sh, serve-release-direct.sh.

Note: `browser-probe.ts` itself injects two debug taps that are **not** production code and must not be swept as
if they were — `[DEBUG] native pointerdown …` (line 867) and the console-tail filters that read
`performInvocation settled`/`interactionSelect`/`leftover` text back out of the captured browser console. These
belong to the probe harness, confirmed by `grep -n 'native pointerdown' T/🔍️browser-probe.ts`.

## 6. Summary for the close-out sweep

- **99** ungated, non-peer, session-added `[DEBUG]` lines/blocks are REMOVE candidates (§2, area totals: area 1
  = 6, area 2 = 19, area 3 = 74 minus the 8 gated/9 peer already carved out — see per-area tables for exact
  file:line), plus the 2 confirmed live floods in §4 (already counted within the 74).
- **22** are already gated (`runtimeDiagnosticsEnabled()`/`runtime_diagnostics_enabled()`/`cfg(test)`) — KEEP,
  no action.
- **17** carry a `PROCEDURAL-3D-END-TO-END` marker in their immediate window — PEER, leave untouched; re-verify
  ownership by re-reading the named comment before deleting anything nearby, since the window scan is a ±20-line
  heuristic and can occasionally attribute a marker to the wrong adjacent block.
- **281** pre-session hits are out of scope by rule 2 (verbatim at `46c3cb9de0`) — already adjudicated by
  `📓️2026-09-10-wave-Z-debug-sweep.md` or belong to the peer ticket's diagnostics facility; not re-listed
  line-by-line here.
- Areas 6 (⏱️trace/🧵️job) and 4 (ui-react) need **no** sweep action — zero/pre-session-only hits.
