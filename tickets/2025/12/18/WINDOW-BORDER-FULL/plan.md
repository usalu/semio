# Previously

- GoldenLayout window stacks were framed using an outer border while the GoldenLayout root uses clipped layout containers.
- Bottom and right frame edges could disappear due to clipping/rounding at the container boundary.

# Plan

- Keep semantic window borders but ensure the frame is rendered inside the window surface so it cannot be clipped.
- Update GoldenLayout styling to use an inset 1px stroke for the stack container.
- Document the window border mechanism in root dev docs.

# Changes

- Updated `js/js/globals.css` so GoldenLayout stack windows render the frame as an inset stroke (via inset shadow) instead of an outer border.
- Adjusted `js/js/globals.css` to render the inset stroke via a `::after` overlay frame after the shadow-based frame was not visible.
- Fixed a broken `.lm_item.lm_stack` CSS block (missing closing brace) and consolidated the GoldenLayout stack frame to a single `::after` inset stroke implementation.
- Restored GoldenLayout chrome and content backgrounds to use the window/base background tokens (instead of `transparent`) while keeping the inset stack frame.
