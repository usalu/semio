# Coverage Audit

Read-only scout evidence; source inspection only. Paths below relative to `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/`.

## Native Renderer
- `🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🧮️axis-solving-and-stack-rendering/🦀️.rs`: sibling frames reserve separator width (21–57); render_axis and resize walk pass theme.gap_standard (104–136). Keep independent centered 20px resize hit areas. Each stack paints its full silhouette at 297, active/normal only, no shared-edge/hover resolution.
- Other Dock geometry callers in `🏷️tab-chrome-and-layout-edits` and `🔲️window-silhouette-and-stack-geometry` also pass gap_standard. Must change consistently, not paint alone. Existing geometry tests expect separator plus body insets; resize tests cover 20px centered hit.
- `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🍱️view-chrome/🧱️chrome-paint-and-navbar/🦀️.rs`: chrome items unconditionally paint four normal edges (91–105), despite hover/active flags. Panels delegate here. Floating panel rects deliberately use panel_inset and column gaps; do not erase detached outlines.
- `.../🧭️navbar-footer-and-overlays/🦀️.rs`: navbar buttons vertically centered, navbar bottom independently painted; logo and role chip gaps prevent assumed flush attachment.

## React Product Wrappers
- `🧱️elements/🏛️ShellHost/📋️panel-hosting/🔶️brand/🟦️.tsx`: logo/name/role groups have gap-single, name px-single, center controls/pane tabs gap-double. Decide which edges truly touch; compositor cannot merge positive gaps.
- `🧱️elements/🏛️ShellHost/🖼️canvas-footer/🔸️canvas/🟦️.tsx`: studio/focused workflow bars use borderNormalBottomClass above docking. Mode wrapper fills width/height, no independent docking gap there.
- `🧱️elements/🏛️ShellHost/🟦️.tsx`: delegates shared Layout/Navbar/Footer.
- Repository-root `🏢️semio-tech/🎡️play/🟦️.tsx`: logo/name/count gap-single, name px-single; globals stylesheet only imports OS renderer CSS.

## Scope
No active delegated React implementation inspected by this audit. No native XAML implementation found under OS; not an exhaustive repository inventory. Native dock/chrome changes need explicit ownership and validation.
