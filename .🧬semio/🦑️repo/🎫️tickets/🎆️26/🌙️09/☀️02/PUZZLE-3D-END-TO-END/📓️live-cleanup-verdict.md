# 🩺 Verdict: `runtime live cleanup faulted for instance 1` — puzzle3d

## Verdict: (a) — structurally impossible on the current puzzle3d boot path

Against **current source** (not the Sep 1 wasm the original measurement used), puzzle3d's document,
config and interaction stores are all guaranteed to have their `snapshot_retirement_factory` installed
before any actor turn runs. The `Err(ValidationFailed("snapshot read retirement factory is not
installed"))` branch at
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:14748-14750` is dead code for puzzle3d's document
store on the path traced below. If the Sep 1 build still showed this fault, either the wasm was stale
relative to source at measurement time, or the fix (the `Some(...)` overrides in the editor file, item 1
below) landed after Sep 1 and simply hasn't been re-verified since — which is exactly the "must be
re-confirmed" caveat both prior notes already flagged.

**This is a source-only finding. No cargo/build/test was run** (forbidden by task scope) — see
"What would still be worth running" at the end for the one remaining empirical check.

## The proof chain, with line numbers

1. **Puzzle3d overrides both hooks to `Some`.**
   `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:6710-6716`
   (`impl ArtifactEditor for Puzzle3dPlayApp`):
   ```rust
   fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
       Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
   }
   fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> {
       Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
   }
   ```
   (This is the file another agent currently owns — read only, not edited, per the task's hard rule.)

2. **The generic `EditorApp<E>` wrapper forwards both verbatim — no interception possible.**
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:27241-27246`
   (`impl<E: ArtifactEditor> ArtifactApp for EditorApp<E>`):
   ```rust
   fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> { E::build_document_store_owners() }
   fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> { E::build_config_store_owners() }
   ```
   `E = Puzzle3dPlayApp`, so this resolves to step 1's `Some(...)`.

3. **There is exactly one production/dev bootstrap path for every editor app, puzzle3d included.**
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:27905-27912` (`editor_surface<E, PA>`, the
   function `PluginBuilder::editor::<E>` uses to build a `SurfaceDeclaration`):
   ```rust
   fn factory<E: ArtifactEditor, PA: PluginApp + From<VcsArtifactApp<EditorApp<E>>>>(def: &AppDefinition) -> PA {
       PA::from(resolve_ready(VcsArtifactApp::with_registry_on_bus(EditorApp::<E>::default(), AppActionRegistry::from_definition(def), semio_framework::ActionBus::production())))
   }
   ```
   There is no separate wasm-only, dev-only, or "reopen" path for a top-level plugin instance — every
   editor app (puzzle3d, lowpoly, writer, cad, …) is instantiated through this one generic factory.
   (I confirmed this is the only non-test caller of `with_registry_on_bus`/`with_registry` besides the
   `testkit::new_app*` helpers, which delegate to the identical `with_registry`/`with_registry_on_bus`
   — so the test harness and production take the same code path here.)

4. **`with_registry_on_bus` installs the owners unconditionally whenever the hook returns `Some`.**
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19478-19495`:
   ```rust
   let mut store = ArtifactStore::new(envelope).await.expect(...);      // snapshot_retirement_factory: None (fresh)
   if let Some(owners) = A::build_document_store_owners() {
       store.install_member_store_owners_exact(owners);
   }
   let mut config_store = ConfigStore::new(config_envelope.await).await.expect(...);
   if let Some(owners) = A::build_config_store_owners() {
       config_store.install_member_store_owners_exact(owners);
   }
   ...
   interaction_store.install_member_store_owners_exact(crate::local_interaction::retirement::interaction_store_owners()); // unconditional, every app
   ```
   `ArtifactStore::new` (store/🦀️.rs:13952, the function the prior notes called `from_new`) does leave
   `snapshot_retirement_factory: ManuallyDrop::new(None)` at construction (store/🦀️.rs:14005) — the
   prior note's premise is correct as far as it goes — but for puzzle3d step 1's `Some` means the `if
   let` on the very next line always fires and overwrites it before the store is ever handed back to
   the app. The interaction store's install is not gated on any app override at all.

