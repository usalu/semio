# React Runtime Verification

## Runtime

- URL: `http://127.0.0.1:5173`
- Title: `Entwerfen mit Bestand · Demonstrator`
- Browser: Codex in-app Chromium runtime
- Capture: `🖼️react-runtime.png`

## Verified Invariants

- Fourteen live `mode-dock-stack` surfaces were present.
- Every sampled stack contained exactly one active clipped content plane.
- Every sampled content plane had reached `data-silhouette-state="ready"`.
- The computed clips were concave polygons matching the measured chip, gap, controls, and body geometry. Example: `polygon(0px 0px, 87.453px 0px, 87.453px 22.391px, 713.234px 22.391px, 713.234px 0px, 863.891px 0px, 863.891px 656.031px, 0px 656.031px)`.
- Every tab chip and controls chip sampled had `ui-glass`.
- Every stack exposed a semantic `tablist` with one selected native tab.
- Gap computed paint was transparent with no backdrop filter.
- Gap computed pointer behavior was `none`; `document.elementFromPoint` at its center resolved outside the stack, proving the cutout passes hit testing to the underlying surface.
- Stack pointer behavior was `none`; interactive chips restore pointer handling explicitly.

## Console

The silhouette compositor itself emitted no error. The running aggregate demonstrator contained existing unrelated command-channel errors and `[DEBUG]` diagnostics from `PluginRuntime` and `ShellHost` for empty or undeclared actions. These were already outside the owned silhouette regions and remain recorded as an aggregate-runtime blocker rather than being attributed to this feature.

