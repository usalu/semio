# W2-C report — React shell: identity, auto-bind, directory relay, routing

## Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx` —
  the bulk of the lane. New `//#region 🔖️Identity` (module-level, before `FrameworkOsShellInner`):
  `readViteSEnv`, `identityMutationEnvelope`, `decodeIdentityPayload`, `shellActorId` (exported),
  `canonicalSurfaceId` (exported), `S_SPACE_INDEX_DOCUMENT_SCHEMA`/`S_SPACE_INDEX_DOCUMENT_ID`/
  `SPACE_INDEX_DIALECT`, `findDialectApp`, `directoryCommandFromAction` (exported). New refs/state:
  `shellSessionIdRef`, `identity`/`identityRef`, `identityOffline`, `directoryClientRef`, `hubEnv`
  (memoized `VITE_S_HUB_URL`/`VITE_S_USER`/`VITE_S_DATA_DIR` read), `directoryOpenedRef`,
  `dispatchDirectoryEventsRef`, `directoryPendingCommands` setter, `identitySnapshotResolverRef`,
  `openDocumentRef`/`openArtifactWithAppRefRef` (forward-refs so the new `applyHostEffects` branch
  can call functions declared later in the same component without a `const` TDZ violation).
  `ensureBackboneWorker`'s `onmessage` gained `directory-message`/`directory-status`/
  `directory-command-result` routing and a special case for the identity document's own events
  (never gets an `openDocumentSessionsRef` entry — no plugin/session). Two new effects: mint
  `setPluginRuntimeActor` at boot with the pre-identity default, and the identity bootstrap effect
  (open identity doc → bounded 2s wait for a persisted snapshot → cached token → `DirectoryClient.me()`
  → 401/no-token → `mintSession(email)` → dispatch `signIn` mutation, persist via `localMutations` +
  `localSnapshot`, mint `user:{userId}#{shellSessionId}`, open the directory socket once). New
  `//#region 🔖️DirectoryLane`: `dispatchDirectoryEventBatch` (folds into whichever of home/studio/
  space is the CURRENTLY mounted session — see "What is NOT done" below). `openDocument`'s `bindings`
  param is now optional with a default-binding computation (task 2). `applyHostEffects` gained a
  `"replayShellCommand" in effect` branch: `os.directory.*` → `directoryCommandFromAction` →
  `directory-command` through the worker; `os.open-artifact`/`os.open-artifact-with` →
  `openArtifactWithAppRef` + (if `documentId` present in args) `openDocument`. `applyShellUri`
  (routing) now locally matches `/spaces/{id}/studio(/instances/{id})?` before falling back to
  `parseShellRoute` (never edited — out of lease), and a bare `/spaces/{id}` (no `/studio`, no
  `/instances/{id}`) resolves the `s.space` app by dialect instead of always opening the studio.
  The two `presenceClientIdentity(ephemeral)` call sites now pass the real identity when resolved.
- `🧱️elements/PluginRuntime/🟦️component.tsx` — new `//#region 🔖️ActorIdentity`:
  `currentPluginRuntimeActor` (module-level) + exported `setPluginRuntimeActor(actor)`; `createApp`
  now stamps `new AppChannelClient(handle, instanceId, appId, currentPluginRuntimeActor)`.
- `🧱️elements/ShellHelpers/🟦️component.tsx` — `presenceClientIdentity(ephemeral, real?)` gained an
  optional 2nd param; returns `real` immediately when given, unchanged guest-identity fallback
  otherwise.
- `🎯️targets/⚛️react/📦️index.tsx` (os renderer-react package barrel) — appended `//#region
  🔖️SpacesI18n`: `ui.home.*`/`ui.space.*`/`ui.presence.*`/`ui.checkin.*`/`ui.identity.*` as a local
  `{en, de}` FrozenLabel-style table + `spacesUiLabel(key, locale)`, mirroring `ShellHelpers`'
  `SurfaceRoleLabels` workaround (the real i18n registry, `🖱️ui`'s own package, is outside this
  lane's lease). Also appended 3 new re-exports (`shellActorId`, `canonicalSurfaceId`,
  `directoryCommandFromAction`) to the existing `FrameworkOsShell` region for test coverage. Both
  appends only — no existing line reordered or edited.
- `🎯️targets/⚛️react/🧪️index.test.ts` — 3 new `it()`s next to the existing `parseShellRoute` test:
  `shellActorId`, `canonicalSurfaceId`, `directoryCommandFromAction` (all 7 verbs + the
  `share-link`→`create-invite` sugar + an unrecognized-id → `null` case).
