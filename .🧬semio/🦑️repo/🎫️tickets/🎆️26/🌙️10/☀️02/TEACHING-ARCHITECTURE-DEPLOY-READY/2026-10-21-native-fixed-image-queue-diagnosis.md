# Native Fixed Image Queue Diagnosis

Read-only Source and ordinary metadata inspection; no process probe/signal, SQLite connection/claim probe, native command, private mutation, unmount or cleanup. Applied build SKILL.md recovery rules.

## Exact Meaning of Current Progress

Process leases/🟦️.ts queueWaitInfo389-400 reads live[0] ticket JSON and publishes its pid/mode under holderPid/holderMode. holderSeconds is floor(now minus arrival encoded in that ticket)/1000. Thus37466(shared),~2443s identifies the earliest retained queue requester and ticket age, not an independently observed actual kernel image-lock holder or current execution activity. Position7 means six retained predecessor tickets; queued8 is the queue directory names count (bounded below by position), not eight proven active image readers. PID metadata alone gives no authority to signal or classify a peer stale.

awaitQueuedTurn475-486 admits a shared cohort only if all preceding ticket rows are canonical shared requests with sharedCohort:true. A later exclusive or noncohort predecessor can block our seventh shared request even while the first displayed row is shared. Queue head alone cannot identify that blocker. A ticket is reaped only after an exclusive independent kernel claim proves its requester gone; age is not proof. The platform persistentOptionsGuardV1 explicitly uses sharedCohort for shared readers.

## Physical Metadata Boundary

Observed .cache/b/g contains fixed.dmg,image.json,m. image.json is version1 with bootSession1E4A0CBE-3710-481F-9614-F8F1FCBD61D0, image dev16777229/ino423682876,size4293918720 and maximumPhysicalBytes4294967296; these are persisted metadata, not current mount or ownership proof. .cache/b/c immediate inventory and bounded .cache/b descendant directory scan did not locate the expected SHA256 resource queue83ad7e5b6b7bbfa81c6c53bbc802d34875f111cc03586189a4dd5f27744d19fc.queue. Its other immediate queue was empty when read. No inference of missing logical queue or stale log follows: actual leaseDirectory/control namespace was not independently resolved. No lease database was opened or lock availability probed.

## Capacity and Recovery

Root reports create27319 capacity waiting8running/1ahead/need3584MiB while unit startup reports image queue. These observations establish concurrent host resource contention. They do not establish a dependency cycle: there is no observed edge from earliest image ticket37466 to Root create27319, nor proof that any image predecessor waits on a capacity reservation held by a successor. Ordinary graph/cache readers may retain their native admission capacity while waiting, so resource pressure can coexist with serialization without deadlock. Actual dependency proof would require authenticated owner/control wait relationships, not PID/age guesses.

The declared build recovery instruction applies when repo refuses at startup: retain refusal/receipts and retry the declared operation through repo so owned publication/directory-swap recovery settles. This current queue is an active wait, not a refusal or evidence of inactive partition corruption. The fixed cache-reset-nx route concerns an inactive Nx partition, not this active graph image. No supported recovery entitlement to detach, delete claims/intents, force unmount or stop another task is established here. Preserve actual owner cancellation/deadline custody and normal declared route.
