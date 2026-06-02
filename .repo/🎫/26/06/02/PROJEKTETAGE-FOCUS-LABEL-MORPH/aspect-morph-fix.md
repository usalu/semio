# Aspect morph fix (8→9)

## Cause
Target ghosts used label layout (wide bar) while crop vars only differed via `--presentation-figure-bg-size-morph`; at `pending`/`running` the whole section forced morph bg vars on all figure slots. FLIP `scale(max(sx,sy))` cannot change box aspect, so tiles jumped to label aspect immediately.

## Fix
1. `revealMorphToFrame` on focus tiles (from next slide `morphFrom` slot positions).
2. `morphFrameCssVars` + `presentation-morph-frame-pair`: pending lays out target ghosts at source frame; running animates frame to label.
3. `presentation-morph-crop-from` / `presentation-morph-crop-to`: crop bg animates source→label on ghosts, focus→label on tiles.
4. Disable reveal FLIP transform on `presentation-morph-frame-pair` targets (CSS frame animation owns geometry).

## Verify
Load projektetage, advance slides 8→9: tiles should morph size and crop into column label ghosts without instant aspect snap.
