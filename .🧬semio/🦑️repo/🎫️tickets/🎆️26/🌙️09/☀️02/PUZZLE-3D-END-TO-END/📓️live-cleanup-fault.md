# 🩺️ Runtime live cleanup fault — verified chain, patch, and compile status

## Verdict

The originally-traced hypothesis does **not** hold for puzzle3d: `Puzzle3dPlayApp` (the
`ArtifactEditor` impl at `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:6389`)
**does** override `build_document_store_owners()` and returns `Some(bounded_document_store_owners(...))`.
I traced the full construction chain with line numbers below; the `snapshot_retirement_factory` on
every store `begin_local_interaction_query` ever takes a lease from (document, config, interaction)
is installed at instance-open time. I could not get a clean compile/test run to empirically confirm
the fault is gone (see "Compile status" — blocked by an unrelated, currently in-flight edit elsewhere
in the repo), so this is a verified-by-reading, not verified-by-running, report. Per the dev's own
instructions I am not forcing a fix onto the originally-named link since it is not broken.

## The verified chain (all via static reading, with line numbers)

1. **Production construction path** — the only place any editor surface (including puzzle3d's) is
   ever built: `editor_surface`'s inner `factory` fn,
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:27134`:
   ```rust
   PA::from(resolve_ready(VcsArtifactApp::with_registry_on_bus(EditorApp::<E>::default(), AppActionRegistry::from_definition(def), semio_framework::ActionBus::production())))
   ```
   `E = Puzzle3dPlayApp` for puzzle3d (confirmed by `✏️s/🔌️plugins/🧩️puzzle/🦀️.rs:14`:
   `Puzzle3dEditor(VcsArtifactApp<EditorApp<Puzzle3dPlayApp>>)`). There is no second/alternate
   construction path (no reopen-specific constructor — reopening loads pack/text data onto an
   already-constructed instance via `load_document_pack`/`hydrate_document_lane`, not a new store).

2. **`VcsArtifactApp::with_registry_on_bus`** (`🔌️plugin/🦀️.rs:19096-19119`) builds all three stores
   `begin_local_interaction_query` later draws snapshot leases from, and installs owners on each:
   ```rust
   let mut store = ArtifactStore::new(envelope).await.expect(...);
   if let Some(owners) = A::build_document_store_owners() { store.install_member_store_owners_exact(owners); }
   let mut config_store = ConfigStore::new(config_envelope.await).await.expect(...);
   if let Some(owners) = A::build_config_store_owners() { config_store.install_member_store_owners_exact(owners); }
   ...
   let mut interaction_store = ConfigStore::new(interaction_envelope).await.expect(...);
   interaction_store.install_member_store_owners_exact(crate::local_interaction::retirement::interaction_store_owners());
   ```
   `A = EditorApp<Puzzle3dPlayApp>` here.

3. **`EditorApp<E>::build_document_store_owners`/`build_config_store_owners`** (`🔌️plugin/🦀️.rs:26501-26507`)
   forward verbatim to `E::build_document_store_owners()`/`E::build_config_store_owners()` — trivial
   one-line delegation, no gap:
   ```rust
   fn build_document_store_owners() -> Option<...> { E::build_document_store_owners() }
   fn build_config_store_owners() -> Option<...> { E::build_config_store_owners() }
   ```

4. **`Puzzle3dPlayApp`'s own overrides** (`✏️editor/🦀️.rs:6389-6395`):
   ```rust
   fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
       Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
   }
   fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> {
       Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
   }
   ```
   `bounded_document_store_owners`/`bounded_config_store_owners` (`🔌️plugin/🦀️.rs:13416-13424`) build a
   real `MemberStoreOwners` with a real `SnapshotRetirementFactory` (`BoundedConfigRetirementFactory`),
   not a stub.

5. **`interaction_store_owners()`** (`🔌️plugin/🕹️interaction/♻️retirement/🦀️.rs:111`) is
   framework-owned and installed unconditionally for every app — always present, no app hook at all.

6. **`ArtifactStore::install_member_store_owners_exact`** (`🏪️store/🦀️.rs`, ~14154-14167) sets
   `*self.snapshot_retirement_factory = Some(owners.snapshot_retirement)` — the exact field
   `take_returned_snapshot_read_retirement` (`🏪️store/🦀️.rs:14534`) reads.

So for puzzle3d's document store, config store, and interaction store alike, `snapshot_retirement_factory`
is `Some` immediately after construction, before any turn runs. The `None`-by-default path
(`ArtifactStore::new`, `🏪️store/🦀️.rs:13803`) is real but is always overwritten by step 2's
`install_member_store_owners_exact` call for all three of puzzle3d's stores.

## Where the lease is actually taken/returned in practice

The only place puzzle3d's document store's `snapshot_read()` is ever called is
`begin_local_interaction_query` (`🔌️plugin/🦀️.rs:24039-24058`, the vortex-picking/hover query) — the
`pending_effects()` snapshot_read path (`🔌️plugin/🦀️.rs:24687`) is gated behind
`mounted_job_prepare_snapshot_read`, which `Puzzle3dPlayApp` never overrides (stays `false`). This
matches "faults on every actor turn": the host drives this local-interaction query every turn for
world-space picking.

## The second finding: `build_artifact_store_one_item_preparation_factory`

Confirmed real and confirmed **not** the same install site. `Puzzle3dPlayApp` implements
`build_config_store_one_item_preparation_factory` (`✏️editor/🦀️.rs:6397`, returns
`Some(Puzzle3dConfigStorePreparationFactory)`) but never overrides
`build_artifact_store_one_item_preparation_factory` — it stays the `ArtifactEditor` trait default
`None` (`🔌️plugin/🦀️.rs:11048-11050`, "an explicit fail-closed publication denial").

This is architecturally a *different* Option, populated at a *different* call site
(`🔌️plugin/🦀️.rs:19133`, `let artifact_one_item_factory = A::build_artifact_store_one_item_preparation_factory();`)
into a *different* struct field (`VcsArtifactApp.artifact_one_item_factory`, not
`ArtifactStore.snapshot_retirement_factory`). It is consumed only by the async worker-job "Emit →
`begin_apply_one`" typed-command publication path (`🔌️plugin/🦀️.rs:22291`), gated at construction
time by whether any registered `ArtifactOwnedToolJobFactory::PUBLICATION_CONTRACTS` declares
`ArtifactToolPublicationLane::Artifact` (`🔌️plugin/🦀️.rs:19143`).

Puzzle3d registers exactly one tool job factory, `Puzzle3dRetainedCommandJobFactory`
(`✏️editor/🦀️.rs:6146-6156`), whose `PUBLICATION_CONTRACTS` only ever declare `HostOnly` and `Config`
lanes — never `Artifact`. So the missing factory is currently **inert**: it neither construction-faults
nor runtime-faults today, and does not explain `runtime live cleanup faulted`. It is real gap for the
broader migration effort this ticket already tracks in `📓️status.md`'s "Likely shared root cause"
section / `📓️interactive-job-migration-recipe.md` (the 18 "Artifact+Config" retained routes, e.g.
`setActiveExample`) — but it shares no code path with `take_returned_snapshot_read_retirement`. My
reading suggests `📓️status.md`'s "these are plausibly the same gap... being verified" should be closed
out as **not the same gap** for the document-store owners half; I did not chase whether the
retained-preparation-factory absence has any bearing on the Config lane specifically.

## The test (added, not yet verified to compile/pass)

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, in `mod tests`
(after line 7386): `local_interaction_query_return_does_not_fault_the_next_maintenance_step`. It uses
`testkit::app_with_registry()` (the `bounded_first_step_tool_proofs!`-safe harness, per the dev's own
note — bare `testkit::app()`/`new_app` faults with a catalog-authority error for this plugin), binds
instance id 1, drives a real `begin_local_interaction_query` → `Started` → terminal `Page` →
`acknowledge_local_interaction_query` → `Closed` cycle through the public `PluginApp` trait (the exact
sequence the host runs every actor turn for vortex picking), then calls a plain
`app.maintenance_step(1, 4096)` and asserts it does not fault.

## Compile status — genuinely blocked, not a silent skip

`RUSTC_WRAPPER="" cargo check -p semio-s-plugin-puzzle --tests` (run twice, once plain and once with
`--keep-going`, both in the foreground with an isolated `CARGO_TARGET_DIR` under this session's
scratchpad to sidestep the shared `target/debug/.cargo-lock`, which was held for 40+ minutes by another
session's `cargo rustc -p semio-s-plugin-stdio`) never reaches checking `semio-s-plugin-puzzle`'s own
lib/tests. Both runs stop on:

```
error[E0283]: type annotations needed
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:1701:24
        obj.insert("x".into(), Value::from(nx));
