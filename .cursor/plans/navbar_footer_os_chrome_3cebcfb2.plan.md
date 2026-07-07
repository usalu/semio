---
name: Navbar Footer OS Chrome
overview: Port the semio UI's Navbar and Footer chrome (flat bar, single hairline edge, unbordered flush logo+title, bordered icon+label chip groups) into the LaTeX print running page header/footer, replacing the current plain fancyhdr look, reusing the same chrome tokens/colors already wired for the OS-window chapter/section chrome.
todos:
  - id: script-metrics
    content: Emit \semio@chrome@navbar@height, \semio@chrome@footer@height, \semio@chrome@font@body in print/script.ts emitSemioTokensSty()
    status: completed
  - id: chrome-bar-styles
    content: Add semio~chrome~bar~navbar/footer and semio~chrome~group tcbox styles plus heading tracker to semio-window.sty
    status: completed
  - id: compose-commands
    content: "Implement \\semio_navbar_apply: / \\semio_footer_apply: composing icon+title/chip content and wire into begindocument hook"
    status: completed
  - id: remove-old-header
    content: "Delete old \\semio_header_footer_apply: and its hook from semio-components.sty"
    status: completed
  - id: page-geometry
    content: Adjust \headheight/\headsep/\footskip for the taller bars
    status: completed
  - id: verify-build
    content: Regenerate tokens, rebuild all 12 template PDFs, visually verify light+dark navbar/footer chrome
    status: completed
  - id: ticket
    content: Reopen OS-WINDOW-STYLE-FOR-PRINT ticket for this extension, close with summary when done
    status: completed
isProject: false
---

## Reference (source of truth, confirmed identical in React and wgpu)

- **Navbar**: full-width bar, bg = `chrome.window`, **single hairline border on the bottom edge only** (no border elsewhere), height = `navbarHeightUiSpacing` (9× compact spacing ≈ 1.8em), padding = `paddingStandardUiSpacing` (1×). Left content (logo icon + app title, `text-sm`/`font_size_body`) is **flush, unbordered**. Grouped controls (mode switch, panel toggles) render as a **separate bordered chip**: full 4-sided hairline border, `border_normal`, containing icon+label at the smaller chrome font.
  - React: [ui/js/react/index.tsx](ui/js/react/index.tsx) `Navbar` (~L9011-9043), `borderNormalBottomClass`.
  - Wgpu: [framework/renderer/wgpu/rs/lib.rs](framework/renderer/wgpu/rs/lib.rs) `render_navbar` (~L8525-8566), `render_chrome_group` (~L7995-8071).
- **Footer**: mirrors navbar but bg-bar border is on the **top edge only**, and its icon+label content (app icon + app name) is *always* wrapped in one bordered chip (`ActionGroup className="border"` / `render_chrome_group`), never flush.
  - React: `Footer` (~L4624-4643), `ActionGroup`/`ActionGroupItem` (~L5836-5926).
  - Wgpu: `render_footer` (~L8725-8788).
- Both already share the exact tokens already emitted into `print/tex/semio-tokens.sty` for window chrome: `semio-chrome-{window,canvas,border-normal,border-emphasized}`, `\semio@stroke@hairline`, `\semio@spacing@compact`, plus new metrics below.

## Current print state (to replace)

```29:37:print/tex/semio-components.sty
\cs_new_protected:Npn \semio_header_footer_apply: {
  \pagestyle{fancy}
  \fancyhf{}
  \fancyhead[L]{\semioemblem[0.8cm]}
  \fancyhead[R]{\SemioSans\leftmark}
  \fancyfoot[C]{\SemioMono\thepage}
  \renewcommand{\headrulewidth}{\semio@stroke@hairline}
  \renewcommand{\footrulewidth}{\semio@stroke@hairline}
}
```

This is plain text with a full-width rule underneath — no bg fill, no bordered chip, not proportioned like the UI navbar/footer. It is hooked globally for every template via `\AddToHook{begindocument}{\semio_header_footer_apply:}`.

## 1. Emit navbar/footer metrics — `print/script.ts`

`ui/styling/tokens.json` already has `metrics.chrome.navbarHeightUiSpacing`/`footerHeightUiSpacing` (both `9.0`) and `metrics.typography.textSmPx` (`12.8`), unused today. In `emitSemioTokensSty()`:
- Extend the `Tokens` type's `chrome`/`typography` metrics fields to include `navbarHeightUiSpacing`, `footerHeightUiSpacing`, `textSmPx`.
- Emit `\semio@chrome@navbar@height` / `\semio@chrome@footer@height` (compactFactor × 9 ≈ `1.8em`) next to the existing `\semio@chrome@titlebar@height` emission (~L192-197).
- Emit `\semio@chrome@font@body` from `textSmPx` (≈ `0.8em`) next to `\semio@chrome@font@title`/`@number` (~L199-204), for the navbar app-title text (matches React `text-sm` / wgpu `font_size_body`).

## 2. New "page chrome bar" mechanism — `print/tex/semio-window.sty`

Add a new region below the existing window-chrome styles (same file, since it shares all chrome tokens/tcolorbox conventions):

