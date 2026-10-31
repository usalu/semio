# Nx Initial Watch Policy Handoff

## Ownership and observed boundary

The Linux Root watcher executor owns any implementation. Course backend worker retains read-only lifecycle/target-cold measurement ownership. No watcher, C, Native or Nx patch has been made by this worker.

Normal unfiltered caller run 54413 authoritatively exited1 before its target tests. It loaded immutable SDK `76acc988863c5ab023b945f8712b9cc50cff02ffa62387e0d4fee7d94210608e` with1,560Sources. The measured store Source hash is `327002579583bf3aa255a6ef2f0ff640ea53563fe13bce43f69ef53c48837269`, matching the three new lower spans.

## Exact owners and call tree

- Authored patch: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩹️patches/nx@23.2.0.patch`, watchWorkspace replacement around line1620.
- Repository policy: same caching root `/🕸️graph/🔏️seal/📥️membership/📥️journal/🟦️.ts`, repositoryFilesystemJournalPolicyV1 lines27–34. It reads statically imported caching `/🔣️policy.json` generatedDirectories and only closed anchored-root/name patterns from root `.nxignore`; no general gitignore is accepted as Source exclusion.
- Published patched JS: `.cache/b/t/m/nx/76acc988863c5ab023b945f8712b9cc50cff02ffa62387e0d4fee7d94210608e/node_modules/nx/dist/src/daemon/server/{start,server,watcher}.js`.
- Pinned Native source: https://raw.githubusercontent.com/nrwl/nx/23.2.0/packages/nx/src/native/watch/watcher.rs . WatchPipeline::new lines114–149; enumerate_watch_paths78–89; register_watches54–75. This external tool remains behind the existing Nx interface; no copied Source in this ticket.

Call tree: server/start.js loads immutable authorizers → withNxDaemonServiceV1 → server.startServer → await watchWorkspace → load actual repository journal policy → new Native Watcher(workspaceRoot, negativePolicyGlobs, false) → watch(callback) → return → log watcher subscription → setupWorkspaceContext → server.listen. No subscription or listening marker appears in the actual failure log.

## Closed negative policy and demonstrated mismatch

roots includes `.🧬semio`. Generated name exclusions are node_modules,.git,.nx,.cache,target,dist,📤️dist,build,out,coverage,test-results,playwright-report,storybook-static,.venv,__pycache__,bin,obj,⚡️cache,🗑️generated,.lake,pkg. Root `.nxignore` additionally contributes only its recognized root/name forms; filename/project-fixture patterns do not become blanket Source exclusions.

Patched JS passes root exclusions as `!/ROOT/**`, names as `!**/NAME/**`, and false for Native use_ignores. Native WatchPipeline constructs its event filter with additional_globs, but initial enumerate_watch_paths receives only use_ignore and walks with create_walker; it does not receive the closed additional policy. Actual initial registration emits warnings for `.🧬semio/.../🗑️generated/.../.playwright-artifacts-*`, fixed-cache Chromium profile subdirectories and `.cache/.../*.queue`. These violate the initial enumeration scope, although event filtering may later discard their events. Blindly enabling gitignore would hide legitimate ignored structural Source and is not an acceptable repair.

register_watches already skips PathNotFound after a warning; only MaxFilesWatch is returned. Suppressing the missing-path warning is not a causal fix. A proper boundary must apply the SAME closed negative policy to initial watch enumeration/registration and later descendant registration, preserving visible ignored Source. The exact120-second total has not been attributed solely to enumeration.

## Actual lifecycle and profile evidence

Actual diagnostic file `.cache/b/o/m/nx/graphs/76acc988863c5ab023b9/91fdd46f0bae76ac62eb/daemon.log` has17,567bytes, modified19:55:11.495UTC. It records backgroundstart19:53:10.357, clientpoll19:53:53.377 and unavailableafter6,000attempts19:55:10.344. It contains no explicit daemon refusal or exitstatus. The real client awaits authenticated acquired IPC before polling; on failedpoll it cancels only its own child and awaits close, but currently discards close status. Thus autonomous earlyexit is not established. No reset or unowned termination occurred.

Outer daemon client1479979 (not the timed caller-law Node child): imageopen255.839ms; reservation8952.089ms; initialbodyrecheck2917.339ms; birthstarts17033.502ms; SDKverify3190.907ms; startup lease starts28871.666ms. The target cold laws did not run, so these spans cannot replace their measurement.

Evidence: `🗑️generated/caller-environment-store-stage-spans.log`, `🗑️generated/caller-outer-daemon-tail.log`, `🗑️generated/store-stage-outer-profiles.json`, `🗑️generated/store-spans-captured-membership.json`. Historical target1293 Node1456926 timedout at30seconds:20,925.572ms prebirthstore gap, noSDKverify; allchecks/deadlines remain unchanged.
