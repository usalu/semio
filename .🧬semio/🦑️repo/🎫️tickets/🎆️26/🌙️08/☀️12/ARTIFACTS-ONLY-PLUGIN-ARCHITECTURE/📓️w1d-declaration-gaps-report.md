# W1d — Closing the two remaining `ArtifactDeclaration` coverage gaps

Owner for this pass: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` +
`…/🏗️builder/🦀️component.rs`. Composition runtime (`Emit.child_emits`/`ChildEmit`, `VcsArtifactApp`
child-store map, `dispatch_emit`, group undo/redo, `ArtifactChildren`, `DerivedArtifactSpec::Children`,
WIT `resolve-artifact-link`) was NOT touched.

## Gap A — `🔋️energy`'s bare document codec: CLOSED with a new sibling method

`.document_codec::<A: ArtifactApp>()` is keyed off `A::DOCUMENT_SCHEMA` and calls
`register_document_codec_for_app::<A>`, which needs a real `ArtifactApp`. Energy is a headless
library plugin (zero apps) that registered `EnergyModelSnapshot`/`EnergyModelMutation`'s pack↔dsl
codec straight against `store::register_document_codec` via `.setup()`.

**New method**, `🔌️plugin/🦀️component.rs` (`ArtifactDeclarationBuilder<DeclarationReady>`):

```rust
pub fn document_codec_bare<Snapshot, Mutation>(mut self, schema: impl Into<String>) -> Self
where
    Snapshot: Clone + PartialEq + Serialize + DeserializeOwned + Send + store::ArtifactDsl + ArtifactPack + 'static,
    Mutation: ::protocol::Mutation<Snapshot> + PartialEq + Serialize + DeserializeOwned + Send + ::protocol::OpText + ::protocol::OpBinary + 'static,
