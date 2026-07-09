# Verify Log — Fix Print Paragraph Chip Alignment

## Changes (`print/tex/semio-window.sty`)

1. **Rule-based canvas paint** — Replaced `\colorbox` with explicit `\rule` + `\rlap` for caps, gaps, and row window bodies so fill cannot bleed past hairline strokes (`\fboxrule` padding was overshooting).
2. **Row window body frame** — Custom `\semio@window@body@begin/end` with side v-strokes, bottom hairline, and width-constrained `minipage` content (fixes 241pt overfull in cover columns).
3. **Expl3 bridge** — `\semio_window_body_begin/end:` defined after body macros; calls via `\csname semio@window@body@...\endcsname` without `\group_begin` (nested groups caused `Missing } inserted`).
4. **GenericWindow region moved** after body bridge definitions.
5. **Row-only custom body** — Non-row windows keep `tcolorbox` so long cover text (`kurzfassung`) does not create multi-thousand-pt overfull vboxes.

## Verify commands

```bash
cd mit-bestand/bericht/zwischenbericht
tectonic -Z search-path=../../../print/tex --outdir dist verify-cover.tex
tectonic -Z search-path=../../../print/tex --outdir dist verify-paragraph.tex
```

## Visual result

- `verify-paragraph-p1-8x-v3.png` — muted paragraph chips (`18 Interviews`, `Recherche`) align cleanly.
- `verify-cover-p1-12x-v3.png` — cover row value fields improved; some header overfull warnings remain from logo/header row layout (pre-existing scale).

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

- `verify-cover-p1-12x-v3.png`
- `verify-paragraph-p1-8x-v3.png`
