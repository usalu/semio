# Independent Invocation Extraction Audit — 2026-11-04

Read-only audit of current extraction, callers and sealing boundary. No builds/tests/source edits.

No actionable correctness regression established in current extraction. The old function body and new invocation-owner body are identical apart from the relative module paths and imported type path required by its new physical owner. Every literal relative dependency observed in the new owner resolves to an existing file.

The extracted function has no import.meta identity, import.meta.dirname, new URL, caller-relative file read, or ambient module initialization dependent on its old script location. Its node:path relative calculation is still relative(root,kernel.entry), preserving workspace-relative selected kernel identity. repositoryCommandKernelV1 remains the same existing selection owner; its own import.meta URL remains physically unchanged.

The same normalizeBuildOptionsV1, withBuildCapacityV1, withImplementationPinV1 and executeImplementationV1 functions are invoked in the same nesting and argument order. The same process.env is read, the same parent Original and phaseControl are passed unchanged, and phaseControl remains the withBuildCapacityV1 control option. No replacement envelope, deadline, capacity policy or cancellation token was introduced. Existing sealed execution continues to capture/launch the selected entry with the Original and issued phase and retains its original live/sealed, fallback, dependency readmission and signal behavior.

Current root script imports owner directly for BuildScript and RepoScript; runtime imports owner directly for resident executor/foreground recursion. The public build script package entry explicitly exports the function from the new owner. Existing source-inspection phase, portable CLI and dashboard control laws now read the new physical owner rather than relying on a compatibility reexport from the root script. A bounded repository-library occurrence scan found these updated call/inspection sites; no stale production import of the old root export was established.

This extraction removes the runtime-to-root-dispatch edge without omitting any dependency of the extracted function. Actual canonical CLI and runtime Source captures plus normal execution remain necessary evidence owned by the host lane. The transient partial-write syntax failure is not a current source finding and no rollback is recommended.
