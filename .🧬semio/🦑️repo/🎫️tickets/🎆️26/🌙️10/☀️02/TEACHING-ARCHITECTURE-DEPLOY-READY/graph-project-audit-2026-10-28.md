# Whole Project Inference Audit — 2026-10-28

Read-only call-chain review; no probes, builds, process changes or source edits. No hot function is attributed by this review. All source paths below are relative to library 🏗️build/🕸️graph/📥️inference/.

## Nonpublishing Boundary

📦️project-nodes/🟨️.mjs exports createProjectInferenceEnvironmentV1(session):96–100 and inferProjectBodyV1(environment,job,admission):104–114. The latter runs complete projectWithDefaults, native preparation, release preparation and target mode attachment, returning an in-memory node. It does not call the project record writer. A single retained actual job/session can therefore be evaluated through this existing exported boundary under observeInferenceSnapshotV1 without taking a live farm job or publishing graph records. Pass undefined admission to match the farm, which normalizes all targets. Preserve real job JSON/native/pipeline/selectedInputs and session contracts, registry, commandInputs, lateClosures, projectsByRoot and componentRoots; inventing empty substitutes changes measured work.

Do not call produceProjectRecordV1:124–126: it persists np records and observation blocks. Do not call the worker service run or claim jobs from the live farm. inferReadOnlyBuildProjectSourcesV1 in 🧱️project-assembly/🟨️.mjs:504–510 is not a substitute: it deliberately rejects Cargo owners. Genuine outer probe invocation/control still owns elapsed-time/cancellation; no live job custody should be copied or fabricated.

## Concrete Worker Environment Difference

The farm 👷️service/🟨️.mjs:13–19 constructs the environment and adopts session inference authority. createProjectInferenceEnvironmentV1 builds context with only workspaceRoot, registry and lateClosures at97, then gives it to createScriptInputCache. No invocation/phaseControl is attached there. The service run at33–39 additionally wraps implementation pin and staging around the writer. Consequently adding genuine invocation/phaseControl to this context measures an explicit variant rather than the exact farm baseline. Prior genuine-control pure probes must not be described as reproducing that farm context. This does not itself explain the CPU cost.

## Stages Not Covered By The Fast Pure Queries

1. 🧱️project-assembly/🟨️.mjs:353 synthesizes the declared target union through cargoTargets, componentBuildTargets, printDocumentTargets and authored targets before target normalization. An authored-target-only31command loop omits that synthesized union.
2. The per-target normalization at383–435 performs native/process owner wrapping, toolClosure cargo-lock-owner at419, generic inputs at422, generated producer coupling429, preciseInputsV1 wrapper431, exclusions434 and mode/cache admission435. Measuring deriveInputs alone does not include this complete wrapper.
3. projectInputs at449 runs the full source input normalization after all target changes. Hash-mode compaction at453/460 differs from returning lists of files. sharedClosureGroups at456 can expand declared provider groups.
4. withClosureRecipesV1 at469–474 traverses the completed named-input/target tree and serializes selected symbolic recipes. resolveClosureRefsV1:478–484 is a later distinct consumer and is not inside inferProjectBodyV1; do not attribute it to the body without evidence.

Useful staged benchmark boundaries are full projectWithDefaults, then withNativePreparation, then withReleasePipelinePreparation, then final targetBuildModes, using the same returned in-memory node and actual shared caches. Avoid source instrumentation of the live worker; a separate retained diagnostic can report stage starts/ends around exported calls.

## Broad Loops Worth Distinguishing

🦀️native-manifests/🟨️.mjs:239–272 loops project targets. It caches generator prerequisite plans by tests=false/true, while nativeDependencyRoots may be asked again for targets consuming transitive native input groups at260–266. nativePreparation:229–235 filters all generator contracts against native roots. This is bounded target/contract work, not evidence of a cycle.

🧩️component-targets/🟨️.mjs:110–118 loops release pipeline targets; for each it unions every session componentRoot with native pipeline-job roots, then runs nativePreparation over all of them. Therefore a probe with componentRoots=[] can omit workspace-wide preparation that production includes. This is the clearest explicit whole-component loop in the reviewed post-normalization chain. It is not established as the stalled project hotspot, and it may not run at all when that project has no pipeline.

