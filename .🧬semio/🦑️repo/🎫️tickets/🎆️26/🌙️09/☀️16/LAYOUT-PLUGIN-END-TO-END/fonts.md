# Layout Export Font Asset Boundary

2026-10-28. Removed the fifteen unconditional binary font includes from the engine font-source owner. These are runtime assets with existing role identities, package declarations, and native loader interfaces. The bounded 26,772-byte Layout Sans emergency font remains embedded; no opaque byte transfer, quota exemption, dependency, or manifest change was introduced.

## Runtime Contract

The sixteen export identities and order remain Layout Sans then TextFontRole::ALL. Count lookup and index zero require no I/O. The first requested additional source resolves an installed TextFontSet, otherwise the native TextFontFiles catalog on native targets, or the existing delivered package asset port on wasm. Neither headless path registers a UI text system. Native discovery preserves SEMIO_TEXT_FONTS and working/executable ancestor discovery.

Only successful native acquisition is cached: a missing or unreadable catalog can arrive later and be retried. The first successful export set is frozen independently from later UI registration, retaining original bytes and pointers throughout resumable text jobs. Failure does not invent substitute glyph data: source lookup returns None when requested assets remain unavailable. Native binaries need the existing font assets deployed or SEMIO_TEXT_FONTS configured, consistent with the established asset delivery architecture.

## Tests and Inspection

Rust tests consume the existing language-neutral sixteen-font catalog, check lengths, sfnt signature, out-of-range indices, repeated pointer identity, package role order and missing package refusal. Two isolated injected-cache laws check failed acquisition followed by success, and a first successful export set remaining stationary when another set becomes available. The existing OpenType.js + Node crypto + Ajv oracle validates this same neutral fixture and independent glyph coverage. Both the font catalog and paragraph publication oracles now pass the required FontWorkLedger to first-party admission, shaping and retirement calls.

Canonical readRustSourceClosureV1 read-only inventory via repo tools exec --light bun:

| Engine Selection | Inputs | Bytes | Active Lines |
|---|---:|---:|---:|
| Native normal | 20 | 249092 | 2210 |
| Native testkit | 20 | 249092 | 2225 |
| Native cfg(test), font laws | 22 | 256515 | 2261 |
| Scene companion cfg(test) | 23 | 218357 | 858 |
| Flow companion cfg(test), native 64-bit | 7 | 179614 | 104 |

Every measured selection fits the global 32 files / 262144 bytes / 4000 active lines without policy changes. Runtime engine cfg(test) now owns only the private font/cache laws. Scene/Drawing/paragraph and flow/projection/metadata tests compile in two domain companion libraries against the real engine public APIs, preserving original test files and fixtures adjacent to their topics. Narrow feature=testkit observations preserve former private-state assertions; ordinary runtime builds expose none. Successful source ownership no longer depends on compiling all fixture bytes into the runtime crate.

Both new companion names and the engine font-law package are registered in the artifact native test router. The nested Cargo workspace lists both companion manifests; the root agent owns their build-owner registrations. The engine gained its previously omitted direct semio-framework-value dependency, which its production code imports, and lost old test-only dependency/allocator ownership now belonging to the scene test companion.

Actual native compilation/tests remain pending at this report revision: the authorized native run stopped earlier at the unrelated Layout diff source boundary before compiler dispatch. The Rust agent owns the next native run after that owner repair; the TypeScript agent owns complete-suite revalidation. This report makes no compiler/test-pass claim and launches no duplicate graph command.

## Files

- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🔤️text-fonts/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🔤️text-fonts/🧪️tests/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🔤️text-fonts/🧪️tests/🔮️oracle.mjs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖼️canvas/🧵️paragraph-work/🧪️tests/🔮️oracle.mjs`
- Ticket `fonts-inventory.ts` and this report.

## Additional Test Ownership Files

The engine root/manifest and scene root changed only test ownership and the missing direct value dependency. Runtime paragraph, flow, projection, metadata, Drawing, segment, composition and Canvas modules no longer mount their relocated tests. Original scene unit, layer-order, paragraph, paragraph flow, Drawing graph, segment, composition, Canvas, flow, projection and metadata test sources are retained and mounted by the two companions. Companion roots are engine/🧪️tests/🎬️scene and engine/🧪️tests/🧵️flow, each with one test library and a 📦️packages/🦀️rust/Cargo.toml. New absolute paths top out at 166 characters. The artifact Cargo workspace and native test router contain the additive registrations.

Ticket inputs engine-test-boundaries.py and engine-test-support.py document the authored edits and remain retained. They are not migration/runtime tools and should not be rerun against the already edited tree.

## TypeScript Validation

The complete authorized TypeScript suite passed: 53 tests across 16 files, 313 assertions, 35.06 seconds. This includes both font-catalog and paragraph-publication independent oracle consumers after the explicit work-ledger fixes. The expanded strict TypeScript check also passed. Exact owned commands and outputs are recorded in typescript.md; native font/cache laws and the two new native test companions still await compiler dispatch past the other source-owner repairs.

## Portable Source Admission

The canonical inventory also evaluated macOS ARM64, Linux x86-64, Windows x86-64, and wasm32 cfg values, explicitly including target pointer width. Every selection retained the same input counts and bytes and stayed within all quotas. Native active lines match the table; wasm engine normal/testkit/test have 2169/2184/2211 active lines and flow test selection has 97. These are source-admission observations, not cross-target compiler success claims. Native companion execution is intentionally native (its test-only file readers and independent parsers use the host).
