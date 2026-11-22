# Framework Fixes Review (2026-11-22)

A read-only review of the framework fixes listed in `native-rust-editor-tests.md`, `icon-generation-prerequisite.md`,
`graph-service-vanishing-path.md` and `native-rust-report.md`. No sources were edited and nothing was built.

Path prefixes used below:
- `PL` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`
- `RB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`

State: commit `01324745f808` (13:42) already contains most of the fixes. The working tree adds more on top: the
mounted-owner constant changed from 33,976 to 377,696, the record-open pin was relaxed, and the maintenance-ladder
tests are new.

---

## 1. HIGH: `CloseEmissionConstruction` can close a construction that is still in flight

`PL/🪜️maintenance-ladder/🦀️.rs:792` (selection), `:169` (stage-0 step), `:803` (demand)

The new rule selects `CloseEmissionConstruction` whenever all of these hold:
- the construction is non-terminal;
- `original_emission_admission` is `None`;
- `original_emission_receipt` is `None`;
- `tool_operations` is empty.

That is also the exact state between two turns of an emission that is still being built.
`dispatch_original_emission` (`PL/🧵️instance/📨️dispatch/📥️staging/📤️emit/🎟️original/🦀️.rs:19`) calls
`OriginalEmissionAdmission::admit(cx, construction, &mut admission)` and returns `Ok(false)`. The constructor's first
call only borrows the binding and returns `Pending` (`🧰️framework/🔨️modules/🧵️job/🏭️construction/🦀️.rs:202`).
`admission` is set only once the build is `Ready`.

If the maintenance ladder runs in between, `close_step` on that construction does one of three things:
- **Livelock.** It clears `binding`, which is the last slot. The foreground's next call finds `binding.is_none()`,
  borrows it again and returns `Pending`. If maintenance runs in every gap, the emission never progresses.
- **Lost outcome or wasted turns.** It releases `authority`, which holds the capture reservation across retries, or
  drops `source`, an unsettled Source outcome that the constructor must settle first (`settling()`). The candidate is
  orphaned, or the capture is paid twice.
- **Hard fault.** `validate()` fails with "constructor changed its actual Source" when the maintenance context's
  `OriginalSourceBinding` (operation, generation, deadline) is not the command turn's.

Two smaller gaps:
- **Zero-progress loop.** When `close_step` cannot advance (frame not `Empty`, or `failure_untaken()`), the unit still
  posts a non-default demand (copy 0, depth 1) and returns `Progress(0)`, not `AwaitingInput`. That recreates the
  idle loop the fix was meant to remove, for failed constructions.
- **Sibling inconsistency.** The step charges `cx` directly. The `Worker` arm uses `cx.with_retained_loan(grant, ..)`.

This affects every plugin, because every typed command goes through the emission construction.

**Fix (clean, single owner):** retire the construction in the foreground, at handoff.
- In `dispatch_original_emission`, when `admit_original_emission_receipt` returns `true`, keep
  `original_emission_receipt` as the in-flight marker.
- Drive `construction.retire_step(cx)` until it is terminal, then clear the receipt.
- Remove `MaintenanceTool::CloseEmissionConstruction`.

If a maintenance sweep is kept anyway, tighten it:
- Select it only for a settled, idle construction. Add
  `OriginalConstructionAdmission::settled_idle()` = `complete && source.is_none() && child.is_none() && authority.is_none() && frame Empty`.
- Return `AwaitingInput` when `close_step` makes no progress.
- Wrap the step in `with_retained_loan(grant, ..)`.
- Add a law test that interleaves the maintenance ladder between every turn of a multi-turn emission build.

## 2. HIGH: the close ladder drops a reserved commit without retiring its metadata (Drop panic)

`PL/🧵️instance/📥️staging/🔒️close-ladder/🦀️.rs:141-147` (step), `:65` (demand)

The restored `commit.meta.close_step` block (`PL/🧵️instance/🧷️command-routes/📥️staging/🧊️reserved-routes/🦀️.rs:112-117`)
is correct. It is in HEAD, unchanged since `b4927d2`, and it retires the metadata before the lease. It is still the
only caller of `ReservedCommitMetadata::close_step`.

The close ladder's `ReservedCommit` rung does something different:
- it retires the Repin and Revert sub-owners;
- then it retires `permit.lease`;
- then it calls `reserved_commits.pop_front()` without closing `commit.meta`.

`ReservedCommitMetadata::drop` (`PL/👷️job/🏛️reserved/🦀️.rs:918-922`) asserts "reserved commit requires paid metadata
retirement". Closing an instance while a reserved commit (undo, redo, copy, cut, paste, revert) is pending therefore
panics. This affects every plugin.

The rung's demand at `:65` also never quotes `meta.close_demands(body)`.

**Fix:**
- In the rung, before the lease step, add
  `if !commit.meta.terminal_is_empty(){let before=..;commit.meta.close_step(cx)?;return Ok(Progress(delta))}`.
- Return `commit.meta.close_demands(body)` from the demand arm while the metadata is not terminal.
- Add a close-during-reserved-commit law test to `🧪️tests/🧪️history-edit-acceptance`.

## 3. HIGH/MEDIUM: three overlapping grant-widening mechanisms leave the interactive grant with no real bound

**Mechanism 1: the policy constant.**
`PL/🏇️mounted-owner/🦀️.rs:36-48`. In the working tree, `LARGEST_INDIVISIBLE_STEP_COPY_BYTES = 377_696`, so
`INTERACTIVE_COMPONENT.maximum_copy_bytes = 524_288`. That is 16x the previous 32 KiB per interactive turn, for every
hub composition and plugin.
- The `INTERACTIVE_COMPONENT` docstring still says "32 KiB copy" and still cites the perf ticket.
- The value was measured on process3d. The history-replay part depends on the type
  (`ArtifactDerivedOperation<P,M>` plus the birth demand), so a plugin with larger `P`/`M` exceeds it again. It is not
  a bound.

**Mechanism 2: the maintenance loan.**
`PL/🧵️instance/📥️staging/🪜️maintenance-ladder/🦀️.rs:22-24, 63`. `covering_loan` widens each unit's loan up to
`cx.retained_grant()`, the whole remaining turn authority. This bypasses the maintenance phase grant the policy
declares.
- With mechanism 1 at 512 KiB, every case measured so far is already funded, so this one is redundant.
- It applies only in the maintenance ladder. The close ladder, which shares the ingress, store-replacement and
  archive-load units, and the foreground drivers do not widen, so the same owner behaves differently by lane.

**Mechanism 3: an inline copy in the store composition.**
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🫴️record/🦀️.rs:126` and the
opening step at about `:219`.
- It is a third copy of the `cover` closure, inlined.
- The pinned receive grant is relaxed to `pinned.restricted_by(available)`. The receiver can now run under less than
  the quote it was pinned for. This is unverified; the report itself says "not yet confirmed".

