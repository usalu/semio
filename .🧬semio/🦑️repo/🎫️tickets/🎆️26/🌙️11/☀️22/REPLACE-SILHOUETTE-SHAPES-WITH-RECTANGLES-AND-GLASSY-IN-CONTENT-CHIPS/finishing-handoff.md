# Finishing handoff: destructive ink, command rows, chromium launch, suite results

## 1 Semantic ink on glass (token level)

Root cause: the dark Delete row is `text-destructive`, which is the `danger` palette colour `#a60009`, 2:1 on dark menu glass.

- New palette colours in `🎨️styling/🔣️.json`: `danger-bright #ff8a90`, `danger-deep #8c0007`.
- Dark appearance `outcome.error` is `danger-bright` (the wgpu theme error ink, `OUTCOME_DARK`); light `outcome.error` stays `danger` (the DiffView removed colour is pinned at 166,0,9 by a fixture shared with wgpu).
- `🎨️styling/🖌️ui/🎨️.css`: light `--destructive-foreground` is `danger-deep`, light `--destructive-border` stays `danger`; dark foreground and border are `danger-bright`.
- Every `text-destructive` text use (ContextMenu item, Stepper, Field, Slider, UIDialog, tutorial recording) is `text-destructive-foreground` now; the border/fill uses keep `destructive`.
- Regenerated with the projection (`styling-emit.mts`, writes only changed artifacts): palette css, `🤖️generated`, `🔷️.cs`, `🦀️.rs` (not compiled), `🐍️.py`, and `📓️print/🖋️latex/semio-tokens.sty` (the print token test was also stale on the glass constants; green now). Print shares the new colours only as `\definecolor`; its `danger-success` schemes are unchanged.
- Test: `🎨️styling/🧪️tests/🧪️semantic-ink-contrast/🟦️.ts` (registered in the styling config), colord a11y as the third-party oracle: destructive, warning, success, info foreground and muted foreground, plus the wgpu error ink, against the window, pane, panel (chip) and menu glass levels composited over the base and window backdrops, at rest and hovered (hover for all but muted and the error ink). 168 cases green, was red for dark destructive (2.0:1) and light destructive on a hovered menu (4.1:1).
- Looked at: `evidence/finishing/contextmenu--grouped.dark.png` (Delete readable), `command--default.png`.
### Per-appearance ink tokens for warning, success, info (and destructive)

