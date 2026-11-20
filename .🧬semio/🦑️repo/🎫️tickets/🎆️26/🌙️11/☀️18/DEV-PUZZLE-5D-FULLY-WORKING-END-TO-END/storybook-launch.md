# Storybook launch: dev puzzle 5d

Status: NOT verified end to end. The launch never got past the repo graph service queue.

## Static findings
- Project `repo-storybook`, target `dev-storybook-puzzle-5d` exists (port 6010, dependsOn prepare-storybook).
- Mapping: `dev storybook puzzle 5d` -> DevScript.parseStorybookSegments joins to scope `puzzle/5d` -> `.storybook/📖️stories/🧭️coordination/🟦️.ts` scope `puzzle/5d` (wasmPackages incl. `@semio-tech/puzzle-wasm`, titlePrefix `🧩️puzzle🕐️5d`, glob `✏️s/🧑‍💻dev/🧩️puzzle/📖️stories/🎭️5d-*/🧪️.story.tsx`). Matches the 5d-timeline story; same shape as puzzle/2d and puzzle/3d. No script change needed.
- Smoke test already exists: `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧊️storybook-spatial/🟦️.ts` (Playwright; `expectStoryLoads` + `puzzle5d-timeline-debug` assertions, nakagin scrub test and per-fixture parts/fasteners tests). No new test added.

## Run
- `repo run repo-storybook:dev-storybook-puzzle-5d` (log: 🗑️generated/sb-dev.log, pid sb-dev.pid).
- After about 17 minutes the process was still in `door.script` with "The graph service holds this request and is working on it" ("Waiting for a free slot in the resident service (9 ahead)"). Port 6010 never opened. Killed my PID.

## Fixes
None (no failure reached the storybook side).

## Remaining
- Re-run when the resident graph service queue is free; then curl index.json, run the storybook-spatial test against the 5d stories, confirm partCount>0 and the checkpoint click.
