# Lowpoly Viewer — Investigation Result

Scope owned: `$A/👁️viewer/` (both `🦀️component.rs` and `🟦️component.ts`, all subfolders), where
`$A = ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any`.
Off-limits (read-only): `$A/✏️editor/**`, `$A/🚪️io/**`, `$A/🧬️schema/**`, `💠️lowpoly/📦️packages/**`.

**Bottom line for all three tasks: zero source edits.** Task 1 is a real, well-evidenced
NOT-IMPLEMENTABLE-TODAY verdict (evidence below), and Tasks 2 and 3 are audits whose evidence says
"leave as is" — every one of the 14 empty viewer facet folders matches the universal, deliberate
pattern used by every other viewer in the repo, and the viewer's TypeScript component already matches
its established peers byte-for-byte in shape. Adding files or code to any of them would be exactly the
cargo-culted boilerplate the brief prohibits.

---

## Task 1 — composed child-mesh resolution: NOT IMPLEMENTABLE within this ticket's boundaries

### Verdict

The viewer's fallback-box placeholder (`$A/👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️model/🦀️component.rs:7-9`)
is not a viewer-side gap I can close. The composed-child machinery the brief describes
(`LowpolyObject.mesh: Option<store::ArtifactChild<SemioMeshSnapshot>>`) is **wired as types only,
nowhere else** — not in the viewer, not in the editor, not in the mutation layer. The real mesh
geometry never round-trips through any child artifact store anywhere in this plugin. This is not a
missing framework API; the framework API exists and is even threaded into the viewer's own render
signature. It is a missing **write path**, entirely inside files I am forbidden to touch
(`🧬️schema/🧬️mutations/🕸️create-mesh`, `✏️editor/⚙️engine`, `✏️editor/🖌️session`), plus the fact that
`LowpolySnapshot` never implements `store::ArtifactRefs` at all, so the framework's generic
child-content loader has nothing to open regardless of what a viewer's render function does with it.

### Evidence chain

