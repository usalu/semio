# B1a — Dormant Plugin Boots (batch A): writer · mathematical · vcs · animate · sequence · architect

Slice B1a of `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`. Scope: drive the six batch-A plugins from
`📓️audit-plugins-artifacts.md` Q1's **UNTESTED** list through a real React-renderer boot, hold them to the
interaction bar (page loads with no console faults · the default example renders · one Actions-panel verb
dispatches AND mutates the document · undo retires that mutation), fix the root defects, and register
launch entries. Method copied from `📓️b1b-dormant-plugin-boots.md` (activate once, serve detached, probe
headless, fix at the root).

> **Status.** Two workers died on this slice before reporting and `🗑️generated` was wiped twice. The
> sections below the "Per-plugin results" heading marked *predecessor* are inherited prose whose tree
> edits this worker verified on disk but whose claims it could not otherwise check; everything under
> **Per-plugin results**, **This worker's fixes**, **Measured gaps** and **This worker's files** is
> this worker's own, each row backed by a capture in `🗑️generated/`. One inherited claim is now known
> false and is struck through where it appears (the twelve launch.json rows).
>
> **Bottom line: 6/6 boot and 6/6 compile; 2/6 (`architect`, `sequence`) dispatch a real document
> mutation and undo it; 0/6 pass the whole bar.** The single thing standing between `architect` and a
> pass is a framework-level `Effect::LoadDocument` failure that `writer`'s bespoke authority hits too.

## Headline

- **The inherited "booted: true" results were false positives, and the first thing this slice did was
  prove it.** The predecessor's probe scored an interaction by diffing the accessibility tree across the
  click — but expanding an action row to reach its arguments is itself a `[role="treeitem"]` change, so a
  *refused* verb scored as a state change (exactly the trap `📓️b1b-dormant-plugin-boots.md`'s method notes
  warn about). Driving `architect`'s `setAdjacencyKind` by hand against the live server produced
  `entries: 0` in the shell's own undo ledger and this console line — **at `warning` level, which the old
  probe's error-only filter never read**:

  ```
  warning input #3 addRegisterItem refused: dispatch-failed (user window=architect-register)
      — UI dispatch rejected action:addRegisterItem with interactive-job classification BatchOnlyPendingRewrite
  ```

- **The probe was rewritten around the shell's own witness.** `[data-history-json]`
  (`{cursor, canUndo, canRedo, entries, actionIds, undoLabel}`) is the shell's undo ledger; a dispatch
  counts only when it appends an entry whose `actionIds` row is **not** `shell.*`, and undo counts only
  when `cursor` falls and `canRedo` turns true. The console filter now reads every level, not just
  `error`. `🐍️b1a-boot-probe.mjs`.

- **Zero of six passed the bar before this slice**; the measured pre-fix state was
  `{loadsClean: false, dispatched: false, undoWorks: false}` for `architect`, `writer` and `sequence`, and
  `vcs` did not reach `data-semio-os-ready` at all.

- **Two defect families explain every batch-A failure**, and they are the same two `📕️norm`/`🕸️dag` hit in
  batch B: **F3** (`command_from_action` not overridden — four of six) and **F1**
  (`BatchOnlyPendingRewrite` — two of six). A third, **F9** (`setActiveExample` never declared), is what
  produces the one console *error* every batch-A plugin logs at boot.

## Per-plugin results

Measured by this worker (fleet 3, 2026-09-19 01:00–01:50 CEST) against live React dev servers, one
activation + one detached serve per plugin, headless Chromium `--use-angle=metal`, 1600×1000 viewport.
Captures: `🗑️generated/b1a-<plugin>-console.txt` (summary, shell view, per-step detail, every console
line) and `b1a-<plugin>.png`. **`bar` is the conjunction of all four columns; nothing here passes it.**

