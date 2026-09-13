# 🧵 `spawn-job`, routed and RUN — the wgpu host finally applies hover and selection (2026-09-13)

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane **wgpu-spawn-job-effect** (Opus).
Target: `http://127.0.0.1:6118/?plugin=generation3d` (editor) and `…&role=viewer`, the coordinator's wgpu serve.

Repo MCP was down all session (`repo -32602 invalid initialize params`; `semio CONNECTION_CLOSED`) — no ticket
was opened, closed or reopened by this lane, `📓️status.md`/`🎫️ticket.json` were not touched, no
git-state-modifying command was run, no dev server was started or stopped, and nothing under
`🗑️generated` that this lane did not create was deleted.

---

## 1. TL;DR

`📓️wgpu-world3d-interaction-2026-09-13.md` §7 left this at "every gesture publishes the exact right
wire message and the guest never applies it". It named the last hop correctly: `interactionSelect` /
`interactionHover` are framework reserved TOOL JOBS delivered as `Effect::SpawnJob`, and the wgpu
bridge dropped that effect.

Three things were wrong, and they are now fixed at three different owning layers:

| # | Layer | Defect |
|---|---|---|
| ❶ | `🎠️kernel` | **Nobody owned the drive rule.** The "start → step to a terminal `job-step` → answer `Event::JobCompleted`" contract existed only as hand-copied constants inside `🔌️plugin/🦀️.rs`'s test helpers. |
| ❷ | `🎭️actor/🖼️wire-turn` + wgpu bridge | **No `spawn-job` case at all**, on EITHER browser door — the effect hit `wireEffectToFriendly`'s default arm. 12–18 `unmapped effect "spawn-job" dropped` per run on 6118, exactly two per gesture. No browser host has ever run a framework reserved job. |
| ❸ | `♾️infinite/🌍️world` | **The gumball drew a mesh it never uploaded.** Unreachable before ❷ — the instant a selection actually applied, the first frame after it died with `present_step faulted: prepared frame submit step: prepared world mesh was missing` and took the whole frame loop with it. |

**Proven on 6118, in both roles**, with `unmapped effect "spawn-job"` = **0**:

| gesture | the pump | the guest's own published scene |
|---|---|---|
| hover the mesh | `job=909 kind=framework.reserved.tool placement=isolated input=174B steps=2 outcome=ok bytes=158` | `extrude@solid#0 … "hovered":true` |
| click | `job=918 … input=189B steps=2 outcome=ok bytes=173` | `extrude@solid#0 … "selected":true`, selection lane `172 B → 185 B` |
| shift-click (additive) | `job=936 … input=190B steps=2 outcome=ok bytes=174` | selection lane `185 B → 201 B` |
| empty click | `job=944/953 … input=121B/136B steps=2 outcome=ok` | `interactionSelect targets:"[]" merge:"replace"` runs |
| marquee | `job=909 … input=194B steps=2 outcome=ok bytes=178` | (see §7 — the APPLY was not observed) |
| orbit | **no job at all**, 8 × `setCamera` | the typed-operation lane is untouched |
| viewer role | `job=836/841 … steps=2 outcome=ok` | `extrude@solid#0 … "selected":true,"hovered":true` |

Pre-fix control from the same probe (`🗑️generated/wgpu-spawn-job/shape`): `selected=false hovered=false`,
selection lane 172 B, **0** occurrences of `"selected":true` in the whole run. Post-fix editor run:
**30**; viewer run: **12**.

**What is NOT claimed** is in §7, and it is substantial: the wgpu shell's frame loop still stops a few
APPLIED selections in, and the marquee's apply was never observed. Both are named exactly.

---

## 2. The hop, end to end

