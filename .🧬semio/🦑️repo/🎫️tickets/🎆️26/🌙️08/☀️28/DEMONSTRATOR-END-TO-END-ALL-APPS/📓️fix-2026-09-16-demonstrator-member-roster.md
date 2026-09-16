# 🎪️ Demonstrator member roster — `s.stdio.semio@v1/kit` not declared

Ticket `26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS` · 2026-09-16 · worker: member-roster fix

## 1. Findings (trace)

### 1.1 The fault

```
ArtifactApp::genesis_child_pack members must open cleanly onto a freshly constructed store:
Fault { origin: Plugin, code: "plugin.internal",
        message: "derived child dialect 's.stdio.semio@v1/kit' is not declared by this app's member roster" }
```

Chain, all in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`:

- `:21669` — `VcsArtifactApp::<A, M>::with_registry_on_bus` ends with
  `this.seed_genesis_children().await.expect(…)`.
- `:21678-21697` — `seed_genesis_children` walks `ChildRestoreProjection::from_snapshot(initial snapshot)`,
  keeps every slot for which `A::genesis_child_pack` returns `Some`, and for each one calls
  `genesis_member_schema::<M>(&dialect)?`.
- `:20845-20852` — `genesis_member_schema::<M>` looks the dialect up in `M::OPEN_DECLARATIONS`
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19766`, trait `MemberFactory`) and faults when
  the roster does not declare it.

So the crash is decided by exactly one thing: the `M` type parameter of `VcsArtifactApp<A, M>`.

### 1.2 How `M` was chosen (before this fix)

`M` defaulted to `store::NoMembers` (`🏪️store/🦀️.rs:19949-19964`, `OPEN_DECLARATIONS = &[]`) for every
surface registered through the plain builder verbs:

- `🔌️plugin/🏗️builder/🦀️.rs:467` `PluginBuilder::editor::<E>()` → `:494` `editor_app::<E>()` →
  `VcsArtifactApp<EditorApp<E>>` (i.e. `M = NoMembers`).
- `:400` `viewer::<V>()` → `:404` `viewer_with_members::<V, store::NoMembers>()`.
- `🔌️plugin/🦀️.rs:32257` `declarations::editor_surface::<E, PA>` / `:32297` `viewer_surface` — same,
  `M = NoMembers`.

The roster-carrying variants existed but had to be opted into **per registration site**:
`editor_with_members::<E, M>` (`🏗️builder/🦀️.rs:519`), `viewer_with_members::<V, M>` (`:408`),
`declarations::editor_surface_with_members` (`🔌️plugin/🦀️.rs:32271`) and `viewer_surface_with_members`
(`:32310`). `editor_mutation_roster::<E>()` (`🏗️builder/🦀️.rs:547`) is unrelated — it only publishes
`SemanticMutation::kinds()` for `contributor.list-artifact-mutations`; it has nothing to do with `M`.

### 1.3 What the standalone plugins declared

- `✏️s/🔌️plugins/🪵️sourcing/🦀️.rs:11-16` closes its fleet over
  `VcsArtifactApp<EditorApp<SourcingCurationApp>, semio_s_artifact_stdio_semio::SemioMembers>` and registers
  through `.declare_artifact(curation::artifact())`, whose subset
  (`🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs:32-33`) uses
  `editor_surface_with_members::<…, SemioMembers, A>` / `viewer_surface_with_members::<…>`.
- `✏️s/🔌️plugins/🌍️gis/🦀️.rs:53-55` uses `.editor_with_members::<Gis2dPlayApp, SemioMembers>` /
  `.viewer_with_members::<GisMapViewer, SemioMembers>`.
- `✏️s/🔌️plugins/🌊️flow/🦀️.rs:30,32` and `✏️s/🔌️plugins/🎬️sequence/…/🪆️subsets/✳️any/🦀️.rs:30-31` do the same.
- `SemioMembers` is the 18-subset `dsl::space_members!` roster in
  `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs:1164`; its `Kit` arm declares
  `("s.stdio.semio", "v1", "kit", "stdio.semio")` — the dialect the sourcing curation derives.

