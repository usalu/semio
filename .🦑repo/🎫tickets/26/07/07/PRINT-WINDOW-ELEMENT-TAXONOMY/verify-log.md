# Verify Log

## Commands

- `bun ./script.ts test` in `print/` — all 12 template PDFs (6 light + 6 dark) built successfully
- `bun ./script.ts build` in `mit-bestand/bericht/` — `zwischenbericht.pdf` and `zwischenbericht-dark.pdf` built successfully

## Behavior

- `\chapter` / `\section` render as plain KOMA headings (no window chrome)
- 14 element environments available: `Image`, `Photo`, `Figure`, `Table`, `Listing`, `Pseudocode`, `Theorem`, `Lemma`, `Proof`, `Equation`, `Glossary`, `Abbreviations`, `Blockquote`, `Epigraph`
- Tier styling:
  - Visual → `semio-primary` border + chip fill
  - Logical → `semio-secondary` border + chip fill
  - Structural → `semio-tertiary` border + chip fill
- Chip text uses `semio-chrome-canvas` for contrast
- `Semiobox` retired; migrated call sites use `Blockquote` or `Table`
- Consecutive element windows are separated by `\semio@spacing@single` via `\addvspace` before each window header