```
pointer ─► … ─► publish_world3d_plan_step ─► interactionSelect                    ✅ the world3d lane
   └─► guest turn ─► Effect::SpawnJob{job, "framework.reserved.tool", input, Isolated}
          │
          ├─ BEFORE: wireEffectToFriendly default arm → console.warn → null       ❷ ← FIXED §4
          │          (no startJob, no stepJob, no job-completed; the guest's
          │           parked RequestFuture on `job` never resolved, ever)
          │
          └─ NOW:   wireSpawnJob ─► admitSpawnedJob (dedup by job id)
                       └─► driveSpawnedJob: startJob ─► stepJob×n ─► terminal      ❶ ← kernel rule
                              └─► spawnedJobCompletedEvent ─► submitTurn
                                     └─► guest reactor Event::JobCompleted
                                            └─► plugin_complete_reserved_spawned_job
                                                   └─► selection APPLIED, scene republished
                                                          └─► gumball drawn                ❸ ← FIXED §5
```

---

## 3. ❶ The drive rule now has ONE owner — `🎠️kernel`

`🧰️framework/🔨️modules/🎠️kernel/🦀️.rs`, new region `🧵️SpawnedJobDrive`, immediately beside
`JobPlacement` and `Effect::SpawnJob`:

```rust
pub const SPAWNED_JOB_STEP_CEILING: u32 = 32;
pub const SPAWNED_JOB_FUEL: u64 = 50_000_000;
pub const SPAWNED_JOB_DEADLINE_MS: u32 = 100;
pub enum SpawnedJobStep { Running, Done { value: Vec<u8> }, Failed { value: Vec<u8> } }
pub struct SpawnedJobCompletion { pub steps: u32, pub outcome: RequestOutcome }
pub enum SpawnedJobDriveError { Stalled { steps: u32 }, Overrun { steps: u32 } }
pub fn spawned_job_completion(steps: &[SpawnedJobStep]) -> Result<SpawnedJobCompletion, SpawnedJobDriveError>
pub fn spawned_job_completed_event(job: u64, completion: &SpawnedJobCompletion) -> Event
impl JobPlacement { pub fn wire_name(self) -> &'static str; pub fn from_wire_name(&str) -> Option<Self>; pub const ALL }
```

with the TypeScript twin (`spawnedJobCompletion`, `SpawnedJobDriveError`, `JOB_PLACEMENTS`,
`jobPlacementFromWireName`, the three constants) in `🎠️kernel/🟦️.ts`, and the language-agnostic law
`🎠️kernel/🧫️fixtures/🧵️spawned-job-drive/🔣️.json` both halves drive.

Three decisions worth stating:

- **The three budget numbers were already a contract, just an unwritten one.** `🔌️plugin/🦀️.rs:26281`'s
  `drive_framework_reserved_spawn_like_host` is documented verbatim as the "Browser `driveSpawnedJob`
  contract: one Isolated admission of 32 `step-job`s with the host's static budget", and asserts the
  reserved job finishes in ≤ 2 steps. That helper is a `#[cfg(test)]` copy of numbers no production
  code held. They live in the kernel now, and the fixture pins them for both languages. The measured
  browser runs report `steps=2` on every single job — the helper's assertion, observed live.
- **A refusal is a COMPLETION, not a drive error.** `Failed(bytes)` resolves the guest's parked future
  with `Err`. Only "no terminal step inside the ceiling" (`Stalled`) and "more observations than one
  admission grants" (`Overrun`) are drive errors. A host that says nothing leaves the interaction
  hanging forever — which is exactly the shape the dropped effect already had.
- **`placement` is refused, never defaulted.** jco lowers a WIT `enum` to a BARE string (measured:
  `placement: "isolated"`, not `{tag:"isolated"}`). Guessing `inline` for an unknown spelling would run
  a pooled job inside the spawning instance's own turn budget.

## 4. ❷ One wire contract, both renderer doors

`🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts` — the module whose own header warns against the
"third divergent copy" — gains:

