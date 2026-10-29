# Bounded Layout Vector Media Projection

2026-10-28. Structured media declares JSON. The export core now projects actual SVG into the schema-first 2d.layout object rather than placing raw XML in a JSON payload. Rust owns the shared schema, full media producer/consumer and registered handoff law.

## Current Contract and Retained Path

The exact projection is {schema:"2d.layout",source:{documentId:actualParent,frontierRevision:capturedCanonicalFrontierHex},svg:actualSvg}. Only genuine parent document identity and the explicitly named captured live frontier are serialized; no unsafe numeric IDs, guessed checkpoint hash or new authoring authority is invented.

The original SVG encoder still produces its raw rope. A media output credit paired with Svg selects projection; ordinary SVG downloads keep the raw route. The existing Base64-stage dispatch drives seven fixed projection phases (header, document id, separator, revision, SVG separator, SVG content, closing delimiter), tracked by one u8 plus the existing byte cursor. Progress reports layout.export.vector-projection. Encoded bytes use the existing encoded rope and existing final queue.

One turn reads at most1024 source bytes into a stack buffer and emits at most6144 escaped bytes. Quotes/backslashes are escaped; bytes0..31 become JSON Unicode escapes; original UTF-8 bytes remain in order across chunk boundaries. No whole SVG string is copied or parsed by the production wrapper. State/cursor writes, source reads, escape buffer writes and final rope writes consume the original OutputWork. Worst-copy usage remains below existing OUTPUT_COPY_BYTES16432 and at most4 work items per emission remain below the existing32-item gate. Existing rope prepare/append limits, cancellation checks, queue output credit, original push/seal and paid close ladder apply unchanged. Output overflow refuses through the existing bound.

## Actual Native Codec Law Ownership

Selected package semio-s-artifact-layout-layout-engine-export-unit-tests owns actual test original_media_vector_projection_escapes_and_cancels_with_original_writer_and_credit. It reads the shared language-neutral media fixture's three projectionCases (quotes/backslash/control/Unicode and UTF-8 crossing a1024-byte source frontier).

The strictly testkit phase selector begins after authored SVG entered the real original writer through existing append_output_fixture. It uses the real original caller and funds the phase mutation; it is not a production bypass. The law admits actual media credit under that caller, moves it into the job, and drives the actual advance_original_work path through encoded output, real queue credit and seal. It verifies actual JSON against the corpus, credit bytes, zero-effect/zero-heap deficient item/copy/depth grants,17000-byte bounded copy turns, cancellation after partial projection, and full actual job closure. The separate registered media law owned by Rust validates the complete producer/child identity route; this codec law does not claim to replace that proof.

## Independent Oracle and Source Admission

Actual managed vector-projection-oracle.ts executed fast-json-patch construction plus JSON serialization and strict Ajv2020 validation against Rust's authored 2d.layout schema for all3projectionCases. Result:3passed; fields documentId/frontierRevision. Native execution remains pending Rust's one sequenced gate; neither a zero-case run nor source-only evidence is counted as native success.

Canonical source-only inventory after all wrapper/testkit changes:
- Engine export production plus testkit22inputs/248915bytes/3035active lines.
- Actual selected export-unit companion cfg(test)10inputs/115405bytes/1207active lines before the subsequent explicit cancellation-cut assertions.
Both fit unchanged32/262144/4000 bounds. The old full engine-export cfg(test) root is not the selected companion: its preexisting all-tests closure is64inputs/1822074bytes and is not presented as a passing owner. No quota exception or unmounted law was added.

## Exact Source Packet

Under ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor:
- ⚙️engine/📤️export/🏗️job-admission-and-close/🦀️.rs
- ⚙️engine/📤️export/✅️validate-plan-and-svg/🦀️.rs
- ⚙️engine/📤️export/⏭️advance-commit-and-codec/🦀️.rs
- ⚙️engine/📤️export/🧪️testing/🦀️.rs
- ⚙️engine/📤️export/🧪️tests/🔬️unit/🦀️.rs
- 🎥️media-export-job-and-factories/🧪️fixture.json (additive boundary corpus row; schema/lifecycle/child entries remain Rust-owned)

Retained inputs vector-projection-inventory.ts and vector-projection-oracle.ts. No new module, queue, runtime external dependency or compiler invocation.

The independent audit required cancellation coverage to fail if the chosen cut was never reached. The law now explicitly asserts that each cut actually executes, that encoded output is nonempty and not yet committed there, and that cancelled advancement changes neither rope. Early completion can no longer silently skip the cancellation check. This is a test-only assertion change; native execution remains pending.
