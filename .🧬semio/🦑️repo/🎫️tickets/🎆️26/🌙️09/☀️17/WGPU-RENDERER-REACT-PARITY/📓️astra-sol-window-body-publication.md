# Display Window Initial Body Publication and Retirement

Source packet completed on 2026-09-20. This packet addresses the checkpoint-10 failure where a real Display transfer created `puzzle3d-main-2` in the dock but left its body empty until an unrelated example switch. The same run kept the closed top and perspective World3d owners visible in `dumpMeshStats`.

## Runtime evidence and cause

The sealed evidence is `📓️astra-checkpoint10-runtime.md`, `🗑️generated/astra-runtime/paired-checkpoint-10-full/steps.json`, and the two `25-window-reopen.png` images. Steps 25–41 created a new tab, then found no World3d target for orbit, pan, zoom, pick, or context. React created a distinct usable `puzzle3d-main-3` immediately.

The topology mutation changed `DockState` and persisted the layout, but it did not owe a guest UI refresh. The next example switch happened to request the Full scope and therefore published the first body. Closed documents and engine owners could remain observable until that delayed refresh/paint lifecycle ran.

The first native producer law also found an independent interaction defect. `finish_dock_drag` awaited the guest `noteShellCommand` during pointer release. A fixture with a real session and a nonrunnable guest remained inside `handle_pointer_button` for more than 143 seconds. Journalling is not a prerequisite for the new body's publication and must not own the pointer turn.

## Production contract

Every successful topology mutation now calls one `owe_window_topology_refresh` authority:

- Display template drop through `finish_dock_drag`;
- direct Display open through `open_display_window`;
- close through `close_dock_window`;
- open-active-in-new-window through `open_active_window_in_new_window`.

The authority records an owed Full scope and wakes the settle lane. `settle_pump_step_inner` services this debt before ordinary deferred actions. A successful `refresh_ui` reads the dock's current instance roster, retires documents outside it, asks the guest for each live body, installs the returned retained leases, and only then acknowledges the topology refresh.

The move journal uses the UI runtime's fixed-credit `BoundedActionQueue`. Admission or publication can refuse once without retaining an owner. Accepted entries remain parked until body refresh succeeds, then move exactly once into the existing deferred lane. The next settle step owns guest dispatch or refusal. This prevents a nonrunnable journal action from delaying or reordering the initial body while keeping pointer ingress bounded.

The production body/owner retirement path stays canonical. No lifecycle code injects a World3d state. The first Shell paint consumes the guest document, registers its World3d owner and physical hit, and `sync_engine_surface_states` determines the live roster. Close removes the generation from the active admitted map, invalidates its token, queues the exact old owner for progressive retirement, and retires the retained document. The retirement law drives that queue to terminal empty rather than weakening the World3d `Drop` assertion.

## Native laws

`🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs` keeps the complete real Display producer boundary:

`build_display_windows_ui` → panel projection → retained document reconcile/paint → published transfer handle → Shell down/move/hitless-up → one new dock instance.

It first isolates the retained publication and queue acknowledgement boundary: the real transfer payload creates `main-2`, admits one topology journal, a manually supplied retained body paints through the canonical Shell window walk, and close invalidates the generation and drains progressive retirement. This part is deliberately not described as guest refresh proof.

The same law then creates `main-2` again with another real Display gesture and crosses the actual producer path. A runnable, test-only ProgramBridge fixture supplies a retained World3d body through `settle_pump_step_inner` and `refresh_ui`; no `window_ui` body exists before that step. The law requires:

- first settle step publishes the new body and releases exactly one admitted move journal;
- the journal fixture has not run before publication;
- the following settle step refuses that move journal exactly once and retains no action owner;
- canonical Shell paint stages, publishes, and resolves a physical `HitKind::World3d` target before any example switch.

The ProgramBridge fixture seams are compiled only under `cfg(test)`. Production browser and native bridge backends are unchanged.

## Neutral fixture and React oracle

The language-neutral fixture and schema are:

- `🧫️fixtures/🪟️window-lifecycle-template-drag/🔣️.json`
- `🧬️schema/🪟️window-lifecycle-template-drag/🔣️.json`

The `publication` vector declares the closed instance, newly opened instance, mounted-after roster, expected fetched roster, body key, surface kind, and controller. The TypeScript law imports React's production `partitionRefreshWindowInstancesV1` helper and proves the newly mounted instance is fetched while the closed instance is skipped. The existing Ajv, independent layout, real React source, and renderer generation-token oracles consume the same fixture.

Focused command:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace -- bun test ./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🪟️window-lifecycle-template-drag/🟦️.ts
```

Result: **5 passed, 0 failed, 25 assertions**.

## Native18 reconciliation and validation boundary

Native17 exposed the synchronous pointer-release stall described above. Native18 compiled and executed 1,173 tests against the bounded topology refresh source, proving that stall is gone. It then exposed two owned assertions:

- the body law inspected the prior resolved hit registry even though the canonical paint had correctly staged the new World3d target; it now crosses the actual `publish_hits` frame boundary before inspecting resolved hits;
- a duplicate release after catalogue capture retirement was being reinterpreted as `canvasPointerUp` on the destination Canvas. Canvas release now requires the matching generation-owned `CanvasGesture`, so a release-only event cannot emit a pointer terminal action, click, or second drop.

Rustfmt check-only parser passes reached the changed ProgramBridge, Scene, Shell, and law files. They return nonzero because these large shared files already differ from rustfmt's preferred layout. No Cargo, native, wasm, generator, or browser job was started by this packet.

The lifecycle production source was compiled by Native18 and included in WGPU activation 11, whose renderer wasm passed. Root's paired checkpoint-11 runtime then passed the previously red sequence: close all → retained Display handle → new `puzzle3d-main-2` produced a live World3d hit immediately, followed by real orbit, pan, and zoom actions without an example switch. All five sealed artifact hashes remained unchanged.

Native19 later passed 1,172 of 1,173 tests; only the Display law's terminal World3d retirement assertion remained red. Terra's refusal audit then found that a per-body render error still released the accepted transfer journal. 📓️astra-sol-window-publication-refusal.md records the bounded required-lease retry, recovery and terminal-refusal follow-up. The paired runtime has closed the ordinary initial-body failure; refused-publication native acceptance remains owned by the next root gate.
