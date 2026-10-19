# Runtime Environment and Historical IDE Retirement

The process runtime environment schema still contained an IDE launch-catalog protocol. Its test read two deleted editor files, so it could not verify current developer workflows. The obsolete protocol and reader were removed. Shared environment vectors now exercise the actual `typeScriptCheckEnvironmentV1` function in prototype and release modes. The authored tests compare retained environment values, cache overrides, removed Node cache paths, input immutability and frozen results with SQLite, Ajv and the repository schema validator. The feature is exposed by the localized `@semio-tech/framework-process:test-runtime-environment` owner target and its existing `📜️script.ts` dispatcher.

The root developer guide now names the dedicated Debugger Availability and Debug a Program dashboard forms, with task input and lifecycle controls.

Independent tracked-file inventory found ten remaining historical ticket launch files. The current explicit request to remove all IDE configuration applies to these standalone files. They were deleted after adding their exact paths to the shared dashboard-control absence fixture. The one direct fixture consumer, the build-tool ticket tokenizer helper's obsolete `--launch` branch, was retired. Its neutral tokenizer and embedded-quote checks remain, with a closed argument selection. Historical Markdown reports, captured inventory data and unrelated input scripts remain intact. No AGENTS file or Git state was modified.

## Verification Record

- `bun repo --help` exited 0 on macOS after the source changes.
- Scoped `git diff --check` completed successfully. These are structural checks, not test-suite passes.
- The existing environment suite and the newly exposed target were invoked through `bun repo`. They did not reach test execution while queued behind other tasks' exclusive dependency-image producers. Redundant owned attempts were cancelled with SIGINT; their Original interruption was reported. No test count or compiler execution success is claimed.
- Independent source review found the relative imports, validators, SQLite oracle, target metadata and script route coherent. Source review does not establish live compiler or dashboard behavior.
- A filesystem inventory after deletion found no conventional launch/IDE configuration filenames below the ticket root. The broader tracked-file review is recorded separately in the independent audit.

The macOS canonical dashboard command initially reproduced `Unexpected build argument before --: commands` on a missing installation. After the build-entry routing repair, the same command logged `[dashboard] Rebuilding prototype because it is not installed`, then failed `Undeclared original Repo build selection`. The execution owner is repairing the precise declared rebuild dependency. This proves the routing transition only; it does not prove compilation, installation or a running dashboard.
