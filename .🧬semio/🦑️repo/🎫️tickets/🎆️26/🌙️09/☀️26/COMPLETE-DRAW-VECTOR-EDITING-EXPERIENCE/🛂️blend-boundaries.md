# Blend Mode Boundary Validation

The prior increment exposed all sixteen blend modes in the inspector. This increment prevents unsupported mode strings from entering through the TypeScript document and sparse-patch parsers, the TypeScript blend diff builder, Rust semantic blend mutations, Rust sparse blend diffs, and the retained Rust mutation candidate.

Rust checks reuse the existing `DRAWING_BLEND_MODES` vocabulary instead of maintaining another inspector-specific match. TypeScript now exports the corresponding owned `BlendMode` type, vocabulary, and parser. The canonical mutation payload and sparse-patch TypeScript types use that type. Schema contracts for documents, aggregate mutations, leaf mutations, and patches now enumerate the supported modes. A null sparse blend patch remains untouched, matching Rust Option semantics.

The TypeScript layer parser validates nested group children iteratively before making one structured clone. Nested blend modes, fill rules, group isolation values, child arrays, and layer kinds therefore receive the same checks as roots. This is still a synchronous parser; a resumable document import remains necessary for large inputs.

## Verification

- Session 56195 exited 1: the newly added boundary test reproduced acceptance of a nested shape with `blendMode: garbage`. This is the intended RED failure, not a build failure. Log: `🗑️generated/tests-blend-boundaries-red.txt`.
- Session 35980 exited 0 after the runtime changes: 274 passing tests, one skip, 148666 assertions.
- Session 12112 exited 0 after adding sparse-null and independent patch-schema checks: 274 passing tests, one skip, 148689 assertions across 28 files. The independent Ajv field audit passed all 69 cases and the publication audit passed all 36 commands. Log: `🗑️generated/tests-blend-boundaries-final.txt`.
- New Rust tests exercise valid and invalid neutral field cases through semantic mutation application, raw diff application, and retained mutation candidates. Rejections must preserve the entire prior document and return retained candidate owners. They have not yet run.
- Native handle 17529 and component build handle 93519 were both polled and confirmed live after these edits. Process inspection also confirmed their live Cargo descendants (39054 native tests; 99402 component build), alongside other shared workspace compilations. No replacement or duplicate native build was launched.
- Native 17529 predates this increment; inspect its terminal roster to determine source coverage, then run updated tests if necessary. The native PDF output and subsequent independent PDF raster oracle remain pending.

## Remaining Boundary Work

The authored Rust layer model and decoded payloads still store blend modes as strings. These checks secure edit application; they do not prove every native deserialization or full layer replacement path rejects invalid modes. A typed authored representation or complete native intake validation is still required. The TypeScript group traversal covers groups as defined by the current schema. Full schema validation of every layer kind remains unfinished.

No current browser interaction validation is claimed. No overall editor-completeness claim is made. PNG/native paint fidelity, complete import, typography/images, selection and path gestures, responsive algorithms, and end-user acceptance remain part of the active goal.

## Changed Files

Relative to `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/`:

- `✳️any/🧬️schema/🔣️.json`
- `✳️any/🧬️schema/🟦️.ts`
- `✳️any/🧬️schema/🔺️diff/🔣️.json`
- `✳️any/🧬️schema/🔺️diff/🟦️.ts`
- `✳️any/🧬️schema/🔺️diff/🦀️.rs`
- `✳️any/🧬️schema/🧬️mutations/🔣️.json`
- `✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️kinds-catalog/🟦️.ts`
- `✳️any/🧬️schema/🧰️owned/🦀️.rs`
- `✳️any/🧬️schema/🧰️owned/🧪️tests/🔬️retained-mutation-authority/🦀️.rs`
- `🎨️style/🧬️schema/🧬️mutations/🌓️set-layer-blend-mode/🧬️schema/🔣️.json`
- `🎨️style/🧬️schema/🧬️mutations/🌓️set-layer-blend-mode/🦠️mutation/🟦️.ts`
- `🎨️style/🧬️schema/🧬️mutations/🌓️set-layer-blend-mode/🔺️diff/🟦️.ts`
- `🎨️style/🧬️schema/🧬️mutations/🌓️set-layer-blend-mode/🔺️diff/🦀️.rs`
- `🎨️style/🧬️schema/🧬️mutations/🌓️set-layer-blend-mode/🧪️tests/✖️normal-to-multiply/🦀️.rs`

This note and the ticket acceptance ledger also changed.
