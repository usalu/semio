# Composer Audit — Spaces Collaboration and Administration End-to-End

**Date:** 2026-09-09  
**Scope:** Read-only inspection of the current working tree only. No files were modified except this report. No build, browser, database, socket, or registered gate was executed in this audit; runtime claims below cite source structure and registered process contracts, not fresh execution.

## Executive verdict

**Neither obligation is fully proven end-to-end today.**

| Obligation | Current classification | Why |
|---|---|---|
| Users collaborate inside spaces | **YELLOW — hub process-proven, browser journey incomplete** | `os-hub:space-journey-process-check` drives two real authenticated identities through membership, administration pages, global/scoped directory sockets, live rename wake, scoped-socket revocation on removal, and restart recovery — but it **honestly skips** document open-plan, presence roster, content edit, rendered surface, and invite-token legs. The registered browser `collab-e2e` target still boots on removed credential carriers (`OS_HUB_ADMIN_TOKEN`, `S_USER`). |
| Admins administrate spaces | **YELLOW — control plane present, terminal delete and live admin member journey unproven** | Admin SPA + relay + `AdminIntentV1` path is real for create/rename/visibility/archive/delete/member/invite via `/admin/api/intents`. Registered `os-hub:admin-live-journey-check` proves bilingual create/read/rebuild only. Shell `SpaceAdministration` pane + worker retained operation are source/native-law strong, but **delete-space terminates on receipt only** with **no GET-after-delete missing-page proof** anywhere in the process corpus. |

The control-plane seams are largely built; the remaining gap is **observed terminal proof** across delete, invite copy (admin lane), two-browser collaboration, and admin SPA member operations.

---

## 1. Space administration pane (`SpaceAdministration`)

**Path:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛂️SpaceAdministration/🟦️.tsx`

### What is present (source-green)

- Renders solely from hub `DirectorySpaceAdministrationPageV1`; author-only invites and capabilities; member/public pages structurally omit admin affordances.
- Closed intent vocabulary: rename, visibility, member role/remove, create/revoke invite, delete, pagination, close, **copy-invite-capability**.
- Capability gating uses `page.capabilities.*` only; dispatch disabled unless `phase === "ready"`.
- Delete uses two-step confirmation (`alertdialog`) before emitting `{ kind: "delete-space" }`.
- Invite copy button appears only when `inviteCapabilityPending === true`; copy status merged into the single `role="status"` live region.
- Full EN/DE label bundles via `spaceAdministrationUiLabel`.

### Shell bridge

**Path:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`

- `shellSpaceAdministrationOpening` → `directory-administration-open`.
- `shellSpaceAdministrationRequest` maps intents to worker requests; hostile intents return `null`.
- `reduceShellSpaceAdministrationState`: `deleted` terminal requires accepted/previously-accepted outcome + 64-hex receipt; erases page.
- Clipboard: worker posts `directory-administration-capability`; Shell calls `copyDirectoryInviteCapabilityV1` then posts exact `directory-administration-capability-result`.

### Tests and fixtures (source-only evidence)

| Asset | Path | Covers |
|---|---|---|
| Pane + reducer laws | `.../🧪️tests/🏛️space-administration/🟦️.tsx` | Delete gating, deleted terminal, clipboard unavailable/success, invite capability lifecycle |
| Delete fixture | `.../🧫️fixtures/🗑️delete-space/🔣️.json` | Accepted outcomes, refused phases, EN/DE copy strings |
| Worker harness laws | `.../🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts` | Full administration turn machine, delete receipt terminal, invite capability retention |

**Pane status:** source-complete for author administration UX; not process-proven in a real browser against a live hub.

---

## 2. Backbone worker — `completeDirectoryAdministrationDeletion` (delete-space)

**Path:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts` (approx. lines 4728–4745, 4892–4898)

### Implemented behavior

```text
delete-space command
  → POST /directory/commands (sealed request)
  → receipt outcome accepted | previously-accepted AND result.kind === "none"
  → completeDirectoryAdministrationDeletion
       phase = "deleted"
       receiptSha256 retained
       page/canonicalJson/inviteToken cleared
       NO subsequent loadDirectoryAdministrationPage
  → postDirectoryAdministrationState once
