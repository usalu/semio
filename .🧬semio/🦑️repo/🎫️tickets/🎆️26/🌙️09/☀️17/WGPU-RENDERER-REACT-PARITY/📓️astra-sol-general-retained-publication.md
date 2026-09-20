# General Retained Preference Publication

## Executed fail-first boundary

Checkpoint 16 physically dispatched `framework/setAppearance { value: "dark" }` and `framework/setLocale { value: "de" }`. Shell root state changed, including the dark canvas and German chrome, while the already-published `framework.settings.general` retained document kept the prior Select values `system` and `en`. The accessibility projection consequently kept `valueText: system`/`en` too.

The neutral fixture and schema define two exact cases on `framework.settings.general`: `system → dark` and `en → de`, with `requiresGuestRefresh: false`. The Rust law `host_preference_dispatch_republishes_general_without_a_guest_refresh` publishes the initial production document, crosses the real `dispatch_action`, and requires a changed retained revision and successor generation, the accepted Select and accessibility value, terminal retirement of the replaced lease, and no owed guest refresh.

Root executed Native50 against unchanged production. The law failed at the first appearance assertion because revision `4642242816561439936` remained unchanged. This establishes the missing publication as a runtime defect rather than an inferred source gap.

## Repair

`setAppearance` and `setLocale` now republish only `framework.settings.general` after the setting command is accepted. `republish_shell_panel_document` builds and admits the successor first, replaces the retained map entry only after admission, and retires the exact prior lease through `retire_one_surface_document`. It does not request a guest refresh and leaves the readable prior lease installed if successor publication is refused.

Native51 passed the appearance/locale publication law after the narrow repair. Native52 then executed the expanded schema cases and reported the exact missing Select roster: `setLayout`, `setTerminology`, `setMergePolicy`, and `setDriver`. Those four proven mutations now use the same narrow General replacement.

Native53 executed the corrected driver workflow reader and established the remaining missing publication roster as `setDriverField`, `setDriverSaveLabel`, `saveDriver`, and `deleteDriver`. The repair republishes only after a valid axis edit changes the draft, after the save label changes, after a successful save, and after a meaningful delete/reset. Invalid or idempotent mutations do not mint a successor.

The executed gates now cover every General authority in this packet:

| General authority | Retained reader | Evidence |
| --- | --- | --- |
| layout | `chrome_build.preferences.ui_layout` | Native52 RED; repaired |
| driver selection | `driver_id` | Native52 RED; repaired |
| terminology | `terminology_id` | Native52 RED; repaired |
| merge policy | `merge_policy` | Native52 RED; repaired |
| driver editor | draft, save label, custom drivers | Native53 RED; repaired |

Theme selection and theme editing publish a different retained leaf, `framework.settings.theme`, and require a separate law. The checkpoint Drivers expand/collapse failure is also separate: collapse state is paint and hit state, not a retained document value mutation.

The independent mounted React General oracle is green: 1 file, 9 tests, including the custom-driver draft/save/reset sequence. Native54 passed all five focused settings, retirement, and locale laws. The only Native54 failure was the separately owned Drivers disclosure geometry law.

## Canonical command label parity

The existing ShellSearch fixture now owns a locale matrix for `os.setThemeId`: English `Set Theme`, German `Thema festlegen`. The mounted React oracle changes the real i18n language and reads `buildOsCommands`; it passes with 1 file and 7 tests total. Native51 executed `built_in_command_labels_match_the_neutral_react_locale_oracle` RED as `Design festlegen` versus `Thema festlegen`; the WGPU registry now uses the canonical React label.

## Refusal ownership

Root review found that the first helper version replaced the map entry before `retire_one_surface_document` admitted the exact prior lease. Its `is_err()` path discarded the returned owner. The fail-first law `general_republication_refusal_preserves_the_exact_readable_owner_and_retries` exhausts the next admission epoch, requires the unchanged prior header/value to remain readable, clears the refusal, and retries to a new successor. Native53 reproduced the owner-loss defect: the expected generation-1 header had already become generation 2.

The repair adds exact `ShellDocumentRetirementAdmission { index, epoch }` ownership to the retirement registry. `try_reserve_admission` validates the closing state, epoch, and free slot before successor publication; `admit_reserved` consumes that same reservation after the retained map swap. A refusal therefore leaves the prior document installed and readable, and the law clears the forced exhaustion and proves an exact retry. Capacity is unchanged. Native54 passed the refusal law together with the four other focused settings and locale laws.

## Audited next packet: Theme and locale fan-out

This is a source audit only. No production change or native-pass claim is part of the completed General packet.

The retained Theme leaf currently has the same proven shape General had before Native50: its builder reads live Shell authority, while every Theme mutation returns without replacing `framework.settings.theme`.

| Theme mutation | Retained consequence that currently has no republication call |
| --- | --- |
| `setThemeId` | selector value, draft cleared, reset/delete availability, resolved field rows |
| eight `setTheme*` field verbs | edited value, dirty/reset state, appearance preview rows |
| `setThemeSaveLabel` | input value and Save disabled state |
| `saveTheme` | selected id, custom option roster, draft/save label reset |
| `applyImportedTheme` | custom option roster and selected id |
| `resetThemeId` | selected id, field rows, draft/reset state |
| `deleteThemeId` | custom option roster and possible fallback selection |

The fail-first law should publish `framework.settings.theme`, cross the real `dispatch_action`, and read the retained nodes after each valid mutation. It should cover one scalar field, one appearance paint channel, save-label enablement, save/select, reset, import, and delete. Invalid numeric field input and an invalid save label must keep the same revision. Each valid consequence must change revision and generation, preserve the replaced lease's exact retirement, and owe no guest refresh. Only after that law runs RED should the individual accepted branches call `republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)`.