- **`semio~chrome~bar~navbar`** / **`semio~chrome~bar~footer`** tcbox styles: `enhanced, nobeforeafter, arc=0mm, boxrule=0pt`, `colback=semio-chrome-window`; navbar has `bottomrule=\semio@stroke@hairline` (others `0pt`), footer has `toprule=\semio@stroke@hairline` (others `0pt`); `height=\semio@chrome@navbar@height` / `\semio@chrome@footer@height`, `valign=center`, `left=right=\semio@chrome@padding`. This is the flat single-edge bar, distinct from the merged tab/controls styles used for chapter/section titles.
- **`semio~chrome~group`** tcbox style: the bordered chip — `enhanced, nobeforeafter, arc=0mm`, full `boxrule=\semio@stroke@hairline`, `colframe=semio-chrome-border-normal`, `colback=semio-chrome-window`, `coltext=semio-chrome-border-emphasized`, `fonttitle` n/a, height = reuse existing `\semio@chrome@titlebar@height` (this *is* `controlHeightUiSpacing`, same semantics as the wgpu/React grouped-control row height), `left=right=\semio@chrome@padding`, `valign=center`.
- **Heading tracker**: add `\tl_new:N \g_semio_current_heading_tl` (global) and set it inside `\semio_window_open_chapter:n` and `\semio_window_open_section:n` (right after `\chaptermark`/`\sectionmark`) — avoids relying on `\leftmark`/`\rightmark`, which behave inconsistently between the `scrreprt` (report/zukunftbau) and `scrartcl` (paper/flyer) branches of `semio.cls`.
- **Composition commands**:
  - `\semio_navbar_apply:` builds `\fancyhead[C]{...}`: a `\tcbox[semio~chrome~bar~navbar]` containing an `\hbox to \linewidth{ \semioemblem[<icon size>]~\SemioSans\fontsize{\semio@chrome@font@body}{...}\@title \hfil \tcbox[semio~chrome~group]{\SemioSans\fontsize{\semio@chrome@font@title}{...}\g_semio_current_heading_tl} }` — flush icon+title on the left, current chapter/section name in a bordered chip on the right.
  - `\semio_footer_apply:` builds `\fancyfoot[C]{...}`: a `\tcbox[semio~chrome~bar~footer]` containing `\hbox to \linewidth{ \tcbox[semio~chrome~group]{\semioemblem[<small icon size>]~\SemioSans\fontsize{\semio@chrome@font@number}{...}\@author} \hfil \SemioMono\fontsize{\semio@chrome@font@number}{...}\thepage }` — icon+author name in a bordered chip on the left (mirrors `ActionGroup`), plain page number on the right.
  - Icon sizes: compute from bar height minus padding via `calc` (e.g. `\dimexpr\semio@chrome@navbar@height-2\semio@chrome@padding\relax` for navbar icon, smaller for the footer chip icon) so icon-to-bar proportion matches the UI's `logo height = control_height - gap`.
- **Replace `\semio_header_footer_apply:`**: remove it from `print/tex/semio-components.sty` (delete the `cs_new_protected:Npn` block, lines 29-37, and the `\AddToHook{begindocument}{\semio_header_footer_apply:}` line) and define the equivalent hook in `semio-window.sty` instead: `\pagestyle{fancy}\fancyhf{}\semio_navbar_apply:\semio_footer_apply:\renewcommand{\headrulewidth}{0pt}\renewcommand{\footrulewidth}{0pt}` (rule widths zeroed since the bars now draw their own single-edge border; doubling them with fancyhdr's rule would duplicate the line). Register via `\AddToHook{begindocument}{...}` in `semio-window.sty`, keeping the single source of truth for all OS-chrome (window + navbar + footer) in one file.
- **Page geometry**: set `\setlength{\headheight}{...}` (≥ `\semio@chrome@navbar@height` + slack) and `\setlength{\headsep}{...}`/`\setlength{\footskip}{...}` sized for `\semio@chrome@footer@height`, so fancyhdr does not warn/clip the taller bars; verify no overlap with existing `margin=2.5cm`/`margin=1.2cm` geometry in `semio.cls`.

## 3. Verification

- `bun ./script.ts generate` in `print/` to regenerate `semio-tokens.sty` with the new metrics.
- `bun ./script.ts test` in `print/` to rebuild all twelve template PDFs (six templates × light/dark).
- Visually inspect at least one light and one dark PDF (e.g. `forschungsbericht.pdf` / `forschungsbericht-dark.pdf`) to confirm: navbar bottom-edge-only border with flush icon+title and a bordered chip on the right; footer top-edge-only border with a bordered icon+label chip on the left and a plain page number on the right; colors/spacing match the existing window chrome exactly; dark theme resolves via the existing `semio-chrome-*` aliasing with no extra work.

## Ticket

Reopen the existing `2026/07/06/OS-WINDOW-STYLE-FOR-PRINT` ticket (same OS-chrome porting initiative, now extended to the page-level navbar/footer) rather than opening a new one, and close it again with an updated summary once verified.

## Out of scope

- No per-template opt-out (flyer keeps the same global navbar/footer as report/paper, matching current behavior).
- Not fixing the pre-existing `\chapter*`/`\section*` numbering-still-increments quirk in `semio-window.sty` — unrelated to this task.