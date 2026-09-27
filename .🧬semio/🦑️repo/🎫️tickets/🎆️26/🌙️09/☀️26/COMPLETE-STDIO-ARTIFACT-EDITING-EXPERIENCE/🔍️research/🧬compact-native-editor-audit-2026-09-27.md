# Compact Native Editor Audit — 2026-09-27

Read-only source audit of the shared compact patch path, native media pilot
editors, Details, and WGPU/React editor surfaces. This report reflects the
working tree inspected on 2026-09-27. No test or build was run by this audit;
the concurrent Rust and catalogue builds remain their owners' evidence.

## P1 — Accessibility `Value` publishes once, then `Blur` can publish the stale draft again

**Affected files**

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1086-1104`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2105-2150`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:4159-4165`

The accessibility `Value` handler stores the new draft and immediately starts
a retained text action using the displayed cell value as its base. It leaves the
focused record unchanged. A following `Blur` invokes the normal focus commit,
which uses the same original `base` and `draft`. Until the retained scene
echo is rendered, the cell still contains that original base, so
`table_editable_text_commit_owned` starts a second retained action.

**Reproduction**

1. Use an accessibility client to focus a TableWindowKit editable cell whose
   current value is `A`.
2. Send `Value("B")`, then `Blur` before the retained update changes the
   rendered row.
3. Two retained publications for `B` are attempted. Depending on the action
   bus's concurrent-action handling, the second attempt is duplicate work or a
   user-visible action fault.

Treat a successful `Value` publication as pending/committed for that focus so
`Blur` cannot resubmit it, or defer publication to the usual blur/Enter
commit path. A pending flag must clear only on the echoed result, refusal, or
explicit cancellation; merely overwriting the focus base would turn the
pre-echo blur into a false conflict.

## P1 — MP4 accepts a sample-data edit but restores the old data

**Affected file**

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/✏️editor/🦀️.rs:125-138`

`mp4Editor_bounded_edit` takes every `sample.data` out of its working
snapshot, applies a generic edit to the empty-data snapshot, then unconditionally
restores the old vectors. A generic edit of `/tracks/0/samples/0/data` is
therefore accepted and produces a `SetSnapshot` whose data is unchanged.

**Reproduction**

For a small MP4 snapshot containing one sample with `data: [7]`, submit
`SetValue { path: "/tracks/0/samples/0/data", value: [8] }`. The generic
edit succeeds against the temporarily empty payload; the restore loop writes
`[7]` back before emission. The action reports success but the document
remains `[7]`.

The prepared pilot regression test and the planned
`prepare_snapshot_patch → apply_snapshot_patch_for_dialect` replacement are
the appropriate resolution. Do not ship the strip-and-restore path while the
direct data address remains routable.

## Rollout risk — shared editor route still projects and publishes full snapshots

**Affected files**

- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs:864-915`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🩹️patch/🦀️.rs:119-287`

The new compact patch helpers are not yet invoked by the shared retained
snapshot-edit reduction. The reduction still calls
`apply_snapshot_edit_for_dialect`, while the normal emission path makes a
`SetSnapshot` and admits up to 16 MiB. As a result a small metadata change in
a large document still projects and sends the complete snapshot, bypassing the
patch's 1 MiB cap, direct native path operations, and compact inverse.

This is a known in-progress rollout gap, not a claim about the planned native
pilot integration. Keep the media pilots unmounted until the compact route is
the route registered by the shared reducer and its wrapped retained command
uses the exact 1 MiB byte admission.

## Checks completed without a defect

- The closed Source section is lazy on first Details render:
  `TreeWindows::sliced` returns a zero-row slice for a default-closed
  section at
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:6526-6544`.
  Its source callback therefore does not project the full snapshot until the
  user opens Source. This audit does not report a first-paint Details
  projection defect.
- React's explicit-draft helper now recomputes conflict state from the current
  persisted buffer at
  `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🟦️.tsx:345-347`.
  The previously observed conflict latch is not present in this snapshot.
- Static inspection found the Rust compact move preparation normalizes its
  insertion path after applying the remove
  (`…/editing/🩹️patch/🦀️.rs:129-165`) and captures inverse values against
  sequential state before reversing edits (`270-287`). Runtime and
  cross-language conformance remain unverified by this audit.