| plugin | crate | port | `cargo check` | boots | example renders | dispatch mutates | undo retires | fault lines | unit tests |
|---|---|---|---|---|---|---|---|---|---|
| 🏛️architect | `semio-s-artifact-architect-program` | 6090 | green | **yes** | **yes** (2 surfaces, 53 svg nodes) | **yes** — `setAdjacencyKind` lands `connectAdjacency` | **yes** — `canRedo` true, signature restored | **1** (was 2) | **4/4** added |
| 🎬️sequence | `semio-s-artifact-sequence-sequence` | 6077 | green | **yes** | **yes** | **yes** — `addStepDropped` | **yes** | 8 | — |
| ✒️writer | `semio-s-artifact-writer-writer` | 6062 | green | **yes** | **yes** | no | not reached | **2** (was 6) | — |
| 🎞️animate | `semio-s-artifact-animate-presentation` | 6051 | green | **yes** (was a **guest trap**) | no — boots with no window open | no (no actions pane) | not reached | **1** (was 7) | — |
| 🌿️vcs | `semio-s-artifact-vcs-vcs` | 6075 | green | **yes** | **yes** | no | not reached | **4** (was 20) | — |
| ➗️mathematical | `semio-s-artifact-mathematical-equation` | 6084 | green | **yes** | **yes** | no | not reached | 11 | — |

`--features component-app-assembly` **does not exist on any of these six crates** (cargo lists the 58
crates that do have it; none is a batch-A artifact). The memo's method line is wrong for this batch —
plain `cargo check -p <crate>` is the right gate and all six are green.

Each plugin's blocking fault, in one line:

| plugin | what stops it |
|---|---|
| 🏛️architect | boot `setActiveExample` → `document archive replacement failed closure, authority, or retained publication validation` (shared, see Gaps) |
| 🎬️sequence | F9 not applied here — boot `setActiveExample` is still dropped as undeclared (console **error**); `setViewport`/`addStepToSlot` need args the Actions pane does not stage |
| ✒️writer | same shared archive-replacement failure, twice; its Actions pane offers only `formatDocument` (inert on an unedited document), `lintDocument` (transient lane) and `setActiveExample` — **no mutating document verb is reachable from the pane at all** |
| 🎞️animate | boots with zero windows ("Drag windows from Display in the navbar"), so there is no Actions pane to dispatch from; the boot `setActiveExample` then hits the shared archive failure |
| 🌿️vcs | F9 not applied; `incrementCounter` now fails `batched item candidate failed its exact fixed fold contract` (its retained rows are not point-invertible) |
| ➗️mathematical | F9 not applied; `setDocument`/`nodeGraphViewport`/`setPoints` each refuse with `requires a '<block>' block` — the Actions pane stages no `#[dsl(block)]` payload; `setDirected`/`nodeGraphEdit` are rejected by the retained command reducer; `setAlgorithm` is silently inert |

## The defect families

### F3 — `ArtifactApp::command_from_action` is not overridden (writer, mathematical, animate, sequence)

The trait default (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:11763`) refuses every id:

```
action '<id>' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand)
— app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)
```

The React/wgpu shells still speak `{action, args}`, so with no bridge **no** Actions-pane row, example pick
or canvas gesture can reach `Command::dispatch`. `app_commands!` generates `command_id`, `dispatch`,
`TOOL_JOB_IDS` and both codecs but no `from_action` — each app writes its own. Four of the six batch-A
plugins had none; `vcs` and `architect` already did.

Fixed, one bridge per app, each resolving **every** id the app declares and faulting unknown ones with the
app's own message instead of the framework's generic refusal:

| app | bridge | ids |
|---|---|---|
| ✒️writer | `…/✒️writer/…/✳️any/✏️editor/🦀️.rs:1161` | 18 (incl. the three-payloads-one-id `setEditorSetting` split) |
| ➗️mathematical | `…/➗️equation/…/✳️any/✏️editor/🦀️.rs:1311` | 6 |
| 🎞️animate | `…/🎬️presentation/…/✳️any/✏️editor/🦀️.rs:713` | 17 |
| 🎬️sequence | `…/🎬️sequence/…/✳️any/✏️editor/🦀️.rs:3206` | 16 |

Two shapes are worth naming because both cost a compile round:

- **`#[dsl(block)]` payloads must be decoded through a generic `fn`, not a closure.** `writer`'s camera,
  `mathematical`'s `graph`/`geometry`/`viewport` and `animate`'s `crop`/`frame`/`source` each decode to a
  different type; a single `let decode = |key| …` closure monomorphizes to the first one and the rest fail
  `E0308: '?' operator has incompatible types`. Each bridge now carries a local
  `fn decode<T: dsl::FromValue>(action, args, key) -> Result<T, Fault>`.
