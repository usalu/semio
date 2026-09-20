# Actions Mounting Lifecycle Audit

Read-only source audit after Native48. I did not run a build, test, or activation.

## Finding: Missing Window Body Must Not Suppress Window Chrome

**Status:** resolved in the current source; high confidence.

The dock plan creates one `MainWindow` entry for every mounted window before a retained body is read. In the missing-body arm, `render_main_window_step` resets only the per-document cursor and sends the *same* `cursor.item` to phase 8; it does not increment the item or return to phase 3. Phases 8–13 then independently process measures, pane chips, utilities, projection, Actions, and Search before phase 13 increments the item. Therefore a body that has not arrived cannot suppress the pane chrome of its already mounted window.

- WGPU missing-body branch: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:23335-23340`.
- Subsequent chrome phases: `.../🦀️.rs:23382-23484`.
- The phase-12 and phase-13 pane bodies are separately optional: `paint_window_pane_body_step` returns complete when folded, too small, or without its own pane document (`.../🦀️.rs:19018-19023`). This preserves the missing *main* body distinction without inventing a fake retained document.
- Window Options is likewise independently absent/folded-safe (`.../🦀️.rs:4574-4579`), while chip availability derives from its own pane-specific state (`.../🦀️.rs:17998-18002`).

This is the necessary repair for the prior behavior described in the native failure: taking no main-window document previously advanced to the next window and skipped phases 8–13, yielding a lease/chrome walk with neither UI-document ingress nor pane controls.

## React Reference

React mounts the window descriptor, its chrome, and a body placeholder independently of guest body delivery. `modeWindows` supplies each normal and extra window with `actionPane`, the fold callback, and `<WindowBodySkeleton />`; the nested `InterpretedUiNode` uses `PENDING_WINDOW_UI_NODE` if its per-window document is absent.

- Base window descriptor and fallback: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:10767-10783`.
- Extra instance equivalent: `.../🟦️.tsx:10806-10828`.
- The placeholder is deliberately a stable loading body: `.../🟦️.tsx:1945-1949`; its visual component is a busy status skeleton at `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🦴️Skeletons/🟦️.tsx:116-122`.

The WGPU repair matches the behavioral boundary: window chrome remains reachable during an absent asynchronous body. It does **not** claim visual skeleton parity, which is separate from preventing the missing body from hiding controls.

## Final Source-Risk Check

No new ordering or ownership issue is evident in the repaired branch.

1. The absent-body arm does not remove or restore an owner, so it cannot consume a document lease or create an ingress generation.
2. It resets `cursor.document`, `scalar`, and `flag` before phase 8, preventing a prior window's stepped document cursor from contaminating the current window's overlays.
3. The increment remains only after both Actions and Search body phases complete (`.../🦀️.rs:23472-23484`), so per-window FIFO and overlay stacking are preserved.
4. The complete frame still passes through `PaneOverlayHits` after `MainWindow`, which drains pane chips in paint order before later docked panels (`.../🦀️.rs:22539-22574`).

## Checkpoint 16 Receipt Criteria

Review the activation artifacts as one settled paired run, not separate screenshots:

1. Seal and later recheck the five WGPU generated artifacts, as required by `📓️astra-checkpoint16-plan.md:109-117`.
2. With equal explicit locale, appearance, density, viewport, and DPR, capture the normal dock before any guest body is known ready. Every mounted pane must still publish its Actions/Search/Utilities control availability according to its own state; an absent body must not produce a surface fault or suppress the otherwise eligible Actions chip.
3. Open Actions through its physical pane chip, then retain a state receipt showing the instance-specific fold transition and Action document/control ids. Scroll to the initially clipped Abort row, click it physically, and require the `engagementAbort` action receipt; reverse scrolling must restore the initial clipping and 24px row pitch.
4. Capture the same state/screenshot sequence against React. Treat a geometry discrepancy separately from an action-ledger discrepancy, as the checkpoint plan requires (`📓️astra-checkpoint16-plan.md:31-36`).

These are review criteria only. I have not inspected a new activation artifact in this audit.
