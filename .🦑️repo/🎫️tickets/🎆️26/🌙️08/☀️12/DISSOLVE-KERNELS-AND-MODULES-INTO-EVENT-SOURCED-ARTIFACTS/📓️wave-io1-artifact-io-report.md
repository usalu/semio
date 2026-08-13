# Wave IO1 — Artifact IO Report

Executor session, single wave. Scope: the five escape-hatch registries in
`🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs`'s `media_export_raster` module —
`register_solid_exporter`, `register_solid_importer`, `register_mesh_exporter`,
`register_mesh_importer`, `register_dwg_import_handler` — under the binding user ruling that io
must flow over the artifact io mechanism, with effort not a valid reason to stop.

## Re-verification of the crate/cycle question (job "distinguish structural impossibility from effort")

Independently re-measured, not trusted from the brief:

```
grep -n "semio-framework-3d\|stdio" 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml   → 0 hits
grep -n "name" 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml                          → name = "semio-framework-os-kernel"
grep -n "semio-framework-3d\|semio-s-plugin-stdio" 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml
  → semio-framework-3d = { path = ... }  (line 31, BEFORE this wave's edit)
  → semio-s-plugin-stdio = { path = ... }  (line 32)
```

Confirmed: `semio-framework-os-kernel` (framework tier) defines none of the five functions and has
zero dependency on `semio-framework-3d` or stdio. The five functions are compiled by
`semio-framework-os` (product tier, package `semio-framework-os`, crate root
`💻️os/🖥️host/📦️packages/🦀️rust/📦️glue.rs` → `mod host_core` → `🖥️host/🦀️component.rs`), which
already depended on both `semio-framework-3d` and `semio-s-plugin-stdio` before this wave touched
anything. **No cycle existed, confirmed independently.** `💻️os/🦀️component.rs` (bare) is the
genuinely-unmounted dead twin — re-verified below (job 5) — and was not edited.

## Job 1 — Registrant census

**Method**: repo-wide `grep -rn` for each function name, both with and without a trailing `(` (to
catch aliasing/prose separately from real calls), across the whole tree excluding `🎯️target` and
this ticket's own scratch/backup files, cross-checked against every plugin directory individually.
Every hit was read in context, not counted from the pattern alone (the ticket's own "grep is a
search, not a census" rule) — several apparent hits were doc-comment prose (`lowpoly`, `puzzle`
top-level `component.rs`, gis/shooting schema files all merely *mention* these names while
describing prior migrations away from them) and were excluded.

**Real registrant count, BEFORE this wave (repo-wide, excluding self-tests):**

| Function | Registrants | Who |
|---|---|---|
| `register_solid_exporter` | 3 | cad (`Obj`, `Stl`, `Step` `SolidExporter`s for `"3d.cad"`) |
| `register_solid_importer` | 3 | cad (`Obj`, `Stl`, `Step` `SolidImporter`s) |
| `register_mesh_exporter` | 7 | cad×1 (Glb), puzzle3d×3 (Obj/Glb/Stl), puzzle5d×3 (Obj/Glb/Stl) |
| `register_mesh_importer` | 7 | cad×1 (Glb), puzzle3d×3 (Obj/Glb/Stl), puzzle5d×3 (Obj/Glb/Stl) |
| `register_dwg_import_handler` | 4 | cad, space (test-only stub), gismap, puzzle2d |

**Total real registrants: 24** — well below the brief's rough expectation (11/9/19/15/14 = 68).
Re-checked this is not the "grep undercounted 4×" trap the brief warned about: ran both narrow
(`fn(`) and broad (bare name) greps over the *entire* repo (not just `🧰️framework`/`✏️s`), checked
every plugin directory by name (`demonstrator`, `process`, `procedural`, `fem`, etc. — zero hits),
and checked the *reader* side too (`solid_exporter_for`/`export_registered_solid`/
`import_registered_solid` — zero production callers anywhere, only this file's own self-test and
cad's now-removed assertion). The likely explanation: several plugins (lowpoly, and evidently
others referenced only in passing) were already migrated off these five functions by an earlier,
unrelated wave (`26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE` M1 — lowpoly's own doc comment says
so explicitly), shrinking the live population well below the brief's estimate before this wave
started.