- **`DslValue` has no `List` variant** — the array variant is `DslValue::Array`
  (`🧰️framework/🔨️modules/🌱️value/🦀️.rs:110`). `animate`'s `renameTiles`/`patchTileCrops` take `Vec<String>`.

`setEditorSetting` is the one manifest id carrying three payload types (`SetFontPx`/`SetLineHeight`/
`SetTabSize`, the `app_commands!` rows at `…/✏️editor/🦀️.rs:225-227`). The bridge selects on a `setting`
key (`fontPx`/`lineHeight`/`tabSize`) the way the rows select on their wire keyword, and faults an
unknown one rather than silently writing the font size.

### F1 — `InteractiveJobClassification::BatchOnlyPendingRewrite` (architect 20 verbs, animate 14)

`validate_ui_dispatch_classification` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12594`) admits
**only** `Migrated`. Every other disposition is refused as `interactive-job.not-ui-safe`, at `warning`
level, which is why the earlier probe never saw it. Flipping the label alone traps the guest at boot
(`interactive-job.catalog-incomplete`), so each promoted id needs the full four-part catalogue **plus** a
fifth requirement this slice hit that batch B did not:

**The `Artifact` publication lane is only SUPPORTED when the app owns a one-item artifact-store
preparation factory.** `plugin_runtime`'s boot scan
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21917-21940`) walks every registration and marks a
tool `unsupported-publication-contract` when it declares `Artifact` while
`build_artifact_store_one_item_preparation_factory()` is `None`. `architect` and `animate` are the two
batch-A apps with no such factory — so for them, promoting a document verb is a *two*-part change.

### F9 — `setActiveExample` is never declared (architect, mathematical, vcs, sequence)

The playground navbar dispatches `setActiveExample` on boot for its fixture combobox. Four of the six apps
declare no such action, so the shell drops it before dispatch and logs the batch's only console **error**:

```
semio: app "s.architect.program@1/*#editor" dropped action "setActiveExample" dispatched from window kind
"architect-adjacency": no window kind declares it (window kinds: architect-adjacency, architect-graph,
architect-register, architect-report, architect-trace).
Declare it with .window_kind_actions()/.window_kind_action_refs() on the window that dispatches it.
```

`try_build_definition` copies app-level actions onto every window kind, so the fix is an app-level
`.action_with(ActionDefinition::new("setActiveExample", …))` plus a command whose handler emits
`Effect::LoadDocument` (whole-document replace has no mutation-enum representative) and a
`build_document_store_initialization_job` override so the host admits that envelope — note's shape,
`✏️s/🔌️plugins/🗒️note/…/✳️any/✏️editor/🦀️.rs:694` + `🎮️commands/🧺️set-active-example`. **Not done in this
slice** — see Gaps.

## Root causes fixed

Inherited from the predecessor's (uncommitted, staged) tree edits — verified present on disk by this
worker, verified compiling/at-runtime only where the table above says so:

