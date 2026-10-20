# Source Target Native Routing Audit

Read-only exact current build selector/script and graph assembly review; no command or Source change.

repo-mcp:test-install metadata.semio.execution.kind=source does not authorize an early graph-free invocation. Current early recovery predicate only admits run with exactly2colon components: repo-mcp:build or repo-mcp:install,or fixed repo maintenance preview/retire names. It validates exact owner manifest and option allowlist. test-install does not match regardless of source metadata,environment mode or omission of configuration flag.

For a nonrecovery run,build script opens authored stores/prepares environment,then calls loadBuildInvocationGraphV1 and withSelectedRustBuildGraphV1 before adapter task execution. There is no generic metadata=source fast path in current script. Source execution metadata is interpreted during graph project assembly: it prevents accidental native-policy/native-target/Cargo/Vitest wrapper classification for that target. It does not suppress Source input/command closure admission or move execution ahead of graph. Thus existing native repo run repo-mcp:test-install remains valid but graph-admitted; it can run32publisher boundary variants once graph permits.

Birth SEMIO_BUILD_MODE=release with no explicit flag can admit repo-mcp:build/install fast recovery because fixed option predicate sees only allowed argv. Explicit --configuration=release remains in resolved args due current resolveBuildModeV1 returning selection.args; early allowlist excludes it,so those build/install invocations fall ordinary path. This flag discrepancy does not create a test-install fast path in either form. Source metadata alone cannot make it eligible.

workspace:test-workspace-build currently lacks source metadata,declares5selfhosting test Source files through the existing build test router,and is a normal graph-admitted owner. Running that declared target is legitimate,not misuse,but absence/presence of source metadata cannot select an early native bypass. Adding a target or changing routing is outside this audit.

Exact current interpretation source: build/🚀️bootstrap/⌨️cli/🟦️.ts plus schema Protocol; build/📜️script.ts runOwnedBuildInvocation/runOwnedBuildInLeases; graph inference/🧱️project-assembly/🟨️.mjs execution!==source guards; native CLI source-kind validation in 🎯️target-policy. No faster existing early source-target port found.