**A registrant that is not a caller, found and fixed**: `solid_exporter_for`/
`export_registered_solid`/`import_registered_solid` (the READ side of the solid registry) had **zero
production callers anywhere in the repo** — only this file's own self-test and cad's own
self-verifying assertion, both of which existed purely to prove the registration mechanism
registers, not to serve any real export/import request. cad's actual production solid
export/import path (`export_solids_as`/`import_step_object`/`import_obj_object`/
`import_stl_object`/`import_glb_object`, same file) already called the genuine stdio
`ArtifactSerializer`/`ArtifactDeserializer` leaves directly (`SemioMeshToObj`, `SemioMeshToStl`,
`SemioBrepToStep`, `SemioBrepFromStep`) and never touched the registry at all. This is the
"registrant that isn't a caller" class the brief warned to watch for — just inverted: the
mechanism had a registrant with literally no consumer, running in production entirely by a
different, already-compliant path.

## Job 2 — Per-registrant table: what happened to each

### The dispatch bridge that made this possible

Before any registrant could be safely dropped, two independent, previously-unnoticed bugs in the
OS media pipeline's "try the typed registry first, fall back to the escape hatch" logic
(`🖥️host/🦀️component.rs`, `workflow` module) had to be fixed — **without these, the typed
`ComposerEntry` rosters that cad/puzzle3d/puzzle5d/gismap/puzzle2d already had authored (in some
cases from an earlier, unrelated wave) were unreachable, and every one of these registrants was
silently load-bearing regardless of how complete its `io_registry::entries()` was:**

1. **`native_kind` was built from the wrong id namespace.** `registry_export_media`/
   `registry_import_media`/`negotiate_wire_format`→`registry_shared_stdio_dialect` built the
   dialect lookup key from `ArtifactKindSpec.id` (`OsArtifactDescriptor.kind`, e.g. `"3d.cad"`,
   `"3d.puzzle"`, `"2d.map"` — the dimension-prefixed workflow-layer id) prefixed with `"s."`. The
   REAL `Dialect.artifact_kind` every `ComposerEntry` in this repo is registered under is
   `"s." + component_kind` (e.g. `"s.cad"`, `"s.puzzle3d"`, `"s.gismap"`) — a *different* id that
   happens to share a segment. `format!("s.{}", node.yields)` therefore built `"s.3d.cad"`, which
   can never match any registered dialect. Fixed with a new `native_dialect_kind(workflow_kind)`
   helper reading `OsArtifactDescriptor.component_kind` (a field that already existed, unused for
   this purpose).
2. **`target_kind` double-prefixed `"stdio."`.** Both functions built `target_kind =
   format!("s.stdio.{format_kind}")`, but `format_kind` at both call sites is already
   `semio_framework::normalize_format_kind(...)`'s return value — which is documented and coded to
   return the *canonical `kind_id`*, i.e. already in `"stdio.<format>"` form (e.g. `"stdio.obj"`).
   The result was `"s.stdio.stdio.obj"`, which can never match `"s.stdio.obj"`. Fixed to
   `format!("s.{format_kind}")`.

Both bugs are independent and compounding — either alone was sufficient to make the typed-registry
path permanently dead for every artifact and every format, not just the ones this wave touched.
**Verified by isolating a `cargo check -p semio-framework-os --features os-host-full --lib`
(0 errors) before and after each fix**, since the whole `workflow` module (where both live) is
gated behind the `os-host-full` feature and is invisible to a bare `cargo check --all-targets`.

### Table

