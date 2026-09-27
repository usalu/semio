# Tutorial Self Scheduling

The WGPU tutorial runtime now owns a bounded nominal 60 Hz wall-clock deadline while playback, recording, or camera convergence is active. `ShellState::next_tutorial_deadline_ms` derives the next cadence from `last_tick_wall_ms`, clips playback to its exact rate-scaled terminal wall time, clips convergence to its exact terminal wall time, and returns no deadline for an absent, paused, deviated, stopped, or completed runtime without convergence. `next_chrome_deadline` takes the earliest Shell chrome and tutorial wall deadline before translating the remaining duration onto the monotonic scheduler clock.

The existing tutorial bridge fixture and strict schema now carry the renderer-neutral clock law: frame cadence, rate-scaled playheads, terminal pause, per-mode deadline cases, convergence, and Chrome/tutorial deadline merge. The exported React `createTutorialClock` oracle uses a controlled `requestAnimationFrame` queue and proves there is one pending frame only while active, exact terminal clamping, and cancellation on pause and dispose.

The native law `tutorial_deadline_self_schedules_playback_recording_convergence_and_stops_at_terminal_states` reads the same fixture. It covers playback, recording, paused and deviated modes, exact playback completion, paused convergence completion, explicit stop, Chrome/tutorial minimum selection, wall-to-monotonic conversion, and post-tick rearming independent of candidate-frame acceptance. Renderer-owned eager publication after `tutorial_tick` and its discard isolation law are maintained by the root runtime lane.

Validation:

- Focused Nx/Vitest oracle: 2 passed, 3 skipped, one file passed. Command: `NX_DAEMON=false bun nx exec --projects=workspace -- bun x vitest run <tutorial-bridge-test> --config <wgpu-vitest-config> --silent=false --reporter=verbose -t 'self-schedules the exported tutorial clock until the neutral terminal frame|validates the neutral round-trip'`. Receipt: `🗑️generated/sol-tutorial/oracle-2.log`.
- Both changed Rust files parse under Rust 2021; the focused native law is queued with the root-owned renderer scheduler laws.
- An initial React target invocation correctly refused the test because the hybrid cross-renderer oracle is registered in the WGPU Vitest suite. The successful run used the registered WGPU config directly through Nx.
