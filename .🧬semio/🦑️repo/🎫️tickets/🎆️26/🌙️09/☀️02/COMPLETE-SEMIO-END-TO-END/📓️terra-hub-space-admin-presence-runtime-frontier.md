# Terra Hub Space Administration, Membership, and Presence Runtime Frontier

Read-only current-tree audit, 2026-09-05. No build, native law, browser, or process command was run for this report. “Available” below means registered/current source evidence, not a fresh pass.

## Decision

**P0 is the missing authoritative Home identity binding; it makes normal author administration unreachable.** The Hub now has a bounded, receipt-bound administration page and command route, and the browser has a retained administration operation. Do not redo those server slices. The authenticated Shell resolves the actual Hub `userId`, but it never invokes the Home application's registered `setClient` action. `HomeConfig.client_id` consequently remains its empty default, so `home_space_rows` assigns no caller role and the visible editor renders only `openSpace`, never `manageSpace`/Share/Rename/Delete for an author.

There is also a small **test-compilation repair** which must precede a Home native gate: `HomeSpaceRow` acquired required `role`, while three editor/viewer fixtures still omit it. One editor test also still expects four actions where an author now has five. This is test-only but makes the native proof unavailable as written.

The bounded first implementation is therefore: bind the *server-returned identity user id* to the exact Home instance whose folded projection is rendered, before its first event page/ACK; make that binding replacement-safe on identity change; repair the stale Home test fixtures/assertion; then exercise the existing administration path in the existing two-browser collaboration launcher. Do not substitute the Shell session actor, a browser-supplied name, or a presence peer for this identity.

## Revalidated Current Path

| Boundary | Current seam | Status / remaining fact |
| --- | --- | --- |
| Authoritative caller identity | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1950-1972` obtains `/auth/sessions/me` and persists `{ userId, displayName }`. | Present, but no path forwards it to Home. `shellActorIdRef` at 1967 is a local session actor, not the member `userId`. |
| Retained Home owner | `.../🏛️ShellHost/🧬️contracts/📇️directory-bootstrap/🟦️.tsx:70-97` creates a **non-visible** Home instance and immediately sends `directory-bootstrap-open`; page application only invokes `applyDirectoryEventPage` at 116-169. | No `setClient` action/address/input exists. The owner may not be the visible Home instance, so an implementation must prove the identity and projection are applied to the rendering instance rather than blindly updating this hidden owner. |
| Home identity state | `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:240-251,390-393` defaults both identity fields to empty and changes them only through `HomeConfigMutation::SetClient`. `🎮️commands/🪪️set-client/🦀️.rs:11-22` registers the action. | A repository-wide current-source census found no production caller for this app action outside the command definition/metadata. |
| Role-gated actions | `✏️s/🔌️plugins/🪐️space/🦀️.rs:477-500,536-550` returns `None` on an empty client id; `.../🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs:71-91` emits lifecycle controls and `manageSpace` only for a Hub `Author`. | With the unbound default, a real author has no normal UI route to management. |
| Pane transport and final authorization | `🎮️commands/🏛️manage-space/🦀️.rs:20-26` emits `os.directory.open-administration`; ShellHost consumes it at `.../🏛️ShellHost/🟦️.tsx:3516-3537`; `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2060-2258` owns one cancellable page/command operation. Hub `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4023-4055,4332-4346,4349-4427` authenticates commands/pages, paginates, and revalidates after reads. | Source path exists. It is not a normal-user runtime proof until the Home action is reachable and a browser journey opens it. |
| Ambient presence | Hub lease slots and bounded/sorted publication are at `bin.rs:1542-1661`; document client frames reach `refresh_presence` at `3294-3296`; the React footer now renders `PresenceBar` at ShellHost `7416-7422`. | Lease expiry/ordering has focused laws, but no browser composite proof. The old collaboration script’s assertion that the React bar does not exist is stale. |

### Exact P0 correction seam

1. Extend the retained Home owner input with an internal, server-derived `{ userId, displayName }` acquired only from the successful `/auth/sessions/me` response. Before it may post `directory-bootstrap-open` or apply a page, invoke Home's exact `setClient` action on the same app instance and require its typed/config terminal result. Build the action address as strictly as `directoryPageInvocation` does at `.../directory-bootstrap/🟦️.tsx:116-129`; do not introduce a mutable renderer-side role flag.
2. Establish which Home app instance feeds the visible table. If the retained owner is projection-only, route the same identity action to the visual Home owner instead (or make the single retained owner render). A hidden-owner-only fix is insufficient unless the UI test demonstrates that the rendered row reads its config.
3. Treat a new identity, sign-out, owner close, or late action completion as an epoch transition: retire the previous owner, clear its page/capability/pending work, and forbid it from writing the new identity. Never feed `shellActorIdRef.current`, a query value, an event-page value, or any presence bytes into `client_id`.
4. Keep the Hub as final capability authority. The local role only hides or exposes affordances; the page and mutation routes must still reject a stale/downgraded caller.

### Required small test repair

`HomeSpaceRow.role` is required at `✏️s/🔌️plugins/🪐️space/🦀️.rs:477-490`, but the following current `#[cfg(test)]` literal constructors omit it:

