# Play Landing Grid Test

Play positions the overview grid below the navbar with `paddingBlockStart` and keeps the same inset when scrolling a card into view with `scrollPaddingBlockStart`. Swipe mode gets the navbar height through `PLAY_INSETS`. The landing grid test still searched for `paddingTop`. It now checks both logical properties and retains its swipe inset and card sizing assertions.

The browser acceptance suite checks the overview card count, navbar visibility through its accessible role, and each card’s accessible name. It also verifies that the overlay becomes visible when returning from a pane. These checks do not measure the navbar and grid bounds. This change affects the Vitest assertion only.

The focused landing grid test reproduced the failure before editing. After the edit, all 13 `brand.ts` tests passed through the repository’s registered Node executable.

```sh
repo tools exec --light node -- node_modules/vitest/vitest.mjs run --config '🏢️semio-tech/🎡️play/🧪️tests/🎚️config/🟦️.ts' brand.ts
```
