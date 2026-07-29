---
name: Print LaTeX Technology
overview: "Add a new independent top-level technology `print`: an expl3-based LaTeX framework compiled with Tectonic, sharing the semio design language (tokens, fonts, logo) through generated artifacts, with robust report/paper/flyer templates plus the full Zukunft Bau report family."
todos:
 - id: ticket
   content: Open ticket via repo MCP (goal r2603)
   status: completed
 - id: scaffold
   content: Scaffold print/ with script.ts, project.json, package.json
   status: completed
 - id: tokens
   content: Implement generate command emitting semio-tokens.sty from ui/styling/tokens.json
   status: completed
 - id: fonts
   content: Implement fonts command fetching TTFs into print/asset/font
   status: completed
 - id: core-cls
   content: Write semio.cls + semio-core/fonts/components/logo .sty (expl3, l3keys, hooks)
   status: completed
 - id: generic-templates
   content: Write report/paper/flyer templates on semio.cls
   status: completed
 - id: zukunftbau
   content: Write zukunftbau.cls and the three Zukunft Bau templates
   status: completed
 - id: build-watch
   content: Implement ensureTectonic, build, watch, test commands
   status: completed
 - id: register
   content: Register in root package.json, launch.json, .gitignore
   status: completed
 - id: verify
   content: Run test build for all templates and verify PDFs
   status: completed
isProject: false
---

# Print LaTeX Technology

## Goal

New top-level `print/` technology for printable LaTeX documents that follow semio ideology (sharp corners, borders, basic geometric shapes, brand colors/fonts). Pure document technology — no framework OS program. Toolchain: Tectonic (zero-touch, cross-platform, XeTeX-based so expl3/l3keys/xparse/lthooks/fontspec all work).

## Directory layout

```
print/
├── script.ts            # bun router: generate | fonts | build | watch | test
├── project.json         # nx targets delegating to script.ts
├── package.json         # @semio-tech/print, bundleKind "asset"
├── tex/
│   ├── semio.cls            # core class: expl3, l3keys config, hooks, type=report|paper|flyer
│   ├── semio-tokens.sty     # GENERATED from ui/styling/tokens.json (colors, spacing)
│   ├── semio-fonts.sty      # fontspec: Anta / Kelly Slab / Share Tech Mono / Noto Emoji
│   ├── semio-core.sty       # expl3 module: key trees, metadata store, hook registration
│   ├── semio-components.sty # title pages, headers/footers, boxes (sharp corners, borders)
│   ├── semio-logo.sty       # handcrafted TikZ emblem/logo (ported from asset/logo/emblem.svg)
│   └── zukunftbau.cls       # loads semio.cls; type=forschungsbericht|zwischenbericht|kompaktbericht
├── template/
│   ├── report/report.tex
│   ├── paper/paper.tex
│   ├── flyer/flyer.tex
│   └── zukunftbau/{forschungsbericht,zwischenbericht,kompaktbericht}.tex
├── asset/font/          # gitignored, TTFs fetched by script.ts fonts
└── dist/                # gitignored, compiled PDFs
```

## LaTeX architecture (per the modern-framework brief)

- **`semio.cls`**: `\ProvidesExplClass`; all logic in `expl3` under the `semio` module namespace (`\semio_..:..`). Single key-value config `\SemioSetup{theme=light|dark, language=de|en, type=..., title=..., ...}` via `l3keys` property trees. User-facing commands via `\NewDocumentCommand`. Injection through `lthooks` (`begindocument` builds cover/title/imprint per type; `enddocument` closes structures). One class, modular `.sty` extensions — a document loads only `semio.cls` or `zukunftbau.cls`.
- **Shared mechanisms** (`semio-core.sty`): metadata store (title, authors, identifiers, funding), theme resolution (light/dark from token colors), page geometry per type, header/footer engine, semantic boxes. All templates consume only these interfaces, never raw packages, so the design stays consistent.
- **Design-language enforcement**: zero corner radius, `--stroke-*`-equivalent rule widths (hairline 1pt-ish scale), no shadows, brand palette only — encoded once in `semio-components.sty`.

