# Chat + MCP navbar panel

## Summary

- Removed ambient `AgentPresence` chrome outside the layout (read as a second MCP footer).
- Added `ui.panelToggle.chat` in the navbar trailing chrome, immediately left of fullscreen (`NavbarTrailingChromeSlot`).
- Chat dock lives on `right-middle` with `AgentChatPanel` (`BasicChatPanel` + MCP presence header).
- Opening chat, details, or settings closes the other right-side dock anchors (Kinan mutual-exclusion subset).

## Verification

- Language-agnostic contract: `🧫️fixtures/🧭️shell-chat-navbar-toggle/🔣️.json`
- wgpu: `RightPanelKind::Chat`, navbar toggle, `shell_command_for_control` test
