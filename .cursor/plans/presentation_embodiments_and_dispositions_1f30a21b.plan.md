---
name: Presentation Embodiments And Dispositions
overview: "Reconcile @semio-tech/framework-presentation with presentation/AGENTS.md: add Video + Pdf (react-pdf) embodiments, rename ParticipantPlacement to a positioned/styled Disposition, add an Analogy template, and make every embodiment render and auto-animate in the React + reveal.js renderer, verified live in 33.projektetage with its real PDF/MP4/PNG assets."
todos:
  - id: ticket
    content: Read repo://goals and open/reopen a repo MCP ticket under the framework goal
    status: completed
  - id: core-embodiments
    content: Add VideoEmbodiment + PdfEmbodiment to core/index.ts and extend the Embodiment union
    status: completed
  - id: core-disposition
    content: Rename ParticipantPlacement->Disposition (add position/style), Arrangement.placements->dispositions, ResolvedPlacement->ResolvedDisposition; update resolver + intro helpers
    status: completed
  - id: core-analogy
    content: Add analogy() template + extend core inline vitest for renames, new embodiments, analogy
    status: completed
  - id: renderer-deps
    content: Add react-pdf to renderer package.json and run bun install
    status: completed
  - id: renderer-views
    content: Add Video + Pdf morph views (react-pdf, worker via import.meta.url), positioning/style wrapper, update switch + re-exports
    status: completed
  - id: renderer-tests
    content: Extend renderer inline vitest + vitest.setup.ts jsdom polyfills for pdf/video/positioned dispositions
    status: completed
  - id: projektetage
    content: Wire real public PDF/MP4/PNG assets into the projektetage deck and update its inline test
    status: completed
  - id: validate
    content: Run core + renderer tests, verify projektetage dev server live (screenshots/CDP), close ticket
    status: completed
isProject: false
---

## Goal

`presentation/AGENTS.md` names embodiments Figure, Video, Text, Pdf and a `Disposition` ("concrete positioned, styled embodiment") plus an Analogy template, but [framework/product/presentation/core/index.ts](framework/product/presentation/core/index.ts) only implements Text/Figure (+ Bullet/Authors/Affiliations) and a flat `ParticipantPlacement` (emphasis only). This work closes that gap (full reconciliation) and proves it live.

Constraints: no AGENTS.md edits; extend existing files only (no new files); add code in `#region`s; `react-pdf`/`reveal.js` stay isolated in the renderer `🔌Adapters` boundary; verify with inline vitest + runtime logs/screenshots.

## 1. Core model — [core/index.ts](framework/product/presentation/core/index.ts)

- Add `VideoEmbodiment { kind: "video"; id?; src; poster?; autoplay?; loop?; muted?; controls? }` and `PdfEmbodiment { kind: "pdf"; id?; src; page?; alt? }`; extend the `Embodiment` union and the docstring header.
- Rename `ParticipantPlacement` -> `Disposition`, made positioned/styled: keep `participantId`, `embodimentId?`, `emphasis`; add optional `position?: { x; y; width; height }` (0..1 slide fractions) and `style?: { opacity?: number; rotate?: number; scale?: number }`.
- Rename `Arrangement.placements` -> `Arrangement.dispositions`; rename `ResolvedPlacement` -> `ResolvedDisposition` (carry through `position`/`style`); update `resolveArrangement` and the `active`/`muted` helpers in `intro`.
- Add `analogy(spec)` template in a new `🔖Analogy` region: opinionated minimal shape `analogy({ id?, name?, source: { label; figure? }, target: { label; figure? } })` producing one morph thought (`source` arrangement -> `mapping` arrangement that morphs source into target via shared `data-id`s). Document the assumption since the AGENTS.md `## Analogy` section is empty.
- Update inline `🧪Tests` for the rename, new embodiments, and `analogy` (slide count, morph ids, positioned disposition resolution).

## 2. React renderer — [renderer/react/index.tsx](framework/product/presentation/renderer/react/index.tsx)

- `🔌Adapters`: `import { Document, Page, pdfjs } from "react-pdf"` and set `pdfjs.GlobalWorkerOptions.workerSrc = new URL("pdfjs-dist/build/pdf.worker.min.mjs", import.meta.url).toString()` in this same module (react-pdf requirement).
- Add `VideoMorphView` (`<div data-id><video .../></div>`, defaults: muted+playsInline; autoplay/loop/controls from embodiment) and `PdfMorphView` (`<div data-id><Document file={src}><Page pageNumber={page ?? 1} renderTextLayer={false} renderAnnotationLayer={false}/></Document></div>`). Both keep `data-id={anchorId}` on the wrapper so reveal.js auto-animates them.
- Extend `MorphPlacementView` switch with `case "video"` / `case "pdf"` (keep exhaustive `never` check).
- Apply Disposition positioning/style: a small wrapper that, when `position` is set, absolutely positions/sizes via `%` and applies `style` (opacity/rotate/scale) so auto-animate can morph rect/transform between arrangements; falls back to current centered flow otherwise.
- Update renamed re-exports (`Disposition`, `ResolvedDisposition`, `analogy`) and `ArrangementSection` (`arrangement.dispositions`).
- Extend inline `🧪Tests`: assert pdf wrapper (`.react-pdf__Document` container) and `<video>` render with `data-id`, positioned disposition gets absolute style, and morph `data-auto-animate` still tags every arrangement.
- Add jsdom polyfills react-pdf/pdfjs need on import (`DOMMatrix`, `Path2D`, `Promise.withResolvers`, `canvas.getContext` stub) to [vitest.setup.ts](framework/product/presentation/renderer/react/vitest.setup.ts).

## 3. Dependencies

- Add `react-pdf` (latest, ^10) to [renderer/react/package.json](framework/product/presentation/renderer/react/package.json) dependencies; `pdfjs-dist` resolves transitively for the worker URL. Run `bun install`.

## 4. Live verification — [33.projektetage/index.ts](mit-bestand/präsentation/33.projektetage/index.ts)

Wire the real assets in [public/](mit-bestand/präsentation/33.projektetage/public) into the deck (a second sequence/thought after `intro`) to confirm runtime rendering + morph:
- Figure: `Screenshot-2023-05-24-at-22-11-19-component-catalogue.png`
- Video: `bauen-mit-bestand.mp4`
- Pdf: `bachelor-thesis-ueli-saluz.pdf`

Update its inline test for the new arrangement count. No vite.config change expected (worker uses `import.meta.url`); add `vite-plugin-static-copy` only if the dev server 404s the worker.

## 5. Validate (repo rules)

- Open repo MCP ticket (read `repo://goals`; reopen the existing presentation ticket if it matches, else `ticket_open` under `🎯framework`); keep temp logs in the ticket folder.
- Run `bun nx run @semio-tech/framework-presentation-core:test` and `@semio-tech/framework-presentation-renderer-react:test`.
- Run the projektetage dev server (port 6050) and confirm via screenshots/CDP that PDF page, video frame, and figure render and auto-animate (no horizontal fly-in regression); add `[DEBUG]` logs while checking, then remove.
- `ticket_close` with summary + file list.

## Out of scope

- Svelte renderer (placeholder stays).
- PDF text/annotation layers and multi-page scrolling (single page render only).