frontend_live received exact API references and the worker-context correction. No speculative performance fix is prescribed; next evidence should time the complete authentic in-memory job boundary and distinguish computation from later record publication.

## Full-Project Probe Review Checklist

Pending frontend_live retained input filenames; no full-project probe script existed at this audit observation. Canonical current construction in project-nodes/🟨️.mjs:150–162 discovers catalog/native owners and computes nativeCommandInputs when admissible Cargo config exists. At221 projectsByRoot includes both project.root and metadata.nativeRoot aliases; componentRoots comes from the complete discovered config list at222. With no ownerAdmission,231 sets selectedTargets undefined and selectedInputs to an empty sorted list;232 derives nativeCandidate from native manifest or native command driver and pipeline from actual target metadata. The farm prepare call at236 passes augmented job JSON/root/nativeRoot and those selected inputs.

A reconstructed current environment must preserve these rules and actual registry membership. A closure recipe nativeCommandInputs handle cannot be replaced by [] or null merely because only one job is timed. The single-job claim must identify an owner present in the real current registry; source path existence alone is insufficient. Historical retained session.bin data is real data but not a current-live-session identity claim unless provenance matches. This checklist is preparatory, not approval of an unread probe.

## Retained Full-Project Input Review

Reviewed ticket graph-captured-node.ts and graph-captured-runner.mjs after creation. No serious guard defect found. The child authenticates loader identities and projected invocation, verifies metadata entry/root, observes the snapshot, reads and validates canonical current owner registry, and checks the running claim includes the selected owner. It calls only inferProjectBodyV1 and captures the returned node in memory; no produceProjectRecord or live farm claim operation is called. The runner retains canonical capture/auxiliary launch, issued operation and Original remaining budget.

Node lines19–25 reconstruct the current authored framework-rs job. Current registry nativeRoots is string[] (admission/🟦️.ts:12), and root/nativeRoot owner mapping matches its canonical sourceOwners function at18. For this ordinary authored framework project, raw JSON plus root matches project-nodes:193–194; the special plugin-registry/framework-os-dev/playground augmentation branches are not selected. Empty selectedInputs and undefined selectedTargets match the non-ownerAdmission construction at231. The native manifest check suffices for this known Cargo owner. Native command inputs are computed by the real late closure API and reconstructed from its actual recipe; they are not empty placeholders. The explicit pipeline refusal makes empty componentRoots irrelevant to this one job.

The probe intentionally reconstructs a current-Source environment rather than replaying the unavailable live farm session. Its claim check establishes current running owner membership, not equality of the claim creation-time registry, source bytes, command recipe or late flag. Current registry owner maps can differ from a live session reconstructed through discovery of automatic Cargo owners. Thus describe results as a current-Source reconstructed claimed-owner in-memory job, not exact replay of producer211237. Production lateClosures=true is selected deliberately; the actual live session flag was not recovered.

These limitations do not prevent the bounded differential from measuring the full current project body, but they bound any exclusion claim. frontend_live received the findings before completing its measurement; this audit itself ran no probe.

## Probe99079 Terminal Boundary And Correction

The generated graph-captured-node.log now records terminal OwnedCommandFailure status1, signal null. Capture completed in1474ms (1462files,18,148,930bytes), captured loader entry in614ms, native-command recipe in1ms, setup59ms. It began the authentic current-Source framework-rs body with native=true, pipeline=false, selectedInputs=[],3753root entries,35contracts and context keys workspaceRoot/registry/lateClosures. It did not record full-node-complete.

The precise refusal is Original bounded graph writer custody required, through repositoryBuildGraphWriterDirectoryV1 → cargoLockInput → projectWithDefaults → inferProjectBodyV1. This does not measure whole-body duration or reproduce the live producer stall. It locates the first unprovided production custody stage after earlier provider queries; no CPU hotspot can be inferred from that refusal.

