---
name: Fix dark PDF theme not applying
overview: Dark PDFs are visually identical to light ones because the theme option never actually reaches the L3 keys system that drives all color switching — a `\keys_set:nn` call silently stores unexpanded macro tokens instead of the string "dark"/"light". Fix the expansion bug, then align the resulting dark chrome colors precisely with the OS app's dark theme tokens (navbar/canvas/border/foreground) so print truly matches the desktop UI.
todos:
 - id: fix-keys-set-expansion
   content: "Fix print/tex/semio.cls: change \\keys_set:nn to \\keys_set:nx (drop redundant \\edef lines) so theme actually propagates to \\l_semio_theme_tl"
   status: completed
 - id: fix-navbar-shell-token
   content: Replace semio-chrome-shell alias with semio-chrome-window directly in semio-core.sty/semio-window.sty to match OS navbar color
   status: completed
 - id: add-foreground-coltext
   content: Add semio-chrome-foreground alias in semio-core.sty and apply as coltext on all tcolorbox window styles in semio-window.sty
   status: completed
 - id: theme-aware-emblem
   content: Make semioemblem stroke color theme-aware via semio-chrome-border-emphasized in semio-logo.sty
   status: completed
 - id: verify-dark-rendering
   content: Rebuild all templates and visually confirm dark PDFs now differ from light (page bg, navbar, text, emblem) and match OS dark tokens
   status: completed
isProject: false
---

## Root cause (confirmed)

I built all 12 PDFs read-only-adjacent (pre-existing `print/dist/` artifacts from the last session) and rendered the `report.pdf` vs `report-dark.pdf` title pages with `qlmanage`. They are pixel-identical — cream `#f7f3e3` background, black text, same emblem — in both "light" and "dark" builds. A `cmp -l` byte diff confirms the two PDFs are byte-identical except for a 739-byte trailer/ID region at the very end of the file; all page-content bytes are identical.

This traces to [print/tex/semio.cls](print/tex/semio.cls) lines 44-53:

```44:53:print/tex/semio.cls
\edef\semio@setup@type{\semio@type}
\edef\semio@setup@theme{\semio@theme}
\edef\semio@setup@language{\semio@language}
\ExplSyntaxOn
\keys_set:nn { semio / setup } {
  type = \semio@setup@type,
  theme = \semio@setup@theme,
  language = \semio@setup@language,
}
\ExplSyntaxOff
```

`\keys_set:nn` takes **two unexpanded (`n`) arguments**. Argument 2 here is the literal token list `type = \semio@setup@type, theme = \semio@setup@theme, ...` — because it is never expanded, l3keys parses the key `theme` and stores its value as the raw control-sequence token `\semio@setup@theme` itself, not the string `dark`/`light` that macro expands to. So [print/tex/semio-core.sty](print/tex/semio-core.sty) line 30 (`theme .tl_set:N = \l_semio_theme_tl`) sets `\l_semio_theme_tl` to a token list containing one macro reference — never the string `"dark"` or `"light"`.

Every subsequent comparison against it therefore always takes the "else" branch, regardless of which theme was requested:

```65:74:print/tex/semio-core.sty
\cs_new_protected:Npn \semio_theme_apply: {
  \str_if_eq:VnTF \l_semio_theme_tl { dark } {
    \pagecolor{semio-dark}
    \color{semio-light}
  } {
    \pagecolor{semio-light}
    \color{semio-dark}
  }
  \semio_chrome_apply_aliases:
}
```

and the same in `\semio_chrome_apply_aliases:` ([print/tex/semio-core.sty](print/tex/semio-core.sty) lines 43-63). This affects **every** template (report/paper/flyer directly; forschungsbericht/zwischenbericht/kompaktbericht via [print/tex/zukunftbau.cls](print/tex/zukunftbau.cls) forwarding into the same `semio.cls` code path) — matching the observed symmetric bug across all 6 pairs.

`\l_semio_type_tl` / `\l_semio_language_tl` are set through the same broken call but are never compared/used elsewhere in the print sources, so this specific bug is only visibly manifest for theme.

