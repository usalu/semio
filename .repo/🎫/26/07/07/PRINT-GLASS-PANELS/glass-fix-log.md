# Glass Panel Fix Log

## Root causes

1. **Pass-1 panel painted into raster source** — flat `fcolorbox` on shipout was baked into pass-1 PDF, so the glass crop blurred the grey box instead of page content.
2. **Opaque `fcolorbox` fill on pass 2** — `semio-chrome-panel!58` covered the embedded PNG.
3. **Shared glass dir for light/dark** — `report-dark` overwrote `report` PNGs with dark-theme tints.
4. **expl3 `\file_if_exist:n`** — unreliable for `.semio-panel-glass/...`; switched to latex2e `\IfFileExists`.

## Fixes

- Pass 1: manifest only; draw panel on pass 2 when PNG or `.ready` exists.
- Glass path: `.semio-panel-glass/\jobname/<id>.png` (per jobname).
- Glass shipout: `\fbox` border + PNG background; fallback `fcolorbox` only when glass step ran but PNG missing.
- `renderPanelGlass`: stronger blur, frost noise (`soft-light`), per-jobname output dir.
- Report demo: `y=5cm` so panel overlaps TOC content.

5. **Saturation multiplier**: `modulate({ saturation: glassSaturate })` — was `* 100` causing neon color corruption on light and dark themes.
