# Generator target has no project owner (puzzle-2d-rs:graph-generate)

## Root cause
The admission graph read (`--exclude-dependencies`) hands `emojiProjectJsonNodes` a file list that holds Cargo manifests but not the sibling `📋️project.json` files (siblings were only added outside admission). The manifest walk then built a manifest-discovered stand-in project named after the Cargo package (`semio-s-artifact-puzzle-2d`, targets build/check/test) at the root of `@semio-tech/puzzle-2d-rs`, because `declaredRoots` did not know that root. The generator contract `semio-s-artifact-puzzle-2d-graph-catalog` matches by `ownerPath`, so `projectWithDefaults` ran it against the stand-in and threw "Generator target has no project owner: @semio-tech/puzzle-2d-rs:graph-generate". Confirmed with temporary [DEBUG] logs (removed): jsonName=semio-s-artifact-puzzle-2d, admission=true, 7 project files in a list of 963.

Two wrong hypotheses were tried and reverted: (1) project JSON reduced to selected targets (jobs carry the full JSON); (2) skipping the stand-in when a project.json exists on disk (then dependency roots lose their owner: `BUILD_OWNER_REGISTRY_INCONSISTENT ... Native dependency has no build project owner: ...contract/editing/rust`).

## Fix
`graph/inference/project-nodes/🟨️.mjs` (`setupPass`): the sibling `📋️project.json` of every listed Cargo manifest is now a candidate in the admission read as well, so the real owner is declared and no stand-in is built at its root. Jobs stay limited to wanted roots; other roots only feed `projectsByRoot`.

## Tests
`graph/inference/project-nodes/🧪️tests/🦀️cargo-stand-in/🟦️.ts` (repo bun, 1 pass, 3 expects). The candidate rule is now the exported `projectConfigCandidatesV1` (used by `setupPass`): the test lists a Cargo manifest without its sibling project file in a temporary tree and asserts the sibling `📋️project.json` is a candidate (a lone manifest without one gains none), that `projectWithDefaults` holds the generator contract of the root for the real owner, and that the package-named stand-in refuses it with "Generator target has no project owner". Calling `emojiProjectJsonNodes` itself was tried and abandoned: past the setup it needs bounded graph writer custody and a captured runtime entry provider (native-build storage), i.e. the full harness.
Admission test file: 19 pass, 1 unrelated failure (native memo, nativeDependencyRoots).

## Runs (logs in 🗑️generated)
- genown-3/4/5 (fleet off, debug): reproduced, EXIT 1 with the owner error.
- genown-6 (stand-in fix attempt 1... root fixed, fleet off): past the owner error, EXIT 75 (heavy owners need more memory than this host gives one inference; fleet off).
- genown-7 (skip variant): EXIT 1 BUILD_OWNER_REGISTRY_INCONSISTENT (hypothesis 2, reverted).
- genown-8 (final fix, fleet on): no "no project owner"; reads 654-672 of 959 owners, then waits for the fleet to infer @semio-tech/stdio-txt-rs, @semio-tech/puzzle-2d-rs, @semio-tech/puzzle-5d-rs. Still running; final EXIT not yet known.

## Status at report time
genown-8.log (fleet on, final fix) had no EXIT yet. It cycles through graph reads (taken 731 of 959) and then waits for the fleet to infer @semio-tech/stdio-txt-rs, @semio-tech/puzzle-2d-rs and @semio-tech/puzzle-5d-rs, which no fleet machine delivers. No non-fleet error occurs. The process is left running; add its EXIT here when it ends.
