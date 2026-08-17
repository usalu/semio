# W2-END — Report

Lane: W2-END, ticket `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET`. Closes the last gap so
`🎛️apps` can be dissolved repo-wide (W3).

## Job 1 — space's `studio` app: the decision

Read `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🦀️component.rs` (887 lines: `SpaceApp: ArtifactApp`, 38
commands across parameter editing, media-port routing, app spawning/instancing, workflow-graph
editing, a compiled-DAG view, a VFS browser, pack/DSL import/export, real convergence + VCS
round-trip tests) and everything around it before deciding.

**Neither option as literally offered fits the evidence cleanly, so I resolved it from what the code
and its own history actually show, not from the two pre-written options' exact wording.**

### Ownership: (a) is disqualified

`SpaceApp::Snapshot`/`::Mutation` are `semio_framework_os::{WorkflowSnapshot, WorkflowMutation}`.
Dispatched a research pass (not from convenience — verified, not assumed) that traced these to their
real definition: `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs`, schema id
`"os.workflow"` (`S_WORKFLOW_SCHEMA`). Two already-closed tickets
(`26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📓️w4b-workflow.md` and `📓️w4b-space.md`) record a
**deliberate, recent** rename of this schema's namespace from `s.*` to `os.*` specifically to mark
`workflow`/`space`/`collection` as OS-owned "peer kernel crates," with the framework module's own doc
comment stating outright they are seeded as OS builtins "because they are not any single app's
document format." The framework has **no** `🗿️artifacts/` taxonomy tree of its own anywhere — that
convention is plugin-only. `SpaceApp` is the sole plugin-side consumer of these types repo-wide.

Inventing `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/` with `Dialect { artifact_kind: "s.space.space",
… }` would misrepresent this: the grammar's own convention (`s.<plugin>.<artifact>`) asserts plugin
ownership of a document that two other, already-landed tickets went out of their way to mark as
explicitly NOT plugin-owned. (a) is rejected on the evidence, not on convenience.

### Literal (b) ("must be deleted") does not survive contact with the code either

Reading further ruled out simple deletion too:

- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`'s
  own doc comment (written by the already-completed, coordinator-reviewed W2 packet P7) explicitly
  calls `🪐️space` "**the sibling `🪐️space` studio app**" and states that shared catalog/backbone
  helpers moved to the plugin root because they are "genuinely shared by 3 surfaces now (this editor,
  the new `👁️viewer`, and the sibling `🪐️space` studio app's own commands)" — i.e. P7 treated studio
  as a permanent, load-bearing sibling, not dead code pending removal.
- Home's own editor commands (`🏙️create-studio`, `🏙️open-space`, `🏙️import-space`,
  `🏙️bind-space-file`) are pure launchers: each does catalog bookkeeping then
  `HostEffect::Navigate { uri: "/spaces/{id}" }`. There is nothing on the other end of that route
  without studio — deleting it turns every one of home's own commands into a dead-end navigation,
  which is not "getting everything working."
- "Fold into home's editor" is structurally impossible for the bulk of studio's behavior: home's
  `ArtifactEditor::Mutation` is `SHomeMutation` over `SHomeSnapshot` (a launcher catalog); studio's 38
  commands mutate `WorkflowMutation` over `WorkflowSnapshot` (a workflow graph) — two incompatible
  document types. There is no command that can move from one to the other without home acquiring a
  second, unrelated document, which is its own kind of taxonomy violation.
- Studio is not vestigial: it carries real, passing tests including a two-instance backbone
  convergence test (`two_instances_converge_on_disjoint_edits_via_backbone`) proving disjoint concurrent
  edits reconcile — exactly the CQRS/event-sourcing property this codebase's own CLAUDE.md mandates.
  Deleting ~90 files of tested, working product behavior with no replacement is not a refactor, it's a
  regression, and nothing in what I read supports it being intentional.

### What I actually did: relocate, don't delete or fake-own

`🪐️space` (studio) **stays a plain, non-surface `ArtifactApp`** — no `Dialect`, no
`🗿️artifacts/🪐️space` node, no editor/viewer pair, registered exactly as before via
`.document_app::<SpaceApp>(create_space_app())` + `.foreign_document_codec::<SpaceApp>(OS_SPACE_SCHEMA)`.
This is the correct reading of "not a per-subset surface at all" — it structurally cannot be one
without a legitimate owned `Dialect`, and inventing one would be dishonest.

What had to change, mechanically, is only that it cannot keep living under a directory literally
named `🎛️apps` (W3's dissolution gate). Followed the one precedent this very ticket already set for
exactly this situation — packet P7b moved `🏗️fem`'s shared, non-surface `🖥️app-surface` module from
`🎛️apps/…` to a new plugin-root `⚙️engine/` facet, reported clean. Applied the same move to studio:

- `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/` (90 files) → `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/`
  (`mv`, content byte-identical).
- Every internal reference `crate::apps::space::…` → `crate::engine::space::…` across all 90 moved
  files (mechanical, verified 0 stray `apps::space` left anywhere in the plugin afterward).
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs`: `//#region 🎛️Apps` → `//#region ⚙️Engine`,
  `pub mod apps { pub mod space { … } }` → `pub mod engine { pub mod space { … } }`, all 55
  `#[path = "../../🎛️apps/🪐️space/…"]` → `#[path = "../../⚙️engine/🪐️space/…"]` (including the
  `📚️Examples` region's `app_space_demo_session` mount, which the mechanical region-replace initially
  missed and I fixed as a second pass), plus the one-line `📚️Examples` mount. Verified all 110
  non-`"."` `#[path]` attrs in the file resolve on disk (script-checked, 0 missing — same verification
  script the cad pilot recipe specifies).
