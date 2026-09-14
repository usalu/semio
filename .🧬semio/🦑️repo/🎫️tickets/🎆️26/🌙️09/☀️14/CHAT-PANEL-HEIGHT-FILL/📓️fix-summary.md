# Chat panel height (revised)

## Problem

Pinning open corner panels to full region height made **every** dock panel stretch navbar-to-footer. Chat still looked squeezed because `BasicChatPanel` relied on `h-full` / `flex-1` without a definite parent height in content-sized panels.

## Intended behavior

- Panels **grow with content** until they hit the layout region `maxHeight`, then the panel body **scrolls**.
- Chat message feed keeps a **minimum readable height** and grows with messages; it scrolls inside the feed when messages overflow the feed box; the whole panel scrolls when total chrome exceeds region max.

## Fix

- Reverted corner panel top/bottom pinning and non-bottom scroll viewport flex stretch.
- `BasicChatPanel`: content-driven column layout; feed uses `min-h-huge` + `overflow-y-auto` instead of `flex-1` / `h-full`.
- `AgentChatPanel` / `createFrameworkChatPanelTab`: removed full-height flex wrappers.

## Verification

- Contract: `🧫️fixtures/🧭️shell-chat-navbar-toggle/🔣️.json` — `panelVerticalSizing`, `chatFeedMinHeightClass`.
- Storybook: `BasicChatPanel` stories still wrap with explicit height for isolated preview.