- `wireSpawnJob` / `wireCancelJob` / `wireJobIdentity`, and the two missing `switch` cases.
  `job` stays a **`bigint`**: it IS the guest's parked request id (`⚛️reactor/🔄️turn/🦀️.rs` — "the request
  id already IS the job id"), and `ShardClient.startJob`/`stepJob` reject a `number` at the u64 lowering.
  The kernel TS `Effect` union was narrowing it to `number`; that is corrected, along with
  `input: readonly number[] → Uint8Array` and `placement → JobPlacement`.
- `wireJobStep` — reads BOTH real shapes of `jobs::job-step`: jco's raw `{tag, val}` and the shard
  worker's normalized `{status, value}`. Reading only one of them turns a finished job into a stall,
  the same class of defect that lost `download-media-export.encoding` today.
- `driveSpawnedJob(drive)` — the shared pump, with `startJob`/`stepJob` injected as a `SpawnedJobPort`
  exactly as `driveInboundRequest` injects `submit`; the TRANSCRIPT is handed to the kernel's rule,
  this loop decides nothing.
- `spawnedJobCompletedEvent(job, completion)` — the WIT `job-completed-event` in the `{kind, payload}`
  envelope every other host-submitted event uses.

The wgpu bridge (`🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts`) consumes it:

- `admitSpawnedJob` takes a `spawn-job` out of the effect stream at the two funnels where a non-frame
  effect lands (`stashLeftoverHostEffects`, `leftoverFriendlyEffects`) plus the typed-operation drive
  and the extension-completion door, **deduplicated by job id** — the same effect is observed twice per
  gesture (once as the command turn's host effect, once as the leftover drain's), and starting a job
  twice is a guest-side identity collision, not a retry.
- `drainSpawnedJobs(instanceId, actorId)` runs them inside the actor's own `serializeWgpuActorCall`
  body, publishes each completion's reply frames on the SAME outcome the shell is waiting on, and
  hands back the LAST turn it drove.
- That last turn is the one whose `more-work` decides whether to keep polling. Reading the command's
  own last turn — which ended before the job started — stopped the drain on work the job itself
  re-armed. Measured: every reserved-job completion turn reports `status=more-work`.

React's `🔌️PluginRuntime` has no `spawn-job` handling either, and this lane did not add it (§8).

## 5. ❸ The gumball drew a mesh it never uploaded

`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`, `append_gumball_geometry`:

```rust
if meshes.contains_key("gumball-plane") {                  // ← before: a CPU-lease check only
    let mesh_version = *mesh_versions.get("gumball-plane").unwrap_or(&0);
    …
    translucent.push(SceneDraw3d { mesh_key: "gumball-plane".into(), mesh_version, … });  // ×3
}
```

Three translucent draws keyed `gumball-plane`, and **no `gpu.ensure_mesh` anywhere** — the one call
every other world draw site pairs with its `SceneDraw3d` (`:10135`, `:10163`, `:10176`, `:10207`,
`:8702`, `:7339`, `:11520`). `sync_mesh_pool`'s `PINNED` list (`"vortex-marker"`, `"cylinder"`,
`"cone"`, `"reference-plane"`, `"vertex-marker"`) did not name it either, so the pool saw it as stale
every frame.

On the GPU a missing mesh is **fatal**, not a skipped draw: `encode_prepared_world_instance`
(`🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:3181`) does
`mesh_store.get_versioned(…).ok_or("prepared world mesh was missing")?`, while the non-prepared
`draw_world_range` right below it silently skips. Measured on 6118 the frame after the first applied
click:

```
[DEBUG] world3d surface=procedural-preview … draws=1 translucent=3 instances=4 … state-meshes=3
[DEBUG] os_host present_step faulted: prepared frame submit step: prepared world mesh was missing
<the frame worker never logs again>
```

`translucent=3` is exactly the three gumball quads. **Fix**: a named `GUMBALL_PLANE_MESH` constant used
by all three sites that used to disagree, `gpu.ensure_mesh(GUMBALL_PLANE_MESH, mesh_version, *plane)`
beside the draws, and the key added to `PINNED`.

## 6. Laws (all run in the FOREGROUND)

