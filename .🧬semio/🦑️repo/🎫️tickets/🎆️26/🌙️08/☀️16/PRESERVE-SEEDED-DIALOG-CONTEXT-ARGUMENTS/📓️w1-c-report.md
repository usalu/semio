# W1-C report — TS identity facet + directory client + backbone worker

## Changed files

**New (identity config facet, self-contained — see "Design decision" below):**
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🪪️sign-in/🟦️component.ts` — `Identity`,
  `IDENTITY_CONFIG_SCHEMA`, `IdentityConfigMutation`, `applyIdentityConfigMutation`,
  `diffIdentityConfigMutation`, `inverseIdentityConfigMutation` (dispatch parent, mirrors
  `🛡️change-merge-policy`'s self-containment pattern).
- `…/🪪️sign-in/🦠️mutation/🟦️component.ts`, `…/🔺️diff/🟦️component.ts`, `…/↩️inverse/🟦️component.ts`
- `…/🚪️sign-out/🦠️mutation/🟦️component.ts`, `…/🔺️diff/🟦️component.ts`, `…/↩️inverse/🟦️component.ts`

**Edited (in-lease):**
- `🧰️framework/🛍️products/💻️os/🟦️component.ts` — `PersistenceBinding.hub` gained `surface?: string`;
  `BackboneWorkerRequest`/`BackboneWorkerResponse` gained `directory-open`/`directory-command`/
  `directory-close` and `directory-message`/`directory-command-result`/`directory-status`; new
  `//#region 🔖️HubBinding` appended inside the existing `//#region 🔖️Directory` (lane 0-A's) holding
  `DirectoryClient`, `DirectoryHttpError`, `HUB_RECONNECT_MIN_MS`/`MAX_MS` (moved here from
  `🟦️backbone-worker.ts`, single source of truth), and in-source tests (fake-WS). Never touched
  `AppChannelCodec`/`AppChannelClient`/`PublicApi`.
- `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts` — `🔖️ConfigLane`: `IDENTITY_CONFIG_SCHEMA`,
  `identityActorConfig`, `foldIdentityEvent`. `🔖️Hub`: `connectHub` appends `?surface=` when
  `binding.surface` is set. New `//#region 🔖️Directory`: offline command queue (bounded 200, drop-
  oldest), `openDirectory`/`closeDirectory`/`submitDirectoryCommand`/`flushDirectoryQueue`, wired
  into `handleTsRequest`. `🔖️Constants`: removed the local `HUB_RECONNECT_MIN_MS`/`MAX_MS` (now
  imported from `component.ts`). New tests: identity fold vectors + directory command queueing.

## Commands run + results (real tail, both files scanned by this package's `🧪️vitest.config.ts`)

