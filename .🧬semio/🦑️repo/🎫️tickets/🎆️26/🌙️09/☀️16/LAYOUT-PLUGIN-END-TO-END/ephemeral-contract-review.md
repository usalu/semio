# Presence and Transient Contract Review

2026-10-28. Read-only fresh App/Editor/Viewer declarations/adapters, actual PresenceStore/TransientStore execution and presence disposer source. No edits, compiler, test or graph. Shared files are actively changing; findings describe observed source, not a proposed stale whole-file replacement.

## Actual Semantic Capability Boundary

Cold ephemeral execution genuinely needs P:Clone and M:ColdMutation<P>: PresenceStore.apply clones the current local P and calls apply_operation on each borrowed mutation; TransientStore.apply does the same. TransientStore.apply_one and InteractionStore cold paths also invoke actual cold semantics. These methods must retain their precise stronger bounds. Neither iterating borrowed mutations nor constructing/retiring the container independently needs M:Clone.

The retained original publication route is different: begin_publish_one receives the actual original mutation by value plus an app-owned ArtifactEphemeralOneItemPreparationFactory and root retirement factory. Missing producer/factory, stale generation or closing store refuses; it does not default into cold apply. The prepared publication moves factory-produced owned state and retains generation/retirement/grant controls. Thus its container/lifecycle/original-publication contract can truthfully be Mutation-only (plus actual send/share/serialization/retirement constraints required by the surrounding owner), without implementing a fake ColdMutation.

PresenceStore constructors/local reads/maintenance currently share a historically broad Clone+Cold impl, despite bodies constructing Arc roots or borrowing/sharing owners. Those should be segregated only where an actual generic original caller needs them. This is the same domain distinction already explicit in TransientStore: constructor and reads use Mutation-only, apply carries method-local cold bounds.

## Fresh Declaration State

The earlier118surface error snapshot had stronger App associated mutations and weaker Editor/Viewer types with unconditional adapters. That mismatch required correction at a truthful contract boundary, not118per-method constraints.

Fresh current source now has Mutation-only ArtifactApp::PresenceMutation and TransientMutation (Rust correction). Editor and Viewer associated mutations currently have Clone+ColdMutation added by a concurrent owner. Their unconditional EditorApp/ViewerApp adapters therefore satisfy the weaker App ephemeral contract today. Conditional adapter bounds would only hide the trait discrepancy by making some declared editors/viewers fail to become apps; they are not a clean repair when the intended generic original execution does not need cold semantics.

A coherent minimal native App contract is valid and preferable to requiring unrelated cold capabilities. Editor/Viewer may expose a deliberate stronger API if their owner requires it, but their original factory/lifecycle methods do not by themselves justify those extra traits. Do not undo another active owner by stale overwrite; coordinate any normalization. No global artifact StoreState/StoreMutationFor weakening is warranted: those aliases deliberately describe the full persistent artifact store/replay capability set.

## Concrete Current Disposer Mismatch

After Rust reported lowering the presence disposer, a fresh read still observed StoreMutationFor<P> in exactly three presence retirement interfaces: bounded_presence_store_disposer, PresenceStoreOwnedDisposer.owns_terminal and its ArtifactOwnedDisposer implementation. Current SPR StoreMutationFor explicitly requires Clone+ColdMutation. The App default build_presence_store_disposer is now only Mutation-bound, so it cannot establish these observed stronger disposer requirements. This is a concrete remaining contract mismatch if that source persists, likely from overlapping live edits.

The disposer bodies call generation/terminal-root observation and existing bounded PresenceStoreRetirement lifecycle. Those operations do not execute cold mutations; generation is an unconstrained scalar read and retirement owns/moves Arc roots. Use a truthful ephemeral mutation bound for these three interfaces, preserving closure/caller/grant checks and exact terminal predicate, rather than importing the persistent artifact StoreMutationFor alias. Rust/coordinator received the exact observed locations for owner coordination.

ArtifactApp default presence/transient preparation factories return None and explicitly deny the lane. Lowering associated cold requirements does not create an implicit producer or a cold fallback. The actual retained preparation factories that deliberately execute cold semantics retain their own stronger typed requirements unchanged.

## Recommended Repair and Proof

Keep actual cold apply/preparation implementations strong. Keep native structural/original App methods minimal. Align default ephemeral disposer/helper bounds to their real lifecycle operations through coordinated fresh reads; if named bound sets are desired, ephemeral capability deserves its own truthful domain contract rather than reusing persistent artifact StoreMutationFor. Preserve current adapter implementations when they already prove App, and avoid hundreds of incidental where clauses or fake implementations.

The next owned native compile must verify the final concurrent declarations. This source review neither claims the118surface diagnostics cleared nor substitutes for the outside document owner aligning its781structural diagnostics. No requested feature or publication gate was removed by the recommended contract distinction.