**Fix:**
- Choose one mechanism. Preferred: make the large owners' steps divisible. Store-replacement `OpeningMembers`
  (377 KB), member record receive (277 KB) and history-replay preparation (34 KB) should quote per-page demands.
- Otherwise, derive the bound per app as a `const fn` from `size_of`, exposed on `ArtifactApp`. Apply it in exactly
  one place, `MountedOwnerPolicyV1`, used by the foreground, maintenance and close lanes alike.
- Delete `covering_loan` and the inline `cover`, or move a single `covering_grant` into `🏇️mounted-owner` and use it
  in all three lanes.
- Restore the exact pin check, or re-offer the pinned grant at the start of a fresh unit.
- Fix the docstring.
- The law `PL/🏇️mounted-owner/🧪️tests/🦀️.rs:463-473` is tautological, because it checks the constant against itself.
  Replace it with a law that measures the real replay-preparation and member-open demands of a fixture app and
  asserts they are funded.

## 4. MEDIUM: `[DEBUG]` scaffolding with process-global atomics remains in store code

`…/🏪️store/🧩️composition/🚪️open/🏭️operation/🫴️record/🦀️.rs:129, 219`: `static N: AtomicUsize` and `eprintln!("[DEBUG] recvnone…")`
/ `recstep…`.

Several `PL/🖥️host/**` and `PL/⚛️reactor/**` files also still contain `[DEBUG]`. The editor report says those belong
to the browser agent.

**Fix:** remove them before closing the ticket, together with the pin relax in finding 3.

## 5. MEDIUM: the dev-playground generated-mount scan

