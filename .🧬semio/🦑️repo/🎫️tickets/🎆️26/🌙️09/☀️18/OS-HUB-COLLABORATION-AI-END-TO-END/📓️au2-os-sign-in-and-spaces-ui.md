# AU2 — OS sign-in flow + end-user spaces surface

Slice AU2 of ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` (2026-09-19, Opus).
Gap closed: `📓️g1-goal-gap-audit.md` §3 ("zero hits for `login` anywhere under the os renderer";
"`🛂️SpaceAdministration` is an **admin** surface, not an end-user browse/switch flow"; "no dedicated
invite-composition or accept-invite screen") and `📓️g2-hub-depth-audit.md` §4 / §10 mismatch #4 /
§12 P0.1 (client-side session-mint consumption with nothing to call).

Everything below was **run**, with the captures named. Nothing is claimed from reading alone.

---

## 1. Baseline measured before writing anything

| claim | how measured | result |
|---|---|---|
| no sign-in UI in the os renderer | `grep -rn "login\|signIn\|sign-in"` over `📺️renderer` | only `ui.identity.signIn`/`signOut` **label keys** with no consumer (`🎯️targets/⚛️react/🟦️.tsx:1429-1431`) — dead keys, exactly G1's finding |
| no end-user spaces surface | read `🛂️SpaceAdministration/🟦️.tsx` (30 857 bytes) | it is per-space, gated on the hub's server-filled `capabilities`, mounted only while a retained admin operation is live (`🏛️ShellHost/🟦️.tsx:11016`) — no list, no switch, no create, no redeem |
| no `POST /auth/sessions` on the hub | `grep -c 'route("/auth/sessions"' 🌎️hub/🏗️bootstrap/🦀️.rs` | `0`, still `0` at 03:00 — AU1 has landed the `🌎️hub/🔐️auth` module and its schema but has **not yet mounted the route** |
| inherited AU2 work | `ls 🗑️generated | grep au2`, `git status` on the slice paths | none — this slice started from zero |

AU1's binding contract appeared at `📓️au1-hub-auth-sessions-and-rate-limit.md` §1 while this slice
was designing; everything here is built against **that** contract plus the hub's own authoritative
schema `🌎️hub/🔐️auth/🧬️schema/🔣️.json`, not against a guess.

---

## 2. Design

Four layers, so the contract is testable without React and the panes are testable without a hub.

**a. Pure contract, no React, no `fetch`.**
`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔐️sign-in/🟦️.ts` and
`…/📇️directory/🏘️spaces/🟦️.ts`. Every side effect enters through an injected port, exactly as the
existing `🪪️session-refresh/🟦️.ts` injects its `read`. Both ship a fixture + owned JSON schema
(`🔣️.json` + `🧬️.schema.json`), mirroring the `🪪️session-refresh` precedent.

**b. Headless lane.** `🧱️elements/🔗️HubConnection/🟦️.tsx` — `useHubConnection(port)`, analogous to
`🔗️AgentBridge`'s `useAgentBridge`. Owns the connection book, the session reducer, the space
projection, command submission, invite creation and redemption; also owns the `os.hub.*` EN/DE
bundle for all three elements.

**c. Two presentational panes.** `🧱️elements/🔐️HubSignIn/🟦️.tsx` and
`🧱️elements/🏘️SpaceBrowser/🟦️.tsx` — no transport, every effect a callback prop, so the same
element renders under a fake hub in tests, in Storybook, and under the hook in the shell.

**d. One composed surface + shell mount.** `🧱️elements/🔗️HubConnection/🏛️workspace/🟦️.tsx`
(`HubWorkspace`), mounted by `🏛️ShellHost` as an overlay on the route `/hub`.

### Decisions worth defending

- **The session capability never leaves the port closure.** `createHubConnectionFetchPortV1` captures
  the minted token in a closure and sends it as `Authorization: Bearer …`; it is never returned,
  never in React state, never in `localStorage`, never in a URL. The persisted-local-only
  `HubConnectionBookV1` carries **hub identity only** (`id, kind, label, lastUserId, origin`) — a
  test asserts the serialized book contains none of `token`/`password`/`sessionId` and not the token
  literal. This keeps the repo's existing stance (the browser-broker proof is memory-only in
  `🏪️store/👷️worker/🟦️.ts:660-830`) rather than weakening it for convenience.
- **A hub can never block local work.** `hubSessionAllowsLocalWorkV1` is `true` in every phase, by
  construction, and the reducer refuses to sign a live session out on a network failure
  (`failed` while `signed-in` records the error and keeps `userId`). The pane keeps a permanently
  visible `role="status"` "Working on this device only" line. The spaces surface has a `stale` phase
  that keeps the last projection rendered and openable — AGENTS.md "support short
  connection-shortages and not freeze the app", local-first over cloud-first.
- **CQRS, no CRUD.** Every mutation is built as a closed `DirectoryCommand` in the *canonical field
  order* (`kind, name, spaceKind, visibility` / `kind, spaceId, role, ttlSecs` / `kind, spaceId`) and
  posted to `/directory/commands`; redemption goes to the hub's own
  `POST /directory/invites/{token}/redeem`. The builders are checked against the **production**
  `parseDirectoryCommandV1`/`sealDirectoryCommandRequestV1`, which re-serialize and byte-compare —
  a wrong key order fails at seal time in this repo, not silently at the hub.
- **No user enumeration.** AU1 §1.1 emits one `invalid-credentials` class for unknown email, missing
  credential and wrong password. The UI has exactly one matching class and cannot distinguish.
- **`credential-sign-in-disabled` removes the form.** A hub with credential sign-in off (the default
  for a development hub) has no password path at all, so `hubSignInFormOfferedV1` returns `false` and
  the `<input type="password">` is structurally absent rather than offered and then refused.
- **No default language.** `hubSignInTextV1("fr")` throws `hub.sign-in.locale-unsupported`, matching
  `directorySessionAuthorityTextV1`. The `os.hub.*` bundle goes through
  `registerUiTranslationBundles`, whose type makes EN **and** DE a compile-time requirement for every
  key — a key added to one locale and not the other does not type-check.

---

## 3. Files added / changed

### Added

| file | what |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔐️sign-in/🟦️.ts` | sign-in contract: routes/bounds, EN+DE text, closed error classes, `session.v1.<32hex>.<64hex>` token parser, mint-body builder, local-only connection book, device-instance id, session reducer, `runHubSignInV1`/`runHubSignOutV1` |
| `…/📇️directory/🔐️sign-in/🔣️.json` + `🧬️.schema.json` | fixture + owned schema (status table, retry-after table, credential/token/origin vectors, EN+DE text) |
| `…/📇️directory/🏘️spaces/🟦️.ts` | space row projection + ordering + filter, write/invite authority, member-presence join, the three command builders, invite token/link parsing, redemption error classes |
| `…/📇️directory/🏘️spaces/🔣️.json` + `🧬️.schema.json` | fixture + owned schema (canonical command field order, invite token vectors, redemption status table) |
| `…/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️HubConnection/🟦️.tsx` | `os.hub.*` EN+DE bundle, `HubConnectionPortV1`, `useHubConnection`, `createHubConnectionFetchPortV1` |
| `…/🧱️elements/🔗️HubConnection/🏛️workspace/🟦️.tsx` | `HubWorkspace` — the composed surface the shell mounts (own leaf, so the label bundle is not in an import cycle) |
| `…/🧱️elements/🔐️HubSignIn/🟦️.tsx` | the sign-in pane |
| `…/🧱️elements/🔐️HubSignIn/🧪️tests/🧩️component/🟦️.tsx` | 31 laws |
| `…/🧱️elements/🔐️HubSignIn/📖️stories/🧪️.story.tsx` | 10 stories |
| `…/🧱️elements/🏘️SpaceBrowser/🟦️.tsx` | the spaces surface |
| `…/🧱️elements/🏘️SpaceBrowser/🧪️tests/🧩️component/🟦️.tsx` | 24 laws |
| `…/🧱️elements/🏘️SpaceBrowser/📖️stories/🧪️.story.tsx` | 8 stories |

