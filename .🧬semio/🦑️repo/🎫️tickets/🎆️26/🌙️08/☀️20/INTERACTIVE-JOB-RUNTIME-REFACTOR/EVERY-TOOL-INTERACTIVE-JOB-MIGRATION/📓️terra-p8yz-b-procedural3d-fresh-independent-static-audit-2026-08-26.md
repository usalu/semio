# P8yz-b Procedural3d Fresh Independent Source/Static Audit

Date: 2026-08-26  
Auditor: Terra (`/root/p8yz_b_procedural3d_audit`)  
Scope: live Procedural3d retained mounted source, local language-neutral fixtures, accepted P8yz-a material, and the submitted P8yz-b implementation report. No production source was modified. No Cargo, Nx, Wasm, browser, or runtime command was run while Flow Rust work is active.

## Verdict: RED — Required Global Raw-Caller Census Is False

The live Rust census is **8**, not the required **9 = shared guard + 8 peers**. It contains the shared Store guard and only seven peer callers:

1. Shooting
2. FEM 2D
3. FEM 3D
4. CAD
5. Puzzle 5D
6. Puzzle 3D
7. Framework directed DAG

The claimed eighth peer, framework Flow VCS, is absent from the exact current `*.rs` result. Procedural3d is also absent, as required. This conflicts with the P8yz-b submitted report's 9-path statement. Therefore the packet cannot be accepted against its explicit global-census contract, even though the isolated Procedural3d source/static boundary below is GREEN.

Exact repeated command and result:

```sh
rg -l --glob '*.rs' 'reject_whole_buffer_artifact_envelope_ingress' | sort
rg -l --glob '*.rs' 'reject_whole_buffer_artifact_envelope_ingress' | wc -l
```

```text
shooting/…/wasm/component.rs
fem/…/2d/…/wasm/component.rs
fem/…/3d/…/wasm/component.rs
cad/…/wasm/component.rs
puzzle/…/5d/…/wasm/component.rs
puzzle/…/3d/…/wasm/component.rs
framework/…/directed/dag/component.rs
framework/…/store/component.rs
8
```

## Evidence Read

- Current root and Procedural-plugin `AGENTS.md` instructions.
- Accepted P8yz-a implementation and independent audit, the P8yz prerequisite report, and the P8yz-b submitted implementation report.
- Live P3 Wasm bridge, editor, mounted snapshot owner, and retained mutation/initializer owner.
- All three P8yz-b JSON law fixtures.

No P8yz-bearing `master-plan.md` was present in the ticket hierarchy discovered from the workspace; the retained implementation/accepted audit material supplied the directly applicable contract. Repository MCP goal/ticket tools were not exposed to this audit process, so ticket lifecycle was not changed.

## GREEN: P3-Local Static Boundary

### Canonical retained ingress

- `P3D3` is the mounted prefix. `admit_byte` compares each prefix byte before it calls `allocate_after_discriminator`; the hostile source law feeds the `P2D2` second byte and asserts that no typed semantic owner exists.
- The five retained canonical owners are present: source, anchor, segment/raw-DEFLATE, catalog, and value. The fixed typed P3 snapshot owner follows them.
- Scoped production slices contain none of the 19 prohibited whole-route spellings: whole hex helpers, `Vec<u8>`, `RecordValue`, `decode_pack`, `decode_document`, whole serde decoding, direct `ArtifactStore::new`, generic diff/apply, or generic clone.
- The non-empty law constructs neuron and output-preview widgets, a valid synapse, two layout entries, recursive neural/dictionary/cluster/generation content; it checks exact synapse row, field-delimited digest, exact layout, full typed equality, unchanged post-discriminator ingress digest, and terminal-empty close.

### Ownership, mutations, and bounded lifecycle

