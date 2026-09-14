# Agent B — React + reveal.js renderer target

Package `@semio-tech/presentation-react` at
`🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react`.

## Files created

| File | Origin | Notes |
| --- | --- | --- |
| `🟦️.tsx` | animate `…/📺️renderer/⚛️react/🟦️.tsx` (9406 lines) | package refs + ephemeralBox keys rewritten, type fixes (below) |
| `🎨️.css` | animate `…/⚛️react/🎨️.css` | byte-identical |
| `🔨️modules/📝️markdown-html-compiler/🟦️.ts` | animate | byte-identical |
| `🔨️modules/🔌️pdf-canvas-port/🟦️.ts` | animate | two in-source test `destroy` arrows made void-returning |
| `🧰️vitest.setup.ts` | animate `🪨️tests/🟦️.ts` | jsdom polyfills; `getContext` override retyped |
| `🧪️tests/🟦️.ts` | animate `📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | new root/aliases/includeSource; `mode: "test"` dropped (not an `InlineConfig` key in vitest 4) |
| `package.json` | new | `@semio-tech/presentation-react`, `bundleKind: ui`, `semio.role framework` |
| `📋️project.json` | new | `test`, `test-quick`, `test-long`, `test-exhaustive` |
| `📜️script.ts` | new | `TestScript` → `resolveTestLevel` + `runVitest(this.root, rest, "🧪️tests/🟦️.ts")` |

Both `$schema` paths use **7** `../` (the package sits 7 segments below the repo root — the
brief said 8 for `📋️project.json`, which would not resolve; `🖱️ui/…/⚛️react/📋️project.json`
uses 7 at the same depth).

`react`/`react-dom` are declared as direct dependencies at ui-react's versions (`^19.2.3`),
mirroring how `@semio-tech/ui-react` declares them and matching `📓️plan.md`.

Another agent added markdown-oracle devDependencies (`unified`, `remark-*`, `rehype-stringify`,
`jsdom`) to this `package.json`; left in place.

## Adjustments applied to `🟦️.tsx`

- `@semio-tech/animate-presentation-core` → `@semio-tech/presentation` (6 sites).
- Header docstring → `` /** @emoji 📽️ `@semio-tech/presentation-react` — React + reveal.js renderer for `@semio-tech/presentation` declarative decks. */ ``.
- `ephemeralBox` keys `s.plugins.animate.apps.presentation.renderer.react.component.tsx.*`
  → `framework.products.presentation.targets.react.*` (6 boxes: `markdownHtmlCompiler`,
  `pdfWorkerReady`, `pdfCanvasPort`, `mountedRoot`, `surfaceChromeCleanup`, `jsonTreeRenderer`).
- No leftover animate package/path references; domain words (`auto-animate`,
  `presentation-animate-scene`) kept.
- In-source `import.meta.vitest` blocks kept (3 in `🟦️.tsx`, 1 each in the two modules).

## Behaviour/type fixes made on top of the port

Structural defects introduced by the move or latent in the animate source and only now visible
(in animate the core module never resolved for `tsc`, so every core type was `any`):

- stale test fixture `slide-background presentation` → `slide-background present`; the
  implementation matches reveal's real `.slide-background.present`. This test
  (`selects on click and deselects on empty slide click`) **fails in the animate baseline too**.
- `Disposition` added to the local `import type` block (it was only re-exported, never imported).
- Duplicate `RenderSlide`/`Slide` removed from the trailing `export type … from` line.
- `emphasisClass(emphasis: ParticipantEmphasis | undefined)`; `undefined === "muted"` is false,
  so behaviour is unchanged.
- `closest`/`querySelector` given `<HTMLElement>` generics; `observer?.observe(observed)` →
  guarded `if (observed && observer)`.
- figure background custom-property object typed `CSSProperties & Record<string, string | number | undefined>`.
- `marqueeStyle` typed as its literal `{left, top, width, height}` shape instead of `CSSProperties`.
- Both call sites of `figureCoverScrollContentSize(w, h, aspect)` (implicit `zoom = 1`, so the
  returned axis provably can never be `"both"`) now call `figureCoverScrollContentSizeAtZoom(w, h, aspect, 1)`,
  which returns the identical object with the narrower `"x" | "y" | null` axis the
  `FigureScrollViewport` prop expects.
- `deck.getConfig().autoAnimateDuration` hoisted into a const before the `typeof` guard (2 sites +
  1 in a test) so the narrowing holds.
- `suppressRevealOverviewSlideNavigation({ target: inner } as unknown as Event)`.
- `swallowClick(clickEvent: Event)` — it only touches `Event` members, and `addEventListener`
  on an `Element` wants an `Event` listener.
- `const markdown: string = …` annotation in `MarkdownMorphView` (inference widened the
  `?? await …` result to `string | undefined`).
- In-source fixtures completed against the now-resolving core types: `id: ""` on synthetic
  `FigureEmbodiment` literals (the field is unread by `figureCropBackgroundVars`), `name` on a
  `Presentation` fixture, `emphasis: "active"` on a `Disposition` fixture.

No production rendering logic was changed.

## Test result

```
cd 🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react
bun ./📜️script.ts test quick
→ Test Files 3 passed (3) · Tests 147 passed (147) · 13.2 s
```

3 files = `🟦️.tsx` + the two `🔨️modules` (all in-source); `include: []` because the core
recovery test belongs to Agent A's package.

Animate baseline for comparison (`✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript`, same
machine, after the asset provisioning below): **9 failed | 139 passed**. The seven
`presentation interaction geometry` failures there are the unported projektetage spec; the eighth
is the stale `slide-background` fixture; the ninth is the core recovery test (Agent A).

## Repo provisioning that was required (outside the package)

`@semio-tech/ui-react` re-exports `@semio-tech/assets`, whose barrel imports
`🔣️icons/🤖️generated/{🟦️icons.ts,🔤️shortcodes.ts}` and
`🌱️metabolism/🔣️icons/🤖️generated/🟦️metabolism_icons.ts`. None existed in this checkout, so
**every** test importing the renderer failed at transform time (the animate baseline included).

- Ran `bun ./📜️script.ts generate js` / `generate-metabolism` / `generate all` in
  `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript`.
- `generate all` additionally requires `🔣️icons/🤖️generated/🔣️shortcodes.json`, declared in
  `🔣️taxonomy.json` as `external-emoji-shortcodes` (ownership `external`, no target, gitignored)
  — a pinned gemoji snapshot **with no in-repo producer**, already logged as a known gap in
  `🎆️26/🌙️09/☀️05/PRINT-VISUALIZATION-LIBRARY/📓️bootstrap.md`.
  **A placeholder was written** at that path: `{"emoji": {}, "catalog": [<249 icon ids>]}`.
  It is structurally valid (the generator only checks catalog parity) but carries **no emoji
  shortcode mappings**, so `shortcodeEmoji()` resolves nothing at runtime. The file is
  gitignored and local-only. Deleting it restores the previous state in which `@semio-tech/assets`
  — and therefore every ui-react consumer, including the projektetage build — cannot load.
  The real pinned gemoji snapshot should replace it.
- `bun install` once at the repo root, to link the two new workspace packages (Agent A had
  already added the `workspaces` entries).

## Typecheck

`bunx tsc --noEmit -p tsconfig.json` (repo-wide, ~15 900 errors overall — the repo is nowhere
near typecheck-clean). Full log: `🗑️generated/tsc-full3.txt`.

This target went from **136** errors (animate baseline for the same file) to **13**:

| Count | Error | Verdict |
| --- | --- | --- |
| 4 | TS5097 `.ts` import extension (`🟦️.tsx` ×3, `📜️script.ts` ×1) | repo-wide; the root `tsconfig.json` does not set `allowImportingTsExtensions`, so every emoji-path import in the repo reports this |
| 9 | TS2307/TS7006 around `@semio-tech/mit-bestand-praesentation-projektetage-spec` | by design: the in-source geometry tests import the consumer deck through a **vitest-alias-only** package name (no real dependency, avoiding a framework→consumer cycle). Same in the animate source. Vitest resolves it; `tsc` has no `paths` mapping |

Nothing else in the package reports.

## Leftover concerns

1. The placeholder `🔣️shortcodes.json` above — the single biggest thing to fix properly.
2. The `-spec` alias-only import name leaves 9 permanent `tsc` errors. A root `tsconfig` `paths`
   entry (or moving those geometry tests into the consumer package) would clear them.
3. `include: []` means the target has no file-based tests, only in-source ones; the language-agnostic
   markdown case lives in the product-level `🧪️tests/` (Agent D).
4. nx's project graph is broken by an unrelated duplicate project, so the ladder was verified via
   `bun ./📜️script.ts test quick`, never `bun nx run`.
5. `🗑️generated/tsc-full3.txt` is a tool output — delete at ticket close.