Files:
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🏭️generated-owner/🟦️.ts`
- `RB/../🎛️commands/🛠️dev/🌱️birth/🟦️.ts:61-62`
- `RB/../🎛️commands/🛠️dev/🎮️playground/🟦️.ts:117-124, 128, 161`

Problems:
- **Repeated full scans.** The synchronous scan (about 12 s) runs at birth, then again in `preparePlaygroundModels`,
  then again in `ensureWgpuRenderer`: up to three times per launch.
- **No progress or cancellation.** It uses `spawnSync` and `readFileSync` and blocks the event loop. This breaks the
  CLAUDE.md rule for expensive operations.
- **Time-of-check race.** Birth declares the launches from scan A. The later runs use scans B and C. In a worktree
  replicated live within about a second, a mount that goes missing in between produces an undeclared child and fails
  again with "Undeclared original Repo build selection".
- **Tool-specific access.** It reads Nx `📋️project.json` and the `{projectRoot}` / `{workspaceRoot}` syntax directly
  (`:30-36`), and spawns `git grep` and `git check-ignore` directly (`:19, :50`). Both bypass the tool-independent
  build-graph and VCS interfaces. The sealed repo graph already knows target outputs.
- **Path bugs.**
  - An output without a `{…}` prefix is resolved against `process.cwd()`, not the workspace (`:34`, `resolve(output…)`).
  - `directory.startsWith(root)` uses an un-normalised root (`:27`).
  - `replace` changes only the first token.
  - On Windows, a `\\?\` root or a different drive-letter case breaks `startsWith`.
- **Fallback.** For a git-ignored file with no declared output, it falls back to "the `generate` target of a nearby
  project". That can run an unrelated generator.
- **Regex limits.** `MOUNT_PATTERN_V1` matches inside comments and strings, and ignores how `#[path]` resolves inside
  inline `mod x { … }` blocks.

**Fix:**
- Compute the prerequisite list once in `rootDevPlaygroundFreshCapabilitiesV1`. Pass it through the invocation as
  declared launches. Have `ensurePlaygroundGeneratedInputsV1` run only the declared requests that are still missing.
- Resolve owners through the repo graph (`show_project` outputs), not by reading manifests.
- Put `git` behind the repo VCS interface.
- Make the scan async, with `signal` and progress reporting.
- Resolve outputs against `root`. Normalise with `resolve(root)` and compare with `relative()`.

## 6. LOW/MEDIUM: the host-action ingress condition is redundant and inconsistent with its siblings

`PL/🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs:49`: `is_none()||self.tools.original_scrub_dispatch.is_some()`

**Correctness: no defect found.**
- `OriginalScrubDispatch::advance` (`PL/🛠️tool-machine/🫧️host/🦀️.rs:118`) sets `*decision` and `binding=None`
  together, so the shell is terminal-empty when the decision is set.
- On re-entry, `prepare_original_tool_dispatch` (`PL/🧵️instance/🧾️typed-operations/📥️staging/⌨️typing/🦀️.rs:24-38`)
  takes it, or emits the frozen fault, and then returns `*decision`.
- If the shell were ever non-terminal with a decision set, `validate()` would fault instead of looping. That is loud,
  not silent. No double-close was found.

**Sibling inconsistency.** The other callers all call `prepare_original_tool_dispatch` unconditionally:
- `prepare_action_tool_dispatch` (`actions/🦀️.rs:152-155, 165`);
- `capture_original_direct_tool_tag` (`:81-87`), through `plugin-app-impl/🦀️.rs:588, 598, 645`.

That works because `prepare` is already idempotent: it returns `Ok(*decision)` when no shell exists. The new guard
only adds a dependency on a global tool slot.

**New behaviour.** A frozen scrub reached through a host-configuration action now surfaces `timeTravel.frozen`.
Before, the shell lingered.

**Fix:**
- Remove the guard and call `prepare_original_tool_dispatch` unconditionally, as the siblings do.
- Extend `host_configuration_uses_one_bounded_event_sourced_lane_before_the_generic_gate`
  (`PL/🧪️tests/🔬️app-typed-command-full-operation/🧾️source-rejection-anchors-and-host-lane/🦀️.rs:129`).
  Better, add a behavioural law: a scrub-tagged host action leaves `original_scrub_dispatch == None` and
  `shared_command_source_pending() == false`, plus a case for the frozen fault.

## 7. LOW: `process3d_demand_grant` padding

`✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🔨️modules/🏠️host/🧰️owned/🦀️.rs:23-27`

- The docstring says "exactly the demand its own owner quoted", but the code pads to the magic number `65_536`.
- This is the third copy of the same helper. The others are `jack_self_funded_grant` (`JACK_SELF_FUNDED_BODY_BYTES`)
  and `plugin_demand_grant` (`PL/🏛️ownership/🦀️.rs:157`, no padding).
- The padding hides a root-admission quote that is too low.
- It is test/testkit-only, so there is no production risk.

**Fix:**
- Add one framework helper, `plugin_self_funded_grant(demand, floor)`, with a named floor constant, next to
  `plugin_demand_grant`. Use it in Jack and process3d.
- Correct the root-admission `close_demand`/quote so the floor is not needed.

