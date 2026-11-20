# 🐚 Live OS Sweep at 1440×900

This sweep used the OS showcase from the previous ticket (`round3-os-live.tsx`), served by `apps-start.sh os` on port 5602 and driven by `os-sweep.js`. It covered 72 parts and 546 targets and found **66 failing checks**, all in 6 parts. The other 66 parts are clean.

1. **Mode-dock tabs** (mode-stack 18, display 18, mode-mixed 12). Hovering or focusing `button#mode-dock-tab-root-design` inside `div[data-slot=mode-dock-tab]` leaves the right side open over y 1.6–28.8, and 4–6 state segments leak into ancestors. The painted active stack shows the closed tab bottom at y=57 (`mode-dock-tab` overlay, 1..97.5). The window silhouette steps at x=97.5 from 57 to 58 to 59, and the body top runs at y=59 (97.5..720). That staircase has two 1 px jogs. The tab cap bottom and the body top must be one line.
2. **owned-dialog** (4): hovering or focusing `input#name` (a box itself) draws no stated outline. The void-element painter path is not used here.
3. **task-manager** (12): buttons such as "Suspend: plugin.cad#1" draw no stated outline on their own box.
4. **agent-chat** (2): `section#framework.approvals.a1` hovered: top open at 1.6–4.2 and 415.8–420, bottom open over the full width, and left and right open at y 236.9–317.2.