error: could not compile `semio-framework-os-infinite` (lib) due to 5 previous errors
```

`semio-framework-os-infinite` is a transitive dependency of `semio-framework-plugin`/`semio-s-plugin-puzzle`.
This is **not** my change — I never touched that module. Verified live/in-flight, not stale:

```
$ git status --porcelain -- 🧰️framework/.../♾️infinite
 M .../🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs
 M .../🕸️dag/🧬️schema/🧬️mutations/↔️move-node/🦀️.rs
 ... (15 sibling mutation files, all modified, uncommitted)
$ stat -f "%Sm" .../🕸️dag/🦀️.rs
Sep  3 00:46:23 2026   # minutes before this check ran
```

Another live session is mid-edit across ~16 files in `os-infinite`'s board/dag module right now
(uncommitted, very recent mtime). The ambiguity (`"x".into()` matching multiple new
`impl From<&'static str> for <ErrorType>` across `protocol`/`naga`/`zune_*`, including
`protocol::FaultCode`) looks like the same class of half-applied-migration fallout `📓️status.md`
already found and fixed inside `semio-s-plugin-puzzle` itself (201→0 errors) — just not yet swept in
`os-infinite`. Per the dev's instruction I did not touch it and did not chase further; it should
resolve once that session's edit lands, or needs its own sweep.

## What is and isn't verified

- **Verified by reading, with line numbers**: the document/config/interaction store owner-install
  chain is intact for puzzle3d; the originally-named link is not broken.
- **Not verified by running**: whether my added test actually compiles and passes — blocked by the
  unrelated `os-infinite` breakage above, not by anything in my change.
- **Not investigated further**: whether some other, not-yet-identified path explains the reported
  runtime fault, if it is still reproducing after this repo's recent `📓️status.md`-documented fixes.
  Given the fault report predates today's `bounded_document_store_owners`/`bounded_config_store_owners`
  wiring I found already in place, it is also possible the fault report is stale — worth a fresh
  runtime repro once `os-infinite` compiles again.

## Files touched

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` — added
  `local_interaction_query_return_does_not_fault_the_next_maintenance_step` to `mod tests`. No other
  production code changed (none was needed for the traced chain).