## Shared design language wiring

- **Tokens**: `print/script.ts generate` reads [ui/styling/tokens.json](ui/styling/tokens.json) (single source of truth, same as the CSS/Rust/C#/Python emitters in [ui/styling/script.ts](ui/styling/script.ts)) and writes `print/tex/semio-tokens.sty` with `xcolor` `\definecolor` for every token (primary `#ff344f`, secondary `#34d1bf`, tertiary `#fa9500`, dark `#001117`, light `#f7f3e3`, grayscale ramp, semantic colors) plus spacing lengths.
- **Fonts**: `print/script.ts fonts` downloads static TTFs for Anta, Kelly Slab, Share Tech Mono, Noto Emoji from Google Fonts into `print/asset/font/` (mirrors `fetchElementsFonts()`; TTF instead of woff2 because fontspec cannot load woff2). `semio-fonts.sty` loads them by file path.
- **Logo**: handcrafted TikZ port of [asset/logo/emblem.svg](asset/logo/emblem.svg) in `semio-logo.sty` (three brand-color shapes + ring — basic geometric shapes, no external image conversion needed).

## Toolchain (zero-touch)

`print/script.ts` has `ensureTectonic()` following the `ensureTrunk()` precedent in [framework/renderer/wgpu/script.ts](framework/renderer/wgpu/script.ts): probe `tectonic --version`, else `cargo install tectonic --locked`. Tectonic auto-downloads TeX packages on first build (no TeX Live needed) and works identically on devcontainer/macOS/Windows/Linux.

- `build [template]` — compiles one or all templates to `print/dist/<name>.pdf`
- `watch [template]` — `tectonic -X watch`-style rebuild loop for authoring
- `test` — smoke-builds every template and asserts the PDFs exist (run to verify, not claimed passing otherwise)

## Zukunft Bau report family (`zukunftbau.cls`)

`\documentclass[type=forschungsbericht]{zukunftbau}` on top of `semio.cls`:

- **Forschungsbericht**: German, chapter-based; metadata commands `\aktenzeichen`, `\foerderzeitraum`, `\doi`; `\makecoverpages` (logo slots for Zukunft Bau/BBSR/Bundesbauministerium — configurable file-path keys with framed placeholders since the protected logos cannot ship in the repo); `\makefundingacknowledgement` emitting the mandatory German+English funding texts; scaffolded chapters Einführung through Schlussworte per the guideline.
- **Zwischenbericht**: section-based short report (Ergebnisse, Projektstand, Mittelverwendung, Ergebnisverwertung, Anlagen); optional network work-package attribution table.
- **Kompaktbericht**: drafting-only mode — Harvard citations (`biblatex` `style=authoryear`, biber via Tectonic), fixed structure (Titelblatt … Impressum), and an end-of-build character/page count report printed to the log (since BBSR does final typesetting).

## Repo registration

- [package.json](package.json): add `"print"` to `workspaces`; add scripts `"dev:print": "bun nx run @semio-tech/print:watch"` and `"build:print": "bun nx run @semio-tech/print:build"`.
- [.vscode/launch.json](.vscode/launch.json): `🛠️dev🖨️print` in group `3_dev` (watch) and `📦build🖨️print` in group `4_build` (order between `📦build📽️presentationplay` 100 and `📦audit🛝playground` 105, following alphabetical-ish ordering).
- [.gitignore](.gitignore): `print/dist/` and `print/asset/font/`.
- No Cargo workspace, plugin registry, or playground port entries (templates-only shape, as decided).
- Ticket via repo MCP `ticket_open` (goal `r2603`), all logs/temp files inside the ticket folder.

## Verification

Run `bun ./script.ts test` in `print/` (Tectonic install → font fetch → token generation → compile all six templates) and confirm each PDF is produced; inspect one PDF visually for brand fidelity.
