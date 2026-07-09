# Verify Log — Fix Print Paragraph Chip Alignment

## Muted chip fill overshoot — fixed 2026-07-10

**Root cause:** `\semio@heading@cap@muted@core` used `\colorbox{semio-chrome-canvas}` without resetting `\fboxsep` to `0pt`. Default `\fboxsep` (3pt) made the fill `6pt` too wide and `3pt` too tall inside rigid `\hbox to \semio@window@cap@w` / `\vbox to \semio@chrome@titlebar@height`, while hairline border strokes (drawn via `\rule`) stayed correct.

**Fix:** Wrap the muted `\colorbox` with `\begingroup \setlength{\fboxsep}{0pt} ... \endgroup`, matching `\semio@window@cap` (colored chips).

**Verified:**
- `verify-paragraph.log` — no `Overfull \hbox (6.0pt too wide)` warnings after fix.
- Full `zwischenbericht` rebuild — no `6.0pt` chip overfull warnings.
- `verify-paragraph-p1-16x-v4.png`, `verify-paragraph-p1-8x-v4.png`, `verify-cover-p1-12x-v4.png`, `zwischenbericht-p1.png`, `zwischenbericht-p5.png`.

## Changes (`print/tex/semio-window.sty`)

1. **Muted chip `\fboxsep` reset** — `\semio@heading@cap@muted@core` canvas fill no longer overshoots border on right/bottom (paragraph chips + window header-row chips).

## Verify commands

```bash
cd mit-bestand/bericht/zwischenbericht
tectonic -Z search-path=../../../print/tex --outdir dist verify-cover.tex
tectonic -Z search-path=../../../print/tex --outdir dist verify-paragraph.tex
```

## Visual result

- `verify-paragraph-p1-16x-v4.png` — muted paragraph chips (`18 Interviews`, `Recherche`) fill matches border.
- `verify-cover-p1-12x-v4.png` — cover window header-row chips aligned.
- `zwischenbericht-p5.png` — body paragraph chips in full document.

## Compile hang (`watch`) — fixed 2026-07-09

**Root cause:** `\semio_window_tier_header:nnn` stored `{ \tl_use:N \l_semio_window_number_tl }` into `\l_semio_window_number_tl` via `\tl_set:Nn` (no expansion), creating infinite recursion when `\semio_window_header_muted_use:` expanded the number for `\edef\semio@window@header@numval`.

**Additional expl3 bugs** (surfaced after hang fix):
- `\semio_window_register_write:nn` used `\alph{semio@window@slot}` in expl3 mode (`Missing \endcsname`).
- `\semio@window@kind@number` via `\edef` in expl3 context also failed; replaced with pure-expl3 `\semio_window_kind_number_set:n`.

**Fix:** Removed `tier_header` calls; header_store reads tls directly; `semio_window_kind_number_set:n` + `int_to_alph:n`/`int_use:c` for register paths.

**Verified:**
```bash
bun nx run @semio-tech/mit-bestand-bericht:build
# → zwischenbericht/dist/zwischenbericht.pdf + zwischenbericht-dark.pdf (~30s)
```

## Prior notes

- Multiple orphaned `tectonic` processes compete (watch + manual builds). **Kill all `tectonic` before rebuilding.**

## Raster artifacts

- `verify-paragraph-p1-16x-v4.png`
- `verify-paragraph-p1-8x-v4.png`
- `verify-cover-p1-12x-v4.png`
- `zwischenbericht-p1.png`
- `zwischenbericht-p5.png`
