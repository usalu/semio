# 🧩️ Why puzzle3d is not working end to end

## The headline

Puzzle3d's UI actions are hard-rejected by the framework before they reach any handler. This is not
a bug in puzzle3d's logic — it is an unfinished Phase-8 migration.

`validate_ui_dispatch_classification`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:11915`):

```rust
if classification == InteractiveJobClassification::Migrated { Ok(()) }
else { Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.not-ui-safe"), …)) }
```

It is called from `dispatch_action` at `:21872`, **before command construction**. So any action whose
declared `InteractiveJobClassification` is not `Migrated` cannot be invoked from the UI at all.

Puzzle3d declares 67 actions. **61 are `BatchOnlyPendingRewrite`; only 6 are `Migrated`**
(`openAddObjectDialog`, `setLocale`, `setTerminology`, `worldPointerDown`, +2). Among the dead ones,
directly on the goal:

| Action | Line | Classification | Consequence |
|---|---|---|---|
| `setActiveExample` | 7038 | BatchOnlyPendingRewrite | example switching is dead — windows cannot change example |
| `setFillCount` | 7042 | BatchOnlyPendingRewrite | the fill tool's slider is dead |
| `fillBuildTick` | 7027 | BatchOnlyPendingRewrite | fill background planning never advances |
| `duplicateSelection`, `deleteSelection`, `translate/rotate/scaleSelection`, all `engagement*`, … | 7010+ | BatchOnlyPendingRewrite | dead |

The enum (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:835`) is documented as the "Phase-8 migration
disposition"; `BatchOnlyPendingRewrite` means the action's implementation still has to be rewritten
into the migrated interactive-job form. Flipping the label without doing that rewrite would be
dishonest — the gate exists precisely to keep un-migrated actions out of the UI.

## Repo-wide context — puzzle3d is the least-migrated app, and the migration is well precedented

| App | Migrated | BatchOnly | % migrated |
|---|---|---|---|
| lowpoly | 48 | 0 | **100%** |
| cad | 32 | 17 | 65% |
| gen3d | 24 | 6 | 80% |
| flow | 23 | 16 | 58% |
| sequence | 20 | 0 | 100% |
| writer | 19 | 0 | 100% |
| puzzle5d | 11 | 41 | 21% |
| puzzle2d | 6 | 34 | 15% |
| **puzzle3d** | **6** | **61** | **8%** |

Repo-wide: 427 `Migrated`, 414 `BatchOnlyPendingRewrite`, 7 `Unclassified`, 3 `ForbiddenFromUi`.
23 files are fully migrated. So this is a half-finished repo-wide effort with several complete
reference implementations to copy — `💠️lowpoly` (48/48) is the cleanest.

## Second, independent runtime blocker

Even before dispatch, every actor turn faulted with
`{"origin":"plugin","code":"plugin.internal","message":"runtime live cleanup faulted for instance 1"}`.
Traced (see `📓️blocker-stdio-mutation-leaf-ownership.md` for the capture) to
`EditorApp::maintenance_step` → `store.take_returned_snapshot_read_retirement()`
(`🏪️store/🦀️.rs:14534`), which returns
`Err(ValidationFailed("snapshot read retirement factory is not installed"))` when a snapshot read
lease has been returned but `snapshot_retirement_factory` is `None`. `ArtifactStore::from_new`
(`:13803`) leaves it `None`; `from_initialized_runtime_with_owners` (`:13845`) sets it. Whether the
puzzle3d open path installs it is under active investigation.

That fault is measured against the Sep 1 prebuilt wasm; it must be re-confirmed against a fresh
build, since `🔌️plugin/🦀️.rs` was rewritten twice on Sep 2.

## What "working end to end" therefore requires

1. A build that runs at all — done (see the other note: stdio, graph, puzzle value-derive, launch
   seed and taxonomy fixes, all landed and verified).
2. The live-cleanup fault fixed, so turns stop faulting.
3. Puzzle3d's 61 actions migrated to `Migrated` — honestly, i.e. with the rewrite the disposition
   promises — at minimum `setActiveExample`, `setFillCount` and `fillBuildTick` for the goal as
   stated.

## Note on "all windows with different examples"

Puzzle3d's edit mode defines ONE window kind, `puzzle3d-main`
(`✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:30`), and the default layout opens TWO instances
of it: `puzzle3d-main-top` (orthographic Top) and `puzzle3d-main-perspective`
(`✏️editor/🎭️modes/✏️edit/🦀️.rs:31-42`) — matching the "Top" / "Perspective" chips visible in the
running shell.

The active example is GLOBAL, not per window: it lives in `Puzzle3dScene.fixture`
(`✏️editor/🦀️.rs:320`) and `setActiveExample` (`:4662`) mutates it for the whole document. Both
window instances render from that one fixture. puzzle2d and puzzle5d are the same. So two windows
showing two DIFFERENT examples at once is not expressible today — it would need per-window example
state. Reading the goal as "every window renders correctly, and both examples work" is satisfiable;
reading it as "each window shows a different example simultaneously" is a design change. Flagged for
the dev to choose.
