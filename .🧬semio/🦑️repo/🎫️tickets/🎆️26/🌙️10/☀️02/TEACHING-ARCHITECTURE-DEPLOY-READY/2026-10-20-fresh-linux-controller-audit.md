# Fresh Linux Controller Draft Audit

Read-only draft review of 🌱️linux schema/plan/script/tests plus 📦️fresh contract; no command,Source mutation,capture,guest action or frozen input change. Draft hashes can still change.

## Concrete Issues Sent To Release

script.ts:39 guest append uses one fs.writeSync(output,b) without checking returned length. Partial writes can truncate retained log while counters claim full bytes/lines. Use bounded complete-write loop and meaningful partialwrite case. script.ts:47 transport reads whole container inspect,including Config.Env,although ownership needs only identity/user/workdir/labels/mount/state. Use exact projected inspection to avoid unnecessary environment exposure and65KiB refusal.

Plan receiptSHA fields are closed-format values but controller does not read/authenticate actual capture/builder/provision receipts. nativeSHA is any64hex,not statically n3. Root-sealed plan SHA is trusted input,and guest verifies actual native bytes,so this can be valid external seal workflow; document actual crossreceipt validation and n3a71cd…/11767992 binding before seal. Do not claim readPlan proves receipt content/freshness.

## Closed Workload And Ownership

Schema21steps match current acceptance9suites/3typechecks/explicitabsolute material/browser/deploy/same-tagbundle,plushelp/preflight/tools/one lock/one sync. All ordinary product argv use native absolute guest executable; original release mode/quick/CI retained. Companion suite inferredtarget still needs actualgraph proof. No registry push/production deployment/export masquerades as gate. Guest deletes parentScriptEnvelope/policy rather than replaying host Original. New plan denies old container+manifest and checks unique cohortname/workloadtoken,exact dedicatedlabels,one writable Sourcevolume,/repo workdir and actual native hash before each child/after all. Draft template remains inadmissible.

Controller starts detached dedicated launcher,then polls exact namespace-bound log/terminal; Docker start client0 is not accepted as workload terminal. It validates ordered native step argv/status/signal and completeallzero/native-recheck/reasonnull before acceptance. Failure cleanup authenticates exact builder+namespace workload custody,requests guest cancellation,then may stop exactly owned dedicated builder. Such fallback is legitimate scope,not categorically forbidden. It never deletes Sourcevolume or builder. Stop success remains separate from guestTerminal,and cleanupfailure stays explicit.

Cleanup uses existing accepted artifact-scratch token with resource linux-workload; resource is actually guest scratch+child process lifetime,not a new generic repo feature. The type permits enrollment,but semantic scope should remain documented narrowly; no Native artifact store/deletion authority may be inferred.45second cleanupbudget may exhaust across15secondguestgrace+25secondstop+boundedinspections; refusal remains truthful,not guaranteed settlement.

## Receiver Coordination And Limits

📦️fresh contract correctly separates inventory/readStable/absence control,sealedmanifest body+digest,exact buildercustody,receiver admission/receive/cancelAndJoin with actual terminal+partialcustody. It is interface decomposition,not an executable receiver or proof of actual fresh snapshot. Controller expects actual source manifest at /repo/.semio-private/source.json,its bytes/hash and fresh identity/provision receipts; these facts must be sealed by concrete capture/provision owner before launch. No arbitrarydigest fallback,no oldmetadata overwrite and no ticketSourcecopy is allowed by this design.

Prepared controller tests execute actual control through semantic fakeports and exact generated guest via fakefilesystem/process child; Ajv+SQLite check schema/identityrelationships/one lock-sync. They do not yet establish actual Docker projection,partialwrites or capture/provision receipt authentication. No actual RED/GREEN,guesthelp,terminal or product pass claimed. Remaining fixes and reviewed final tests required before Root execution.
