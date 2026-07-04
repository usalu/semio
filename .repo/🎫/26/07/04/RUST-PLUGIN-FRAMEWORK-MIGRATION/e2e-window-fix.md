# E2E Window Render Fix

## Root cause

`refreshUi` in `os-shell.tsx` destructured `Promise.all` results incorrectly:

```ts
const [windowNodes, ...panelNodes] = await Promise.all([...windows, ...panels]);
```

`windowNodes` was the **first render result** (a single `UiNode`), not an array. Indexing `windowNodes[0]`, `windowNodes[1]`, … yielded `undefined` for every window kind → `Missing window: s-media-graph` (and vfs, compiled-dag).

Panels accidentally received window render outputs (offset by one), which is why Catalogue/Parameters appeared to work.

## Fix

```ts
const windowCount = nextSession.app.windowKinds.length;
const rendered = await Promise.all([...]);
const windowNodes = rendered.slice(0, windowCount);
const panelNodes = rendered.slice(windowCount);
```

## Secondary

- `processPluginOps` now applies `setDocument` ops and updates session by instance id.
- Flow WASM init wrapped in try/catch to avoid uncaught `NoCompatibleDevice` in headless Playwright.
- E2E script filters headless-only GPU errors and asserts no `Missing window:` text.

## Verification

```
node .repo/🎫/26/07/04/RUST-PLUGIN-FRAMEWORK-MIGRATION/s-studio-e2e-verify.mjs
# PASS: S studio end-to-end workflows verified
```