## 8. LOW: off-by-one in `retire_failure` (the `RetainedCloneClose` import fix itself is correct)

`PL/🧪️testing/🎟️caller/🦀️.rs:77-87`

The import path `semio_framework_value::retained_clone::RetainedCloneClose` matches the convention.

The loop checks for terminal-empty only at the top of each iteration. A retirement that completes on the last allowed
turn falls out of the loop and hits `panic!("…exceeds its supplied close turn bound")`.

**Fix:** check for terminal-empty after each step (or iterate `0..=N` and step only while `< N`).

It also duplicates the begin/step sequence of `ReservedCommitMetadata::close_step`. Consider one shared
`RetainedCloneClose` driver.

## 9. LOW: graph `vanishedMemberAsChangeV1`

`RB/🕸️graph/📥️load/🧲️membership-follow/🟦️.ts:20-26, 30-38`

The ticket note `graph-service-vanishing-path.md` is out of date. `settledInventory` (`RB/🕸️graph/📥️load/🟦️.ts:226-245`)
no longer calls the function. It is now used only inside `followedPathsV1`, which takes just its paths and handles them
as `path-moved`. The `BUILD_MEMBERSHIP_CHANGED` code it attaches is never read.

- **Root path accepted.** It treats the root itself (`distance===""`) as a member path. The sibling `inside` check in
  `followedPathsV1` excludes the root.
- **Masked errors (bounded).** ENOENT from any non-member reader below the root (caches, receipts) is now retried
  instead of surfaced. The `repeats>=2` and `repairRounds` limits keep this bounded, so it is not an endless loop.

**Fix:** reduce it to `vanishedMemberPathV1(root,error):string|undefined`, using the same `inside` predicate (excluding
`""`). Update the ticket note.

## 10. LOW: exit-log start row

`RB/🏡️resident/🧾️exit-log/🟨️.mjs:38`, `RB/🏡️resident/🏗️composition/🌐️host/🟨️.mjs:77, 130, 140`

- **Misleading "killed" message.** The service writes its own start row only after `await service.start()`
  (`host:77`). Until then, the last row is the pidless launchd row, which now counts as dead. A client that loses the
  service during start-up would print "killed without an exit record". The message is only shown after
  `alive===false` (`🚪️client/🛰️plan/🟨️.mjs:157, 214`), so the impact is limited to diagnostics.
- **Halved history.** Every start now writes two rows, which halves the 32-row history.

**Fix:**
- Write the service's own start row before `service.start()`.
- Have the launcher row carry `via:"launchd"` and treat it as "starting" while it is younger than the start timeout.
- Optionally, let the service's row replace the launcher's row instead of appending.

---

## (b) Other plugins and tests that encode the old behaviour

- **Literal 32 KiB grants.** Several plugin tests hand-write an interactive-like grant with `maximum_copy_bytes: 32_768`
  instead of referencing `MountedOwnerPolicyV1::INTERACTIVE_COMPONENT`:
  - puzzle `🎮️commands/📤️jobs/🦀️.rs:706`
  - draw `📤️export-document/…/🦀️.rs`
  - fem 2d/3d `🧵️session/🧪️tests`
  - layout export harness
  - xlsx and pdf editor tests
  - raster `pixel-work.rs`
  - `✏️s/🧑‍💻dev/🌊️flow/🧪️tests/🌿️catalogue`

  They do not fail, but they no longer mirror production. Any of them that drives history replay (34 KB) starves
  under 32 KiB. Point them at the constant, or at a per-app derived bound (see finding 3).
- **Tests already using the constant.** generation3d, sequence-runtime and brep-extension use `INTERACTIVE_COMPONENT`.
  They silently picked up the 512 KiB budget, which can hide any per-turn regression in them.
- **No test anywhere covers the cases behind findings 1 and 2.** That is: maintenance interleaved with a multi-turn
  emission construction, and closing an instance with a pending reserved commit.
- **Trinity.** `trinity_mounted_owner_policy()` returns `INTERACTIVE_COMPONENT`, so Jack and the other Trinity
  components also run at 512 KiB now.

## (c) CLAUDE.md rule check

- **Pragmatic workarounds instead of fixing the owner.** Findings 1, 3 and 7 patch the symptom (a maintenance sweep,
  widened loans, padded grants) instead of fixing the owner: foreground retirement and divisible, honest quotes.