- The retained owner catalog includes all 14 P3 mutations, explicitly including 3D-only `delete-widget-position`; combined depth is 12. It includes history edit/meta/inverse/cursor/redo/checkpoint, fresh conflict rejection, cluster children, control, and four output authorities.
- The direct initializer validates the envelope, copies the initial snapshot, seeds history/meta, replays forwards per field, observes inverse and redo lanes, restores checkpoint, creates the initialized runtime, and closes displaced/candidate owners incrementally. No generic store diff/apply/clone route is present in the scoped initializer path.
- The bridge provides fixed begin/preflight/admit/seal/poll/take/resume/retry/output-ACK/load-ACK/cancel/close transitions. Its local laws cover zero/max/+1 credits, repeated rejected controls, producer preflight, progress/checkpoint/preview/terminal output behavior, lost leases, stale/wrong/ABA identity, interrupted and idempotent close, and populated ordinary-drop failure.
- Source laws exist for insufficient fuel/deadline, cancelled/stale initialization, fourteen-variant structural decoding, and terminal-empty retirement. They were inspected but not executed.

### Publication and oracle boundaries

- The real `VcsArtifactApp` law drives maintenance, validates authority immediately before the atomic replacement path, asserts all-field/all-14 equality and digest, explicitly ACKs, and checks Missing/WrongOperation/WrongGeneration/WrongBase/WrongParent candidates retain the last valid snapshot.
- The language-neutral oracle fixture is non-empty and exact: 2 widgets, 1 valid synapse, 2 layout entries; the expected move is `(12.5, -8.25)`. The oracle declares the owned `Procedural3dSemanticOracle` interface and `serde_json` as an existing test-only dependency with `runtimeDependency: false`. The Rust oracle law is `#[cfg(test)]`; no runtime-oracle result is claimed because Cargo is deferred.

## Commands and Independent Probes

All commands listed here exited successfully unless their result is shown above.

```sh
rustfmt --edition 2021 --check \
  procedural3d/editor/wasm/component.rs \
  procedural3d/editor/component.rs \
  procedural3d/snapshot/binary/component.rs \
  procedural3d/mutations/binary/component.rs
```

Result: `rustfmt_exit=0`. This is a parse/format-shape check, not a typecheck or runtime claim.

Standalone Bun parsing of the three JSON fixtures reported:

```json
{"status":"PASS","discriminator":"P3D3","routeLayers":7,"mutationOwners":14,"combinedDepth":12,"nonempty":[2,1,2],"semanticResult":{"widgetCount":2,"synapseCount":1,"layoutCount":2,"movedId":"source","x":12.5,"y":-8.25,"synapseId":"source-preview","fromPort":"solid","toPort":""},"oracle":{"library":"serde_json","scope":"test-only-existing-dependency","ownedInterface":"Procedural3dSemanticOracle","runtimeDependency":false}}
```

An independent static source predicate reported:

```json
{"status":"PASS","productionSlices":3,"forbidden":19,"canonicalLayers":5,"bridgeTransitions":13,"vcsFailureCases":5}
```

Read-only in-memory hostile mutations were also rejected by independent Bun predicates:

```text
PASS mounted hostile mutation probe
PASS envelope and bridge hostile mutation probe
```

The hostile inputs respectively removed every canonical value-layer spelling or injected `Vec<u8>`/`decode_pack`; and injected a forbidden whole-route/direct-store spelling into the typed-envelope or production-bridge slices. They were not written to disk.

## Deferred Gates

Cargo/native/Wasm execution remains deliberately deferred. Consequently this audit does not claim executable P3D3 round-trip, actual VCS swap, credits/lifecycle behavior, runtime oracle equality, or browser behavior. Once the Rust-source embargo lifts, run the submitted focused P3 Cargo tests, native library suite, Wasm check, and coordinator-owned browser/timing matrices, then re-run the global census before reconsidering this verdict.

## Blocking Defect

Restore or formally revise the exact raw-caller acceptance ledger so that the live result satisfies the stated contract: **9 Rust paths = one shared guard + eight peer callers**, with Procedural3d absent. Do not treat the submitted report's historical count as evidence for the current tree.

