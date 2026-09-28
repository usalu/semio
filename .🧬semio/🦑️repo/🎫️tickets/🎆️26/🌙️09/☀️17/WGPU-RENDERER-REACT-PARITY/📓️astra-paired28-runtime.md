# Paired Renderer Build 28 Runtime

Build 28 loaded in the existing 1280×720 browser view with unchanged Puzzle activation and guest artifacts. Its WASM is 135944844 bytes, SHA-256 `6da73dd89f244e48b7d083dc813d708cf3846579aa3faad7ff04ad3c8445b57e`; the remaining eleven seal entries match build 27.

The Projection menu visibly paints all row labels, icons and selection. The opaque identity-row overpaint is resolved in this browser run. Selecting Curvilinear updates the title, selected accessible row and rendered projection. Removing the synthetic wheel retains the large roof framing observed in React, replacing build 27's much smaller view. This is a qualitative match, not pixel equality: the two renderers still differ in lighting, edges and exact framing.

The now-visible menu exposed a remaining ordering error: WGPU placed Curvilinear at the top and Parallel at the bottom, whereas React keeps its default Tree direction down inside the bottom-right pane. Source repair changes the projection body's block flow to Down while retaining Rtl inline flow. The neutral pane fixture now records these axes; React is tested inside an actual up/rtl FlowProvider, and native geometry now checks row identities in declared order instead of only ascending hit registration positions. Native and rebuilt browser verification of this repair are pending.

Numeric-hyphen labels also differ visually: the literal `1-Point` is rendered as `Point-1` by React's RTL DOM but remains `1-Point` in WGPU. Canonical labels are identical in source; this is a bidi presentation gap, not a taxonomy naming change. It remains open.

No browser warning/error logs were returned during these checks. HMR remains on despite the server config flag because the shared `serveVite` inline config supplies its own HMR object; the earlier one-line renderer config alone does not disable it. No stability claim is made for that setting.

The strengthened React projection suite passed 2/2 on 2026-09-27 around 23:48 UTC (Vitest 3.94 seconds; Nx 5.1 seconds, exit 0). Native geometry and browser rebuild remain pending.

## Selected Foreground Mismatch

A DOM-backed read of React’s selected Curvilinear row reports `rgb(247, 243, 227)` for its row and button text. WGPU visibly uses dark text on the same active fill. WGPU consumes generated `chrome.active_foreground`; both default light/dark Rust token sets resolve it to the dark palette token, matching current styling JSON. The handwritten `.dark` CSS rule in `styling/🖌️ui/🎨️.css` instead sets `--active-foreground: var(--color-light)`. Canonical styling JSON’s dark appearance explicitly uses the dark token (the existing contrast corpus covers activeForeground/activeBase). This is a cross-renderer theme-authority inconsistency; changing the Rust literal alone would contradict the current schema and contrast rules. No palette production change was made during this finding.