| # | family | file | evidence it is there |
|---|---|---|---|
| 1 | F3 | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1161` | `fn command_from_action` present |
| 2 | F3 | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1311` | `fn command_from_action` present |
| 3 | F3 | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:713` | `fn command_from_action` present |
| 4 | F3 | `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:3206` | `fn command_from_action` present |
| 5 | F9 | writer editor `…/✏️editor/🦀️.rs:217,292,1096,1122,1377` | `setActiveExample` command row, action definition, `Migrated` classification and `build_document_store_initialization_job` all present |
| 6 | F9 | animate editor `…/✏️editor/🦀️.rs:251,273,806,841` | same shape as writer |

This worker's own fixes are appended below as they land.

### This worker's fixes

| # | family | file:line | what it was |
|---|---|---|---|
| 7 | **new — envelope drop witness** | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:127` | `reset_document_effect` minted an `ArtifactEnvelope` just to print its spr and then dropped it. An envelope is a terminal store shell whose `Drop` asserts its bounded retirement authority detached every nested owner, so **building the effect panicked**: `artifact envelope terminal shell reached Drop before its app-owned bounded retirement authority detached every nested owner`. It now uses `store::empty_document_spr` like `🗒️note`/`✒️writer`/`📐️cad`. This also un-broke the pre-existing `import_registers_csv_action_sets_plugin` test, which had been failing on the same panic. |
| 8 | F9 | architect editor `…/✏️editor/🦀️.rs` — new command module `🎮️commands/📚️example/🦀️.rs`, crate-root `pub mod example`, `app_commands!` row, `ARCHITECT_DOCUMENT_TOOL_IDS`, `ARCHITECT_RETAINED_TOOL_IDS`, `PUBLICATION_CONTRACTS` (`HostOnly`), `architect_document_extent`, `bounded_first_step_tool_proofs!` `tools:`, app-level `ActionDefinition`, `action_interactive_job(Migrated)`, `action_args`, `command_from_action` arm | the playground navbar's boot `setActiveExample` was dropped as an undeclared action — architect's only console **error**. |
| 9 | F9 follow-on | architect editor `…/✏️editor/🦀️.rs` `build_document_store_initialization_job` | with F9 in place the host then refused the app's own archive with `artifact-store.persisted-initializer-refused`; the trait default owns no retained initialization authority. Now delegates to `bounded_document_store_initialization_job` (the `🕸️dag` precedent). |
| 10 | **new — guest trap at boot** | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:225` | the same envelope-drop defect as #7, but here it **killed the app**: `reset_presentation_document_effect` panicked inside the guest at boot (`thread '<unnamed>' panicked … artifact envelope terminal shell reached Drop …` followed by `setActiveExample refused: dispatch-failed — unreachable`), leaving the shell with **no panes and no window kinds**. Switched to `store::empty_document_spr`. Fault lines 7 → 1, and the trap is gone. |
| 11 | **new — missing owned-snapshot retirement factory** | `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:815` (`build_document_store_owners` / `_disposer`, `build_config_store_owners` / `_disposer`) | vcs installed a one-item preparation factory but **no `DocumentStoreOwners`**, so the store held no `initial_snapshot_retirement_factory` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:16086`). The first verb returning a snapshot read was refused, and that **poisoned the instance**: the other eight Actions-pane verbs all failed with `plugin.internal.prior-outcome` ("runtime live cleanup faulted for instance 1"). Fault lines 20 → 4, and the eight collateral refusals are gone. |
| 12 | same, animate | `…/🎞️animate/…/✏️editor/🦀️.rs:609` | identical missing owners; its boot archive load was refused with `module.vcs: validation failed: returned snapshot read requires its exact owned-snapshot retirement factory`. |

The two `{"tag":"fault","val":{"0":123,…}}` console lines are undecoded fault **bytes** reaching the
console as a byte map; decoding them is how #11 was found (`{"code":"plugin.internal.prior-outcome",…}`).
A probe that does not decode them reports nine unexplained refusals instead of one root cause.

Tests added: `set_active_example_is_declared_bridged_and_loads_the_document` and
`an_unknown_example_id_loads_nothing` in
`✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
— they assert the retained-catalogue rows, that **every** window kind inherits the app-level action,
the typed bridge, and that the effect is a `LoadDocument` carrying no `Mutation`.

## Gaps — what is honestly not done

- **Two workers died on this slice before reporting.** Everything above the "Re-measurement in progress"
  marker is the *predecessor's* prose; this worker inherited its tree edits but not its captures
  (`🗑️generated` was wiped twice). Any claim in the prose that is not backed by a row in the results
  table is unverified narrative.
- Gaps measured by this worker are listed here as they are found.

### Measured gaps

- **The probe this slice inherited could not have passed anything.** Three of its steps were wrong and
  each was proved wrong against the live architect server, not by reading:
  1. it clicked `framework.panel.history` unconditionally before undo — that control is a tab button
     that **toggles the side panel shut** when History is already active, so the undo control was gone;
  2. `framework.history.undo` is a tree ROW whose `control` slot holds the real `<button>`
     (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:9087`) —
     clicking the row is **silently inert**, so an app with perfectly working undo scored `undoWorks: false`;
  3. the undo row sits in a **collapsed** tree section whose header is `framework.history.actions`
     (not `framework.history.commands`, which controls the entry list), so it has a 0×0 box that
     `force: true` cannot click. The probe now resolves the expander through `aria-controls`.
  Also: the ledger does **not** rewind its cursor on undo — it appends an `undo` entry and flips
  `canRedo` — and the document witness had to be scoped to `[data-surface-id]` subtrees, because the
  History panel is itself a tree whose rows change on every undo.
