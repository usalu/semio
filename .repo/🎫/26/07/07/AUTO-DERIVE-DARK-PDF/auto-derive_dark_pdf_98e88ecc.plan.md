---
name: Auto-derive dark PDF
overview: Stop hand-maintaining six `*-dark.tex` wrapper files and six duplicate `TEMPLATES` entries in `print/script.ts`. Instead, derive each dark PDF automatically at build time from its canonical (light) `.tex` file, and let logo/image assets optionally supply a `-dark` variant that is picked up by convention.
todos:
 - id: script-derive
   content: Collapse TEMPLATES to 6 families and derive dark .tex/.pdf automatically in print/script.ts (build/watch/test)
   status: completed
 - id: remove-dark-files
   content: Delete the 6 hand-written *-dark.tex wrapper files and add print/.gitignore for the regenerated artifact
   status: completed
 - id: logo-theme-variant
   content: Add automatic light/dark image resolution by -dark suffix convention in semio-logo.sty
   status: completed
 - id: verify-build
   content: Run print build/test to confirm all 12 PDFs still generate correctly
   status: completed
isProject: false
---

## Problem

Today "dark" is configured, not derived:

- 6 hand-maintained wrapper files exist only to flip one class option:

```1:2:print/template/report/report-dark.tex
\documentclass[type=report,theme=dark,language=de]{semio}
\input{report.content.tex}
```

- `print/script.ts` lists all 12 (6 light + 6 dark) as separate, parallel entries in `TEMPLATES` ([print/script.ts](print/script.ts) lines 44-57), so every new template requires two hand-written entries and two hand-written `.tex` files that must be kept in sync forever.
- Logos (`\semio@logo@slot` in [print/tex/semio-logo.sty](print/tex/semio-logo.sty)) take one filename and use it verbatim for both themes — there is no way to supply a theme-specific image, and no automatic fallback.

## Plan

### 1. Derive dark `.tex` automatically in `print/script.ts`

- Collapse `TEMPLATES` to the 6 canonical (light) families only (drop all `*-dark` entries).
- Add a pure helper:

```ts
function deriveDarkTexSource(lightSource: string): string {
 if (!/\btheme=light\b/.test(lightSource)) throw new Error("template missing theme=light; cannot derive dark variant");
 return lightSource.replace(/\btheme=light\b/, "theme=dark");
}
```

- In `compileTemplate`, after compiling the canonical light `.tex` to `<name>.pdf` (unchanged), write the derived source to a sibling `<name>-dark.tex` next to the canonical file (same directory, so its relative `\input{*.content.tex}` / `\addbibresource{references.bib}` keep resolving unchanged) and compile it to `<name>-dark.pdf` via the existing `compilePrintDocument`.
- The sibling `-dark.tex` is a regenerated build artifact, not a source file: gitignore it (new pattern, see below) and overwrite it on every build/watch cycle.
- `resolveTemplates` keeps matching by family id only (`report`, `paper`, …) — building a family now always yields both PDFs; there is no separate "-dark" target to request.
- `TestScript` still asserts all 12 PDFs exist, by checking `<name>.pdf` and `<name>-dark.pdf` for each of the 6 families.
- `watchTemplates`'s file-watch handler ignores paths matching `*-dark.tex` so the derived artifact never triggers its own rebuild loop.

### 2. Remove the hand-maintained dark wrapper files

Delete the 6 currently-untracked wrapper files (they're regenerated automatically now):

- `print/template/report/report-dark.tex`
- `print/template/paper/paper-dark.tex`
- `print/template/flyer/flyer-dark.tex`
- `print/template/zukunftbau/forschungsbericht-dark.tex`
- `print/template/zukunftbau/zwischenbericht-dark.tex`
- `print/template/zukunftbau/kompaktbericht-dark.tex`

Add a `print/.gitignore` with `template/**/*-dark.tex` so the regenerated files never get committed even if a build is interrupted before cleanup.

### 3. Optional theme-specific images by convention (`print/tex/semio-logo.sty`)

- `\RequirePackage{semio-core}` so `\l_semio_theme_tl` is available.
- Add an expl3 resolver: given a supplied path like `zukunftbau-logo.pdf`, when the active theme is `dark`, compute the candidate `zukunftbau-logo-dark.pdf` (insert `-dark` before the final extension) and use it **only if that file actually exists**; otherwise fall back to the originally supplied path. Light theme always uses the supplied path as-is.
- Update `\semio@logo@slot` (used by `\semio@logo@slot[3.5cm]{zukunftbau-logo.pdf}` etc. in [print/tex/semio-components.sty](print/tex/semio-components.sty)) to resolve through this helper before both the `\IfFileExists` check and `\includegraphics`.
- No call-site changes needed: authors keep passing one filename; supplying a same-named `-dark` sibling is entirely optional. This mirrors the existing `-dark` suffix convention already used for chrome color tokens (`semio-chrome-dark-*`) and template files.
- Bump the `\ProvidesPackage{semio-logo}` version/date comment to reflect the change.

### Out of scope

- The TikZ-drawn `\semioemblem` is not "user-supplied" imagery and is left as-is (always strokes `semio-dark`) — not part of this request.
- No changes to `asset/logo/*_dark.svg` (different pipeline, different naming convention, unused by print today).

### Verification

- `bun ./script.ts test` (or `nx run @semio-tech/print:test`) from `print/` must still produce and assert all 12 PDFs, now via 6 configured families instead of 12.
- Confirm no `*-dark.tex` files remain tracked in git after a build.