5. **The install writes into the exact field the fault site reads.**
   `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:14364-14372`:
   ```rust
   pub fn install_member_store_owners_exact(&mut self, owners: MemberStoreOwners<P, Mutation>) {
       assert!(self.snapshot_retirement_factory.is_none() && ... , "a freshly constructed member store must not carry preinstalled or terminal owner authority");
       *self.snapshot_retirement_factory = Some(owners.snapshot_retirement);
       *self.initial_snapshot_retirement_factory = Some(owners.initial_snapshot_retirement);
       *self.mutation_retirement_factory = Some(owners.mutation_retirement);
       *self.owned_disposer = Some(owners.store_disposer);
       ...
   }
   ```
   `owners.snapshot_retirement` is the `MemberStoreOwners.snapshot_retirement: Arc<dyn
   SnapshotRetirementFactory<P>>` field (store/🦀️.rs:2009-2018) — the same type/field the fault site
   reads. It is **not** a null-object: `bounded_document_store_owners`/`bounded_config_store_owners`
   (plugin/🦀️.rs:13558-13584) construct it as `Arc::new(BoundedConfigRetirementFactory::<P>::new())`, a
   real, working `SnapshotRetirementFactory` impl (plugin/🦀️.rs:13550-13554), passed positionally into
   `MemberStoreOwners::new` (store/🦀️.rs:2026-2032) as its first argument, which maps 1:1 onto the
   `snapshot_retirement` field.

6. **The fault site itself, unchanged from the original trace, confirmed at current line numbers.**
   `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:14746-14750`
   (`ArtifactStore::take_returned_snapshot_read_retirement`):
   ```rust
   if !self.snapshot_read_leases.has_returned() { return Ok(None); }
   let Some(factory) = (&*self.snapshot_retirement_factory).clone() else {
       return Err(VcsError::ValidationFailed("snapshot read retirement factory is not installed".into()));
   };
   ```
   Given steps 1-5, `self.snapshot_retirement_factory` is `Some` for puzzle3d's document store from the
   moment the store exists — the `else` branch cannot be taken regardless of whether a snapshot read
   lease was ever issued or returned.

## Where the generic "runtime live cleanup faulted" message actually comes from