```

Non-accepted outcomes → `failed` terminal, never `deleted`. Opening administration on a missing space (404) → `denied`/`forbidden`, never `deleted`.

### Worker test law (explicit)

**Path:** `.../🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts` — `"settles delete only from an exact accepted receipt and never interprets a page 404 as deletion"`

After successful delete:

- Terminal state `{ phase: "deleted", receiptSha256 }`.
- **POST count = 1** (command only).
- **GET count = 0** after submit — deliberately no refresh, no missing-page probe.

### RED — delete missing-page terminal

There is **no** registered process step that:

1. Issues `delete-space` against a real SQLite hub.
2. Then `GET /directory/spaces/{id}` (or re-open administration) and asserts **404/empty denial** with no member/invite leakage.

Hub native laws cover removed-member denial (`space_administration_page_v1_route_denies_a_removed_member_and_leaks_no_rows` in `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`) but **not** post-`SpaceDeleted` page reads after an author-initiated delete command.

**Classification:** receipt-only terminal is **intentional in worker source**; **end-to-end delete proof is RED**.

---

## 3. Hub directory commands, presence, two-user sockets

### Directory command + publication ordering

**Path:** `/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs`

- `append_and_publish_locked` keeps append + synchronous `tx.send` inside the same writer-guard lifetime (`execute` → lock → decide → append → publish → unlock).
- Prior audit RED about `drop(clock)` before fanout appears **repaired** in current source.
- Registered gate: `os-hub:directory-ordered-publication-check`.

### Space administration HTTP surface

**Path:** `/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs` — `get_directory_space`, `build_directory_space_administration_page_v1`, `seal_space_administration_page_v1`

- Author/member/public discriminated pages with canonical receipt + session binding.
- Removed member: next read is denial with no row leakage (native law).

Registered gate: `os-hub:space-administration-native-check` (four bin-unit laws listed in `📜️script.ts`).

### Two-user space journey (strongest collaboration evidence)

**Fixture:** `/Users/ueli/Documents/semio/🌎️hub/📇️directory/🧫️fixtures/🚻️space-journey-v1/🔣️.json`  
**Driver:** `proveDirectorySpaceJourneyV1Process` in `/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts`  
**Gate:** `bun nx run os-hub:space-journey-process-check --skip-nx-cache`

**Proven steps (when gate passes):**

| Step id | What it exercises |
|---|---|
| `create-space` / redacted retry | Idempotent command receipt |
| `author-reads-administration-page` | Author page + capabilities + receipt verify |
| `nonmember-administration-page-denied` | Non-enumerating denial |
| `author-adds-b-as-spectator` / `member-reads-administration-page` | Member page omits invites/capabilities |
| `author-promotes-b` | Role upsert |
| `announce-document` | Descriptor announcement |
| `open-global-sockets` / `open-scoped-socket` | Authenticated socket hello + grants |
| `member-renames-space` / `author-observes-dirty-wake` | Live directory event on peer action |
| `author-reads-ordered-event-page` | Bounded event page, no cross-space leak |
| `author-removes-b` / `scoped-socket-revoked` | Scoped socket closes (4401); global socket may remain (documented) |
| `removed-member-page-denied` | Administration page denial |
| `author-self-revokes` / `self-revoked-read-denied` | Session terminal + socket 4401 |
| `restart-preserves-history` | Durable space + renamed history |

**Honest skips (explicit in driver):**

| Skip id | Reason |
|---|---|
| `document-open-plan-and-presence` | Open-plan not admissible / not driven |
| `document-content-edit` | No mutation commands |
| `rendered-surface` | No renderer |
| `invite-token-leg` | Invites not created in journey |
| `alternate-directory-backends` | SQLite only |

**Not in journey:** `delete-space`, admin SPA, browser Shell, presence lease expiry under stalled transport.

### Presence

- Fixture + oracle: `🌎️hub/🧫️fixtures/👥️presence-lease-v1/`  
- Gates: `os-hub:presence-lease-{source,native,process}-check`  
- Space journey **does not** claim presence; skip documents open-plan block.

### Obsolete browser collab target

**Path:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`

