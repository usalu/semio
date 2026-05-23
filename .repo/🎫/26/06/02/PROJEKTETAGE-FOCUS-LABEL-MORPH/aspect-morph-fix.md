# Aspect morph fix (8→9)

## Cause
Target ghosts used label layout (wide bar) while crop vars only differed via `--presentation-figure-bg-size-morph`; at `pending`/`running` the whole section forced morph bg vars on all figure slots. FLIP `scale(max(sx,sy))` cannot change box aspect, so tiles jumped to label aspect immediately.

## Fix
1. `revealMorphToFrame` on focus tiles (from next slide `morphFrom` slot positions).
2. `morphFrameCssVars` + `presentation-morph-frame-pair`: pending lays out target ghosts at source frame; running animates frame to label.
3. `presentation-morph-crop-from` / `presentation-morph-crop-to`: crop bg animates source→label on ghosts, focus→label on tiles.
4. Disable reveal FLIP transform on `presentation-morph-frame-pair` targets (CSS frame animation owns geometry).

## Regression (7→8 stole label morph)
`revealMorphToFrame` on focus tiles activated crop/frame CSS on every auto-animate involving the focus slide, including catalogue→focus (7→8).

## Scope fix
`presentation-arrangement--many-to-one-morph` is set only when `data-settle-before-morph-to` on the from slide lists the to slide `title` (arrangement id). Crop morph CSS requires that class (not 7→8).

## Position/size (fade in place)
Removed CSS frame morph + `transform: none`; position/size uses reveal FLIP again.

## Aspect flicker (vertical → horizontal snap)
Target ghosts used label-frame crop at rest (`--presentation-figure-bg-size`) and switched to source crop at `pending` (`-morph`), causing Stütze to flicker. `figureCropBackgroundVarsTargetGhost` sets rest=source frame, morph=label slot; `pending` keeps rest (source), `running` animates rest→morph. Focus tiles unchanged (rest=focus, running crop-to→label).

## Verify
- 7→8: catalogue splits into focus tiles (one-to-many), no flight to label slots.
- 8→9: focus tiles morph aspect/frame into per-tile label ghosts, labels fade in.
