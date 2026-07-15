# Verify Log — Fix Print Paragraph Chip Alignment

## Muted chip fill overshoot — fixed 2026-07-10 (re-applied v5)

**Root cause:** `\semio@heading@cap@muted@core` used `\colorbox{semio-chrome-canvas}` which pads with `\fboxsep` (default 3pt), making fill `6pt` too wide / `3pt` too tall past hairline borders drawn via `\rule`.

**Fix (durable):** Replaced `\colorbox` with rule-based paint (`\semio@window@cap@paint` via `\rlap{\rule{…}{…}}`) in `\semio@window@cap@muted@vbox`. Side strokes use `\semio@window@cap@raise` for vertical alignment. No `\fboxsep` dependency — survives regressions.

**Verified:**

- `verify-paragraph.log` — no `Overfull \hbox (6.0pt too wide)` warnings.
- `zwischenbericht.log` — no `6.0pt` chip overfull warnings.
- `verify-paragraph-p1-16x-v5.png`, `verify-cover-p1-12x-v5.png`, `zwischenbericht-p1.png`, `zwischenbericht-p5.png`.

## Changes (`print/tex/semio-window.sty`)

1. **Rule-based muted chip paint** — `\semio@window@cap@paint`, `\semio@window@cap@muted@vbox`, `\semio@window@cap@raise`; `\semio@heading@cap@muted@core` no longer uses `\colorbox`.

## Verify commands

```bash
cd mit-bestand/bericht/zwischenbericht
tectonic -Z search-path=../../../print/tex --outdir dist verify-cover.tex
tectonic -Z search-path=../../../print/tex --outdir dist verify-paragraph.tex
```

## Visual result

- `verify-paragraph-p1-16x-v5.png` — muted paragraph chips (`18 Interviews`, `Recherche`) fill matches border.
- `verify-cover-p1-12x-v5.png` — cover window header-row chips aligned.
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

- `verify-paragraph-p1-16x-v5.png`
- `verify-cover-p1-12x-v5.png`
- `zwischenbericht-p1.png`
- `zwischenbericht-p5.png`