1. **The framework API a viewer would call exists and reaches the viewer's render boundary.**
   `ArtifactView<'a, P>` (🧰️framework `🔨️modules/🔌️plugin/🦀️component.rs:7989`) carries:
   ```rust
   pub struct ArtifactView<'a, P> {
       pub snapshot: &'a P,
       pub history: &'a HistoryView,
       pub children: ChildContentView,   // <- this is the resolver
       ...
   }
   ```
   and `ChildContentView` (same file, `~8389`) exposes exactly the read API needed:
   ```rust
   pub fn typed_read<S: ArtifactPack + Send + Sync + 'static>(&self, slot: &str, child_id: &str)
       -> Result<store::SnapshotReadRef<'_, S>, Fault>
   ```
   `ArtifactViewer::render`'s trait signature (`fn render(body_key: &str, doc: &ArtifactView<'_,
   Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>) -> UiNode`, line `26207`) already hands a
   viewer's own top-level `impl ArtifactViewer for LowpolyViewer` (which I own,
   `$A/👁️viewer/🦀️component.rs`) a `doc.children` alongside `doc.snapshot`. So structurally, nothing
   in the framework *forbids* a viewer from resolving a composed child — the shape is there.

2. **But `doc.children` is populated exclusively from a per-app-instance `child_content_root` field,
   which is only ever written through `register_child`, and NOTHING in the whole lowpoly plugin calls
   `register_child`.**
   ```
   $ grep -rn "with_member\|register_child\|child_content_root\|ChildMemberRegistry" ✏️s/🔌️plugins/💠️lowpoly
   (zero matches)
   ```
   The generic dispatch driver builds every `ArtifactView` (for both `EditorApp` and `ViewerApp`, since
   both share the same `ArtifactApp` trait) as:
   ```rust
   let doc = ArtifactView::with_children(snapshot.as_ref(), history.as_ref(),
       ChildContentView::clone(&self.child_content_root)).await;
   ```
   (`🔨️modules/🔌️plugin/🦀️component.rs:20903`, `24523`, etc.) `self.child_content_root` starts and
   stays `ChildContentView::EMPTY` unless some app-level code calls `.register_child(...)`
   (`🔨️modules/🔌️plugin/🦀️component.rs:19784`). Lowpoly never does — not the editor, not the viewer,
   not the mutation handlers. `doc.children.typed_read(...)` would return `Err("no live child store for
   slot … child …")` on every single call, unconditionally, for every lowpoly document that has ever
   existed.

3. **`LowpolySnapshot` never implements `store::ArtifactRefs`, so the framework doesn't even know a
   lowpoly document HAS children.**
   ```
   $ grep -rn "ArtifactRefs\|child_slots\|#\[child\|ArtifactChildren" \
       ✏️s/🔌️plugins/💠️lowpoly/…/🧬️schema
   (zero matches)
   ```
   `ArtifactRefs::child_refs()` (`🔨️modules/🏪️store/🦀️component.rs:2704`) defaults to `Vec::new()`
   ("a leaf artifact needs zero boilerplate"). Lowpoly never overrides it. This is corroborated
   in-repo by `LowpolyObject.mesh`'s own doc comment
   (`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🦀️component.rs:126-131`, read-only, not mine to
   edit):
   > "…it does not recurse into a `Vec<T>` element type's own fields the way
   > `LowpolySnapshot.objects: Vec<LowpolyObject>` would need. … Consequently `#[child(...)]` cannot be
   > applied here … and `LowpolySnapshot::child_slots()` cannot discover this slot."
   (Note: a *manual* `impl ArtifactRefs for LowpolySnapshot` walking `self.objects` would sidestep the
   derive-macro limitation — this is a plugin-level gap, not a hard framework wall — but it still lives
   in `🧬️schema`, off-limits to me, and by itself doesn't create real child content either.)

4. **The mesh content itself is never persisted to any child store — only logged as ephemeral event
   payload.** `create-mesh`'s own doc comment
   (`$A/🧬️schema/🧬️mutations/🕸️create-mesh/🦀️.rs:1-8`, read-only):
   > "`mesh_workspace` below is event-log payload data only — the originating session's
   > `🖌️session::LowpolyScratch` cache replays it into its own live kernel content;
   > `diff::diff` never writes it onto the document (`LowpolyObject` carries no mesh content field at
   > all …)."
   So even if I invented a `register_child`/`ArtifactRefs` wiring in files I don't own, there is still
   no producer anywhere that writes real geometry bytes into a child artifact store keyed by the
   `mesh_child_handle`'s `(child_id, target)` — the handle is set, the content behind it never exists.

5. **The editor itself — which I was asked to mirror — does not resolve `object.mesh` at all for its
   own rendering, read-only confirmed in
   `$A/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️model/🦀️component.rs:112-127`:**
   ```rust
   fn world_meshes_json(doc: &LowpolyDocument, texture_cache: &HashMap<String, String>) -> String {
       let items: Vec<Value> = serde_json::from_str(&doc.tessellate_all_json()...
   ```
   `doc: &LowpolyDocument` is the editor's own **live in-session kernel** (`✏️editor/⚙️engine`,
   off-limits), populated by live edit operations (add-primitive, extrude, …) — never by reading the
   persisted `mesh` child handle back from a store. There is no "how the editor resolves a composed
   child for rendering" to mirror, because the editor doesn't do that either; it keeps geometry alive
   in-session and never round-trips it through the child-store read path. A viewer, by construction,
   has no such live session (`Config = NoConfig`, `handle()` always returns `ViewEmit::default()`), so
   it cannot reproduce that in-session cache even in principle.

6. **A genuinely working sibling pattern exists for a different geometry shape (real MeshWindowKit
   peer, `🧩️puzzle`'s 3d viewer), and it does NOT use composed-child resolution — it uses a client-fetched
   URL id.** `✏️s/🔌️plugins/🧩️puzzle/…/👁️viewer/…/🧊️main/🦀️component.rs:36-44`:
   ```rust
   let mesh_id = object.mesh_url.as_deref().filter(|url| !url.is_empty())
       .map(world3d_mesh_id_from_url).unwrap_or_else(|| PUZZLE3D_VIEW_FALLBACK_MESH_KIND.to_string());
   ```
   `Puzzle3dObject.mesh_url: Option<String>` is a plain externally-hosted URL, resolved client-side by
   the host renderer (`world3d_meshes_json_from_kinds_and_urls` emits `{"id":…, "url":…}` entries,
   `🔨️modules/🔌️plugin/🦀️component.rs:36561`), never server-resolved child content. This doesn't
   transfer to lowpoly: lowpoly meshes are user-authored procedural geometry (primitives/extrude/
   boolean/etc.), not static hosted assets with a stable URL, so there is no `mesh_url` equivalent to
   read.

### Why I did not write "best-effort" resolution code anyway

I considered adding `doc.children.typed_read::<SemioMeshSnapshot>(slot, child_id)` per object in
`model::render`, falling back to the box placeholder on `Err`. I rejected it:
- It would be **permanently dead** — `doc.children` is provably always `ChildContentView::EMPTY` for
  every lowpoly document until off-limits files add a `register_child` call, so the lookup can never
  once succeed in the current codebase.
- There is **no established `slot` naming convention** to code against — `LowpolySnapshot` has no
  `ArtifactRefs`/`child_slots()` implementation anywhere, so I would be inventing a string
  (`"mesh"`? `format!("mesh-{id}")`?) with no way to verify it against whatever the eventual write side
  picks. Shipping a guess and calling it "implemented" is exactly the fake behavior the brief forbids.
- I cannot write a meaningful Rust unit test for it either: proving it works needs a live
  `ChildContentView` populated via `register_child`, which only an `ArtifactApp`-owning driver
  (off-limits) can construct.

### Handoff (requires coordinated changes outside this ticket's viewer scope)

1. **`$A/🧬️schema`**: implement `impl store::ArtifactRefs for LowpolySnapshot` by hand (walk
   `self.objects`, emit a `ChildRef` per populated `mesh`) — the derive-macro limitation documented on
   `LowpolyObject.mesh` blocks the automatic path but not a manual one.
2. **`$A/🧬️schema/🧬️mutations/🕸️create-mesh`** (and `🧨️delete-mesh`): actually persist
   `mesh_workspace` into a real child artifact store via the `CompositionCoordinator`/`register_child`
   path (`🔨️modules/🔌️plugin/🦀️component.rs:19784`) keyed by the same `(slot, child_id)` the manual
   `ArtifactRefs` impl above would declare, instead of only logging it as inert event-log payload.
3. **`$A/✏️editor/⚙️engine` / `✏️editor/🖌️session`**: once (1)-(2) exist, load persisted child content
   back through `doc.children.typed_read::<SemioMeshSnapshot>(...)` when a document is opened (not only
   keep the ephemeral in-session `mesh_workspace` cache), so a genuinely persisted mesh survives a
   reload — currently it does not.
4. Only once (1)-(3) land does `👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️model/🦀️component.rs`'s `render`
   have anything real to resolve — at that point, threading `doc.children` from
   `$A/👁️viewer/🦀️component.rs`'s `render()` into `model::render(doc.snapshot, doc.children)` and
   calling `typed_read::<SemioMeshSnapshot>(slot, child_id)` per object (falling back to the box
   placeholder only when a given object truly has no mesh child) is a same-day, viewer-only change.

---

## Task 2 — 14 empty viewer facet folders (ticket's diff report says 13; actual count on disk is 14)

`find` over `$A/👁️viewer` turned up 14 `📌️empty.md`-only folders (the pre-existing diff report at
`📝️canonical-shape-diff.md` undercounts by one — it lists 11 viewer paths in its "Missing" bullet,
missing `…🌐️model/🎚️options/`, `…🌐️model/👥️presence/`, and `…🎭️modes/👁️view/🫧️transient/`, of which
two are new; net delta explains the 13-vs-14 mismatch). I audited every one against the richest
populated reference viewers in the repo: 📐️cad (1 instance), 🧱️block (3: ◻2d/🖐️5d/🧊️3d), 🪐️space (2:
🏠️home/🪐️space), 🧩️puzzle (3: ◻2d/🖐️5d/🧊️3d) — 9 independent viewer instances total.

**Finding: every one of these 9 reference viewers has the exact same 14-shape set of folders, and every
one of them is empty (📌️empty.md only) too, with zero exceptions found anywhere.** This is not lowpoly
lagging behind a richer pattern — it is the universal, deliberate shape of a *viewer* (as opposed to an
*editor*, where these same facets ARE populated). Confirmed structurally in code, not just by absence:

- Every reference viewer's `impl ArtifactViewer` (cad `🦀️component.rs:43-49`, block3d `:45-51`, space
  `:42-48`, puzzle3d `:42-48`, lowpoly `:41-47`) declares
  `type Config = NoConfig; type Presence = NoPresence; type Transient = NoTransient;` — identical,
  framework-sanctioned sentinel types meaning "this facet does not exist for this app," not "not done
  yet."
- Every reference viewer's `Command` enum is a single inert `Noop`-shaped variant with a doc comment
  explaining why (lowpoly's own, `$A/👁️viewer/🦀️component.rs:19-22`): "a surface that never dispatches
  anything through `handle`" — real per-command payload modules the way `✏️editor/🎮️commands/*` carries
  them "would be pure ceremony."
- The Model window's own doc comment (`$A/👁️viewer/…/🌐️model/🦀️component.rs`, module doc) states the
  shell renders selection/hover/presence generically off the interaction domain the window declares —
  i.e. presence-like collaborative-viewing state is a framework/shell concern, not a per-plugin schema
  folder.

| # | Folder (relative to `$A/👁️viewer/`) | Decision | Why |
|---|---|---|---|
| 1 | `🎚️config/` | Leave empty | `Config = NoConfig` — identical in all 9 reference viewers, all empty |
| 2 | `🎭️modes/👁️view/🎚️config/` | Leave empty | same `NoConfig`, mode-level; matches all 9 references |
| 3 | `🎭️modes/👁️view/🎮️commands/` | Leave empty | commands are the single inert `LowpolyViewCommand::Noop` inline in the top-level file, not a per-command folder — matches all 9 references, and lowpoly's own doc comment explains why |
| 4 | `🎭️modes/👁️view/👥️presence/` | Leave empty | `Presence = NoPresence`; selection/hover shown generically by the shell per the Model window's own doc comment; matches all 9 references |
| 5 | `🎭️modes/👁️view/🫧️transient/` | Leave empty | `Transient = NoTransient`; matches all 9 references |
| 6 | `🎭️modes/👁️view/🪟️windows/🌐️model/🎚️config/` | Leave empty | window-level config folder is empty in every reference window (cad's `📐️shape`, block3d's `🌐️world`, space's `🏠️main`, puzzle3d's `🧊️main`) — no viewer window anywhere in the repo populates this |
| 7 | `…/🌐️model/🎚️options/` | Leave empty | same — empty in every reference window; a viewer has no dropdown/tool options (no utilities) |
| 8 | `…/🌐️model/🎬️actions/` | Leave empty | a viewer never dispatches mutating actions (`handle` always returns `ViewEmit::default()`); empty in every reference window |
| 9 | `…/🌐️model/👥️presence/` | Leave empty | window-level presence folder empty in every reference window; presence is shell-generic |
| 10 | `…/🌐️model/🪛️utilities/` | Leave empty | no utilities are declared for the viewer's window (`utilities` list is absent from the viewer's `WindowKindDefinition`, unlike the editor's `["move","rotate",…]`); empty in every reference window |
| 11 | `…/🌐️model/🫧️transient/` | Leave empty | `Transient = NoTransient`; empty in every reference window |
| 12 | `🎮️commands/` (viewer-level) | Leave empty | same reasoning as #3, at the artifact level; matches all 9 references |
| 13 | `👥️presence/` (viewer-level) | Leave empty | same reasoning as #4/#9, at the artifact level; matches all 9 references |
| 14 | `🫧️transient/` (viewer-level) | Leave empty | `Transient = NoTransient`; matches all 9 references |

No files added. Populating any of these would mean lowpoly's viewer is the *only* one in the repo with
content there — a deviation from, not a completion of, the established pattern, and exactly the
"boilerplate to make a folder non-empty" the brief prohibits.

---

## Task 3 — `$A/👁️viewer/🟦️component.ts`: already at parity, no changes made

The brief's premise ("552 bytes, only exports dialect constants") is accurate but the comparison target
matters: measured against cad's own top-level viewer TS file, the two are structurally identical:

```
$ wc -c $A/👁️viewer/🟦️component.ts   ✏️s/…/📐️cad/…/👁️viewer/🟦️component.ts
     552 …lowpoly…/👁️viewer/🟦️component.ts
     532 …cad…/👁️viewer/🟦️component.ts
```

Both files are: a module doc comment ("Read-only counterpart of `✏️editor/🟦️component.ts` … no
mutation-shaped exports"), one `*_VIEWER_DIALECT` const, one `*_VIEW_MODE_ID` const, and a single
`export * from "./🎭️modes/👁️view/🪟️windows/{window}/🟦️component"` re-export. The 20-byte difference is
just `"lowpoly"` (7 chars) vs `"cad"` (3 chars) appearing twice in identifier/dialect strings. This is
the established top-level pattern, not a gap.

The real behavioral content lives in the window-level TS file
(`$A/👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️model/🟦️component.ts`), which already declares a real typed
view-model (`LowpolyViewModelInstance`/`LowpolyViewModelViewModel` with `id`, `meshId`, `position`,
`rotation`, `scale`, `label`, `smoothShading`). Comparing it against its true peer — not cad (which uses
a bespoke non-`MeshWindowKit` shape system, so its window TS types different domain fields like
`references`) but `🧩️puzzle`'s 3d viewer, the other `framework.window.mesh`-based viewer in the repo —
shows the same shape almost field-for-field:

```ts
// puzzle3d — ✏️s/…/🧩️puzzle/…/👁️viewer/…/🧊️main/🟦️component.ts
export interface Puzzle3dViewInstance {
  id: string; meshId: string; position: [number, number, number];
  rotation: [number, number, number, number]; scale: [number, number, number];
  label: string; disabled: boolean;
}
```
vs lowpoly's `id, meshId, position, rotation, scale, label, smoothShading` — same 7-field shape, same
naming convention, same doc-comment template referencing `MeshWindowKit`/`MeshView`/`ViewEmit`-only. No
UI-facing strings exist in either file (both are pure type/const declarations with no `Label`/localized
text), so there is nothing to make bilingual here — the `{"en":…,"de":…}` pattern applies to
`LocalizedLabel`s in the `.rs` window/mode definitions (already present, e.g. `LocalizedLabel::native("Model",
"Modell")` in `$A/👁️viewer/…/🌐️model/🦀️component.rs` and `LocalizedLabel::native("View", "Ansicht")` in
the mode file), not to a pure type-mirror `.ts` file. No changes made — lowpoly's viewer TS component is
already at the standard cad and puzzle3d both hold.

---

## Verification

Ran from `/Users/ueli/Documents/semio` with `export DEVELOPER_DIR=/Library/Developer/CommandLineTools`.
No source files were changed by this ticket slice (confirmed: `git status --porcelain -- $A/👁️viewer/`
returns empty), so there is nothing new to regress — the goal here was to confirm the pre-existing
baseline still holds.

**`bunx tsc --noEmit` — ran to completion, clean for my scope.** Repo-wide it reports 5,983
pre-existing errors (mostly `TS5097 "allowImportingTsExtensions"`, a tsconfig setting gap, plus
concurrent-ticket fallout in `🖥️server/🎛️coordinator`, `📚️library/🧹️normalization`, etc. — none of it
mine to fix). Of the 7 lowpoly-related hits, all 7 are in `📦️packages/` and `✏️editor/📚️examples`/
`📚️examples/` — off-limits or out-of-scope paths I did not touch. **Zero hits anywhere under
`$A/👁️viewer/`** (`grep "👁️viewer" tsc-check.txt | grep -i lowpoly` → no output), confirming Task 3's
verdict directly: the viewer TS component type-checks clean today.

**`cargo check -p semio-s-plugin-lowpoly --all-targets` — could not complete; blocked entirely upstream
in `🗄️stdio`, not in lowpoly.** Two attempts:
1. First attempt against the shared `/Users/ueli/Documents/semio/target` dir was blocked on
   `target/debug/.cargo-lock` for 50+ minutes behind other concurrent sessions' builds (confirmed via
   `lsof`: a sibling `cargo check -p semio-framework-plugin -p semio-s-plugin-procedural --all-targets`
   held the lock) and was ultimately killed by the harness's background-task management before
   producing output — `uptime` during this window showed `load averages: 48.65 68.18 77.59` on what is
   evidently a machine running many concurrent agent sessions' builds at once.
2. Second attempt used an isolated `CARGO_TARGET_DIR` (this ticket's own scratchpad, avoiding the
   shared lock) and ran to completion in ~80 minutes under the same heavy load. Result:
   ```
   error: could not compile `semio-s-plugin-stdio` (lib) due to 329 previous errors; 1403 warnings emitted
   ```
   `semio-s-plugin-stdio` is `🗄️stdio` — the crate lowpoly depends on for `SemioMeshSnapshot`
   (`use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;`,
   `$A/../../🦀️component.rs:5`) — and it is **not mine to touch** (not `$A/👁️viewer/`, not even inside
   `💠️lowpoly` at all). Every located error site (139/139, via
   `awk '/^error/{...}/--> /{...}'` over the full log) resolves to a path under
   `✏️s/🔌️plugins/🗄️stdio/…` — none in lowpoly. Representative errors: `cannot find type
   'BinarySnapshot'/'DeflateSnapshot' in this scope`, `unresolved import
   …artifacts::md::standards::v_commonmark::subsets::any::schema::mutations::set_snapshot::SetSnapshot`,
   `cannot find function 'agg_diff'/'agg_inverse' in this scope`, plus a `MutationLeaf source authority
   failed: semanticKind must be lowercase kebab-case` proc-macro validation failure. This is the exact
   shape of another session's in-progress refactor breaking a shared crate mid-flight (binary/deflate/
   markdown subsets under active restructuring), not a lowpoly or viewer defect — `semio-s-plugin-lowpoly`
   cannot be type-checked as a whole crate while its own dependency fails to compile, regardless of what
   this ticket does or doesn't change.

**`cargo check --target wasm32-wasip2`, `cargo clippy`, `cargo test -p semio-s-plugin-lowpoly --lib`
— not run separately.** All three would hit the identical `semio-s-plugin-stdio` compile failure as a
blocking dependency before ever reaching lowpoly's own code (confirmed by the same `error: could not
compile 'semio-s-plugin-stdio'` being the first hard stop cargo hits for this crate under any target),
so repeating them would only re-derive the same external result at further cost (each of the two
`cargo check` attempts above took 50-80 minutes under the machine's current concurrent load). This
should be re-run once the `🗄️stdio` refactor lands; since this ticket slice made zero edits anywhere,
re-running later carries no risk of surfacing anything new from this deliverable specifically.

## Handoff summary (also listed inline above)

1. `$A/🧬️schema`: hand-write `impl store::ArtifactRefs for LowpolySnapshot` (walk `objects`, emit
   `ChildRef` per populated `mesh`) — off-limits to me, blocked for the auto-derive path by the
   documented `#[child(...)]`-doesn't-recurse-into-`Vec<T>` limitation, but open for a manual impl.
2. `$A/🧬️schema/🧬️mutations/🕸️create-mesh` (+ `🧨️delete-mesh`): persist `mesh_workspace` into a real
   child artifact store via `register_child`/`CompositionCoordinator` instead of leaving it as
   inert event-log payload the document diff never touches.
3. `$A/✏️editor/⚙️engine` + `✏️editor/🖌️session`: load persisted child content back via
   `doc.children.typed_read::<SemioMeshSnapshot>(...)` on open, not only the ephemeral in-session
   `mesh_workspace` cache — today a persisted mesh does not survive a reload even in the editor.
4. Once (1)-(3) exist: `$A/👁️viewer/🦀️component.rs`'s `render()` should thread `doc.children` into
   `model::render`, and `model::render` should call `typed_read::<SemioMeshSnapshot>(slot, child_id)`
   per object, falling back to the box placeholder only for objects genuinely missing a mesh child.
   This last step is fully within viewer ownership and is a same-day change once the write side lands.
5. **Unrelated build blocker, flag to whoever coordinates concurrent sessions**: at the time of this
   ticket, `semio-s-plugin-stdio` (`✏️s/🔌️plugins/🗄️stdio`) fails to compile with 329 errors (unresolved
   imports/types in its `md`/binary/deflate subsets, plus one `MutationLeaf semanticKind` proc-macro
   validation failure) — almost certainly another session's in-progress refactor. Since lowpoly depends
   on `semio-s-plugin-stdio` for `SemioMeshSnapshot`, `cargo check -p semio-s-plugin-lowpoly` (and
   anything downstream of it — wasm target, clippy, tests) cannot succeed until that lands, independent
   of anything in this ticket.
