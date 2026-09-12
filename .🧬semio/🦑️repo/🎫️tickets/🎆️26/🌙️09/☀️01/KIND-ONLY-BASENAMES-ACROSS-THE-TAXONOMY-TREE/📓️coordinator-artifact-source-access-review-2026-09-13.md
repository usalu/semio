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

**Status:** pending acceptance correction. Root has not claimed these native controls pass after a repair.
