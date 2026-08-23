# Terra Independent P6g Second-Remediation Acceptance Audit

Date: 2026-08-24  
Scope: current working tree only; read-only production/verifier audit.

## Verdict

**RED — do not accept P6g.** The six findings in the 2026-08-23 coordinator audit are materially repaired in the mounted worker path, but the current production route still performs ungoverned model-sized visual/output allocation and contains an unretained mesh allocation in a governed mesh turn. The permanent verifier does not discriminate either failure, so its passing self-test is not acceptance evidence for this requested contract.

## Confirmed Repairs

| Requirement | Current-source evidence | Result |
| --- | --- | --- |
| Progress dirties the mounted surface | `Event::JobProgress` resolves the current fixed job binding and appends the exact instance window to `dirty_render`; completion does likewise. `JobRenderBindingRegistry` rejects superseded bindings. | PASS (static) |
| Lazy snapshot issue | `VcsArtifactApp::pending_effects` calls `mounted_job_prepare_snapshot_read` before `snapshot_read`; the FEM census creates only a fixed pending admission until complete. Default editors issue no lease. | PASS (static) |
| Live commit revalidation | Store authority is sequence-checked (`publish_authority`/`authority_matches`); `CommitReady` checks the lease against generation and full canonical revision immediately before `Done`. | PASS (static) |
| Mounted numerical construction | Model, domain, assembly-plan, CSR, and PCG construction are retained state machines; zero fuel/deadline returns before work. Assembly uses `mounted_node_id`, `ReserveIndices -> Indices -> ReservePositions -> Positions -> Stiffness -> Complete`, and three reclaim turns. | PASS (static, limited to these owners) |
| Fixed registry, ABA and source admission | 32 current slots, 64 FIFO shell slots, tagged bounded job ids, nested snapshot cursor, capacity observations, and cancel-before-spawn are present. | PASS (static) |
| 30-class arithmetic | The 30 claims sum to 3,698 claimed items and 381 page roots. With the 4,096 input maximum this yields exactly **8,175 items** and **5,754,880 bytes**. The source has exact +1 and FIFO handback fixtures. | PASS (arithmetic/source) |

Mounted `DofMap` is a `Vec<(String, Dof)>`, and the bounded mesh point index is a `Vec<((u64, u64), u32)>`; neither is a mounted `HashMap`.

## Exact Blocking Findings

### 1. The production-mounted visual/output route is still whole-model, allocating and cloning during render

The new reactor correctly dirties the FEM surface on `JobProgress`, then immediately calls the editor render path. That route reaches `fem2d_live_visual_layers` in `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️component.rs`:

- lines 143–187 build a whole `Vec<serde_json::Value>` and traverse all regions/fields;
- lines 144–145 allocate and sort a region collection;
- lines 148–160 allocate each region polyline and JSON layer;
- lines 161–162 clone and sort all assembling ids;
- lines 171–172 clone and sort all live fields; and
- every iteration formats ids and allocates JSON output.

This is now production-reachable specifically because the repaired progress invalidation renders this visual. It has no retained encoder owner, item/byte admission, cancellation, deadline/fuel cursor, incremental disposer, or terminal counter. It fails the requested visual/output one-semantic-unit-per-grant and actual-backing-preflight requirements. The historical P6g report calls this P6i residual; that residual cannot coexist with this audit's stated acceptance contract.

### 2. The mounted mesh path still makes hidden, unobserved allocations inside its worker turn

In `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️component.rs:1095`, `MeshJobStage::Classify` constructs an ephemeral `Vec` with `collect()` before the bounded append loop. It has no retained owner, no `try_reserve_exact`, no capacity observation, and no close cursor. This path is reached by `MeshJob::new_bounded` from the mounted session.

The same owner family also uses dynamic `BTreeSet`/`BTreeMap` insertions for triangulation/edge state without allocator-capacity observation; the 30-class catalog reserves a nominal page per tree node but cannot prove actual backing of those allocations. Therefore the claimed preflight of actual observed mesh backing is not established.

The element `Stiffness` phase additionally adopts `element.stiffness_global(&context).data` only after the callee has allocated it (`analyses/component.rs:1456–1464`); it post-checks capacity rather than pre-admitting the actual backing. This reinforces the same allocation-proof gap.

## Close and Counter Qualification

The mounted state now incrementally disposes graph, mesh, model build, assembly construction/assembly/CSR, PCG, domain, visual roots, snapshot handle, and fault page. That is a substantial repair.

However, its snapshot terminal condition is only local. `MountedState::close_step` calls `SnapshotRead::return_to_registry` and then `retire_one` releases the mounted shell's process credit immediately, while the store's lease registry can still retain the returned snapshot root until its independent cursor processes it. `MountedState::terminal_is_empty` does not witness `SnapshotReadLeaseRegistry::terminal_is_empty`. Thus this audit does not certify the broader "all populated root/control/page counters are zero" claim at the mounted session terminal boundary.

## Verifier Assessment

`toolJobFem2dMountedSessionExact` checks literal source markers and its self-tests mutate a synthetic source string. It does not inspect the live visual encoder or reject `MeshJobStage::Classify`'s `collect()` allocation, dynamic mesh-tree allocations, or late stiffness allocation. Consequently:

- `bun ./📜️script.ts verify interactivity tool-jobs --self-test`: **PASS**, 328; insufficient for the blockers above.
- The existing listed mutations do discriminate progress invalidation, lazy snapshot issuance, store authority, fixed registry/ABA, constructor splitting, and catalog marker removal, but not the two blocking properties above.

## Checks Performed

- Scoped `rustfmt --edition 2021 --check --config skip_children=true` on the mounted session, FEM analyses/sparse/mesh, store, reactor, and plugin source: **PASS**.
- Deterministic ledgers `📊️p6g-tool-jobs-a.json` and `📊️p6g-tool-jobs-b.json`: byte-identical, SHA-256 **`781b2c3a5124f8bedc3bb04f6a59082ce0c2336e3dc65432d510226758c95630`**.
- Working-tree `git diff --check`: no P6g whitespace finding; only the unrelated DXF CRLF warning.
- Staged `git diff --cached --check`: reports six trailing-space lines in coordinator-owned Phase 3/Phase 10 reports. This is an older shared index snapshot; the corresponding current working-tree lines are repaired. It is not a P6g source blocker and no index command was run.

No Cargo, Nx, Wasm, browser, runtime, network, or broad build command was run.

## Required Acceptance Gate

Before a GREEN P6g verdict, cursorize/admit/close the live visual JSON encoder and remove or retain/pre-admit every mesh and element temporary allocation above. Add permanent source mutations that independently fail for (1) visual vector/field clones or whole layer encoding and (2) the mesh `collect()`/unobserved temporary or late stiffness allocation. Then prove terminal store-lease retirement at the same boundary where mounted process credit is returned.