### 1.4 Why the demonstrator's roster lacked it

`✏️s/🔌️plugins/🎪️demonstrator/🪪️manifest/🎪️demonstrator/🦀️.rs` bundled the six foreign apps with the
plain verbs — `.editor::<SourcingCurationApp>(…)`, `.viewer::<SourcingViewer>(…)`,
`.editor::<Gis2dPlayApp>(…)` — and its `dyn_enum_close!` fleet spelled the variants as
`VcsArtifactApp<EditorApp<SourcingCurationApp>>`, i.e. `M = NoMembers`. The roster was a property of the
*registration call*, not of the app, so a bundle could (and did) silently register the very same app type
with an empty roster. Nothing in the type system objected; the mistake surfaced only at app-creation time
in the browser.

Which apps actually derive genesis children (repo-wide `fn genesis_child_pack` overrides outside the
framework): **only** sourcing curation (editor `…/✏️editor/🦀️.rs:888`, viewer `…/👁️viewer/🦀️.rs:67`,
both → `crate::genesis_catalog_pack`, `🗂️curation/🦀️.rs:245-251`, minting a `SemioKitSnapshot` under
`s.stdio.semio@v1/kit`) and gis gismap (editor + viewer). `Process3dPlayApp`/`Process3dViewer` override
nothing, so `seed_genesis_children` is a no-op for them — the *bearbeiten* pane's failure is therefore NOT
this fault; aussuchen (sourcing) and verfolgen (gis map) are.

## 2. Design

