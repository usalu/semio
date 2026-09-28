# Full Blend Mode Inspector

The inspector now offers all sixteen modes already supported by the canvas, SVG, and PDF compositing implementations: normal, multiply, screen, overlay, darken, lighten, colorDodge, colorBurn, hardLight, softLight, difference, exclusion, hue, saturation, color, and luminosity. Each choice has English and German labels. Field admission accepts the same canonical values and rejects CSS spellings, unknown modes, and empty input.

The language-neutral field-patch corpus grows from 50 to 69 cases, covering every blend mode and invalid spellings. The existing independent Ajv audit consumes that same corpus. Native tests were added for every common and mixed choice, selection locks, accessible names, localized option text, semantic mutation, exact undo, and preservation of unrelated document fields.

While reviewing the native tests, three isolation-era assumptions were corrected: the semantic catalog now has eighteen entries; set-group-isolation targets the group entity; and isolation field fixtures must use a group rather than the shape fixture used for general fields.

## Verification

- RED: Nx Draw TypeScript test session 79326 exited 1. The independent field-schema oracle rejected colorDodge, demonstrating the incomplete inspector contract. Log: `🗑️generated/tests-blend-inspector-red.txt`.
- GREEN: Nx Draw TypeScript test session 89172 exited 0: 273 passed, one skipped, zero failed, 148561 assertions in 28 files. The independent Ajv audit passed all 69 field-patch cases, and the publication audit passed all 36 commands. Log: `🗑️generated/tests-blend-inspector.txt`.
- The skipped test remains the PDF raster oracle: actual native PDFs have not yet been emitted by the pending native run.
- Native session 17529 is still running. It started before these inspector changes. Its terminal test roster must be inspected to establish source coverage; no native pass is claimed here. If this version did not include the newly added tests and isolation test fixes, run the updated native suite after it terminates.
- Component build session 93519 is still running and predates these edits. No current browser interaction verification is claimed.

## Files Changed in This Increment

All implementation paths are relative to `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/`:

- `✏️editor/📌️panels/🔍️properties/🦀️.rs`
- `✏️editor/📌️panels/🔍️properties/🧪️tests/🎛️selection/🦀️.rs`
- `✏️editor/🗣️terminology/🦀️.rs`
- `🧬️schema/🧬️mutations/🦀️.rs`
- `🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/🧬️mutations/🧫️fixtures/🎛️field-patch/🔣️.json`
- `🧬️schema/🧬️mutations/🧫️fixtures/🎛️field-patch/🧬️schema/🔣️.json`

This note and `📋️acceptance.md` also changed. The ticket and overarching goal remain active. This increment does not establish end-user completeness. PNG fidelity, import breadth, typography, selection/path workflows, progress/cancellation, current component activation, and browser acceptance remain outstanding. A future vocabulary consolidation should also reject unknown blend modes at authored model/semantic-mutation boundaries, whose string representation remains broader than the inspector contract.
