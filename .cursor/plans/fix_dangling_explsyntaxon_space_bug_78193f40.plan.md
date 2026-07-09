---
name: Fix Dangling ExplSyntaxOn Space Bug
overview: Fix a package-level bug that silently deletes every space character in the Zwischenbericht body text (headings and prose) from `\begin{document}` onward, causing the "overshooting"/"wrong spacing" text rendering the user reported.
todos:
 - id: fix-explsyntax
   content: Add missing \ExplSyntaxOff at end of \AfterEndPreamble block in print/tex/semio-window.sty
   status: completed
 - id: rebuild
   content: Rebuild zwischenbericht.tex and zwischenbericht-dark.tex with bun ./script.ts build
   status: completed
 - id: verify
   content: Render and visually verify pages 5-7 (and spot-check earlier pages) in both light/dark PDFs show correct spacing/wrapping
   status: completed
 - id: ticket
   content: Reopen relevant ticket, log the fix, and close with summary
   status: completed
isProject: false
---

## Root cause

`print/tex/semio-window.sty` has an `\AfterEndPreamble` hook that is deferred code executed right after `\begin{document}`:

```505:517:print/tex/semio-window.sty
\AfterEndPreamble {%
  \ExplSyntaxOff
  \makeatletter
  \setlength { \parskip } { 0pt }
  \setlength { \semio@block@sep@skip } { \semio@spacing@single }
  \setlength { \parindent } { 0pt }
  \semio@chrome@dims@compute
  \semio@hierarchy@hooks@install
  \makeatother
  \ExplSyntaxOn
  \semio_chrome_footer_install:
  \semio_header_footer_apply:
}
```

It turns `\ExplSyntaxOn` back on (line 514) to call the two `expl3`-named functions, but never turns it back off before the block ends. Under `\ExplSyntaxOn`, `expl3` sets the catcode of the plain ASCII space character to 9 (ignored) — that's the standard `expl3` convention where source-code spaces are cosmetic and `~` is used for a literal space. Because this hook fires globally in the main vertical list (not inside an extra `{}` group), that catcode change is **not undone**, so every space typed anywhere in the document body from that point on (section/subsection/paragraph titles, all running prose) is silently dropped by TeX's tokenizer.

This matches exactly what was rendered and captured in `/tmp/zb-page-5.png` / `-6.png` / `-7.png`: words are concatenated ("undFragmenttypenfürdenStahlbeton-Kontext", "18Interviews"), so paragraphs become effectively one unbroken run of characters that then overflows ("overshoots") the window/text width because TeX has no space-based breakpoints to justify/wrap the line.

Content typed in the preamble (`\title`, `\subtitle`, `\kurzfassung`, …) is unaffected because those arguments are tokenized before `\begin{document}` runs the hook, so their space tokens are already frozen with the correct catcode 10 — this is why the navbar title "Zwischenbericht --- Entwerfen mit Bestand" renders correctly while body headings/prose do not.

All other `.sty` files in `print/tex/` pair their file-level `\ExplSyntaxOn`/`\ExplSyntaxOff` correctly (verified via grep); this is the only unbalanced occurrence.

## Fix

In `print/tex/semio-window.sty`, close the `\ExplSyntaxOn` opened at line 514 before the `\AfterEndPreamble` block ends, e.g.:

```latex
\AfterEndPreamble {%
  \ExplSyntaxOff
  \makeatletter
  \setlength{\parskip}{0pt}
  \setlength{\semio@block@sep@skip}{\semio@spacing@single}
  \setlength{\parindent}{0pt}
  \semio@chrome@dims@compute
  \semio@hierarchy@hooks@install
  \makeatother
  \ExplSyntaxOn
  \semio_chrome_footer_install:
  \semio_header_footer_apply:
  \ExplSyntaxOff
}
```

This restores normal space catcode (10) for the remainder of the document, fixing headings, chip labels, and all body prose spacing/wrapping simultaneously with a single-line change.

## Verification

1. Rebuild with `bun ./script.ts build` in `mit-bestand/bericht` for both `zwischenbericht.tex` and `zwischenbericht-dark.tex`.
2. Re-render pages 5-7 (the "Ergebnisse" content) of both PDFs to PNG (same `pdfjs-dist` + `@napi-rs/canvas` approach used for diagnosis) and visually confirm:
   - Interword spaces are present in prose and in heading/chip titles (e.g. "18 Interviews", not "18Interviews").
   - Lines wrap normally within the window border / text width, no overshoot.
   - No new overfull/underfull box warnings introduced by the fix.
3. Spot-check earlier pages (cover, TOC, workpackages table) to confirm no regression there.
4. Check the build log for the `Underfull \hbox` warnings already present in `zwischenbericht-dark.tex` around lines 323-343 (from git status, pre-existing table content) — resolve if related, otherwise leave as pre-existing/unrelated.

## Ticket workflow

Reopen the ticket that produced this content (per `AGENTS.md`, a new message is presumed related to the prior task) rather than opening a new ticket, add this fix/verification artifacts to that ticket folder, then close it again with an updated summary listing `print/tex/semio-window.sty` as modified.
