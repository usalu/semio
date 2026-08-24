# Terra Independent P6g Third-Remediation Acceptance Audit

Date: 2026-08-24  
Scope: current working tree; read-only source and scoped static verification only.

## Verdict

**GREEN — accept P6g third remediation.** The two residuals recorded in the prior RED audit are closed in the production-mounted route. No P1q, P1w, P6h, or P6i work was started.

## Prior Residual 1: Mounted Progress To Render

This route is retained and generation-qualified end to end.

- The reactor's `Event::JobProgress` obtains the fixed accepted job binding and dirties precisely that instance window; `JobCompleted` does the same while completing the binding: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️component.rs:1121` and `:1135`.
- The actual Fem2d caller is `with_live_visual(doc.render_operation(), ...)`, and it calls `render_with_progress` with the borrowed live lease: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:332`.
- `with_live_visual` resolves the current fixed shell and refuses an instance, base-revision, or generation mismatch; it supplies only `visual_current` whose lease matches all three values: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️session/🦀️component.rs:2253`.
- `Fem2dMountedVisualBuild` is a retained staged output/order owner. It reserves then observes/adopts the fixed 16 KiB output backing, retains region/assembly/field ordering cursors, emits incrementally, and moves the exact backing into a generation-qualified sealed lease: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️component.rs:255`, `:311`, and `:554`.
- Candidate, current, and displaced lease owners are distinct. A stale candidate and a saturated/displaced candidate remain exact retained close owners; current becomes displaced only through `publish_visual_candidate`, and the displaced owner closes before a later candidate can open: session `:923`, `:1024`, and `:1033`.
- Close walks candidate, displaced, and current separately and requires their terminal witnesses: session `:1556`, `:1573`, and `:1590`.
- Production `render_with_progress` consumes `Fem2dMountedVisualLease::layers_json` and contains no live-layer encoder, structure/triangle encoder, `serde_json`, sort, or collect path: model `:861`. Its `to_owned` is the bounded scene-value copy required by the canvas value API; it does not clone the document/model or construct visual data.

Therefore the historical batch `fem2d_live_visual_layers` path remains present only for non-mounted helpers/tests and is not production-reachable from mounted progress rendering.

## Prior Residual 2: Mesh Classification And Stiffness

- The bounded mesh preparation, edge authorities, point index, output points, and output triangles reserve before their consumer stages and reject backing above the fixed page authority: `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️component.rs:1027`, `:1106`, `:1188`, `:1196`, and `:1204`.
- `MeshJobStage::Classify` reads exactly one retained face through `triangulation.triangles.get(self.face_cursor)`, appends through retained fixed indexes/output vectors, and advances one face cursor. Its block contains no `collect`, `Vec` construction, or `BTree` authority: mesh `:1211`.
- The file's `BTreeMap`/`BTreeSet` occurrences belong to the separate unbounded `OwnedTriangulation`/`triangulate` route; the mounted route starts at `MeshJob::new_bounded` and uses vector authorities. They are not reached by the mounted `Classify` stage.
- Element stiffness follows the required retained sequence: `ReserveStiffnessCredit -> AllocateStiffness -> ObserveStiffnessBacking -> AdmitStiffnessBacking`. Allocation remains in `PendingElementBuild`, actual capacity and dimensions are checked before admission, and only then moves to `PendingElementAssembly`: `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️component.rs:1476` through `:1516`.
- A rejected observed stiffness backing remains in `pending_build`; `AssemblyJob::close_step` retires stiffness, positions, and indices one exact owner at a time: analyses `:1238` through `:1259`. The hostile over-page backing fixture verifies the same retained close helper: `:2505`.

## Store Return Witness

`SnapshotRead::return_to_registry_witness` creates a registry/index/generation witness only after exact return. The witness is terminal only after that exact slot is absent from the registry: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:237` and `:276`. Mounted close retains the witness until that terminal condition, before permitting session/shell credit release: session `:1598` through `:1612`, and `retire_one` releases credit only after the whole state reports terminal: `:2271`.

## Earlier Accepted Repairs Rechecked

1. Progress invalidation and exact fixed binding: confirmed in the reactor event path above.
2. Lazy snapshot issuance: the framework computes `snapshot_is_admitted` through `A::mounted_job_prepare_snapshot_read` before `self.store.snapshot_read().await`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:19943`.
3. Commit revalidation: the mounted `CommitReady` and `PublishFinalVisual` paths re-check current identity, exact generation/canonical store authority, and cancellation before exposure: session `:1300`.
4. Retained construction and close: graph, mesh, model builder, assembly construction/job, CSR, PCG, domain, visual owners, snapshot witness, and fault all have separate retained close lanes: session `:1336` through `:1629`.
5. Fixed registry/ABA/source admission: 32 active slots, 64 FIFO shells, tagged job IDs, retained snapshot census, cancel-before-spawn, and exact process credit are in session `:17`, `:79`, `:2187`, and `:2245`.
6. Catalog accounting and FIFO credit handback: the 30-class catalog has a one-for-one class test, exact maximum credit, plus-one rejection without partial credit, and FIFO shell return: session `:2425`.

## Recomputed Inventory

I independently transcribed the 30 `MountedOwnerClaim` pairs and recomputed their totals.

| Quantity | Computation | Result |
| --- | --- | ---: |
| Owner classes | claim array length | 30 |
| Fixed roots | sum of all claim roots | 398 |
| Fixed retained items | sum of all claim items | 3,806 |
| Process items | `4,096 + 3,806 + 398` | 8,300 |
| Process bytes | `4,194,304 + 398 × 4,096` | 5,824,512 |

These exactly match `SESSION_MAXIMUM_ITEMS` and `SESSION_MAXIMUM_BYTES`. Input item/byte plus-one and inventory-page plus-one are both rejected by the source fixture without partial registry credit.

## Verifier And Mutation Audit

The predicate has eleven explicit source parameters. The live call reads the actual model, mesh, analysis, session, store, reactor, editor, FEM root/glue/jobs, and passes the actual framework plugin source as the eleventh `frameworkPlugin` argument: `📜️script.ts:4681` through `:4687` and `:4766`. The framework argument now points to the real `A::mounted_job_prepare_snapshot_read` before snapshot issuance.

All six third-remediation hostile mutations are faithful parameter-position mutations against the predicate and were exercised by the self-test:

1. restore the whole visual encoder;
2. remove the stable field-order cursor;
3. restore classification `collect()`;
4. remove stiffness-backing observation;
5. remove the store return witness; and
6. reduce exact visual handback to a boolean.

The predicate rejects each at `📜️script.ts:3514` through `:3524`.

## Executed Checks

- `rustfmt --edition 2021 --check --config skip_children=true` over the five changed Rust sources: exit 0.
- Scoped working-tree and staged `git diff --check` over the six P6g source/verifier files: no output, exit 0.
- `bun ./📜️script.ts verify interactivity tool-jobs --self-test`: `[verify interactivity tool-jobs] self-tests=328 clean.`
- `bun ./📜️script.ts verify interactivity tool-jobs`: expected nonzero global gate (`0 admitted`, `884 remaining`); the emitted failure list contains no FEM2D/P6g predicate failure.
- `📊️p6g-tool-jobs-a.json` and `📊️p6g-tool-jobs-b.json`: byte-identical; SHA-256 `e755ebcb3ab6e3004b563017f7a573bdafc59b36b2c95ea9cd4c1a7ab7ab3afc`.

No Cargo, Nx, Wasm, browser, runtime, network, or broad build was run.