- `✏️s/🔌️plugins/🪐️space/🦀️component.rs`: `.document_app::<crate::apps::space::SpaceApp>(…)` →
  `.document_app::<crate::engine::space::SpaceApp>(…)` (2 real refs), plus a doc-comment fix.
- 4 cosmetic doc-comment fixes in `✏️s/🔌️plugins/🖍️draw/…` (outside my lease, one-line-each, safe)
  that referenced `apps::space`'s `open_space` by name.
- Top-of-file module doc in `📦️glue.rs` rewritten to record this reasoning in place (so the next
  reader doesn't have to re-derive it).

Net effect: **zero functional change** to studio — same types, same commands, same tests, same
registration mechanism — only its module path and containing directory moved out of the retired
`🎛️apps` taxonomy concept.

### Known gap, flagged not fixed

`⚙️engine` is not in `🔣️taxonomy.json`'s `pluginChildDirs` (`["🎛️apps", "🎮️commands"]` today) as a
sanctioned plugin-root facet — `🏗️fem`'s own instance of this same pattern (P7b) is the only other
one in the repo and is equally unsanctioned in the taxonomy file itself, just filesystem-precedented
and policy-clean in practice. This is a taxonomy-vocabulary gap for the W0/coordinator layer (a
`pluginChildDirs` addition, or a purpose-built facet name for "plugin-root non-surface app content"),
not something this plugin-scoped lane can invent unilaterally. Flagging it here rather than silently
extending taxonomy.json myself.

## Job 2 — delete the `🎛️apps` grouping files

- `✏️s/🔌️plugins/🔋️energy/🎛️apps/🦀️component.rs` — one-line doc-only stub, mounted via
  `pub mod plugin_apps;` in `📦️glue.rs` (dead, no real content). Removed the mount and the file/dir.
- `✏️s/🔌️plugins/🗄️stdio/🎛️apps/🦀️component.rs` — one-line doc-only stub, not mounted anywhere
  (verified via repo grep before deleting). Removed the file/dir.
- `✏️s/🔌️plugins/🎪️demonstrator/🎛️apps/` — confirmed empty of files (`find` returned only the
  directory itself). Removed the directory.
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🦀️component.rs` — same one-line doc-only stub as energy/stdio's, not
  mounted anywhere. Removed once the `🪐️space` app subtree above was relocated out from under it,
  leaving the directory empty; then removed the directory.

## Verification

### `find ✏️s/🔌️plugins -type d -name '🎛️apps' | wc -l`

```
0
```

### `cargo check`, serial, `RUSTC_WRAPPER="" … --all-targets --keep-going`

All three runs blocked before reaching my lease's own crate — `semio-s-plugin-stdio` (a dependency of
all three) fails to compile with ~125-128 errors, confirmed **live, uncommitted, in-progress peer
work**: `git status --porcelain` shows dozens of modified stdio files right now, and
`git log --date=iso` shows a same-day commit (2026-08-16 12:10:56) on top of further uncommitted
changes on top of that. Matches this ticket's own documented "known-broken by live peers" case for
`semio-s-plugin-stdio`.

| crate | cargo exit | errors touching my lease's own files | reached "Checking" line |
|---|---:|---:|---|
| `semio-s-plugin-space` | 101 | 0 (grep-verified against the log) | no — blocked upstream on stdio |
| `semio-s-plugin-energy` | 101 | 0 | no — blocked upstream on stdio |
| `semio-s-plugin-demonstrator` | 101 | 0 | no — blocked upstream on stdio |

Full combined output: `🧪️w2-end-cargo.txt` (35k+ lines, three runs concatenated with `===` headers).

### Live-filesystem policy check (scoped to `🔋️energy`, `🗄️stdio`, `🪐️space`, `🎪️demonstrator`)

Imported and called `policySubsetSurfaceCompletenessBreaches`, `policyViewerPurityBreaches`,
`policyContributedSurfaceTargetBreaches`, `policyOsConfigShapeBreaches` directly from `📜️script.ts`
against the live filesystem (not the cached `bun ./📜️script.ts policy` snapshot, per this repo's own
standing note that the CLI reads a cache):

```
surface-completeness breaches (4 plugins): 0
viewer-purity breaches (4 plugins): 0
contributed-surface-target breaches (4 plugins): 0
os-config-shape breaches (repo-wide): 0
```

Scratch script: `🧪️w2-end-check-policy.ts` (copied into this folder from the scratchpad for the
record).

## Files touched

Deleted:
- `✏️s/🔌️plugins/🔋️energy/🎛️apps/` (whole dir, one stub file)
- `✏️s/🔌️plugins/🗄️stdio/🎛️apps/` (whole dir, one stub file)
- `✏️s/🔌️plugins/🎪️demonstrator/🎛️apps/` (empty dir)
- `✏️s/🔌️plugins/🪐️space/🎛️apps/` (whole dir, including the relocated `🪐️space` subtree — see below)

Created (relocated, content unchanged):
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/**` (90 files, moved from `🎛️apps/🪐️space/`)

Edited:
- `✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📦️glue.rs` (removed `plugin_apps` mount)
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs` (`🎛️Apps` region → `⚙️Engine` region, all
  paths repointed, top-of-file doc comment rewritten)
- `✏️s/🔌️plugins/🪐️space/🦀️component.rs` (`apps::space::` → `engine::space::` in the two real
  `.document_app()`/`.foreign_document_codec()` refs + one doc comment)
- 55 files under the relocated `⚙️engine/🪐️space/` tree (mechanical `apps::space` → `engine::space`
  module-path rename, no behavior change)
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/{📄️commit-document,📄️set-active-example,📄️set-fixture-json,📄️set-snapshot}/🦀️component.rs`
  (cosmetic doc-comment fix, one line each, outside my lease but trivial)

Scratch (ticket folder, `.txt`): `🧪️w2-end-cargo.txt`. Scratch (ticket folder, `.ts`, kept for the
record): `🧪️w2-end-check-policy.ts`.

## Not done / follow-ups

1. `⚙️engine` as a plugin-root facet has exactly two filesystem instances (`🏗️fem`, now `🪐️space`) and
   zero entries in `🔣️taxonomy.json`'s `pluginChildDirs`. W0/coordinator should either add it there
   or mint a purpose-built name for "plugin-root non-surface app content" before W3 locks the taxonomy
   shape further.
2. Re-run all three `cargo check`s once `semio-s-plugin-stdio`'s live in-progress work lands — 0 errors
   were found in any of my lease's own files in this pass, but none of the three crates were actually
   reached by the compiler yet.
