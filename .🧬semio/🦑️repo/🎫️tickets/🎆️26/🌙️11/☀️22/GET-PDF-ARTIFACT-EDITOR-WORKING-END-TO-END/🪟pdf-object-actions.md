# Object window action ownership

The title field lives in the Object pane (`pdf.object`) and commits `set-info-field`. That verb was declared only on the page window (`pdf.page`). The shell addresses a commit to the focused window, and the guest refuses `window kind pdf.object does not own action set-info-field`.

`inspector_window_definition` now owns the same edit verbs as the page window. A fresh wasm build is required before the browser journey can commit the title.

The journey in `pdf-pane-journey.ts` now also checks a real page size (612 by 792), reopens the saved file, and requires both pages of `two-pages.pdf` (`Page 1 / 2` and `Page 2 / 2`, media box 595).
