# 📝️ haiku-consumer-audit: React and Site Viewpoint Propagation (relayed by the coordinator)

This is a static trace; nothing was run. `R` = `🎯️targets/⚛️react/🔨️modules`. The site target renders no drawings; it only mounts the app.

## Already correct

These components pass an explicit viewpoint: compare, the viewer stage and frames, the sheet cells, placed figures, playground, browse and overview cards, the custom element, the export panel, its snippet, and the playground export.
- A viewpoint change keeps the pose, clip, time, scale and representation in the viewer, compare and playground.
- URL state round-trips `vp`, `vl`, the compare pane tokens and the playground `vp`/`m`.
- Every download filename and its metadata carry the viewpoint.
- The selector has a live region.

## Findings, all routed to sonnet-viewer

| Severity | Where | Finding |
|---|---|---|
| High | `R/🏠️overview/🟦️.tsx:91` | `Frieze` hard-codes `viewpoint="elevation"`, so it ignores the preview preference (`:166,203,235,276`) |
| Medium | `R/📤️downloads/🟦️.ts:105`, `R/🧮️pipeline/🟦️.ts:32,100` | `viewpoint` is optional. A missing viewpoint names the file `undefined` and silently draws the elevation (violates D15) |
| Medium | `R/🔬️detail/🟦️.tsx:75` | The compare link drops the pose or clip selection and the time |
| Low | `R/🧍️figure:25,117`, `R/📍️placed:82-84` | `viewpoint` is optional, and an evaluation error leaves an empty slot with no message |
| Low | `R/🗺️routing:200,287,321`, `R/💾️persistence:179` | An invalid `vp` or stored preference falls back to elevation silently, unless the page runs the problem check |
| Low | `R/🔬️detail/📤️export-panel:91` | The lede does not show the viewpoint |
| Low | `R/🧪️playground:325-326` | A mirror on front or plan is kept in state but not drawn; it should be refused |
| Gap | — | No React test covers the viewpoint paths |
