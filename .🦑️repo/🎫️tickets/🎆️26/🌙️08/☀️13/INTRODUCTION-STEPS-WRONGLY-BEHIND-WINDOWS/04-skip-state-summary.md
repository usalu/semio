# Summary: Keep Skipped Demonstrator Introductions Dismissed

## Result

The consolidated shell state now models automatic introduction launch separately from active step presentation. Skip transitions the active step to `null`, and subsequent effect executions for the same demonstrator brand/app launch are reducer no-ops instead of resetting the tour to step `0`.

## Changes

- Added the `AUTO_START_INTRODUCTION` event and per-shell launch-key ledger to the overlay slice.
- Kept manual `SET_INTRODUCTION_STEP` transitions for Next, Back, Skip, Done, and Introduce App.
- Removed the active step index from the auto-start effect dependencies.
- Preserved focus suppression: a backgrounded demonstrator pane dismisses an active tour, and restoring focus does not auto-restart the already-claimed launch.
- Added reducer regression coverage for first auto-start, Skip, repeated same-key auto-start, and a distinct app launch resetting to step `0`.

## Verification

- `bun nx run @semio-tech/framework-renderer-react:test-quick --skip-nx-cache -- -t "auto-starts each introduction launch once"`: passed, 1 test.
- `bun nx run @semio-tech/framework-renderer-react:test-quick -- -t "shell store reducer"`: passed, 24 tests.
- `bun nx run @semio-tech/framework-renderer-react:lint`: passed.
- Runtime: focused the Aggregator in the running mit-bestand demonstrator, clicked Skip on “Die 3D-Ansicht”, observed one dismissal event, and confirmed the title/button remained absent after 1.2 seconds.

## Verification Constraints

- The unfiltered renderer quick suite exceeded its repository-enforced 30-second budget before reporting results; focused suites passed.
- The demonstrator production build remains blocked before bundling by an existing Node strip-only parsing error in `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts` (`constructor(readonly type...)`).
- Repo MCP repeatedly failed its startup handshake with a broken pipe, so the existing ticket could not be formally reopened or closed through MCP.
