# Nx Four-Field Selection Read-Only Audit

2026-10-10. Source and authored launch-row read only; no execution/tests/edits outside this report.

Current `📚️library/🔌️nx-plugin/📤️arguments/🧬️schema/📥️native-caller.json` requires workspaceRoot, arguments, native and selection. `⚡️caching/📦️artifacts/📋️native-orchestration/📥️caller/🟦️.ts::nativeNxChildEnvironmentV1` preserves requested.arguments unchanged and sets selection to `["nx", ...selected]`, then validates complete caller schema. These fields serve distinct purposes: arguments preserves original request; selection describes the actual child after public selection resolution.

Bootstrap `⚡️caching/🚀️bootstrap/📜️script.ts::resolveNxInvocation` resolves public Nx commands before graph construction. Direct `run` starts with supplied segments and normally returns those segments; transformations apply to explicitly recognized Print/build/watch/runtime target patterns. Bootstrap:265 issues child environment using actual child args through nativeNxChildEnvironmentV1. Entry:421 preserves supplied/inherited capabilities unchanged across original envelope handoff. Thus a four-field contract is required before initial dispatch, even though selection may later be updated for a transformed child.

Fresh parsed `.vscode/launch.json` confirms the ten root-owned current rows (CSV Source/Native, TSV Source/Native, Actor Source/Native, Value Literal Refusal Source/Native, Core Source/Native) have selection equal exact authored initial command.slice(4).split(" ") and equal their arguments. Their commands are simple direct Nx run with --skip-nx-cache/--excludeTaskDependencies, no shell quoting or public alias transformations. Core Native correctly selects `@semio-tech/framework-os-kernel:test-snapshot-receiving-native`; Core Source selects `@semio-tech/framework-rs:test-snapshot-receiving-source`.

Do not generalize split(" ") to other launch rows: an older factory-metadata row has quoted -E expression with spaces, and its actual argv intentionally differs from naive split. Another unrelated retained-value row still lacks selection. Neither is one of the root ten selected rows. No claim that every workspace row was migrated follows.

Seed counterparts were not located by the exact Unicode launch name in this audit; root owns their migration/check. Validate their resolved explicit arrays against their respective simple command, not GUI names or old inherited arguments. No generated runtime or child receipt was inspected here, so no replacement Native qualification is implied by authored-array equality.

Core mounted law source still contains exactly six `original_receiving_` functions: three in `🚪️io/⏱️control/🧪️tests/nested-io.rs` and three in `🚪️io/⏱️control/🛫️snapshot/🧪️tests/🦀️.rs`. Control root explicitly mounts snapshot:3 and nested_io_tests:129; snapshot root explicitly mounts tests at its tail. This validates those six source registrations remain present, not their Native runtime execution.
