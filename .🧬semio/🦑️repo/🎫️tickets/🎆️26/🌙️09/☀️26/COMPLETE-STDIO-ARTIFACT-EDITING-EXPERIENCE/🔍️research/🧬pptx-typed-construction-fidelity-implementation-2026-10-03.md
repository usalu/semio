# PPTX Typed Construction Fidelity Implementation

Date: 2026-10-03

## Boundary

The typed PPTX builder now keeps `PptxSnapshot` as its only authority. Imported packages are extended in place at their canonical PresentationML/XML and OPC boundaries. The derived `PptxPresentation` view is used only to validate `from_snapshot`; it is no longer cached and rebuilt through `build_minimal_pptx` after each constructor call.

`build_minimal_pptx` remains the initial constructor only when OPC parts, content types, relationship owners, archive comment, and retained XML parts are all empty. Non-empty authorities must expose a valid presentation part, namespace bindings, and a retained slide layout or return a typed builder diagnostic without replacement synthesis.

## Append contract

- Paragraph construction resolves the active canonical slide, appends one DrawingML paragraph to the last non-placeholder text box, or appends a new text box with the lowest unused shape identity.
- Slide construction appends a canonical `sldId`, a new slide XML part, its content-type override, an owner-local presentation relationship, and a slide-layout relationship.
- New part, slide, shape, and relationship identities use the lowest unused valid identity. Existing vectors and XML children retain their order and identity.
- PresentationML, DrawingML, and office-relationship namespace URIs and prefixes are derived from the exact insertion scope. Strict URIs, custom prefixes, and local prefix shadowing are retained. A presentation root does not need an unused DrawingML declaration; a new slide takes its PresentationML/DrawingML vocabulary from the last retained slide, or the retained layout for a zero-slide package.
- Existing OPC members, archive comment, content types, relationship groups, XML attributes, dimensions, notes, theme, custom parts, and unmodeled subtrees are not regenerated.

## Neutral witness and independent oracle

The schema-first fixture `🏗️typed-construction-fidelity` handcrafts a Strict package with custom prefixes, local PresentationML/relationship shadowing at `sldIdLst`, local DrawingML shadowing at `txBody`, non-sequential relationship and slide identities, a 16:9 presentation size, formatted text, an unmodeled shape subtree, notes, theme, and custom XML. Its law asserts exact structural prefixes for retained XML and relationship vectors, then saves the result and checks it independently with the third-party `zip` and `quick_xml` crates before reopening it with the production decoder.

The same law also constructs from genuinely empty authority, appends one slide and paragraph, saves, and reopens it. This pins minimal synthesis to the empty boundary.

## Execution evidence

- Initial execution: `🗑️generated/pptx-typed-construction-red-1.log`. The new law was present, but compilation stopped in concurrent Store inverse API drift before the PPTX witness ran.
- First runnable fidelity execution: `🗑️generated/pptx-typed-construction-focused-2.log`. Nx cache disabled; 1 passed, 132 skipped. This covered imported package preservation, Strict/custom-prefix append behavior, independent ZIP/XML inspection, and production decoder reopen.
- Expanded execution with the empty-authority assertion: `🗑️generated/pptx-typed-construction-focused-3.log`. Compilation reached the shared UI runtime and stopped before PPTX linking at `size_of::<ui_contract::Component>() <= SURFACE_COMPONENT_COPY_WORK_BYTES`. This is an external component-footprint blocker; no PPTX failure was emitted.
- Fresh focused native execution after the compact UI publication revision landed: `🗑️generated/pptx-typed-construction-focused-4.log`. Command: `NX_DAEMON=false CARGO_TARGET_DIR='<ticket>/🗑️generated/pptx-construction-target' bun nx run @semio-tech/stdio-pptx-rs:test --skip-nx-cache --args='quick --lib typed_construction_appends_to_imported_canonical_package_without_rebuilding_it --no-fail-fast'`. Result: 1 passed, 132 skipped; Nx cache disabled.
- Fresh full component-enabled native execution: `🗑️generated/pptx-typed-construction-native-full-1.log`. Command: `NX_DAEMON=false CARGO_TARGET_DIR='<ticket>/🗑️generated/pptx-construction-target' bun nx run @semio-tech/stdio-pptx-rs:test --skip-nx-cache --args='quick --lib --no-fail-fast'`. The registered target enabled `component-app-assembly`. Result: 133 passed, 0 failed, 0 skipped; Nx cache disabled.
- Registered TypeScript/SQLite execution after schema integration repair and neutral fixture mounting: `🗑️generated/pptx-typed-construction-typescript-sql-4.log`. Command: `NX_DAEMON=false bun nx run @semio-tech/stdio-pptx:test --skip-nx-cache`. Result: 16 passed, 0 failed, 118 assertions; Nx cache disabled. The suite validates the neutral corpus with Ajv 2020-12 and covers physical authority, independent SQL mutation, exact profile, ownership, cancellation, signed64, and history through Bun SQLite.

## Source inventory

- `base/🧬️schema/🏗️construction/🦀️.rs`: canonical in-place append implementation.
- `base/🧬️schema/🦀️.rs`: single-authority builder wiring and typed diagnostics.
- `base/🧫️fixtures/🏗️typed-construction-fidelity/🔣️.json`: neutral fidelity corpus.
- `base/🧫️fixtures/🏗️typed-construction-fidelity/🧬️schema/🔣️.json`: fixture schema.
- `base/🧬️schema/🧪️tests/🔬️unit/🦀️.rs`: canonical and independent-oracle law.
- `base/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts`: registered Ajv validation of the neutral fidelity corpus beside the existing SQLite laws.
- Three existing PPTX mutation test sites were updated for the current infallible `PptxDiff::inverse` API without weakening their apply/restore assertions.
- Shared XML `snapshot/🔣️.json` was aligned with its canonical codecs: `prologPosition` is a canonical unsigned decimal string and external identifier fields are `systemId`/`publicId`. The registered Ajv law exposed each stale declaration before the final 16/16 run.

## Remaining scope

The scoped canonical builder supports slide and paragraph construction with the existing basic run-formatting surface. It does not yet expose typed constructors for every PresentationML object or formatting property. The save law proves logical OPC/XML preservation, exact retained custom-part bytes, archive comment retention, independent ZIP/XML readability, and production reopen; it does not require byte-identical ZIP container metadata because appending members necessarily rewrites the ZIP container.