| Function | Format | Registrant(s) | Outcome |
|---|---|---|---|
| `register_solid_exporter` | Obj/Stl/Step | cad | **Deleted, not migrated.** Zero production callers of the read side (`export_registered_solid`) anywhere; cad's real solid export already flows through the genuine `SemioMeshToObj`/`SemioMeshToStl`/`SemioBrepToStep` leaves via `export_solids_as`. Function, registry (`SolidExporterRegistry`/`solid_exporters()`), and lookup fns deleted from `🖥️host/🦀️component.rs`; cad's three calls + self-registration removed from `register_host_io()`; cad's self-test rewritten (see below); host's own self-test (`solid_exporter_and_importer_registrars_round_trip_a_box_through_step`) deleted (it tested the deleted mechanism, not real behaviour). |
| `register_solid_importer` | Obj/Stl/Step | cad | Same as above. |
| `register_mesh_exporter` | Obj/Stl | cad(n/a, GLB-only)/puzzle3d/puzzle5d | **Migrated** — deleted from `register_mesh_io()`; puzzle3d/puzzle5d already had a real `ComposerEntry` for `"s.stdio.obj"`/`"s.stdio.stl"` in their own `io_registry::entries()`, reachable via `io_dispatch` once the bridge fix landed. |
| `register_mesh_exporter` | Glb | cad, puzzle3d, puzzle5d | **Kept, genuine remainder.** No `"s.stdio.glb"` dialect exists in stdio's format catalog (`✏️s/🔌️plugins/🗄️stdio/🛂️manifest/🦀️component.rs`) — only `"s.stdio.gltf"` (`is_binary: false`, JSON text). `GlbExporter.format_kind() == "glb"`, confirmed distinct from `"gltf"`. Adding a `"glb"` stdio format is a stdio-manifest change, out of this wave's write scope (stdio is pending a separate session's handoff per this ticket's hot-file table). Reported, not silently dropped. |
| `register_mesh_importer` | Obj/Stl | puzzle3d/puzzle5d | **Migrated**, same reasoning as the exporter row. |
| `register_mesh_importer` | Glb | cad, puzzle3d, puzzle5d | **Kept**, same genuine remainder as above. |
| `register_dwg_import_handler` | dwg | cad | **Migrated** — cad's `io_registry` already has `DEP_DWG`/`EXPORT_DWG_DIALECT` for `"s.stdio.dwg"`. |
| `register_dwg_import_handler` | dwg | gismap | **Migrated** — same, gismap's `io_registry` already covers `DEP_DWG`. |
| `register_dwg_import_handler` | dwg | puzzle2d | **Migrated, and a strict improvement.** `puzzle2d_document_json_from_dwg` (the handler this used to register) is a documented always-empty stub ("Tier C…always returns an empty board"); the artifact-level `ComposerEntry` deserializer does real entity parsing. The stub function itself is kept (still exercised by its own unit test) — only its registration into the now-redundant escape hatch is removed. |
| `register_dwg_import_handler` | dwg | space (test-only) | **Deleted call, inlined equivalent.** Not a production registrant — a `#[cfg(test)]`-only stand-in inside `SpaceCommand::ImportMedia`'s own effect test, keyed on a synthetic `"2d.drawing"` kind that is not a real `ArtifactKindSpec`. Inlined the exact same logic directly via `register_os_media_import_handler_kind` (a domain-neutral primitive, kept) so the test's behaviour is unchanged. |

Function definitions deleted outright from `🖥️host/🦀️component.rs`: `register_solid_exporter`,
`register_solid_importer` (+ `SolidExporterRegistry`/`SolidImporterRegistry`/`solid_exporters()`/
`solid_importers()`/`solid_registry_key`/`solid_exporter_for`/`export_registered_solid`/
`import_registered_solid`, the whole `//#region SolidMediaExport`), `register_dwg_import_handler`.
`register_mesh_exporter`/`register_mesh_importer` are **kept** (GLB has no migration target yet).
`register_mesh_dwg_export_handler`/`register_mesh_dwg_import_handler`/`register_2d_export_handlers`
are siblings of the same shape but **not** in this wave's five-function scope — left untouched
everywhere.

