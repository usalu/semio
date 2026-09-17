# Hover latency (2026-09-17)

Reported: the hover highlight lands about a second after the pointer. Measured on the served cad page (headless Chromium, `--use-angle=metal`, [🐍️cad-hover-latency-probe.mjs](./🐍️cad-hover-latency-probe.mjs)): **450–590 ms** from `pointermove` to the guest's `hoveredId` reaching the DOM, per hover change, on every change. The desktop app on the same machine shows more because the main thread is busier.

## Where one hover went (hop-trace spans, `semio.hop.*` User Timing, before any fix)

| ms | stage | what |
|---:|-------|------|
| 0 | `pointermove` → raycast → `dispatchSettled("interactionHover")` | host, coalesced to one outstanding hover |
| 6 → 17 | action turn | guest: 10 ms — the reserved `interactionHover` job runs inline and is done |
| 17 → 176 | `job-completed` turn | guest: **154 ms** — the job's completion event applies the hover to the interaction store and re-renders every window the `cad` domain binds (all four) |
| 176 → 258 | continuation | guest: **82 ms** — publishes the one window whose selection lane changed |
| 258 → 330 | `patch.install` | host: 72 ms — the whole world surface (~100 KB of paged mesh lane) re-admitted |
| 330 → 650 | host `refresh` (`scope: partial, windows: 4`) → `surface-visible` ×4 | guest: **186 ms** — renders all four windows AGAIN, finds nothing changed (`patches: 0`) |
| 650 → 715 | React `commit` | host: 74 ms — the shell root re-renders (every dispatch rebuilt the state object), then the R3F scene graph reconciles |
| **715** | `data-guest-selection-json.hoveredId` changes; the highlight material follows on the next frame | |

Everything the user sees waited for the last row: the world host painted hover **only** from the guest's echo.

## Fixes

