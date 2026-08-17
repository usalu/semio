# W4-F report — presence chrome, reachable toolbar affordances, and the TypeError

Lane 4-F. Working list executed in the brief's order; re-ran `bun ./📜️script.ts verify collab` after
each round of fixes.

## Summary of what was fixed

1. **`#s-presence-peers` now renders in the React shell.** `ShellHost/🟦️component.tsx`: a new
   `presencePeers` memo reads `session.viewState.presencePeersJson` (already set by the existing
   `event.kind === "presence"` branch in `ensureBackboneWorker`, keyed to the CURRENT session's
   instance — i.e. already filtered to the current `(space, document, surface)` scope with no extra
   plumbing) and reshapes the wire's `{clientId, name}` pairs into `PresenceBar`'s `{actor, label}`.
   Rendered via `<PresenceBar id="s-presence-peers" peers={presencePeers} />`, right-aligned in the
   footer navbar (`footerItems`), mirroring the wgpu shell's own placement
   (`Shell/🧊️component.rs::render_presence_bar`). Confirmed live in a browser: the element exists with
   the right id and renders its empty-state text ("No one else is here") when the roster is empty.

2. **`#s-home-create-space` / `#s-space-create-artifact` toolbar buttons wired**, real `UiNode::Button`
   elements dispatching `createSpace` / `createArtifact` with no args (the existing `mod+n`-style
   pattern for opening a dialog first). Both handlers already had (Home) or now have (Space, see item 4
   below) an "empty args → `HostEffect::OpenDialog`" branch, so no new dispatch machinery was needed —
   only a reachable DOM element with the frozen id. `bun ./📜️script.ts verify collab`'s `CollabE2eDom`
   region was updated to click these directly (`collabClickToolbarButton`) instead of hunting the
   command palette — the palette's own `argCarrying` branch opens the bottom-middle command PANEL form,
   never a `[data-slot="dialog-box"]` modal, so the old approach could never have satisfied
   `collabWaitForDialog` even if the item had been found. No assertion was weakened; `collabWaitForDialog`
   is unchanged.

3. **Fixed the `TypeError: Cannot read properties of undefined (reading 'length')`.** Root-caused via a
   standalone Playwright probe (`page.on("pageerror")` with a full stack, not just the message) against
   a freshly booted `s` react shell:
   ```
   TypeError: Cannot read properties of undefined (reading 'length')
       at parseBackboneWorkerWire (🟦️component.ts:1132)
       at decodeBackboneWorkerRequest (🟦️component.ts:1150)
       at decodeWorkerRequest (🟦️backbone-worker.ts:50)
       at workerScope.onmessage (🟦️backbone-worker.ts:57)
   ```
   `🟦️backbone-worker.ts`'s `onmessage` unconditionally treats every incoming message as
   `{wire: Uint8Array}` (`decodeBackboneWorkerRequest(message.wire)` → `wire.length`). `ShellHost` was
   internally inconsistent: **9 of 11** `worker.postMessage(...)` call sites posted a bare
   `BackboneWorkerRequest` object (structured-clone, no `.wire`), while 2 correctly wire-encoded it
   (`{ wire: encodeBackboneWorkerRequest(request) }`) — the response side already handles both shapes
   defensively (`"wire" in messageEvent.data ? decode… : messageEvent.data`), but the request side never
   did. Every plain-object post crashed the worker's `onmessage` (uncaught, inside the worker, invisible
   to the posting call site's own `.catch`). Fixed by wire-encoding all 9 broken call sites to match the
   2 already-correct ones — identity bootstrap (`open`/`send`×2/`directory-open`), `openDocument`,
   `closeDocument`, the `os.directory.*` relay, and `touchSpaceIndexArtifact`'s space-index open. This
   was the identity-bootstrap effect's *first* `postMessage` call, so it fired at every shell boot with a
   hub env configured — confirmed fixed with the same standalone probe (zero page errors after the fix)
   and re-confirmed with `bunx vitest run` (see below — same 9 pre-existing failures, no new ones; one of
   those 9 pre-existing failures independently exercises the identical code path with a hand-built test
   fixture missing the newer `messages` field, unrelated to this fix, not touched).

4. **Found and fixed a second, genuine "our own work" gap** while making the Space toolbar button
   actually usable: `SpaceIndexEditor` (`✏️editor/🦀️component.rs`) had **no
   `command_from_action` override at all**. `ArtifactEditor`'s default impl unconditionally errors
   (`app.command.unsupported`), and `dispatch_action`'s final `else` arm — the ONLY path a plain
   `onAction`/`handleAction` click reaches an app's own command through — calls exactly that. Every
   Space-app action dispatched via a button click (`openArtifact`, `requestDeleteArtifact`,
   `createArtifact`, invite/remove-member, set-visibility, copy-invite-link, …) was therefore silently
   rejected. Added the full bridge (14 actions, `str_field`/`u64_field` idiom mirrored from
   `HomeCommand::command_from_action`), plus `create_artifact::handle`'s own "empty name/kindId → open
   the `createArtifact` dialog" branch (mirrors Home's `createSpace`). Both are exercised by new tests
   (see below).

