# Boot failure: "command wire exceeds its declared maximum" on setContributions

## Cause (guest side)
The host sends `setContributions` as one pack-encoded invocation (~136 pages x 4 KiB, about 557 KB for the generator hub; the flow extension closure). The guest decodes it to `{action,args}` and `prepare_original_typed_action` runs the app's declared action producer, which re-encodes the typed command through `OpBinaryProducer` (`📜️declaration/👷️job/🎮️command/🧬️op-binary/🦀️.rs`, phase ENCODE: `bytes.len() > self.maximum` -> OwnershipLimit "command wire exceeds its declared maximum").

The default producer (`declared_action_command_bridge`, `EditorApp`/`ViewerApp` fallback) prices every command at `RUNTIME_COMMAND_WIRE_MAXIMUM` = 65 536. generation3d (editor+viewer), generation2d and flow declare their `setContributions` tool contract at `COMMAND_MAXIMUM_BYTES` (8 MiB, "crosses whole"), but their action producer never followed that declaration, so the real pack exceeded the producer's ordinary ceiling. Not a wire accounting bug: the host re-offer of backpressured pages is correct, pages are counted once (`written == measured` invariant), and the pack-cut per app (`consumedTopicsForApp`) is unchanged (the generator app is meant to receive every flow extension).

## Fix
Same owner-selected-maximum mechanism lowpoly already uses for its 2 MiB imports (`OriginalActionCommandBridge(birth, maximum_for)`): each app that declares a whole-pack contributions route overrides `original_action_command_producer` and prices `SetContributions` at its declared constant, everything else at `RUNTIME_COMMAND_WIRE_MAXIMUM`:
- generation3d editor `🪟️artifact-editor-impl/🦀️.rs` (`GENERATION3D_CONTRIBUTIONS_RAW_BYTES`)
- generation3d viewer `👁️viewer/🔭️viewer-and-interaction-topology/🦀️.rs` (`GENERATION3D_VIEW_CONTRIBUTIONS_RAW_BYTES`)
- generation2d editor `🕹️play-app-editor/🦀️.rs` (`GENERATION2D_CONTRIBUTIONS_RAW_BYTES`)
- flow editor `🎛️flow-play-app/🦀️.rs` (`FLOW_CONTRIBUTIONS_RAW_BYTES`)
Other apps (cad, process3d, sourcing) receive packs cut to <= 49 152 bytes and stay under 65 536.

## Test
`generation3d/🧪️tests/🔬️action-producer/🦀️.rs`: `a_pack_wider_than_the_ordinary_runtime_wire_reaches_its_command` dispatches a 128 KiB `setContributions` action and asserts the refusal (if any) is not the "declared maximum" one.

## Status / risks
- NOT yet compiled or run: `repo cargo --store=shared` stalled waiting for compiler image space (0.1-0.3 GiB free of 4 GiB). Re-run `check -p semio-s-artifact-procedural-3d-editor -p semio-s-artifact-procedural-2d-editor -p semio-s-artifact-flow-flow-editor -p semio-s-artifact-procedural-3d-viewer` once space is free. Possible compile friction: viewer-side visibility of `GENERATION3D_VIEW_CONTRIBUTIONS_RAW_BYTES` (`pub(super)`), closure-to-fn-pointer coercion.
- ENCODE reserves `capacity_bytes = maximum` (8 MiB) and builds one contiguous Vec of the whole pack (~557 KB), above the 64 KiB `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` soft ceiling. If the live guest then refuses with an allocation/grant fault, the next step is a streamed (paged) encoder through `ArtifactOriginalCommandWire` + `ArtifactPreparedOperationSource` instead of the single-Vec producer.

## Rebuild set
Guest only: `procedural` and every hub embedding generation3d/generation2d/flow editors or the 3d viewer (procedural, puzzle, demonstrator-level compositions carrying them). No host TS / site / support restage needed.