- `collab-e2e` still sets `OS_HUB_ADMIN_TOKEN: "e2e-admin"` and polls admin API with static bearer.
- Shell boot uses `S_USER` env carrier.
- Protected hub startup rejects these carriers; target is **RED for current auth model**.

### Home / Shell directory fold (collaborator observation lane)

**Path:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/.../✏️editor/🦀️.rs`

- `foldDirectoryEvents` is now `InteractiveJobClassification::Migrated` (prior `BatchOnlyPendingRewrite` RED is **repaired in source**).
- Shell folds accepted command receipts locally **and** live socket batches (`ShellHost` directory-command-receipt handler).
- Browser process gate: `os-hub:directory-home-browser-process-check process` chains `proveDirectoryEventPageV1Process` + static WASM Shell probe — **not** a full two-user WGPU/document edit journey.

---

## 4. Admin SPA / hub admin UI

### Session and transport (repaired vs 2026-09-03 audits)

**Path:** `/Users/ueli/Documents/semio/🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔑️AdminSession/🟦️.tsx`

- Fragment `#semio-admin={64-hex}` → `POST /__semio/admin/bootstrap` → HttpOnly relay cookie.
- No `sessionStorage` bearer; `AdminClient` uses `credentials: "same-origin"`.
- Fail-closed gates: probing / unauthorized / unreachable.

**Path:** `/Users/ueli/Documents/semio/🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔑️AdminSession/🟦️.tsx` (AdminClient)

- All mutations via `POST /admin/api/intents` with typed `AdminIntentV1` + terminal receipt validation.
- `createSpace`, `renameSpace`, `setVisibility`, `archiveSpace`, `deleteSpace`, `upsertSpaceMember`, `removeSpaceMember`, invite issue, etc.

### Spaces page

**Path:** `/Users/ueli/Documents/semio/🌎️hub/🔨️modules/🛡️admin/🧱️elements/🏛️SpacesPage/🟦️.tsx`

| Control | Wired | UI quality |
|---|---|---|
| Create space | `client.createSpace` | Dialog |
| Rename / visibility | intents | `window.prompt` |
| Archive / delete | `archiveSpace` / `deleteSpace` | `window.confirm` |
| Member upsert | `upsertSpaceMember` | No pending/error state; `.catch` absent |
| Invite create + copy | `createSpaceInvite` + `navigator.clipboard?.writeText` | Token rendered in `<code>`; copy fire-and-forget |

### Connections page (repaired)

**Path:** `/Users/ueli/Documents/semio/🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔗️ConnectionsPage/🟦️.tsx`

- Bounded REST poll of `/admin/api/connections` every 2s.
- **No** tokenless `DirectoryClient.stream()` (prior stale live-stream RED removed).
- Recorded-binding snapshot only; freshness indicator.

### Registered admin live journey

**Gate:** `bun nx run os-hub:admin-live-journey-check --skip-nx-cache`

**Proves:** protected relay + SQLite hub + Chromium SPA; EN/DE tabs; overview; **create-space intent**; bounded spaces read; rebuild operation poll/cancel.

**Does not prove:** member upsert/removal, delete-space terminal, invite copy, second collaborator observation, administration pane in Shell.

---

## 5. Invite copy capability — dual lanes

### Shell / SpaceAdministration lane (strong)

| Property | Implementation |
|---|---|
| Token custody | Worker retains `inviteToken` until exact clipboard success |
| Disclosure | One `directory-administration-capability` message per transfer epoch |
| Retry | Failed copy → `inviteCapabilityStatus: "failed"`, token retained |
| Success | Token erased; duplicate results rejected (`already-settled`, `mismatch`) |
| UI | Copy status in live region; button disabled while `copying` |
| Tests | Worker harness + `copyDirectoryInviteCapabilityV1` unit laws |

**Paths:** `backbone-worker.ts` (`requestDirectoryAdministrationCapability`, `settleDirectoryAdministrationCapability`); `ShellHost/🟦️.tsx`; `🧪️space-artifact-creation-owner/🟦️.ts`.

### Admin SPA lane (weak)

**Path:** `🏛️SpacesPage/🟦️.tsx` (MembersPanel)