Correction to the earlier nonpublishing description: avoiding produceProjectRecord prevents project-record publication, but projectWithDefaults itself writes content-addressed Cargo/input closure leaves. inputs/🟨️.mjs:131–134 captures Cargo lock inventory and calls publishInputClosureV1. The farm normally supplies canonical private staging around its whole body (farm service:33–39). Thus inferProjectBody is not physically write-free.

The canonical next scope is withBuildGraphStagingV1 (inputs/💾️storage/🟨️.mjs:99–101) around the same body, with a private owned absolute root. WriterDirectory:206–208 checks that staged namespace and returns before global graph writer admission; this is the supported worker staging path, not a fabricated writer. Keep no NPrecord production or adoption into global cache. The probe would then measure private staged closure creation too. Only frontend_live/parent may authorize and execute that follow-up; this audit ran no probe.

Path caution from actual source: publishInputClosureV1:79–82 appends ~UUID to each content-addressed file and checks the full path against the revision path bound before write. Full ticket absolute root plus namespace,64hex.json and37character temporary suffix may exceed256. Check exact computed length; do not relax the guard or hide external input scripts to obtain a result. Existing canonical farm temporary staging is short, but its use must respect the ticket output rules.

Authority distinction remains: probe99079 authenticated its Original/captured loader and current claim membership; it did not recover producer211237 original session. A future staging probe remains a current-Source reconstruction unless live session provenance is established.

## Staged Probe18842 Independent Review

Read current retained graph-captured-node.ts and generated graph-captured-node-staged.log without launching a probe. The log records capture2278ms (1462files,18,150,109bytes), loader entry781ms, native-command recipe0ms, setup77ms and full-node-complete29,706ms. Result is framework-rs at the actual authored root, unavailable=null,32targets,339,979serialized bytes,26,540observations,1654analysed modules. frontend_live separately confirmed its owned execution terminal0 with final Original guards/normal settlement. No first failure is recorded.

The retained entry line19 now wraps the same setup/observed body in canonical withBuildGraphStagingV1, using only ticket 🗑️generated/n and a staged-leaf callback. It retains loader authentication, genuine received invocation, issued30second query control and final authority/proof checks. It calls neither NPproduce nor global staged-leaf adoption. Private writes are nevertheless real:207leaves in build-lock-closures, build-input-closures and inference (derived recipe records). At this read, physical staging inventory independently matched207files and maximum final path202UTF16 units. Lock closure temporary paths add37units (observed maximum238); the retained conservative preflight includes an extra inference segment and stays within256. Source publisher also enforces the path guard. frontend_live owns removal of its generated staging after inventory; this audit changed no outputs.

This demonstrates one complete current-Source reconstructed framework project body, including private closure/recipe staging, finishes in about30seconds under the captured Bun loader. It does not reproduce a40minute full graph stall or measure the128project shared environment. It does not time observeProjectInferenceV1 record validation, observation packaging/blocks, NPpublication, staged adoption or later resolveClosureRefsV1. The most useful next measured boundary is the production observation/record wrapper or an authentic shared-cache context, with owned isolated staging and explicit authorization for its writes; another raw provider microquery would not close that gap.

The current validated registry/claim membership/authentic Original establish legitimate probe authority. They still do not make it the exact session originally admitted by producer211237. Preserve that distinction in any performance exclusion claim.

## Observation Record Wrapper Benchmark Prerequisites

Read-only source review; no benchmark launched. observation/🟨️.mjs:139–152 exports observeProjectInferenceV1. Supply explicit real reusable predicate, owner, production project key, actual stamps and unique private cachePath. The production key at project-nodes:235–236 includes project contract authority, registry digest, native command revision, assembly authority and late closure revision before root/json/selected inputs. A private diagnostic key may deliberately differ, but must not be called the live producer key. Preserve inferProjectBody compute and the same observation snapshot; do not fake authority digests or disable schema/reuse/current-source guards.

