# Generation3d controls and hover audit

Read-only production-path audit, 2026-11-20. Anchors are repository-relative. No production sources changed.

## Immediate candidate: inspector output hover

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs:185` renders actual selected-node output rows via `reflected_output` (lines 347–361), but those rows have no hover action. Hovering a geometry-bearing output in this real inspector consequently does not publish the channel target that would highlight its corresponding preview. Flow outline rows already publish `INTERACTION_HOVER_ACTION_ID` with `Trigger::HoverPreview` at `✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs:141`. Bind the output row root to the same graph-domain `widget@channel` target, preserving nested value windows; apply only when the declared output exists and use the existing clear-on-leave semantics. Test rendered output-row hover, clearing, and corresponding world marks.
- The connected input source button at inspector lines 230–238 selects its source but has no hover binding. It is a genuine navigation affordance; optionally reuse the same source channel target on hover so source preview and graph light up, but ensure pointer-leave does not erase a newer hover. Lower priority than output rows.

## Existing controls and scope

- Production generate form delegates at `✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs:44` to shared `✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🖼️semantic-ui/🦀️.rs:214`. Supported kinds are text, longText, number, slider, boolean, single, vector, note and image. Controls already use framework input/slider/toggle/select; vector is a column of individually labeled number fields (lines 262–276), not an arbitrary raw HTML control. Consider shared vector input only if it supports the form's arbitrary number of fields and per-axis action arguments; current XYZ contract alone is not proof of equivalence.
- Inspector scalar, checkbox, select, vector, slider and button are already framework semantic components (inspector lines 80–105, 257–279, 300–324); these are not remaining ad hoc widgets. Production generation tree delegates to shared semantic UI (`✏️editor/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs:47`).
- Editor and viewer world previews both derive hover from graph targets and upstream sources (edit preview lines 369, 420–423; viewer preview `👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs:304, 319–341`). Both publish channel-qualified interaction IDs; viewer's retained table does not itself prevent hover-only updates. No additional defect established there in this pass.

No tests run: inspection only.