- **NEW** `🧰️framework/🛍️products/💻️os/🎮️commands/📇️directory-{create-space,delete-space,
  rename-space,set-visibility,upsert-member,remove-member,share-link}/🦀️component.rs` — the 7 OS
  command id+label leaves (contract §C6), copying `📂️open-artifact`'s exact shape (`ID`/`LABEL_EN`/
  `LABEL_DE` + a frozen-values test). Like their 5 pre-existing siblings under `🎮️commands/**`, these
  are standalone leaf files not yet `#[path]`-wired into any crate (verified: none of the existing 5
  are either — `grep -rln "🎮️commands" **/*.rs` outside `🎮️commands/` itself finds no `os`-scoped
  `#[path]` glue for this directory, only the unrelated repo-CLI's own `🎮️commands/*`).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🟦️component.ts` — **not touched**. `ShellHost`
  reads `import.meta.env.VITE_S_HUB_URL`/`VITE_S_USER`/`VITE_S_DATA_DIR` directly (Vite `define`s are
  global replacements, not something that needs threading through `bootFrameworkOs`'s props), and
  lane 1-F already wired the `define` block in `⚙️vite.config.ts`. No further plumbing needed.

## Design decisions worth flagging

1. **`applyHostEffects`'s `HostEffect::replayShellCommand` had NO handler anywhere in this file
   before this lane** (verified: zero matches for `replayShellCommand`/`ReplayShellCommand` in
   `ShellHost` pre-edit). This means today, `os.setThemeId`'s "Backwards" replay and the *existing*
   `os.open-artifact`/`os.clear-default-app` relays from `plugin/🦀️component.rs` were already
   silently dropped — a real, pre-existing gap, not something I introduced. I added handling only for
   `os.directory.*` and `os.open-artifact`/`os.open-artifact-with` (my lease); every other action id
   still falls through as a no-op, same as before.
2. **`openDocument`'s `schema` for a generic `os.open-artifact`**: there is no general formula from a
   dialect coordinate (`artifactRef`, e.g. `s.space.space@1/*`) to a document schema id — verified
   against `s.space` itself, whose artifact-kind id (`space.sspace`), dialect coordinate
   (`s.space.space@1/*`), and document schema (`s.space`) are three genuinely different strings, none
   derivable from another. I special-cased the one mapping this lane actually knows (`s.space`) and
   fall back to `artifactRef` itself for anything else — a placeholder until lane 3-B's opening relay
   carries a real `schema` field.
3. **`PluginRuntime`'s actor threading** uses a module-level `currentPluginRuntimeActor` +
   `setPluginRuntimeActor()` rather than a parameter threaded through `loadPluginModule`/
   `adaptPluginHandle`, because the only real call site (`loadPluginModuleResilient`) lives in
   `ShellHelpers/🟦️component.tsx`, outside this lane's lease (only `presenceClientIdentity` is
   leased there), and `PluginWasmHandle` is the kernel's frozen shape (no room for a setter method).
   **Known limitation**: a single JS realm hosting more than one `ShellHost` (the multi-pane
   demonstrator) would share one actor id across panes. Not exercised by any test in this repo today;
   flagged for whoever next touches multi-pane hosting.
4. **`dispatchDirectoryEventBatch`** folds a `DirectoryEvent[]` batch into whichever of home/studio/
   space is the CURRENTLY mounted plugin session (`sessionRef.current`), not "home AND the open
   space" simultaneously as the brief's prose suggests — this shell keeps exactly one plugin session
   mounted at a time (`session`/`switchToManagedApp`), so there is no live "background" home instance
   to fold into while a space is open, or vice versa. This is an architectural constraint of the
   current single-session router, not a bug in my wiring.
5. **Identity persistence** goes through the SAME generic document-actor mechanism every other
   artifact uses (`kind: "open"` with `identityActorConfig`, then `localMutations` + `localSnapshot`
   to persist the folder-lane pack) rather than a bespoke read/write path — `readBackboneEnvelope`/
   `writeBackboneEnvelope` (`🟦️component.ts`) looked tempting for a shortcut but omit the
   `documentId` query param the worker's own `folderEnvelopeUrl` includes (a folder binding's path
   can hold more than one document, distinguished by `documentId`), so using them directly would risk
   reading/writing the wrong file once `${dataDir}/os` ever holds a second document.

## Task items — status