### Changed (append-only where the file is shared with a peer)

| file:line | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🟦️.ts:4880-4972` | new `🔖️HubSignIn` export region (both modules' public API) + `directoryCommandRequestJson` re-export. Append-only; `git diff` is 93 pure insertions |
| `…/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx:1405-1413` | new `🔖️HubConnection` export region so stories and the shell import from `@semio-tech/framework-renderer-react` |
| `…/🧱️elements/🏛️ShellHost/🟦️.tsx:388-389` | import `HubWorkspace` + `createHubConnectionFetchPortV1` |
| `…/🧱️elements/🏛️ShellHost/🟦️.tsx:2569-2593` | `hubWorkspaceOpen` state + the one `hubConnectionPort` built per shell (real `fetch`, real `localStorage`, real clipboard, real command sealing) |
| `…/🧱️elements/🏛️ShellHost/🟦️.tsx:~6017` | `/hub` route branch inside `applyShellUri` — opens the overlay, returns before any session teardown |
| `…/🧱️elements/🏛️ShellHost/🟦️.tsx:~11016` | the overlay render block, closing back to `/spaces/{open}` or `/` |
| `…/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts:84,87` | both new suites registered in `engineTestSuites` (a co-located suite in no include list is a gate that measures nothing) |
| `…/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts` | `hubAuthContractOracle` (cross-twin drift gate against `🌎️hub/🔐️auth`) + `HubSignInSpacesCheckScript`, registered as `hub-sign-in-spaces-check` |
| `…/🎯️targets/⚛️react/📦️packages/🟦️typescript/📋️project.json` | nx target `hub-sign-in-spaces-check` |
| `.vscode/🧩️launch.seed.jsonc` + `.vscode/launch.json` | row `📦️check⚛️react🔐️hub-sign-in`, group `4_build`, order `206.1475`, inserted identically in both so the generated/seed pair stays byte-consistent |

Ticket-folder scratch: `🔣️au2-tsconfig.json` (scoped typecheck program).

---

## 4. Tests — real counts

Command (also the new nx target `@semio-tech/framework-renderer-react:hub-sign-in-spaces-check`):

```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript
bun ./📜️script.ts hub-sign-in-spaces-check
```

Capture `🗑️generated/au2-check.txt`, exit 0:

```
hub-auth-contract-oracle: checks=15 clean
 Test Files  2 passed (2)
      Tests  55 passed (55)
