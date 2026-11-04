# Storybook Import Resolution Fix

## Cause

The live Mode story imported three play functions from `ui/🧱️elements/🎨️Canvas/🧪️tests/🎭️storybook-interaction/🟦️.ts`, but that referenced module did not exist. Vite therefore rejected the entire Mode story module. This was a genuine missing source file, not a browser cache problem.

## Fix

Added the authored interaction module with real checks for QuadLayout activation, maximize/restore, and measured transparent silhouette/content clipping. Kept the story's play functions rather than removing its interaction coverage. Added language-neutral feature scenarios and a permanent ui-react regression entry that checks every UI story's relative imports using the existing TypeScript parser, then imports the Mode story through Vitest/Vite and checks its executable play exports.

## Validation

- Red: the new import audit reported exactly the missing Mode interaction module before the fix.
- Green: both source-import regression tests pass; all UI story relative imports resolve.
- Strict TypeScript check of the authored interaction module passes; whitespace check clean.
- Real Chromium on the running Storybook at http://localhost:6011: Quad Layout, Maximize Stack and Content Through Glass all finish their play phase with no page exception, no HTTP 500 response and no Storybook error message. Quad Layout ends with Detail active, Maximize Stack restores its two stacks, and Content Through Glass keeps its measured one-stack scene.

`check-mode-stories.mjs` is retained as the browser reproduction input. Generated diagnostic output was removed; the active Storybook server log remains while the service runs.
