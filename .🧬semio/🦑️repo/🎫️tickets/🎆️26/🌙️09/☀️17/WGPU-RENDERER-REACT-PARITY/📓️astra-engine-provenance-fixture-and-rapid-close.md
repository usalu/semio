# Engine Provenance Fixture and Rapid Close

## Retained engine provenance fixture

Native144's retained engine provenance law stopped before behavior because the fixture searched `ScenePointerTarget.surface_id` for the authored pane ID `procedural-main-graph`. Production deliberately assigns `surface_id` to the owning retained document/window and carries the authored pane identity in the ComponentScene's explicit node key (`pane_id` at projection). The fixture now resolves the accepted target by `NodeKey::Explicit(authored_surface)`, asserts `target.window_id` and `target.surface_id` are the exact document owner, then preserves its original live-host state, pointer, wheel, and forged-owner refusal assertions. No production liveness guard was weakened.

Source: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧲️engine-surface-retention/🦀️.rs`. Rustfmt parse validation is green. Native execution remains root-owned.

## Immediate popup-dismiss and cap close

Checkpoint20 proves a later standalone cap close, final close, and reopen, but its first Top close immediately after dismissing a Window Options popup was ignored. A language-neutral `rapidPopupClose` row now lives in the shell journal fixture. The native law publishes the dock through the real candidate seal/ack path, admits a Window Options Select owner, dispatches one complete outside-dismiss gesture, and immediately dispatches one complete accepted Top-cap gesture over the same presented registry. It requires Top absent, Perspective focused, exactly one `shell.windowClose` note, and no stale candidate.

Source:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧫️fixtures/📜️journal-sequences/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🛰️wgpu-dock-close-reopen-journals/🦀️.rs`

The law is source-coherent and Rustfmt/JSON parse validation passed. Native execution and the required paired Browser21 physical revalidation remain root-owned. Production is unchanged; the source law cannot substitute for the browser counterexample because it does not drive a real DOM/GPU frame between the two gestures.

## Native145 fixture corrections

Native145 confirmed the explicit Chat accessibility field but exposed an exact localization mismatch: the mounted React source and neutral fixture say `Message to the agent` / `Nachricht an den Agent`, while the WGPU locale table said only `Message` / `Nachricht`. The WGPU entries now match the authored labels.

The same run exposed two stale pane fixture assertions from before Actions and Search received independent fold owners. The language-neutral pane fixture now declares no shared folds and lists all five pane chips as independent. The native laws assert Actions and Search separately, including independent sibling state. These changes preserve the prior production repair and update only its exact oracle.
