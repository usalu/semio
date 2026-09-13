# Artifact Source Access Acceptance Findings

Root read the first extracted shared source-access implementation and executed a private native filesystem control as uid501. Two acceptance gaps are confirmed in actual behavior, beyond source inspection. This is not an artifact-law conformance failure or a native application test.

| Current boundary | Actual native outcome | Required correction |
| --- | --- | --- |
| Source text below an ancestor symbolic link | `policySourceText` returned `state:file` and the private target marker. | Check the admitted ancestor chain or enforce an equivalent no-follow traversal boundary before reading. Checking only the final leaf with `lstat` follows linked ancestors. |
| Source directory below an ancestor symbolic link | `policySourceDirectory` returned `state:directory` and its target entry. | Apply the same boundary to directory access. |
| Unreadable file, native mode000 | Typed reader returned `state:unreadable`, but the actual shared `policyReadFileSafe` wrapper returned an empty string. | Keep deliberate absence semantics separate and surface non-missing failures to the real callers. |
| Unreadable directory, native mode000 | Typed reader returned `state:unreadable`, but shared `policyReaddirSafe` returned an empty array. | Do not report unreadable evidence as an absent clean owner through the wrapper used by remaining root laws. |

All inputs and targets were created beneath `🗑️generated/coordinator/source-access-hostile`. The marker contained only ticket-private test text. The control restored file/directory permissions in `finally`, never touched shared source permissions, and emitted `[DEBUG]` evidence. Its raw result is `source-access-hostile/result.json`. No Git operation, live cleanup, publication or fixture output outside this ticket was invoked.

Sol and Terra were notified before artifact extraction acceptance. The proposed11-owner/13-context split and direct9/2132 structural/law control are useful evidence, but they do not close these exposed shared-reader paths. Add portable and native controls for the real wrapper behavior and ancestor links; retain actual existing policy diagnostic semantics without source-body hashes or new runtime libraries. The fixed `Safe` API names must not become a compatibility excuse for silently preserving the old failure behavior.

## Current Native Repair Verification

Root reran the actual source-access owner against a new private filesystem tree as uid501 after Sol's repair. Both text and directory reads beneath a linked ancestor now return `symlink`. Native mode000 file/directory reads return `unreadable`; both shared Safe wrappers throw for unreadable and linked input. Intentional missing input still returns the documented empty string/list. The control exited0 with `[DEBUG]` evidence and restored permissions in `finally`. Raw current output is `🗑️generated/coordinator/source-access-hostile/native-after/result.json`. This closes the four exposed shared-reader defects above without claiming discovery or aggregate acceptance.

**Status:** artifact extraction acceptance remains open. Terra independently found that artifact owner discovery collapses an EACCES discovery root to no owners and no aggregate findings. Its current direct control also exposes two invalid operation stubs that fail ancestor admission before reaching their intended leaf states. See `📓️terra-root-artifact-schema-law-audit-2026-09-13.md`; Sol is repairing those separate boundaries. Earlier9/2132 was a pre-repair checkpoint, not the current green gate.

## Final Acceptance

The remaining discovery defect and operation stubs are repaired and independently accepted. Root fully read Sol's final artifact report and confirmed all23 explicit attributed paths exist. Final source control11/2147 passes directly and through the actual isolated Nx target; field-parity Nx, compiler6/86 and root import pass. Terra's independent2146 checkpoint, injected EACCES issue/aggregate breach and native linked-ancestor evidence are retained separately. The bounded artifact extraction is accepted; its live192owners/2,000diagnostics are still policy debt, not a conformance-green claim.