The private staging wrapper must enclose the complete observation wrapper, not only its body. cachePath belongs under its private inference/np namespace; companion .records is created automatically. observedRecordStoreV1 at observation:701–709 installs canonical publication, flush/retain/compact/release occur at775–779, final header packing and optional publication at788–792. Staging optional publication (inputs/storage:116–118) checks private path containment, writes and notes leaves; it does not exercise mounted graph quota/headroom behavior. A successful private result therefore covers record encoding/pack work, not global admission or adoption. Account for .records pack/block names and temporary suffixes in path bounds.

The wrapper seals load authority via inferenceAuthorityDigestV1:123–129. A current-Source probe must retain its actual authority; adopting fabricated producer rows would invalidate it. Source snapshot at87–90 creates nested project validation behavior. retain/inherit observations, JSON roundtrip and row validation are part of the measured wrapper; recordScopes is created by it, not supplied as a fabricated external object.

Cold means a fresh private cache path and no prior in-process project image. A same-process same-path/key warm repeat can take projectImages fast path143–145 and never read its physical record; label it in-memory warm reuse. A disk-warm comparison requires an independently admitted fresh process over retained private data and identical actual key/authority. RACY_MS and inconsistent/source-changing facts can suppress physical publication even though compute returns (788); count compute callbacks, confirm physical cache/pack outputs and observe actual branch instead of equating fast repeat with disk reuse.

## Eight-Loop Producer Context

project-nodes:77–81 defaults requested concurrency to max(1,min(8,availableParallelism()));249 caps it by job count and creates the real memory governor using environment graph budget, measured baseline and phase signal.253 starts that many governed dispatch loops;254 may grow loop count after farm readiness. Thus eight is not proof of eight simultaneously admitted jobs. The128project producer shares one environment scripts/native maps (223) and parent validation snapshot, while each wrapper opens its own record store scope.

A small faithful concurrency comparison needs distinct actual jobs and cache paths, the same shared environment and canonical governor/budget, and preserved observed snapshot. Eight independent processes, ungoverned Promise.all, or eight copies of the same owner produce different caches/authority/storage collisions and do not reproduce that context. The current producer environment and session provenance remain unrecovered; any reconstructed eight-job measurement must say so. frontend_live received these prerequisites and evidence boundaries.

## Inline Late Resolution After The Record Wrapper

Root identifies actual producer211237 as inline with no farm children. Production project-nodes:236 uses reusable = projectRecordReusableV1 AND inferenceInventoryReferencesHoldV1, then237–242 runs resolveClosureRefsV1 when late before the dispatch loop increments finished at251. A farm worker record completion therefore does not establish inline owner completion. Earlier record-wrapper probes omit this post-resolution stage.

The exported resolver in 🧱️project-assembly/🟨️.mjs:478–484 returns early only without closureRecipes. Otherwise it resolves every recipe using the SAME scripts cache, recursively binds marker values through namedInputs and targets, compacts/deduplicates symbolic groups, and merges identical commandSources groups at488–500. This may perform additional roster compaction and whole-node rewrites beyond the already measured per-target questions.

🧭️script-inputs/🟨️.mjs:859–863 sequentially ensureClosureEntry for each recipe, merges direct rows and shared layers, substitutes derived walk facts and excludes duplicate supported walk layers. Module maps at704 share entries, inflight promises, failures and literal inputs across projects. Entry validity at728–733 is memoized by current observation round. New entries at737–755 use detached observation and the currently bound workspace scripts; closureApi:806–809 binds late scripts. Thus a cache/environment reset or independently constructed parser cache changes the work being measured.

Strong bounded next measurement: within the same genuine captured Original, current observed snapshot and canonical private staging, retain the actual cold/warm NP wrapper result and its same environment.scripts, then time exported resolveClosureRefsV1 separately. Record recipe count, literal-input table count, returned row/derived counts, final node size/hash and final authority/proof checks. Measure warm repeated resolution separately; do not count wrapper return as owner finished. No new probe is launched by this audit.

Fresh-process NP replay requires the actual closureInputs table and normal record/provider settlement: script-inputs provider settle:797–801 hydrates known recipes/literal inputs before resolving required provider entries. Dropping that table can cause literal-inputs-unknown at893–896 and is not a performance result. Same-process returned result/cache avoids that ambiguity.

