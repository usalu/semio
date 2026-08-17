# W2 Packet P5 (flow + procedural) — Report

Lane: W2 packet P5. Plugins `✏️s/🔌️plugins/🌊️flow/**` and `✏️s/🔌️plugins/🌀️procedural/**`. Recipe followed:
`📓️w2-cad-report.md`'s 16-step migration recipe. Contract: `📋️contract-freeze.md` §1, §2, §2.6. SDK gaps
from `📓️w0-f-report.md`/`📓️w2-fix-report.md` confirmed already closed — every packet in this lane used
bare `semio_framework_plugin::{ArtifactEditor, ArtifactViewer, Editor, Viewer, EditorApp, ViewerApp,
ViewEmit, Dialect, StandardId, SubsetId, MeshWindowKit, MeshView, WindowKit}` throughout.

Execution: four parallel foreground sub-agents, one per disjoint tree (flow's single app; procedural's
`◻2d`; procedural's `🧊️3d`; procedural's brand-new `🧩️assembly`), then this session did the shared
wiring (`📦️glue.rs` ×2, plugin root `🦀️component.rs` ×2, `Cargo.toml` cosmetic fix, `🎛️apps` deletion
×2, cross-plugin referrer fix) and ran the verification gates, per the packet brief's own recommended
execution order.

## Sub-agent reports (full detail, not duplicated here)

- `📓️w2-p5-flow-notes.md` — flow's single app (2 modes, 5 windows) migrated to editor + real viewer authored.
- `📓️w2-p5-procedural2d-notes.md` — procedural2d's `◻2d` app (2 modes, 5 windows) migrated to editor + real viewer authored.
- `📓️w2-p5-procedural3d-notes.md` — procedural3d's `🧊️3d` app (2 modes, 5 windows) migrated to editor + real `MeshWindowKit` viewer authored.
- `📓️w2-p5-assembly-notes.md` — assembly's editor + viewer authored FROM SCRATCH (no prior app), including the artifact-root `component.rs` (`artifact_kind()`/`definition()`/`ASSEMBLY_DIALECT`) that didn't exist before this packet. **Also documents a real blocking gap** (assembly's schema tree is missing the artifact-facet descriptor + non-Rust/TS leaf files needed for `declaration()` — see below).

## Coordinator wiring (this session)

### `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📦️glue.rs`

Old `//#region 🎛️Apps` (mounting `apps::flow::*` from `../../🎛️apps/🌊️flow/…`) split into `//#region
✏️Editor` (`pub mod editor { pub mod flow { … } }`, mounted from
`../../🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/…`) and `//#region 👁️Viewer`
(`pub mod viewer { pub mod flow { … } }`, same base but `…/👁️viewer/…`), built by taking the old
apps-region text verbatim and doing a scoped path substitution (editor tree is byte-identical in
internal structure to the old apps tree) plus a hand-built viewer region (much smaller — one mode, one
window). The `📚️Examples` region's `app_flow_demo_session` mount repointed at the new editor path.
Disk-verification script (from the cad pilot) run against the WHOLE file: **116 `#[path]` attrs, 0
missing.**

### `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs`

Old `//#region 🎛️Apps` (mounting `apps::procedural2d`/`apps::procedural3d`) split the same way into
`//#region ✏️Editor` / `//#region 👁️Viewer`, covering `procedural2d` and `procedural3d` fully. The
`🗿️Artifacts` region's pre-existing `pub mod assembly { pub mod standards { … } }` block (assembly's
schema was already mounted before this ticket) gained a new artifact-root mount at its top
(`#[path = "../../🗿️artifacts/🧩️assembly/🦀️component.rs"] mod component; pub use component::*;`) —
this is **safe and mounted** (no trait-bound requirement on free functions/consts). Assembly's
**editor and viewer sub-modules were NOT mounted** — see "Assembly: confirmed blocking gap" below; the
region carries an explanatory comment instead, pointing at `📓️w2-p5-assembly-notes.md`. `📚️Examples`
region's `app_2d_demo_session`/`app_3d_demo_session` mounts repointed. Disk-verification script run
against the WHOLE file: **312 `#[path]` attrs, 0 missing.**

### Plugin roots

