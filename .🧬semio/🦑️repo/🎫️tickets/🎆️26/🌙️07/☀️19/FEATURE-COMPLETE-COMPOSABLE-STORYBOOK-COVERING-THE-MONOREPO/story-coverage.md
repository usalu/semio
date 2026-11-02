# Storybook Component Coverage Repair

## Result

Added twelve explicit component-subject CSF files with 31 variants. Each has exactly one default meta naming the real public component. No existing implementation, configuration, manifest, or browser-test file was edited.

Inspected the live status-surface, fullscreen/titlebar, tutorial UI/clock, hint, floating-surface host, locale-label, existing story, and onboarding gesture-declaration flows before writing stories. Used the supplied audit rather than repeating configuration/runtime investigation.

### States and Interaction

- Badge: neutral/default, all five semantic tones, icon.
- StatusChip: all four status values with translated status labels.
- StatCard: plain metric, positive delta/icon, negative delta.
- EmptyState: placeholder and action with visible callback feedback.
- ErrorView: non-retryable and retryable; retry increments an announced attempt count rather than pretending a failed operation recovered.
- DesktopTitlebar: browser/non-desktop without controls, and a story-owned window host whose callbacks minimize/maximize/close its own panel; reset restores it. These are component callback demonstrations, not mocked Electron/browser APIs.
- TutorialBar: paused/playable timeline, recording, zero-duration timeline. Uses the actual `createTutorialClock`, seeks, rate changes, pause/stop, mute/caption/record toggles, chapter creation, and disposal on unmount.
- TutorialCaptions: visible, explicitly hidden, no cue.
- TutorialGhostPointer: click, drag, orbit, no cue; normalized viewport points and adjustable progress. Gesture button/modifier fields match the live onboarding declaration.
- TutorialVideoOverlay: paused, playing, no source. Uses the existing checked-in logo animation MP4, bundled through a relative `new URL`, with a keyboard-operable play/pause control; no network fixture or substituted media API.
- ChromeControlHint: ordinary and `always` behavior, using its real delayed hover/focus/portal implementation.
- DisabledReasonHint: initially refused and initially runnable, with a labeled checkbox to change refusal in place; the action stays focusable with `aria-disabled`, describes its actual reason element, and refuses execution while unavailable.

Visible UI strings use existing `useLabel` keys and the existing English/German locale toolbar. Native buttons, labeled input, pressed/disabled semantics, and polite callback output are used where relevant. Standalone hints use the component's documented floating-host fallback; no fake provider, timer, clock, portal, or runtime mocks were added.

## Language-Neutral Coverage Contract

The JSON fixture fixes the twelve subjects, their separate canonical story files, and required named states. The new Bun test checks live-source imports, syntax, one default export/component meta, required states, and absence of module-mock calls. It independently validates metadata, story count, and resulting story IDs using the already-installed third-party Storybook CSF parser, cross-checking the TypeScript AST inspection. Incidental JSX without subject metadata is explicitly rejected.

This is a regression guard for the requested known set, not a claim of exhaustive current/future UI-export coverage.

## Validation

All commands used `PATH=$HOME/.local/share/semio/launcher/bin:$PATH repo tools exec --light bun`.

1. Test-first baseline: **2 passed, 12 failed**, because none of the twelve subject files existed.
2. Final focused test:
   ```sh
   repo tools exec --light bun -- test './🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️story-subject-coverage/🟦️.test.ts'
   ```
   **14 passed, 0 failed, 163 assertions.** Storybook's own parser successfully discovered all 31 variants.
3. TypeScript `createProgram` using repository compiler options and the twelve new stories as roots: **zero diagnostics in new story files** after fixing initial local typing errors. The dependency graph still has **15 diagnostics**, including absent generated manifest/UI-contract declarations and consequent implicit-any/exhaustiveness errors in existing sources. Therefore this is not a clean full-graph typecheck; gesture types depending on missing generated declarations are not fully certified.
4. Inspected the complete contents and owned untracked-file status for all fourteen new source/test/fixture files. No existing owned files were modified.
5. Formatting CLI was unavailable (`prettier` is not installed/resolvable). No package install or manifest change was attempted.

Browser health, runtime setup repair, generated preparation and full-build validation remain with the lead as assigned; none were duplicated or claimed here. No MCP ticket state was fabricated: full native MCP remains unavailable per handoff.

## Exact New Files

- `🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️badge/🧪️.story.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️status-chip/🧪️.story.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️stat-card/🧪️.story.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️empty-state/🧪️.story.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️error-view/🧪️.story.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️desktop-titlebar/🧪️.story.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️tutorial-bar/🧪️.story.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️tutorial-captions/🧪️.story.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️tutorial-ghost-pointer/🧪️.story.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️tutorial-video-overlay/🧪️.story.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️chrome-control-hint/🧪️.story.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️disabled-reason-hint/🧪️.story.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/📚️story-subjects/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️story-subject-coverage/🟦️.test.ts`
- This report: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️19/FEATURE-COMPLETE-COMPOSABLE-STORYBOOK-COVERING-THE-MONOREPO/story-coverage.md`
