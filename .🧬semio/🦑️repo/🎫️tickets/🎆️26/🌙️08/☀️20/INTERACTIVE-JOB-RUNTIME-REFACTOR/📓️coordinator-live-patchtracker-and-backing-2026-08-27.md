# Coordinator Live PatchTracker and Native Backing Review

The coordinator read the actual production PatchTracker grant, producer, ready handoff and close paths, plus runtime ReadyPatch/Job Drop. No production source was edited by the coordinator. These are source findings for true regression tests, not newly reproduced runtime failures.

## Actual Production Boundary

Runtime glue cfg(test)-gates the old transaction module and its export. A read-only exact-file git diff was empty at review; no author or change time is inferred. The actual production SurfaceReconciler consumer found by the scoped scan is Plugin reactor/patches/component.rs. Therefore test-only transaction changes cannot establish live output admission.

PatchTracker currently reserves a rejected-owner slot before mounted rendering, but only checks ready-slot availability later in drive_one. SurfaceReconcileJob.take_ready separately reserves a handback after candidate seal. Both capacities need exact preproducer ownership, not only a new output queue after work has begun.

The producer Complete branch evaluates reconciler.take(), reservation.take() and take_complete() as a tuple before confirming all three exist. A partial tuple must not lose the extracted original roots; all required slots must be checked before detachment. drive_one also removes the SurfaceSlot and producer into locals across the child step. The bounded ownership contract must be tested through unwind, not only ordinary Err restoration. Job Drop has an existing handback path, while ReadyPatch Drop allocates a boxed retained state; neither is evidence of a successful bounded output handoff.

Ready publication must transfer into an already admitted exact target in place. A fallback drop or ignored return_ready_patch refusal must not substitute for publication/retirement. Existing NativeCloseKey/captured lifetime/issued UI receipt semantics remain Dag-owned; the retained lane owns the narrow mounted-grant/producer/ready-pool adoption and its tests.

## Source Snapshot

```text
bc327b99d2290a8a7418e3ae517017d9f4c208141ece0664f26f803a9288fca3  Plugin/⚛️reactor/🩹️patches/🦀️component.rs
0de710f36e1f50a6f6f072898a27b672dadc1ae440b599ab8cbbc086e0375d83  UI/runtime/🦀️reconcile.rs
69cc21349494184781bef45e3e2e4f804e7ccb1a5fb14c7a37c9ad3324252cc2  UI/runtime/📦️glue.rs
```

These are reviewed snapshots, not a blanket hold on ongoing source work. Two initial path probes omitted an emoji variation selector or guessed the wrong job-directory emoji; the resulting path-not-found errors were coordinator lookup errors, not evidence disappearance.

## Actual Native Output Reviewed

The coordinator read the retained native reports and raw result/DEBUG output. The sole native executor, not the coordinator, ran these commands:

- Full UI contractR74:159PASS,0skip,6.794s,exit0.
- Runtime static/output-poolR65:6PASS,109skip,.173s,exit0. Exact fixed accounting contract390800 + runtime143568 =534368bytes, no dynamic root slot, static backing retained after final dynamic release.
- Original inline lawR66:1PASS,114skip,.047s,exit0. Original expected deltas remain[0,0]; tree-item-icon19368→19368 and reserved-binding218280→218280, while traversal items increase. No heap owner was made zero by this result.
- Actor return-wireR8:5PASS,108skip,.071s,exit0. Not a new full113 run.
- Common Kernel source layoutR1:1PASS,260skip,.016s,exit0. TurnResult2040,Effect192,Presence576,UiTurnPatches1768,fixedpage4098,fixedresult4144,messagecursor96; nativeOwnerMounted=false and sourceBackingAdmitted=false.

Raw evidence remains in the corresponding master ticket member files. The color-warning preamble mentions node:assert loading, but its actual message is NO_COLOR/FORCE_COLOR; it is not an assertion failure. Full runtimeR67 was still active when these results were reviewed.

The old three-copy estimate, logical list capacity, upfront arrays, dynamic source/candidate/output/retired overlap, Process fit and all-app timing remain separate unclosed obligations. No limit was increased and no output was published.

## Native Versus Guest Compile Scope

Dag's exact cfg review corrected an earlier overbroad blocker: undefined close_instances in poll_kernel is a real wasm32/wasip2 guest-source gap, but poll_kernel and wit_bridge are cfg-gated and do not by themselves block native Plugin --lib --no-run. Root approved one genuine native inventory after the current Kernel entries gate and Mutation's exact fixture source release. No alternate NativeCloseKey mock or compatibility harness is approved, and a native pass would not certify guest reduction.

## Actual Plugin Native Inventory R5

The sole compiler ran the real existing Plugin target with --no-run after698 selected nested source hashes were captured. It exited1 before any tests. The one observed compiler blocker is the contributed-mutation-wire #[path] at main29133 resolving below the inline plugin_runtime virtual module directory. The actual fixture exists at Plugin/🧪️tests/🧬️contributed-mutation-wire/🦀️.rs; root independently located both the declaration and the physical file. This is a module mount-path error, not an established lost fixture. All source holds were immediately released, and Mutation owns the exact correction. Raw: `🧪️member-plugin-native-inventory-r5-2026-08-27.txt`.

The new output pool's static143568-byte runtime registration does not cover PatchTracker's ten boxed per-tracker/thread-local banks. Their actual dynamic backing requires its own exact preadmission/initialization or a deliberately shared registry; it must not be mislabeled as process-static. The retained lane explicitly owns this live accounting follow-through.
