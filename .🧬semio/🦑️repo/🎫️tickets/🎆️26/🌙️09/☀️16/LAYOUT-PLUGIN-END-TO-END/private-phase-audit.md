# Private Native Phase Audit

Read-only source audit; no gates or child processes launched. Reviewed the genuine-birth private-workspace issuer, receiver, funding schema/helper, witness owner/child, nativeCallerPhaseV1, process routing projection, runtime Source admission, and extracted private-repository retirement.

## Phase and Authority

The final phase change resolves the identified finite-phase pre-spawn guard structurally: nativeCallerPhaseV1 creates a declared finite child phase beneath the exact issued caller control, and withRequestedIssuedResourceControlV1 narrows it again. scriptResourceProcessEnvironmentV1 intersects original/phase deadlines and preserves signal ancestry. The original invocation is never reconstructed locally; the actual OS child receives the projected wire through receiveScriptProcessInvocation.

Fixed purposes and parent selection are schema checked before allocation. Checkout Source identity remains separate from private workspace identity; entry/argv/cwd, actual parent PID, owner metadata, and root device/inode are checked before Source admission or store work. Native transport is narrowed to inherited limits and the existing 16 MiB / 65,536-line capture ceilings. Child completion is matched to actual spawned PID, limits, deadline, retirement, and disappeared workspace. No raw shadow Cargo root or checkout Source copy appears in the reviewed helper. Source admission remains scoped through child store retirement.

## Actionable Limits

1. Cancellation/deadline cleanup is not guaranteed by this helper. Receiver finally calls retirePrivateRepositoryRootV1 with the body control; that immediately requires a live issued control. If body cancellation or deadline exhaustion occurs after output/store effects, retirement is refused. disposeFreshPrivateBirthV1 deliberately leaves any root containing more than owner/metadata, and the parent likewise only disposes a fresh root. A bounded accepted physical cleanup scope reserved before effects is needed if retirement on these failure paths is a requirement. This is independent of the successful-path finite-phase fix.
2. Receiver metadata reads are not bounded in acquisition: readFileSync loads the whole metadata file before checking the 4096-byte limit, and owner-file read has no size/type check. For a strict hostile replacement boundary, open and inspect bounded regular non-symlink metadata before reading, retaining the directory identity check. The normal issuer writes bounded metadata, so this does not invalidate the normal successful path.

## Validation Status

No execution verdict inferred. Existing strict 26171, pure 28685, and actual rerun remain owned by the parent. Successful source inspection does not replace those results.