`bun nx run @semio-tech/framework-os:test` **hung** (no output after 90s+, backgrounded, machine
shared/saturated per `📓️w0-a-report.md`'s precedent) — used the documented fallback:

```
cd 🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript
bunx vitest run -c 🧪️vitest.config.ts
```
```
 Test Files  2 failed | 2 passed (4)
      Tests  2 failed | 334 passed (336)
```
The 2 failures are the **same pre-existing** wasm-artifact failure `📓️w0-a-report.md` already
recorded ("2 pre-existing wasm-artifact failures", `matches the Rust plan_workflow across shared
fixtures decoded via wasm` — missing `📦️host/📦️packages/🦀️rust/pkg/semio_framework_os.js`, not
built in this environment), counted twice because the config's `include`/`includeSource` both list
the same 2 files (pre-existing quirk, not introduced here). Baseline before my changes: 320/322
passing (same 2 failures); after: **334/336**, i.e. every new test (14) passes, zero regressions.
Full log: `🧪️1-c-vitest-direct.txt`. Verbose listing of just the new suites:
`🧪️1-c-vitest-verbose-directory-identity.txt` — all of `DirectoryClient.stream` (3), `identity config
facet` (3), `backbone-worker directory lane` (1) show `✓`.

**No Rust touched** → `cargo check -p semio-framework-os-kernel` not run (brief: conditional on
touching Rust in the config schema; I didn't — see design decision below).

## Design decision — identity facet is self-contained, `🎚️config`'s shared schema files untouched

My lease's literal wording ("plus the minimal additions to `🎚️config/🧬️schema/{component.json,
component.rs,component.ts,…}`") conflicts with `📋️ownership-and-handoffs.md` §A, which lists
`💻️os/🎚️config/**` existing triads as leased to **me** but marks `💻️os/🎚️config` opening-preferences
triads (the actual `component.json`/`component.rs`/`component.ts` + `set-default-app`/
`clear-default-app`) as owned by the concurrent `ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` ticket
(confirmed via that ticket's own `MUTATION-OUTCOMES` peer's `ownership-and-handoffs.md` line 33). I
resolved this by following the **exact precedent already in the tree**: `🧬️mutations/
🛡️change-merge-policy/🦀️component.rs` (added by `MUTATION-OUTCOMES` lane 2-D) folds its schema +
dispatch enum into its own leaf file specifically to avoid the same shared parent — its header doc
says so explicitly. I did the TS-equivalent: `Identity`/`IDENTITY_CONFIG_SCHEMA`/
`IdentityConfigMutation`/apply/diff/inverse all live in `🪪️sign-in/🟦️component.ts` (the dir-level
file, sibling to its own triad subdirs), `🚪️sign-out` imports `Identity`/`IdentityConfigMutation` as
types from it (no runtime cycle — verified: sign-out's files only `import type` back). **Never
touched** `🎚️config/🧬️schema/{🔣️component.json,🦀️component.rs,🟦️component.ts}` or `🧬️mutations/
🦀️component.rs`. No JSON schema file added either (matches `🛡️change-merge-policy`'s precedent —
Rust/TS-only facet, no companion `component.json`).

**No Rust twin authored by me** — while writing this I discovered a concurrent session (lane 1-D,
confirmed by its own header docs) already landed `🔨️modules/📇️directory/🪪️identity/🦀️component.rs`
and `🔨️modules/📇️directory/🔌️client/🦀️component.rs`: a native-shell (`mint_or_restore`) helper +
Rust `DirectoryClient` twin, explicitly scoped OUTSIDE `🎚️config/**` because "🎚️config/** is
peer-leased to 1-C". No file conflict. **Naming mismatch worth flagging**: 1-D's docstring assumes
my facet lives at `💻️os/🎚️config/🪪️identity/` (a top-level sibling dir) — it actually lives at
`💻️os/🎚️config/🧬️schema/🧬️mutations/🪪️sign-in/🟦️component.ts` per my lease's literal, explicit path.
This is prose-only (1-D never imports my path, they independently re-declare `Identity`), so nothing
is broken, but lane 2-C/2-D should import from the real path above, not the one in 1-D's comment.
1-D's `SessionView`/`SessionMintResponse` wire shapes (`{userId,email,displayName,expiresAt}` /
`{token,user_id}` snake_case) and `HUB_RECONNECT_MIN_MS`/`MAX_MS` (500/30_000) match what I
independently built into `DirectoryClient` — cross-checked field-for-field, no drift.

## Task items — status

1. **Identity facet** — done (self-contained, see above). `identityActorConfig(actor, dataDir?)`
   binds `[{kind:"folder", path: "${dataDir}/os"}]` when `dataDir` given, else `[]` (mirrors opening
   preferences' local-only pattern for callers with no `S_DATA_DIR`, e.g. a bare browser preview).
   `foldIdentityEvent` mirrors `foldOpeningPreferencesEvent`'s "last envelope wins" whole-record
   fold. Sign-in/sign-out triads mirror `SetDefaultApp`/`ClearDefaultApp`'s inverse laws exactly
   (sign-in's inverse restores the prior session or signs out; sign-out's inverse restores the prior
   session or no-ops).
2. **`DirectoryClient`** — done, in `🟦️component.ts`'s new `🔖️HubBinding` region. All 7 methods +
   `DirectoryHttpError` (carries `status` so callers can distinguish "hub rejected" from "hub
   unreachable"). Imports `DirectoryCommand`/`DirectoryEvent`/`DirectoryStreamMessage`/`SpaceView`/
   `MemberView`/`DocumentView`/`InviteView` from lane 0-A's module — none redeclared. `fetch`/
   `WebSocket` only. `stream()` never throws into the caller (malformed frames dropped in a `try`;
   verified by a dedicated test) and resumes reconnects from the highest `seq`/`headSeq` actually
   observed, not the caller's original `since`.
3. **`PersistenceBinding.hub.surface`** — done; `connectHub` appends `?surface=<encoded>` only when
   set.
4. **Backbone worker directory lane** — done: the three request kinds, three response kinds, bounded
   (200, drop-oldest) in-memory offline queue, flush attempted on every stream message (any message
   is evidence of a live connection) and after every successful `submitDirectoryCommand`, pending
   count surfaced via `directory-status`. A definitive HTTP rejection (`DirectoryHttpError.status`
   present) is surfaced immediately and dropped, never queued forever; only a transport/network
   failure (no `status`) queues.
5. **Tests** — done, all in-source (this package's `🧪️vitest.config.ts` only scans `component.ts`/
   `backbone-worker.ts`): identity fold vectors (sign-in → sign-out → sign-in, plus all four inverse
   cases), `identityActorConfig` with/without `dataDir`; `DirectoryClient.stream` against a
   hand-rolled fake `WebSocket` (no existing fake-ws pattern was actually in the tree — the brief's
   pointer to one in `🟦️backbone-worker.ts` didn't pan out, checked via repo-wide grep for
   `WebSocket`/`FakeWebSocket`/`stubGlobal`, zero hits before this change) — replay-then-live
   no-gap/no-duplicate, reconnect resumes from last seq with doubling backoff, malformed-frame
   safety; directory command queueing while offline + flush on the next live signal, in
   `backbone-worker.ts`.

## What is NOT done / out of scope for this lane

- No Rust identity/config-store wiring (lane 1-D's territory, see above).
- No shell-side caller (React `os.directory.*` action funnel, auto-bind on `openDocument`) — lane
  2-C/2-D.
- `bun nx run @semio-tech/framework-os:test` itself never completed in this session (hung); only the
  direct `vitest run` result above is a confirmed pass — flagging per "never claim a test passes that
  you did not run."

## sharedFileRequests

None — no foreign-leased file was touched. The design-decision section above documents a
self-containment choice made to *avoid* a shared-file conflict, not a request to touch one.
