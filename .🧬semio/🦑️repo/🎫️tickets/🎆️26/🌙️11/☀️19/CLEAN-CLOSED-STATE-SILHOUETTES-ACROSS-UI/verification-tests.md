# Verification: Tests and Type Checks

Run natively with `node_modules/.bin/vitest run --config <cfg>` and `node_modules/.bin/tsc --noEmit -p <tsconfig>`. A first run of all suites in parallel caused load timeouts (course 11 failures, teaching 3); the numbers below are from quiet re-runs. Concurrent tidying deleted `🗑️generated` twice during the run, so raw outputs are not kept.

## Suites

| Suite (config) | Result | Notes |
|---|---|---|
| UI, `🧱️elements` + `border-presentation` + `owned-locale-detector-retirement` (`vitest-ui.config.ts`) | 1268 passed, 5 failed (before fix: 1 more the celebrate test after my edit, see below) -> 4 failed, 1 skipped | 4 failures need Playwright Chromium: Tree `text-flow` x3, Layout `component` x1. Other failing files are `bun:test` or helper files, as before |
| UI full config (everything under 🖱️ui) | 14 failed tests: 8 stale CSS/class assertions (fixed, below), 4 Chromium, `prepared-readback` and `text-advances` (Chromium), TUI `pointer-routing` (unrelated, see below) | |
| course (`vitest-course.config.ts`) | 1154 / 1155 under full run; the one `load-failures` test passes alone (6/6) | known full-suite timeout under load |
| OS final | 158 / 158 | |
| interpreter in-source | 196 / 197 | corpus count (78 vs 77), someone else's fixture |
| demonstrator | 29 / 29 | |
| teaching | 275 / 276 | `catalog loading` (invalid asset, data under edit) |
| hub-admin | 27 / 28 | `command-routing` (Nx test level), environment |
| OS elements | 292 / 294 | both need Chromium (MediaTransportHost lifecycle, empty-dock notice) |
| OS engine (`vitest-os-engine.config.ts`) | 2579 passed, 49 failed, 59 skipped -> after fix 48 failed | 48 are not from this ticket, see below |
| presentation (`vitest-presentation.config.ts`, new) | 197 / 197 after fix | 13 failing files are helper or kit files with no suite (as in other products) |
| cad-renderer aside frame | 4 / 4 | copied `cad-aside-frame.test.ts` from the 17th ticket |

## Fixes (all stale assertions of this ticket's removed state lines; no product or framework source changed, `shared-border.ts` and `🎨️.css` untouched)

- `🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🌳️owned-locale-detector-retirement-and-more/🟦️.tsx` ~371-377, ~407-410: celebrate `::after` padding/animation rules, `@utility border-loading|waiting` and the introduced inset box-shadow are removed in the CSS, now asserted absent.
- `.../📦️control-chrome-1/🟦️.tsx:143`: pressed toggle no longer carries `data-[state=on]:border-active-base`. `:375`: the window `:hover` normal-border rule is gone, asserted absent.
- `.../🪟️uiintroduction-appearance-1/🟦️.tsx:~279-289`: hover emphasis is the compositor's `[data-shared-border-segment][data-shared-border-state="hover"]` stroke; no window `:hover` rule.
- `.../🪄️tree-helpers-1/🟦️.tsx:453,460`: `border-loading-active` and `border-waiting-active` are now plain aliases of `border-loading|waiting`.
- `💻️os/.../🧪️tests/⚙️puzzle3d-settings-document/🟦️.tsx:54`: Stepper buttons are `self-stretch` (cells fill the group), no longer `h-medium`.
- `🎤️presentation/🧪️tests/♻️enlarge-and-reset-interaction/🟦️.tsx:213`: the actions chip group has no CSS `border` (the compositor frames it; removed in the previous ticket), asserted absent.
- New config `vitest-presentation.config.ts` (jsdom, presentation setup file).

## Remaining failures, not fixed

- Playwright Chromium missing: UI Tree `text-flow` x3, Layout `component` x1, `prepared-readback`, `text-advances`; OS elements 2 (see above); OS engine Chromium and wgpu oracles (`text-input-oracle`, `native-theme-invalidation`, `platform-prefs`, `browser-cursor-presentation`, `gpu-animation-clock`, `wgpu-worker-step-budget`, `retained-toggle-semantics`, `diff-view-produced-surface`, `chrome-palette` x3, `icon-svg-lighting`, `wgpu-browser-media`, `asset-cancellation`, `retained-frame-progress`).
- Unrelated source-text, wgpu or package contracts in OS engine (none touch a file of this ticket): `settings-general-layout` (expects `inset = "var(--panel-inset)"` in `anchorPositionStyle`), `shell-holds` (cap row padding 92px vs 95.2px, `uiSpacingPx`), `grip-focus` (viewport walk), `tree-row-rects`, `wgpu-document-reconcile`, `wgpu-document-owner-move`, `wgpu-u64-seam`, `wgpu-runtime-assets`, `wgpu-extension-store-door`, `package-integration` x6 (Bun pin), `shell-component-decomposition`, `procedural-descriptors`, `plugin-runtime-suite`, `hub-projection`, `driver-editor`, `presented-media-slots`, `turn-patch-batch`, `embedded-mount`, `node-graph-gestures`.
- UI TUI `pointer-routing`: "the seam opens under the active tab" (row char `─` vs space). The TUI engine and fixture `🧫️fixtures/🖱️pointer-routing/🔣️.json` and `tui` Rust sources are under concurrent edit (newer than this ticket's design.md); design.md lists TUI consumers as still open.
- hub-admin `command-routing`, teaching `catalog loading`, interpreter corpus 78 vs 77: as in the baselines.

## Type checks (`tsc --noEmit`)

Projects: ui react (`🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript`), course, OS engine react. All three exit with errors; the repo has no tsconfig for the presentation or the whole ui package. The bulk is missing generated modules (`🤖️generated/🪪️manifest`, `🎭️actor`) and their implicit-any cascade (OS 546 lines, ui 60, course 18). Intersected with the changed-file list, no error is on a line this ticket changed and none concerns `sharedBorderlessBoxProps` or `useSharedBorderBox` (both resolve through `@semio-tech/ui-react` and `/chrome`). Errors in files this ticket touched:

- `🖱️ui/🧱️elements/🪢️SharedBorderRow/round3-framework.test.tsx:36` string not assignable to `UiLabel` (same class of error exists in untouched tests, e.g. `app-mode-nav.test.tsx`).
- `🖱️ui/🧱️elements/🎨️Canvas/🧪️tests/shared-border-dock.test.tsx:53` `dataset` on `Element`; `:154` `ModeWindowDescriptor[]` shape.
- `🖱️ui/🧱️elements/🥞️LayeredOverview/🧪️tests/🧩️component/🟦️.tsx:231` `IntersectionObserverEntry` cast.
- `🖱️ui/🎯️targets/⚛️react/🎓️introduction/🟦️.tsx:177,337,397` and `🎬️introduction-demo/🟦️.tsx:31` implicit any, from unresolved generated types.
- OS: `ShellHost/🟦️.tsx:279,300,314` (NoticeStack `orientation`, `actionProps` needing `id`), `📇️directory-bootstrap:89`, `🖼️canvas-footer/🔸️canvas:59,88,164,166`, `🔌️extension-runtime:38`, `📝️staged-args/🟦️.tsx:259-336`: all `AppDefinition`/`unknown` cascades from the missing generated manifest module, or Notice props not touched by this ticket's lines.
- Not fixed; none caused by the mechanism change.