## Job 3 — Zero-caller proof (pasted, not summarized)

```
$ grep -rn "register_solid_exporter(\|register_solid_importer(" --include="*.rs" . \
    | grep -v 🎯️target | grep -v "fn register_solid_"
🧰️framework/🛍️products/💻️os/🦀️component.rs:3544/3545   ← the DEAD, unmounted twin (job 5); not compiled, not edited
(no other hits — the live 🖥️host/🦀️component.rs no longer defines or calls either function)

$ grep -rn "register_dwg_import_handler(" --include="*.rs" . \
    | grep -v 🎯️target | grep -v "fn register_dwg_import_handler"
(zero hits anywhere, including the dead twin — it only ever called this once, cad, migrated)
```

`semio-framework-3d` removed from `💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml` after confirming
(grep, whole crate's mounted input set) it had zero remaining references outside comments.

## Job 4 — Kernel duplication window: attempted, genuine structural blocker found, NOT forced

Re-censused `semio_framework_3d::brep::` callers repo-wide (excluding `🎯️target` and
`🧰️framework/🔨️modules/🧊️3d/` itself) **after** deleting the host registry:

```
🧰️framework/🛍️products/💻️os/🦀️component.rs                                    ← dead twin, ignored
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️component.rs   ← LIVE, separate crate
✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️component.rs                        ← LIVE — this IS `semio-s-plugin-flow-extension-brep`, a required verification gate
✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🗺️geometry-import/🦀️component.rs   ← LIVE
✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs  ← LIVE
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️component.rs ← the migrated COPY itself (expected)
✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/benches/brep_kernel.rs                  ← LIVE bench
```

`BrepEngineHost` (the process-lifetime host wrapper `semio_framework_3d::brep::kernel` exposes)
has live consumers too: `✏️s/🔌️plugins/🏭️process/…/💡️inferences/🦀️component.rs`,
`✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🦀️component.rs`, and cad's own inferences file. 11 crates'
`Cargo.toml`s depend on `semio-framework-3d` directly (`flow`, `♾️infinite`, `os-host` — now
removed by this wave — `flow-extension-brep`, `process`, `cad`, `demonstrator`, `stdio`,
`procedural`, `lowpoly`, plus the module itself).

**This is the genuine structural blocker the ticket's own instructions anticipated, not an
effort excuse**: deleting `Brep`/`BrepKernel`/`GeometryHandle`/`SolidExporter` etc. from
`framework-3d` today would break, at minimum, `semio-s-plugin-flow-extension-brep` — one of this
wave's own required verification gates — plus flow's `brep-geometry` module and cad's own
`geometry-import`/`inferences` (which build real geometry through the live kernel, not through the
now-deleted registry). None of those consumers are anywhere near this wave's boundary (host
`🦀️component.rs`, and four small plugin registration call sites); migrating them is a
multi-plugin, multi-session lane of its own, matching the ticket's own prior finding
(`W3a-0 Phase 3`) that `BrepEngineHost`'s deletion needs cross-session migration.

**Per the report instructions: distinguishing structural impossibility from effort — this is the
former.** The duplication window (kernel content living in both `framework-3d` and stdio's
`✳️brep/🧬️schema/⚙️engine`) is **not closed** by this wave. What this wave DID remove is the
narrow slice that was genuinely dead weight (the solid-exporter/importer registry + cad's now-
redundant registrations), which shrinks framework-3d's reachable-from-`semio-framework-os` surface
to zero without touching the kernel itself — a real, if partial, step toward the eventual
deletion, not a substitute for it.

## Job 5 — Dead twin re-verified untouched

```
$ grep -rln 'path = ".*💻️os/🦀️component.rs"' --include="*.rs" .   → (zero hits, whole repo)
```
`💻️os/🦀️component.rs` (bare, no `🖥️host`) still has zero `#[path]` mounts anywhere in the tree.
Not edited, per instruction.