| Lane | Result |
|---|---|
| `bun nx run workspace:spawned-job-drive` (TS twin) | green — `transcripts=5 refusals=3 placements=3 ceiling=32 events=2 oracle=ajv` |
| `bun nx run workspace:spawned-job-drive-native` → `cargo test -p semio-framework --lib spawned_job_drive` | **7 passed** |
| `cargo test -p semio-framework-os-infinite --lib gumball_` | **7 passed** (2 new + 5 pre-existing) |
| `@semio-tech/framework-actor:test-quick -t "spawn-job"` | **5 passed**, 253 skipped |
| `@semio-tech/framework-actor:test-quick` (whole suite) | 243 passed / 15 failed — all 15 pre-date this lane, see §7 |
| `@semio-tech/framework-os-shell:test-quick` | **7 passed** |
| `@semio-tech/framework-os:test-quick -t "effect"` | **1 passed**, 4 files skipped |
| `nx run @semio-tech/framework-renderer-wgpu:wasm` | `Successfully ran target`, dist `20:47`, warnings emitted (proof it really compiled) |
| `nx run @semio-tech/framework-renderer-wgpu:generate-frame-worker` | green ×5 (see §9 — the wgpu page does NOT hot-reload TS) |

Raw output: `T/🗑️generated/wgpu-spawn-job/{ts-law,rust-law,gumball-law,actor-suite,react-shell,os-effects}.txt`.

**The 10 new fixture rows**, all recorded from the wire, not invented:

- `🎠️kernel/🧫️fixtures/🧵️spawned-job-drive/🔣️.json` — the budget, the placement vocabulary (3 + 5
  refusals), 5 transcripts (terminal on step 1 / step 2 / after the terminal / on the very last admitted
  step / a `failed` that still completes), 3 refusals (empty transcript = literally the wgpu defect,
  stall at the ceiling, overrun past it), and both `job-completed` event arms.
- `🎭️actor/🧫️fixtures/📨️effect-wire-routes/🔣️.json` — 6 new rows: the two **verbatim** payloads 6118
  recorded (`job 909`, 174 B `interactionHover`; `job 916`, 189 B `interactionSelect`, both
  `FRRESV01` + `u64 work-items=2` + the JSON call), the `cancel-job` twin, and three refusals
  (unknown placement, job id 0, empty kind). The fixture gained a `u64Fields` convention so a WIT `u64`
  is stated as decimal TEXT and rehydrated to a `bigint`, and a `{base64}` form for `pack` fields —
  narrowing either would make the fixture assert a shape no renderer door ever sees.

**Third-party oracles.** Rust: `serde_json` serializes `Event::JobCompleted` and is asserted against the
fixture's declared JSON — an encoder that shares no line with this module. TypeScript: `ajv` compiles a
JSON Schema for the `job-completed` wire event and validates the builder's output; the drive loop is
driven through a SCRIPTED guest port answering in jco's RAW `{tag, val}` shape, so the loop and the rule
are checked separately rather than by each other.

## 7. What is NOT claimed

1. **The wgpu shell's frame loop still stops a few applied selections in.** Deterministic in kind,
   non-deterministic in when (measured after 1, 2 and 4 applied selections across five runs). The
   signature is exact and is NOT this lane's layer:
   - the last line is always `wgpu-shell render leave surface=framework.panel.history` — the final
     surface of the shell's post-selection render sweep (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3910`);
   - the next statements in that function are `derive_utility_nodes`, then `refresh_app_catalogue`,
     `window_engagements`, `window_measures`. A temporary `[DEBUG]` on the bridge's own invoke door
     (since removed) showed **14 enters / 14 leaves** — no guest call is outstanding, and the shell
     never reaches one;
   - the page's MAIN thread stays fully alive throughout (`requestAnimationFrame` 3 ticks in 21–36 ms,
     canvas present, every probe step);
   - no panic, no `present_step faulted`, no `frame deferred action failed`, no worker fault, no
     console error of any kind after the fix in §5.
   It is reachable ONLY because interactions now apply. Probe: `T/🐍️wgpu-spawn-job-probe.mjs`
   (`SEMIO_PROBE_ONLY=<step>` boots fresh for one gesture, which is how the later gestures were measured
   at all). Evidence: `T/🗑️generated/wgpu-spawn-job/{editor-5,editor-7,final-editor-2}/console.txt`.
