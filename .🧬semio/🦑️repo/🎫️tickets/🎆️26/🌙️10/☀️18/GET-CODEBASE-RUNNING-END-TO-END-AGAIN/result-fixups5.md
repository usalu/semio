# Result fixups5: comment docstrings, anchor tests, Vite loaders, Rust test seams, tsc fixture types

Executor `fixups5` (foreground, no sub-agents, no modifying git command, no cargo, no dependency sync). `C` = this ticket folder. HARD FREEZE respected: nothing under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/` was edited, no `package.json`, `bun.lock`, root `📜️script.ts` or `🔣️taxonomy.json`. Library fixes (item 5) went only into the staged copies and `patches.json` under `C/tsc-stage-tools/`.

Housekeeping: `aud-verify9` landed in `C/C/`; moved to `C/audit-verify-1019i.md`, empty `C/C/` removed.

## 1. The 35 body-level `/** */` blocks (comments-a)

The comments-a log truncates the text at about 160 characters, so the full texts were read back from `fd14da55f18^` (the commit before the strip). Tools: `C/fixups5-tools/` (`blocks.json` 35 rows, `block-texts.json` full texts, `extract-block-texts.py`, `restore-block-docstrings.py`; the scripts' scratch folder was deleted, they are inputs only).

Rule applied: a `/** */` that sat before a `useEffect(...)` or an assignment cannot be a docstring of that statement, so its text is appended as an emoji-led paragraph to the docstring of the enclosing named declaration (hook, function, class method). 30 restored, 5 dropped.

| Where (19 files) | Restored into |
|---|---|
| `🌍️world/🎨️r3f/🎬️canvas` (wheel hook on the event source) | `WorldCanvas` |
| `🎭️actor/📮️shard-client/⏱️watchdog` (monotonic advance only) | `ShardWatchdog.pollHeartbeatSab` |
| `TextEditor/👥️session-presence-and-caret`, `World3dHost/🎛️marquee-and-gumball` | `useEditorSessionPresence`, `useWorldMarqueeAndGumball` |
| `ShellHost/⚡️host-effects/📡️operation-subscriptions` (4) | `useOperationCompletionSubscriptions`, `useOperationProgressSubscriptions` |
| `ShellHost/🎮️focus-and-input/🎯️focused-program-and-introduction` (3) | `useFocusedProgramAndIntroductionState` |
| `ShellHost/🏗️dock-assembly/*` (3) | `useIntroductionRevealEffects`, `usePanelPathsAndOverrideReveals`, `useToolTabSelectionEffects` |
| `ShellHost/📬️artifact-opening`, `🔁️contributions-refresh`, `🖼️canvas-footer` (4), `🗄️shell-uri-documents`, `🛂️directory-route`, `🛫️spawned-session` (2) | new emoji-first docstrings on `useArtifactOpening`, `useContributionsAndRefresh`, `useCanvasAndFooter`, `useShellUriAndDocuments`, `useDirectoryAndRoute`, `useSpawnedSession` (these hooks had none; each summary line starts with an emoji not used elsewhere in its file) |
| `ShellHost/🔒️identity-document-ports`, `🖱️shell-actions/🧭️input-funnel`, `🚀️plugin-lifecycle`, `🔗️HubConnection`, `🛠️ShellHelpers/🎛️command-registry` | existing docstrings of `useHubPortInferenceAndDocumentBackbone`, `resolveDispatchTarget`, `usePluginLifecycleState`, `useHubCommandCallbacks`, `buildOsCommands` |

Dropped (pure narration of the next statement or a duplicate of an existing docstring): #6 (restates the hook summary), #11 (identical to the docstring of `programHistoryProjectionsRetainedV1`), #14 (names a call), #17 (names a call), #31 (history of a deleted function, and it sat above an unrelated effect).

Also restored the `Effect::LoadDocument` note as a docstring paragraph of `applyLoadDocumentEffect` (`📥️document-load-effects`), as the audit asked.

Verification: `bun build --no-bundle` of all 19 edited files: 19 ok (exact loop run). Two phrases that said "the effect above" were reworded so they read correctly inside a docstring.

## 2. Three anchor-broken tests, re-anchored on code

| Test | New anchor |
|---|---|
| `🧪️tests/🪟️spawned-program-session/🟦️.tsx` (focus) | the enclosing `useEffect(` of `const holder = document.activeElement;` (`lastIndexOf` from the code line, plus a presence guard) |
| `🧪️tests/♻️hot-swap-session-handover/🟦️.ts` | `cancelFlowTasks(sessionRef.current);` and the fixture's `}, [surfaceId]);` within 900 characters |
| `🧱️elements/🛠️ShellHelpers/⏪️time-travel/🧪️tests/📡️reprojection-status/🟦️.tsx` | reads `ShellHost/⚡️host-effects/📥️document-load-effects/🟦️.ts` (moved file) and slices `applyLoadDocumentEffect` up to the next `export` |

Run (each with `./<file>`):
- `bun test ./…/♻️hot-swap-session-handover/🟦️.ts`: 8 pass, 0 fail.
- `bun test ./…/🪟️spawned-program-session/🟦️.tsx -t "hands keyboard focus"`: 1 pass. The full file: 50 pass, 4 fail; the 4 are `ReferenceError: document is not defined` (`render` needs the jsdom environment of the vitest run, not available under `bun test`), unrelated to the anchors.
- reprojection-status under `bun test`: 1 pass, 4 fail for the same `document is not defined` reason (the source-scan assertion lives inside a DOM test). The moved-file assertion was therefore run on its own (`C/fixups5-tools/reproj-probe.ts`, same path, same slice, same three checks): result `[1, true, true]`, the value the test expects.

## 3. Vite config loader for the gis and puzzle variants

No variant-side edit is possible or needed. `✏️s/🧑‍💻dev/🎭️variants/{🌍️gis,🧩️puzzle}/🏗️builder/🌐️vite/🟦️.ts` are bare `createFrameworkOsDevConfig(...)` defaults with no call site of their own; the only routes that load a variant's `viteConfig` are the os dev `♻️activation/🌐️serve/🟦️.ts` and `🚚️distribution/📜️script.ts` (`config: contribution ? resolveDevContributionFile(...)`), and both lines carry `configLoader: "native"` (checked: both lines, 931 and 815 characters). The frozen law `🎛️vite-config-loader` already pins exactly that line in both routes, so the variants are covered. The audit's `runner` failure came from running the probe with the wrong loader.

Re-check with `C/e2e-tools/resolve-vite-config.mjs <config> build native`:
- gis and puzzle variants: pass the inventory step and stop at the missing activation artifact `registry/dist/sessions/s/🎮️playground-session/🟦️.ts` (expected before activation).
- os dev builder: loads under native, evaluates its whole import closure (2.5 to 4 s) and stops at line 93, `Unknown playground module identity: framework-empty` (also for `SEMIO_PLUGIN=s` and `puzzle`): the bare builder gets its playground list from the activation inventory, so this is the same pre-activation stop. The earlier "module runner has been closed" does not occur under native. `runner` for os dev was started and not awaited (it prints Vite warnings first); native is the loader the product uses.

## 4. Rust `pub` widenings of rg-test2 behind `test-support`

Crate `semio-framework-os-store-one-item-batch` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️one-item-owners`).
- `📬️paged-one-item/🦀️.rs`: the five functions (`retain_seal_returns`, `returned_edit_demands`, `close_returned_edit`, `returned_post_demands`, `close_returned_post`) now live in the private `mod returned_custody` (still `pub fn` inside it, so the lib uses them through a private `use`), they are no longer public API of `paged_one_item`. `Stage` is no longer re-exported at the module root. New `#[cfg(feature = "test-support")] #[doc(hidden)] pub mod test_support { pub use … Stage; pub use … the five }`.
- `Cargo.toml`: `[features] test-support = []` and `[dev-dependencies] semio-framework-os-store-one-item-batch = { path = ".", features = ["test-support"] }`, so only the crate's own test targets get the seam.
- Test files re-pointed: `paged::Stage` to `paged::test_support::Stage` (4 sites in `🧬️mutation-apply/🧪️tests/🦀️.rs`), the five imports in `🧪️tests/🔗️source-custody/🦀️.rs` to `paged_one_item::test_support`.
- Verification: `rustfmt +nightly-2026-07-20 --check --config skip_children=true,disable_all_formatting=true` (parse only) on the three edited `.rs`: rc 0. `Bun.TOML.parse` of the manifest shows the feature, the self dev-dependency and the `[[test]]`. `bun test ./🧰️framework/🛍️products/💻️os/🧪️tests/📦️crate-boundaries/🟦️.ts`: 127 pass, 0 fail. **Not compiled**: cargo is blocked by `Cargo.lock` (`--locked`), so the self dev-dependency and the `use super::*` visibility are unproven by a compiler; the lock needs the usual refresh first.

## 5. tsc-stage: the 16 `expect<T>` widenings removed at the fixture types

Edited in the staged set; `patches.json` regenerated from the real files (`C/tsc-stage-tools/restage.py`, anchors verified to apply in order), `staged/` refreshed (identical to the edited copies; `stage.py check`: 23 patchable, 1 base drift as before). Root causes, no `any`, no `ts-ignore`:
- Fixture rows read as the closed type the implementation returns, once per fixture: `Omit<Row,"expected"> & {expected: Union}` plus a single boundary assertion on the JSON array (storage `CacheLinkRefusalRow`, bazel actions `Expecting<…>`, store-receipt `ReceiptFileRow` and `entries: readonly StoreReceiptEntryV1[]`, dashboard-control `PathCaseRow`/`RoleCaseRow` with a new `IdeDisposition`, path-budget `PathBudgetCase`, source-ownership `cases` role type).
- Oracle result types aligned with the API: `BazelUncachedReasonV1`, `IdeDisposition`.
- Cache decisions: `ajv.compile<Decision>` so `validateDecision` is a type guard; `decisionOf` returns `string[] | null` like the JSON arrays.
- Two real narrowings instead of casts: invocation test compares `actual.env.SEMIO_BUILD_MODE` with `oracle.mode ?? undefined`; dashboard test computes `invoked = native.dashboardInvocation(row.args)` once and throws if a launch happens for a non-dashboard invocation (before, `toEqual(undefined)` could only fail).
- New (peer test committed at `fd14da55f18`, same class): `🧪️tests/🧱️package-implementation-admission/🟦️.ts` `SourceCaseRow`, added as the 23rd patch.

Confirmation:
- `C/fixups5-tools/tsc-fixtures.sh` (`tscheck-files.mjs`, closure of the 8 edited roots, 30 s): 0 errors with the widenings removed, 19 errors before the typing.
- Overlay tsc over the whole library config (`tscheck.mjs … 📦️packages/🟦️typescript/tsconfig.json` with the exported overlay, 5,030 files, 128 s): **2 errors**, both in `🏗️build/🧪️tests/🕸️graph/🟦️.ts` lines 954 and 959, a 51-line addition a peer is editing right now (uncommitted `M`; `semanticValueImports` rows, `expect(...).toBe(row.expected)` with a string against a role union). Same fix class (type the `semanticValueImports` row's `expected`); left alone because the file is mid-edit. Before the 23rd patch the run showed 9 errors (this file 7, the admission test 1, plus the same file's mid-edit state); 5 of them vanished by themselves when the peer finished.
- Tests of the patched copies (scratch mirror of `stage.py mirror`, compared test by test with the unpatched originals): storage, bazel actions (10 pass), cache, store-receipt, path-budget (45 pass), package-implementation-admission (20 pass) identical; invocation 2 of 3 pass (the third, "actual build CLI reads help", fails only in the mirror: "workspace root … markers not found", a mirror artifact); dashboard-control identical failures (3, pre-existing) apart from a mirror-only path-length check; source-ownership one mirror-only path assertion. No difference attributable to the edits. Mirror deleted.

## Open items

1. Compile evidence for item 4 after the `Cargo.lock` refresh: `check -p semio-framework-os-store-one-item-batch --tests`.
2. Peer in-flight: the two `🕸️graph` test errors above; apply order for the queued patches unchanged (`stage.py check` before `apply`).
3. `🚫️async:` register decision (audit item 2) is not part of this task.
4. Optional, frozen: a permanent law that resolves every variant `viteConfig` natively is not needed (the existing law pins the routes); the `✂️node-native-typescript` finding F2 of e2e-recheck is unchanged.
5. `bun test` cannot run the DOM-based suites (`document is not defined`); they need the vitest jsdom run.

## Files

Edited (repo): the 19 files of item 1 plus `📥️document-load-effects/🟦️.ts`; tests `🧪️tests/🪟️spawned-program-session/🟦️.tsx`, `🧪️tests/♻️hot-swap-session-handover/🟦️.ts`, `…/📡️reprojection-status/🟦️.tsx`; Rust `📬️paged-one-item/🦀️.rs`, `📬️paged-one-item/🧬️mutation-apply/🧪️tests/🦀️.rs`, `📬️paged-one-item/🧪️tests/🔗️source-custody/🦀️.rs`, `📬️one-item-owners/📦️packages/🦀️rust/Cargo.toml`.
Ticket: `C/tsc-stage-tools/patches.json`, `staged/*`, `tscheck-files.mjs`, `restage.py`; `C/fixups5-tools/*`; `C/audit-verify-1019i.md` (moved); this file.
