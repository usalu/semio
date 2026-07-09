# Verify Log — ExplSyntaxOn Space Bug Fix

## Root cause

`print/tex/semio-window.sty` `\AfterEndPreamble` hook called `\ExplSyntaxOn` for footer install but never restored `\ExplSyntaxOff`. Under expl3, space catcode 9 (ignored) persisted globally after `\begin{document}`, stripping all inter-word spaces from body text.

## Fix

Added `\ExplSyntaxOff` at end of `\AfterEndPreamble` block (line 517).

## Build

```
cd mit-bestand/bericht && bun ./script.ts build
```

Exit 0. Both `zwischenbericht.pdf` and `zwischenbericht-dark.pdf` built successfully.

## Text extraction (page 5, after fix)

```
die Kette am Stahlbeton-Test-Case. Kapitel 1 stellt die Ergebnisse dieser Arbeitspakete im Berichtszeitraum dar, Kapitel 2 den Projektstand.
```

Before fix: words were concatenated (`undFragmenttypenfürdenStahlbeton-Kontext`).

## Visual check

Rendered pages 2, 5, 6, 7 for light and dark PDFs. Confirmed:

- Inter-word spaces present in prose and heading chips
- Lines wrap within text width (no overshoot)
- Navbar title unchanged (preamble tokens unaffected)

Pre-existing underfull hbox warnings in `zwischenbericht-dark.tex` lines 323–343 (skeleton section headings) — unrelated to this fix.
