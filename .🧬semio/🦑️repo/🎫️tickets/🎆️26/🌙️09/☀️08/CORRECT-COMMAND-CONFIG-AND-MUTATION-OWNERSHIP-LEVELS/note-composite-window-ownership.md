# Note Composite Window Ownership

## Result

The Note composite editor now owns restorable camera state in the concrete `WindowConfig` partition and owns engagement input in the concrete `WindowTransient` partition. `NoteConfig` is empty. Camera and engagement mutations no longer have a second app-wide owner.

The exact owner pair is registered only for `note-play-window-composite`:

| Lane | State | Address authority |
| --- | --- | --- |
| WindowConfig | `NoteCompositeWindowConfig { camera }` | the captured `ViewModel` window id whose roster entry has the composite kind |
| WindowTransient | `NoteCompositeWindowTransient { engagement_input }` | the captured `WindowTransientSnapshot` id and kind |

`setCamera`, `setCameraZoom`, and camera events from `inkApplyEvents` publish addressed WindowConfig snapshots. `engagementInput` and `engagementSubmit` update the captured transient value. Submit can also publish its artifact rename mutation, so its declared lanes are Artifact plus WindowTransient.

The Note retained command work owner accumulates persistent emissions separately from `EphemeralEmit`. Successful work transfers that ephemeral bundle once through `ArtifactCommandWorkStep::CompleteWithEphemeral` using `mem::take`. Restore clears both bundles. Cancellation and close release one owned item per admitted turn across every persistent and ephemeral lane, including all pending exact-window transient mutations, and terminal emptiness covers both bundles.

The transient owner uses the shared `ArtifactEphemeralTransferPreparationFactory`. Its domain preflight measures the fixed owner metadata plus the engagement string, and its transfer function moves the snapshot mutation directly into the next root. State and mutation owners use `OwnedValueRetirementFactory`; there is no codec, diff, apply, whole-root clone, or app-local publication lifecycle. A tiny-grant regression retires a nonempty string mutation to terminal emptiness.

## Neutral and runtime laws

The neutral fixture carries two concrete composite windows with independent camera and engagement values. The Rust law parses it with third-party `serde_json`, compares project JSON output structurally, and checks project DSL/pack and text/binary operation equivalence.

Two registry-backed native laws are present:

- two concrete composite windows publish and render different cameras, keep the document and app-config packs unchanged, export two exact config packs, reload both packs into another app, and drive both apps to terminal-empty close;
- two concrete composite windows isolate engagement input, document replacement resets transient state and generation, replacement cancels an unpublished retained transient mutation that captured the old authority, and close drains the remaining owners.

## File ledger

- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🟦️.ts`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔗️.graphql`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧫️fixtures/🔣️.json`
- deleted `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs`
- deleted all three files below `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/📷️set-camera`
- deleted all three files below `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/💬️set-engagement-input`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧬️schema/{🦀️.rs,🔣️.json,🟦️.ts,🛰️.proto,🔗️.graphql}`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧫️fixtures/🔣️.json`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️set-camera/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️set-camera-zoom/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💬️engagement-input/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️engagement-submit/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖊️ink-apply-events/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗃️set-active-example/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧭️navigator/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️testkit/🦀️.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`

## Retained-capacity admission correction

Note transient preflight now charges engagement_input.capacity() because direct transfer preserves the allocation. Its regression gives a one-character String capacity greater than the Store one-item maximum, requires preparation to return the exact String pointer and capacity, and then retires the returned mutation to terminal emptiness with one-item/one-byte grants. The accepted multi-kilobyte input retirement law remains. Native execution is pending in the parent-owned queue.

## Validation evidence

| Command | Result | Evidence |
| --- | --- | --- |
| `CARGO_INCREMENTAL=0 bun nx run @semio-tech/note-note-rs:check --skip-nx-cache` through ticket `cargo-trinity` | RED before Note compilation: shared OS kernel generic bounds failed in `close_assembly_publication` and `TransientStore` | `🗑️generated/note-window-check-1.log` |
| Same Note check after shared-bound repairs | RED in shared plugin exports and a stale scalar test-size assertion | `🗑️generated/note-window-check-2.log` |
| Same Note check after shared plugin repairs | RED after reaching Note: fully-qualified value-derive imports were required | `🗑️generated/note-window-check-3.log` |
| Same Note check after the derive repair and shared transfer-factory migration | NO RESULT; session 22619 and PID 16786 disappeared while the log still contained only Nx running heartbeats | 🗑️generated/note-window-check-4.log |

The neutral fixture include now names its actual window/🧫️fixtures owner. The orphaned check is not evidence of compilation or runtime behavior. The parent owns the next full abstraction-ownership-validation:note-document-contract-native registered-app target and the Puzzle 2D, Puzzle 3D, and Puzzle 5D runtime reruns.

The ticket and parent goal remain open.
