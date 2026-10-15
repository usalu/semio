# Native Storage Platform Registration

The permanent neutral `@semio-tech/framework-process:test-native-storage-platform` target calls the process `📜️script.ts test native-storage-platform` route. It combines fixed-storage format/path/platform cases and the closed platform-maintenance contract suite. The target is uncached, has no task dependencies, and is explicitly classified as Source execution. Both editor launch and seed contain English and German entries using the existing explicit artifact-directory input.

The fixed path test now requires `SEMIO_TEST_ARTIFACT_DIR` and creates its disposable filesystem only beneath that directory. It removes that owned temporary directory in its finalizer.

## Actual Verification

- TypeScript JSONC parser accepted both launch files; each has exactly two matching entries, one English and one German. Actual target command dispatch was checked.
- Low read-only review found no registration or artifact-boundary defect.
- The first direct Bun invocation omitted the `./` exact-file prefix and Bun crashed before tests (exit 133, 21.379 seconds, peak 1.95 GB). This is an invocation failure and not a semantic RED. Its original output was retained in `🗑️generated/native-storage-platform.log`.
- The corrected exact-file invocation reached the intended files. Fixed storage passed 3 tests / 36 assertions. The platform contract module failed import because `completeNativeImageMaintenanceV1` is absent from the current source. This is an actual unresolved contract mismatch; no complete-suite pass is claimed. Output: `🗑️generated/native-storage-platform-exact.log`.
- The storage agent was assigned the current Source/test alignment before another exact run. The permanent runner already supplies absolute file paths and therefore does not use the ambiguous direct-invocation filter form.

## Resolved Maintenance Contract

The actual API uses `{acquired:false}` or `{acquired:true,value}`. The previously authored schema, fixture and tests incorrectly described a nonexistent receiver field/helper. They now describe that actual API; no new helper or receiver field was introduced.

The aligned regression exposed a separate real lifetime gap: the three actual platform maintenance methods returned after the callback without a final backing recheck. Before production edits, actual TypeScript-transpiled method execution observed two checks where three were required; the AST law also observed no post-callback recheck. This semantic RED is retained in `native-maintenance-red3.log`. Two preceding attempts stopped on an authored test syntax error and incorrect relative import, and are setup failures rather than semantic RED.

The real opened Mac/fixed methods now check original control after the initial awaited recheck before releasing shared custody, and keep their exclusive lease through callback completion, another control check, the final backing recheck and final control check. The provisioned Mac owner applies the same completion checks while retaining its original owner. Original callback values and failures, including a thrown undefined, pass through; the opened leases still release in their finalizers.

The final exact-file suite passed **6 tests / 205 assertions / 1.255 seconds**, exit zero, in `native-storage-platform-final.log`. It executes the actual three production method AST bodies through the existing TypeScript transpiler with external filesystem/lease seams, real control admission and independent SQLite outcome oracle. Ajv and the repo schema validator agree on the language-neutral result corpus. The cases include initial-await cancellation before shared release, body cancellation, cancellation during final backing recheck, exact success/error/undefined identity and exclusive release. This method-level test does not claim native Linux or Windows execution, nor independently prove physical mount behavior. Physical integration remains covered by the separately recorded macOS owner gates.

Low reviewed the actual production correction clean. Root handed the Source lane back to Core after the final run; no unresolved import mismatch remains.
