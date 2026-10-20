# Scoped Compiler Descriptor Observation

## Observed Failures

Retirement 3079 exited 1 before deletion because its global descriptor stdout reached the unchanged 16,777,216-byte cap; stderr was empty. Scoped retirement 65984 also exited 1 before deletion with 230 stdout bytes and empty stderr; its exact capture status was not retained. The next guarded attempt 35049 persisted descriptors.result.json: status 1, signal null, reason exit, stdout 400 bytes, stderr 0. Its actual output contained both physical build locks and both readonly root-directory descriptors. Thus root-directory visibility is proven on this macOS host, while recursive +D search status remained unsuitable. No unit was deleted by those attempts.

## Primary-Source Cause

[Apple lsof main.c](https://github.com/apple-oss-distributions/lsof/blob/main/lsof/main.c) checks every expanded Sfile and returns 1 when any requested file was not displayed. A nontrivial +D tree includes ordinary unopened files, so additional positive witnesses cannot establish status 0. Apple's published parser lacks -Q. The [upstream 4.95.0 parser](https://github.com/lsof-org/lsof/blob/4.95.0/main.c) supports -Q by disabling failed-search reporting. The [official manual](https://lsof.readthedocs.io/en/stable/manpage/) separately documents filesystem selection, recursive selection, warnings, and filesystem crossing. No general status-1 acceptance is introduced.

## Current Platform Plan

On macOS, retirement selects the dedicated authenticated compiler backing filesystem exactly once with +f and --. The backing root comes only from the required live NativeStorageReservation or authenticated historical image; a path-only DTO does not issue storage authority. The actual mount table must identify that precise mounted root. Any nested filesystem overlapping an ancestor or descendant of either guarded build/target scope refuses the observation. A mounted-filesystem query avoids expanding unopened files into individual failed searches. Exact descriptor classification still applies only to the guarded roots and refuses every unknown descriptor there; unrelated paths outside those roots retain the prior behavior.

Linux retains the recursive union with documented -Q plus warning-enabled +w and filesystem-only crossing. Strict status 0, null signal, exit reason, empty stderr, unchanged output bounds, and every real root/physical-lock witness remain mandatory. Unsupported tool options or operational warnings still refuse. Native Linux bootstrap now provisions the existing lsof prerequisite through its four established manager package arrays; the devcontainer already includes it. Linux runtime verification remains pending.

Readonly root witnesses use O_NOFOLLOW and O_DIRECTORY, bigint descriptor/named identity checks, immediate finally ownership, and complete close drainage. Primary and all close errors remain observable, including undefined primary errors. Existing physical locks are in-process Bun FFI locks and share the actual process PID; Python is only an independent lock test oracle.

Deletion now compares device identities at its existing root, ancestor and per-entry lstat frontiers. It refuses a late nested filesystem before unlinking or descending into it, without another source traversal or relaxed currency limits. Mount observations are bounded and captured before/after the descriptor census and after recensus.

## Tests and Scheduling

Schema-first neutral coverage currently includes fourteen selector cases, eleven witness-lifetime cases, seven mount-table cases, two device cases and six selected-name cases. Ajv validates the closed documents. Independent SQLite relations verify selection, mount overlap, device equality and lifetime equations. TypeScript extraction runs the actual private witness helper and actual deletion function against controlled failure frontiers. High's actual owner extraction covers real orchestration wiring. No Source body pass is claimed yet.

An unrun Source kernel fixture that reacquired diagnostics was removed after the parent identified a fair-queue parent-SH/child-SH cycle behind a queued writer. No new Native authority or private cache was fabricated. The genuine parent-owned maintenance attempts are the actual kernel oracle: 35049 proves root/lock visibility; the next guarded mounted-filesystem attempt must establish clean status and successful custody before deletion. No additional compiler or cleanup job was launched by this agent.

Windows preview is separately implemented through its existing portable observation port. Windows inactive compiler deletion still lacks qualified physical file-lock and complete handle backends and explicitly refuses; this is a platform gap, not a zero-touch completion claim.

Independent final source review accepted the selected filesystem scope, shadowed-parent mount handling, intervening/descendant mount refusal and strict selected-name admission. Parent-owned filesystem retirement 43261 is the next genuine runtime attempt; no outcome is inferred while it runs. Imported source remains frozen during that attempt.

## Actual Mounted-Filesystem Kernel Gate

Parent attempt 43261 produced retirement-filesystem/turn-yS8Tjs/descriptors.result.json with status 0, signal null, reason exit, stdoutBytes 400 and stderrBytes 0. Direct readback of descriptors.stdout identifies actual PID 83820, two exact profile lock files, and both exact build/target root-directory descriptors. The descriptor and mount-scope gates therefore reached clean runtime admission on this macOS host. Final recensus and whole retirement completion remain pending; no deletion count or Source suite pass is inferred from this gate.

## Whole Guarded Retirement Result

Parent-owned attempt 43261 settled whole-command exit 0. Its actual result reports 6,696 retired units, 4,294,966,853 logical bytes (below the 4 GiB deletion ceiling), and 7,841,587,200 available bytes. The descriptor inventory returned clean status 0 with the exact physical-lock/root witnesses, and all three mount observations passed. This proves the macOS mounted-filesystem retirement path on this host. It does not establish Windows/Linux support or passing Source regression bodies. The parent then released the single Native slot for focused persistent-law session 20690.