2. **The marquee's APPLY was not observed.** The gesture publishes
   `interactionSelect … method:"rectangle"`, the reserved job runs (`input=194B steps=2 outcome=ok
   bytes=178`), and the guest logs its `validate-state-pruned` verdict — but across two runs (one with a
   4× settle budget, 27 s after the job completed) the republished scene never carried `"selected":true`
   for the marquee. Hover, click, shift-click and empty-click all do. Unresolved, and not claimed.
3. **`interaction selection lost reason=validate-state-pruned` fires on every select** —
   `dispatched=graph=object:extrude@solid;vortex=object:extrude@solid`,
   `validated=graph=node:extrude@solid`. The guest's own diagnostic (`🔌️plugin/🦀️.rs:20158`), an
   `eprintln!`, not a fault: the `graph` domain applies (`"selected":true` is published) while the
   `vortex` domain's readback disagrees on `object` vs `node` granularity. Pre-existing vocabulary drift
   between the pick's granularity and `protocol::validate_state`; untouched here.
4. **The gumball was not seen ON SCREEN.** §5 is proven by the two new Rust laws, by the disappearance
   of `present_step faulted` (5 runs, 0 occurrences, against a deterministic fault before), and by the
   frame loop surviving the first applied selection — not by reading pixels.
5. **The inspection panel "reacts" is proven by its patch, not its content**: `renderSurface
   surface=framework.panel.inspection … patched=1 nodes=4 rev=2` (it was `rev=1 patched=0` before the
   click), and the history panel gains `rev=2`. The rendered text was not read.
6. **React's door was not given a `spawn-job` case.** `🔌️PluginRuntime/🟦️.tsx` contains no occurrence of
   `Job` at all, and no production caller of `ShardClient.startJob`/`stepJob` existed anywhere before
   this lane — §7 of the world3d report is right that this gates every reserved verb on every browser
   target. The shared contract (§3, §4) is now in place for React to consume in one small edit; doing it
   without a React runtime proof would be a claim this lane cannot make. **Follow-up 1.**
