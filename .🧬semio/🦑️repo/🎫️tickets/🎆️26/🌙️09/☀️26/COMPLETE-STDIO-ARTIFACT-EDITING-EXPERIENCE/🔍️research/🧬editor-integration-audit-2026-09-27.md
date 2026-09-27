# Editor Integration Audit

Read-only source audit on 2026-09-27. No product files changed and no tests were run. This report excludes the in-flight TypeScript lossless-codec fix and the text-editor routing repair owned by other workers.

## Confirmed Defects

### P1 — Native Source Drafts Above 4 KiB Cannot Apply

`text_editor_apply_explicit_draft_into` serializes the complete draft action argument object and gives it to `checked_action_string_bytes` as one string ([EngineCanvas WGPU:6354](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6354)-[6359](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6359)). The WGPU action builder limits every individual string to 4 KiB, each action to 16 KiB, and the queue to 1 MiB ([action limits](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎬️action/🦀️.rs:19); [per-string refusal](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎬️action/🦀️.rs:193)). The shared snapshot-edit tool admits wire payloads through 16 MiB ([contract](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs:568)).

Reproduction: open a source-backed Details editor in WGPU, make a source draft whose encoded action arguments exceed 4 KiB (a normal formatted JSON document will do), then press Apply or Cmd/Ctrl+Enter. The call returns `StringCredits` before action publication. The document stays unchanged even though this size is valid for the shared artifact editor contract.

The source-draft control needs a transport that pages or otherwise carries bounded chunks while preserving one logical, atomic source replacement. Raising only the shared artifact cap does not fix the native renderer limit.

### P1 — WAV Advertises 32 MiB Source Replacement but the Shared Retained Tool Refuses It Above 16 MiB

The WAV editor admits `ReplaceSource` values up to 32 MiB ([WAV editor](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🦀️.rs:327)-[332](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🦀️.rs:332)). The common snapshot-edit execution contract permits 16 MiB and its retained wire factory explicitly rejects a larger declared input ([shared cap](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs:567)-[612](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs:612); [refusal](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs:649)-[660](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs:660)).

Reproduction: edit a WAV source snapshot with a serialized source between 16 MiB and 32 MiB, then apply. Editor admission succeeds; retained-tool construction rejects it with `snapshot edit rejects oversized wire or checkpoint owner`; no mutation is emitted. Media work is already addressing this mismatch.

## Remaining Uncertainty

- The WGPU action fault is propagated as `Result`; this audit did not run the runtime path to verify whether the host exposes `StringCredits` to the user or drops it. Either outcome leaves valid source drafts unapplied.
- React source-draft publication was not fully traced after the assigned TextWindowKit repair started, so this report does not claim a matching React cap.
- An accessibility Table editable-cell focus sequence looked vulnerable to stale-base conflict after an accessibility `Value` event, but it needs an isolated runtime test before it is actionable; it is intentionally not filed as a defect.