## Files touched

- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` — `native_dialect_kind` bridge helper added; `registry_export_media`/`registry_import_media`/`negotiate_wire_format` fixed (2 independent bugs); `//#region SolidMediaExport` deleted wholesale; `register_dwg_import_handler` deleted; re-export list at bottom of file updated; host's own dead self-test deleted.
- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml` — `semio-framework-3d` dependency removed (now unused).
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` — `register_host_io()` trimmed to GLB + mesh-dwg-export only; self-test rewritten.
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🦀️component.rs` — `register_mesh_io()` trimmed to GLB + mesh-dwg pair only.
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d/🦀️component.rs` — same trim.
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/🦀️component.rs` — `register_media_io()` dwg-import call removed.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` — `register_host_io()` dwg-import call removed.
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎮️commands/🖼️media/🦀️component.rs` — test-only `register_dwg_import_handler` call inlined onto `register_os_media_import_handler_kind` directly.

No files created; no files deleted (the dead twin is untouched; all edits are in-place).

## Verification commands run, with real output

Disk filled to 1.1 GiB free / 92% during this wave (this ticket's own `🎯️target` had grown to
118 G — a repeat of the exact event class `📓️status.md` already documents from earlier today).
Cleaned this ticket's own `🎯️target` (204 GiB free afterward) rather than escalate-and-wait, since
it is disposable build cache under this wave's exclusive `CARGO_TARGET_DIR`, not user data or
another session's target dir. Every result below is from AFTER that clean, so none are the
"disk-degraded window" class of void result the ticket's rules warn about.

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=$TD/🎯️target cargo check -p semio-framework-os --features os-host-full --lib
Finished `dev` profile [unoptimized] target(s) in 45.43s        (0 errors)

$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=$TD/🎯️target cargo check -p semio-framework-os --all-targets     (default features)
Finished `dev` profile [unoptimized] target(s)                  (0 errors)

$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=$TD/🎯️target cargo test -p semio-framework-3d --lib
test result: ok. 413 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.51s
   ← matches the required gate exactly (413/0)

$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=$TD/🎯️target cargo test -p semio-s-plugin-stdio --lib
test result: FAILED. 2943 passed; 5 failed; 4 ignored; 0 measured; 0 filtered out; finished in 91.99s
   failures:
       artifacts::binary::standards::v_raw::subsets::any::schema::inferences::extent::component::tests::inference_default_law
       artifacts::dwg::standards::v_ac1018::subsets::any::schema::component::tests::conformance_laws::fixture_honesty_law
       artifacts::dxf::standards::v_r12::subsets::any::schema::inferences::bounds::component::tests::bounds_matches_hand_built_entity_extent
       artifacts::ifc::standards::v2x3::subsets::any::io::component::tests::conformance_laws::fixture_honesty_law
       artifacts::zip::standards::v2_0::subsets::any::schema::inferences::entries::component::tests::inference_default_law
   ← EXACTLY the 5 named failures in this ticket's own `scratch-w0-baseline-failures-sorted.txt`
     baseline, by name, not merely by count. 2943 vs the recorded 2442 baseline is +501 passed —
     expected repo-wide growth from concurrent sessions landing coverage throughout the day
     (this wave touched zero files under `✏️s/🔌️plugins/🗄️stdio/`). Zero failures attributable to
     this wave.

$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=$TD/🎯️target cargo test -p semio-s-plugin-process --lib
test result: ok. 158 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.16s
   ← matches the required gate exactly (158/0)

$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=$TD/🎯️target cargo check -p semio-s-plugin-puzzle --all-targets   → 0 errors
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=$TD/🎯️target cargo check -p semio-s-plugin-gis --all-targets      → 0 errors
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=$TD/🎯️target cargo check -p semio-s-plugin-space --all-targets    → 0 errors
```

