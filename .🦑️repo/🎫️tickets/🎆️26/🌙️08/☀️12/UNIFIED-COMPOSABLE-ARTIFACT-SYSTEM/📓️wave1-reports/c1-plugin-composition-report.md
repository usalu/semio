# W1-C1 — Plugin Composition Runtime

**Status**: code complete; final `--all-targets` verification was interrupted by machine saturation (see Verification). Written by the orchestrator from on-disk evidence after the authoring agent was terminated by a session limit and then stalled in an idle-wait loop on contended builds.

## What landed

Crate `semio-framework-plugin`, file `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`.

| Task | Item | Evidence |
|---|---|---|
| 1 | `ChildEmit` struct + typed constructor | `:3991` (struct), `:3999` (impl) |
| 1 | `Emit.child_emits` | 17 references in-file |
| 2 | `VcsArtifactApp` child-store map + `ChildStoreFactory` wiring | present |
| 3 | `dispatch_emit` group routing via `CompositionCoordinator::dispatch_group` | 18 references |
| 4 | Group undo/redo through `undo_group`/`redo_group` | 5 references |
| 5 | `ArtifactChildren` trait | 1 declaration |
| 5 | `DerivedArtifactSpec::Children` + `derive_artifact_facets!` children arm | 7 `type Children` references |
| 6 | WIT host import `resolve-artifact-link` | `🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️world.wit` |

## Send bound

The child-store map made `VcsArtifactApp` non-`Send`: `HashMap<(String,String), (ArtifactDialect, Box<dyn SpaceMember>)>` became a field of a type that `impl<A: ArtifactApp> PluginApp for VcsArtifactApp<A>` requires to be `Send`. `dyn SpaceMember` had no `Send` supertrait, producing **57 × `E0277`** and breaking the lib-test build of every plugin crate in the workspace — five concurrent sessions blocked.

**Chosen: `pub trait SpaceMember: Send`** (`🏪️store/🦀️component.rs:3902`), a sanctioned cross-boundary edit into another wave's file, authorized by the orchestrator and documented at the declaration site.

Rejected alternative: widening to `Box<dyn SpaceMember + Send>` at the map and `ChildStoreFactory::create`/`open` return types. That would have been narrower but required the same cross-crate edit anyway (the factory signatures live in store), while leaving the trait's thread-confinement implicit. A store member that is in practice thread-confined should say so in its bounds.

**Caveat for a later reviewer**: the browser/WASM path uses `spawn_local` with non-`Send` futures. No implementor was found that is legitimately non-`Send`, but if one is added later it will now fail at the trait rather than at the use site — which is the intended, earlier failure.

## Orchestrator repairs folded in

Four errors fixed directly while the authoring agent was down. All were **pre-existing debt newly exposed**, not regressions from this wave — they only surface under `--tests`/`--all-targets`, which had been unbuildable for other reasons:

- `:3152` `TutorialBase { document_dsl: None, … }` → `artifact_dsl`
- `:3439` `assert_eq!(definition.document_json, …)` → `definition.artifact_json`
  Both are fields renamed by the **closed** ticket `26/08/10/RENAME-DOCUMENT-TO-ARTIFACT-THROUGHOUT-CODEBASE` (`🛂️manifest/🦀️component.rs:1436`, `:2682`). Trap avoided: `source.document_json()` one line above `:3439` is a *method* that still exists — only the struct field moved.
- `:10326`, `:10368` `IoPayload::Text("")` → `IoPayload::Text(String::new())` inside the `subset!` macro.

## Verification

- `cargo check -p semio-framework-plugin --all-targets` → **clean** (verified by the orchestrator immediately after the four repairs above, before tasks 5–6 landed).
- `cargo check -p semio-framework-os-kernel` → clean, 49 warnings (baseline).
- **A post-task-5/6 re-verification is OUTSTANDING.** The machine reached ~20 concurrent `rustc` processes from five sessions with the volume at 98% (18 GiB free of 926 GiB; the default `./target` alone is 428 G). Builds began exceeding 10 minutes and failing with disk-pressure artifacts — corrupt dep-info (`could not parse/generate dep info`) and spurious third-party failures (`futures-executor`, `icu_*`, `schemars`, `libc` build script). These are environmental, not code. **Re-run `--all-targets` once the machine is quiet before treating W1 as signed off.**

## sharedFileRequests

- `🏪️store/🦀️component.rs:3902` — `SpaceMember: Send` supertrait (applied, authorized).
- From B2, still open: `GroupMeta.actor`/`coalesce_key` are accepted but unwired; honouring them needs an object-safe `set_local_actor_id`/`AmendLast` seam on `SpaceMember`.
- From B2, still open: `ArtifactEnvelope.dialect` remains `Option<…>`; making it required measured at 106 files / 168 call sites and was correctly deferred.

## Concurrent-churn observations

- The `📌️panels/📄️document` → `📄️artifact` glue-mount break (9 plugins) was orphaned debt from the same closed rename ticket; repointed by the orchestrator.
- `semio-framework-os-kernel` had no `[dev-dependencies]` section at all while `🏪️store/🔄️sync` used `tempfile::tempdir()` at three sites; a target-gated dev-dep was added, unblocking the triad law harness for four sessions.
- Both classes of break were invisible to `cargo check` because they live in `#[cfg(test)]` code. **`--all-targets` is the only trustworthy gate in this repo.**
