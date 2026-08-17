# Final Summary — Artifact Viewers and Editors per Subset (#2559)

Goal `🎯r2602🎯runningsketchpad🎯runningsketchpadapps`. Start commit `63686457bd` (2026-08-16 02:50:31 +0200).

## What changed

The general app system is gone. A plugin no longer ships `🎛️apps/<app>/…`; every **artifact subset** now owns two surfaces:

```
🗿️artifacts/<kind>/🏅️standards/🔖️<v>/🪆️subsets/<subset>/
  🧬️schema/  🚪️io/  📚️examples/          (unchanged)
  👁️viewer/  🎭️modes/<mode>/🪟️windows/<window>/{🦀️component.rs, 🟦️component.ts, …}
  ✏️editor/  🎭️modes/<mode>/🪟️windows/<window>/{🦀️component.rs, 🟦️component.ts, …}
```

**Measured on disk at close** (enumerated with `find`, never a grep count):

| | |
|---|---:|
| `🎛️apps` directories | **0** (was 33) |
| `👁️viewer` surfaces | **143** |
| `✏️editor` surfaces | **143** |
| files under the new surfaces | **7 652** |
| `SCAFFOLD` residue | **0** |

## Mechanisms landed

**C1 identity** — `AppRole {Viewer, Editor}`, `AppRef`, required `AppDefinition.{role, dialect}`, and the frozen canonical id grammar `<kind>@<standard>/<subset>#<role>` with `surface_app_id`/`parse_surface_app_id`. `AppBuilder::build_definition` rejects a hand-written id.

**C2 SDK** — `ArtifactApp` split into `ArtifactEditor` and a new `ArtifactViewer` whose `handle` returns `ViewEmit`. **The read-only guarantee is a type property, not a check**: `ViewEmit` has no field, constructor or method that can carry an artifact or draft mutation, and `ViewerApp`'s adapter builds its `Emit` with `..Default::default()`, so `artifact_mutations`/`draft_mutations` are empty *by construction*. Audit #2 attacked this specifically and found no escape (no `From`/`Into`/`Deref`/`pub(crate)` path). On top of that sits the runtime guard (C2.3): a viewer instance rejects undo/redo/checkpoint/alternative/revert/cut/paste/import with `viewer.read-only`, treats any non-empty `artifact_mutations` as a hard SDK fault, renders history read-only, and attaches its store `Rights::Read` only. Seven frozen window kits (`Text/Table/Tree/Image/Mesh/Document/Media`) carry the thin surfaces.

**C3 hosts** — `AppRouter` (owner-first deterministic ordering, `surface.conflict`, contribution gate) and `OpeningResolver` (four-step precedence) in **both** the Rust wasmtime host and the TS browser host, reconciled line-by-line against each other with a shared parity fixture. Five OS commands (`os.open-artifact`, `os.open-artifact-with`, `os.set-default-viewer`, `os.set-default-editor`, `os.clear-default-app`) on channel tags 27/28/29 with golden vectors decoded identically from Rust and vitest.

**C4 preferences** — `OpeningPreferences` in the OS `🎚️config` lane with `set-default-app` / `clear-default-app` mutation triads. The resolver reads a **fold over the config op log**, never a mutable map — verified event-sourced on both sides, no CRUD, no CRDT.

**C5 UI** — role-aware sessions `(artifact_ref, AppRef)` in both shells; read-only chrome (role chip, hidden `Mutation`-kind actions, disabled undo/redo); "Open with…" in context menu, palette and Document panel; a `SettingsDefaultApps` tab writing only through the OS commands; boot role from `SEMIO_APP_ROLE`/`VITE_SEMIO_APP_ROLE`. Every string ships en+de, English first.

**C6 taxonomy** — `schemaVersion` 4 → 5. Surface vocabulary added additively in W0; `appsDirName`/`appChildDirs`/`appComponentDirs`/`appSchemaSpecFilenames` deleted and `pluginChildDirs` → `["🎮️commands"]` only in W3, after the last `🎛️apps` was gone — the APA ordering lesson, since the Rust gate panics if a listed facet dir is missing. Four new policies (`policySubsetSurfaceCompletenessBreaches`, `policyViewerPurityBreaches`, `policyContributedSurfaceTargetBreaches`, `policyOsConfigShapeBreaches`) plus a permanent taxonomy-derived `new surface` scaffolder.