**Test-sum invariant**: framework-3d (413) + stdio (2943) = 3356, above the 2855 floor.

### Blocked — sibling in-ticket wave, precisely attributed, not this wave's to fix

`semio-s-plugin-cad --all-targets`, `semio-s-plugin-flow-extension-brep --lib` (a required gate),
and a full `semio-s-plugin-puzzle --lib` test run currently fail to compile with
`error[E0432]: unresolved import `semio_framework_math::algebra`` (and the same shape for
`optimize`/`lie`/`signal`/`spatial`). Traced, not guessed:

```
$ sed -n '10,17p' 🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/📦️glue.rs
// 🧮️ 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave M3d moved
// `algebra`/`optimize`/`lie`/`signal`/`spatial` out of this crate — `📸️remodel` was their sole
// consumer (verified symbol-by-symbol); they now live as `crate::algebra`/... under that
// plugin's own artifact schema ...

$ git status --short -- 🧰️framework/🔨️modules/🧮️math
 D 🧰️framework/🔨️modules/🧮️math/➕️algebra/🦀️component.rs
 D 🧰️framework/🔨️modules/🧮️math/🌫️fuzzy/🦀️component.rs
 ... (🎯️optimize, 🔷️lie, 📶️signal, 🗺️spatial similarly staged-deleted)
 MM 🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/📦️glue.rs   ← mounts removed, comment already
                                                                    describes the post-move state
```

