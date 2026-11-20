# Darwin constrained lab admission

The ticket launcher runs one cold, frozen, four-core machine through all readiness stages with an 8,192 MiB lab budget. It wraps the complete create, run and cleanup lifetime in a fresh 8,192 MiB and four-core request on the account's canonical host-capacity ledger. The launcher clears inherited, handed-over and simulated capacity settings. The lab still uses its own independent per-machine capacity model; the outer grant is an actual scheduling reservation.

`preflight` checks the exact frozen input roster and available backing space without taking a host grant or creating an image. `run` repeats these checks under the grant before any image operation. The image size is selectable from 41 to 48 GiB and defaults to 48 GiB. The admitted physical capacity covers the full configured image, 2 GiB of backing overhead, at least the 12 GiB host reserve, and 1 GiB of free-space margin. Once the image exists, every check charges the exact sparse backing allocation against that cap and requires sufficient current free blocks for remaining growth. This budgets a full copy across filesystems and all subsequent physical growth of the ticket backing image. The host free-space guard observes the shared partition every two seconds while the child runs.

The only temporary endpoint outside the ticket is a private, short directory under `/private/tmp`. The only backing store is a ticket-generated case-sensitive APFS sparseimage, and its formatted volume must still report at least 40 GiB available before the child starts. Admission uses the complete prospective clone path and requires at most 256 UTF-16 units for each tracked and nonignored untracked input. Preflight reports both relative maxima and the full clone maximum. The 210-unit tracked maximum is the observed checkout measurement.

The launcher defaults to 10,800 seconds for the lab's full machine period and accepts an explicit `--timeout=30..14400` override. R197 run 4 reached readiness in 2,658.9 seconds at 8 GiB and four cores, including 967 seconds for dependencies and 1,351 seconds for workspace. That run used a one-second Native stage. The cold all-stage plan uses the current Native setup boundary of 7,200 seconds, so the default budgets that boundary, the historical downstream duration, and roughly 942 seconds of margin. The full lab uses the chosen duration for the overall observation period and its workspace stage, while the Native, repo-mcp, dependencies and tools stages enforce their own boundaries. [R197 run 4](result-R197.md)

After child settlement the launcher copies the exact bounded machine report to `🗑️generated/l-*/m1.json` and the aggregate verdict to `verdict.json` when available. Receipt export verifies the source device, inode, pathname, byte length and stable timestamps, refuses symbolic links and writes a new file without mutating existing evidence. Per-run admission, original error and cleanup outcomes are recorded beside the receipts. Cleanup verifies the exact backing inode and private mount relationship using the Native image inventory. It leaves unresolved or busy images in place, uses a bounded cleanup signal and never force-detaches, recursively deletes an external directory, or operates on another mount. A nested image still active at cleanup preserves the outer image and the recorded error. A receipt export fault preserves the detached backing for examination. A process kill that prevents cleanup can also leave the recorded owned backing and mount.

## Verification

The six focused tests passed with 61 assertions. Language-neutral fixtures cover closed arguments, physical accounting and UTF-16 boundaries. Ajv independently validates allowed option data. The tests also verify the actual lab command shape, refuse ambiguous image associations and exercise byte-identical machine receipt export on a private filesystem fixture, including symlink, wrong-device, oversized and preexisting-destination failures.

| Read-only probe | Tracked paths | Maximum tracked / full clone path | Available physical bytes | Required physical bytes | Result |
| --- | ---: | ---: | ---: | ---: | --- |
| 41 GiB configured | 168,059 | 210 / 249 | 46,122,876,928 | 60,129,542,144 | Refused |
| 44 GiB configured | 168,059 | 210 / 249 | 44,951,494,656 | 63,350,767,616 | Refused |
| 41 GiB after path and timeout review | 168,059 | 210 / 249 | 47,573,798,912 | 60,129,542,144 | Refused |

The last probe reported a 183-unit maximum relative untracked path and the 10,800-second default.

The installed Darwin tools confirmed case-sensitive APFS formatting support, mountpoint detach support, and the expected inventory fields. CLI help and rejection of a caller-supplied mount path were exercised. The coordinator ran no image creation, attachment, detach, or full lab. Actual formatted free bytes at the smallest size and a live machine verdict are pending a physically admitted run.

```sh
repo tools exec --light bun -- '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES/constrained.ts' preflight --image-gib=41
repo tools exec --light bun -- test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES/constrained.test.ts'
```

For a live attempt after physical admission, launch from the workspace root with the same image size used for preflight.

```sh
repo tools exec --light bun -- '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES/constrained.ts' run --image-gib=41 --timeout=10800
```

After the coordinator released its full 14,336 MiB graph grant, the normal 41 GiB preflight still refused disk admission. It measured 46,014,742,528 bytes free against 60,129,542,144 required, with all 168,059 tracked source paths accepted and no image attached.