| Layer | Defect | Fix |
|-------|--------|-----|
| `🌐️World3dHost/🟦️.tsx` (framework host) | The instance highlight came from `selection.hoveredId` alone — the guest's echo of the host's own raycast, one round trip late | `WorldInstanceChromeStore.setLocalHover`: the raycast result paints at once (`worldHoverPaintIdV1(local, guest)`: the pane's own claim leads while the pointer is inside the pane, the guest's lane leads when the pointer is elsewhere — an outliner or a remote presence hover still shows). The claim is withdrawn on the canvas root's `pointerleave`. No guest semantics change: the same `interactionHover` reaches it, the same lane comes back, a click reads the same ids. The painted hover is mirrored on `data-hover-paint-id` for probes. Law in `🔬️engine-contract`. |
| `🐚️Shell/🟦️.tsx` (framework host) | `shellReducer` spread a fresh `ShellState` (and every slice reducer a fresh slice) on EVERY action, so the eight identity-preserving dispatches one guest refresh fans out (`mergeRecordPreservingIdentity` kept the entries, the spread threw the bail-out away) re-rendered the whole shell tree on every hover echo — ~20 ms of chrome reconcile per pointer move, plus the R3F tree of every pane | `withField` writes a slice field only when `Object.is` differs; `shellReducer` returns the SAME state when every slice kept identity (`shellStateUnchanged`), so `useReducer` bails out. 43 single-field cases rewritten. Law: a same-value fan-out returns the identical state; a real change touches only its slice. |
| `✏️editor/🎭️modes/✏️edit/🦀️.rs` (cad guest) | `world_meshes_json` re-tessellated every kernel-backed object from the host snapshot on every render, and a hover renders all four panes — four Concrete Forest re-tessellations per pointer move for a lane whose bytes never changed | `world_meshes_json_cached`: the lane is remembered per materialized `Arc<CadWorkingScene>` (a `Weak` pins the allocation, so the address is the geometry's identity and a stale hit is impossible) plus a digest of every object field the tessellation reads. Law `mesh_lane_is_cached_per_materialized_scene`. |
| `✏️editor/🦀️.rs` (cad guest) | `interaction_scope` unset: the framework's fallback placed a hover on the four world bodies, but any verb it could not place fell to `Full` (every body, every panel, every rail) | `cad_interaction_scope`: Hover → the four world bodies only; Select/Clear/SelectAll → world bodies + Inspection + Artifact panels + the HUD count; mode/granularity → `None`; a foreign domain → the framework's answer. Law `cad_interaction_scope_keeps_hover_to_the_world_bodies`. |

## After

| | before | after |
|---|---:|---:|
| painted hover (what the user sees), pointer move → highlight | 450–590 ms | **5–100 ms** (`paintMs` in the probe; the spread is main-thread contention with the previous echo's commit) |
| guest echo, pointer move → `hoveredId` in the DOM | 715 ms (trace) / 450–590 ms (probe) | **428 ms** (trace) |
| guest `job-completed` turn | 154 ms | 32 ms |
| guest publish turn | 82 ms | 71 ms |
| guest re-render of the four windows on the host refresh | 186 ms | 82 ms |

The echo still pays for what is structural and NOT cad-local:

1. **Three guest turns per hover** (action → reserved job → `job-completed` → publish) where the hover itself takes 4 ms. The reserved-job shape exists for bounded execution of every framework verb; a pure interaction-store write does not need it.
2. **The world surface is republished whole**: `paged_text_carrier` re-chunks the 100 KB mesh lane into ~200 text nodes per render and the retained surface re-admits them on the host (`patch.install` 60 ms, `intake.advance` ≈ 20 ms per hover in the CPU profile). Unchanged lanes should be content-addressed pages the tracker elides and the host keeps.
3. **The host refresh after an action re-renders every scoped surface** (`surface-visible` ×4) although the guest's own dirty pass in the same hop already published the one that changed — a second full render to learn `patches: 0`.
4. **Hover is still a guest round trip for the highlight's semantics only** — the paint no longer waits for it. Collaboration is untouched: the guest's hover state, the presence mirror and the lane are exactly what they were; only what is painted between raycast and echo differs, and it differs toward what the guest answers.

## Verification

- Native: `mesh_lane_is_cached_per_materialized_scene`, `cad_interaction_scope_keeps_hover_to_the_world_bodies`, `world_scene_render_cost_probe` (cold 731 ms / warm 1.7 ms for the Building pane in a debug build — the ~70 ms guest publish turn is reactor and patch overhead, not cad) pass; a full `semio-s-artifact-cad-cad --lib` run was SIGKILLed three times while `Blocking waiting for file lock` behind a peer fleet of ~46 concurrent `cargo test`s (exit 137 before any test ran), so the crate-wide figure stays the phase-2 331/19 plus these laws.
- React: `🔬️engine-contract` "shell store reducer" block 32/32 (incl. the new identity law), the `worldHoverPaintIdV1` law, `searchControlledLineV1`; `🖱️world3d-interaction` has 7 pre-existing failures (`granularity: "handle"` vs the suite's `"object"`, committed default `WORLD3D_DEFAULT_INTERACTION_GRANULARITY = "handle"` at HEAD, suite unchanged since 09-12) plus jsdom's missing canvas — none touched by this phase.
- Browser: [🐍️cad-hover-latency-probe.mjs](./🐍️cad-hover-latency-probe.mjs) after: painted 6–36 ms (one 315 ms outlier under a concurrent echo commit), echo 310–590 ms; the phase-2 interaction probe still commits after these changes (Place Column and Box re-run, 0 faults).

## Tooling kept

- [🐍️cad-hover-latency-probe.mjs](./🐍️cad-hover-latency-probe.mjs) — `paintMs` (painted) vs `echoMs` (guest lane) per hover change, plus a 40-move sweep.
- Hop-trace spans read straight from `performance.getEntriesByType("measure")` (`semio.hop.*`) — the breakdown table above; a CDP `Profiler` sample aggregated by component name found the shell-root re-render and the R3F reconcile.
