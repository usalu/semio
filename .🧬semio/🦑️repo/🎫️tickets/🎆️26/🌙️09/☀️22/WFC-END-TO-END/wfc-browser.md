# WFC Browser Scenario Inputs

Inputs are authored in this ticket, without copied repository source. `wfc-browser-cases.json` is the portable case table; `wfc-browser-schema.json` is its JSON Schema. Five small modules export `timeTravelScenarioV1` through a common factory, using the existing framework time-travel runner.

## Coverage

- Step 101 stages named action forms through the existing matrix input interface and requires a new document edit. Every variant changes a seed; tiled/graph variants create a tile; grid3d also changes its colour. Exact scoped slider keyboard input prevents the matrix helper's generic first-slider fallback from seating the wrong colour.
- Step 102 starts fill through the public tool controls, observes public ARIA progress, pauses and verifies a held progress value when a nonterminal run is observable, resumes, finalizes, dismisses and compares preview screenshots. It starts another run and aborts immediately while Starting, Running, Paused or Complete. It requires an Aborted status and no authored document edit; abort of Complete is explicitly reported as cancellation before finalization. Every solve has a 120-second deadline and abort cleanup.
- Bitmap also invokes the public no-argument `pin-preview` action and requires one authored edit. Other artifacts intentionally persist models/seed/pins and derive their preview.
- Step 103 attaches a fresh ticket-generated folder through the existing history-session interface, independently observes a nonempty folder, reloads the page, then requires the authored ledger rows to return.
- Step 9 is the framework's shared console/fault digest. Harness owns Chromium, fresh locale contexts, launch/reuse, cancellation, reports and screenshots.

## Validation Actually Run

`repo tools exec --light bun -- ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️22/WFC-END-TO-END/wfc-browser-check.ts"`

Result: Ajv validates the language-neutral case table, and all five scenario modules load successfully with their intended variants, ports and ordered steps. Node's TypeScript syntax check also passes. This is input/module validation, not an executed browser acceptance result.

R2 consolidated native session88441 and bitmap browser session55225 are live after admission, waiting for the shared dependency builder. Previous sessions57596 and61555 terminated before compilation. No native pass or browser pass is claimed.

## Commands

Run from repository root. With no serve listening, timeTravel calls activatePlaygroundRuntime before spawning the admitted variant, so it prepares fresh source automatically. If reusing a ready serve, first start that variant with repo run @semio-tech/framework-os-dev:dev-VARIANT-react-prototype --lang=en in another terminal. The dev target owns activation/preparation; warm-only serve-VARIANT does not. ensureDevServe returns immediately for a ready reused serve and does not invoke its beforeSpawn activation hook on reuse, so a warm stale server alone is not fresh-source evidence.

```powershell
repo run @semio-tech/framework-os-dev:time-travel --lang=en -- --universal --variant bitmap --serve http://127.0.0.1:6041/ --renderer react --locales en,de --only 101,102,103,9 --scenario ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️22/WFC-END-TO-END/wfc-browser-bitmap.ts" --out ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️22/WFC-END-TO-END/🗑️generated/browser-bitmap"

repo run @semio-tech/framework-os-dev:time-travel --lang=en -- --universal --variant grid2d --serve http://127.0.0.1:6042/ --renderer react --locales en,de --only 101,102,103,9 --scenario ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️22/WFC-END-TO-END/wfc-browser-grid2d.ts" --out ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️22/WFC-END-TO-END/🗑️generated/browser-grid2d"

repo run @semio-tech/framework-os-dev:time-travel --lang=en -- --universal --variant wfc2d --serve http://127.0.0.1:6043/ --renderer react --locales en,de --only 101,102,103,9 --scenario ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️22/WFC-END-TO-END/wfc-browser-wfc2d.ts" --out ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️22/WFC-END-TO-END/🗑️generated/browser-wfc2d"

repo run @semio-tech/framework-os-dev:time-travel --lang=en -- --universal --variant grid3d --serve http://127.0.0.1:6044/ --renderer react --locales en,de --only 101,102,103,9 --scenario ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️22/WFC-END-TO-END/wfc-browser-grid3d.ts" --out ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️22/WFC-END-TO-END/🗑️generated/browser-grid3d"

repo run @semio-tech/framework-os-dev:time-travel --lang=en -- --universal --variant wfc3d --serve http://127.0.0.1:6045/ --renderer react --locales en,de --only 101,102,103,9 --scenario ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️22/WFC-END-TO-END/wfc-browser-wfc3d.ts" --out ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️22/WFC-END-TO-END/🗑️generated/browser-wfc3d"
```

The explicit `--only` is required: the existing runner changes a `--universal` run without `--only` to steps 24 and 9, silently excluding plugin scenario steps. Universal mode also lets the boot gate accept the two-window WFC layout; the default scenario desktop gate otherwise demands three windows.

## Exact Limits

This scenario currently targets React. WGPU is refused with a named prerequisite because these observers consume public DOM/ARIA rather than the WGPU mirror. No private session or guest transport is used.

A tiny default graph may finish before Pause or Abort can be observed. Such a run records pause-not-observed, pause-raced-with-completion or abort-not-observed as a note and does not claim coverage of that phase. A larger prepared problem may allow nonterminal control coverage to be observed; scheduling still prevents a timing guarantee. The default examples are intentionally kept small and bounded here.

The preview screenshot comparison proves rendered pixels survive dismissing the finalized run; it does not inspect semantic tile assignments. Native fixture/oracle tests own semantic assignment correctness. Durable bitmap keeping uses the new public `pin-preview` action; the script never manufactures a solved pixel payload.

Folder reload checks authored ledger persistence and independently checks files were written. It does not claim that transient solve caches survive reload. The complete runner still needs an admitted browser/guest build and actual UI execution; host capacity has not admitted that verification in this session.

## R2 Input Corrections

The journey explicitly activates each author window before action forms and the bitmap output window before Pin Solution. The latter window prefix is declared in the portable case schema. Grid3d changes its newly created tile to a different colour, so its +1 edit assertion is meaningful. Complete tool runs remain legally abortable before finalization and now exercise that public cancellation route even when a tiny search outruns browser sampling; mid-search pause/cancel coverage remains separately qualified. Ajv schema and all five scenario imports pass after these corrections.
