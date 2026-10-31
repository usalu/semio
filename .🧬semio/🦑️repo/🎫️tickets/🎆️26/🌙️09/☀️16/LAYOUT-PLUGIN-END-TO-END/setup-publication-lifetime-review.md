# Setup Publication Lifetime Review

Read-only review of the setup agent's current TypeScript hunks. No source edits, tests, native jobs, setup, cache or publication operations were performed by this reviewer. Rust's EDIT retry remains under its own controller.

## Captured Runtime Source

The fixed publication adapter now receives the real CapturedRuntimeEntryCommand as its fourth argument and passes it to the defining fixedCommandSpec's required third argument. That defining function constructs command DATA from the captured program/arguments/input/environment and the exact fixed action; it does not re-admit an ordinary cancelled control. The publisher captures nativeDirectoryRuntimeEntry under its actual body phase before the first fixed child and retains the same immutable entry under the already accepted physical-cleanup enrollment.

Ordinary children obtain the command through capturedRuntimeEntryCommand with their actual requested descendant. Cleanup obtains it through capturedRuntimeEntryCleanupCommand with the exact retained enrollment. The latter checks its private token/enrollment identity and live accepted cleanup budget. No new ScriptInvocation, copied context or replacement deadline is required.

## Stdin And Terminal Closure

Captured runtime Source permits at most 4194304 bytes. The canonical owned-command binary-input interface permits the same 4194304-byte payload and 5592408 encoded characters; therefore the ordinary command's base64 input forwarding preserves the actual immutable Source while the smaller text-input limit would be inappropriate.

The raw accepted-cleanup child uses the existing publishOwnedAuxiliaryInput helper and retains its Promise until terminal handling. That helper records the first write/pipe failure independently from completion, including an undefined primary, and settles after write completion or a failed pipe close. The publisher awaits both the child close and input publication before checking failures and reporting settlement. Its existing output/deadline/cancellation checks, physical reattachment cleanup and unresolved-custody retention remain active; closeNativeImagePublication refuses a pending child.

The reviewed ordinary and retained publication paths use the same corrected body/cleanup functions. This source review found no concrete dropped Source, caller-phase or cleanup lifetime in those hunks. Actual platform execution and the setup agent's source tests/typecheck are separate evidence and are not claimed here.

## Type Ownership

The source-closure runtime module now imports ScriptInvocation from the canonical process routing export. The low-level process invocation module does not export that public type. The reviewed correction changes the import owner without fabricating or casting an invocation.

All TypeScript edits remain owned by setup_publication. The earlier publication-wiring-review and setup-publication-source-notes reports describe the dispatcher/observation gap; this report covers only the captured-entry/stdin/cleanup integration.