The production reusable inventory guard is separate: observation/📥️reuse/🟨️.mjs scans source/lock marker leaves through the real graph consumer, checks ancestry/device/singlelink/bounds/content identity and yields/rechecks its actual control/store. Private staged closure leaves are not automatically available in that global consumer. Do not replace this predicate with true and claim faithful production reuse, fabricate a mounted store, or globally adopt diagnostic leaves. Either retain its exact supported custody and report observed result, or explicitly mark global inventory reuse validation as an unmeasured boundary. The last private NP wrapper and late-resolution comparison can still be informative under that stated limit.

Frontend_live received exact references and the same-cache/closureInputs/reuse-guard constraints. No code, process, live session or global graph state was changed.

## Producer211237 Captured Source Provenance Check

Bounded read-only metadata check; no live pipe/socket content was read, no attachment/signal, no broad cache crawl and no source copy. /proc/211237/cmdline was filtered in memory to executable, digest and short command tail only. Its actual auxiliary loader expected digest is09779154bf5ca995db25160c46cbe069c998a0c103f256b5a394ba45dd4eb7f5; command tail is run @semio-tech/framework-os-mcp-rs:build --output=stream --parallel=1. The evaluated loader text was not emitted.

Prior graph-cpu report104 names /tmp/semio-graph-farm-image-194452-cXr00b/snapshot.json for worker212277 and its1633rows/sixteenchangedfiles comparison. That exact prior snapshot no longer exists at this check. /tmp contained no top-level semio-graph-farm-image/session directories. This old-worker evidence must not be transferred to211237.

The canonical sealed policy names .🧬semio/🦑️repo/⚡️cache/implementation; that store path is absent here. The exact known live claim49620f499d3a2883fbe02d40 remains running with128owners, but its closed metadata keys are base, implementation, owners, stamps, startedMs, state, token, version; it is not a persisted source snapshot location. Frontend_live confirms no known exact persisted image pointer, no farm session for current inline producer, and that its earlier open NPpack is an inference record pack, not the captured Source image.

No readily available authenticated persisted snapshot matching211237 expected digest was found through these known locations. Therefore exact old/current relevant module differences cannot be established by this audit. The current probes execute fresh currentSource; fast results do not eliminate an implementation difference in the long-running process, but neither do they establish such a difference or identify a hotspot. Even a static byte divergence would require a measured causal link before attributing65CPUminutes to it.

## Current Producer211237 Resource Samples — 19:09 UTC

Exactly two read-only /proc211237 stat, schedstat, status and smaps_rollup samples were taken through repo Node,20.044seconds apart:2026-10-28T19:09:04.139Z and19:09:24.183Z. No signals, attach, external profiler or live pipe/socket read. Both sampled task states were R.

| Counter | First | Second | Delta |
|---|---:|---:|---:|
| User CPU ticks |408350|410210|1860|
| System CPU ticks |37655|37800|145|
| Scheduler runtime ns |4370359030034|4390267971400|19908941366|
| Scheduler wait ns |23731876924|23735992477|4115553|
| Minor faults |43105368|43314027|208659|
| Major faults |1359710|1361483|1773|
| RSS kB |2160552|2165124|4572|
| PSS kB |2142394|2146487|4093|
| Swap kB |369900|362816|-7084|
| SwapPSS kB |369900|362816|-7084|

Scheduler runtime advanced19.909seconds in20.044wallseconds (about99.3percent of one CPU), while scheduler delay advanced only4.116milliseconds. This directly establishes current211237 is consuming CPU rather than predominantly waiting to be scheduled during this sample. Major/minor faults are active and swap remains about354MiB at the second sample, so memory pressure/page-in activity is real; the swap total decreased about6.9MiB. These counters do not identify file versus swap origin for individual major faults or a JavaScript stack. Despite those faults, the sampled main task spent nearly all measured walltime executing; a primarily scheduler-stalled or disk-sleeping explanation is unsupported for this20second window.

This current-producer evidence supersedes relying on the earlier212277 CPU sample for that narrow conclusion. It does not prove useful graph progress, identify an algorithm, establish a source snapshot difference or explain the entire65minute history.