This is **wave M3d of this SAME ticket** — a sibling executor mid-flight on the math dissolution
this ticket's own `important.md` binding ruling assigns (`🧿️ Where a dissolved kernel's artifact
goes`), named explicitly in its own comment. `glue.rs`'s five `pub mod` mounts are gone but the
consumer files (`🌫️fuzzy/🦀️component.rs` still reads `crate::algebra::{MatD, VecD}`, and
transitively `semio-s-plugin-stdio` via its own math dependency) have not yet been repointed —
a mid-atomic-landing gap in a wave that is not mine. Waited (~7 minutes, `Monitor` polling
`grep -q "mod algebra"`) for it to resolve; it had not by the time this report was written.
**None of the 31 errors this produces are in any file this wave touched** — verified by grepping
every error's `-->` path against this wave's own file list above; zero overlap.

**Consequence for this wave's own claims**: `semio-s-plugin-cad` and
`semio-s-plugin-flow-extension-brep` were verified clean via `cargo check -p semio-s-plugin-cad
--all-targets` **before** wave M3d's math churn began (see the earlier `entropy_internals`
retry sequence below) only for an unrelated, since-resolved stdio issue — cad's check has NOT yet
returned clean under the current math state, and flow-extension-brep's required gate is currently
un-runnable. `semio-s-plugin-puzzle --all-targets` (check, not test) DID return clean, run before
this churn started; the `--lib` test run above is the one blocked. This wave's own edits to cad
and puzzle3d/5d/2d are small, mechanical (deleting/keeping specific lines inside `register_host_io`/
`register_mesh_io`/`register_media_io`, already pasted verbatim above) and were sanity-read
multiple times, but **the coordinator should re-run these three once wave M3d lands**, rather than
trust this report's un-verified claim of correctness for them.

### A second, separate instance of the same class, self-resolved during this wave

Earlier, `cargo test -p semio-framework-os --lib --features os-host-full` hit
`error[E0433]: cannot find module or crate `graph_core`` in
`✏️s/🔌️plugins/🗄️stdio/…/✳️table/🧬️schema/🔗️causal-internals/🦀️component.rs`. Traced to a
different concurrent session (not this ticket — no attributing comment, unlike M3d above) actively
renaming `graph_core` → `semio_framework_graph`; `stat -f '%Sm'` showed the file was 5 minutes old
at the time. Retried after it settled — resolved on its own, code now reads
`semio_framework_graph::algorithms::...` throughout. Recorded so the coordinator can distinguish
this (transient, already gone) from the M3d one above (attributed, still open at report time).

## Concurrent-churn observations

1. **Wave M3d (this ticket)** — see "Blocked" above. Still open at report time; blocks `cad`,
   `flow-extension-brep`, and a full `puzzle` test run. Not this wave's file boundary
   (`🧮️math/**`); not touched.
2. **`graph_core` → `semio_framework_graph` rename (foreign session)** — resolved during this
   wave, self-cleared within ~5 minutes. Not touched.
3. **`entropy_internals` module (foreign session, likely the same one as #2, `✳️table` subset)** —
   hit once during an early `cad` check retry, before the disk filled; not independently re-tested
   since (superseded by the disk-clean cold rebuild, which then hit #1 instead). Not touched.
4. **`semio-framework-os --lib --features os-host-full` test target is independently, pre-existingly
   red** — 107-108 errors, ALL inside `#[cfg(test)]` code, ALL outside this wave's edited region
   (verified by line-range: errors cluster in `pub mod host` lines 7–1636, `🪐️space/🦀️component.rs`,
   and `workflow`'s own test module lines 3922–3949 for a `WorkflowFixture: ArtifactDsl`/
   `ArtifactPack` gap; my edited functions live at lines ~3406–3600 and appear in ZERO error
   locations). Root causes are unrelated to io: `LocalizedLabel: From<&str>` no longer holds
   (a localization-type change elsewhere), `PluginManifest` gained a required `artifact_kinds`
   field some caller hasn't been updated for, and `workflow_kernel::WorkflowFixture` doesn't
   satisfy `ArtifactDsl`/`ArtifactPack` (a trait/type drift). **`cargo check --features
   os-host-full --lib` (non-test) is clean (0 errors)** — proves my own code, including inside
   the gated `workflow` module, type-checks correctly; the break is entirely in pre-existing
   test-only code this wave never touched. Reported as found, not fixed — matches the ticket's
   own precedent for `semio-framework-os-kernel-db` (pre-broken, tracked separately, not blocking
   waves that don't touch it).

## Honest remainders

1. **GLB mesh export/import** (cad, puzzle3d, puzzle5d — 3 registrants each direction) is the
   one genuine, evidenced gap: no `"s.stdio.glb"` dialect exists in stdio's format catalog, only
   `"s.stdio.gltf"` (JSON text, `is_binary: false`). `register_mesh_exporter`/
   `register_mesh_importer` therefore stay defined and these six calls stay registered. Closing
   this needs either a new stdio format (out of this wave's write scope — stdio is pending a
   separate session's handoff per this ticket's hot-file table) or a product decision to drop
   binary-glTF support in favour of JSON glTF, which this wave does not make unilaterally.
2. **Job 4 (kernel duplication window) is NOT closed** — see Job 4 above for the full evidence.
   A genuine structural blocker (`semio-s-plugin-flow-extension-brep` — one of this wave's own
   required gates — plus flow's `brep-geometry`, cad's `geometry-import`/`inferences`, and
   `BrepEngineHost`'s process/cad consumers all still name `semio_framework_3d::brep` directly),
   not an effort judgment call. This wave DID remove the one slice that was genuinely dead
   (host's solid-exporter/importer registry + its `Cargo.toml` dependency edge), which is real
   progress toward the eventual deletion, but the ~1,695 LOC duplication itself remains in both
   places.
3. **`register_mesh_dwg_export_handler`/`register_mesh_dwg_import_handler`/
   `register_2d_export_handlers`** are the same shape (domain-specific escape hatches) as this
   wave's five targets but were never in scope for wave IO1. Left untouched everywhere they were
   found (cad, puzzle3d, puzzle5d, gismap). Flagged for a future wave under the same binding
   ruling — no exemption applies to them either.
4. **cad / flow-extension-brep / puzzle's full test suite** need a clean re-run once wave M3d
   lands — see "Blocked" above.