`✏️s/🔌️plugins/🌊️flow/🦀️component.rs`: `.document_app::<crate::apps::flow::FlowPlayApp>(…)` →
`.editor::<crate::editor::flow::FlowPlayApp>(create_flow_app())` +
`.viewer::<crate::viewer::flow::FlowViewer>(create_flow_viewer())`. Added `#[cfg(test)] mod
surface_tests` calling the REAL `semio_framework_plugin::testkit::{assert_viewer_never_mutates,
assert_editor_and_viewer_share_dialect}` (no local stand-ins needed, per `📓️w0-f-report.md`).

`✏️s/🔌️plugins/🌀️procedural/🦀️component.rs`: two `.document_app::<…>` calls → four
`.editor()`/`.viewer()` calls (procedural2d, procedural3d). Assembly's `.editor()`/`.viewer()` calls
were added and then **removed again** once a real `cargo check` proved they don't compile yet (see
below) — the file carries an explanatory comment instead. Also fixed, in the SAME edit: two
pre-existing bugs unrelated to this ticket's migration that blocked this very file from compiling —
`.artifact(crate::artifacts::procedural2d::declaration())` /
`.artifact(crate::artifacts::procedural3d::declaration())` were missing the
`.map_err(semio_framework_plugin::PluginAssemblyError::definition)?` unwrap every other already-migrated
plugin's root carries (declaration() returns a `Result`, `.artifact()` takes the bare value — confirmed
by reading `PluginBuilder::artifact`'s real signature, not assumed); and
`.host_media_handler(…, crate::apps::procedural3d::procedural3d_document_from_mesh)` still pointed at
the now-deleted `apps` path (fixed to `crate::editor::procedural3d::…`). Added `#[cfg(test)] mod
surface_tests` for procedural2d and procedural3d only (assembly's tests would not compile either, same
reason).

### Fixes inside sub-agent-authored files (this session, after re-running `cargo check`)

Two genuine migration bugs in flow's own new files, found by a real `cargo check` run and fixed:
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs` —
  a `paired_apps::<FlowPlayApp>(...)` test call was missed by the agent's mechanical
  `VcsArtifactApp`/`new_app` fallout pass; needs the `EditorApp<...>` adapter like every other such
  call in the same file. Fixed to `paired_apps::<EditorApp<FlowPlayApp>>(...)`.
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️component.rs` —
  a test asserted `def.role == semio_framework::AppRole::Viewer`, but `semio_framework` (bare) is not a
  direct Cargo dependency of this crate (only `semio-framework-plugin`, which re-exports `AppRole` via
  its own blanket `pub use semio_framework::*;`). Fixed to `semio_framework_plugin::AppRole::Viewer`
  (the assembly sub-agent independently hit and self-fixed the identical pattern in `energy.model`'s
  viewer, confirming this is a real, recurring, not-yet-curated import trap, not a one-off).

Two genuine migration bugs in procedural3d's own files, found the same way:
- `create_procedural3d_app().definition` — `create_procedural3d_app()` now returns `AppDefinition`
  directly (not the old `App { definition, examples }`), so the `.definition` field access in
  `the_manifest_stitches_every_taxonomy_node` no longer compiles. Fixed by dropping `.definition`.
- `assert_two_instances_converge::<Procedural3dPlayApp, …>` — this testkit fn requires `A: ArtifactApp`
  (the runtime trait), which only the `EditorApp<E>` adapter implements, not the authoring trait
  implementor directly. Fixed to `assert_two_instances_converge::<semio_framework_plugin::EditorApp<Procedural3dPlayApp>, …>`.