- Invite token stored in React state and displayed in DOM (`data-slot="admin-space-invite-token"`).
- Copy: `void navigator.clipboard?.writeText(inviteToken)` — no await handling, no `aria-live` outcome.
- i18n key `admin.spaces.inviteCopied` exists in `📚️I18n/🟦️.tsx` but is **unused**.
- No one-shot erase after copy; token survives panel close until state reset.

**Classification:** Shell lane is source-green; admin lane is **RED for production invite hygiene**.

---

## 6. Evidence matrix (current tree)

| Layer | Collaboration | Space admin (author) | Space admin (hub operator) |
|---|---|---|---|
| Schema / types | Green | Green | Green |
| Hub REST + events | Process gate (`space-journey-process-check`) | Native laws (`space-administration-check`) | Backend + live create gate |
| Directory sockets | Process (global + scoped; removal on scoped) | N/A | N/A |
| Worker retained op | N/A | Source + harness | N/A |
| Shell pane | Not process-proven | Source + component tests | N/A |
| Admin SPA | Not in admin live journey | Partial (intents wired) | Live create only |
| Delete terminal | N/A | Receipt-only; **no GET 404 proof** | Intent wired; **no process proof** |
| Invite copy | Skipped in space journey | Shell strong / admin weak | Admin weak |
| Two-browser | `collab-e2e` RED | — | — |

---

## 7. Remaining blockers (paths + severity)

### P0 — delete missing-page terminal

| Blocker | Path(s) |
|---|---|
| Worker intentionally skips post-delete page fetch | `🧰️framework/.../🧵️backbone-worker.ts` (`completeDirectoryAdministrationDeletion`; no call to `loadDirectoryAdministrationPage`) |
| Harness asserts zero GET after delete | `🧰️framework/.../🧪️space-artifact-creation-owner/🟦️.ts` (~1915–1940) |
| No hub process step for delete + 404 administration read | `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (`proveDirectorySpaceJourneyV1Process` — no delete step); fixture `🌎️hub/📇️directory/🧫️fixtures/🚻️space-journey-v1/` |
| Hub `SpaceDeleted` event exists but no administration-route law after delete | `🌎️hub/📇️directory/🦀️.rs` (`DeleteSpace` → `SpaceDeleted`); gap in `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` space administration section |

### P0 — two-user journey (browser + skipped legs)

| Blocker | Path(s) |
|---|---|
| Space journey skips document open, presence, invite, edit, render | `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (~16926–16931) |
| Registered `collab-e2e` uses obsolete auth | `🧰️framework/.../🧑‍💻dev/.../📜️script.ts` (`COLLAB_E2E_ADMIN_TOKEN`, `S_USER`, `OS_HUB_ADMIN_TOKEN`) |
| Document open prerequisites still gated on trusted catalog / open-plan | skips reference open-plan status; related gates `native-openable-catalog-provider-check`, `browser-document-open-check`, etc. |
| Global directory socket stays open after member removal (scoped closes) | space journey step `scoped-socket-revoked` note in `📜️script.ts` (~16979–16982) |

### P1 — admin live hub (member operations + delete + invite copy)

