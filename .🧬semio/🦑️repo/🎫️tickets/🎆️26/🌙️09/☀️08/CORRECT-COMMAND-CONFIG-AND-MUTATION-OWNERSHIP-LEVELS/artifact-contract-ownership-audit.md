# Artifact Contract Ownership Audit

## Evidence

The expanded presence review exposed a separate root artifact contract problem. Forty-seven artifact schemas still declare fields explicitly classified as config, presence, or transient. Their Rust aggregate types and sparse artifact diffs duplicate the app or surface contracts. For example, Puzzle 2D's `Puzzle2dArtifact::from_snapshot` supplies default selection, utility, camera, and interaction fields that its `to_snapshot` method omits; its artifact diff can independently change those same UI fields. Those fields have existing app config/presence owners and live consumers already use those owners.

This is the same ownership leak already removed from Jack and Rewriting, now evidenced across other artifact families. Document camera fields explicitly belonging to a snapshot remain artifact data. Removal targets the duplicate aggregate declarations, matching sparse diff members and codecs, and their authored fixture copies. It does not delete the proper app/window owner.

## Validation Plan

Extend the existing language-independent document-only artifact law beyond the four Trinity root/diff schemas. Run it before changing the affected contracts, then remove the duplicates in all five schema representations and update their direct consumers. Re-run schema field/state parity, applicable artifact mutation and round-trip laws, and native checks. Names alone do not establish ownership; separate incorrectly annotated transient-looking fields require producer/consumer review.

The initial inventory is temporary generated evidence under this ticket. Final checked ownership and validation results will be retained in this report.

## Contract Validation

The expanded 98 root/diff contract fixture first failed against Lowpoly's duplicated UI fields. After removing the explicitly classified duplicates, strict Ajv accepted all 98 document-only property contracts. The ownership report also passed its 9 direct and 4 nested vectors and reported zero misplaced surface declarations. This establishes the normative JSON contract state; five-format parity and runtime validation remain in progress.

The first native host build overlapped the schema and diff-codec edits and reported Puzzle 3D diff members that had just been removed. The current diff source no longer contains those stale accesses. That run is not a runtime pass and must be rerun after its remaining dependency jobs finish.

The global dialect scan also covered FEM's differently named `🌐️any` subsets and found 16 additional root/diff declarations. These four duplicated UI fields per FEM artifact have been removed, bringing the fixture to 102 contracts across 51 artifact owners including Trinity.

A separate producer/consumer review identified 33 fields in 15 artifact owners incorrectly marked as artifact data: hover projections, gesture drag flags, preview sequence counters, generation previews, brush previews, and FEM solver/mesh display output. These fields are omitted by the authored document snapshots and already have live interaction or application owners. The new neutral regression rejected an artifact containing hover and generated-preview state; it first failed before the checker change. Their duplicate artifact/diff declarations, embedded artifact definitions, codecs, and mutation fixtures have now been removed. Current live app/window owners remain under their respective implementation lanes.

Rust storage layout is not treated as wire shape: Jack's retained `content` storage, for example, serializes graph nodes and edges. The five-format check verifies declaration identity and owner state. Runtime codec and mutation tests establish wire behavior; a textual struct-field comparison would incorrectly reject valid storage abstractions.

## Native Validation Contract

The permanent `workspace:verify-artifact-contract-ownership` target derives the 51 owning Rust crates from the neutral artifact contract fixture and compiles their test targets. `workspace:test-artifact-contract-ownership` executes the existing `produces_committed_diff` and `committed_diff_carries_before_to_after` fixture laws across the same owners, checking exact generated wire deltas and their document effect. Both commands have launch configurations. Native validation remains pending while the shared window-config interface is integrated; source-only success is not runtime proof.

## Current Enforcement Result

The isolated Nx `abstraction-ownership-validation:enforce` run completed successfully in 10.4 seconds with zero misplaced declarations. It checked 10 ownership vectors, 4 nested-schema vectors, 3 command-source vectors against Ajv, and 102 artifact/diff contracts across the five schema formats. This closes the remaining orphan utility enum findings. Native compile/runtime checks and the semantic window-state audit remain separate requirements.