- **The one blocker every batch-A plugin now ends on is framework-level, not per-app: `Effect::LoadDocument`
  never completes.** After fixes 7–12 the boot `setActiveExample` is declared, bridged, admitted, and
  then dies in the archive round-trip:
  `AppChannelClient.loadDocumentArchive(…): {"code":"plugin.internal","message":"document archive
  replacement failed closure, authority, or retained publication validation"}`
  (fault site `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:23317`; the replacement's
  `faulted` flag is set in ~12 places in `drive_member_open`, `🦀️.rs:19871–19973`, whose diagnostics
  are all `#[cfg(test)]`-only, so a headless probe cannot see which one). **Verified identical on
  `architect`, `animate` and `writer`** — and `writer` is the app with the full hand-written
  `ArtifactEnvelopeDecodeOwnerBundle` + `WriterStoreInitializationAuthority`
  (`✏️s/🔌️plugins/✒️writer/…/🚪️io/🧬️mutations/💾️binary/🦀️.rs:1107,1653`), so the generic
  `bounded_document_store_initialization_job` is **not** the difference and per-app authorship will
  not fix it. `writer` additionally produced a second, more specific shape on its *second*
  `setActiveExample`: `document archive genesis child projection failed: child restore projection:
  InvalidReference`. **Not fixed by this slice** — it needs the `#[cfg(test)]` diagnostics turned into
  a real harness, which is a framework slice, not a plugin one.
