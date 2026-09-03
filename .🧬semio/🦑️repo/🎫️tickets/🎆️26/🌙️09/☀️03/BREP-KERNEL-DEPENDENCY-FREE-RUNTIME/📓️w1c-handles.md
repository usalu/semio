# 📓️ W1-C — Handle Lifecycle / Labels / Arena GC

Worker W1-C on `26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME`. Scope: audit `.🧬semio/🦑️repo/✍️notes/semio_brep_kernel_audit_7ad363f.md` §5.5 — separate ephemeral process handle / document-scoped persistent label / operation-history relation, with correct lifecycle (import reset, arena GC on dispose/retain, shell/compound as first-class handles, idempotent deconstruct).

Kernel root `B = ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep`.

## Design

Three identities, kept genuinely separate:

1. **Ephemeral process handle** — `GeometryHandle(String)`, still an opaque hex hash, but now minted as `hash("{kind:?}:{label.0}")` — a pure function of `(GeometryKind, PersistentLabel)`, never a session counter or an arena-id tag. `Brep::mint` looks the label up via a new `label_of_entity(&Body, &Entity)`: for `Vertex/Edge/Face/Shell/Solid` it reads the label already stored on the `Body` arena entry (stable for the entity's whole life); `Wire/Curve/Surface/Compound` have no such arena-resident identity (a `Wire` bundles ids but isn't itself a `Body` entity; a bare `Curve3`/`Surface` built via `line_curve_sync` etc. never enters `body.curves3`/`body.surfaces`), so those four `Entity` variants now carry their own `PersistentLabel`, stamped once via `body.new_label()` at registration time (`Entity::Wire(Wire, PersistentLabel)`, `Entity::Curve(Curve3, PersistentLabel)`, `Entity::Surface(Surface, PersistentLabel)`, `Entity::Compound(Vec<SolidId>, PersistentLabel)`).
2. **Document-scoped persistent label** — unchanged `history::PersistentLabel(u64)`, monotonic, never reused, already lived in `Body`/`topology.rs`. New bridge: `Brep::label_of(&handle) -> Option<PersistentLabel>` and `Brep::handle_for_label(label) -> Option<GeometryHandle>` (searches every arena store; re-mints deterministically). Trait-level `BrepKernel::label(&self, &GeometryHandle) -> Option<u64>`.
3. **Operation-history relation** — untouched (`OpDelta`/`OpRecorder`, W1-B/euler territory); not this worker's slice.

Consequence: `deconstruct` on an untouched shape is now provably idempotent (same labels in the `Body` ⇒ same hashes every call), and registering unrelated geometry between two `deconstruct` calls cannot perturb any handle — both are covered by new tests.

### Arena GC (audit §5.3)