Roster moved from the registration call to the app itself — the option the ticket prefers ("roster is a
property of the app definition, so any bundle gets it right by construction"):

- `ArtifactEditor::Members` / `ArtifactViewer::Members`, associated types bounded
  `store::SpaceMember + store::MemberFactory + Send + 'static` and **defaulted to `store::NoMembers`**, so no
  existing surface author changes.
- Every surface verb now instantiates `VcsArtifactApp<EditorApp<E>, E::Members>` /
  `VcsArtifactApp<ViewerApp<V>, V::Members>`: `PluginBuilder::editor`, `editor_with_examples`,
  `editor_app`, `viewer`, and `declarations::editor_surface` / `viewer_surface`.
- The four `*_with_members` entry points are **deleted** (no compat layer): there is now exactly one way to
  register a surface and it always carries the app's own roster. The four crates that used them
  (sourcing, sequence, gis, flow) declare `type Members = semio_s_artifact_stdio_semio::SemioMembers;` on
  their eight app types instead, so their behaviour is byte-identical.
- Associated-type defaults are a nightly feature (`#![feature(associated_type_defaults)]`, enabled in the
  `semio-framework-plugin` crate root only — proven by probe that downstream crates need no gate). The
  earlier objection to them (`26/08/17/…/📓️terra-dedyn-fw-os-spacemember-report.md`) was about putting a
  mandatory `type Members` on `ArtifactApp`, which every plugin implements; a *defaulted* type on the two
  authoring traits has zero blast radius.
- Consequence: a bundle that spells a fleet variant with the wrong roster no longer compiles (the
  `From<VcsArtifactApp<EditorApp<E>, E::Members>>` bound fails), and `ArtifactEditor::Members` is the single
  source of truth the demonstrator composes from — nothing is hand-copied.

## 3. Edits

Framework (`semio-framework-plugin`):

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🦀️.rs:4` — `#![feature(associated_type_defaults)]`.
- `…/🔌️plugin/🦀️.rs:30679` — `ArtifactEditor::Members` (defaulted `store::NoMembers`).
- `…/🔌️plugin/🦀️.rs:31127` — `ArtifactViewer::Members` (same).
- `…/🔌️plugin/🦀️.rs:30667,30670` — new `EditorSurfaceApp<E>` / `ViewerSurfaceApp<V>` aliases
  (`VcsArtifactApp<EditorApp<E>, <E as ArtifactEditor>::Members>`) for fleet enums to spell.
- `…/🔌️plugin/🦀️.rs:32308,32323` — `declarations::editor_surface` / `viewer_surface` instantiate
  `E::Members` / `V::Members`; `editor_surface_with_members` / `viewer_surface_with_members` deleted.
- `…/🔌️plugin/🏗️builder/🦀️.rs:400,457,473,480` — `viewer`, `editor`, `editor_with_examples`, `editor_app`
  now bound and instantiate `E::Members` / `V::Members`; `editor_with_members` / `viewer_with_members`
  deleted.
- `…/🔌️plugin/🦀️.rs:20856,20876,20882` — new `genesis_children_declared` + the two public laws
  `assert_editor_genesis_children_declared::<E>()` / `assert_viewer_genesis_children_declared::<V>()`,
  re-exported at the crate root (`…/🔌️plugin/🦀️.rs:38609-38610`). They replay exactly what
  `seed_genesis_children` does, without constructing the app.

Apps that compose `s.stdio.semio@v1/*` children now declare their roster once, on themselves:

- `✏️s/🔌️plugins/🪵️sourcing/…/✳️any/✏️editor/🦀️.rs:816`, `…/👁️viewer/🦀️.rs:45`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/…/✏️editor/🦀️.rs:773`, `…/👁️viewer/🦀️.rs:40`
- `✏️s/🔌️plugins/🌊️flow/…/✏️editor/🦀️.rs:2167`, `…/👁️viewer/🦀️.rs:41`
- `✏️s/🔌️plugins/🎬️sequence/…/✏️editor/🦀️.rs:3014`, `…/👁️viewer/🦀️.rs:41`

Registration sites that used the deleted verbs (behaviour unchanged — same roster, now inherited):

- `✏️s/🔌️plugins/🌊️flow/🦀️.rs:30,32`, `✏️s/🔌️plugins/🌍️gis/🦀️.rs:53,55`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs:10,32,33`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs:10,30,31`
- `✏️s/🔌️plugins/🌊️flow/…/👁️viewer/🧪️tests/🔬️unit/🦀️.rs:23`

Demonstrator:

- `✏️s/🔌️plugins/🎪️demonstrator/🪪️manifest/🎪️demonstrator/🦀️.rs:29-43` — the `dyn_enum_close!` fleet now
  spells every variant as `EditorSurfaceApp<…>` / `ViewerSurfaceApp<…>`, so `SourcingEditor`,
  `SourcingViewer` and `GisEditor` carry `SemioMembers` by construction. The `plugin()` body is
  unchanged: `.editor::<SourcingCurationApp>(…)` now means "with the roster sourcing declares".
- `✏️s/🔌️plugins/🎪️demonstrator/🪪️manifest/🎪️demonstrator/🧪️tests/🔬️unit/🦀️.rs:120-146` — new test
  `every_bundled_surface_declares_its_derived_child_dialects`, asserting the law for all ten surfaces.

## 4. Test

`✏️s/🔌️plugins/🎪️demonstrator/🪪️manifest/🎪️demonstrator/🧪️tests/🔬️unit/🦀️.rs` —
`every_bundled_surface_declares_its_derived_child_dialects` runs the new law over all ten bundled
surfaces. The law (`artifact_app_laws::assert_{editor,viewer}_genesis_children_declared`) replays
`seed_genesis_children`'s own lookup — initial snapshot → `ChildRestoreProjection` → `genesis_child_pack`
→ `genesis_member_schema::<Members>` — and RETURNS the number of derived children it resolved, which the
test pins per surface (sourcing editor 1, sourcing viewer 1, gis2d editor 2, the other seven 0). Without
that count the assertion would pass vacuously for an app that stopped composing; the first run of the
pinned version in fact failed with `left: 2, right: 1` on gis2d, i.e. the counts are real.

Failure evidence (the "before" state), two independent runs:

1. `type Members = store::NoMembers;` put back on `SourcingCurationApp` →
   `semio-s-artifact-sourcing-curation` no longer compiles:
   `🪆️subsets/✳️any/🦀️.rs:33: error[E0277]: the trait bound 'A: From<VcsArtifactApp<EditorApp<SourcingCurationApp>>>' is not satisfied`
   (log: `🗑️generated/roster-hosts-check-2.txt` sibling run, captured in chat).
2. The demonstrator fleet variant put back to its pre-fix spelling
   `SourcingEditor(VcsArtifactApp<EditorApp<SourcingCurationApp>>)` →
   `🪪️manifest/🎪️demonstrator/🦀️.rs:71: error[E0277]: the trait bound 'DemonstratorApps: From<VcsArtifactApp<EditorApp<...>, ...>>' is not satisfied`
   (log: `🗑️generated/roster-demo-test-prefix-shape.txt`).

Both reverts were undone immediately. This is stronger than a red test: after the fix the bug's exact
shape cannot be written down any more — the compiler rejects a bundle that registers a composed app over
a roster the app does not declare. The runtime law remains as the belt-and-braces check for an app that
gains a derived child whose dialect its own roster never declared.

Note on the earlier ManuallyDrop: the first version of the law dropped the probe snapshot and tripped
`ordered-map root must be explicitly retired before drop` (`🌱️value/🗂️ordered/🦀️.rs:81`). Retiring one
needs the owner factories a live `ArtifactStore` installs, so the test-only law hands the probe to
`ManuallyDrop` — documented on the function.

## 5. Commands and outcomes

All logs under `…/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/`.

| command | outcome |
| --- | --- |
| `cargo check -p semio-framework-plugin --lib` | `Finished` (roster-fw-check-1/2.txt) |
| `cargo check -p semio-s-plugin-sourcing -p …-gis -p …-flow -p …-sequence --lib` | `Finished` in 6m51s (roster-hosts-check-2.txt) |
| `cargo test -p semio-s-plugin-demonstrator --lib every_bundled_surface_…` | **ok**, 1 passed (roster-demo-test-final2.txt) |
| `cargo test -p semio-s-plugin-demonstrator --lib` | 8 passed, 2 failed — both pre-existing and unrelated (roster-demo-test-final3.txt) |
| `bun nx run @semio-tech/demonstrator-plugin:materialize-dev` | success, 10m34s, component + descriptor staged (roster-materialize-dev-1.txt) |
| `bun nx run @semio-tech/mit-bestand-demonstrator:activate-dev` | success, 22m55s, "Activated Demonstrator dev: 28 completed components" (roster-activate-dev-1.txt) |

The wasm side is covered by `materialize-dev` itself: it runs `component-dev`, i.e. the real
`wasm32-wasip2 / wasm-dev` build of `semio-s-plugin-demonstrator` and of all six bundled host crates, so a
separate `cargo check --target wasm32-wasip2` would only have duplicated that work (the laws are
`#[cfg(test)]`-gated and never reach the component).

Two unrelated failures in the demonstrator suite, both in procedural's territory and untouched by this
work — worth a separate look:

- `contribution_consumers_declare_the_hidden_app_command`: `s.procedural.generation3d@1/*#editor` now also
  declares `setContributions`, so the pinned consumer list is short by one.
- `aggregate_runtime_renders_every_demonstrator_window`: rendering `procedural.play.main` with `"{}"`
  faults `missing field 'locale' at line 1 column 2` — a render-argument decode change, not a roster one.
  It aborts on the first app, so it never reaches the sourcing/gis surfaces this fix is about.

Housekeeping: the shared build dir was full (`No space left on device` during the host-crate check, 1.0 GB
free, `debug/incremental` alone at 102 GB). I pruned
`.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/incremental` only — incremental state is a cache, so no
fingerprints were invalidated — which freed 48 GB. See memory note "Incremental Cache Regrows Under Fleet".

Deployment: `:6029` is still listening (`bun`, pid 17227) and was never started or stopped by this work;
`activate-dev` re-staged the receipt for it to pick up. `curl -o /dev/null -w '%{http_code}'
http://localhost:6029/` returned `000` after more than two minutes — the port accepts the connection but
the server never answers, which matches the known "release serve wedges after host edit bursts" note
(memory: `project-release-serve-wedges-after-host-edit-bursts`). It was NOT restarted here; recycling it
and the browser check of the aussuchen/bearbeiten/verfolgen panes are left to whoever owns that server.
