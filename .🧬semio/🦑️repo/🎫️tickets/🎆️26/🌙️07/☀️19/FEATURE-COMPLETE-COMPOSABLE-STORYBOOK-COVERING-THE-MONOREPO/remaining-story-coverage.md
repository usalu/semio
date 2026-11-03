# Remaining Public Visual Family Coverage

## Result

Added five explicit live-component CSF subjects and fourteen variants. Inspected their implementations, package exports, existing story conventions, translation bundles, and repository instructions first. No existing implementation, first-twelve story, configuration, runtime runner, manifest, or target was edited. This bounded guard covers seventeen known subjects, not all current/future exports.

| Subject | New variants |
| --- | --- |
| Aside | Default, Tones (note/tip/caution/danger) |
| NotFound | Default, WithDescription, LocalBackNavigation |
| Catalogue | Default, Empty, Palette (real draggable items with semantic icons and JSON payloads) |
| Pane | Default (folded), Open, Expanded |
| WindowMeasuresTree | Default, LoadingAndWaiting, Upward |

Pane uses the real PaneHost and real folding, resize, drag/reanchor callbacks, with reset. WindowMeasuresTree uses actual groups/leaves, numeric input, full-width reset, loading/waiting presentation and upward flow. All story-owned visible labels use existing English/German useLabel keys; native inputs/buttons are labeled and local navigation reports feedback politely. Existing global providers remain authoritative; no fake providers, portals, browser APIs or module mocks were introduced.

NotFound's local-navigation variant deliberately intercepts its back-button click at the surrounding capture boundary and switches a story-owned local view. It does not exercise navigateOwnedRoute or change browser history/location. Reset restores the missing view. This is necessary because the component exposes parentPath but no navigation callback; it demonstrates the real rendered back affordance without navigating the Storybook iframe.

## Exact Files

New:
- `🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️aside/🧪️.story.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️not-found/🧪️.story.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️catalogue/🧪️.story.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️pane/🧪️.story.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️window-measures-tree/🧪️.story.tsx`

Extended supplied files:
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/📚️story-subjects/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️story-subject-coverage/🟦️.test.ts`

Report: this file.

## Validation

Commands used `PATH=$HOME/.local/share/semio/launcher/bin:$PATH repo tools exec --light bun`.

- Test-first: extended JSON contract and count assertion before adding stories: **14 passed, 5 failed**, all missing requested story files.
- Final focused command: `repo tools exec --light bun -- test './🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️story-subject-coverage/🟦️.test.ts'`: **19 passed, 0 failed, 254 assertions**.
- Third-party Storybook CSF parser independently validates metadata, exported variants and story IDs against TypeScript source inspection. The guard additionally requires JSX use of every subject and imported/rendered PaneHost and WindowMeasureTreeGroup/Leaf companions.
- TypeScript createProgram using root compiler options and all five stories as roots: **zero diagnostics in new stories**, **15 existing dependency-graph diagnostics**. Full-graph certification remains blocked by existing generated declarations as supplied in handoff.
- Inspected complete final contents/diffs of all seven owned source/test/fixture files. Absolute owned paths are below 256 characters. No installs, generated artifacts, staging, commits or MCP lifecycle claims.

## Remaining Validation / Risks

Lead owns browser/dev/static integration of the fourteen variants. Focus checks on pane fold/drag/resize/reset, catalogue native drag behavior, local back/reset without URL change, numeric edit/reset, and group fold toggles.

At agent handoff, WindowMeasureTreeGroup emitted an unnamed icon-only fold button without expanded semantics. This was outside the agent's assignment.

## Lead Integration

The lead subsequently fixed that implementation with an instance-unique `aria-labelledby` relationship and `aria-expanded`, and added a browser play checking its accessible name, collapse/expand states, child visibility and restored input value. All fourteen added variants passed in the final 239-variant cold dev and static Chromium runs; Button autodocs passed separately in both. This resolves the specific handoff accessibility caveat, not an exhaustive accessibility audit. See `repair.md` for the final validation and infrastructure boundaries.