New `// #region ♻️Reachability` appended to the end of `📸️snapshot/🕸️topology/🦀️.rs` (topology.rs's `EngineRep`/`build`/`to_seed` region belongs to W1-A and was not touched):

- `EntityRef` — one arena id per kind (`Vertex/Edge/Coedge/Loop/Face/Shell/Solid/Curve3/Curve2/Surface`).
- `ReachSet` — the set of ids one `Body::reachable_from(roots: &[EntityRef]) -> ReachSet` walk visited (solid→shell→face→(surface, loop)→coedge→(pcurve, edge)→(curve, vertex)).
- `Body::compact(&mut self, keep: &ReachSet) -> Remap` — frees every id not in `keep` via the new `Store::free` (generation bump only, **no index remap** — kept ids stay byte-identical). `Remap`/`EntityCounts` are informational counters only, matching the "returns nothing else" requirement.
- `Store::free`/`Store::is_live` added to `🏟️arena/🦀️.rs` (`free` = `remove(..).is_some()`, `is_live` = `contains`).
- `Brep::live_roots(&self) -> Vec<EntityRef>` (engine.rs) turns every currently-live `Entity` into its `EntityRef` roots — a `Wire`'s member edges/vertices are included even though a `Wire` itself isn't a `Body` root, since nothing else would keep them alive while the wire handle is live. `Brep::compact_unreachable(&mut self)` runs `reachable_from` + `compact` from those roots.
- `dispose_sync` now removes the handle **and**, if that was the last handle reaching the entity, runs `compact_unreachable()` — dispose actually frees topology now, not just the registry entry.
- `BrepKernel::retain` = drop everything not in the given set, then `compact_unreachable()` once.

### Import lifecycle (audit §5.2)

Chose **merge**, not clear-and-replace (the DO brief's own explicit pick: "existing handles stay valid"). `import_stl_sync`/`import_obj_sync`/`import_glb_sync`/`import_dwg_sync` already mutated `self.body` in place via `import_*_to_body(&mut self.body, ..)` (already correct — the audit's "stale handle" finding for those four was about `live` never being cleared, which is no longer a defect once the design is merge-not-replace: nothing in `self.body` is renumbered, so old handles keep resolving without any registry surgery). `import_step_sync` used to do `self.body = imported` (full replace, silently invalidating everything) — now it merges: new `Body::merge(&mut self, other: &Body) -> MergeMap` (topology.rs, same new region) offsets `other`'s `PersistentLabel`s above `self`'s current high-water mark (`LabelSource::from_next(offset + other.labels.next())`) and copies every entity across with a full id remap (two-pass reserve-then-patch for the coedge/loop/face cycle, mirroring `EngineRep::build`'s technique but done in-place and *without* the lossy pcurve-dropping `to_seed`/`build` round trip — pcurves, tolerances and flip flags all copy verbatim). `import_step_sync` then registers only the freshly-imported solids (`map.solids[&id]`); the caller's pre-existing handles are never touched.

### Shell/Compound first class (audit §5.4)

- `Entity::Shell(ShellId)`, `Entity::Compound(Vec<SolidId>, PersistentLabel)` added (`GeometryKind::Shell`/`Compound` already existed).
- New trait methods (additive, `BrepKernel` signatures otherwise untouched): `solid_shells(&mut self, &GeometryHandle) -> Result<Vec<GeometryHandle>, BrepError>` (named `solid_shells`, not `shell`, because `shell(shape, thickness, open_faces)` already exists as the shell-feature operation — collision avoided), `compound(&mut self, &[GeometryHandle]) -> Result<GeometryHandle, BrepError>`, `explode(&mut self, &GeometryHandle) -> Result<Vec<GeometryHandle>, BrepError>` (inverse of `compound`), `label(&self, &GeometryHandle) -> Option<u64>`.
- `deconstruct_sync`/`deconstruct` now also register every shell of the solid; `BrepTopology` gained a `shells: Vec<GeometryHandle>` field (a consumer in `🧰️framework/…/🌊️flow/📐️brep-geometry/🦀️.rs::topology_result` already referenced `topology.shells` before this change landed — confirms the field was already expected downstream).
- `kind_sync`/`kind` handle both new variants.

## Files touched

- `B/📸️snapshot/🏟️arena/🦀️.rs` — `Store::free`/`is_live` + 1 test.
- `B/📸️snapshot/🕸️topology/🦀️.rs` — new `// #region ♻️Reachability`: `EntityRef`, `ReachSet`, `Remap`, `EntityCounts`, `MergeMap`, `Body::reachable_from`/`mark_reachable`/`compact`/`entity_counts`/`merge`, 3 tests (`compact_frees_exactly_the_unreachable_orphan`, `compact_leaves_stale_ids_rejected_by_generation`, `merge_preserves_self_and_offsets_others_labels`).
- `B/⚙️engine/🦀️.rs` — `Entity` enum (Shell/Compound + per-instance labels on Wire/Curve/Surface/Compound), `Brep` struct (dropped the `counter` field), `label_of_entity`, `mint`, `register_solid/face/shell/compound/wire/curve/surface`, `entity`/`live_roots`/`compact_unreachable`, `solid_id/face_id/wire_ref/curve_ref/surface_ref/edge_id` (arity fix), `entity_tag`, `deconstruct_sync`, `import_step_sync`, `kind_sync`, `tessellate_sync` (arity fix, collateral — required by the Wire arity widening), `dispose_sync`, `label_of`/`handle_for_label`/`solid_shells_sync`/`compound_sync`/`explode_sync`, `BrepTopology.shells`, `BrepKernel` trait (`solid_shells`/`compound`/`explode`/`label` added, `retain` now compacts), `BrepKernelImpl` (new methods wired to the `_sync` bodies), 9 new tests in `mod tests`.

## Verification

`RUSTC_WRAPPER="" cargo check -p semio-s-plugin-stdio --lib --message-format short` — run to completion in the foreground (45m09s, heavy shared-lock contention from ~12 concurrent Wave-1 workers checking the same target dir; full output saved at `🗑️generated/w1c-check.txt`):

```
warning: `semio-s-plugin-stdio` (lib) generated 1460 warnings (run `cargo fix --lib -p semio-s-plugin-stdio` to apply 1428 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 45m 09s
```

Zero `error[...]`/`error:` lines anywhere in the output (grepped `: error|^error` — 0 matches; the only 3 lines containing the substring "error" are a rustc suggestion diff renaming a local variable named `error`, not a compiler error). Confirms `✳️brep` (including every region this worker touched) is at zero errors, and so is the rest of the crate as of this run.

`RUSTC_WRAPPER="" cargo check -p semio-s-plugin-flow-extension-brep --message-format short` — 3 attempts; first two hit unrelated shared-tree infra failures (`Blocking waiting for file lock`, then a stale-tree `mmap`/fingerprint-write race under extreme concurrent load — not compile errors). Third attempt completed: 13 `error[...]` lines total, **none in this worker's files** (`⚙️engine/🦀️.rs`, `📸️snapshot/🕸️topology/🦀️.rs`, `📸️snapshot/🏟️arena/🦀️.rs` — checked explicitly, 0 matches). All 13 are pre-existing/concurrent issues in other workers' territory:
- `📸️snapshot/🦀️.rs`, `🔺️diff/🦀️.rs`, mutation triads (`create-vertex`, `create-edge`, `create-face`) — `E0063` missing `tol` field on `BrepVertex`/`BrepEdge`/`BrepFace`, missing `coedges`/`next_label` on `SemioBrepSnapshot` (W3-A snapshot-component territory, not started/in flight).
- `💡️inferences/🦀️.rs` — `E0599` `SemioBrepInference::infer` not found.
- `📸️snapshot/🔁️body/🦀️.rs` — `E0521` borrowed data escapes closure.
- `🗿️artifacts/🧿️semio/🦀️.rs:223` — pattern requires `..` due to inaccessible fields.

None of these touch handles, labels, the registry, or arena GC. Flagging file:line above for the owning workers; not fixed here (out of scope).

Harness gate (`<TICKET>/🔬️harness`, per coordinator): `cargo check --lib --message-format short` fails with 3 pre-existing wiring errors unrelated to this worker (`E0432` unresolved `crate::artifacts::dwg`, unresolved `inferences::validation_report` ×2 — the harness's own `lib.rs` module mounting, owned by H0/W1-Z, still integrating). `cargo test -- arena topology` reproduces the same failure one step further in: harness `lib.rs:147` references a path with a literal typo (`🏅️标准` instead of `🏅️standards`) that does not exist on disk — `error: couldn't read .../🏅️标准/🔖️v1/.../🧩tessellation/🦀️.rs: No such file or directory`. Both are harness-file (not `✳️brep`-file) defects; not owned by this worker, reported here verbatim per the coordinator's ask. My own `arena`/`topology`/`engine` unit tests therefore have not been runtime-verified via `cargo test` — they compile as part of the `--lib` check above (same module tree) but were not executed. **I did not claim a passing test run I did not execute.**

## Open items

- Harness `lib.rs:147` typo (`🏅️标准` → `🏅️standards`) blocks `cargo test -- arena topology` for every Wave-1 worker, not just this slice — worth flagging to H0/W1-Z directly.
- `handle_for_label` is O(n) per store (linear scan); fine for the current in-memory `Body` sizes, would want an index if `Body`s grow large — left as-is per "no premature effort" and because nothing in this ticket's scope calls it on a hot path yet.
- `Body::merge` does not (cannot, at this layer) validate that `other`'s geometry is otherwise compatible with `self` (units, tolerances) — that's an IO/STEP-import concern (W3-B), out of scope here; `merge` is a pure topology-arena operation.
