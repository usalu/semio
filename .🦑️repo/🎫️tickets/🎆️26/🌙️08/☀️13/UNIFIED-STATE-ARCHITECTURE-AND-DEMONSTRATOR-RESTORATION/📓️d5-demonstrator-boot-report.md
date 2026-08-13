# D5 — Demonstrator boots again

**Status: the demonstrator page renders and its apps run.** Verified in the browser at
`http://localhost:6029`: six panes, **0 crashed**, 5 live canvases, no console errors other than the
benign `/plugin-modules/watch` + `/extensions/watch` hot-reload polling 404s.

The reported symptom was "the demonstrator page is still empty". It had **three** independent
causes, stacked. None was the one the earlier waves predicted.

## Cause 1 — the disk was full (the actual root cause)

`df` reported **257 MB free of 926 GB — 100% full.**

That explains what looked like unrelated failures:

- The dev server's **esbuild service had died** (`"The service is no longer running"`), so
  `📦️index.tsx` returned **HTTP 500** and the React root never mounted. That is literally the empty
  page.
- `cargo check` was failing with `No space left on device` — and that is why
  `semio-s-plugin-procedural`'s error count appeared to swing wildly (94 → 16 → 116) across the
  session. A large share of those "errors" were failed writes, not real compile errors. Earlier
  conclusions that read the swing as pure peer churn were partly wrong.

Consumed by per-ticket cargo build directories: 122G + 118G + 25G + 18G + 10G across ticket
`🎯️target/` dirs, plus 117G in the repo's own `target/`.

**Freed 202 GB** by deleting only regenerable caches:
- `target/debug/incremental` (**80 GB**) — incremental-compilation cache, disposable by design,
  never a build output.
- the `🎯️target` of the one **closed** ticket (INTRODUCE-INFERENCE-SCHEMA-FAMILY…, 10 GB).

Deliberately **not** touched: the `🎯️target` dirs of the four still-open peer tickets (283 GB). They
are regenerable too, but deleting them would cost those sessions 30–90+ min of rebuild each; that is
their call, not mine. **If disk pressure returns, that is where the space is.**

## Cause 2 — an unguarded optional field took down the whole shell

With the server healthy the page rendered, but **all six panes** died with
`TypeError: Cannot read properties of undefined (reading 'join')`.

Captured the real stack by installing a `console.error` hook and forcing a remount via **HMR**
(which preserves `window` state, unlike a reload — a plain reload wipes the hook before React
mounts). It pointed at:

```ts
// ShellHelpers/🟦️component.tsx
export function appBreadcrumb(breadcrumb: readonly string[]): string {
  return breadcrumb.join(APP_BREADCRUMB_SEPARATOR);          // ← unguarded
}
export function resolveAppBreadcrumb(app, terminology): readonly string[] {
  return app.terminologyBreadcrumbs?.[terminology] ?? app.breadcrumb;   // ← can be undefined
}
```

`AppDefinition.breadcrumb` is **declared optional** (`breadcrumb?: readonly string[]`,
`🛂️manifest/🟦️component.ts:839`) and every consumer dereferenced it anyway. This runs inside
`FrameworkOsShellInner`'s **render**, so a single app whose manifest omits the field destroyed the
entire shell — and in a multi-pane host, all six panes at once.

Fixed by accepting `undefined` and degrading to an empty breadcrumb: an app with no breadcrumb
renders a nameless title, it does not kill its host. Regression test added to the existing suite
(`🧪️index.test.ts`, "survives an app whose manifest declares no breadcrumb at all").

This is a genuine type-level bug, not a workaround for stale WASM — the type said optional, the code
assumed required.

## Cause 3 — a 5-day-old dev server

The process holding port 6029 had been up **5 days, 20 hours**. Restarted it; the plugin registry
regenerated cleanly on boot.

## Verification actually performed

| Check | Result |
|---|---|
| Page loads, React root mounts | ✅ |
| Six panes render, none crashed | ✅ `crashedPanes: []` |
| Live canvases | ✅ 5 |
| Console errors | ✅ only `/plugin-modules/watch` + `/extensions/watch` polling 404s |
| Generator pane opens full-screen | ✅ live node graph (Poly → Extc, wire/solid ports), minimap |
| Guided tour runs | ✅ "Der Ablauf-Editor" step 1/2 |
| Node graph drives 3D preview | ✅ extruded column + axis gizmo in the Vorschau window |
| Renderer suite | 301 passed / 6 failed — **none mine**: 4 content-drift assertions (command categories, brand tour text, footer logos), 1 missing `toHaveTextContent` matcher, 1 SSR render. Zero occurrences of `breadcrumb`/`join` in the whole output. |

## Still on staged (Aug-4) WASM — and why

The build reuses `🔌️plugin-modules/` unless `FORCE_PLUGIN_BUILD=1`, and the staged demonstrator
component is dated **Aug 4**. A forced fresh build still fails because
`semio-s-plugin-procedural` does not compile.

I took procedural from **105 → 52** errors by fixing three unambiguous mechanical defects:

1. **Namespace collision (procedural2d).** glue does `pub use component::*`, so the file's own
   re-exported builder *functions* land back in `super`; `use super::{change_schema, …}` then
   imported both the module and the same-named function, colliding in the value namespace. Removed
   the import and qualified all 28 references with `super::`.
2. **Stale slot names (procedural3d).** Eight `use super::remove_*/set_*` imports named modules glue
   no longer declares; glue now declares the semantic names the file already references elsewhere.
   Rewired the 8 imports (the old names appeared nowhere else but a stale comment).
3. **Missing path segment (procedural2d).** Six builders imported as `<slug>::<fn>` actually live at
   `<slug>::mutation::<fn>`.

The remaining **52** are app-layer call sites still using the peer's **deliberately deleted** generic
vocabulary (`Procedural2dMutation::SetWidget`, `Generation(…)`). Choosing which semantic variant
replaces each old call is that session's design decision, not a mechanical rewrite — so I stopped
there rather than guessing on their behalf.

**Consequence:** the demonstrator runs on a WASM build that predates this ticket's Rust work, so the
Rust-side changes (composition lanes, presence/transient, child stores) are **not** exercised by the
running page. Some stale-manifest symptoms are visible — e.g. `setActiveExample` reporting "unknown
fault" for example ids the Aug-4 build does not know. Rebuilding is one `FORCE_PLUGIN_BUILD=1` away
once procedural's vocabulary migration lands.