7. **`placement` is honoured as the native host honours it — same instance.** `Isolated` means "its own
   pooled actor" in `📓️design-abi.md`, and `🖥️host/🧵️shard/🦀️.rs:1841` documents that even the native
   shard runs every placement on the SAME instance that spawned it ("routing to a DIFFERENT pooled
   instance needs the actor pool … documented gap"). The browser pump matches that, deliberately.
8. **No renderer-wgpu wasm rebuild was needed after the last TS change** — only the frame-worker bundle.
   The Rust half of this lane (§5) is in the `20:47` wasm; every later probe ran against it.

## 8. Fix-forward on peers' work

None needed — nothing blocked this lane. The 15 failures in `@semio-tech/framework-actor:test-quick` are
in six files this lane never touched (`🚪️lifetime/🟦️.ts`, `🚪️lifetime/🩹️patch/🟦️.ts`,
`🪪️activation/🚪️instance/📥️output/🟦️.ts`, `📤️return/🟦️.ts`, `📤️return/📨️response/🟦️.ts`,
`📮️shard-client/🟦️.ts`), none of which imports `🖼️wire-turn` or the kernel's spawned-job region; they are
ajv `$ref`-resolution and watchdog-timing failures, four of which
`📓️download-media-export-encoding-2026-09-13.md` §7 already reported this morning. The `🖼️wire-turn`
suite itself is green, including all 6 new rows.

## 9. A workflow fact the next wgpu lane needs

**The 6118 page does NOT hot-reload TypeScript.** `🌐️server/🟦️.ts` sets `server: { watch: null }` and
`optimizeDeps: { noDiscovery: true }`, and mounts only COMPLETED artifacts; the whole TS host — the
plugin bridge, `🖼️wire-turn`, the kernel twin — is pre-bundled into
`🎞️frame-worker/🤖️generated/🟨️.js`. A source edit is invisible until

```
CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false bunx nx run @semio-tech/framework-renderer-wgpu:generate-frame-worker
```

re-emits it (~30 s warm, ~2 min cold), after which a plain page reload picks it up. This lane's first
measurement run reported `[DEBUG] wire spawn-job: 0` alongside 14 live drops purely because of this —
the instrument was in the source and not in the bundle.

## 10. Files

| File | What |
|---|---|
| `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs` | **new** `🧵️SpawnedJobDrive` region: the 3 budget constants, `SpawnedJobStep/Completion/DriveError`, `spawned_job_completion`, `spawned_job_completed_event`, `JobPlacement::{wire_name,from_wire_name,ALL}`, test mount |
| `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` | TS twin + `Effect.spawnJob`/`cancelJob` corrected to the real wire types (`bigint` job, `Uint8Array` input, `JobPlacement`) |
| `🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/🧵️spawned-job-drive/🔣️.json` | **the language-agnostic law** |
| `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧵️spawned-job-drive/🦀️.rs` | Rust laws (7), `serde_json` oracle |
| `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧵️spawned-job-drive/🟦️.ts` | TS twin (10 sections), `ajv` oracle + a scripted guest port |
| `🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts` | `wireSpawnJob`/`wireCancelJob`/`wireJobIdentity`/`wireJobStep`/`driveSpawnedJob`/`spawnedJobCompletedEvent`/`wireSpawnedJobs`, the two missing switch cases |
| `🧰️framework/🔨️modules/🎭️actor/🧫️fixtures/📨️effect-wire-routes/🔣️.json` | 6 new rows (2 recorded verbatim, 1 `cancel-job`, 3 refusals) + the `u64Fields`/`base64` conventions |
| `🧰️framework/🔨️modules/🎭️actor/🧪️tests/📨️effect-wire-routes/🟦️.ts` | rehydrates u64/pack rows; refusal rows |
| `…/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts` | `pendingSpawnedJobs`/`admitSpawnedJob`/`drainSpawnedJobs`, the four admission funnels, the `more-work` verdict correction |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs` | `GUMBALL_PLANE_MESH`, `append_gumball_geometry`'s `ensure_mesh`, `PINNED` |
| `…/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` | 2 new laws |
| `📜️script.ts`, `📋️project.json`, `.vscode/launch.json` | the `spawned-job-drive` / `-native` lane |
| `T/🐍️wgpu-spawn-job-probe.mjs` | the browser proof (`SEMIO_PROBE_ONLY`, `SEMIO_PROBE_SETTLE_SCALE`, a main-thread liveness ping) |

## 11. Follow-ups

1. **Give React's `🔌️PluginRuntime` the same case.** It now needs only `admitSpawnedJob` +
   `driveSpawnedJob` + `spawnedJobCompletedEvent` around its existing `ShardClient`; the contract and the
   laws are already shared. Until then no reserved tool verb applies on the React target either.
2. **The shell wedge in §7.1** — the wgpu frame loop stopping after the post-selection render sweep.
   Owner: `🐚️Shell/🎯️targets/🧊️wgpu` / `🧊️renderer`, not the actor layer.
3. **The marquee apply (§7.2)** and **the `object`/`node` granularity drift (§7.3)**.
4. **`🔌️plugin/🦀️.rs`'s `drive_framework_reserved_spawn_like_host` should call the kernel constants**
   rather than repeat `32` / `50_000_000` / `100` — it is now a second copy of a contract that has an
   owner.
5. **The reserved job's output is 16 bytes smaller than its input, every time** (174→158, 189→173,
   190→174, 194→178). Consistent, unexplained, and worth one look before someone treats it as a size
   check.