## Gates at close

| gate | result |
|---|---|
| `cargo test -p semio-framework-plugin --lib` | **214 pass / 6 fail** (from 160/59) |
| `cargo check -p semio-framework-plugin --all-targets --keep-going` | **0 errors** |
| `bun ./📜️script.ts check` (catalog + launch.json fresh) | **PASS** |
| `bun nx run @semio-tech/repo-lib:test` | 170 pass / 18 fail — **identical to baseline, 0 new** |
| four surface policies, live filesystem | **0 breaches** |
| structural enumeration | **PASS** (table above) |
| `bun ./📜️script.ts verify gate` | **BLOCKED** — pre-existing dependency-cruiser failure, predates this ticket |
| `cargo check --workspace --all-targets --keep-going` | **FAIL, 753 errors** — all attributed per-crate to two live peer tickets |
| browser end-to-end | **BLOCKED** — see below |

## What is NOT done, and why

1. **The browser end-to-end was never run.** It was attempted for real, not skipped: `semio-s-plugin-stdio` does not compile to wasm (164 errors, `absorb`/`apply` signature mismatch from the live `26/08/16/FULL-STDIO-…` ticket), and the cad artifact's schema is glued into that crate, so the example cannot load. Evidence in `🧪️w4-final-job3-blocked.txt`. **The read-only behaviour is therefore proven by type structure and unit test (`viewer_rejects_every_contract_mutating_verb`, all 8 verbs), not by a live browser session.** That distinction is deliberate and should not be read as "verified in the app".
2. **`cargo check --workspace` fails with 753 errors**, none in this ticket's files — attributed crate-by-crate with `git status --porcelain` / `git log --date=iso` to the live FULL-STDIO and MUTATION-OUTCOMES peer sessions. A clean workspace measurement needs a quiet tree.
3. **6 residual SDK test failures**, re-confirmed as genuinely inherited (artifact-identity grammar, VCS edit-reference validation, history-conflict validation, one flaky global-registry isolation test) — not this ticket's fallout.
4. **18 pre-existing `repo-lib` failures** unchanged; several pin vocabulary a live peer may be mid-rename on, so "fixing" them could revert in-flight work.
5. **space's `studio` app** is not a surface. Evidence-based decision, recorded in `📓️w2-end-report.md`: its document is the framework-owned `WorkflowSnapshot` (`os.workflow`), so space cannot honestly claim ownership of an artifact for it, and the two document types are structurally incompatible with `🏠️home`, so nothing could fold in. It was relocated out of `🎛️apps` into a plugin-root `⚙️engine/` facet — the precedent fem set — with zero functional change.
6. **`assembly`'s surfaces are authored but not mounted**: its schema tree lacks the artifact-facet descriptor and non-Rust leaves, so its snapshot/mutation types do not yet satisfy the SDK's trait bounds. Pre-existing gap, flagged for a follow-up ticket.
7. **`EngineCanvas` (framework renderer) depends on a plugin's internals** (`puzzle::…::BoardHost`). Repointed so it compiles, but the layering inversion is real and worth its own ticket.

## Corrections made to the record

- **59 SDK test failures were mischaracterised** by several lane reports as "pre-existing debt". They were this ticket's own fallout: contract §1's canonical-id assertion rejecting stale fixtures like `"synthetic-play"`. 53 were fixed by building ids through `surface_app_id(&dialect, role)` rather than string literals; the remaining 6 are genuinely inherited.
- **`view_action_emitting_ops_is_rejected` exposed a real production bug**, previously masked by the id panic: the kind-discipline guard resolved verbs only via `get_command`, missing plain declared actions. Fixed.
- **`semanticCollections` was NOT extended** with the surface dirs — doing so would have created 286 phantom collection roots, because a collection root requires a manifest in exact bijection with its children, and a surface's children are taxonomy vocabulary. A surface is a subset *facet*, like `🧬️schema`.
- **`AppDefinition.dialect` is `ArtifactDialect`**, not the `&'static str`-based `Dialect` the plan named — the latter cannot deserialize.
- **The `SemanticMutation` bound on `PluginBuilder::editor`/`::viewer` was wrong**: it was never required by the surface traits and silently blocked 32 stdio subsets from registering at all. Split into opt-in `editor_mutation_roster` / `viewer_mutation_roster`; stdio's registrations went 112 → 176.

