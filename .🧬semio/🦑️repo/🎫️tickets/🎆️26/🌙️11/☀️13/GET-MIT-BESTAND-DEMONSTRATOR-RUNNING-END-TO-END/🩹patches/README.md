# Patches (not applied; nothing under 🧰️framework was edited)

| File | Destination | Apply |
|---|---|---|
| `script-inputs.diff` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs` | `patch` the one line in `closureApiV1().resolve` (a diff, not a full copy, because that file carries concurrent peer edits and `[DEBUG] hitcost` lines) |

The patch only skips `supportOf(layer)` when `derived` already holds the walk question (`derived.has(row.path)`), so the full layer read and parse happens once per walk question per `resolve` call instead of once per entry that names it.
Measured in `prof-osdev.mjs` (no row log): `resolveClosureRefs` 201 s warm with 112 s inside `layerBatchesSync` under `supportOf`; with a row log (the service) the same stage takes 10 s, so the gain applies to callers without a log only. Not measured after the change.

Tests to add (`🧪️tests/🌊️layer-stream/🟦️.ts`): resolve two recipes whose stored entries name the same fact layer, spy on `store.layerBatchesSync`, expect one read per digest and an identical `derived` list (language-agnostic oracle: the derived row paths and support lengths).

No patch for issue A or B: see `🕸️graph-slice-slowness.md`, section "Third session".