## Fix 1 — Repair the expansion bug (the actual "dark isn't dark" bug)

In [print/tex/semio.cls](print/tex/semio.cls), replace `\keys_set:nn` with `\keys_set:nx` (expand argument 2 before parsing) and drop the now-redundant `\edef` indirection:

```tex
\ExplSyntaxOn
\keys_set:nx { semio / setup } {
  type = \semio@type,
  theme = \semio@theme,
  language = \semio@language,
}
\ExplSyntaxOff
```

This is the minimal, correct fix — `\semio@type`/`\semio@theme`/`\semio@language` are already fully-expandable macros holding plain text ("report"/"dark"/"de") produced by kvoptions, so `nx` expansion resolves them to their string values before l3keys parses the key list.

## Fix 2 — Align resulting dark colors with the OS app's dark theme

Once theme actually flips, two remaining gaps stop it from looking "same as OS":

**a) Navbar/footer bars use the wrong chrome token.** OS maps `theme.navbar` / `theme.button` from `chrome.window` (`ui/wgpu/rs/lib.rs` `from_chrome`, line 5419: `navbar: Rgba::from_chrome(&chrome.window)`), but print's shell alias uses the base page color instead:

```43:62:print/tex/semio-core.sty
    { dark } {
      ...
      \colorlet{semio-chrome-shell}{semio-dark}
    }
  } {
    ...
    \colorlet{semio-chrome-shell}{semio-light}
  }
```

`semio-chrome-window` (already generated per-theme in `semio-tokens.sty` from the same `chrome.window` token) is the correct OS-matching color. Fix: delete the redundant `semio-chrome-shell` alias entirely and use `semio-chrome-window` directly at its two call sites in [print/tex/semio-window.sty](print/tex/semio-window.sty) (`\semio@chrome@chip`'s `\fcolorbox`, and `\semio@chrome@navbar@bar` / `\semio@chrome@footer@bar`'s `\colorbox`).

**b) Boxed body text has no theme-aware color.** `\chapter`/`\section`/`Semiobox` all render inside `tcolorbox` ([print/tex/semio-window.sty](print/tex/semio-window.sty) lines 51-95, `semio~window`/`semio~window~chapter`/`semio~window~section`/`semio~window~boxed`/`semio~window~inner` styles) which defaults `coltext` to black regardless of ambient `\color`. Once dark actually applies, all body text would render black-on-near-black. Fix: add a `semio-chrome-foreground` alias (mirroring the already-generated `semio-chrome-{light,dark}-foreground` tokens) to `\semio_chrome_apply_aliases:` in `semio-core.sty`, and add `coltext=semio-chrome-foreground` to each of those tcolorbox styles.

**c) Emblem strokes are theme-blind.** `\semioemblem` in [print/tex/semio-logo.sty](print/tex/semio-logo.sty) hardcodes `draw=semio-dark` on all 5 outline paths, so on a dark page the outline nearly vanishes against the near-black background. Swap `draw=semio-dark` → `draw=semio-chrome-border-emphasized` (already theme-flipped: `#001117` light / `#f7f3e3` dark) on all 5 `\draw` commands, matching the OS's `border_emphasized` token.

## Verification

- Rebuild via `bun ./script.ts test` from `print/`.
- Re-render `report.pdf` vs `report-dark.pdf` (and spot-check one zukunftbau-family pair) with `qlmanage -t` and visually confirm: dark page background (`#001117`), light text, dark navbar/footer bars using `#07181d` (matching OS `chrome.window`), theme-aware emblem outline.
- Confirm `cmp` shows the two PDFs now differ well before the trailer region (i.e., page content itself differs), not just the last ~700 bytes.

## Ticket

Per repo workflow, open a ticket (goal `r26-02`) for this fix before editing, since it's a distinct bug from the closed AUTO-DERIVE-DARK-PDF ticket (that ticket only automated `.tex` derivation; it never touched theme-application logic). Close it with a summary and full file list once verified.