- **Repeated code.** The `cover` closure exists twice, the self-funded grant helper three times.
- **Temporary logs.** `[DEBUG]` prints remain (finding 4).
- **External tools used directly.** The generated-owner scan reads Nx manifests and spawns `git` directly (finding 5).
- **No progress or cancellation.** The generated-owner scan (finding 5).
- **Inaccurate docstrings.**
  - `INTERACTIVE_COMPONENT` still says 32 KiB.
  - `process3d_demand_grant` says "exactly the demand".
  - `graph-service-vanishing-path.md` is out of date.
- **Comments inside definitions.** None were added by these fixes. The `//` comments already present inside
  `maintenance_ladder_unit_step` (`PL/🧵️instance/📥️staging/🪜️maintenance-ladder/🦀️.rs:66-67`) and inside the
  constructor (`🏭️construction/🦀️.rs:205-206, 231-232`) predate them, but they break the same rule.
- **Shims and legacy paths.** None were introduced.

---

## TS FIXES DONE

Tests: bun 1.4.2 from the repo tool cache (`SEMIO_TEST_ARTIFACT_DIR` set to `🗑️generated`).

**5. Generated-mount prerequisites (`native-build/📥️source/🏭️generated-owner/🟦️.ts`, dev playground)**
- One async inventory walk collects the Rust sources and `📋️project.json` manifests; mounts and manifests are read with bounded concurrency. Owners are computed from that inventory, not per missing path.
- Cancellation: `signal` is checked between directories and files. Progress: `advanceScriptInvocation(invocation, "playground:generated-inputs", stage)` from the existing script control.
- Paths resolve against the workspace root (`resolve(root, output)`, `within()` instead of `startsWith`). The `git grep` and `git check-ignore` spawns are gone; "generated" now means the `🤖️generated` path segment.
- `serveRootPlaygroundV1` scans once and passes the list to `preparePlaygroundModels`, `ensureBootComponents`, `ensureWgpuRenderer` and `runNativePlayground`.
- `readRustSourceClosureV1` awaits the owner lookup on its error path (it was synchronous and broke the owner-naming test).
- Measured on this workspace: about 29 s before the concurrency change, about 2.9 s after; same nine targets.
- Not done: birth (`rootDevPlaygroundFreshCapabilitiesV1`) still scans, because its declared launches are needed before the invocation exists. A launch therefore scans twice (birth, serve), not once. Carrying the birth list into the born invocation needs a change to the validated native caller schema. Owner resolution still reads `project.json` directly, because the sealed build graph needs an invocation that birth does not have.
- Birth test: the expectation now includes the generated prerequisites of the real workspace, and the file sets a 60 s default timeout because each birth call scans the repo.

**9. Graph vanished paths (`🕸️graph/📥️load/🧲️membership-follow/🟦️.ts`)**
- `vanishedMemberAsChangeV1` is replaced by `vanishedMemberPathV1(root, error): string | undefined`. It excludes the root itself and paths outside it, using the same inside predicate as `followedPathsV1`.
- `followedPathsV1` returns the path (path-moved follow-up). The `BUILD_MEMBERSHIP_CHANGED` wrapper was dropped because `settledInventory` never read its code.
- Test updated, including a `followedPathsV1` wrapped-cause case. `graph-service-vanishing-path.md` updated.

**10. Exit log (`🏡️resident/🧾️exit-log/🟨️.mjs`, `🏗️composition/🌐️host/🟨️.mjs`)**
- One start row per start: the launcher writes it at spawn (with pid, or pidless with `via:"launchd"`). The service calls `completeServiceStartRowV1`, which fills the pid into a pidless launchd row and never appends.
- `explainServiceLossV1` returns "the service is still starting under launchd" for a pidless launchd row younger than `starting` (default 120 s, options `now` and `starting`). Older pidless rows still read as killed without an exit record.
- Test added to `🧪️tests/🧾️exit-log/🟦️.ts`. The 120 s window is a constant, not the client's `startCeilingMilliseconds`; aligning them is a follow-up.

**Results**
- generated-owner tests: 4 pass, 0 fail.
- membership-follow, vanished-directory, exit-log: 10 pass, 0 fail (after `SEMIO_TEST_ARTIFACT_DIR` is set; the support module needs it).
- dev birth tests: 12 pass, 2 fail. Both failures are environment or fixture state and not in code I changed: `graph-free component transpile cache` (fixture bytes read as `base`) and `standalone GIS and Space` (no activation receipt in the cache).
- Rust source admission tests (`📥️source/🎟️admission`, `🧪️tests`): 8 failures from other in-flight work (for example `concurrentEditErrorOfV1 is not defined`, "Original parent Script process invocation required"). Not investigated.
- Not verified: a full `dev playground` launch, and the `serveRootPlaygroundV1` path (no test drives it).