- `🎨️styling/🔣️.json`: new colours `warning-deep #544300`, `success-deep #2d452d`, `info-deep #483d32` (light), `success-bright #93c294`, `danger-bright #ff8a90`, `danger-deep #8c0007`. The appearance `outcome` group gained `errorInk`, `warningInk`, `successInk`, `infoInk` (light: the deep colours; dark: `danger-bright`, `warning`, `success-bright`, `info`). `outcome.error` is back to `danger` in both appearances (it is the stroke and diff hue; the Rust token parity test pins it). Mono premade (`🌓️theme/🔣️.json`) carries the four keys too (light `danger` #333, dark `warning` / `info`).
- CSS (`🖌️ui/🎨️.css`): `--{info,success,warning}-ink` per appearance, theme colours `--color-{info,success,warning}-ink`, so `text-info-ink` etc. exist; destructive text stays `text-destructive-foreground`.
- All text uses switched: every `text-warning|success|info` in `🧰️framework` (HistoryTable, AgentApprovals, Interpreter tree-walk, EventFeedHost, ShellHost shell model, ShellSync, TextEditor helpers, coordination story, draft-editor test) is `-ink`; borders and fills (`border-warning`, `bg-warning/20`) are unchanged.
- Rust consumers (edited, NOT compiled; agent aba571b1ee5f514d9 told to recompile): `🎨️styling/🔤️tokens/🦀️.rs` (generated, `OutcomePalette` + four `*_ink` fields), `🎨️styling/🎭️theme/🧩️component/🦀️.rs` (`Theme.{error,warning,success,info}_ink`, filled in `from_chrome`, `tone_ink` returns them), tests `🧪️tests/🔬️targets-wgpu-reconcile-unit/🦀️.rs` and `🧪️tests/🔬️targets-wgpu-theme-token-parity/🦀️.rs`.
- Test `semantic-ink-contrast` now: 284 contrast cases (destructive, warning, success, info foregrounds and inks, muted, the four wgpu inks; window, pane, chip, menu glass; over base and window backdrops; rest and hover) plus 8 cases that the CSS ink and the wgpu outcome ink are the same colour. Dark success needed the brighter variant (4.49:1 on a hovered menu). 327 styling tests green.
- Print: `semio-tokens.sty` regenerated (new `\definecolor`s only).

## 2 Command palette rows (law 12)

`⌨️Command`: the list is a vertical cells owner (`data-shared-border-cells`), every row a cell (`data-shared-border-cell`, `min-h-(--chip-row-height)!`, `px-single`), the group lost its padding so rows span the frame. Tests: `⌨️Command/🧪️tests/🧩️component` (new "rules every row as a cell"), and `🔎️ShellSearch` parity test updated (it pinned `p-single` on group and row).

## 3 Chromium launch

- `🖱️ui/🧪️tests/🌐️chromium-launch` now also serves `◻️2d` text alignment and `🧊️3d` viewer gpu tests.
- New `launchBrowserV1` in the repository-owned `🦑️repo/.../🧪️testing/🌐️browser/🟦️.ts` (resolves playwright through `PLAYWRIGHT_MODULE_SPECIFIER`, forces `channel: "chromium"`); 15 `🧑‍💻dev` files (journey, benchmarks browser, 11 tests, two-human worker and browser) use it instead of literal `chromium.launch`. They keep their own `args` and `headless`.
- Not routed, with reason: `🖌️raster` svg-export browser (runtime tooling, must not depend on a test helper; legacy shell is fine for its headless render); `📓️print` chart-mutations (playwright runs in an inline script string in a subprocess); `🧑‍💻dev/⚖️parity` execution (loads playwright through `loadPlaywrightV1(repoRoot, ...)` from another install); hub, course, presentation and plugin tests (outside the requested lanes, 20 files).
- Storybook frame visuals: the runner config `.storybook/🧪️tests/🧪️browser-runner/🟦️.ts` now sets `channel: "chromium"` in the chromium project, so the normal suite command works on this Mac without `STORYBOOK_CHROMIUM_PATH` (`frame-visuals-run.mjs` lost it). Static build recipe: `story-static-build.sh` (`storybook build` in prototype mode, `repo run` still fails on the graph). Result: 9 of 9 passed against a fresh build of the final CSS.

## 4 Suite results

All through `lockf -k /private/tmp/claude-501/semio-vitest.lock`, `PLAYWRIGHT_BROWSERS_PATH` set.

| Suite | Command | Result |
| --- | --- | --- |
| UI react | `compositor-vitest.sh` | 83 files, 1713 tests pass (rerun after the ink change) |
| OS renderer | `os-vitest.sh` | 3594 pass, 7 tests + 2 suites fail, exactly the environmental set of `os-suite-handoff.md` (package-integration x7, hidden-tab-surfaces and node-graph-hosts do not load) |
| Styling | styling config, ink + levels + theme-resolve | 327 pass |
| Print tokens | `print-tokens-run.sh` | 62 pass |
| Storybook frame visuals | `frame-visuals-run.mjs` on the static build | 9 pass |
| Hub admin | admin config | 29 pass |
| Presentation | `presentation-hub-vitest.config.ts` | 4 pass |
| Demonstrator | demonstrator config | 105 pass |
| Course | course config (also with `SEMIO_TEST_LEVEL=long`) | 20 files: 875 pass, 1 fails: `local-stack` "runs the same installed Vite CLI under each declared runtime" spawns the pinned `bun`, not installed here (environmental) |

### Why the course config runs 20 files, not 50

The course vitest config (`📚️course/🧪️tests/🎚️config/🟦️.ts`) is unchanged since HEAD and lists exactly 20 test files (`wave2-react-handoff.md` also recorded 20: 19 pass, 1 environmental). The earlier "50 files, 1191 tests" came from a wider ad hoc include, not from the config. I ran the wider set (`🧪️tests/**` plus the assignments builder test, 52 files): the 32 extra files are not vitest files. 16 are Playwright browser specs (`Playwright Test did not expect test() to be called here`, they belong to the end-to-end gate `📚️course/🎭️e2e`, which needs a dev server and is out of scope), 2 more use `test.afterEach`, and the rest are fixture/helper modules without a suite ("No test suite found", imported by the 20 real tests). So nothing is left that is meant to run under vitest; the 876 tests of the 20 files are the course's node suite. The ad hoc config was deleted.

One regression found and fixed on the way: the first OS run showed DiffView removed pixels 140,0,7 instead of 166,0,9 because light `outcome.error` had moved to `danger-deep`; it stays `danger` now.

Generated output deleted.
