# Original Drawing Export Engine

The core export planner now borrows an actual ChildAdmissionRead for each bounded Drawing composition turn. Production engine code is independent of the Draw editor registration. It never builds a synthetic provider or substitutes inline content.

## Outer API

- required_drawing_source() returns only the bounded active DrawingSourceAddress.
- original_work_demand() quotes the selected core event plus its original work milestone.
- advance_original_work(cx, Option<&ChildAdmissionRead>) advances one unit and returns Pending, Progress or Ready. It creates no preview/checkpoint publications; the outer retained producer owns those.
- step_with_drawing_read(cx, Option<&ChildAdmissionRead>) retains the existing InteractiveJob publication protocol. Ordinary step(cx) supports no-Drawing paths and explicitly refuses an active Drawing without its actual read. A numeric source change ends the current loop before reusing the borrowed read.
- drawing_consumers_terminal_is_empty() guards outer parent/child custody return after every retained plan and composition retirement.

The active-address metadata copy is charged before planning can advance a frame cursor. Both background and placed Drawing routes call current composition next_demand(snapshot,read,body) and advance(snapshot,read,grant). Paid error receipts flow into the same work ledger before refusal.

## Evidence

export-original-drawing-check.mjs passes strict TypeScript with 128 source files and zero diagnostics. Five neutral read-gate cases exercise actual registered readonly Draw qualification and actual vector preparation: 24,180 preparation turns, 312 cancellation cuts, 73,780 denial probes, two original reads. Portable custody is explicitly logical rather than Native allocator evidence.

Nine neutral PDF Drawing fixtures preserve background/placed geometry, red alpha stroke and width, blue fill, editable Plan/Title text, actual Draw text-field mutation admission, red sample identity, rotated placement and explicit missing resolution/compositing capability. The actual portable Drawing planner agrees with independent Skia paths/font metrics and Three matrices over 60 primitive events, two samples and one text edit. Native PDF/Canvas laws use original Store registry-issued typed SnapshotRead, its admitted RetainedCloneSource and the actual registered Draw admission/vector pair; no inline VectorScene codec remains in those laws. Native raw sample/compositing paths explicitly refuse when no actual owner is supplied.

Nine edited Native source files pass Rust grammar parsing only. No Rust compiler or Native test execution was available through the required repo command infrastructure. Native output, allocator, visibility and cross-crate type claims remain unverified.

## Outer Review

Root's export-tool-job/retained/drawing.rs uses matching engine call signatures. Its outer drawing_demand correctly adds provider_demand to inner.original_work_demand. The minimum item gate was flagged to Root: provider and route consume two items before the core milestone's two own items and any selected child event, so a three-item grant cannot advance core work. Root owns that outer change. Root's per-address source map and consumer-before-owner close order match this API.

Media export still needs a real original typed parent read and captured child-root custody from its host; that separate necessary infrastructure is being implemented next.

## Files

- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🏗️job-admission-and-close/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/⏭️advance-commit-and-codec/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/✅️validate-plan-and-svg/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📕️pdf-png-and-package-encoding/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📖️drawing-read/🔣️.json
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📖️drawing-read/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📖️drawing-read/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📖️drawing-read/🧪️tests.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📖️drawing-read/🧪️tests.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📖️drawing-read/🧪️pdf.json
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📖️drawing-read/🧪️pdf.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🗺️planning/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🔬️unit/✒️pdf-drawing-laws/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📦️packages/🦀️rust/Cargo.toml
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/LAYOUT-FEATURE-COMPLETE/export-original-drawing-check.mjs
