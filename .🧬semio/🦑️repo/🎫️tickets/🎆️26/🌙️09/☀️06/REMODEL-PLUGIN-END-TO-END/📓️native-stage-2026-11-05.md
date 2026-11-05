# Native Stage Evidence — 2026-11-05

Root inspected the current canonical Cargo launcher and its public phase ledger, then queried the live run handles separately. This distinguishes recorded preparation from compiler execution without treating a sticky heartbeat, elapsed time, zero CPU sample or absent descendant as proof.

Canonical launcher `repo/library/process/cargo/launcher` writes store begin/ready, inputs begin/ready, graph begin/end, source begin/end, admission and child begin/end/error. Its actual stream execution calls runOwnedCommand with the same controlled signal/environment, forwarding lines. Graph and source admission precede final compiler execution. Scheduler preparation happens before main inputs begin; its one-unit admission is not the full plugin graph.

The current ledger is `.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/timing/cargo-phases.jsonl`. The retained read-only native-stage-status helper accepts up to eight numeric process identities, reads only this ledger and its one rotation with a16MiB per-file bound, and emits matching marks. No live process or store is modified. Snapshot at1793847746093 reads the80366byte ledger successfully.

| Gate | Captured PID | Store Ready Elapsed ms | Main Inputs Begin Elapsed ms | Latest Retained Mark |
|---|---:|---:|---:|---|
| Artifact workspace71724 |41432|337569|1490536|inputs.begin|
| Hub52625 |53092|325730|1662550|inputs.begin|
| Component17951 |40120|423143|1658600|inputs.begin|
| JPG45038 |36356|410222|1525645|inputs.begin|
| Image62574 |24588|331597|1488293|inputs.begin|
| UItext35598 |43512|394463|1280419|inputs.begin|

There are no later retained inputs.ready/graph/source/child marks for these PIDs in this snapshot. Combined with the current configuration-publication/input messages, the evidence is consistent with ongoing input preparation. Ledger appends are best effort, so missing marks alone do not prove no compiler could have run. Actual gate success still requires terminal output and tests; handles are separately confirmed live. These elapsed intervals include shared coordination and are not CPU durations or completion estimates.

The broad process snapshot at1793847382876 likewise sees the captured subtrees live and no Cargo/rustc descendants at that instant. That observation is qualified rather than promoted to proof of a deadlock or compiler result. Existing gates are retained. No new native invocation, policy change, bypass or cancellation was performed for this diagnosis.

## Subsequent Live Snapshot

Ledger1793848138523 retains the same latestinputs.begin marks. Separate ownedleases1793848141239 shows workspace41432 nowholds configuration-view28 and compiler-writer, Hub53092 configuration-view20, and component40120 advances to queueposition2of4 behindruntime48836. Actualshared JSproducer55240 remainslive and reports ownership:frontier done1942208, held2091s. Counter scope is cumulativewithinthat ownershipinvocation/replays, not unique files or a completionestimate. These observable coordination changes support continued verifiedwaiting, not a deadlock/completionclaim. Allsixroot handles were repolledlive; no newnativecall, cancellation or policychange.

## Exact Main-Input Continuation Scope

Root inspected current repo/library/process/cargo/dependencies. withRepositoryCargoInputs first admits environment/storage arguments and ensures original lockfiles, then holds exact per-selection configuration views, rechecks those views, and enters owned registry readiness/fetch custody before calling the launcher callback that marks inputs.ready. The locked dependency fetch is an owned captured Cargo command; final compiler stream execution is a separate later launcher child. Compiler workspace shared-writer custody is deliberately taken before registry fetch custody. This explains what the recorded maininput interval can contain; no specific unlogged substep is inferred solely from elapsed time. No source or policy edit was made. Canonical source: dependencies/🟦️.ts lines50–86 and launcher/🟦️.ts lines229–237 at inspection.

Worker image62574 reports registry-fetch exclusive queue2of2 behindactualworkspace41432 with cargo progress (~466s). This directcoordination evidence identifies the currentregistryfetch predecessor and refines inputs.begin beyond elapsedtime inference. UI35598 publicationqueueadvanced behindcomponent40120. Compiler/testresultsremainpending; existinghandlesretained.

## Full Workspace Reaches Graph Capture

Authoritative retained ledger snapshot1793850208445 shows workspace71724 captured41432 inputs.ready at1793850127361, elapsed6178399ms, and graph.begin at1793850127363, elapsed6178401ms. This supersedes its earlier inputs.begin latestmark. Current delivered stdout/stderr has cargo:planning-capture plus Cargo package-cache-lock waits; the handle remainslive. Full source admission, final compiler child and actualtests are not yet proved. Other owners retain their own independent stages. No cancellation/restart/newnativeinvocation.

Subsequent delivered workspace71724 chunk explicitly advances through cargo:unit-graph and cargo:foreign-source, without source/compiler/test verdict. Rootmatched actual resourcefields, not an elapsed-time guess. Package-cache wait is therefore an earlier observation, not a permanent currentstageclaim.

## Actual Source Admission Beyond Previous Deadline Failure

Workspace71724 delivered actual source-admission sequence for1037units. Unit77 semio-framework-job-context completes with cumulative91readOperations/464files/4914943bytes; these are cumulative graph counters, not thatunitalone. Units78–86 complete and87semio-framework-kernel enterscfg. Prior70165 failed during job-context77 cfg without completion. Currentrun therefore passes that concrete priorfailure stage after the deadline repair; this is neither whole-source-admission completion nor Rust/testsuccess. Fullraw delivered70986characters retained through boundedlogger; no truncation shown. Existinghandle retained, no budgetraise/restart.

Workspace71724 sourceadmissionadvances through212complete to213semio-framework-pack-json cfg (149cumulativeReadOps,1296cumFiles15312489B). Earlier110os-channelread completed. Worker image62574 independentlyadvances postregistry throughplanningcapture to cargo:unit-metadata. Neither is anactualcompiler/testresult. UI35598 currentregistryqueue5of6behindHub53092; allsixroothandlesandfullfleetremainlive. No duplicateinvocation or scopechange.

Actualadmissionnowadvancesworkspace71724 through278plugin-interpreter complete/current279plugin-interpreter-imports read (408cumreadOps/1804cumFiles24038878B). Independentimage62574 mainadmission324of436 withmemo0physicalreads,2017cumfiles24658443B. Bothacceptedhandlescontinuewithoutcompiler/testverdict. Thiscontinuationverifiedsamehandles/fullfleet andnewadmissioncounters; no restart/extra nativeinvocation or capchange. Previousgoalturnactualinputready/graph/sourceprogressclassifiedasprogress.

## Actual Presentation Source Refusal

Workspace71724 is nowterminalexit1 beforecompiler: sourceunit376of1037 semio-framework-ui-contract-presentation refused with Unbalanced Rust source delimiter. Units374identity/375operations completed; presentationphysicalread reached521cumreadOps2416cumfiles32172247B beforefailure. Fullterminal75480char deliveredchunk retained. Image62574 independentlyterminalexit1 withsamecrate/sourceerror, noimage lawrun. This is a shared framework prerequisite failure, not plugin testpass/failure. Engineowns current-livebyte + independentrustfmt/ownedparser diagnosis; rootwillnotrestart untilverifiedrepair/currentchangedsourceproof. Rootfiveotherhandlesremainlive:52625,17951,4515,13612,95649. No cancellation of peers/currentgates.
