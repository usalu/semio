# Sol Independent P3 Atomic Family, Wake, and Legacy-Zero Second Re-Audit — 2026-08-23

## Audit admission

The requested Terra admission remained unavailable because of the shared scheduler limit. This is
the authorized independent Sol High second source re-audit. I did not author the P3 wake packet or
its evidence repair, and I made no production-source change. I read the first independent audit,
the rejected re-audit, the appended P3 implementation report, the live source, and the relevant
working/staged/`HEAD` diff.

Cargo, Nx, native tests, Wasm, browser execution, network work, root lint, and runtime timing were
not run. The Rust fixtures were inspected but not executed.

## Verdict

**ACCEPT — the isolated wake-evidence repair at source level.** The formerly permissive
`>= 4`/single-replacement guard is gone. The live predicate now enumerates all five named renderer
handoffs, requires one exact token field in each and five globally, separately requires the
presenter container chain and both host owners, and rejects every authorized adversarial mutation.

This does not accept Phase 3 or Phase 5. Both remain **RED** for the separately recorded ownership,
runtime, GPU, terrain-input, retirement, and timing residuals.

## Exact live ownership census

| Requirement | Result | Independent evidence |
| --- | --- | --- |
| Durable authority | PASS | The sole production `WorldCursorWakeAuthority::new()` is the `RuntimeMailboxInner` field initializer in renderer glue. The two other constructor matches are inside canonical World tests. Production builds each `World3dBuildContext` from `runtime.world_cursor_wake_authority()`; no production default-context recreation exists. |
| Five named glue handoffs | PASS | `AppFrameBuild`, `AppFrameAfterChrome`, `AppFramePresentation`, `AppFramePreparation`, and `AppPresentStep::Complete` each contain exactly one `cursor_wake: Option<WorldCursorWakeToken>` field. The exact global field-spelling census is five. |
| Presenter containment | PASS | `AppPresenter` contains exactly one `pending: Option<AppPresentCursor>`; `AppPresentCursor` contains exactly one `frame: AppFramePresentation`. The pending presentation therefore retains the exact typed token owner through the frame rather than extracting a boolean. |
| Host containment | PASS | `OsHost` contains exactly one `cursor_wake_requested: Option<WorldCursorWakeToken>`, and `OsHostRetirement` contains exactly one. The repository-local host-field census is exactly two. |
| Internal boolean erasure | PASS | The renderer glue/host census has zero `cursor_wake: bool`, `cursor_wake: Option<bool>`, `request_frame: bool`, `cursor_wake_requested: bool`, or `cursor_wake_requested: Option<bool>` internal fields. The browser output DTO remains the sole final-edge `request_frame` projection and is outside the internal glue/host census. |
| Exact ACK and ABA | PASS | `WorldCursorWakeAuthority::acknowledge` mutates only when the pending generation equals the non-`Copy` token generation. Closing, stale, and duplicate tokens return false. Native presentation acknowledges before transferring the exact token to `OsHost`. |
| Coalescing and loss prevention | PASS | `request` reuses the retained pending generation. Host replacement accepts only a strictly newer token, preventing a late older completion from overtaking it. Native/browser each have exactly one final token take. |
| Bounded close witness | PASS within the audited scalar authority | `OsHostRetirement` takes its retained token, then asks the presenter to take its pending frame token. The shared authority closes pending, current, and acknowledged generation scalars on distinct grants before terminal state; the terminal witness names all of them. |

## Independent adversarial mutation run

I independently reproduced the live `exact` predicate over the current glue, host, native, and
browser sources and applied the full mutation matrix. The unmodified baseline returned true. All
**18** mutations returned false:

1. remove the `AppFrameBuild` token;
2. remove the `AppFrameAfterChrome` token;
3. remove the `AppFramePresentation` token;
4. remove the `AppFramePreparation` token;
5. remove the `AppPresentStep::Complete` token;
6. erase all five typed handoffs;
7. inject an extra internal boolean wake channel;
8. inject a sixth exact token field;
9. erase `AppPresentCursor.frame`;
10. erase `AppPresenter.pending`;
11. erase the `OsHost` token field;
12. erase the `OsHostRetirement` token field;
13. recreate `World3dBuildContext::default()` per frame;
14. skip exact acknowledgement;
15. drop rather than retain the native token;
16. replace newest-generation comparison with unconditional acceptance;
17. skip the retained host token during close; and
18. observe the browser token without consuming it.

The live run reported `baseline=true`, glue token count **5**, host token count **2**, internal
boolean count **0**, and `allRejected=true`. This directly closes the single blocker in
`📓️sol-independent-p3-atomic-family-wake-legacy-zero-reaudit-2026-08-23.md`.

## Legacy-zero and atomic-family spot checks

- The exact authored-Rust scan returned zero legacy `struct Mesh3d`, `type Mesh3d`,
  `Mesh3d::from_buffers`, or `-> Mesh3d` spellings. `LegacyMeshOracleData` remains **17** references,
  confined to ui-scene's test module and canonical World's individually test-gated oracle/test
  module.
- The face overlay production path changes `face_overlay_generation` only on
  `WorldFaceOverlayMeshStep::Complete`. The permanent fixture supplies distinct marquee,
  hovered, and selected face IDs, observes a staged bucket before visibility, requires all three
  generation-qualified keys, and preserves generation 800 across a stale partial generation 900.
- The terrain production path inserts `terrain_built_tiles` only on cursor `Complete`; drawing
  checks that marker before exposing any band. `TERRAIN_COLOR_BANDS` remains ten, so partial
  generation output stays hidden.
- The wake fixture retains one authority across consecutive frames, coalesces a 128-request storm,
  proves duplicate/stale acknowledgement refusal, rearms to a greater generation, and drains its
  scalar close phases to the exact terminal witness.

## Executed static gates

| Gate | Result |
| --- | --- |
| edition-2024 scoped `rustfmt --check --config skip_children=true` | PASS on ui-scene math, canonical World, renderer glue, OsHost, native winit, browser Worker, and frame job |
| renderer-glue parser via `rustfmt --emit stdout` | PASS |
| `bun ./📜️script.ts verify interactivity --self-test --format json` | PASS; DENY clean |
| `bun ./📜️script.ts verify interactivity --format json` | PASS; DENY clean |
| independent 18-case mutation matrix | PASS; baseline true and every intended defect rejected |
| exact wake/host/boolean/authority/final-take scans | PASS: 5 / 2 / 0 / 1 production authority / 1 native + 1 browser take |
| legacy-zero scan | PASS: zero legacy type/constructor/return spellings; 17 test-only oracle references |
| scoped and whole working/staged/`HEAD` diff checks | PASS |
| Cargo/Nx/native/Wasm/browser/runtime timing | Not run; no build or runtime PASS claimed |

## Remaining Phase 3 RED residuals

- presenter-witnessed normal retirement of superseded face and terrain generations;
- bounded retirement of old and partial generation-qualified registry leases outside realm close;
- typed retained terrain input in place of the current JSON/serde materialization;
- dynamic `HashSet`/`HashMap`/frame-vector and pending-packet retirement;
- PNG/JPEG/MVT/SVG semantic jobs and bounded GPU table/upload/atlas/raster/cache replacement;
- opaque render, submit, and present timing plus complete presenter/GPU close;
- full realm terminal-empty proof across every renderer/runtime owner; and
- native timing and real Wasm/browser scheduling and close evidence.

The accepted scope is therefore only the corrected, meaningful source guard and its unchanged live
generation-token flow. No broader phase acceptance follows.