Locale has a wider retained reader set. `setLocale` now republishes General only, but every mounted shell-owned leaf is built from `locale_id`: Theme, Keybindings, Default Apps, Conflicts, Display, Marketplace, Sync, Task Manager, Chat, Hub, tool leaves, and command-category leaves. `sync_dock_tabs` is also the owner that rebuilds framework/app panel tab labels from the new locale while preserving the persisted dock skeleton; `setLocale` currently does not call it. Palette rows and directly painted chrome resolve `locale_id` on demand and do not need a retained body replacement.

The locale fail-first law should mount at least General, Theme, Keybindings, and Default Apps, dispatch `en → de`, and require one successor revision per mounted localized leaf plus translated concrete labels. It should also require the same active dock paths and override skeleton with German tab labels after `sync_dock_tabs`. An unmounted leaf must not be created, and no guest window body may render.

A multi-panel locale replacement cannot be an unbounded loop inside `dispatch_action`. Each successor temporarily coexists with its prior lease, and calling the current helper repeatedly would run the large retirement drain once per leaf. The bounded production shape should be one `ShellLocalizedPanelRefreshCursor` holding the new locale generation and an exact snapshot of mounted shell-owned leaf ids. One frame step republishes at most one leaf, then advances the existing retirement registry until that prior owner reaches terminal before claiming the next. A second locale choice supersedes the cursor generation and restarts from the still-mounted ids; already-published documents are harmlessly replaced under the newest locale. `setLocale` updates `locale_id`, calls `sync_dock_tabs`, arms this cursor, republishes General through the cursor as part of the same roster, and requests the existing cursor wake. It must not call `refresh_ui`, render guest bodies, raise document capacities, or create absent panel documents.

The refusal law should exhaust retirement admission before the first locale leaf, prove every prior document remains readable with its exact revision and locale, clear the refusal, and walk the cursor to all-German successors. A mid-roster refusal must retain the exact current id and resume there; it must not silently skip to a mixed-language terminal state.

## Theme fail-first boundary prepared for Native55

The Theme source audit is now an executable contract. The strict neutral fixture `🎨️settings-theme-publication` sequences the actual host actions over one mounted `framework.settings.theme` document:

1. select `mono`;
2. edit `colors.primary` to `#123456`;
3. enter the save label `Retained Publication` and enable Save;
4. save and select `custom.retained-publication` while clearing the label and enabling Delete;
5. reset to `semio`;
6. reselect the saved custom theme;
7. delete it and fall back to `semio`.

The mounted React oracle renders the real shared `Tree`, `Select`, `Input`, and `Button` controls and executes that sequence with Testing Library. Its focused result is 1 file, 2 tests passed. The test also verifies that every neutral action and control address is authored by `ChromePanels` (mapping the WGPU `resetThemeId`/`deleteThemeId` verbs to React's `resetTheme`/`deleteTheme` host methods).

The native law `theme_host_mutations_republish_the_mounted_retained_leaf_without_guest_refresh` publishes the production Theme tree with Colors expanded, crosses the real Shell dispatcher for every fixture step, and compares retained header generation/revision. When unchanged, it records the missing action and manually republishes only so later consequences remain independently testable. It then reads the exact retained Select/Input/Button states, requires terminal retirement of each replaced lease, and requires no owed guest refresh. Cleanup removes the custom theme and restores `semio` before the final missing-publication assertion.

Native55 compiled the law in the full 1,238-test renderer census and reached the real workflow. It stopped during test continuation after `setThemeColor`: the authored `primary` color is sorted at index 37 of 43, outside the initial 28-row virtual Theme window, so the retained reader correctly reported that row absent. The corrected law derives the absolute primary row from the canonical `BTreeMap` and positions the production Theme scroll window there before initial publication. Selector controls remain materialized independently, while the exact real `framework.settings.theme.colors.primary` Input enters the retained document.

Native56 executed the corrected law against unchanged production and reached its intended final RED with the exact roster `[setThemeId, setThemeColor, setThemeSaveLabel, saveTheme, resetThemeId, setThemeId, deleteThemeId]`. All retained state observations passed after test-only continuation, proving the authority mutations themselves were correct and only the mounted retained leaf stayed stale.

The production repair reuses only `republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)`. Selection republishes when the selected id or draft changes. The shared eight-field branch compares the complete canonical document before and after the parser-backed commit, so all eight valid verbs replace the mounted leaf while invalid numeric input keeps the exact revision. Save-label changes, successful save, successful import, meaningful reset, and meaningful delete do the same. Empty/invalid imports and idempotent label/reset/delete operations do not mint a successor. The helper already owns successor admission and exact prior-lease retirement; Theme adds no parallel publication abstraction.

The neutral workflow now also carries a complete minimal canonical imported theme. Its mounted React oracle triggers the actual Import button and applies the same parsed-label slug rule, requiring `custom.imported-publication` to appear selected with Delete enabled. The native workflow crosses the real `applyImportedTheme` completion action. A separate native no-op law requires invalid radius text and `{}` import text to retain the exact header and retire no owner. The existing eight-verb document law now asserts color, spacing, font stack, stroke, radius, opacity, metric, and appearance-paint mutations individually. The extended React oracle remains green: 1 file, 2 tests passed in 7.23 seconds.

Native57 is the executed Theme gate receipt in `🗑️generated/astra-runtime/native-renderer-57.log`. All three Theme-owned laws passed: the full retained workflow, invalid-input exact-revision law, and eight-field authored-coordinate law. The mixed five-test invocation finished 3 passed and 2 failed because the separately owned Drivers and Display child-owner retirement laws were still red; neither failure entered the Theme workflow.