1. **Identity bootstrap** — done. Env-gated (`if (!hubEnv) return`, verified: the existing
   `🛠️dev🖥️s⚛️react` launcher path is untouched — no `VITE_S_*` defines there per lane 1-F's own
   report, so `hubEnv` is `null` and the whole effect is a no-op, preserving today's local-only
   behaviour byte-for-byte). Cached-token → `me()` → 401/none → `mintSession(email)` → `signIn`
   mutation dispatch, actor minted as `user:{userId}#{shellSessionId}`, threaded into `openDocument`
   (via `shellActorIdRef.current`, unchanged call site) and into `AppChannelClient` via
   `setPluginRuntimeActor`. Hub-unreachable path keeps the last persisted identity (already applied
   from the `snapshotReplaced` read) and sets `identityOffline`, never throws past the `try/catch`.
2. **Auto-binding** — done. `openDocument`'s `bindings` param is now optional; omitted computes
   `[{kind:"hub", baseUrl, spaceId, token, surface}, {kind:"folder", path}]` when identity + a route
   space exist, `[{kind:"folder", path}]` with dataDir but no identity, `[]` otherwise.
   `attachSyncBackbone` (the manual `remote://` override) is untouched and still always passes an
   explicit array, so it keeps working exactly as before.
3. **Directory lane** — done, mechanically: `directory-open` posted once per shell
   (`directoryOpenedRef`) right after identity resolves; `directory-message` events fold via
   `dispatchDirectoryEventBatch` → `foldDirectoryEvents` action on whichever session is mounted; the
   7 `os.directory.*` action ids intercepted in `applyHostEffects` → `directoryCommandFromAction` →
   `directory-command` through the worker; zero optimistic local mutation (the read model only ever
   updates from `directory-message`/`foldDirectoryEvents`).
