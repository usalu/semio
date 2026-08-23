# Sol Independent Re-Audit: Phase 3 Atomic Family, Wake, and Legacy-Zero Packet

Date: 2026-08-23

## Audit Admission

The coordinator requested an independent Terra/high audit, but Terra admission was blocked by the
shared scheduler limit. This is the explicitly authorized independent Sol High re-audit. No
production source was edited. Cargo, Nx, Wasm, browser execution, network work, root lint, and
runtime timing were not run.

## Verdict

**REJECT — source packet evidence.** The live wake remediation itself is source-coherent: one
durable authority survives frames, the non-`Copy` generation token reaches the presenter and host,
exact acknowledgement precedes host retention, and native/browser consume the retained directive
once at their final edge. The legacy-zero, complete face-family, partial-terrain hiding, and realm
scalar-close properties remain coherent.

The permanent adversarial fixture is not meaningful for its required token-to-boolean mutation,
however. The source guard accepts that mutation, so this packet cannot use it as evidence that
future internal token erasure is denied. Phase 3 remains **RED** independently of this verdict.

## Re-Audit Results

| Requirement | Result | Evidence |
| --- | --- | --- |
| One durable wake authority | PASS | `RuntimeMailboxInner` owns the sole production `WorldCursorWakeAuthority::new()` instance. Each `World3dBuildContext::new(runtime.world_cursor_wake_authority())` receives only a shallow `Arc` clone. Production has no `World3dBuildContext::default()` recreation. The two other constructor matches are under the canonical World's test module. |
| Full generation-token flow | PASS | `WorldCursorWakeToken` derives neither `Copy` nor `Clone`. `Option<WorldCursorWakeToken>` is retained through `AppFrameAfterChrome`, `AppFrameBuild`, `AppFramePreparation`, `AppFramePresentation`, `AppPresentStep::Complete`, and `OsHost.cursor_wake_requested`. No internal `cursor_wake: bool`, `Option<bool>`, or boolean host field exists. The only `request_frame: bool` is the browser output DTO at the final platform edge. |
| Exact acknowledgement and ABA | PASS | `acknowledge` mutates only when `pending_generation == token.generation`; closing, stale, and duplicate tokens return `false` before mutation. It clears that exact pending scalar and records the acknowledged generation. The token is non-`Copy`, and `build_and_publish_snapshot` borrows it for acknowledgement before moving it into host retention. The consecutive-frame fixture proves a later admitted wake has a greater generation and that a duplicate/stale token cannot consume it. |
| Coalescing and wake storm | PASS | While a generation is pending, `request` returns that generation without advancing authority state. The fixture issues 128 requests and verifies one generation, carries the same pending generation through a consecutive frame, then acknowledges and rearms to a strictly newer generation. `OsHost::retain_cursor_wake_directive` replaces only with a greater generation. |
| Native final edge | PASS | Presentation completion acknowledges and retains the exact token, then invalidates `RESOURCE_READY`. `about_to_wait` requests the redraw and has exactly one authored `host.take_cursor_wake_directive()` call, guarded by the consumed invalidation reason. |
| Browser final edge | PASS | Browser presentation uses the same `OsHost` completion path. `BrowserTickOutput.request_frame` is the sole boolean projection and invokes exactly one `host.take_cursor_wake_directive().is_some()` before the other continuation conditions. |
| Exact wake close | PASS | Host retirement first removes one retained host token, then removes one presenter token, then drives the shared authority. Its four close grants clear pending generation, current generation, acknowledged generation, and terminal state separately. `terminal_is_empty` requires terminal plus all three scalar witnesses empty/zero. Frame-build close already drains frame/preparation tokens before runtime retirement. |
| Legacy `Mesh3d` zero | PASS | Exact `rg` over authored Rust returned zero `Mesh3d`, `Mesh3d::from_buffers`, or `-> Mesh3d` matches. `LegacyMeshOracleData` occurs only in ui-scene's `#[cfg(test)] mod tests` and the canonical World's individually `#[cfg(test)]` oracle plus test module. Both oracle paths materialize through paged writer/seal APIs before consumption. |
| Atomic three-category face family | PASS | The fixture uses three distinct face identifiers routed to marquee, hovered, and selected categories. It proves a staged bucket is invisible, all three generation-qualified keys exist before `face_overlay_generation` changes, and an interrupted stale superseding generation leaves generation 800 visible. Live publication failure restores the exact rejected lease, key, phase, and retry state into the cursor. |
| Partial ten-band terrain family | PASS | `TERRAIN_COLOR_BANDS` is ten. `terrain_built_tiles` changes only on cursor `Complete`, after band ten and source retirement. Draw construction skips the whole tile while the marker is absent, so any partial band set remains hidden. This property and its complete-marker fixture are unchanged from the prior accepted source inspection. |
| Realm scalar retirement | PASS | Dynamic World close handles an active face cursor first, clears one of three color options per grant, then visible and retired generation scalars separately; terminal-empty names every witness. Wake close likewise retires one scalar per call as described above. |
| Adversarial wake fixture | **FAIL** | `exact` accepts four or more typed frame fields, while the live glue contains five. The mutation uses single-occurrence `replace("cursor_wake: Option<...>", "request_frame: bool")`; it therefore leaves four typed occurrences and the independently required typed `AppPresentStep::Complete` spelling. Every other predicate remains true, so the following `assert!(!exact(...))` would fail if executed. The claimed token-erasure mutation does not reject its target defect. |

## Exact Blocking Repair

The source guard must enumerate every required typed handoff or require the exact current count and
must mutate the intended specific handoff (or all internal handoffs). A mutation that changes any
one of after-chrome, build, preparation, presentation, or presenter completion to a boolean must
make the guard return `false`. The fixed guard should also retain the existing recreation, missing
acknowledgement, lost token, stale/newest coalescing, close-take, and browser non-consuming
mutations. No live production behavior needs changing for this isolated finding.

## Static Gates

- Edition-2024 scoped rustfmt check: PASS for ui-scene math, canonical World, `winit_app.rs`,
  `frame_job.rs`, `os_host.rs`, and `browser_worker.rs`.
- Renderer-glue edition-2024 parse through `rustfmt --emit stdout`: PASS.
- Exact legacy scan: zero `Mesh3d` type/constructor/return spellings.
- Wake scans: one production authority construction; five typed glue handoffs; zero production
  `World3dBuildContext::default()` or internal boolean wake fields; one native and one browser final
  token take.
- Scoped and whole working, staged, and HEAD `git diff --check`: PASS.
- This new report's independent `/dev/null` whitespace check emitted no errors (exit 1 only because
  the file is new).
- Rust fixtures were inspected but not executed because builds were prohibited.

## Remaining Phase 3 RED Residuals

- Presenter-witnessed normal retirement for superseded face and terrain generations.
- Bounded retirement of old/partial generation-qualified registry leases outside realm close.
- Typed retained terrain input; current JSON/serde materialization remains indivisible.
- Dynamic HashSet/HashMap/frame-vector and pending-packet retirement.
- PNG/JPEG/MVT/SVG semantic jobs, GPU table/upload/atlas/raster/cache replacement, opaque
  render/submit/present timing, and complete presenter/GPU close.
- Full realm terminal-empty across every renderer/runtime owner.
- Native timing plus real Wasm/browser scheduling and close evidence.

This re-audit does not accept Phase 3, Phase 5, or the broader runtime matrix.
