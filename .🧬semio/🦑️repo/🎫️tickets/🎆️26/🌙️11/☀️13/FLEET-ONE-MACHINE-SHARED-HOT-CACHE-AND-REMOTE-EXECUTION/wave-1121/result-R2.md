# R2: platform-neutral remote action format and runner

## Action format (argv, no shell, every OS)
- Command: program `@tool:rustc` (resolved by name from the worker's tools), compiler arguments, environment, empty working directory (relative to the input root when set).
- Paths below the worker's private input root are spelled `@root` or `@root/<relative posix path>` (at the start of a value, after `=` or after `,`). The worker maps them to native separators at execution (`remoteExecutionNativePathV1`, `expandRemoteExecutionRootV1`); the encoding and the action digest stay platform independent. Windows mapping is covered by a `path.win32` table test.
- The one producer is `offloadCommandV1` (`🚀️offload/📜️action`); the shell script and `tool.environment` are removed from the policy, no dual format. Order of arguments equals the old script (working-directory remap, macOS object prefix only for an `apple-darwin` target, plan arguments, shipped-directory remaps, absolute source last).
- Validator (`🧱️actions`) checks the program, `@root` remaps, the oso prefix, sources below the root, `HOME=@root/.tmp`, manifest and OUT_DIR paths below the root, and the root report variable.
- The worker writes the root report file and creates the declared output directory (`policy.root.directories`), as the script did.

## Sandbox and Windows
- Task directory is 12 random hex characters, the input root is `r`, long worker names are cut to 16 characters with a hash (`remoteExecutionWorkNameV1`), so paths stay inside the 256-character budget.
- Input symlinks are placed after the tree is built by `placeRemoteExecutionLinkV1`: a link where allowed, otherwise a copy of the target (files and directories); no privilege needed. Outputs are reported as POSIX relative paths on every OS.
- `isolated-root` on win32 passes argv, env and cwd through (test).

## Refusals removed
- `remoteExecutionActionPlatformWhyV1`, the `action-unsupported` code (type, texts, JSON schema enum), the agent and readiness special cases, the command's platform why and the win32 line in `🚀️offload/🟦️.ts`. Placement already keys on platform properties: a macOS unit finds no compatible Windows worker as a normal placement result, a Windows requester reaches a Windows worker (placement test).

## Tests
All non-cluster files pass (see landings.md). Determinism tests run real rustc through the argv form and are byte-equal to local builds. Not caused by R2: `⌨️command/🧪️tests` serve test times out (also before), `🧪️tests/🟦️.ts` ByteStream test hit its 5 s limit once under load.

## Follow-up: audit defects D1-D9
- D1 native flavour is required and passed by the worker; D2 input links that leave the root (absolute, UNC, drive, `..`) are refused before placement, in the worker (status 3) and in the placement function; D3 env values with `@root` must be clean whole-value paths, package text is never expanded; D4 sources, `-L`, `--extern`, remaps use one strict relative check, every placeholder in any argument is validated; D5 inputs never carry an exec bit; D6 remapped names are spelled with `/`, `@root`-relative remaps only; new cross-OS digest test (posix and win32 requester, one digest); D7 root paths end at a comma; D9 count corrected (bazel flags run 4 tests).
- Known limit: on Windows the remapped names in compiler output use `/` spelling, so they can differ textually from a local Windows build; D8 (English-only diagnostic refusals) unchanged.
- Test counts now: actions 10, worker 5, sandbox 6, why 10, placement 7, scheduler 8, wire 8, offload 27, determinism 5, action 2, toolchain 4, bazel flags 4, refusal-reason 3.
