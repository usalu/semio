# Renderer Tutorial Interaction Contract Seam

## Observed Source

- `ShellHelpers::captureTutorialUiSnapshot` reads removed `PluginViewState.selectionJson`. `ActiveSession` contains only plugin/instance/app/viewState; the current view-state schema contains no full interaction selection.
- `TutorialUiSnapshot.interactionSelection` is required. Its per-domain `DomainSelection` includes granularity, ids and optional anchor. Sparse tutorial selection changes currently contain granularity/ids only.
- Plugin `VcsArtifactApp::interaction_state` combines the persisted-local interaction store with ephemeral hover. The selection is not fundamentally a remote-presence projection.
- `PluginHandle.ephemeralSnapshot` and `AppFrame::Ephemeral.interaction` expose only declared-broadcast selection/hover assembled for a heartbeat. Hidden/non-broadcast local domains cannot be reconstructed from that field.
- `pushPresence` writes remote peers, not the local interaction store. It must not be repurposed for tutorial seek.
- Reserved interaction selection dispatch goes through `next_selection`, active selection mode and topology validation. Replaying a multi-id snapshot through normal select may normalize differently under the current mode; it is not an exact restoration API.

## Minimal Producer Authority Needed

1. Read the exact local `InteractionState.selection` through the app channel, including non-broadcast domains, with an instance/revision identity. The renderer can then supply the actual map to tutorial capture instead of storing a fabricated empty map or adding obsolete `selectionJson`.
2. Restore a typed local selection snapshot (or exact one-domain selection) through the framework interaction owner, validated against current domain declarations/topology, and publish the accepted local interaction revision. Clear domains absent from a full snapshot. Return explicit rejection; never silently reroute to remote presence or ordinary active-mode selection.
3. Keep this restore operation in the interaction lane, not the artifact edit history. Any full state/large id processing needs the retained input/cancel/Store ownership rules already required by this ticket.

No Rust/channel API changes made in this renderer packet. No no-op, compatibility field, or cast was added to hide this gap. The coordinator can route the producer seam while renderer BuiltNode/surface and unrelated control contracts are repaired.

## Additional Current Issues

`diffTutorialUiSnapshots` compares ids using comma-joined strings and omits a domain removed from the next snapshot. Both lose exactness; the eventual fixture must include ids containing commas, cleared domains and non-broadcast selection.

The old `patchDocumentTreeSelectedIds` function targets removed recursive UiNode fields. Current `BuiltNode.component.type=tree` binds `interactionDomain` and receives selection through the presence overlay. This must join the actual current overlay/interaction owner, not bolt legacy selectedIds onto the tree record.
