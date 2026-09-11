# wgpu `shell-boot` — why the phase is silent, and what now speaks

Lane: renderer engineer — wgpu plugin bridge / shell-boot phase.
Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, 2026-09-10.

Live finding this lane answers (`📓️runtime-verification-2026-09-09.md`, wgpu boot #6, 12:20–12:35,
box load 90–118, `http://localhost:6118/?plugin=generation3d`):

```
shell-boot 86 %      reached after ≈170 s
… then nothing. No further progress, no fault card, no console line at all, for ≥5 minutes.
```

On React the same build logs `actor trapped` on every extension-result delivery and restores the actor;
on wgpu the plugin bridge is silent.

## 1 Why `86 %` is exactly one opaque `await`

`shell-boot` is ONE phase name covering ONE promise.

| site | what it is |
|---|---|
| `🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:643` | boot phase 6 → `BootPhase { stage: "shell-boot", progress: 0.7, shell_boot: true }` |
| `🎞️frame-worker/🟦️.ts:489` | `progress(step.stage, 0.65 + step.progress * 0.3)` → `0.65 + 0.7 × 0.3 = 0.86` — the banner's `86 %` |
| `🎞️frame-worker/🟦️.ts:490` | `await monitoredSuspension("shell-boot", () => bootstrap.bootShell())` |
| `🌐️browser-worker/🦀️.rs:655` | `boot_shell` → `ShellState::boot().await` |

Everything the guest does for the whole boot — every actor's `create_app`, its first step, its
contributions, its example, and the first `renderDocument` for every window and panel surface — happens
inside that single `await`, under a single declared phase whose ceiling is 900 000 ms. There is no
intermediate `progress`, no per-actor declaration, and nothing on the TypeScript side of the bridge
prints anything at all between `create_app` entering and the whole boot returning. `86 %` is therefore
not a stage the boot stopped at; it is the ONLY value it can ever show while the guest works, and the
watchdog is — correctly, by `📓️wgpu-boot-watchdog-2026-09-10.md`'s law — not allowed to touch it.

