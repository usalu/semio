# Checkpoint 19 Probe Readiness

## Scope

This audit checks whether the existing paired runtime probe can distinguish the checkpoint 18 Settings failure from a geometric overlap or a final-state-only success. It changes neither production nor the probe before checkpoint 19. The fresh runtime result remains the authority for whether more diagnostics are needed.

The intended checkpoint 19 journey is the existing bounded subset:

`boot,dismiss-tour,settings-open,settings-app,settings-general,settings-drivers-open,settings-drivers-close,settings-close`

with the existing 240 second boot budget and a 2400 ms minimum settle interval.

## Existing accepted witnesses

The probe already captures the required endpoint authority:

- `wgpuDriver.observe()` reads `dumpChrome()` and `dumpStructure()` and returns the chrome generation, every published hit with its control id, kind, rectangle, window, and action, the surface census with panel/window/dialog levels, and fullscreen state (`🐍️parity-interact-probe.mjs:561-579`).
- `wgpuDriver.actions()` reads the complete bounded chrome action journal with sequence, controller, action, window, origin, and arguments (`:581-584`).
- boot samples once per second, writes `console.txt` and `boot-progress.json` every 15 seconds, and accepts only after readiness, more than four hits, stable controls/surfaces/cameras/action sequence, and a later chrome generation (`:593-621`).
- settling samples the accepted chrome authority every 250 ms and records elapsed time, first later publication, first semantic change, generation before/after, and whether publication advanced (`:623-645`).
- each step preserves its operation receipt, settling summary, full accepted control registry, action delta, surface/control/fullscreen delta, named geometry, diagnostics, console, and endpoint screenshot (`:1386-1423`).
- WGPU diagnostics preserve the accepted chrome census, mesh census, global and per-panel frame statistics, structure, native accessibility projection, and browser accessibility mirror (`:647-654`).

The Settings steps also have semantic gates rather than timing-only waits:

- `settings-app` waits for all four application NumberSteppers (`:705-711`).
- `settings-general` waits for the Appearance and Language controls (`:712-716`).
- Drivers starts with all seven child controls absent, requires all seven after opening, and requires all seven to retire after closing (`:721-734`).
- closing Settings requires every Settings panel surface to retire (`:736-739`).

The current WGPU ledger is a presentation witness. `Shell::acknowledge_presented_input` first accepts the exact candidate, then calls `InputState::publish_hits`, and only afterward publishes accessibility, chrome hits, and chrome surfaces (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13028-13050`). `note_chrome_hit_registry` increments the diagnostic generation when it replaces the hit ledger, and `note_chrome_surfaces` replaces the census from the same complete chrome walk (`🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:4293-4325`). Therefore checkpoint 19's `dumpChrome.generation`, hits, and surfaces describe accepted presentation input rather than an unpresented candidate.

## Checkpoint 18 distinction

Checkpoint 18 recorded:

| Step | Chrome generation | Published Settings authority | Pixels |
| --- | ---: | --- | --- |
| `settings-open` | 6 | none | old full-width World |
| `settings-app` | 6 | none | old full-width World |
| `settings-general` | 7 | Settings toggle plus four NumberSteppers | old full-width World |

The completed layout already reserved the World to `[437.6, 60.8, 849.59985, 907.2]`, while the screenshot still showed the earlier full-width World. The defect was temporal publication authority, not current-frame scene/UI paint order. Checkpoint 19 can now reject a recurrence because the endpoint screenshot, accepted chrome generation, exact control rectangles, surface census, action journal, and frame diagnostics are all preserved per step.

## Remaining diagnostic gap

`settle()` discards its intermediate 250 ms observations. It retains only the first publication/change times and final generations. `dumpFrameStats()` is read only after settling. Its bounded `frameLatency` snapshot contains `totalObservations`, evictions/refusals, stage totals, and recent phase summaries with renderer-frame generation, stage, sequence, timing, and work (`⏱️frame-latency/🦀️.rs:120-199`), but the current receipt cannot say which renderer-frame generation or presenter stage advanced during the click-to-presentation interval.

The endpoint screenshot proves the final pixels and geometry. It does not timestamp whether pixels caught up exactly when the accepted chrome generation advanced. Production introspection also does not expose a live `AppPresentPhase` or pending presentation witness. Adding such production state before a fresh result is not justified.

## Bounded fallback diagnostic

If checkpoint 19 reproduces a logical/physical gap, extend only the ticket probe's WGPU `settle()` loop:

1. Read the existing `dumpFrameStats()` beside `observe()`.
2. Keep at most 64 compact samples, appending only when a signature changes or once per second as a liveness heartbeat.
3. Derive the newest renderer-frame phase from `frame.frameLatency.recentPhases`, filtering `authority.domain === "rendererFrame"` and choosing the greatest `lastSequence`.
4. Preserve each sample as:

   `{ atMs, chromeGeneration, controlCount, surfaceCount, actionSeq, totalObservations, rendererFrameGeneration, rendererStage, rendererSequence }`

This uses an existing bounded diagnostic surface and adds no production dependency or runtime capacity. It distinguishes:

- no frame turns after the operation;
- a renderer generation stuck in one presenter stage;
- advancing frames with unchanged accepted chrome;
- accepted chrome advancing before the endpoint pixels agree.

If that timeline still shows the accepted generation advancing before visible pixels, take one additional probe screenshot at the first accepted chrome-generation advance and retain the normal endpoint screenshot. Do not add this capture to the canonical run unless the fresh result demonstrates the mismatch, because it changes probe timing.

## Readiness verdict

The existing checkpoint 19 probe is sufficient for the canonical fresh acceptance run. It directly tests final Settings visibility, the World reservation, accepted control routing, Drivers disclosure/retirement, and Settings closure. The only missing evidence is an intermediate frame-phase timeline, for which the bounded probe-only extension above is prepared but deliberately not applied before the fresh result.
