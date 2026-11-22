# Generated Rust Mount Prerequisites

Cause: Rust `#[path]` mounts (icons, ui styling tokens, trinity, reasoning, writer, puzzle, draw, flow) point at untracked generator output. Nothing generated it before the dev playground read the Rust source closure, so it failed with `process.source.unresolved-module`.

Fix (generic, not icon specific):
- `native-build/📥️source/🏭️generated-owner/🟦️.ts`: `missingMountedModulesV1` finds missing `#[path]` targets, `generatedModuleOwnersV1` names the owning `project:target` (a target whose outputs cover the file, else the `generate` target of a nearby project when the file is generated or git-ignored), `generatedPrerequisiteArgumentsV1` turns them into neutral `run` requests.
- Source reader throws the new catalog key `source.unresolved-generated-module` (en/de) naming the target; plain missing modules keep `source.unresolved-module`.
- Dev playground: `ensurePlaygroundGeneratedInputsV1` runs those owners as owned neutral children before components, descriptor emitter and wgpu renderer; `caller/📤️child` declares the same launches for `dev playground` (same pattern as the Storybook engine descendants).
- Tests: `🏭️generated-owner/🧪️tests/🟦️.ts`, registered in `test-cargo-source-admission` runtime entries. Oracle: `git check-ignore` is the third-party ignore check used by the implementation.

Validation: repo MCP fails with "Closed host capacity Policy required: /swap/activeMegabytesPerSecond" (host capacity policy), so native bun ran the tests: 2 new pass; the existing suite shows one unrelated failure (shared engine slot fixture, another session's work).
Not verified: a full `dev playground` launch; the scan costs about 12 s on this repo.

## Follow-up: declared launches and live launch

`Undeclared original Repo build selection` came from declaring the generator children in the wrong place: `repo ./📜️script.ts dev playground ...` is born by `🎛️commands/🛠️dev/🌱️birth` (`rootDevPlaygroundFreshCapabilitiesV1`), not by `createBuildBootstrapCapabilitiesV1`. The declaration now lives there (`launches` gains `["repo","run",<owner target>]` per missing generated mount, from the same `generatedPrerequisiteArgumentsV1`); the earlier edit in `📥️caller/📤️child` was removed.

Live run of `SEMIO_LANGUAGE=en repo ./📜️script.ts dev playground process3d react prototype` (log: 🗑️generated/dev-generated-inputs.txt): the error is gone and the first generator child (`@semio-tech/draw-drawing-rs:graph-generate`) was launched, but it then queued for over 8 minutes behind host capacity (memory pressure, load above 20, several other runs). Port 6022 never started listening within that window, so the end-to-end result is NOT confirmed. I killed the process I started. Re-run when the host is idle, or after `repo run-many --targets=generate,graph-generate` has restored the outputs (the scan then declares no children).