The exact string is not at the store fault site — it is a generic wrapper one layer up:
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:30322`:
```rust
RUNTIME_MAINTENANCE_FAULT => Err(plugin_internal_fault(format!("runtime live cleanup faulted for instance {}", cell.id))),
```
This status is set whenever `EditorApp::maintenance_step` (called from `RuntimeLiveCleanupJob::step`,
plugin/🦀️.rs:29565-29614, itself driven every actor turn by `run_runtime_live_cleanup_turn`,
plugin/🦀️.rs:29656) returns any `Err`, or times out, or produces zero-progress for too many turns
(`RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT`, `runtime_live_cleanup_nonterminal_status`,
plugin/🦀️.rs:29634-29650). It is a generic maintenance-turn failure surface, not proof of this specific
store fault — the original investigation's job of tracing it down to the store call was the right move,
and that trace target (`ArtifactStore::take_returned_snapshot_read_retirement`) is what step 6 above
shows is now unreachable for puzzle3d.

`EditorApp::maintenance_step`'s exact production body is at plugin/🦀️.rs:24254-24280; its fast-path
branch (all other queues idle) is the one that calls
`store.take_returned_snapshot_read_retirement()`:
```rust
fn maintenance_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
    ...
    if <every other queue is idle> {
        let pump = &mut self.document_snapshot_read_returns;
        let store = &mut self.store;
        return pump.drive(|| store.take_returned_snapshot_read_retirement().map_err(|error| error.into_fault()), maximum_items, maximum_bytes);
    }
    ...
}
```

## Item 3 — `snapshot_retirement` vs `snapshot_retirement_factory`: confirmed the same authority

Yes, they are the same field under two names at two layers, not merely "related" as the prior
investigation guessed:
- `MemberStoreOwners<P, Mutation>.snapshot_retirement: Arc<dyn SnapshotRetirementFactory<P>>` (the
  value `build_document_store_owners()`/`bounded_document_store_owners()` produce), store/🦀️.rs:2014.
- `ArtifactStore<P, Mutation>.snapshot_retirement_factory: ManuallyDrop<Option<Arc<dyn
  SnapshotRetirementFactory<P>>>>` (the value the fault site reads), store/🦀️.rs:13566.
- `install_member_store_owners_exact` is the sole bridge: `*self.snapshot_retirement_factory =
  Some(owners.snapshot_retirement)` (store/🦀️.rs:14369).

## Item 4 — the regression test at `✏️editor/🦀️.rs:~7735`

`local_interaction_query_return_does_not_fault_the_next_maintenance_step`
(`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:7735-7768`):

- Builds a registry-backed `Puzzle3dApp` via `app_with_registry()` — the **same**
  `with_registry`/`with_registry_on_bus` production genesis path traced in steps 3-4 above (the test
  harness is not a shortcut around owner installation).
- Binds instance id 1, then drives the vortex-picking **local interaction query** to completion:
  `Started` → terminal `Page` → `acknowledge_local_interaction_query` → `Closed`. The test's own
  docstring states this is "the only path by which the puzzle3d document store's `snapshot_read()`
  lease is ever taken and returned" — i.e. it is the one thing that makes
  `self.snapshot_read_leases.has_returned()` true and would let the fault site's `if` guard fall through
  to the factory check at all.
- Then calls `app.maintenance_step(1, 4096).expect("maintenance step after a returned snapshot read
  lease must not fault")` — this is exactly `EditorApp::maintenance_step` (plugin/🦀️.rs:24254), which on
  its idle fast path calls `store.take_returned_snapshot_read_retirement()` (the exact fault site).

**This test would catch the fault if it recurred.** It exercises the real lease-issue-and-return path
(not a synthetic one), constructs the app through the real production bootstrap, and asserts on the
exact call the bug report named. Given the source proof in steps 1-6, this test should currently pass —
but per task scope I did not run it; that is the one empirical confirmation still open (see below).

## Item 5 — comparison with lowpoly and writer

This produced a real asymmetry, though not one that threatens the verdict:

- **`✒️writer`** overrides `build_document_store_owners()` the same way puzzle3d does — returning
  `Some(crate::artifacts::writer::spr::writer_document_store_owners())`
  (`✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1190-1192`,
  its own custom owners rather than the generic `bounded_*` helper, but the same mechanism/field).
  Writer does **not** override `build_config_store_owners()` (not found by name in its editor file).
- **`💠️lowpoly`** overrides **neither** hook
  (`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` has zero
  matches for `build_document_store_owners`/`build_config_store_owners`), so it falls to the
  `ArtifactEditor` trait's default, which is `None` for both
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26663-26669`).

  That means lowpoly's document and config stores structurally **never** get a
  `snapshot_retirement_factory` installed — by the same chain as steps 1-6, lowpoly would hit the exact
  `Err(ValidationFailed(...))` branch *if* it ever issued and returned a document-store snapshot read.
  That lowpoly is reported "100% migrated" and presumably usable in practice is evidence that lowpoly's
  actor turns never exercise a returned document-store snapshot-read lease (its own local-interaction
  query path, if it has one, may not touch the document store the way puzzle3d's vortex-picking query
  does) — not evidence that a missing factory is safe in general. So the comparison actually runs the
  other way from how item 5 was framed: **puzzle3d does something writer also does and lowpoly does
  not** — it explicitly installs document/config store owners, which is what closes this exact gap.
  Nothing about lowpoly's construction is a technique puzzle3d is missing.

## What would still be worth running (not done here — out of scope)

Source proof is complete for the document-store path, but two things are worth a real (not `cargo
check`, an actual `cargo test`) run once the build backlog clears, to convert this from "structurally
proven" to "empirically confirmed":
1. `cargo test -p semio-s-plugin-puzzle
   local_interaction_query_return_does_not_fault_the_next_maintenance_step` — should pass given the
   above; if it doesn't, the discrepancy is almost certainly in a path this analysis didn't reach (e.g.
   config-store or interaction-store retirement pumps invoked from a different stage of the 21-stage
   `maintenance_step` match this note did not fully enumerate), not in the document-store chain traced
   above.
2. A real dev-server actor turn against a freshly built wasm (once the stdio `✳️base`→`🧱️base` rename
   blocking `buildEngineWasm` converges, per `📓️findings-2026-09-05.md` item 1) to rule out any residual
   gap between source and the currently-loaded prebuilt wasm.