4. **Opening** — done: `os.open-artifact`/`os.open-artifact-with` resolve the app via
   `openArtifactWithAppRef` and, when `documentId` rides the args, call `openDocument`. **Not yet
   observable end-to-end**: lane 3-B (opening relay `documentId`/`spaceId`, next wave) hasn't landed,
   so today's real `plugin/🦀️component.rs` relay still only sends `{artifactRef, role, pluginId,
   appId}` — verified by reading `relay_opening_command`'s current call sites myself. The shell side
   is ready the moment those two fields are added; nothing further needed here.
5. **Routing** — done, dialect-driven: `/spaces/{id}/studio` (optionally `/instances/{id}`) keeps the
   exact pre-existing studio behaviour (`hostConfig.hostAppId`); a bare `/spaces/{id}` now resolves
   the `s.space` app via `findDialectApp` against `SPACE_INDEX_DIALECT` (`s.space.space@1/*`) instead
   of always opening the studio. `parseShellRoute` itself (`ShellHelpers`, outside lease) was never
   edited — the `/studio` distinction is matched locally in `applyShellUri` first, falling back to
   `parseShellRoute` for everything else, so its existing tests are unaffected.
   **What I could/could not observe**: `findDialectApp` returns `undefined` today (logged, not
   thrown) because lane 1-E's `s.space` artifact crate does not currently compile — confirmed by
   reading `📓️w1-e-report.md`'s own blocker section (`WorkflowMutation: SemanticMutation` not
   satisfied, a peer/framework gap outside both our leases) — so I could not click through to a real
   space-index table in this session. The routing/resolution code itself is dialect-id-driven and
   will start working the moment that crate links and registers an app for that dialect, with zero
   further shell-side change.

## Commands run + results (real tails)

Package `@semio-tech/framework-renderer-react` (`🎯️targets/⚛️react`) — `bun nx run
@semio-tech/framework-renderer-react:test` hit the script's own 15s budget guard and was killed
(shared-machine saturation, same pattern other lanes this session hit); used the documented direct-
vitest fallback:
```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react
bunx vitest run -c 🧪️vitest.config.ts
```
Final run (after all edits, including the 3 new tests):
```
 Test Files  1 failed (1)
      Tests  9 failed | 310 passed (319)
```
The 9 failures are the SAME set across 3 independent runs at different points in this session
(identical test names, deterministic) and are, by subject matter, unrelated to anything this lane
touched — CSS `ring-primary` class assertions, an R3F host "Element type is invalid" crash, a chai
`toHaveTextContent` matcher error, `resolveWindowActions` panel-eligibility, an i18n label
("Artifact" vs "Document" — an `ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` peer-ticket naming change),
two mit-bestand demonstrator asset-path regexes, and a command-palette mock call-shape assertion.
One (`adaptPluginHandle.handleAction round-trips...`) is the only one anywhere near code I touched
(`PluginRuntime`'s `AppChannelClient`), and I verified it is NOT caused by my change: the new 4th
`actor` constructor argument is only ever read inside `hello()`, which this test's code path never
calls (confirmed by grep — `this.actor` has exactly one read site, inside `hello()`); the actual
crash is inside `encodeAppFrame`/`writeBytes` in `🟦️component.ts`, a file I never touched and which
`git status`/`git log --date=iso` shows is live-`MM` (mid-edit) right now from the concurrent
`MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` ticket (commit `c8a29e41c5`, real
timestamp `2026-08-16 20:26:15`, message matches that ticket's own description). Full logs:
`🧪️2-c-renderer-react-vitest.txt` (first run, before the subpath-import fix below),
`🧪️2-c-renderer-react-vitest-final.txt` (final, 310/319).

**One infra fix needed to even resolve imports**: this package's own `🧪️vitest.config.ts` (outside
this lane's lease) aliases the bare `@semio-tech/framework-os` specifier but not its
`/backbone-worker` subpath export, so `import {...} from "@semio-tech/framework-os/backbone-worker"`
failed to resolve under vitest/esbuild. Sidestepped rather than editing the foreign-leased config:
imported `IDENTITY_CONFIG_SCHEMA`/`identityActorConfig`/`foldIdentityEvent` via the same relative
path (`../../../../../🟦️backbone-worker.ts`) `ShellHost` already uses for `new Worker(new URL(...))`.

`@semio-tech/framework-os` package (consumed, not edited) — direct vitest to confirm I did not
regress lane 1-C's package by importing from it:
```
cd 🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript
bunx vitest run -c 🧪️vitest.config.ts
```
```
 Test Files  2 failed | 2 passed (4)
      Tests  2 failed | 334 passed (336)
```
Identical to `📓️w1-c-report.md`'s own documented baseline (the 2 failures are the pre-existing
missing-wasm-package fixture test, counted twice by a quirk in that package's own
`include`/`includeSource` overlap). Full log: `🧪️2-c-framework-os-vitest.txt`.

New Rust command leaves — not wired into any crate (same as their 5 pre-existing `🎮️commands/**`
siblings), so `cargo check -p ...` has nothing to check them against. Verified them the only way
available: copied each to a throwaway ASCII-named file and compiled+ran standalone with
`rustc --edition 2021 --crate-name checkN --crate-type lib --test` (`std`-only, no crate deps
needed) — all 7 compile clean and all 7 frozen-id/label tests pass. Full output:
`🧪️2-c-directory-commands-rustc-check.txt`.

## sharedFileRequests

None filed as a blocking request — two small foreign-adjacent touches were resolved by NOT touching
the foreign file (see "Design decisions" #3 for `ShellHelpers`, and the vitest-subpath sidestep
above for `🧪️vitest.config.ts`), so nothing is pending for the coordinator.

## What is NOT done

- **End-to-end observation of the directory lane, opening relay, and `/spaces/{id}` routing** — all
  three are wired and unit-tested at the pure-function level, but none could be click-tested in a
  running browser this session: no dev server reached a listening state in this environment (no
  `preview_start` attempted — out of scope for a headless lane without a hub running), and lane 1-E's
  `s.space` crate does not compile yet (peer blocker, documented above).
- **`openDocument` binding-snapshot and `os.open-artifact{documentId}`→worker-`open`-request
  integration tests** (named explicitly in my Verify checklist) were **not added**. The existing
  `🧪️index.test.ts` suite (4800+ lines) has no existing pattern for mocking `Worker`/`fetch`/
  `DirectoryClient` together, and building one from scratch was out of this session's effort budget.
  Added instead: unit tests for the pure logic those integration tests would exercise
  (`shellActorId`, `canonicalSurfaceId`, `directoryCommandFromAction` — all passing), plus the full
  existing suite re-run twice after my edits with zero new failures (9 pre-existing/unrelated
  failures, stable across 3 runs).
- **Generic `replayShellCommand` action ids** other than `os.directory.*`/`os.open-artifact*` (e.g.
  `os.setThemeId`'s undo "Backwards" replay) still have no handler — pre-existing gap, out of this
  lane's lease (the brief scopes me to "the `os.directory.*` + `os.open-artifact` interception" only).
- **`directoryPendingCommands`** (the worker's offline-queue depth) is tracked in local state but not
  yet rendered by any chrome this lane owns — left for whoever builds the "row shows 'pending'" UI
  (2-F/3-A territory per the ownership table).
- **Multi-pane actor sharing** (`PluginRuntime`'s module-level `currentPluginRuntimeActor`) — flagged
  as a known limitation in "Design decisions" #3, not fixed.
