# Kitapp Error Boundary And Layout Recover

**Goal:** R26-02 / Running Sketchpad / Apps (repo MCP unavailable in agent env; ticket created manually.)

**Work:** ErrorBoundary logging + `errorRecoverKey` reset; LayoutCanvas `getAllComponents` fix; seed default `windowLayout` when missing; Playwright assertion; MCP `search`/`repo://goals` not reachable from tooling.

**Status:** closed

**Summary:** ErrorBoundary now logs `componentDidCatch` with `[DEBUG]` and dev-only message strip; recovery uses `errorRecoverKey` (kit id) instead of unstable `children` identity to stop reset/retry loops. Seeded default `windowLayout` once per kit when missing. Fixed LayoutCanvas active-tab walk (`getAllComponents(stack)`). Playwright `initHome` asserts kit error-boundary fallback is absent.

**Files:** `semio/sketchpad/index.tsx`, `.repo/🎫/26/05/09/KITAPP-ERROR-BOUNDARY-AND-LAYOUT-RECOVER/ticket.md`

**Tests:** Extended Playwright `sketchpad › Kit` to assert zero `data-testid="semio.sketchpad.kit-app.error-boundary-fallback"` after metabolism label is visible. Full `npm test` in this workspace still reports unrelated failures (fixture Zod, Home selection, folder kit tests); not attributed to this change.