- editor local/hub rows: `.../✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs:190-196`;
- viewer hub row: `.../👁️viewer/🎭️modes/👁️view/🪟️windows/🏠️main/🦀️.rs:86-88`.

Set local/viewer rows to `None`; set the editor author row to `Some(DirectorySpaceRole::Author)`. Update `a_hub_row_stamps_the_space_row_id_and_carries_dispatchable_row_actions` at editor `:225-235` from four to five buttons and assert `manageSpace` carries the exact row `spaceId`. Add a spectator Hub row (`Some(Spectator)`) and an empty/old-identity row test: both must have only `openSpace`. These are not compatibility changes; they bring the now-required struct shape and current author policy into the test corpus.

## Bounded Execution Handoff, In Order

1. **P0 — Home identity-to-render binding.** Add the explicit action dispatch/terminal check and the fixture repairs above. The single end-user assertion is: after `/auth/sessions/me` yields A, an event page containing A's author membership makes the actual rendered A row expose `manageSpace`; with B/spectator/empty identity it does not. Verify that clicking the action opens the existing pane and the page comes from the Hub.
2. **P1 — do not silently destroy an issued invite.** The renderer offers Copy at `🛂️SpaceAdministration/🟦️.tsx:273-288`. The worker clears `operation.inviteToken` before the main thread reports clipboard success at `backbone-worker.ts:2243-2251`; `copyDirectoryInviteCapabilityV1` returns `true` when `navigator.clipboard` is absent because of optional chaining, and ShellHost discards the boolean (`ShellHost:1167-1175,1708-1711`). A browser without Clipboard API, or a rejected write, loses an irrecoverable capability while claiming no failure. Change this to an operation-bound transfer/copy-result protocol: retain the token only in the worker until an explicit success response; fail visibly and retain/re-offer on unavailable/rejected copy; erase on one success; never place it in React state, URL, telemetry, or logs. A duplicate acknowledgment must not disclose it twice.
3. **P1 — normalize document presence on the server.** `stampSession` writes only color/surface in the browser (`backbone-worker.ts:1035-1041`), while Hub stores and republishes arbitrary client `peer: Vec<u8>` (`bin.rs:1590-1627,3294-3296`). The separately published directory actor is server-derived, but document chrome decodes the opaque peer. An authenticated member can forge embedded actor/label/user/role/surface/color in a raw presence frame. At the document frame boundary, bounded-decode then overwrite identity/surface/color from the admitted socket/lease; reject malformed or over-limit peers and store/re-encode only the normalized bytes. This is a visible impersonation problem, independent of lease TTL.
4. **P2 — fold composite collaboration only after the current DocumentOpenPlan activation work lands.** Reuse the detailed server-only composition packet in `📓️terra-space-admin-two-user-journey-p0.md`; do not create duplicate admin/socket infrastructure. Update its existing browser counterpart `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:2500-2650` only when the Hub-selected opening path can reach an actual document: it currently states that `os.open-artifact` lacks document identity at `2582` and incorrectly says the React presence bar is absent at `2634`.

