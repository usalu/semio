# Current Draw Entry And Acceptance Audit

Read-only review of the ticket definition, retained wiring, runtime and completion audits. No source edits or tests were performed. Historical pass reports do not establish current-source results.

The ticket requires native and wasm32-wasip2 plugin checks, native tests, a fresh component-derived descriptor pair and registry check, a real passing Draw TypeScript package, and the React Draw development entry on port 6064 with nonempty canvas and panels, example switching and functioning editor actions.

The existing route selects Draw as the default session and exposes the drawing.play.composite canvas surface. The retained wiring audit finds aligned pointer commands, typed actions, active-window utility lookup, editor and viewer registration; no further registration gap is established without execution.

The browser acceptance target exercises Demo, blank and Demo switching, rectangle creation, undo and redo, PDF export with independent PDF.js/canvas painting and pixel comparison, and reload after real folder persistence. Run separately for English and German and inspect screenshots. Current completion audit records these journeys and current native, TypeScript, component, descriptor and registry gates as unproven.

Canonical retained routes include repo run @semio-tech/draw-drawing-rs:test --lang=en, its separate check target, repo run @semio-tech/draw-js:test --lang=en, the framework-os-dev activation target, and draw-js:browser-acceptance with explicit serve, locale and ticket-generated output arguments. Verify current graph and source before execution because prior reports describe different snapshots.
