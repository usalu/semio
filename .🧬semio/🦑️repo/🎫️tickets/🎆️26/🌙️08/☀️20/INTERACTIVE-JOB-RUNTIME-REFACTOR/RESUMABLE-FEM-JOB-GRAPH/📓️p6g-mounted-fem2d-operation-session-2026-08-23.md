# P6g Mounted Fem2d Operation Session — 2026-08-23

## Verdict

**SOURCE-AUDIT-READY, NOT ACCEPTED.** The live 2D FEM editor now mounts a retained,
generation-tagged operation session on the existing reactor job path and consumes its live visual.
The source verifier accepts the P6g contract. P6h numerical micro-cursors, P6i visual encoding,
runtime/build validation, and the full interactive-tool migration remain red.

## Live production route

1. `VcsArtifactApp` binds app instance, base revision, store generation, canonical base bytes, and an
   exact registered snapshot read into `ArtifactView`.
2. The FEM2D editor's production `pending_effects` reconciles that authority with the fixed session
   registry. Revision or generation replacement emits `CancelJob` before a new `SpawnJob`.
3. The reactor resolves the registered `BoundedJobFactory` and gives the session exactly one bounded
   job opportunity per `step-job` call. Rejected input ownership remains observable.
4. The retained session advances input preflight, mesh, assembly, PCG, validation, publication, or
   close as explicit states. It revalidates base revision, generation, canonical base, and
   cancellation before commit.
5. The production model window obtains only the exact current `Fem2dLiveVisual`; stale or colliding
   app authority produces no visual.
6. App maintenance and app close both drive the mounted session. Terminal close is witnessed before
   the broader app reports terminal.

## Fixed ownership and admission

- Current app authorities: fixed 32 direct slots with exact app identity validation.
- Retained job shells: fixed 64 slots with a free ring and a 52-bit counter under a fixed domain tag.
- Job limits: 4,096 items, 4 MiB retained input, 64 node units, and 16 KiB output per admitted job.
- Snapshot ownership is moved from the registered store lease only after all registry reservations
  succeed.
- Nested FEM input preflight retains outer, inner, and deep cursors and advances one region point,
  hole owner/point, load, combination term, or scalar collection entry at a time.
- Assembly may retain the exact `Arc<AnalysisModel>` through `AssemblyJob::new_owned`; no borrowed
  model is smuggled across turns.

## Changed source

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🦀️component.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️component.rs`
- `✏️s/🔌️plugins/🏗️fem/🦀️component.rs`
- `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️session/🦀️component.rs`
- `📜️script.ts`

The pre-edit census is in `📓️p6g-mounted-fem2d-operation-session-census-2026-08-23.md`.

## Permanent source evidence

The root verifier checks live mount reachability and rejects these mutations:

- resizable session registry;
- truncated canonical base authority;
- missing job-domain ABA validation;
- whole nested input preflight;
- commit without exact validation;
- restart without cancellation;
- renderer fallback to `None`; and
- run-to-completion PCG child driving.

The session source also has focused fixed-capacity, collision/reuse, cancel-before-spawn,
hostile-input identity, and base/generation commit fixtures.

## Gates

- Scoped Rust formatting: PASS with edition 2021 and `skip_children=true` on shared module roots.
- Tool-job verifier self-tests: PASS, 318.
- Full tool-job verifier: expected DENY, 0 admitted / 884 residual / 18 failure classes. No P6g
  mounted-session failure is present.
- Deterministic full verifier ledgers: `📊️p6g-tool-jobs-a.json` and
  `📊️p6g-tool-jobs-b.json` are byte-identical with SHA-256
  `2ad91c55d42b6ce2a086bf138aa105a647eb0b38ee0c96639545f65ddccbafec`.
- Negative live-route scan for `block_on`, `run_to_completion`, private `WorkerPool::new`, and a
  child-step `while` loop: zero findings.
- Scoped, whole working, staged, and HEAD diff hygiene: PASS.

No Cargo, Nx, Wasm, browser, or runtime command was run.

## Honest residuals

- P6h remains required for model construction, sparse assembly, PCG internals, and other numerical
  loops that are stateful but not yet proven one numerical micro-opportunity per grant.
- P6i remains required for the model-sized `Fem2dLiveVisual` JSON/layer encoder.
- The underlying shared reactor registry and the broader Phase 8 ownership/disposal work are not
  accepted by this packet.
- The full tool-command roster remains red at 0/884. This packet does not activate a typed tool
  command.
- Source validation is not a runtime claim.
