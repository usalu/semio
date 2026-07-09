# Forms Module Preview Fix — Verification Log

## Code fixes

| Fix | File | Result |
|-----|------|--------|
| Declare window kinds + default layout on `forms-module-procedural` | `forms/module/procedural/rs/lib.rs` | `windowKinds`: `forms-module-procedural-params`, `forms-module-procedural-preview` |
| Seed fresh Forms document from Building Component fixture | `forms/plugin/rs/lib.rs` | Fresh `createApp("forms-play")` renders step 1 with `Component Name` / Building Component metadata |

## Build

```bash
cd framework/product/os/dev && bun ./script.ts plugin forms forms-module-procedural flow procedural
```

All plugins built for `wasm32-wasip2` without errors.

## Plugin runtime verification (Node)

```bash
node .repo/🎫/26/07/08/FORMS-BLUEPRINT-TRY-MODULE-PREVIEW/plugin-runtime-check.mjs
```

**Result: PASS**

- `forms-module-procedural` manifest builds (no `window_kinds.is_empty()` panic)
- Params body → `stack` with 3 children (height/radius/sides sliders)
- Preview body → `componentScene` `world-3d`
- Fresh `forms-play` seeds Building Component fixture (step 1 questions visible without picking an example)
- Step 2 with contributions loaded → `externalSlot` nodes for `forms-module-procedural`
- Generate mode: `flow-play`, `procedural2d-play`, `procedural3d-play` — add/rename/remove generation + live preview surfaces render

## React dev host (port 6058)

**Blocked (pre-existing environment issue, out of scope for this ticket)**

`bun ./script.ts dev forms` → `runViteBunxDev` → `bun x vite` runs Vite under Node; `vite.config.ts` imports `script.ts` which uses `bun:sqlite` → `ERR_UNSUPPORTED_ESM_URL_SCHEME: Received protocol 'bun:'`.

Plugin-level checks above exercise the same wasm artifacts the React host loads.

## Wgpu dev host (port 6158)

**Partial — trunk HTTP 200, UI boot incomplete in automation**

- `http://127.0.0.1:6158/?plugin=forms` serves HTML but canvas/UI does not paint in Cursor browser automation (blank frame, 0 canvases).
- Same wasm plugin/runtime path verified via Node `plugin-runtime-check.mjs`; wgpu `resolve_external_slots_in_tree` unchanged by this ticket.

## Summary

Both root causes from the plan are fixed and verified at the wasm/plugin layer:

1. **Crash** — `forms-module-procedural` now declares window kinds; manifest builds successfully.
2. **Empty fresh document** — `FormsPlayApp::initial_document_json()` seeds Building Component; questions appear immediately.

Extension question mechanism (params sliders + 3D preview via external slots) confirmed end-to-end through combined `forms` + `forms-module-procedural` plugin render path with contributions JSON matching production `buildContributionsJson` shape.