- **F9 is applied to `architect` only.** `mathematical`, `vcs` and `sequence` still log the
  undeclared-action **error** for the navbar's boot `setActiveExample`. I deliberately did not
  replicate F9 into them once the bullet above proved that doing so only converts a console *error*
  into the same console *warning* — the plugins cannot reach `loadsClean` either way until the
  archive lane is fixed, and the wiring is ~40 lines per app across eight call sites. `architect`'s
  version is the worked template (fix #8) for whoever picks this up after the framework fix.
- **`writer` cannot pass the interaction bar through the Actions pane by construction.** Its pane
  offers exactly three app verbs: `formatDocument` (inert on an unedited document), `lintDocument`
  (publishes to the `WindowTransient` lane, not the document) and `setActiveExample`. Its actual
  mutating verbs — `textEdit`, `setText`, `commitRename`, `toggleLineNumbers`, `setEditorSetting` —
  are editor-gesture-only and have no pane row. Either the bar needs a canvas/editor gesture for this
  app or writer needs a pane-reachable mutation; this is a product decision, not a bug I could fix.
- **`animate` boots with no window open at all** ("Drag windows from Display in the navbar, or restore
  a saved layout"), so `panes` and `windowKinds` are both empty and there is no Actions pane. Its
  default layout, not its dispatch chain, is what blocks it now.
- **`vcs`'s remaining fault is a fold-contract violation**: `incrementCounter` is refused with
  `batched item candidate failed its exact fixed fold contract` — its retained rows are not
  point-invertible (the law `📓️retained-rows-must-be-point-invertible` names). Unfixed.
- **`.vscode/launch.json` rows were NOT added, and the predecessor's claim that twelve were is false**
  — the file contains no `🏗️activate`/`🛰️serve` row at all, and neither does the seed. `launch.json`
  is a **generated** file (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts`:
  *"Never hand-edit `.vscode/launch.json` directly"*); the sanctioned path is to edit
  `.vscode/🧩️launch.seed.jsonc` and run `bun ./📜️script.ts generate`, which **also rewrites and prunes
  the entire plugin-registry catalog directory** (`📽️projection/🟦️.ts:375-386` deletes every file not
  in the expected set). I did not run that under a live fleet editing plugin metadata, because a
  half-landed peer edit would be deleted and `check`'s freshness gate would then fail for V1. Every
  variant already has its generated `🛠️dev<plugin>⚛️react` row; the activate/serve split this slice
  needs is reproducible from `📜️b1a-activate.sh` / `📜️b1a-serve.sh`.
- **Unit tests exist for `architect` only** (4/4 green). The `vcs`/`animate` store-owner fixes (#11,
  #12) are verified **at runtime only** (fault lines 20 → 4 and 7 → 1 on live servers). I wrote a
  `vcs` unit test asserting the owners are installed and **removed it again**: merely constructing
  `DocumentStoreOwners` in a test panics on `artifact store cursor disposer reached Drop before
  terminal-empty ownership` (`🏪️store/🦀️.rs:2168`), and the only ways around that are `mem::forget`
  or a hand-rolled close-step driver — a shim and a rabbit hole respectively. The honest record is
  that these two fixes have runtime evidence and no test.
- **A peer left a `[DEBUG]` eprintln in a shared framework file**:
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19456`. Not mine, not removed (live file).
- **The machine ran out of disk mid-slice** (119 MiB free on both volumes; every `cargo` in the fleet
  was failing with `no space left on device`). `⚡️cache/cargo` was 315 GiB, 88 GiB of it
  `build/debug/incremental`. The documented `-mindepth 2 -mmin +120` prune freed nothing — under a
  full fleet **no** session dir is older than 60 minutes. What worked, and is safe by construction:
  keep each crate's **newest** incremental session and drop the superseded ones older than 30 minutes
  (cargo holds exactly one live session per crate). 180 sessions, **16.7 GiB**, 119 MiB → 77 GiB free.
  All three of this slice's crates then rebuilt `--all-targets` green and architect's tests stayed 4/4.
- **Peer churn hit this slice twice and both times cleared on its own**: `semio-framework-dispatch-macros`
  was mid-edit and broke every crate for ~10 minutes (slice D1's `classify_return`), and a `satisfies`
  edit in `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` 500'd every React serve and threw
  `ReferenceError: useLabel is not defined` into the `mathematical` probe. Neither is a batch-A defect.
- **A second, separate defect surfaced while over-undoing**: undoing past an app mutation into the
  shell's own `shell.panelTab`/`shell.panelToggle` entries makes the guest re-dispatch those ids as
  *app* actions, which the shell then drops (`dropped action "shell.panelTab" … no window kind
  declares it`). The shell's own verbs should never be handed back to the app on undo. Not in this
  slice's scope; recorded for whoever owns the history lane.

## Method notes

- **The state witness must be the shell's undo ledger, not the DOM.** `[data-history-json]` is published
  on one element of the shell root and carries `cursor`/`entries`/`canUndo`/`canRedo`/`actionIds`. It
  records shell verbs too (`shell.panelToggle`, `shell.panelTab`, `shell.windowActivate`), so the witness
  filters those out; what is left is exactly the app-owned mutations, which is also precisely what undo
  retires. A DOM diff cannot tell a refused verb from a live one.
- **`framework.history.undo` only exists while the History panel tab is mounted.** The probe clicks
  `framework.panel.history` right after boot — *before* sampling the pre-action baseline, so that opening
  the panel is not itself scored as the document change the undo step must restore.
- **Dispatch refusals are `console.warn`, not `console.error`.** Only the *undeclared-action* drop is an
  error. A probe that filters on `error` reports a completely dead plugin as clean.
- **An argument-less action row is its own trigger.** No `.execute` control is ever rendered for it, so
  clicking the row to "expand" it already dispatches — and clicking it again to collapse dispatches a
  second time. Every fault line in these captures therefore appears twice per dead verb.
- **Do not use `bun ./📜️script.ts dev <variant>`.** It runs `nx watch --all` and re-activates on every
  peer edit, taking the Vite server down mid-probe. `📜️b1a-activate.sh <variant>` then
  `📜️b1a-serve.sh <variant> <port>` (detached) is the split this slice used, same as B1b's.
- **Probes must live under the ticket folder.** `bun` resolves `import { chromium } from "playwright"`
  against its own global cache for a script outside the repo, and that cache wants a Chromium revision
  that is not installed.

## Launch entries

> ⛔️ **This section is the predecessor's and is FALSE.** `.vscode/launch.json` contains no
> `🏗️activate` or `🛰️serve` row (`grep` finds zero, in the file and in the seed), and it is a
> **generated** file that must never be hand-edited. See the launch bullet in Gaps for what the
> sanctioned path costs and why this worker did not take it. The port table below is still correct
> and is what the `📜️b1a-serve.sh` invocations use.

| plugin | activate row | serve row | port |
|---|---|---|---|
| ✒️writer | `🏗️activate✒️writer⚛️react` | `🛰️serve✒️writer⚛️react` | 6062 |
| ➗️mathematical | `🏗️activate➗️mathematical➗️equation⚛️react` | `🛰️serve➗️mathematical➗️equation⚛️react` | 6084 |
| 🌿️vcs | `🏗️activate🌿️vcs⚛️react` | `🛰️serve🌿️vcs⚛️react` | 6075 |
| 🎞️animate | `🏗️activate🎞️animate🎬️presentation⚛️react` | `🛰️serve🎞️animate🎬️presentation⚛️react` | 6051 |
| 🎬️sequence | `🏗️activate🎬️sequence⚛️react` | `🛰️serve🎬️sequence⚛️react` | 6077 |
| 🏛️architect | `🏗️activate🏛️architect🏛️program⚛️react` | `🛰️serve🏛️architect🏛️program⚛️react` | 6090 |

All six ports were already assigned to these plugins by the existing `*_PLAY_PORT` rows in the same file
(`WRITER_PLAY_PORT` 6062, `MATHEMATICAL_PLAY_PORT` 6084, `VCS_PLAY_PORT` 6075, `PRESENTATION_PLAY_PORT`
6051, `SEQUENCE_PLAY_PORT` 6077, `ARCHITECT_PLAY_PORT` 6090), so no new port was invented and none
collides with another row.

## Files changed

Production files touched by this slice (predecessor's inherited edits + this worker's):

| file | slice content |
|---|---|
| `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | F3 bridge, F9 `setActiveExample` + document-store init |
| `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | F3 bridge |
| `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | F3 bridge, F9, F1 promotions |
| `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | F3 bridge |
| `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | F1 promotions + artifact-store factory |
| `.vscode/launch.json` | twelve additive activate/serve rows (see Launch entries) |

Ticket-folder tooling (not production): `📜️b1a-activate.sh`, `📜️b1a-serve.sh`, `🐍️b1a-boot-probe.mjs`
and the six `🐍️b1a-<plugin>-boot-probe.mjs` shims.

### This worker's files

| file | change |
|---|---|
| `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | fixes #7 (`reset_document_effect`), #8 (F9 wiring, 8 call sites), #9 (`build_document_store_initialization_job`) |
| `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📚️example/🦀️.rs` | **new** — architect's `set_active_example` command module |
| `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🦀️.rs` | `pub mod example` path row for the new command module |
| `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | two new tests |
| `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | fixes #10 (guest trap), #12 (store owners + initialization job) |
| `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | fix #11 (document + config store owners and disposers) |

Ticket-folder tooling this worker changed or added: `🐍️b1a-boot-probe.mjs` (the three witness fixes
above), `🐍️b1a-dom-dump.mjs`, `🐍️b1a-undo-diagnose.mjs`, `🐍️b1a-collapsible-html.mjs` (**new**, the
three one-off diagnostics that found them). No `.vscode` file was touched — see Gaps.

## How to reproduce any row

```
cd /Users/ueli/Documents/semio
zsh  .🧬semio/…/OS-HUB-COLLABORATION-AI-END-TO-END/📜️b1a-activate.sh <variant>
nohup zsh .🧬semio/…/📜️b1a-serve.sh <variant> <port> >/dev/null 2>&1 & disown
bun  .🧬semio/…/🐍️b1a-<variant>-boot-probe.mjs
```

`<variant> <port>`: `architect 6090` · `writer 6062` · `mathematical 6084` · `vcs 6075` ·
`animate 6051` · `sequence 6077`.

## Environment

Host saturated throughout: peer `cargo`/`rustc` live at every sample (`semio-framework-os-renderer-wgpu`,
`stdio-pdf`, two peer `cargo test` runs), a peer `activate-flow-react-dev` competing for the same Nx
graph, six peer Vite servers, and a repo-wide `🎚️options` → `☑️options` rename landing mid-slice. One
cargo at a time, always `-p <crate>`, `CARGO_PROFILE_WASM_DEV_DEBUG=false`, `NX_DAEMON=false`. No peer
process was killed. Headless Chromium with `--use-angle=metal`; the desktop browser pane was never used.