## Findings worth keeping

- **A cached policy run lies.** `bun ./📜️script.ts policy` reads a snapshot; calling the policy functions against the live filesystem found 9 missing window TS twins after the cached run reported 0 breaches. Enumerate live.
- **Directory structure is not module path.** `#[path]` lets a module be declared in one place and sourced from another. Every packet that derived module paths by parsing its own `📦️glue.rs` succeeded; the trap that bit five packets was a mistyped emoji directory (`🏅️标准` vs `🏅️standards`) that `ls` catches instantly and inference never does.
- **A guard with no test is a claim, not a property.** The role guard sat parked and unnoticed through three waves precisely because nothing failed without it — the type-level closure kept every real viewer honest, so the missing runtime clauses were invisible until audited for directly.

---

## Reopened pass — closing the deferred gaps

The ticket was reopened after the first close to finish what had been recorded as blocked.

**The stdio compiler gate ran.** The peer ticket `26/08/16/FULL-STDIO-…` had migrated stdio's mutation implementations to the new `MutationOutcome`/`MutationApplyResult` API and explicitly *deferred the compiler gate*, freezing its source pending "the parent's serialized compiler/test gate". That gate was exactly what blocked this ticket's end-to-end, so it was run: four shards took `semio-s-plugin-stdio` from **598 → 0 errors**, and `cargo test -p semio-s-plugin-stdio --no-run` links. Every fix was either a test call site adapting to the new `Result`-returning `apply`, or a missing `use` — typed rejection propagation, preflight validation and atomicity rules were left untouched, per that ticket's contract. `📦️glue.rs` needed no edits.

**Three further breaks were fixed to get the cad playground running**, each found by actually trying to boot it rather than by inspection:
1. `IoWireError` no longer converts to `String` (a peer changed the type) — two wasm-guest call sites in `🔌️plugin/🦀️component.rs` were stale, plus `set_io_fallback_dispatcher` had become a struct-taking, `Result`-returning API while the guest still passed a bare closure. Fixed, and `IoFallback`/`IoFallbackDispatcher` added to the framework crate-root re-exports.
2. `ShellHost` read `app.dialect` unguarded in two places. A non-surface app — space's `studio`, deliberately left without a dialect — made `dialectCoordinate(undefined)` throw on boot. Both sites now skip apps with no dialect, which is the correct semantic: an app bound to no subset has nothing to "open with".
3. cad's playground metadata still named the retired app id `cad-play`; updated to the derived `s.cad.cad@1/*#editor` and the session regenerated.

**A real defect in cad's own declarations was found by the same boot attempt** and is worth recording, because it was invisible to every static gate: cad's `plugin()` assembly was failing preflight, so its WASM manifest shipped `pluginId: "assembly-failed"` with **zero apps** — the shell simply reported "appId does not resolve". Two independent bugs in `🗿️artifacts/📐️cad/🦀️component.rs`: no declared capability for its own native composer (`s.cad@1/*` — only the eight stdio bridges were declared), and a codec row claiming `"cad.document"` instead of the real `DOCUMENT_SCHEMA` `"cad.scene"`. Both fixed on cad's side; the peer-owned capability mechanism was not weakened. cad's manifest now carries both `s.cad.cad@1/*#editor` and `s.cad.cad@1/*#viewer`.

**The same missing-native-composer bug exists in six more plugins** — `mathematical`, `note`, `forms`, `layout`, `imperative`, `sourcing`/`curate` — each of which will ship an `assembly-failed` manifest until declared. Found, not fixed; this is the highest-value follow-up in the list.

**Method note worth keeping:** the assembly failure proves a class of defect no policy in this ticket could catch. Every taxonomy gate was green, every crate compiled, every unit test passed — and the plugin still shipped an empty manifest, because the failure was a *runtime preflight* that degrades to a stub instead of failing loudly. Booting the thing found it in minutes. A static gate cannot substitute for running the artifact.
