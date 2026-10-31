# Lively Pet Workload Transport

Actual 83108 failed the workload precondition at pet-walk:234: two completed throws (`model-3d`, `radiatory`) instead of at least three. The later overlap assertion was never evaluated. This is not evidence of a collision-engine defect.

The actual Playwright trace records a 60.267-second observation. The mouse presses occurred at +1.020s/+25.151s; releases at +21.790s/+43.125s. The two held drag/settle sequences took 20.770s/17.975s. Perch checks completed at +45.183s, after the existing 45-second loop admission cutoff. Existing carry performs 24 separately awaited trusted moves with 24 separately awaited frame evaluations, then 10 stationary move/frame pairs. Protocol round trips consumed the workload window; neither perch check exhausted its wait.

Prepared test-only correction, held until Root's coherent checkpoint:

```diff
-export async function carry(page: Page, body: Body, to: { readonly x: number; readonly y: number }): Promise<void> {
+export async function carry(page: Page, body: Body, to: { readonly x: number; readonly y: number }, transport: "paced" | "continuous" = "paced"): Promise<void> {
+  if (transport === "continuous") {
+    await page.mouse.move(to.x, to.y, { steps: 24 });
+    await page.evaluate(framesPass, 10);
+    await page.mouse.move(to.x, to.y);
+    return;
+  }
```

Only the lively-minute test selects `continuous`; every other caller retains the existing default. Playwright supplies 24 trusted native input steps through the target, the browser observes 10 actual settling animation frames, and a final real stationary pointer sample precedes the unchanged fling/release sequence. No synthetic DOM input or product state manipulation is proposed. The 60-second watcher, three-throw minimum,300-frame minimum, multiple-pet assertion, empty-overlap assertion,45-second admission cutoff and all deadlines remain unchanged.

Actual diagnostic RED 73935: one pass/one fail/77 assertions. The existing paced helper matches the neutral DATA and independent Lodash interpolation oracle. Selecting the proposed continuous mode against the current actual extracted helper still produces 35 move calls instead of three. This is a narrow function recording-port test, not browser/native-input or collision acceptance. The initial operand spelling was correctly refused by the Bun test operand guard; that separate refusal is preserved and was corrected with an explicit `./` path, without bypass.

Inputs: `pet-lively-transport.json`, `pet-lively-transport.test.ts`, `pet-collision-trace.mjs`. Generated evidence: `pet-lively-transport-red.log`, `pet-collision-trace-summary.json`. The Source helper and shared browser file are still unedited. After the checkpoint, mount the equivalent law in the existing ordinary pet-cast unit owner, run semantic checks and a real unchanged-assertion browser acceptance through the supported owner.

The documented Mouse.move `steps` option emits the requested number of interpolated movement events in viewport CSS coordinates. This supports using the existing native input interface for the same24-position path; the recording-port RED does not establish physical scheduling or event trust. [Official Playwright Mouse API](https://playwright.dev/docs/api/class-mouse#mouse-move). The actual browser check remains required.

The expanded neutral diagnostic also covers the omitted fourth argument explicitly: default and explicit paced behavior both pass, while the proposed continuous mode remains the single actual RED. This protects the default independently of the new selector. The historical two-case RED remains preserved; latest evidence is pet-lively-transport-default-red.log.

The only browser caller edit is within the lively-minute loop:

```diff
-    await carry(page, pet, over);
+    await carry(page, pet, over, "continuous");
```

The shared implementation adds the prepared branch immediately after the unchanged mouse.down call. No other caller, pickup precondition, fling coordinates or release changes.

## Independent actual shared Source review

Current carry default paced path retains all24 interpolated move/frame pairs and10 stationary move/frame pairs. Only lively-minute caller selects continuous, after same real cursor/pickup/down: actual mouse.move with24 steps,10 actual RAFs and final stationary sample, followed by unchanged caller fling/up/perch/watcher. No product engine, minimum-three-throws/300frames/multiplepets/empty-overlap assertion,45s admission cutoff or deadline change. Ordinary pet-cast owner now imports neutral default/explicit-paced/continuous observations of the actual AST-extracted helper and independent Lodash interpolation/count checks. This proves transport sequence only; real trusted browser input and unchanged collision acceptance remain unexecuted at review. No concrete static blocker found.

## Authorized repair checkpoint

After Root settled83108 with its owned SIGINT, the reviewed helper branch and single lively caller were applied. All default paced loops/pickup and minute/throw/frame/petcount/overlap/deadline assertions remain. Actual recording GREEN is3/3 with omitted/default, paced and continuous modes; the Source-owned recording interface also passed independent Node assertions/Lodash checks. One law with the same neutral3-case corpus is mounted in the existing ordinary pet-cast unit file; its ordinary owner still needs execution. Five exact Source hashes are in generated/pet-lively-source-checkpoint.json.

One exact original browser case is now owned57499 through normal registered Playwright, canonical config/project pets/one worker, retained6061, same assertion/budget scope and no new server. Native reader corrected-path owner14133 is separate. Neither result is inferred before actual terminal.

## Actual unchanged-assertion browser positive

57499 authoritatively exited0: one original pets-project case passed in1.3minutes against retained6061/8791, with normal registered Playwright and no extra server. Every original assertion completed:60-second observation, at least3throws, more than300frames, at least2pets on one frame, and no recorded overlap. The list reporter does not retain exact successful annotation counters, so only the asserted bounds are claimed. No broader pet/browser/topology acceptance is inferred.

Normal ordinary teaching target84687 now owns the complete pet-cast unit file including the newly mounted transport law; its result/count is pending. Full unfiltered teaching count must be refreshed after this added law; prior19/824green predates it. Browser receipt and five hashes: generated/pet-lively-source-checkpoint.json.

## Independent PET57499 acceptance check

Read actual pet-lively-browser-green.log: original pets-project pet-walk203 passed1/1,1.3min summary. Owner authoritative0 is recorded in checkpoint; all five current Source/DATA hashes independently match that checkpoint. Every unchanged60s/minimum3throws/>300frames/minimum2pets/empty-overlap assertion remains after watcher settlement, so the passing case establishes their bounds. Reporter does not retain successful annotation counters, and none are invented. This supersedes the earlier physical-input-pending qualification only for this exact prototype case; ordinary new unit law/fullBOTH and all other gates remain distinct.

## Ordinary-owner priority deferral

84687 exited1 before any unit: graph inference runtimeReferenceParser was undefined. After the parent genuine parser checkpoint, same full pet-cast owner58638 was queued. Root explicitly priority-deferred only this unadmitted6802MB request to unblock2048MB repair checks. At04:37:20.563UTC, exact issuer341759/start1760242/current boot/logFD/command digest and zero children were reverified; official admission still listed it waiting with zero grants. Cooperative SIGINT settled authoritative130. At04:37:32UTC the issuer and shell were absent; official waiter/grant both absent. No peer was signaled and no scope/cap changed. Receipt: generated/pet-lively-unit-priority-deferral.json. Full pet-cast ordinary acceptance remains pending the parent-approved requeue.
