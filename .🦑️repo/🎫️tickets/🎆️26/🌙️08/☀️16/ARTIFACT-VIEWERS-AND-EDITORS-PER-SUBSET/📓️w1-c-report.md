# W1-C — React Shell Role-Aware UI

Lane 1-C. Lease: `📺️renderer/🧑️‍🎨️engine/🧱️elements/{ShellHost,ChromePanels,ShellHelpers,Shell}/🟦️component.tsx`.
No new generic `🖱️ui/🧱️elements/**` element was needed — everything composes existing `Select`/`Icon`/
`Tree` primitives already imported by these four files.

## What landed

### C5.5 — boot role (`Shell/🟦️component.tsx`)

- `resolveBootAppRole(explicit?: AppRole): AppRole` (`Shell/🟦️component.tsx:186`) — reads
  `import.meta.env.VITE_SEMIO_APP_ROLE`, validates `"viewer"|"editor"`, falls back to `"editor"` on an
  unset/invalid value (warns, mirrors `resolveShellLocks`'s fall-back-but-stay-safe idiom). Guarded
  with try/catch for non-Vite embeds where `import.meta.env` doesn't exist.
- `FrameworkOsBootOptions.appRole?: AppRole` (`Shell/🟦️component.tsx:161`); `bootFrameworkOs`
  (`:1050`) resolves it via `resolveBootAppRole(options.appRole)` and passes it to `FrameworkOsShell`.
- `TransientNotice` type + `OverlayState.transientNotice`/`.openWithFocusRole` fields, `SET_TRANSIENT_NOTICE`/
  `SET_OPEN_WITH_FOCUS_ROLE` actions and reducer cases (`Shell/🟦️component.tsx:139,161,449-458,577-578,`
  `620-624`), initial state at `:1006`. `PluginManifest` gained optional `artifactKinds`/`dependencies`
  fields (`:97-105`) — additive, mirrors Rust `PluginManifest.artifact_kinds`/`.dependencies` (already
  present in the raw wasm-decoded manifest; only the shell-local TS type was missing them) so
  `AppRouter.build` can read ownership/dependency data when a plugin supplies it.
- `FrameworkOsShellProps.appRole?: AppRole` (`ShellHost/🟦️component.tsx:680`), threaded into
  `FrameworkOsShellInner` (`:762`) and into `establishPrimarySession`'s non-studio app pick: when
  `appId` isn't pinned, prefers `manifest.apps.find(app => app.role === appRole)` before falling back
  to `manifest.apps[0]` (`:1021`).

### C5.1 — role-aware sessions

`ActiveSession.app` was already `AppDefinition` (0-A's contract), so `session.app.role`/
`session.app.dialect` is the read path everywhere below — no new session field, no id-string parsing.
**Blocked on a stale generated binding — see "Not done" below.**

### C5.2 — viewer chrome

- **Role chip / read-only badge**: `ShellHost/🟦️component.tsx:5032`, next to the navbar app title —
  `surfaceRoleChipText(session.app.role, uiLocale)` ("Viewer"/"Betrachter", "Editor"/"Editor"), title
  attribute carries the read-only notice text for a viewer session.
- **Hides `Mutation`-kind actions/commands**: `filterDefinitionsForRole` (`ShellHelpers/🟦️component.tsx:1845`)
  applied in the shell fallback context menu (`ShellHost/🟦️component.tsx:4234`, replacing the bare
  `resolveWindowActions` call) and in `resolvedCommands` (`:4270`-area — actually the palette filter is
  at the `resolveCommands` call site, filters `entry.definition.kind === "mutation"` for a viewer
  session).
- **Disables undo/redo, hides checkpoint/revert**: the History footer tab (`ShellHost/🟦️component.tsx:4270`
  `isViewer` flag) disables the Undo/Redo buttons and omits Checkpoint/revert-to-command controls for a
  viewer session — "renders the history panel read-only" (contract §2.3).
- **Dispatch guard + fault handling**: `onAction` (`ShellHost/🟦️component.tsx:2876`) blocks a
  `Mutation`-kind action client-side for a viewer session and shows a notice instead of round-tripping;
  its `.catch` (and `onCommand`'s, `:4477`) also catches a `SemioFaultError` whose `fault.code ===
  SURFACE_FAULT_CODES.ViewerReadOnly` and shows the same notice instead of only `console.error`-ing —
  "arrives from the host, surfaces as a non-blocking notice, not a crash" (contract §2.3/§5).
- **Non-blocking notice UI**: `showTransientNotice`/`isViewerReadOnlyFault` (`ShellHost/🟦️component.tsx:3680,3692`),
  rendered at `:5680`+ as a dismissible `role="status" aria-live="polite"` banner, auto-dismiss after
  4s, never blocks the canvas (distinct from the existing blocking `error` state/`ShellFaultBoundary`).

### C5.3 — "Open with…"

- Frozen text + ids in `ShellHelpers/🟦️component.tsx:1782-1833` (`surfaceRoleChipText`,
  `openArtifactWithText`, `setAsDefaultText`, `defaultAppsSettingsTabText`/`…Label`,
  `OPEN_ARTIFACT_WITH_VIEWER_COMMAND_ID`/`_EDITOR_COMMAND_ID`) — see "Frozen strings live outside the
  chrome dictionary" below for why these resolve locally instead of through `shellLabel`.
- `groupOpenWithEntries` (`ShellHelpers/🟦️component.tsx:1864`) — `AppRouter` entries for one dialect,
  split by role, annotated `current`/`isDefault`.
- **Context menu**: `shell.openArtifactWith` entry (`ShellHost/🟦️component.tsx:5641-5642`), only shown
  when the session's dialect has ≥1 registered surface; routes through `dispatchShellMenuAction`
  (`:5595`) which focuses the Document panel's `FRAMEWORK_PANEL_TAB_ARTIFACT_ID` tab.
- **Command palette**: `open-artifact-with-viewer`/`open-artifact-with-editor` added to `buildOsCommands`
  (`ShellHelpers/🟦️component.tsx:3248-3253`, gated on `hasOpenArtifactSurfaces`), handled in
  `dispatchOsCommand` (`:3303-3308`) — sets `openWithFocusRole` and focuses the Document panel, same as
  the context-menu entry (neither dispatches a wire command itself; the row click inside the picker
  does — see next bullet).
- **Document panel**: `openWithSection` (`ShellHost/🟦️component.tsx:4212`), appended as one section per
  role (`:4247-4248`) to the existing `artifact.root` section — owner-plugin-first, each row opens that
  surface via `openArtifactWithAppRef` (`:3613`) and carries a "★/☆ Set as default" toggle wired to
  `dispatchSetDefaultApp`/`dispatchClearDefaultApp` (`:3584`, `:3596`-area).

### C5.4 — Settings sub-tab

- `FRAMEWORK_SETTINGS_DEFAULT_APPS_TAB_ID = "framework.settings.default-apps"` (`ChromePanels/🟦️component.tsx:76`)
  — matches Rust `PanelTabKind::SettingsDefaultApps.id_str()` (`🛂️manifest/🦀️component.rs:2607`).
- `DefaultAppsHostApi`/`DefaultAppRow` + `buildSettingsDefaultAppsTree` (`ChromePanels/🟦️component.tsx:862-935`)
  — one row per `(dialect, role)`, a `Select` bound to that role's `AppRouter` options plus a "None"
  entry that clears the pin; writes go through `host.setDefault`/`host.clearDefault` only.
  `createFrameworkSettingsPanelTab` (`:939`) takes an optional `getDefaultAppsHost` and appends the tab
  (`:985-999`) only when supplied.
- `ShellHost/🟦️component.tsx`: `appRouter` (`:3537`), `knownDialects` (read off every loaded
  `AppDefinition.dialect`, since `AppRouter` deliberately has no "every registered dialect" accessor),
  `defaultAppsRows`/`defaultAppsHost`/`defaultAppsHostRef` (`:3642-3667`), wired into
  `createFrameworkSettingsPanelTab(() => settingsHostRef.current, () => defaultAppsHostRef.current)`.

### Opening preferences (client mirror)

`openingPreferences` (`ShellHost/🟦️component.tsx:3565`-area) is a `useState<OpeningPreferences>`
advanced via `foldOpeningPreferences` (from `@semio-tech/framework`, lane 1-B) — the exact same
event-sourced fold the host uses to materialize `os.config.opening`, never a mutated map. See "Not
done" for why this can't read back the host's own log yet.

## Commands run + results

```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react
bun /Users/ueli/.bun/bin/bun node_modules/vitest/vitest.mjs run --config 🧪️vitest.config.ts \
  --passWithNoTests --testTimeout 15000 --hookTimeout 15000 --teardownTimeout 15000 --reporter=dot
```
→ **306 passed / 9 failed** (315 total), 18.9s. Full output: `🧪️w1-c-tests.txt`.

`bun nx run @semio-tech/framework-renderer-react:test` (the `project.json`-registered target) itself
times out at its own 15s/300s budgets on this machine even with zero of my changes exercised — the
`quick`/`long` reporters are too slow for a 315-test suite here; invoking `vitest.mjs` directly with
`--reporter=dot` (same flags the wrapper builds) is what actually completes. Noted so a re-run knows
to do the same.

**All 9 failures are pre-existing and unrelated to this lane** — none reference `role`/`viewer`/
`editor`/`openWith`/`defaultApps`/`AppRouter`/any symbol this lease introduced. Confirmed by name:
plugin-module-loading timeout (worker mock), a `ring-primary` CSS assertion in forms rendering, an
"Element type is invalid" VFS-scene test, `toHaveTextContent` not registered as a Chai matcher,
`resolveWindowActions` returning extra reserved actions, a `commandCategories` label drift
("Artifact"→"Document" — a rename apparently landed by a concurrent session, not this lease),
two mit-bestand logo-path regex mismatches, and a `CommandAddress` shape drift in
`buildCommandCategoryTabs`'s test fixture (`{kind:"os"}` vs the current `{owner,commandId}` shape).
This tree has confirmed concurrent churn from peer tickets (see `📌️important.md` and this repo's own
`git log` — the auto-commit at `0727b80a` bundled work from several sessions at once); these 9 did not
change count or content across two full runs bracketing this lane's edits.

```
bunx tsc --noEmit -p <scratch tsconfig extending the root one, adding
  allowImportingTsExtensions + the same @semio-tech/* → file path aliases 🧪️vitest.config.ts uses,
  include: the 4 edited files>
```
→ exits 2, ~1150-1165 diagnostics **in the whole reachable program**, essentially unchanged whether or
not the alias `paths` are supplied. The root `tsconfig.json` has no `paths` for `@semio-tech/*` at all
and lacks `allowImportingTsExtensions` (this repo's imports use explicit `.ts`/`.tsx` extensions
throughout) — bare `tsc --noEmit` against it fails on ~570 `TS5097` import-extension errors before it
even gets to real type errors, repo-wide, on files this lease never touched (`♻️mit-bestand/🧺️demonstrator/`
`🟦️brand.ts`'s `IntroductionGesture`/`TutorialTracks` mismatches, `🧰️framework/📦️packages/🟦️typescript/`
`🟦️glue.ts`'s statechart `eventCount` errors, ~148 errors inside `@semio-tech/ui-react`'s own
`📦️index.tsx`). This is a pre-existing gap between this repo's actual build pipeline (Vite/Nx, which
resolves the aliases and strips `.ts` extensions) and a bare `tsc` invocation, not something this lease
can close. Diagnostics were then filtered to lines inside the 4 edited files and cross-checked against
every `👁️✏️` marker this lease added: **zero diagnostics reference any symbol this lease introduced**
(`AppRouter`, `AppRef`, `DefaultApps*`, `OpenWith*`, `surfaceRoleChipText`, `openArtifactWithText`,
`setAsDefaultText`, `viewerReadOnlyNoticeText`, `filterDefinitionsForRole`, `isMutationKindDefinition`,
`groupOpenWithEntries`, `foldOpeningPreferences`, `SURFACE_FAULT_CODES`, `encodeDefaultAppValue`,
`dialectCoordinate`, `TransientNotice`, `openWithFocusRole`, `resolveBootAppRole`,
`PendingAppChannelMethods`, `pendingAppChannelFor`). Every diagnostic that DOES land near a marker
(cross-checked line-by-line) is one of: (a) the pre-existing `AppDefinition.role`/`.dialect` binding gap
below, (b) pre-existing `iconId`/`LocalizedLabel`/`Select` looseness already present in unmodified
neighboring code in the same files (verified by finding the identical error shape on unmodified lines),
or (c) a genuine pre-existing bug this lease found but did not introduce — see below. Full filtered
output is in `🧪️w1-c-tests.txt`.

Null-byte corruption check (`python3 -c "...count(b'\x00')..."`) on all 4 files: **0** — see "Drift risk
found and fixed" below.

## NOT done, and why

1. **`AppDefinition.role`/`AppDefinition.dialect` don't type-check yet.** `🛂️manifest/🦀️component.rs`
   has both fields (confirmed by reading the source directly — `role: AppRole` at `:2686`, `dialect:
   ArtifactDialect` at `:2690`), but the ts-rs-generated mirror `🛂️manifest/🤖️generated/🟦️manifest.ts`
   (`AppDefinition` at `:69`) does **not** — its `mtime` is 2026-08-14 10:46, two days before this
   ticket's start commit, meaning the regen 0-A's own lease description calls out ("ts-rs regen") has
   not run since. This produces ~15 `TS2339: Property 'role'/'dialect' does not exist on type
   'AppDefinition'` diagnostics at every `session.app.role`/`session.app.dialect` read this lease added
   (`ShellHost/🟦️component.tsx:1025,2880,3552,3662,4209,4216,4217,4260,4414,4467,5022,5023,5610` and
   similar). **This is not a defect in this lease's code** — every read is against the frozen contract
   shape (§1), and none of these sites use a workaround/cast to paper over it (that would hide the real
   signal once the regen lands, and risks lying about a capability that isn't actually there yet). The
   generated file is outside this lease (`🛂️manifest/🤖️generated/` was 0-A's), so it wasn't touched.
   **Action needed**: someone re-runs the manifest crate's ts-rs export (or the coordinator re-triggers
   0-A) to regenerate `🛂️manifest/🤖️generated/🟦️manifest.ts`; this lease's code will then compile with
   zero changes.
2. **`os.open-artifact`/`os.set-default-viewer`/`os.set-default-editor`/`os.clear-default-app` are not
   wired end-to-end over the wire.** `AppChannelClient` (`💻️os/🟦️component.ts:2364`) needs the RAW
   `exchange` ABI, but `PluginRuntime`'s `PluginWasmHandle` (`PluginRuntime/🟦️component.tsx:47`, out of
   this lease) wraps that ABI behind typed methods and does not re-expose `exchange` itself — confirmed
   by reading `adaptPluginHandle`'s own test (`PluginRuntime/🟦️component.tsx:1151`): `transactionPrepare`/
   `transactionCommit`/`transactionUndo`/`transactionRedo` are wrapped that way (each internally riding
   its own `AppChannelClient`), but `openArtifact`/`setDefaultApp`/`clearDefaultApp` are not. This lease
   first tried constructing its own `AppChannelClient` directly from the handle ShellHost is given — that
   doesn't type-check (`Property 'exchange' is missing in type 'PluginWasmHandle'`) and would have been
   architecturally wrong even if it had, since two independent `AppChannelClient`s would race the same
   instance's `seq` counter. Fixed by feature-detecting the three methods as optional
   (`PendingAppChannelMethods`, `ShellHost/🟦️component.tsx:3568-3576`) — structurally typed (no `any`/
   lying cast), inert today, and will start actually firing the moment `PluginRuntime` adds the same
   three wrapper methods the transaction family already has, with zero further changes here. Until then:
   the **local** effects (`openingPreferences` fold, the Settings table, the Document panel's picker,
   the actual session switch on "Open with…") all work today; only the host notification/persistence
   round trip is inert.
3. **`openingPreferences` has no host readback.** `AppChannelClient` (as of this write) only has
   `setDefaultApp`/`clearDefaultApp` (writes), no `readConfig`/`loadConfig` for `os.config.opening`. This
   shell's `openingPreferences` is therefore a **local-only mirror**, advanced solely by this shell's
   own `dispatchSetDefaultApp`/`dispatchClearDefaultApp` calls (via the exact same `foldOpeningPreferences`
   the host uses) — a pin made by another shell/session is not reflected here until a readback API lands.
   Documented inline at `ShellHost/🟦️component.tsx:3561-3564`.
4. **`artifactRef` for `openArtifact`/context is approximated as the dialect coordinate**
   (`dialectCoordinate(dialect)`, e.g. `s.cad.cad@1/*`), not a per-document identity — ShellHost has no
   surfaced concept of "which specific document" beyond dialect yet. Documented at
   `ShellHost/🟦️component.tsx:3600-3602`.
5. **Command-palette `studio.undo`/`studio.redo` (hostMode's own host-controller history,
   `ShellHost/🟦️component.tsx` ~5090s) were intentionally left un-gated** — they drive the STUDIO HOST
   app's own history, not the focused artifact session's, so they're outside this contract's viewer/
   editor scope.
6. **W2 hasn't shipped real per-plugin viewer/editor surfaces yet.** Every piece here (`AppRouter`
   grouping, the Settings table, "Open with…") is built against the live `AppRouter`/`AppDefinition.role`
   contract and is exercised today by whatever single-role apps already exist (compiling once finding
   #1 lands) — visually empty "Open with…"/no extra Settings rows until W2 gives dialects a second
   registered role. This is expected, not a gap in this lease.

## Drift risk found and fixed (report per CLAUDE.md "you MUST NOT assume")

While authoring `ChromePanels/🟦️component.tsx`, two isolated single-space string literals I wrote
(`` `${app.pluginId} ${app.appId}` `` in `encodeDefaultAppValue`, and `value.indexOf(" ")` in
`decodeDefaultAppValue`) were silently written to disk as a literal NUL byte (`\x00`) instead of a
space — confirmed by reading the file back with Python in binary mode (`Read` itself displays the NUL
as a normal space, masking it). Both were caught only because `Edit`'s exact-string match then failed
against the actual on-disk bytes; fixed with a targeted binary patch. All 4 files were re-audited
(`python3 -c "...count(b'\x00')..."`) and are clean (0) as of the final report. **Any future editor
of these files should not assume a plain-looking space character in a template literal or string
literal was written correctly without a binary check** — this looks like an environment-level encoding
issue, not something specific to these files.

## Pre-existing bugs spotted (not fixed — out of scope, flagged instead)

- `Shell/🟦️component.tsx:135-136,145-146` (`SpaceProgramEntry`/`SpawnedAppEntry`): both types declare
  `label` twice (`readonly label: string;` immediately followed by `readonly label: readonly string[];`)
  — a real `TS2300`/`TS2717` compile error, pre-existing (not touched by this lease), unrelated to
  viewer/editor work. Flagged for a follow-up, not fixed here (this file is under concurrent edit and
  the duplicate predates this ticket).

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🟦️component.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ChromePanels/🟦️component.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx`
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/🧪️w1-c-tests.txt` (new)
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/📓️w1-c-report.md` (new, this file)

No files outside this lease were edited. `🖱️ui/🧱️elements/**` was not touched — no new generic element
was needed.