| Blocker | Path(s) |
|---|---|
| Admin live journey stops at create-space | `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (`proveAdminLiveJourney` ~3056–3099); fixture `🌎️hub/📇️directory/🧫️fixtures/🚶️admin-live-journey-v1/` |
| Member upsert has no accessible pending/succeeded/denied UI | `🌎️hub/.../🏛️SpacesPage/🟦️.tsx` (~154–158) |
| Invite copy lacks Shell-grade capability transfer | `🏛️SpacesPage/🟦️.tsx` (~175–186); unused `admin.spaces.inviteCopied` in `📚️I18n/🟦️.tsx` |
| Destructive actions use `window.confirm` / `window.prompt` | `🏛️SpacesPage/🟦️.tsx` (~307–316, rename/visibility handlers) |

### P2 — presence under collaboration

| Blocker | Path(s) |
|---|---|
| Presence lease laws exist but not tied to space journey | `🌎️hub/🧫️fixtures/👥️presence-lease-v1/`; gates `presence-lease-*-check` |
| Space journey explicitly skips presence | `proveDirectorySpaceJourneyV1Process` skip `document-open-plan-and-presence` |

---

## 8. Highest-leverage next implementation lanes

Ordered by closure power for the ticket question (“collaborate in spaces **and** admins administrate end-to-end”):

### Lane A — Delete missing-page terminal (smallest admin-author closure)

1. Extend `🌎️hub/📇️directory/🧫️fixtures/🚻️space-journey-v1/🔣️.json` with steps: `author-deletes-space`, `deleted-space-page-denied` (expect empty 404 on `GET /directory/spaces/{id}`).
2. Add native law in `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`: after `DeleteSpace`, author and ex-member reads are non-enumerating denials.
3. Optionally extend worker harness: after `deleted` terminal, simulate GET 404 → ensure Shell stays `deleted` (not `denied` conflation) — **without** auto-fetch in production unless product requires refresh UX.
4. Register step in `proveDirectorySpaceJourneyV1Process`.

**Why first:** Closes the explicit receipt-only gap; reuses existing journey infrastructure; unblocks honest “admin deleted the space” claim.

### Lane B — Two-user journey completion (collaboration truth)

1. Land trusted open-plan/catalog prerequisite so `document-open-plan-and-presence` skip can be removed or narrowed.
2. Extend `space-journey-v1` fixture with invite create + redemption or author upsert path already present + **scoped document socket presence** once open-plan passes.
3. Retire or rewrite `@semio-tech/framework-os-dev:collab-e2e` to use FD3 bootstrap + broker relay (mirror `space-journey-process-check` identity model).
4. Add `directory-home-browser-process-check process` assertion for second identity observing folded membership via Shell static probe (build on existing gate).

**Why second:** Converts hub-process YELLOW into browser-observable collaboration.

### Lane C — Admin live hub member + delete + invite (operator journey)

1. Extend `🚶️admin-live-journey-v1` fixture: upsert second profile, member intent, EN/DE status selectors, post-delete bounded read.
2. Extend `proveAdminLiveJourney` with Playwright steps on `#admin-space-member-add-*`, delete confirm, invite copy outcome (`aria-live`).
3. Port admin invite copy to operation-bound transfer (mirror worker pattern) or hide token after one copy panel close.
4. Replace `window.confirm`/`prompt` with accessible dialogs + `role="status"` on intent receipts.

**Why third:** Closes operator-facing administration separately from in-space Shell pane.

---

## 9. Recommended gate order (after implementation)

```sh
# Prerequisites / current green-ish control plane
bun nx run os-hub:space-administration-native-check --skip-nx-cache
bun nx run os-hub:directory-ordered-publication-check --skip-nx-cache
bun nx run os-hub:admin-backend-check --skip-nx-cache

# Hub two-user collaboration (partial today; extend for delete + open-plan)
bun nx run os-hub:space-journey-process-check --skip-nx-cache

# Admin operator browser journey (create-only today)
bun nx run os-hub:admin-live-journey-check --skip-nx-cache

# Shell/worker administration harness (source/native)
bun nx run @semio-tech/framework-os:space-artifact-creation-owner --skip-nx-cache

# Future: replace obsolete target
# bun nx run @semio-tech/framework-os-dev:collab-e2e   # RED — do not treat as evidence
```

---

## 10. Explicit non-claims

- This audit did not run any command above; pass/fail is **not** asserted for the current tree state.
- PostgreSQL/Neo4j backends, production OIDC, full WGPU rendering, artifact mutation edit loops, and invite redemption ordering are out of scope unless cited as skips/blockers.
- Energy-model plugin `delete-space` mutations are unrelated plugin fixtures, not hub directory administration.

---

## 11. Related ticket audits (context only)

Prior reports in this ticket folder remain directionally useful but predate several repairs (ordered publication, admin relay session, `foldDirectoryEvents` migrated, ConnectionsPage REST poll, space-journey process driver):

- `📓️terra-space-administration-user-journey-current-audit.md`
- `📓️terra-two-user-space-admin-runtime-audit.md`
- `📓️terra-admin-backend-correctness-implementation-audit.md`
- `📓️terra-hub-admin-live-bilingual-audit.md`

This document supersedes their **current-tree** classification where noted above.
