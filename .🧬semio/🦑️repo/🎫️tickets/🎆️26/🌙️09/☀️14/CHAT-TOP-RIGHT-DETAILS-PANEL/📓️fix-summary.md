# Chat panel top-right dock fix

## Problem

Chat used a dedicated `ui.panelToggle.chat` in `Navbar.trailingBeforeFullscreen` while the panel body lived on `right-middle`. Side-middle anchors are not in `PANEL_TAB_BAR_HOSTS`, so the floating panel always carried its own tab strip — two Chat toggles and a panel opening mid-right instead of under the top-right Details chrome like Inspektion.

## Fix

- Mount `frameworkChatTab` on `top-right` with `detailsRightTabs` (`tabBarHost: "chrome"`).
- Clear `right-middle` default dock slot for chat.
- Remove the trailing navbar chat toggle from `ShellHost`.
- Update language-agnostic fixture `🧫️fixtures/🧭️shell-chat-navbar-toggle/🔣️.json`.

## Follow-up

wgpu shell still exposes `ui.panelToggle.chat` in retained chrome; React path is corrected first.
