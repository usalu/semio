The first-party Unicode16 UAX14 owner replaces the paragraph cursor's temporary whitespace/hyphen/CJK policy. Executed TypeScript proof: all 16,672 official Unicode16 LineBreakTest cases agree with both our runtime and the independent @cto.af/linebreak 3.0.5 implementation through a test-only first-party observation interface. The complete corpus is mounted in native tests too; native compilation and execution remain unverified because the current repo frontend cannot run this checkout's build commands.

Data is pinned to Unicode16.0.0 and UAX14 revision53 (2024-09-02). Line_Break and East_Asian_Width sources, URLs, SHA256 and byte lengths are recorded in the checked-in provenance. The 44,604-byte first-party binary contains 3,715 contiguous property intervals. SHA256: 5e193916bfd43d8b810bfc44cd715b7673471e54a6fe39d03e3a7853da807e53. Rebuilding from the retained official inputs produced byte-identical data. The complete unmodified official corpus SHA256 is 910759a611a479f37df4f535d2e64d7589be3c5ac5f7491cf1fb4fa4cdb211e9. Unicode license URL is part of the authority schema/provenance; the existing Unicode asset owner retains the Unicode License.txt.

Admission validates one range per unit, including version, extent, reserved bits, complete coverage, class and East Asian flags. The cursor reads one scalar, probes one ordered property record, computes one contextual record, or emits one boundary per unit. Context for quotes, spaces, numeric sequences, Brahmic syllables and RI pairing is retained as scalar metadata, so no boundary decision scans a source prefix. Source offsets are original UTF8 bytes. Native inputs are borrowed with captured pointer/extent checks. Original cells and unpublished outputs remain controlled-retirement owners; cancellation does not synchronously drop the accumulated source-sized buffers. Immutable source strings/data remain borrowed throughout.

Paragraphs now own the admitted break cursor and use its legal boundary list for wrapping. Breakpoint rewind follows the chosen wrap position. All mandatory separators (CRLF, CR, LF, VT, FF, NEL, LS, PS) avoid font lookup; bidi paragraph direction advances only on paragraph separators. The default UAX14 policy does not add locale-specific dictionary segmentation or discretionary hyphenation. Layout TextOwners admits the static line-break data in bounded ready_one turns and includes all new children/data in its existing controlled retirement. PDF contract/encoding/job fields and the shared display_one image branch were not modified in this phase.

Executed checks:

- 16,672 official cases, independent Unicode16 oracle, exact UTF8 positions and mandatory flags; all pass.
- Four portable boundary fixtures across grants1/7/4096; 14 malformed-authority/source/cancellation/grant/scalar refusals pass.
- Eight portable paragraph cases across three grants: 24 actual geometry runs, 24 OpenType+Sharp outline pixel comparisons, 40 partial cancellation/one-unit retirement checks pass. New fixtures cover all mandatory separators and opening punctuation followed by spaces.
- Authority/point JSON schemas validate the provenance and portable points. Asset regeneration is identical.
- Strict TypeScript runtime owner compilation reports zero owned errors. The dependency-only existing 2d ImportMeta.dir diagnostic remains under Node-only types.
- Eight touched native sources/tests parse with zero tree-sitter Rust grammar errors. This is syntax evidence, not rustc or native runtime proof. Native fixture, allocation witness and controlled close tests are mounted but unexecuted.

The framework 2d TS package declares exact test-only devDependency @cto.af/linebreak=3.0.5. No runtime dependency was added. The actual normal Bun test is mounted under the existing Unicode test root and imports a small first-party oracle adapter. The tool-API checker used the same adapter with integrity-pinned npm tarball inputs because dependency installation through the current repo frontend is unavailable. The root build owner must refresh the workspace lock using the current repo interface when available; this phase does not modify the shared root lock/build infrastructure.

Reproducible ticket inputs: line-break-assets.mjs builds the binary from LineBreak-16.0.0.txt/EastAsianWidth-16.0.0.txt; line-break-check.mjs executes actual source tests; line-break-oracle.mjs reads integrity-checked tarballs according to line-break-oracle.json and loads the first-party adapter. The downloaded linebreak-oracle-3.0.5.tgz, unicode-trie-oracle-3.2.6.tgz and official text/HTML documents are retained inputs, not generated logs. paragraph-check.mjs was updated to admit the line data.

Source files changed in this phase:

- 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/↩️line-break/📦️data/🔢️line-break.bin
- 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/↩️line-break/📦️data/🔣️provenance.json
- 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/↩️line-break/🟦️.ts
- 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/↩️line-break/🦀️.rs
- 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/↩️line-break/🧪️tests/📚️oracle/🟦️.ts
- 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/↩️line-break/🧪️tests/🟦️.ts
- 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/↩️line-break/🧪️tests/🦀️.rs
- 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/↩️line-break/🧪️tests/🧫️fixtures/LineBreakTest-16.0.0.txt
- 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/↩️line-break/🧪️tests/🧫️fixtures/🔣️.json
- 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/↩️line-break/🧬️schema/📍️point/🔣️.json
- 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/↩️line-break/🧬️schema/🔣️.json
- 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/🦀️.rs
- 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/🧪️tests/🟦️.ts
- 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️run/🦀️.rs
- 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️run/🟦️.ts
- 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️run/📄️paragraph/🦀️.rs
- 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️run/📄️paragraph/🟦️.ts
- 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️run/📄️paragraph/🧪️tests/🦀️.rs
- 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️run/📄️paragraph/🧪️tests/🟦️.ts
- 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️run/📄️paragraph/🧪️tests/🧫️fixtures/🔣️.json
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📄️paragraph-planning/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🎨️fidelity/🦀️.rs
- 🧰️framework/🔨️modules/◻️2d/📦️packages/🟦️typescript/package.json

The existing Fidelity rotation test now accepts both the original Glyph arm and retained Outline arm. ExportTextState::glyph already exposes the original borrowed ParagraphGlyph placement; glyph_cluster exposes the original borrowed Unicode cluster text/range/first-glyph flag. Parent owns semantic PDF text integration. Interactive native viewport/preview synchronous TextSystem builders are outside this export integration and still need their own controlled paragraph scheduling if required for the full application goal.