Cosmetic doc-comment fixes (no compile dependency): `Cargo.toml:69` (`🎛️apps/🧊️3d` → the procedural3d
editor's actual location); two stray `crate::apps::procedural3d::` doc mentions in
`🗿️artifacts/🧊️procedural3d/🦀️component.rs` and its `🧬️schema/🦀️component.rs` (the latter is under
live concurrent peer edit — only this one doc line touched, the peer's `MutationOutcome` work left
alone, confirmed via a targeted `Edit`, not a whole-file rewrite).

### Outside-lease referrer fixed

`✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️component.rs` had a REAL compile dependency
on procedural3d's now-deleted `apps::procedural3d` path (`use procedural::apps::procedural3d::{…}`,
`.document_app::<Procedural3dPlayApp>(…)`) — fixed to `procedural::editor::procedural3d::{…}` and
`.editor::<Procedural3dPlayApp>(…)`, same pattern `📓️w0-f-report.md`/`📓️w2-fix-report.md` already used
for cad/puzzle3d. Two test literals (`bundle_registers_its_own_and_the_six_foreign_demonstrator_surfaces`,
`contribution_consumers_declare_the_hidden_app_command`) and one `Cargo.toml` playground metadata row
updated from the old hand-written `"procedural3d-play"` to the now-derived `surface_app_id` value
`"s.procedural.procedural3d@1/*#editor"` (verified against `PROCEDURAL3D_DIALECT`'s real value, not
assumed). No other outside-lease referrer found repo-wide for either plugin (`grep -rln
"apps::flow::\|apps::procedural2d::\|apps::procedural3d::"` across the whole repo, excluding these two
plugins' own trees: one hit, inside an unrelated old ticket's scratch `main.rs`, not part of the
compiled workspace).

### Deletion

