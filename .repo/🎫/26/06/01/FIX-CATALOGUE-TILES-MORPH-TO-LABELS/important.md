## Layout centering (2026-06-01)

- `centerResolvedArrangement` in core: centers visible placement bounds on every positioned slide.
- Positioned sections: flex center + canvas `margin: 0 auto`.
- Figure tiles: cover-fill at rest when frame/crop aspect differs (centered focal point); stretch when aspects match (grid).
- `morphParticipant` duplicate tiles hidden (`presentation-figure-tile-frame--morph-participant-duplicate`) so only the catalogue participant tiles draw on Bauteilarten.
