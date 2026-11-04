# Shared Boundary Validation

## Final Results

- Normal checked-in ui-react configuration registers all new border, compositor, dock, menu and Select/Tabs regression entries.
- Latest real-App follow-up integration run: **19 files, 718 tests passed**. Includes all 565 shared shell/control tests, existing window/panel/layout/shell/control/menu suites, new geometry/state/clip tests and the independent Chromium flex-layout oracle.
- Final marker-only lifecycle follow-up: **29 focused tests passed**, including shared leases, independent roots, StrictMode, forwarded callback/object/cleanup refs, teardown ordering, RAF cancellation, attribute/style restoration and idle draining.
- Full normal UI suite after lifecycle integration: **53 files passed, one failed; 1279 tests passed, one timed out**. The sole timeout is the unchanged Diagram 20,000-node cooperative-projection case under a 15-second wall-clock budget. It passes isolated with the real compositor and a 60-second budget (12.40 seconds test time). A diagnostic-only compositor-disabled run also took 15.83 seconds, confirming host/test timing is not evidence of a shared-border functional failure. No Diagram implementation or test was changed; no unfiltered full-suite all-green claim is made.
- Final combined strict TypeScript check passes for WindowChrome, Resizable, Canvas, Select, Tabs, ContextMenu, Popover, ToggleGroup and Navbar, including their source dependency closure. Canonical styling, icon and manifest projections were generated from existing sources; no manual generated stubs or lockfile changes.
- Native actual-source dependency-free kernels: **11 Rust tests passed** on lead rerun. Independent lodash/Ajv native oracle passed earlier. Full native integration compilation/tests remain blocked by repository native-source capture exceeding its 256-character path limit. Native GPU runtime execution is unverified.
- Final whitespace check passes. Generated ticket outputs were removed; input scripts/configuration and markdown reports remain. The subsequent live Storybook request retains only its active server log while the service runs on 6011.

## Real Browser

Chromium 156 fixture uses actual React controls, compiled canonical Tailwind/CSS and real shell portals, without presentation mocks:

- Two stepped window silhouettes and navbar bottom boundary join; hover takes priority over active paint.
- Select overlay dimensions remain approximately 77×22 for a 77.375×22.391 frame despite descendant icon sizing selectors.
- A 400×160 shell-owned popup is reachable over neighboring window content; Select stays in the same portal and both dismiss correctly.
- Forced-colors hover resolves to system Highlight; reduced-motion mode is enabled in that check.
- Marker-only root renders **zero windows and three attached frames**, including adjacent standalone control groups; hover works without WindowChrome.
- Normal/forced-color screenshots reviewed before cleanup.

## Reproduction Inputs

- `run-ui-tests.mjs`: normal checked-in ui-react configuration plus existing explicit owner-policy contract; real setup/imports, no presentation mocks. Default budget 15 seconds; `SEMIO_TEST_BUDGET_MS` can explicitly select a different diagnostic budget.
- `browser-check.mjs`, `browser.tsx`, `browser-env.mjs`: isolated actual-browser integration using existing esbuild, Tailwind compiler/scanner and Playwright; Nix library setup is confined to this test helper, native defaults elsewhere.
- `native-check.py` and `native-oracle.test.ts`: actual-source native kernel and independent fixture checks.
- Focused development configs are retained as reproducible inputs. `diagram-baseline.config.ts`/setup are diagnostic-only; their disabled-compositor result is not included in production validation claims.

## Infrastructure Limits

`repo` launcher dependencies were restored with unchanged lockfiles. Native MCP remains unavailable (`bootstrap.md`), so this folder is a report location, not a falsely claimed registered/closed ticket. Full native build capture needs infrastructure repair before GPU integration can be accepted as fully verified. Rotated/skewed hosts remain outside the axis-aligned shared-boundary contract.
