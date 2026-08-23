# P6g Mounted Fem2d Operation Session — 2026-08-24 Third Remediation

## Verdict

**SOURCE-AUDIT-READY.** The three blockers in the independent second-remediation audit are closed in
the production-mounted route. Live progress now publishes only a retained, generation-qualified
visual lease; mounted mesh classification has fixed observed vector authorities and no ephemeral
classification collection; element stiffness backing is quarantined, observed and admitted before
adoption. Snapshot return credit remains held until the store registry proves exact terminal return.

This is a static/source handoff. Cargo, Nx, Wasm, browser, runtime, network and broad builds were not
run by contract. P6h/P6i are not started by this packet; their broader numerical and publication
work remains separately scoped.

## Live production route

1. `VcsArtifactApp` binds app instance, base revision, store generation, canonical base bytes and an
   exact registered snapshot read into `ArtifactView`.
2. FEM2D reconciliation retains a fixed generation-tagged job shell and emits cancellation before
   replacement spawn.
3. Each mounted worker grant advances one retained graph, mesh, assembly, PCG or visual cursor.
4. Visual changes mark a candidate dirty. A mounted encoder reserves and observes fixed output and
   order backing, advances stable region/assembly/field ordering and scene emission incrementally,
   then seals an immutable lease for the exact app/revision/generation.
5. Publication revalidates the live session/store authority. Latest-wins replacement moves the old
   lease into retained incremental close; rejected publication returns the complete candidate owner.
   The last valid lease remains visible until its replacement is sealed and admitted.
6. The production `render_with_progress` route consumes only the sealed lease output. It has no call
   to the batch structure, mesh-triangle or live-overlay encoders and performs no sorting, collection
   or `serde_json` encoding.
7. Final solve publication first requests and seals the lossless final visual, then exposes `Done`.
   Cancellation, fault, lost handle, replacement and app close all retain their output owners until
   exact incremental retirement.

## Third-remediation ownership repairs

### Mounted visual/output owner

- `Fem2dMountedVisualBuild` owns a bounded JSON writer, region/assembly/field order indexes, semantic
  cursors, exact output pages and a sealed lease slot.
- Output backing is reserved, observed against the 16 KiB authority and admitted before any byte is
  written or transferred.
- Region, assembly and field ordering is stable and incremental; production progress rendering no
  longer reaches `fem2d_live_visual_layers`, `fem2d_structure_layers`,
  `fem2d_region_triangles`, collection sorting or whole `serde_json` encoding.
- Candidate/current/displaced are distinct retained owners. Generation mismatch, lost publication
  authority and latest-wins displacement preserve the full owner for exact close/handback.
- `mounted_visual_output_exact_maximum_plus_one_and_displacement_handback` covers exact maximum,
  maximum + 1 rejection, stable ordering, immutable current output and exact displaced retirement.

### Mounted mesh classification/index authority

- Mounted preparation point lookup, fixed constraints and indexed edges are fixed vector authorities,
  each reserved and observed before entries are admitted.
- Mounted triangulation points/faces/insertion order and the three simultaneous insertion workspaces
  are pre-reserved, observed against their page authorities and retained on allocation failure.
- `MeshJobStage::Classify` borrows exactly one face with
  `triangulation.triangles.get(self.face_cursor)` and appends at most one admitted output face per
  grant. It contains no `collect()` or temporary `Vec` construction.
- `mesh_mounted_classification_indexes_admit_maximum_reject_plus_one_and_close_exactly` covers exact
  index maximum, +1 rejection and populated close handback.

### Element stiffness backing

- Mounted element build now advances through
  `ReserveStiffnessCredit -> AllocateStiffness -> ObserveStiffnessBacking -> AdmitStiffnessBacking`.
- Kernel output is quarantined inside the retained pending builder. Its actual dimensions, capacity
  and bytes are inspected against the already-reserved page credit before ownership is adopted.
- Rejected allocation stays in the pending owner and retires through the same exact vector close
  cursor; there is no post-check-only ownership credit.
- `mounted_element_stiffness_observes_before_admit_and_retires_rejected_backing` supplies the hostile
  over-page rejection and exact rejected-allocation retirement fixture.

### Store lease terminal qualification

- `SnapshotRead::return_to_registry_witness` returns a generation-qualified
  `SnapshotReadReturn` witness.
- The mounted close state retains that witness until the store registry reports that exact returned
  slot terminal. Process credit and the session shell are not released while the store still owns the
  snapshot root.

## Exact 30-class inventory

The catalog remains exactly 30 simultaneous owner classes and now names the real vector/page owners:
`MeshPreparationIndexVector`, `MeshTriangulationWorkspaceVectors`, `MeshEdgeIndexVectors`, five
visual vectors, 85 visual string roots and twelve output pages for candidate/current/displaced.

- Fixed owner roots: **398**.
- Fixed retained items: **3,806**.
- Input maximum: 4,096 items and 4 MiB.
- Exact process maximum: **8,300 items** and **5,824,512 bytes**.
- One additional input item, input byte or inventory page is rejected without partial registry credit.
- Process-zero and full FIFO shell/credit handback remain covered by
  `process_owner_inventory_admits_exact_maximum_and_returns_exact_credit`.

## Permanent verifier evidence

The live predicate now reads the FEM model-window and mesh sources in addition to the session,
analysis, store, reactor and plugin sources. Its hostile mutations independently reject:

- restoration of the whole live visual encoder in production render;
- removal of the stable visual field-order cursor;
- restoration of the classification `collect()`;
- removal of actual stiffness-backing observation;
- removal of the store return witness; and
- weakening full visual slot handback to a boolean result.

The predicate takes eleven explicit source arguments. The live caller passes the framework plugin as
the eleventh `frameworkPlugin` argument; the earlier undefined argument was a transient concurrent
mid-edit state and is not present in this handoff.

## Changed source

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️component.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️session/🦀️component.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️component.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
- `📜️script.ts`
- this report and the two deterministic P6g ledgers.

## Scoped verification

- Scoped Rust formatting on the five changed Rust sources: **PASS**.
- Tool-job verifier self-test: **PASS**, 328.
- Two live-source verifier ledgers: byte-identical, SHA-256
  **`e755ebcb3ab6e3004b563017f7a573bdafc59b36b2c95ea9cd4c1a7ab7ab3afc`**.
- Live-source result: expected global **DENY**, 0 admitted / 884 residual / 18 failure classes;
  **no FEM2D/P6g failure remains**.
- Cargo, Nx, Wasm, browser, runtime, network and broad builds: **not run**.

## Scope boundary

The broader P6h numerical microcursor and P6i public live-visual publication contracts remain
separate coordinator work. This P6g remediation does not claim their unrequested surfaces and does
not start P1q or P1w.