## First Executable Laws

1. **Identity first, page second.** Extend `.../📦️packages/🟦️typescript/🎯️targets/⚛️react/📇️directory-home-bootstrap.test.tsx` (current owner/page tests at `72-104`) so a test handle records a successful `setClient` invocation containing exactly Hub `userId`/`displayName`; it must finish before `directory-bootstrap-open`, first page application, and ACK. On owner replacement, a late A terminal result must not affect B.
2. **Rendered author affordance.** Extend the current editor table laws at `.../🏠️main/🦀️.rs:205-245`: an author Hub row contains exactly `openSpace`, Rename, Share, Delete, and `manageSpace`; spectator, blank, local, and viewer rows do not contain management. This simultaneously closes the required-field test compilation gap.
3. **Normal Home → pane → Hub.** Amend the existing `@semio-tech/framework-os-dev:collab-e2e` browser process rather than a synthetic bridge: A’s visible Home row opens the pane; a real canonical page/receipt permits an administration intent; B's spectator view has no author affordance and Hub refuses the author mutation if invoked directly.
4. **Capability delivery failure.** No Clipboard API and rejected `writeText` report failure and leave one operation-bound capability available for retry; a successful write is transferred once then erased; duplicate acknowledgments and pane close leak no token.
5. **Forged presence refusal/normalization.** A raw admitted member frame with a forged peer actor/label/role/surface/color yields only the socket-derived peer to another client; malformed bytes yield no roster entry/publication. Preserve the existing old-live/reconnect and bounds laws.

## Available Gates and Honest Limits

| Gate | What it currently covers | Limit |
| --- | --- | --- |
| `bun nx run os-hub:space-administration-source-check` / `...:space-administration-native-check` | Schema/source fences plus four Hub route laws: author receipt, spectator denial, removed-member no-row leak, canonical cursor. Registered at `🌎️hub/📦️packages/🦀️rust/📋️project.json:153-167`; exact native names in `📜️script.ts:6784-6789`. | No Home author-button or browser pane proof; not run here. |
| `bun nx run os-hub:presence-lease-source-check`, `...:presence-lease-native-check`, `...:presence-lease-process-check` | Server-clocked TTL, live-owner replacement, bounds/actor order, restart-empty/member-only directory projection. Registered at `project.json:105-127`; native names at `📜️script.ts:6930-6954`. | The “process” selector executes Hub laws, not a two-browser Shell journey; it does not prevent opaque peer forgery. |
| `@semio-tech/framework-os-dev:collab-e2e` | The only registered two-browser/HUB development launcher. | Its documented document-open blocker and obsolete PresenceBar assertion make it non-acceptance today; it does not exercise the new normal administration pane or `setClient`. |
| `📓️terra-space-admin-two-user-journey-p0.md` packet | Reusable Hub/admin/SQLite/socket/restart composition design. | Server/process-facing only; it must not be reported as normal Shell UI verification. |
| `📓️sol-directory-home-browser-process-acceptance-p0.md` | Retained Home controller fixture/Chromium evidence. | It explicitly does not prove a mounted ShellHost/backbone worker, verified target handoff, or visible Home ownership. |

## Nonclaims

- This audit does not claim the Hub routes, source gates, native laws, Home materialization, or any browser process is freshly green.
- The administration page’s role/receipt/revalidation and presence lease/restart slices were rechecked, not reimplemented; the P0 above is their missing normal-user activation link.
- A Hub-selected authenticated DocumentOpenPlan/execution target remains a separate prerequisite for calling a two-user rendered document edit/presence journey end-to-end. Neither the identity binding nor a raw relay may bypass it.