On November 20 the physical preflight passed with 77,019,414,528 free bytes. While waiting behind two whole-project grants on the 32 GiB host, inspection showed that the launcher started its machine timeout before requesting capacity. That could spend a cold machine's execution time in the scheduler queue. The launcher now uses the canonical cancellable wait control and starts its bounded execution timer when the grant is delivered. The neutral timing-scope fixture failed on the prior arrangement and passed with the corrected launcher. Seven tests passed with 63 assertions. The original queue request received a normal interrupt while its exact 8,192 MiB/four-core request was still waiting, before any image was created. Its replacement was submitted after the interrupted process exited. The requested machine period remains 10,800 seconds.

On November 20 a real retry lost disk admission between its preliminary check and its memory grant. It observed 59,861,999,616 free bytes against 60,129,542,144 required. The launcher now waits for physical headroom outside the scheduler and relinquishes its grant when the subsequent check finds that space has fallen. Its path and backing checks still run under every successful grant, and a post-launch disk or mount failure still terminates and cleans up the private volume. The new neutral admission fixture exercises two attempts, one interrupted by a disk shortage after admission, and cancellation while waiting. All eight wrapper cases passed with 70 assertions. A fresh cold invocation with this behavior started at 01:52 UTC. The normal cache GC dry-run also exited zero, reporting 2.33 MiB in its machine footprint-log category. It performed no reclamation.

## Disk admission progress

The ticket-owned launcher now prints a bounded `[lab] disk-wait` row once per existing 30-second wait. Each row gives measured `freeBytes` and `requiredFreeBytes` from that pass. `before-grant` identifies a shortage before requesting memory. `after-grant` uses the measured shortage that caused the memory grant to be released, and is printed after that release. The same cancellation signal controls waits and prevents further reports once cancelled. This adds no probes and leaves both admission floors and the live invocation in place.

The neutral fixture exercises cold shortage, space lost while waiting for memory, and cancellation. An isolated execution of the actual function matched its expected progress and grant-release sequence, and SQLite independently confirmed that every reported sample measured free bytes below the requirement. The complete short wrapper suite passed eight tests with 74 assertions through the repository tool entry. It did not create or attach a disk image.

## Disk eligibility while queued for memory

Session 9961 reached the ordinary 8,192 MiB queue during the 06:33 disk recovery. By 06:37 the measured free space was again about 57.26 GB, below the 60,129,542,144-byte requirement, and its memory request remained queued. Before the change, the next physical check after entering the queue came under the eventual grant.

The ticket launcher now resamples the exact generated parent on the capacity gate's existing wait notice, whose normal cadence is five seconds. These notices follow an actual ungranted decision. A measured disk refusal aborts only that request through its normal queue cleanup. The launcher reports `while-queued` after withdrawal settles and returns to its existing 30-second disk admission loop. Measurement errors and parent cancellation retain their original reason. A grant starts the execution budget inside the same callback as before. Its strict path, disk and grant checks run before image work, followed by the existing live backing checks.

In the private file-backed gate, an 8,192 MiB competitor left room for just one grant. The pre-fix launcher kept its deteriorating request in the queue until the test deadline. The fixed launcher settled that row while the competitor stayed admitted, returned to disk wait, and admitted again through the ordinary gate after disk recovered. A second test lost disk eligibility at grant issuance and verified that the held preflight returned an `after-grant` refusal and released the exact grant before any launch. Cancellation at issuance likewise left neither a running body nor a grant. A neutral schema and SQLite oracle cover these cases, an unchanged eligible sample, and exact cancellation and disk source errors in the queue. The final full wrapper suite passed 15 tests and 160 assertions with `repo tools exec --light bun -- test`. Session 9961 continued using its already-loaded launcher. The verification used a private ledger and created no images.

The coordinator initially preserved the existing request when an immediate check found that disk had recovered to 60,213,514,240 free bytes. At about 06:49 UTC, a later physical check measured 59,368,828,928 bytes. The exact old process and its 8,192 MiB four-core waiter were current, the process had no grant, and no new image existed. The coordinator sent a normal SIGINT to that owned process, verified its typed cancellation and that its ledger row cleared, and launched the updated wrapper once using the same profile.

At 06:50 UTC the replacement had passed disk preflight and was in the ordinary memory queue. A host sample measured 64,544,231,424 free bytes and the memory gate still declined an execution grant with nine other requests running. There was no new private backing or stage receipt. The updated request retains its own current queue position and automatically returns to disk admission if physical headroom drops again.

At 07:08 UTC, the actual replacement had emitted a `while-queued` disk refusal with 59,977,273,344 free bytes against 60,129,542,144 required. The live capacity ledger showed no 8,192 MiB waiter. The same session then emitted ordinary `before-grant` disk checks. It had published no new backing or stage result. The separate measured host-capacity sample showed 4,484 MiB of admission headroom, and all eight declared backing images failed their normal reclaim predicate at approximately 59.63 GB of free host space.
