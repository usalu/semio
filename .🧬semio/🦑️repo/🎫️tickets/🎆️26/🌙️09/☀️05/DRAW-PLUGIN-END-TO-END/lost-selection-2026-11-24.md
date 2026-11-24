# Lost layer selection at Draw browser boot (2026-11-24)

## Root cause: test bug (weak readiness check), not host/guest

The journey waited for `waitLayers(minimumBootLayers = 1, minimum)` and then clicked
`layersPanel ... [role=treeitem][id^="...drawing-play-layers."]:not(add.*):not(empty)` `.first()`.

The playground boots with the placeholder document (one layer row `path.initial-layer`) and only then
applies the `Demo` example via `setActiveExample`, which replaces the document with rows
`path.emblem-orange|red|teal`. The engagement status reads `1 layer · 0 selected` for both
documents, so the `>= 1` check is satisfied by the placeholder. The click lands on `initial-layer`,
the host dispatches `interactionSelect` for a layer that the Demo document then replaces, and the new
rows are unselected, so the inspector stays on "Select a layer ...". Heavy CPU load only widens the window.

Evidence (probe `probe-lost-selection.mjs`, mode `old`, no throttling): 3 of 4 iterations failed.
The DOM event log showed `rows: initial-layer` at t=4820, click on `...path.initial-layer` at 5072,
and `rows: emblem-*` at 5528 (8764 in another run). The guest logged
`[DEBUG] command turn interactionSelect`, so the click reached the row and the host did dispatch it.
In the passing iteration the rows were already `emblem-*` when clicked.

The product behaviour is correct: a selection on a layer of a replaced document is void.

## Fix

- `✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🔣️.json`: new `exampleLayer: "emblem-orange"` (a layer of the Demo document).
- `.../📐️.json`: `exampleLayer` required, const `emblem-orange`.
- `.../🟦️.mjs`: the row locator is now `[id="panel:framework.panel.artifact/drawing-play-layers.path.${fixture.exampleLayer}"]`.
  Playwright's click waits for that row to exist, so the actual readiness signal (Demo loaded) gates the click; no retries or sleeps.

## Verification

- Probe, 5x CPU throttle (CDP `Emulation.setCPUThrottlingRate`), fixed locator: 16 iterations, 0 failures.
- Full en journey (`browser-en-l1`): the `boot` step passes; the journey then passes `blank`, `example`, `attach` and fails at the
  `utilityBar.unfold` click (the "Display" canvas-footer label intercepts pointer events), a later, unrelated failure.

## Rebuild

None. Only the test and its fixture/schema changed.
