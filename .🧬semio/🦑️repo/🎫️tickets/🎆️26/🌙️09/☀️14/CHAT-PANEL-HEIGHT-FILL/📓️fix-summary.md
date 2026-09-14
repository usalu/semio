# Chat panel height fill

## Problem

After docking chat on `top-right`, the panel body looked vertically squeezed: the message feed collapsed while the composer stayed visible. `BasicChatPanel` and `AgentChatPanel` use `h-full` / `flex-1`, but open corner panels only sized to intrinsic content height, so the flex chain never received a definite block size.

## Fix

- Pin open **corner** panels (`top-left`, `top-right`, `bottom-left`, `bottom-right`) with the opposite vertical inset (`bottom` for top corners, `top` for bottom corners) so they fill the layout region between navbar and footer.
- Use a flex column scroll viewport for all panel bodies (not only bottom-anchored panels).
- Align `createFrameworkChatPanelTab` empty-state hosting with `uiNodeToTreePanelConfig` (full-height flex wrapper + tree className).
- Give `AgentChatPanel`'s chat host `h-full` so the feed expands inside the pinned panel.

## Verification

- UI: `owned-locale-detector-retirement` — open corner panel pins opposite vertical edge.
- Contract: `🧫️fixtures/🧭️shell-chat-navbar-toggle/🔣️.json` fields `openCornerPanelPinsOppositeVerticalEdge`, `chatTreeEmptyStateHostClassName`.
