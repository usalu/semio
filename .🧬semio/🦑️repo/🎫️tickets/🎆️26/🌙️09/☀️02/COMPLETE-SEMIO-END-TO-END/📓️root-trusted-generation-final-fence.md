# Trusted Generation Final File Fence

The fixed Stdio/GIS materializer now verifies the whole staged filesystem closure before generation rename. The same helper checks an already-existing generation and the rotation stage. It admits exactly the root/package/GIS/Stdio directories and the bundle plus four component/descriptor files, refuses symbolic paths and unexpected entries, reads through the shared bounded stable descriptor helper, compares independently supplied lengths/SHA256 values, wipes temporary buffers, then rechecks every directory/file identity without another callback or asynchronous yield. A final checkpoint precedes that small synchronous identity pass. This is race/change detection inside a private build directory, not a sandbox against a hostile process changing filesystem state after the final syscall.

The generation parent may already exist on a repeated build; it is now admitted as a regular non-symlink directory instead of unconditionally failing `mkdir`. Rotation shares the same owned interrupt/deadline/progress controller as initial materialization. Neither materializer publishes the current pointer: the existing candidate Hub readiness/admission flow still owns that step.

## Registered Evidence

-92975 is the expected missing-fence-helper RED.
-49776 is GREEN9 for valid bytes, same-length component/descriptor corruption, altered bundle, extra/missing file, symbolic file/directory paths and cancellation. AJV and independent WebCrypto SHA256 inputs exercise the real helper.
-27095 exposes the missing post-read identity checkpoint in the new mutation law; the mutation had not yet been injected, so this is an instrumentation RED, not proof the existing identity comparison accepted changed bytes.
-63730 is GREEN11 after a post-read component replacement is injected and rejected, plus an oversized sparse component is denied. The fence is integrated into initial/existing/rotation paths; their full real Cargo/Hub process journey is still unrun.
-10555 is a substantive RED: spreading a rotation receipt could overwrite the helper's fixed pathname with `record.component.path`. The helper now selects only the length and SHA fields explicitly.
-83439 is GREEN12 (`🗑️generated/generation-stage-COHdTZ`), including a receipt carrying an untrusted path which cannot redirect reads. Successful cases also require exactly one final identity checkpoint and no callbacks afterwards. Fresh handoff11 remains GREEN (`fresh-component-staging-0vNtlt`), as does the existing bootstrap neutral corpus.

Windows symbolic-leaf coverage uses a directory junction at the required file path, avoiding privileged file-symlink creation; non-Windows exercises a file symlink. No Windows execution is claimed from the macOS receipts.

## Remaining Work

The codec JSON inputs are still reopened after long component builds and need one bounded, strict, immutable capture before those builds. Rotation/current-pointer reads still have older read helpers and need the matching source fence, not merely output validation. The browser actor record, source/policy generation binding, immutable loader/lease/body and contained Worker remain separate unimplemented activation work. No current end-to-end frontend or collaboration completion claim follows.
