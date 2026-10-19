# Command Startup and Developer Launches

## Changes

The root `📜️script.ts` now registers command owners lazily. Verification imports its test lanes only when selected, and policy imports its audit owner only when run. Help loads only platform and process routing code. The root Repo route calls the existing build CLI birth function, preserving its authored caller resources instead of creating an empty workspace caller.

The `test entrypoint` route runs its defining suite through the existing owned process interface. It is registered as `workspace:test-command-entrypoint`. The developer command owner loads the OS MCP gateway only on its selected OS route, so repository developer commands do not require the unrelated application gateway to be installed first. `.vscode/launch.json` provides bilingual launch entries for help, tool setup, workspace setup, build, test, run, dev, a selected task, affected tests, project listing, startup contracts and Bazel contracts. Backend and mode choices flow through the same neutral Repo interface.

## Test-Driven Evidence

1. Before the change, the language-neutral startup corpus failed because root help eagerly imported an unrelated application test whose workspace package was not installed. The independent esbuild boundary rejected 55 eager command dependencies.
2. After lazy registration, root help passed. The dedicated regression then reproduced `Original Repo build caller required` for both backend help routes through the generic workspace caller.
3. Routing Repo through `runBuildCliV1` resolved that second failure.
4. An added developer-command import regression reproduced a second unrelated application dependency: the missing `@semio-tech/framework-os-mcp` package. Moving that import into the selected OS MCP route resolved it.
5. Latest `bun ./📜️script.ts test entrypoint`: **4 passed, 0 failed, 41 assertions**. The earlier three-case suite also passed directly and was independently rerun by the read-only integration auditor. A final integration rerun detected an unmatched trailing root brace during concurrent work; only the unmatched brace was removed and the complete suite rerun successfully.

The third-party esbuild oracle bundles startup while rejecting eager application owners and runs the resulting program against the same expected help text. The fixture also verifies both Nx and Bazel help through the public root route.

## Limits

Full repository build/test/setup execution requires the native cache backend. The host privilege limitation and its actual runtime evidence are recorded in `storage-2026-10-19.md`; startup success does not qualify those operations.
