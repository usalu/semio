# Fullscreen Icons In Window Name Chips

Fullscreen icons now belong to the actual name chips of both overview cards and opened video windows. The overview name chip contains maximize; the opened name chip contains an icon-only native restore button beside its title. There is no separate right-cap restore chip. Ansehen remains a text-only chip on the right.

The cursor targets the name-chip icon. Clicking an opened window name alone does not restore it; pressing and releasing its embedded icon does. Names truncate safely while the canonical icon button stays visible. The title cap keeps its canonical 22.4px height and the body geometry is unchanged. Other products retain their default card behavior.

[Updated 8.25-second preview](./title-chip.mp4) · [Overview](./title-chip-overview.png) · [Opened window](./title-chip-opened.png) · [Icon interaction](./title-chip-control.png). Content-only playback and the opening/closing camera moves are preserved.

Actual Chromium tests verify English/German names and actions, name-only clicks, embedded-icon hover/press/release, topic and video opening from header icons, text-only view footers, shared defaults, and independent geometry/pixels. A cap-height regression was caught and fixed before final geometry validation. The preview also decodes all 66 frames, preserves its eight source frames and camera joins, and retains identical overview boundaries. See [tests](./title-chip-tests.md), [implementation](./title-chip-implementation.md), [media verification](./title-chip-verification.md) and [visual audit](./title-chip-audit.md).