`✏️s/🔌️plugins/🌊️flow/🎛️apps/` and `✏️s/🔌️plugins/🌀️procedural/🎛️apps/` removed in full, once every
real file had a real destination and both plugins' glue.rs/plugin-root/demonstrator referrers were
confirmed fixed (grep for `crate::apps::flow`/`apps::procedural2d`/`apps::procedural3d` across each
plugin's own tree: 0 hits post-deletion, re-checked).

## Assembly: confirmed blocking gap (not fixed — verified by actual compile attempt, not just theory)

The assembly sub-agent's own report flagged, from reading the SDK's typestate builder, that
`declaration()` cannot be authored because assembly's schema tree is missing its artifact-facet
descriptor and the JSON-Schema/GraphQL/Protobuf leaf files every other migrated artifact's schema
carries (only `.rs`/`.ts` leaves exist for assembly's `📸️snapshot`/`🔺️diff`/`🧬️mutations`). This
session went one step further and actually tried mounting assembly's editor+viewer into `📦️glue.rs`
and wiring `.editor()`/`.viewer()` on the plugin root — a real `cargo check -p semio-s-plugin-procedural`
then failed with:

```
AssemblySnapshot: ArtifactDsl` is not satisfied
AssemblySnapshot: ArtifactPack` is not satisfied
AssemblyMutation: OpBinary` is not satisfied
AssemblyMutation: OpText` is not satisfied
AssemblyEditorCommand: OpBinary` is not satisfied
```

anchored directly at `impl ArtifactEditor for AssemblyEditor` / `impl ArtifactViewer for
AssemblyViewer`. Reading `ArtifactEditor`/`ArtifactViewer`'s own trait definitions
(`🔌️plugin/🦀️component.rs:12991-13011`, `:13136-13151`) confirms these bounds
(`Snapshot: ArtifactDsl + ArtifactPack`, `Mutation`/`Command`: `OpText + OpBinary`) are hard,
structural requirements — not a wiring choice. Since `ArtifactDsl`/`ArtifactPack`/`OpText`/`OpBinary`
are exactly the traits the missing schema leaves would derive, this is the SAME root cause the
sub-agent already identified, now confirmed by a real compiler error rather than only the typestate
reading. **Decision made this session**: back out the mount (glue.rs regions carry an explanatory
comment instead of the assembly sub-modules; plugin root carries a comment instead of the `.editor()`/
`.viewer()` calls) rather than ship a crate that doesn't compile. Assembly's editor+viewer source files
remain complete and correct on disk, ready to mount the moment a follow-up ticket authors the missing
schema-facet leaves (recommended by the sub-agent: a new, narrowly-scoped schema/protocol-authoring
ticket using `energy.model`'s real, complete schema tree as the template — not bundled into a W2
surface packet). Assembly's `artifact_kind()`/`definition()`/`ASSEMBLY_DIALECT` (free functions/consts,
no trait bounds) ARE mounted and compile cleanly.

## Dialects (verified against each artifact's own `definition()` capability row, none guessed)

| subset | `artifact_kind` | canonical surface ids |
|---|---|---|
| `s.flow.flow` | `"s.flow.flow"` (matches `s.flow.schema.artifact` capability descriptor) | `s.flow.flow@1/*#editor` / `#viewer` |
| `s.procedural.procedural2d` | `"s.procedural.procedural2d"` (matches `s.procedural2d.schema.artifact`) | `s.procedural.procedural2d@1/*#editor` / `#viewer` |
| `s.procedural.procedural3d` | `"s.procedural.procedural3d"` (matches `s.procedural3d.schema.artifact`) | `s.procedural.procedural3d@1/*#editor` / `#viewer` |
| `s.assembly` (authored, not wired — see above) | `"s.assembly"` (the schema's own pre-existing `#[artifact_schema(id = "s.assembly")]`, NOT the packet brief's illustrative `"s.procedural.assembly"` guess — the assembly sub-agent grepped the real schema tree first, per the brief's own escape clause) | `s.assembly@1/*#editor` / `#viewer` (once wired) |

## Verification run

### Cargo check

`RUSTC_WRAPPER="" cargo check -p semio-s-plugin-flow --all-targets --keep-going` and the same for
`semio-s-plugin-procedural`, run serially, three rounds each as fixes landed. Full output in
`🧪️w2-p5-flow-cargo-short{1,2,3}.txt` / `🧪️w2-p5-procedural-cargo-short{1,2,3}.txt`.

- **Flow, own files: 0 errors** after the two fixes above (confirmed: every remaining error's `file:line`
  anchor grepped and cross-checked — none inside `✏️editor/`/`👁️viewer/` content this packet authored).
- **Procedural, own files: 0 errors** after the fixes above (same check, across `editor::procedural2d`,
  `editor::procedural3d`, `viewer::procedural2d`, `viewer::procedural3d`).
- **Two categories of remaining errors, both confirmed NOT caused by this packet:**
  1. **Pre-existing, repo-wide, predates this ticket touching flow/procedural at all.** Every one of
     flow's ~10 schema mutation-kind files (`🧬️mutations/*/🦠️mutation/🦀️component.rs`), plus its
     `🎚️config`/`👥️presence` Mutation impls, plus the equivalent set in procedural2d/procedural3d/
     assembly's own schema trees, still implement `MutationKind::diff(...)  -> Diff` (bare) instead of
     `-> protocol::MutationOutcome<Diff>` — the shape the MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-
     CLASS-CONFLICTS ticket's own commit (`5a1367d`, already at HEAD) requires. Verified pre-existing,
     not migration-caused, three ways: (a) `git show HEAD:"…/🎛️apps/🌊️flow/🎚️config/🦀️component.rs"`
     (the file's location BEFORE this packet moved it) shows the identical bare-`FlowConfig` signature
     — the move carried the bug forward unchanged, it didn't introduce it; (b) cad's own equivalent
     files (`✏️s/🔌️plugins/📐️cad/…/🧬️mutations/*/🦠️mutation/component.rs`,
     `…/✏️editor/🎚️config/component.rs`) are ALL already `MutationOutcome`-shaped, proving the sweep
     landed for cad but was simply never run against flow's or procedural's plugin trees; (c) none of
     the four sub-agents or this session ever edited any of these files' `diff`/`mutate` logic (grep
     confirms only cosmetic doc-comment edits and, for the schema/component.rs files under live peer
     edit, zero edits at all). **Not fixed** — this is real per-mutation semantic work (several cad
     examples add `MutationOutcome::empty().warn(...)` no-op detection, not a blanket mechanical wrap),
     belongs to the MUTATION-OUTCOMES ticket's own scope, and risks introducing wrong domain judgments
     if hand-guessed here (same reasoning the assembly sub-agent used for its own out-of-scope gap).
     Flagged for that ticket's owner or a dedicated follow-up.
  2. **Live, ongoing, upstream workspace churn — moves with every re-run, confirmed via `git status`/
     `git log --date=iso` each time, never via commit-message text.** Round 1:
     `semio-s-plugin-stdio`-class breakage upstream of both crates. Round 2 (procedural):
     `semio-framework-os-kernel` itself failed (`store`/`vcs`/`spr::testkit` `MutationApplyError` shape
     mismatches). Round 3 (flow, moments later): the SAME crate's failure had moved to a different file
     within it (`📡️spr/🧪️testkit/component.rs`, still uncommitted-modified, confirmed live). Round 3
     (procedural, moments after that): `semio-framework-os-kernel` now compiled clean, but
     `semio-framework` itself failed (`🔁️workflow/component.rs`, its own `Mutation::apply` shape
     mismatch, `🧰️framework/📦️packages/🦀️rust/📦️glue.rs` confirmed uncommitted-modified at that
     moment). This is the exact "failure genuinely moves upstream as the peer session keeps editing"
     pattern `📓️w2-cad-report.md` and `📓️w0-f-report.md` both documented, now observed a third time
     independently. **0 errors ever anchored inside either plugin's own files across any round.**
     Re-run once the live `MUTATION-OUTCOMES`/framework churn settles.

### Live-filesystem policy check

`.🐍️w2-p5-policy-check.ts` (adapted from the mathematical packet's own precedent script), calling
`policySubsetSurfaceCompletenessBreaches`/`policyViewerPurityBreaches`/
`policyContributedSurfaceTargetBreaches`/`policyOsConfigShapeBreaches` directly against the live
filesystem (not the cached `bun ./📜️script.ts policy` snapshot, per the brief's explicit warning).
Output: `🧪️w2-p5-policy.txt`.

- **176 breaches repo-wide** (other, not-yet-migrated subsets — expected, not this packet's job).
- **0 breaches for `🌊️flow` or `🌀️procedural`** across all four policies — surface-completeness,
  viewer-purity, contributed-surface-target, os-config-shape.

## Files touched

Full per-subset file lists are in the four sub-agent notes files (linked above). Summary:

Created (by sub-agents, verified in place):
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/{✏️editor,👁️viewer}/**`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/{✏️editor,👁️viewer}/**`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/{✏️editor,👁️viewer}/**`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🦀️component.rs` (new artifact root)
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/{✏️editor,👁️viewer}/**` (authored, not yet mounted — see blocking gap)

Edited (this session, coordinator wiring + fixes):
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📦️glue.rs` (Editor/Viewer region split, examples repoint)
- `✏️s/🔌️plugins/🌊️flow/🦀️component.rs` (`.editor()`/`.viewer()`, `surface_tests`)
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs` (`paired_apps` fix)
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️component.rs` (`AppRole` import fix)
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs` (Editor/Viewer region split, assembly artifact-root mount, examples repoint, assembly editor/viewer deliberately NOT mounted)
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml` (cosmetic doc fix)
- `✏️s/🔌️plugins/🌀️procedural/🦀️component.rs` (`.editor()`/`.viewer()` ×4, `.artifact()` `map_err` fix ×2, `host_media_handler` path fix, `surface_tests`)
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🦀️component.rs` (doc fix)
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` (doc fix only, on a file under live peer edit)
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs` (`.definition` field-access fix, `assert_two_instances_converge` `EditorApp` fix)
- `✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️component.rs` (import, builder call, 2 test literals)
- `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml` (1 `app =` row)

Deleted:
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/` (whole tree)
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/` (whole tree)

Scratch (ticket folder): `🧪️w2-p5-flow-cargo-run1.txt`, `🧪️w2-p5-flow-cargo-short{1,2,3}.txt`,
`🧪️w2-p5-procedural-cargo-short{1,2,3}.txt`, `🐍️w2-p5-policy-check.ts`, `🧪️w2-p5-policy.txt`.

## Handoff

1. Re-run `cargo check -p semio-s-plugin-flow` / `-p semio-s-plugin-procedural` once the live
   `MUTATION-OUTCOMES`/framework churn (`semio-framework-os-kernel`, `semio-framework`) settles —
   expected clean for both plugins' own files based on every round this packet observed.
2. The MUTATION-OUTCOMES ticket (or a dedicated follow-up) needs to finish landing its
   `Mutation::diff -> MutationOutcome<Diff>` sweep against flow's and procedural's schema
   mutation-kind/config/presence files — cad already has this; flow and procedural do not, and it
   predates this ticket.
3. A new schema/protocol-authoring ticket needs to give assembly's schema tree its missing
   artifact-facet descriptor + JSON-Schema/GraphQL/Protobuf leaf files (`energy.model`'s tree is the
   template) before `crate::artifacts::assembly::declaration()` can exist and assembly's already-authored
   editor/viewer can be mounted in `📦️glue.rs`/registered on the plugin root — both are ready and
   waiting, `📓️w2-p5-assembly-notes.md` has full detail.