5. **`#s-home-create-space`/`#s-space-create-artifact` are wrapped with two `UiNode::Separator` spacers**
   before the button. Documented, evidenced workaround for a real, out-of-lease framework gap: a plugin
   window's root `UiNode::Stack` renders flush against the window's top edge, but the window's own
   floating tab-strip chrome (`z-index: 20`) overlays that exact strip. `ComponentSceneHost`/`TableHost`
   already gets `26px` of clearance (`--window-content-dead-line`) for free from its own wrapper; a bare
   `Stack` root does not. Confirmed via live `elementFromPoint` hit-testing at the button's own center
   (before: resolves to the tab-strip's button, not mine; after two separators: resolves to the button
   itself, ~8px of margin past the dead-line). The real fix belongs in the interpreter's `UiStackHost`
   (`Interpreter/🟦️component.tsx`, framework-owned, outside this lease) — noted in both button files'
   doc comments for whoever picks that up.

## A larger, out-of-lease blocker found and NOT fixed here

**`EditorApp<E>`/`ViewerApp<E>`'s `ArtifactApp::APP_ID` is a hardcoded placeholder
(`"surface"`, `🔌️plugin/🦀️component.rs` ~13258/13399), and `VcsArtifactApp::handle_action_invocation`'s
ownership check (`if address.app_id != A::APP_ID`, ~line 11738) still compares against it literally.**
`ShellHost`'s `encodeWindowActionInvocation` sets `address.appId = session.app.id` (the real canonical
id, e.g. `s.space.home@1/*#editor`), so this check **always fails** for any `EditorApp`/`ViewerApp` —
i.e. for essentially every editor/viewer plugin in the product, not just Home/Space. Reproduced live: a
real click on `#s-home-create-space` returns
`{"fault":{"code":"plugin.internal","message":"action app owner s.space.home@1/*#editor does not match
surface", ...}}`. This is why STEP 1 still fails after items 1-5 above are all individually correct and
independently verified working: the button is reachable, opens correctly-shaped click machinery, but
the wasm-side ownership check rejects the resulting `handle_action_invocation` call before it ever
reaches `command_from_action`/`create_space::handle`. The struct's own doc comment
(`EditorApp<E>`, ~line 13230-13239) already documents this as a known, deferred gap ("the day…
`A::APP_ID` usages… are switched to read an instance id instead of the const"), introduced when
`EditorApp`/`ViewerApp` were added earlier the same day (commit `07873f842a`). Out of my lease
(`🔌️plugin/🦀️component.rs` is framework-owned, not `✏️s/🔌️plugins/🪐️space/**` or `ShellHost`). Flagged
as a background task suggestion (`task_106e8635`) with full repro + file:line evidence rather than
touched directly.

## Per-step table (run1 = spacer fix not yet in; run2 = final, everything in)

| Step | run1 (`🧪️4-f-collab-e2e-run1.txt`) | run2 final (`🧪️4-f-collab-e2e-run2.txt`) |
|---|---|---|
| 1 create space | FAIL — `click: Timeout 30000ms exceeded` (button existed, occluded by the tab-strip — led to the spacer fix) | FAIL — `waitFor: Timeout 15000ms exceeded` (click now succeeds — confirmed by screenshot `🧪️3-c-step1-user1.png`, button fully visible/clickable; `[data-slot="dialog-box"]` never appears — the `APP_ID="surface"` blocker rejects the dispatch before `HostEffect::OpenDialog` can fire) |
| 2 share + open | FAIL — skipped (no space id) | FAIL — skipped (no space id) |
| 3 create artifact | FAIL — palette has no matching item (pre-toolbar-button) | FAIL — skipped (no space id) |
| 4 co-edit | FAIL — skipped | FAIL — skipped |
| 5 presence | FAIL — `user1's presence roster has 0 peer(s)` (element now exists — first run after the wiring landed) | FAIL — same, 0/2 peers (no document ever opens, so no presence scope exists — consequence of the same blocker, not a presence-wiring defect) |
| 6 check-in | FAIL — skipped | FAIL — skipped |
| 7 admin connections | FAIL — `[]` | FAIL — `[]` (no sync session ever opens — same root cause; `/admin` HTML half of the check passes) |
| 8 hub restart | FAIL — skipped | FAIL — skipped |

Both runs: **0/8**, and — notably — **zero `[collab-e2e] page errors observed` lines in either run**
(that line only prints when a critical page error was captured), confirming the TypeError fix held
under the real two-browser harness, not just my standalone probe.

Screenshot evidence for STEP 1 (run2): `🧪️3-c-step1-user1.png` shows a fully rendered, correctly
positioned "Create Space +" button, clear of the window tab-strip — the click lands, the button is not
occluded; the dialog simply never opens.

**What's real and independently verified even though the top-line count is still 0/8:**
- `#s-presence-peers` exists in the DOM with the right id/roster wiring (was previously "does not exist
  in the React shell" — an unambiguous, confirmed regression fix).
- `#s-home-create-space` / `#s-space-create-artifact` exist, are visible, are click-target-correct
  (dead-line spacer fix), and dispatch the right `{controllerId, action, args: none}` — confirmed with a
  standalone browser click producing the EXPECTED next-layer error (the `APP_ID` blocker), not a missing-
  element or wrong-wiring error.
- The literal TypeError from the brief is gone — reproduced before/after with a real page-error stack.
- `SpaceIndexEditor::command_from_action` now exists and is fully tested — necessary but, until the
  `APP_ID` blocker is fixed elsewhere, not yet sufficient to observe end-to-end.

## Changed files

- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️component.rs`
  — new `action_create` label (en/de).
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️component.rs`
  — `#s-home-create-space` button + dead-line spacer workaround + `render` now wraps `render_rows` in a
  `Stack`; 3 new tests, 2 existing tests updated to find-by-type instead of assuming index 0.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🏠️main/🦀️component.rs`
  — `#s-space-create-artifact` button + dead-line spacer workaround; `render` split into `render_table`
  (pure, unit-tested exactly as before) + `render` (wraps it); 1 new test, 1 existing test repointed at
  `render_table`.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱create-artifact/🦀️component.rs`
  — empty name/kindId → open dialog instead of failing on unknown kind; 1 new test.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
  — **`SpaceIndexEditor::command_from_action` added from scratch** (14 actions bridged); 2 new tests.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx`
  — `presencePeers` memo + `<PresenceBar id="s-presence-peers">` in the footer; all 9 inconsistent
  `worker.postMessage(...)` call sites wire-encoded to match the 2 already-correct ones.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` (`🔖️CollabE2e`
  region) — replaced `collabOpenCommandPalette`/`collabRunPaletteCommand` with
  `collabClickToolbarButton`; STEP 1/STEP 3 now click the frozen ids directly. No assertion weakened —
  `collabWaitForDialog`/`collabSubmitDialog`/`spaceE2eAssert` calls are untouched.

## Commands run + results (real tails)

- `cargo check -p semio-s-plugin-space` — clean (warnings only, all pre-existing/unrelated); run inline,
  not logged to a file separately.
- `cargo test -p semio-s-plugin-space --lib` — **210 passed; 0 failed** (204 baseline + 6 new: 3 Home,
  1 Space render-wrapper, 1 create-artifact dialog-guard, 2 `command_from_action` coverage/empty-click).
  Log: `🧪️4-f-space-lib-test.txt`.
- `bunx vitest run -c 🧪️vitest.config.ts` (framework-renderer-react) — **322 passed | 9 failed**,
  identical to lane 3-A/2-C/2-F's own documented pre-existing set (CSS-class assertions, an R3F crash, a
  chai matcher, `resolveWindowActions` panel-eligibility, the "Artifact"/"Document" i18n rename from the
  peer `ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` ticket, two mit-bestand asset-path regexes, a
  command-palette mock shape) — none touch this lane's changes; re-ran twice (before and after the
  ShellHost fix) with identical counts both times. Log: `🧪️4-f-renderer-react-vitest.txt`.
- `bun ./📜️script.ts verify collab` — two full runs. `🧪️4-f-collab-e2e-run1.txt` (button+presence wiring
  landed, spacer fix not yet in): STEP 1 changed from "palette has no item" to a real click-target
  timeout (the button existed but was occluded — this is what led to the spacer-workaround investigation
  above). `🧪️4-f-collab-e2e-run2.txt` (spacer fix included): final numbers pasted above.

## What is NOT done

- **STEP 1-4, 6, 8 still fail** — blocked on the `EditorApp`/`ViewerApp` `APP_ID = "surface"` framework
  bug (see above), out of lease, flagged as `task_106e8635`.
- **STEP 5** (presence roster count) will stay at 0/2 until STEP 1-4's document-opening path works, since
  presence is scoped to an open document — not something to fix independently.
- **STEP 7** (`/admin/api/connections: []`) is, with high confidence now, a *consequence* of STEP 1-4
  never opening a real sync session, not an independent admin-API defect — `/admin` itself does return
  HTML correctly (that half of STEP 7 already passes structurally, per the harness's own note).
- The dead-line spacer fix (item 5) is a workaround, not the real fix — flagged inline in both files'
  doc comments and in this report for whoever owns `Interpreter/🟦️component.tsx`.

## sharedFileRequests

None filed as formal requests — the one out-of-lease file I needed
(`🔌️plugin/🦀️component.rs`) is large, framework-owned, cross-cutting, and the fix is architectural
rather than a small additive change, so it was flagged via `spawn_task` (`task_106e8635`) with full
repro instead of asking the coordinator to perform a trivial edit.