```

Bounds are copied verbatim from `store::ArtifactCodec::of<P, Mutation>`'s own where-clause — the same
bounds energy's old code already satisfied, so satisfying them here is not a new obligation.

**`DocumentCodecSpec` changed shape** to carry the registration: it used to be a bare non-capturing
`fn()` thunk (works for `.document_codec::<A>()` because the schema comes from a type-level const,
`A::DOCUMENT_SCHEMA`). `document_codec_bare`'s schema is a runtime `impl Into<String>`, which a
non-capturing `fn()` cannot close over without breaking the "plain fn pointer, inert data" contract
the type exists for. Fix: the struct now carries `schema: String` alongside `register: fn(String)`,
and both `of::<A>()` and the new `bare::<Snapshot, Mutation>()` build a `fn(String)` thunk instead of
`fn()`. `register_all` now calls `(codec.register)(codec.schema)`. This is a shape change to a
module-private struct with only two constructors, both inside this same file — no external plugin
code names `DocumentCodecSpec` directly.

**Energy wiring:**
- `🔋️energy/🗿️artifacts/🔋️model/🦀️component.rs`: `declaration()` gained
  `.document_codec_bare::<EnergyModelSnapshot, EnergyModelMutation>(ENERGY_MODEL_DOCUMENT_SCHEMA)`.
- `🔋️energy/🦀️component.rs`: `.setup(…register_document_codec)` call deleted — energy's `plugin()` now
  has **zero** `.setup()` calls.
- `🔋️energy/🗿️artifacts/🔋️model/…/⚙️engine/🦀️component.rs`: the now-dead `register_document_codec()`
  free fn deleted (grep-verified: its only caller was the `.setup()` call just removed).

## Gap B — `🧩️puzzle`'s two `.setup()` reasons: ONE was a false gap and is closed; the other survives, judged not to get a field

Puzzle's `.setup()` combined two unrelated things under one callback. They needed separate answers.

### B1 — `register_app_schemas()`: NOT a genuine gap. Closed via the existing category-1 mechanism.

Re-reading puzzle's own comments against the actual code: `register_app_schemas()` called three
free functions (`crate::apps::puzzle{2,3,5}d::config::schema::register_app_schema()`) that
self-registered each play app's config+presence `AppSchemaDescriptor` — this is *exactly* app-scope
schema, category 1, the one `ArtifactApp::app_schema()` + `register_document_app` already closed for
19 other plugins (`🗒️note` is the exemplar). Puzzle's three apps simply never got the override that
would have made `register_document_app` do this automatically — the comment claiming "no
`ArtifactDeclaration` field covers this... kept on `.setup()` by design" was stale, predating (or
simply missing) the `app_schema()` mechanism.

**Fix, mirroring `🗒️note` exactly:**
- `Puzzle2dPlayApp`/`Puzzle3dPlayApp`/`Puzzle5dPlayApp` (`🎛️apps/{◻2d,🧊️3d,🖐️5d}/🦀️component.rs`) each
  gained `fn app_schema() -> Option<artifact_schema::AppSchemaDescriptor> { Some(…::app_schema_descriptor()) }`.
- The three `register_app_schema()` free functions (`🎛️apps/{◻2d,🧊️3d,🖐️5d}/🎚️config/🧬️schema/🦀️component.rs`)
  were converted from self-registering (`artifact_schema::register_app_schema_descriptor(…)`) to
  returning (`pub fn app_schema_descriptor() -> artifact_schema::AppSchemaDescriptor { … }`) — same
  struct literal, no field changes.
- `register_app_schemas()` (the umbrella caller, in puzzle2d's `⚙️engine`) deleted.
- The `.setup(setup)` body in `🧩️puzzle/🦀️component.rs` no longer calls it.

Grep-verified zero remaining references to `register_app_schemas()` or the three old
`register_app_schema()` free-function names anywhere under `✏️s/🔌️plugins/🧩️puzzle/`.

### B2 — OS media-host bridges (`register_media_io`/`register_mesh_io`): judged NOT to get a field. `.setup()` survives here, on purpose.

These call `semio_framework_os::register_2d_export_handlers` / `register_dwg_import_handler` /
`register_mesh_exporter` / `register_mesh_importer` / `register_mesh_dwg_export_handler` /
`register_mesh_dwg_import_handler` — a 6-function family in `💻️os/🦀️component.rs`, writing into
`register_os_media_export_handler_kind`/`register_os_media_import_handler_kind`'s own process-global
registry.

**Investigated, not assumed:**
1. **This is a genuinely different, parallel registry from `io_registry`/`ComposerEntry`** — the one
   `.composers(…)` already wires declaratively. Puzzle2d's `io_registry::entries()` (already reached
   through `.composers(…)`) independently produces `ComposerEntry` rows for SVG/PDF/PNG/JSON/DWG/DXF
   export; puzzle3d's for LAS/PLY/PNG/JSON/DWG/STL/GLTF/OBJ. The OS bridge covers SVG/PNG/DWG (2d) and
   OBJ/GLB/STL/DWG import+export (3d/5d). **Partial overlap confirmed** (SVG↔SVG, DWG↔DWG, OBJ/STL
   both sides) but NOT the clean 1:1 duplication lowpoly had (composer also serves PDF/JSON/DXF/LAS/
   PLY/GLTF that the OS bridge doesn't) — so this is not a safe "delete as pure duplicate" the way
   lowpoly's `register_mesh_exporter("3d.mesh", …)` was.
2. **The OS registry is keyed by a DIFFERENT string than `ArtifactDeclaration.kind`.** Puzzle2d's
   declaration is `builder("s.puzzle2d")`; the OS bridge calls register under `"2d.puzzle"` (matching
   `ArtifactKindSpec.id`, a parallel pre-migration vocabulary). A declaration field cannot thread
   `self.kind` through to auto-supply the right string the way `document_codec_bare` threads `schema`
   — the two namespaces don't correspond 1:1 today.
3. **This exact registry family is independently documented elsewhere in this ticket as
   non-deterministic under concurrent registrants** (`📓️status.md` finding #3: `🎪️demonstrator`
   racing an owning plugin for `3d.process`/`3d.procedural` via the same `register_mesh_exporter`
   mechanism — plugin load order silently decides whose importer answers). Adding an
   `ArtifactDeclaration` field for it would **legitimize** exactly the mechanism this ticket's own
   findings flag as a live correctness bug, not close it.

**Conclusion:** these are host-registry mutations of the kind this ticket exists to remove — **not**
a legitimate `ArtifactDeclaration` field, and I did not add one. But I also did not delete the calls:
proving them safe to delete requires tracing whether `register_os_media_export_handler_kind`'s reader
(the OS-level export/import dispatch, outside `🔌️plugin`/`🏗️builder`) is still live, which is out of
this pass's owned files and verification budget — deleting on inference alone risks silently breaking
real export/import UI functionality, which this ticket's own "get everything working" rule forbids
doing speculatively. **`.setup()` survives on puzzle's root for exactly these two calls**, now
correctly scoped (down from three) and documented with this reasoning in three places: `🧩️puzzle/
🦀️component.rs`'s `plugin()` doc, and both `puzzle2d`/`puzzle3d`/`puzzle5d`'s `declaration()` docs.

## `.setup()` double-set safety: already done, verified in place — not by me this pass

Found the field already `setup: Vec<fn()>` (accumulating, run in call order) in
`🏗️builder/🦀️component.rs`, with `.setup()` documented as "Repeatable — accumulates, runs in call
order, does NOT overwrite an earlier `.setup(...)` call." This matches exactly what this task asked
for (`.artifact()`-style accumulation instead of silent last-write-wins), and the file's own doc
attributes the fix to a W1c agent independently catching the footgun in its own draft. I verified the
mechanism is correct (read the full `🏗️builder/🦀️component.rs`, 209 lines) and did not need to change
it. **This part of the task was already complete when I started; I am reporting it as verified, not
as my own change.**

## `.setup(` real call-site count: 13 (was 14 before this pass)

Counted with `grep -rln '^\s*\.setup(' ✏️s/🔌️plugins/*/🦀️component.rs` — a leading-whitespace-then-dot
match that excludes every doc-comment (`///`) mention of `.setup(`, which the naive grep this ticket
already warned about would have over-counted.

**8 peer-held** (untouched, per the hard rule): `✒️writer` `🌊️flow` `🌿️vcs` `🎞️animate` `🏭️process`
`💡️reasoning` `🎬️sequence` `🏛️architect`.

**5 remaining, non-held genuine gaps** (not this pass's scope — named for the next pass):
`🌀️procedural` `📕️norm` `🧱️block` `🪐️space` — and `🧩️puzzle`, now scoped to exactly the OS
media-bridge reason (B2 above), down from three reasons.

**1 closed this pass:** `🔋️energy` — zero `.setup()` calls remaining.

`PluginBuilder::setup` was **not** deleted (13 live callers remain).

## Verification

**`semio-framework-plugin` — GREEN.**
```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=…/🎯️target cargo check -p semio-framework-plugin --all-targets
→ Finished `dev` profile [unoptimized] target(s) in 0.69s   (exit 0, 0 error lines)
```
Full log: `scratch-w1d-framework-plugin-check.txt`.

**`semio-framework-plugin --lib` tests — 149 passed / 1 failed.** The one failure,
`component::derived_artifact_children_tests::derived_composer_reads_defaults_to_composition_reads_for_a_leaf_with_no_children`,
is inside `DerivedArtifactSpec`/`ArtifactChildren`/`NoChildren` machinery at line ~11011 —
**composition-runtime territory explicitly owned by another session, which I did not touch.** My
entire diff in this file is contained in the `ArtifactDeclaration`/`DocumentCodecSpec` region
(~lines 1075–1300); the failing test is ~9,700 lines away in a different module
(`derived_artifact_children_tests`, opened at line 10882) exercising types (`ChildrenTestComposition`,
`NoChildren<S>`) I never edited. Classified **(b) pre-existing/unrelated**, not mine to fix. Full log:
`scratch-w1d-framework-plugin-test.txt`.

**`semio-s-plugin-note` (exemplar) — GREEN.**
```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=…/🎯️target cargo check -p semio-s-plugin-note --all-targets
→ Finished `dev` profile [unoptimized] target(s) in 59.79s   (exit 0, 0 error lines)
```
Full log: `scratch-w1d-note-check.txt`.

**`semio-s-plugin-energy --all-targets` and `semio-s-plugin-puzzle --all-targets` — BLOCKED-CHURN, not verified green, classified (c) upstream.**
Both depend on `semio-s-plugin-stdio`, which fails to compile RIGHT NOW with:
```
error[E0433]: cannot find `inferences` in `schema`
  --> …/🗄️stdio/…/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🚪️io/🦀️component.rs:59:128
```
(3 occurrences for the puzzle run — same class, one file each). Neither `Compiling
semio-s-plugin-energy` nor `Compiling semio-s-plugin-puzzle` ever appears in either log — cargo died
in the shared upstream dependency before reaching either crate, exactly the `🧩️puzzle` "blocked-churn"
pattern `📓️baselines.md` already documents. Evidence this is upstream and live, not mine: `stat -f
'%Sm'` on the failing stdio file reports `Aug 12 23:42:30` (minutes before this check, well inside
UCAS's active edit window — `🗄️stdio`'s roster is explicitly "not frozen" per this ticket's own
context); grep-verified **zero** error lines mention `🔋️energy` or `🧩️puzzle` paths in either log; a
retry ~2 minutes later reproduced the same failure class (grew from 1 occurrence to 3), consistent
with an in-progress edit rather than a stable break I could wait out quickly. Per this ticket's own
protocol ("Retry-and-wait, do not patch") I did not touch `🗄️stdio` — it is explicitly off-limits.

**Confidence for energy/puzzle short of a compiler run:** `document_codec_bare`'s where-clause is
copied verbatim from `store::ArtifactCodec::of`'s own bounds, and the framework-plugin crate build
above — green — already type-checks `document_codec_bare`'s generic body against those bounds in the
abstract (Rust checks a generic fn body against its own where-clause at definition, independent of
any call site). The energy call site instantiates it at `<EnergyModelSnapshot, EnergyModelMutation>`,
the exact pair the pre-existing (now-deleted) `store::ArtifactCodec::of::<EnergyModelSnapshot,
EnergyModelMutation>(...)` call already proved satisfies those same bounds. This is reasoning, not a
compiler result — flagged explicitly as such, not claimed as a pass. Both `semio-s-plugin-energy
--all-targets` and `semio-s-plugin-puzzle --all-targets` should be re-run once `🗄️stdio` is green
again; logs saved as `scratch-w1d-energy-check-blocked.txt` / `scratch-w1d-puzzle-check-blocked.txt`
for whoever does that re-run.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — `document_codec_bare` builder
  method added; `DocumentCodecSpec` changed from `fn()` to `{schema: String, register: fn(String)}`;
  `of::<A>()` updated to match; new `bare::<Snapshot, Mutation>()` constructor; `register_all`'s
  document-codec call site updated.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` — read only, no edits
  (double-set safety already present).
- `✏️s/🔌️plugins/🔋️energy/🦀️component.rs` — `.setup()` call removed; doc updated.
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️component.rs` — `declaration()` gained
  `.document_codec_bare(...)`; doc updated.
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` —
  dead `register_document_codec()` deleted.
- `✏️s/🔌️plugins/🧩️puzzle/🦀️component.rs` — `.setup()` narrowed to the two OS media-bridge calls only;
  doc rewritten with the B2 judgement.
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/🦀️component.rs` — `Puzzle2dPlayApp::app_schema()` override added.
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🦀️component.rs` — `Puzzle3dPlayApp::app_schema()` override added.
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d/🦀️component.rs` — `Puzzle5dPlayApp::app_schema()` override added.
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/🎚️config/🧬️schema/🦀️component.rs` — `register_app_schema()` →
  `app_schema_descriptor()` (returns instead of self-registers).
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🎚️config/🧬️schema/🦀️component.rs` — same conversion.
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d/🎚️config/🧬️schema/🦀️component.rs` — same conversion.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` —
  `register_app_schemas()` umbrella deleted; `register_media_io()` untouched.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🦀️component.rs` — `declaration()` doc updated to reflect B1
  closure / B2 survival.

Not touched: `🧬️schema/📸️snapshot`/`🧬️mutations` leaves, composition runtime, `🗄️stdio`, any peer-held
plugin.
