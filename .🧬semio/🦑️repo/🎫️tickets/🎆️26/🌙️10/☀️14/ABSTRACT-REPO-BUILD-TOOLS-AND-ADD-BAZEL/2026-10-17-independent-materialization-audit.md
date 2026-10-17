# Independent Task Materialization Audit — 2026-10-17

Read-only source comparison of Bazel task materialization against the checked-in Nx executor bridge and installed Nx 23.2.0 exec implementation. No build, native operation, tooling store action, or runtime CPU gate was run. Configuration merging, per-command forwarding flags, and dependency traversal are excluded because another agent owns them.

## Material Finding: Bazel Exec Reinterprets Literal Callback Arguments as Build Tokens

Priority P2. `🏗️build/🧭️selection/🟦️.ts:60–64` translates `repo exec --projects=app -- <literal argv>` into a synthetic `repo-exec` command by quoting each supplied argument. This promises the quoted values remain literal. `🏗️build/🔧️tools/🏗️bazel/📜️script.ts:285` subsequently calls `interpolate(command, workspace, task)` on the entire constructed shell command, including those quoted callback arguments. The interpolation at lines 121–123 unconditionally substitutes `{workspaceRoot}`, `{projectRoot}`, `{projectName}`, and `{options.*}` inside any quoting.

Concrete source-proven trigger: `bun repo --tool=bazel exec --projects=app -- node -e 'console.log(process.argv[1])' '{projectName}'` materializes a last argument of `app` rather than the literal `{projectName}`. A literal `{options.missing}` throws `Build task requires option missing` before the intended child starts. Quoting cannot protect against this substitution because it happens before the shell parses quotes.

Nx contrast: installed `node_modules/nx/dist/src/command-line/exec/exec.js:61–78` assembles the received callback arguments and executes them without build-token interpolation. Its ordinary exec path sets project cwd and project environment but never calls an interpolator on the callback. This defect is separate from per-command forwardAllArgs/configuration/dependency handling.

Repair should represent exec executable and arguments as a literal vector through the execution boundary, or explicitly prevent authored-build interpolation for the synthetic callback. A regression must exercise final materialization, not only `parseArgsStringToArgv` of the intermediate command: current `🏗️build/🧭️selection/🧪️tests/🟦️.ts:49` checks only that intermediate roundtrip and cannot detect the subsequent transformation. Include literal build tokens and unknown option tokens.

## Additional Inspected Behavior

- Nx literal-argument bridge and Bazel command runner both quote ordinary forwarded arguments using the same single-quote escaping before Bun exec. No additional ordinary-argument mismatch established.
- Both adapters overlay declared task environment and then enforce selected tool and workspace/task identity. Ambient no-configuration inheritance exists in both implementations and is not a newly established adapter difference.
- Bazel parallel command failure aborts sibling controller and awaits allSettled; task signal combines issued phase and local cancellation. Sequential commands check their current issued phase before each launch.
- Watch uses an issued child phase, an explicit failure slot preserving arbitrary thrown values, and awaits the pending command before closing. Recognized neutral child callbacks enter the owned child invocation path. The watch/affected agent owns remaining event-selection and progress repairs.

## Boundaries

The finding is proven by source control flow, with a concrete argument transformation contrast. This auditor did not execute either backend. Windows shell quoting was inspected but no additional Windows runtime claim is made without executing the platform. Native private physical proof is documented separately in `2026-10-17-native-private-proof.md`.
