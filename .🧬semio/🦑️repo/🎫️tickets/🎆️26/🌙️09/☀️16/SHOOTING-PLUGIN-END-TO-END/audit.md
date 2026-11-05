# Shooting Independent Contract Audit

Date: 2026-11-04. Read-only production-source review; no builds or tests run, to avoid duplicating the native and React agents.

## Confirmed Contract Mismatch

The addShot manifest description promises a shot framed by the current viewport camera. Its handler in editor/commands/shot ignores its config argument and creates a shot with camera_id: None. The effective-camera helper shooting_resolve_shot_camera explicitly resolves such a shot through the current config camera. Both icon preview and export supply that live camera. Consequently navigating after adding a shot changes its exported framing, and reopening in another local config can change it again. Capture the viewport camera in a saved camera entry and assign its id when creating the shot, or deliberately revise the public contract if live framing is intended. Add a language-agnostic create-shot, navigate, export-camera vector.

## Presence Publication Gap

The presence JSON/Rust schema and presence oracle agree that selectedShotIds and camera represent shot selection and live camera broadcast to peers. The executable editor registers the presence type, codec, mutation and retirement factories, but source search found no publication producer in the editor commands or retained-command implementation. SetCamera and LoadSavedCamera emit config mutations only; shot selection is also config state. This is a confirmed absence in the plugin producer path, but the final multi-user behavior requires checking generic shell behavior before treating it as a runtime failure. Generic implicit mirroring would have to be explicit and tested to establish this contract.

## Aligned Surfaces

Config JSON and Rust both contain default formats/shapes, selected shot ids, centerModel, fitRevision and camera. Presence JSON/Rust fields match. Their camera references point to the existing artifact schema id. The app-schema descriptor embeds the corresponding JSON, Rust, TypeScript, GraphQL and Proto leaves. The Rust production mutation bridge reads real descriptor aggregates for document, config and presence rather than a separate handwritten mutation list. The executable editor exposes both bundled examples.

Manifest actions, descriptions, options, windows and terminology consistently provide English and German labels. No browser accessibility interaction was executed by this audit. Generated default document labels such as Shot N and Camera N are English data labels; their localization policy should be distinguished from UI chrome.

## Existing Coverage and Limits

The repository has language-agnostic mutation feature files and handcrafted vectors plus Rust mutation/schema/editor contracts. The TypeScript package test script covers the three examples only (as recorded in exploration.md). Native and React agents own execution results. This audit makes no build-success or end-to-end runtime claim.

## Follow-up: Generic Presence Trace

The publication gap is confirmed for shot selection and live camera. Framework path: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/. Under that root, 📜️declaration/🪟️surface/✏️artifact-editor-trait/🦀️.rs:287 returns empty presence/transient vectors by default. 📜️declaration/🪟️surface/🖊️editor-app/🦀️.rs:178 merely wraps E::ephemeral. ShootingPlayApp does not override ephemeral. More decisively, 🧵️retained-command/🧵️retained-command-job/🦀️.rs:594 sets EphemeralEmit::default for Complete, while CompleteWithEphemeral carries the supplied presence mutations. Shooting retained commands never construct CompleteWithEphemeral.

🧵️instance/🔌️plugin-app-impl/🕹️ephemeral-interaction/🦀️.rs:67 reads the actual presence store; generation zero yields empty presence bytes. It separately broadcasts interaction selection through assemble_presence_interaction. Therefore asset picks can broadcast under the manifest selection.broadcast flag, but this does not publish ShootingPresence selectedShotIds or camera. There is no generic config-to-presence mirroring on this path.

Recommended retained-publication precedent: ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗃️io-and-retained-commands/🦀️.rs:90 returns CompleteWithEphemeral with ctx.presence_out. The shooting execution owner should emit its existing ReplacePresence mutation from the retained producer and declare Presence publication authority for camera/selection commands, preserving the other presence field. Test both changes reaching a nonzero generation and preserving unrelated presence data. Merely adding the ArtifactEditor::ephemeral override will not fix retained Complete.

## Follow-up: Fresh Changes

Inspected the currently visible shooting sun-enabled request correction and native icon-export pending-close correction. The latter changes the scene close pending branch to return Ok(false), matching retire signature and preserving scene ownership until close reports completion; no new lifecycle defect was identified in that branch. This is source review only. The TypeScript test script now adds parsing and sun tests, so the earlier three-example-only inventory describes the pre-change state.