```

| suite | tests | covers |
|---|---:|---|
| `🔐️HubSignIn/🧪️tests/🧩️component` | **31** | fixture vs. Ajv + `fast-deep-equal`; EN/DE key parity and the `fr` refusal; AU1's whole status table; `retry-after` header **and** `retryAfterMs` body; origin normalization (10 vectors incl. path/credential/query/fragment/`ftp` refusals); token shape (5 vectors); credential bounds (5 vectors + both byte edges); mint body field order, lowercasing and the 1024-byte cap; connection book persists exactly 5 identity fields and no secret, repairs 5 corrupt forms, caps at 16, survives a throwing store; reducer laws (offline never signs out, refusal never clears a live session, submit→minted→expired→re-auth, hub switch resets identity but keeps connectivity); `runHubSignInV1` against a fake hub for 200/400/401/403/429×2/503/thrown/garbled-200/abort; sign-out on 204 and on 503; pane: labelled autofillable form, disabled-until-valid submit, one alert region per denial, rate-limit lockout, password form removed on `credential-sign-in-disabled`, busy+cancel pair, local-only line in every phase, sign-out only when signed in, full German render with no English leaking, inline invalid-origin; hook: sign in → user recorded and token absent from storage, rate-limited attempt, cancel a slow attempt and the late response never installs, add/switch/forget a remote hub, sign out even when the hub refuses revocation |
| `🏘️SpaceBrowser/🧪️tests/🧩️component` | **24** | fixture vs. Ajv; ordering (author→member→public, newest first, id tie-break); filter; write/invite authority; **every command builder round-tripped through the production `parseDirectoryCommandV1`/`sealDirectoryCommandRequestV1`**; invite token vectors (7) and link rendering; redemption status table (9); member-presence join and email fallback; surface: semantic list + single `aria-current`, search-driven switching, live phase region with stale rows still usable, per-member presence and a bar that only shows connected members, create-space intent + hidden when signed out, invite creation only where I author, one-shot link with copy/discard, all six redemption failure texts, redeem intent + busy control, full German render, **no fixed-pixel width on any control/section** (the phone-width law); hook: mount load, create-space sealed canonically then reload, create-invite receipt → one-shot link → clipboard copy, clipboard refusal reported honestly, redeem a pasted link by its bare capability + a 409 → `already-member`, hub stops answering → `stale` with rows still rendered |

**Cross-implementation oracle (15 checks), not a mock:** `hubAuthContractOracle` reads the hub's own
`🌎️hub/🔐️auth/🧬️schema/🔣️.json` and `🌎️hub/🔐️auth/🦀️.rs` and asserts that AU1's
`CredentialSignInRequestV1.required` equals this slice's request field order, that
`SIGN_IN_REQUEST_MAX_BYTES = 1024` still holds in the Rust, that every credential vector's admission
verdict agrees with the hub schema, that every token vector's verdict agrees with the hub's
`SessionCapabilityV1` pattern, and that the Rust `AuthErrorCodeV1::status()` arms still map
400/401/403/429/503. If AU1 changes the route, **this gate goes red** instead of the app.

Typecheck (deliverable 5 — T4's scoped program): `bunx tsc --noEmit -p 🔣️au2-tsconfig.json`,
captures `🗑️generated/au2-typecheck-{1,2,3}.txt`. **0 diagnostics in any file this slice owns**, and
0 at the lines it added to `🏛️ShellHost/🟦️.tsx` (that file's 19 remaining diagnostics and
`💻️os/🟦️.ts`'s 2 are pre-existing — the latter is a duplicate `DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES`
import at lines 25 and 4260 that is present in `git show HEAD:` too; both belong to T4).

---

## 5. Runtime evidence

- **Both suites executed**, 55/55, exit 0, three times (before the hub-schema alignment, after it,
  and after the `ShellHost` wiring). `🗑️generated/au2-check.txt`, via the registered script entry
  `bun ./📜️script.ts hub-sign-in-spaces-check` — the exact command the nx target runs.
- **The nx wrapper could not be captured green under the current fleet load.** Two attempts
  (`🗑️generated/au2-nx-check.txt`) both end in `NX  The daemon timed out while processing
  HASH_TASKS` after ~10 min of project-graph work, before the target's own command starts. That is
  the nx daemon under a saturated machine, not this target: the target is a plain
  `nx:run-commands` whose `command`/`cwd` are copied field-for-field from the adjacent
  `directory-home-bootstrap-check` row, and the command it would run is the one measured above.
  Re-run it when the fleet is idle.
- **The panes were exercised in a real DOM** (jsdom, `@semio-tech/ui-react/test`): forms submitted,
  buttons pressed, `setUiLocale("de")` applied and asserted, `aria-busy`/`aria-current`/`role="alert"`
  read back off the rendered tree.
- **Accessibility is asserted, not claimed**: every input has a real `<label for>` (checked by id),
  `autocomplete="username"`/`current-password`, `aria-invalid` + `aria-describedby` on invalid
  fields, one `role="alert"` region per denial that takes focus, `role="status" aria-live="polite"`
  for phase and local-only lines, `aria-current="true"` on exactly one space row, and accessible
  names on every icon button (a `Button` renders its icon as text, so the computed name needed an
  explicit `aria-label` — found by a red test, not by inspection).
- **Phone width is asserted by a law**, not by eyeballing: no control, section, list or aside carries
  a `w-[Npx]`/`min-w-[Npx]`/`max-w-[Npx]` class or an inline pixel width; layout is one column with
  `sm:`/`md:` opt-ins and `min-w-0` on every flex child. Both stories also ship a `Phone` story at
  `mobile1`.

**What was NOT verified at runtime, and why.** No real sign-in against a booted hub:
`grep -c 'route("/auth/sessions"' 🌎️hub/🏗️bootstrap/🦀️.rs` returned `0` at 02:20 and again at 03:00.
AU1 has landed `🌎️hub/🔐️auth/{🦀️.rs,🧬️schema/}` (decoder, policy, password, rate-limit) but has not
mounted the route, and `📓️au1-hub-auth-sessions-and-rate-limit.md` §3–§7 are still `_to be filled_`.
There is nothing to sign in *to* yet. The strongest available substitute — asserting this slice's
wire against the hub's own schema and Rust status table — is the 15-check oracle above, and it runs
on every invocation of the gate. Booting the hub per `📓️h1-hub-build-and-boot.md` §6.2 was therefore
not done: it would have proved only that `POST /auth/sessions` 404s.

---

## 6. Honest gaps

1. **No live sign-in.** See §5. The moment AU1 mounts the route, the one command to run is
   `bun nx run @semio-tech/framework-renderer-react:hub-sign-in-spaces-check` (drift gate) followed
   by a boot per `📓️h1-hub-build-and-boot.md` §6.2 with `OS_HUB_CREDENTIAL_SIGN_IN=1` and a
   `POST /auth/sessions` curl. The fetch adapter, not the panes, is the only untested-at-runtime
   layer: `createHubConnectionFetchPortV1` is covered by type-checking and by the fact that the
   hook's every consumer is tested against the same interface, but no test drives it over real HTTP.
2. **`HubWorkspace` is reachable only by URL (`/hub`).** It is not in the command palette, has no
   keybinding, and no chrome affordance points at it — `buildOsCommands` lives in `ShellHelpers`
   and adding a verb there would have collided with peers mid-flight. One command entry is the
   remaining work to make it discoverable.
3. **Members and presence are wired but not fed.** `SpaceBrowser` renders a real roster with live
   presence and the domain-neutral `PresenceBar`, and `spaceMemberPresenceV1` joins a member list
   with connected user ids — but `ShellHost` currently passes `members={[]}`. The roster source is
   the hub's per-space administration page (`DirectorySpaceAdministrationMemberRowV1`) plus the
   presence lane in `🏪️store/👷️worker`; joining those two into the workspace was out of this slice's
   reach without a second retained worker operation. Everything below the prop is tested.
4. **`expiresAtMs` is always `null` after a mint.** AU1 §1.1 deliberately keeps the expiry out of the
   mint response and puts it in `GET /auth/sessions/me`; the hook does not yet make that second call,
   so `hubSessionNeedsReauthenticationV1` currently only fires on an explicit `expired` event (a
   401 from a later call), not on a locally known deadline. The reducer and the predicate already
   handle the deadline — only the follow-up read is missing.
5. **Sign-out is local-first by design.** A hub that refuses `DELETE /auth/sessions/me` still drops
   the in-memory capability, so the human is never trapped. That means a refused revocation leaves a
   server-side session alive until it expires. Stated here because it is a deliberate trade, not an
   oversight.
6. **The wgpu renderer has none of this.** This is React-only, which widens exactly the
   React/wgpu chrome drift `📓️g1-goal-gap-audit.md` §3 already flags for M2's chat panel.
7. **One pre-existing os-product test failure observed, not mine.**
   `@semio-tech/framework-os:test` → `admits only closed semantically valid directory
   administration requests` fails because the `announce-document` descriptor schema now requires
   `artifactId` while the test's fixture still sends `documentId` — a peer's `documentId`→`artifactId`
   rename (same one H1 §1 describes). This slice's change to `💻️os/🟦️.ts` is 93 append-only export
   lines and touches no schema. Flagged for whoever owns that rename; the other 224 tests in that
   file pass.
8. **Storybook stories are authored and type-checked but not rendered.** They fall inside the os
   renderer scope's glob (`.storybook/📖️stories/🧭️coordination/🟦️.ts:118`), and a storybook build
   was not run — it is a whole-workspace build and the fleet was already saturated.
