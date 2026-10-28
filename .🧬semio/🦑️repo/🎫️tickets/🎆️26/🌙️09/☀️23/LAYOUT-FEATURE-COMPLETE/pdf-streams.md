# PDF Image, Paint And Unicode Streams

Layout PDF encoding consumes the same funded Fidelity path/image plan as SVG and PNG. It no longer calls synchronous proxy_png_rgb or formats a whole image as inline hex. RGB bytes and original alpha are emitted in bounded 4096-byte image and SMask XObject chunks. The per-image 65536-pixel shortcut was removed in favor of the authored total pixel/output and decoder capabilities. Page resource dictionaries advance one source item per turn. Geometry uses shared mapped path records, including rectangle rotation and distinct fill/stroke alpha via ExtGState. Streamed first_item page offsets replace repeated prefix scans. Missing-link placeholders remain renderable; a corrupt supplied ready proxy still faults. Placeholder marks are rotated and avoid accidental filled triangles.

The native text path emits invisible semantic text alongside exact glyph outlines, preserving positioned Unicode for extraction/import. Borrowed cluster text and glyph placement come from original paragraph owners. Text is scanned/emitted one scalar per turn. The complete CMap excludes surrogate scalar codes and emits 1026 bounded BMP/supplementary ranges, mapping four-byte scalar codes to valid UTF-16BE sequences. Font CMap objects and indirect stream lengths retain ordered object/xref ownership.

## Executed Evidence

Production TypeScript helper tests pass: 2 portable RGBA images with 4 grants each and 6 refusals match Sharp independent channel extraction. A shared 257×257 transparent image recipe (66049 pixels) matches Sharp RGB and alpha across bounded chunks. All 1026 emitted CMap records and 3 text cases match PDF.js 5.7.284 extraction, including German, CJK and supplementary Unicode endpoints. These use actual TypeScript sources and independently read PDF CMap witness files, not native Layout job execution. Strict checking the real test runner reaches 180 source files with 0 diagnostics.

All 16 changed native PDF source/test files parse without tree-sitter Rust grammar errors. Every static include target resolves. Reviewed paths are at most 182 UTF-16 code units, below the 256 limit. These checks do not prove native compilation.

The owned verification command is layout-pdf-streams in the Layout package verify router. Independent readers are hidden behind owned test-only oracles.mjs interfaces. Exact artifact-local devDependency versions are sharp 0.34.5, pdfjs-dist 5.7.284 @napi-rs/canvas 1.0.11 and iconv-lite 0.7.3; no external runtime API/dependency was added. Lock regeneration awaits the current repo package frontend.

## Native Laws And Limits

Native regression tests now inspect independent lopdf content operations, XObject RGB/SMask planes, text and object/stream/xref lengths, replacing byte-format and character-to-glyph-count assumptions. The new shared integration recipe covers a large alpha image, German/emoji story, rotated rectangle and separate fill/stroke alpha. These native laws are authored, not executed.

Lopdf 0.44.0 does not resolve custom Encoding streams through its high-level font factory. The test-only text lens removes Encoding from its own parsed document copy to select lopdf’s independent ToUnicode decoder. This limitation is explicit in its docstring. PDF.js separately consumes the actual custom Encoding plus ToUnicode witness without that lens. Neither observation is claimed as a native Layout job run.

Native PDF export/import, original-caller cancellation/checkpoint/retirement execution, full rendered visual QA and cross-platform runtime acceptance remain unverified. The authorized cached Windows repo CLI is unchanged and lacks the current run/cargo interface; see native-runner.md. No full PDF/spec or Layout completion claim is made.

## Editable Text Markers

The native PDF job now wraps each emitted semantic cluster in standard BDC/EMC marked content with tag SemioLayoutText. A schema-first text-stream owner defines UTF-16BE BOM family strings, u32 weight and italic properties. The font-family bytes advance one Unicode scalar per job step; borrowed glyph_style metadata comes from the shared retained paragraph owner. Applying the source ExtGState before invisible text preserves source fill alpha. The importer recognizes that explicit Layout scope, preserving positions and font details while retaining existing handling of other invisible PDF text. See interoperability-pdf-marker.md for the consumer policy.

Actual TypeScript source checks pass 3 portable marker/font cases and 6 refusals. Independent iconv-lite 0.7.3 agrees on font-family hex; PDF.js reads each marked-content PDF, observes the SemioLayoutText tag and extracts unchanged text. This verifies valid PDF marker syntax and bytes; PDF.js does not expose custom application properties through its public text API. Native independent lopdf laws inspect those properties and the complete job roundtrip law was authored by the interchange owner, but neither native path has run. The current 180-source strict test closure is clean.

Removed the per-turn page-index Vec rebuild from PDF/SVG/PNG planning; selection now resolves a scalar index. Existing all-page and single-page native laws retain the selection contract.
