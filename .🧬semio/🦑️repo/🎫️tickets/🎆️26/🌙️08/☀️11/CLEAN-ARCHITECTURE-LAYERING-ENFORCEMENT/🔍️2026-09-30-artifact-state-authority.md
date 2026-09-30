# Artifact State Authority Audit

Read-only source inspection of the sequence editor and the generic Rust artifact store on 2026-09-30. No general state rewrite was performed. The separately documented focused IO target passed real native/runtime checks.

## Observed Authoritative Boundary

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:15605` defines `ArtifactStore` with private envelope, current snapshot root, event DAG, cursor ledgers, generation, and revision. `snapshot()` returns a cloned value (16251), `snapshot_ref()` an immutable reference (16256), `snapshot_read()` an immutable generation/revision lease (16261), and `snapshot_owner()`/`snapshot_root()` cloned `Arc<P>` roots (16275–16281). No `ArtifactStore::snapshot_mut`, mutable current root, or mutable envelope accessor was found. `envelope()` at 16099 returns an immutable reference.

`dispatch` (17205) routes commands through private `dispatch_inner`; `begin_apply_batch` (17243) takes typed mutations and expected generation/revision; `advance_apply_batch` (17501) drives admitted publication. These are the visible mutation publication paths. A full transitive proof of every store implementation branch is outside this narrow audit.

`reset` (16151) is a separate public adoption boundary, not an arbitrary current-root setter: it adopts a whole event-log envelope through crate-private `set_state`, private `adopt_state`, `validate_durable_history`, `fold_envelope_history`, `validate_history_lanes`, and `fold_history`; then publishes roots and bumps the generation. Classify this as validated document reload/genesis adoption. A requirement that literally all state changes must be mutations would need to specify the allowed initialization/reload authority explicitly rather than treating this method as a silently exposed snapshot mutation.

`set_local_actor_id` and `set_merge_policy` concern local actor/merge policy. Checkpoint repinning uses `set_checkpoint_composition_pins` (16227), rederives content-addressed identity, and records a Repin history transition. None of these inspected APIs directly exposes a mutable live artifact snapshot.

## Mutable Candidates and Scratch Data

`ArtifactEnvelopeOwners` has public fields (2654), and a standalone `ArtifactEnvelope` implements `DerefMut` (2865). These expose candidate/replay-envelope construction. The live store does not expose `&mut ArtifactEnvelope`, so the existence of a mutable envelope DTO alone does not prove a live store write bypass.

`ArtifactStoreInitializationRuntime::current_mut` (14057) exposes its owned initialization candidate before adoption; the live `ArtifactStore` root remains private. Its initial-digest consistency should be covered by initialization-specific tests rather than a broad source prohibition on every method named `current_mut`.

`SequenceHost` owns an editable `SequenceHostSnapshot`, camera, and derived DAG (sequence editor 475). `replace_snapshot` (536) changes this host-owned scratch value after checking its schema. The node-graph command's `setHostSnapshot` path calls it inside `sequence_child_emit_from_host_mutation`. That helper (194) constructs a cold host from the captured artifact/child read, applies scratch edits, and returns typed `SemioFlowMutation::SetSnapshot` in a `ChildEmit` on the exact content-child lane (181). It does not hold `ArtifactStore` or directly assign store state. The older `ops_from_host_mutation` helper (947) likewise returns typed `SequenceMutation` values computed from the scratch difference.

## Finding and Limits

No externally accessible mutable **live Rust ArtifactStore snapshot** bypass was established by this inspection. The earlier suspicious sequence `replace_snapshot` call is a scratch edit that produces a mutation emit. Generic `Arc<P>` read roots cannot by themselves exclude a domain snapshot type with interior mutability; proving that invariant would require a schema/type-level contract and representative domain checks. The TS store implementation and every other plugin were not exhaustively audited, so this is not a repo-wide claim.

## IO Renderer Purity Enforcement

The generic resolver currently owns `DependencyDirectionRule` and checks resolved graph edges, including type/dynamic/export imports. A schema/taxonomy contribution can express a strict rule with source scope `🚪️io` and targets in renderer/UI engine roles, plus external renderer package targets. That would be more reliable than searching file text for react/bevy/egui strings and would not mistake comments for dependencies. Contribution ownership should be generic framework IO; target roles belong to renderer contributions.

At inspection time `dependencyDirectionEdges` required exactly one strict rule (line 26 of `🕸️dependencies/🧭️direction/🟦️.ts`), so adding an IO rule needs the generic contribution aggregation to accept multiple declared rules and independently verify each resolver verdict. Rust purity also needs resolved Cargo/module ownership, not merely package edges: the same framework crate can contain both renderer and IO modules. This audit did not introduce a special-case regex or claim that the existing area-layer rule already proves renderer independence.

## Executed Authority Gate

After this source audit, the root added the existing runtime authority laws to the owning kernel's canonical contribution. `@semio-tech/framework-os-kernel:canonical-architecture` passed 13 exact native laws, including the neutral publication fixture/third-party JSON oracle, immutable roots, one/200 mutation publication, cancellation, forged digest/cursor refusal, stale/saturated admission, wrong-owner checks, policy and shared schema formats. These results support the inspected live-store authority boundary and do not extend the source audit into a universal interior-mutability proof.
