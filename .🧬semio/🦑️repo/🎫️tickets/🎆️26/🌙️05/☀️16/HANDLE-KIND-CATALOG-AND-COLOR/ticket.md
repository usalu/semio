# Handle Kind Catalog And Per Handle Color

**Status:** Done

**Goal:** Every synced handle has a non-empty `handleKind`; kinds are defined in a separate catalog `{ id, name, color }`; optional handle `color` overrides the kind color for drawing.

**Files:** `elements/client/lib/board/rs/lib.rs`, `elements/client/lib/board/index.ts`, `elements/client/lib/board/index.tsx`, `elements/client/lib/board/play/index.tsx`, `.storybook/stories/elements/board/Board.stories.tsx`, `elements/client/lib/board/rs/pkg/*` (wasm-pack output when rebuilt).
