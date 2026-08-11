# P2-FG3 Independent Verification Report

Independent verification of FG3's 4 fan-out agents: gltf/2.0, pdf/1.4+1.7 (combined agent, 2
standards), ply/1.0, svg/1.1 — 5 standards total. Every claim below was re-checked against disk
and re-run, not taken from the self-reports.

## Summary verdict

All 4 self-reports are accurate. No binary-frame shortfall recurred anywhere in FG3 (the #1 risk
item after FG1's own shortfall). All 5 standards' `DiffCodec`/`OpBinary` bodies were read directly
and confirmed genuinely real (or, for pdf/1.4, genuinely already-real via the derive path, correctly
left unmodified). All grammar/protocol files use the real dialect header and model the correct side
per each artifact's hybrid classification. All 5 known authoring pitfalls were checked for
recurrence — none found. Registration is complete (5 roles × 4 standards + 5 roles × 2 pdf
standards = 30 roles total). Fixtures are real. Full crate suite is clean: **1806 passed, 0
failed, 1 ignored** — better than any individual agent's own concurrent-churn-affected run, since
all sibling FG3 work has since landed.

## Per-standard results

### gltf/2.0

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::gltf"` → **49 passed, 0 failed** (matches
  self-report exactly).
- `real_dialect_confirmed`: yes. Snapshot grammar is a real RFC8259 JSON grammar (json pilot's own
  depth) + real glTF top-level member set; snapshot protocol is the real `.glb` binary container
  (12-byte header folded into `framing magic` 8 bytes + `total_length` field, length-first
  tag-dispatched `repeat` of JSON/BIN chunks) — read directly, both headers use `dialect
  grammar`/`dialect protocol` correctly.
- `binary_frame_confirmed`: yes. Read `GltfDiff::encode_diff`/`decode_diff`
  (🔺️diff/component.rs:3196-3268) — genuine `dsl::ByteWriter`/`ByteReader`, varints, length-prefixed
  blobs, tri-state flags, zero `print_diff().into_bytes()` residue. Read
  `GltfMutation::encode_op`/`decode_op` (🧬️mutations/component.rs:455-570) — genuine `format u8 +
  tag u8 + variant payload` frame reusing the diff module's `write_bin_*` primitives, zero
  `print_op().into_bytes()` residue.
- `fixtures_real`: yes. `example.dsl.semio` has the real `semio stdio.gltf.dsl v1` preamble
  followed by genuine JSON; `example.pack.semio` exists.
- `registration_confirmed`: yes. 5 `LanguageRole` entries confirmed by direct grep
  (Document/Ops/Diff/Pack/Spr).
- `pitfalls_avoided`: yes. No bare-paren productions, no hand-rolled `{INT|IDENT}*` hex, no
  reserved-keyword production names, no multi-line-wrapped productions, no `Ref(...)` in any
  `.protocol.semio` file.
- Notes: the GLB `framing magic` folds the mandatory `version=2` constant into the 8-byte magic
  comparison (`0x676C544602000000`) — a documented, justified design choice given `Framing::Magic`'s
  fixed-8-byte mechanism constraint, consistent with gif87a's own precedent for the same wall.

### pdf/1.4 + pdf/1.7 (combined agent)

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::pdf"` → **157 passed, 0 failed** (matches
  self-report exactly; baseline was 142/0 before this wave).
- `real_dialect_confirmed`: yes, with one documented and justified deviation from the brief's own
  classification. The brief said "grammar models the real COS text syntax" but a direct read of
  both standards' `impl store::ArtifactDsl for PdfSnapshot` confirms `print_dsl` hex-encodes the raw
  `encode_pdf(self)` byte output verbatim (matching `binary/raw`'s own precedent), not a
  reserialized COS text form — so the snapshot grammar is honestly `document = artifact-mark
  hex-body` for both standards. The REAL COS structure instead lives in the pack/protocol facet,
  which is correct: 1.7's protocol file models the real `%PDF-` header magic (5 bytes, `header
  fixed 5`) plus a `backward` block anchored on `startxre` (`0x7374617274787265`, an 8-byte
  truncation of the real 9-byte `startxref` keyword, justified by the `backward` magic literal's own
  8-byte cap) locating the xref/trailer region — read directly and confirmed to match.
- `binary_frame_confirmed`: yes, in both directions.
  - 1.7 (hand-rolled, `#[derive(dsl::DslDiff)]`/`#[derive(dsl::DslOps)]` confirmed rejected): read
    `PdfDiff::encode_diff`/`decode_diff` (🔺️diff/component.rs:1977-2003) — genuine `format u8 | flags
    u8 | [declared_version][info][pages][objects][trailer]` frame with real recursive
    `enc_*_bin`/`dec_*_bin` primitives, zero text-as-bytes. Read `PdfMutation::encode_op`/`decode_op`
    (🧬️mutations/component.rs:443-513+) — genuine `format u8 | tag u8 | variant payload` frame,
    same real primitives reused.
  - 1.4 (derive path, correctly left unmodified per the brief's own instruction that "upgrading an
    already-real codec is a no-op"): read `PdfMutation`'s `impl protocol::OpBinary` — calls
    `dsl::variants_binary::encode_op`/`decode_op` (the real framework-generic derive-path binary
    encoder), confirmed NOT a text shortcut. Read `PdfDiff` — `#[derive(dsl::DslDiff)]` with no
    hand-written `DiffCodec` body at all (derive-generated, routes through the real `.spk`
    container per the recipe's §2.4) — confirmed genuinely real, correctly untouched.
- `fixtures_real`: yes, for both standards (1.4's rewritten, 1.7's newly created in a per-standard
  folder mirroring gif 87a/89a's own precedent). Preambles `semio stdio.pdf.dsl v1` /
  `semio stdio.pdf.1.7.dsl v1` confirmed present, followed by genuine hex-encoded real PDF bytes.
- `registration_confirmed`: yes. 10 roles total confirmed by direct grep (5 per standard ×2).
- `pitfalls_avoided`: yes, same checks as gltf, clean.
- **Restraint check (item 7, pdf-specific)**: confirmed. The 1.7 protocol file's own doc comment
  and structure explicitly stop at the `backward` block's opaque `tail bytes` — no attempt at
  xref `/Prev`-chain resolution, hybrid-stream handling, or arbitrary backward jumps to resolve
  individual indirect objects inside the protocol walker. That logic correctly stays Rust-side
  (`decode_pdf`'s own `Resolver`/`build_xref`), exactly per the M2/PDF-1.7 exclusion this ticket's
  own recipe §6 pre-approves.

### ply/1.0

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::ply"` → **36 passed, 0 failed** (matches
  self-report exactly).
- `real_dialect_confirmed`: yes. Snapshot grammar is the real ASCII header syntax (magic line,
  format line with all 3 real format-kind alternatives, comment/element/property declarations,
  `end_header`, then a flat token-vocabulary body) — read directly, matches
  `parse_header_text`/`header_text` production-for-production. Snapshot protocol is `framing
  record` + real `header fixed 4 { field magic fixed 4 }` + one opaque `chain body bytes` — a
  documented, pre-approved M2 exclusion (the recipe's own §6 names `ply` explicitly), verified
  live by the agent (not assumed) that no primitive can locate the unboundedly-long
  `end_header\n` terminator or let a grammar-parsed value parameterize a protocol field.
- `binary_frame_confirmed`: yes. Read `PlyDiff::encode_diff`/`decode_diff`
  (🔺️diff/component.rs:1169-1190) — genuine `dsl::ByteWriter` flag-per-field frame
  (`format`/`comments`/`elements`), zero text-as-bytes. Read `PlyMutation::encode_op`/`decode_op`
  (🧬️mutations/component.rs:259-320+) — genuine `format u8 + tag u8` header + real per-variant
  payload encoding, zero text-as-bytes.
- `fixtures_real`: yes. `example.dsl.semio` has the real `semio stdio.ply.dsl v1` preamble
  followed by a genuine ASCII ply document (magic, format, comment, vertex/face elements incl. a
  list property, end_header, 3 real data rows). `example.pack.semio` is new, genuine SEMIO-envelope
  wrapped bytes.
- `registration_confirmed`: yes. 5 roles confirmed by direct grep.
- `pitfalls_avoided`: yes, same checks, clean.
- Two additional real, in-scope Rust bugfixes were independently confirmed on disk:
  `encode_pack_with` now calls `encode_ply_with_format(self, self.format)` (was previously
  ascii-forcing, silently discarding a binary-format snapshot's real format on every Pack
  round-trip — confirmed at 📸️snapshot/component.rs:169-186); `header_text`'s signature now takes
  `comments: &[String]` (confirmed at ⚙️engine/component.rs:301), closing the previously-documented
  one-directional comment-dropping gap.

### svg/1.1

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::svg::"` → **69 passed, 0 failed** (matches
  self-report exactly).
- `real_dialect_confirmed`: yes. Snapshot grammar is a real SVG(=XML) document grammar, read and
  compared directly against xml's own just-landed FG1 grammar — the header directives
  (`comment none`, `string double raw`, `string single raw`), the `artifact-mark`/`document`
  production shape, the `declaration`/`doctype`/`element` productions, and even the surrounding
  doc-comment rationale are structurally parallel to xml's file, confirming item 8 of the brief
  (svg genuinely mirrors xml's shape rather than diverging arbitrarily). Snapshot protocol is
  `framing record` + `chain payload utf8` (text-native, matches xml's/json's own honest boundary).
- `binary_frame_confirmed`: yes. Read `SvgDiff::encode_diff`/`decode_diff`
  (🔺️diff/component.rs:1124-1166) — genuine `format u8 | flags u8 |
  [declaration][doctype][root]` frame with real recursive `enc_node_diff_bin`/`dec_node_diff_bin`,
  zero text-as-bytes. Read `SvgMutation::encode_op`/`decode_op`
  (🧬️mutations/component.rs:547-620+) — genuine `format u8 | tag u8 | variant payload` frame, zero
  text-as-bytes.
- `fixtures_real`: yes. `example.dsl.semio` has the real `semio stdio.svg.dsl v1` preamble
  followed by a genuine `<?xml ...?><!DOCTYPE svg><svg ...>` document. `example.pack.semio` is new.
- `registration_confirmed`: yes. 5 roles confirmed by direct grep.
- `pitfalls_avoided`: yes, same checks, clean.
- **svg-specific check (item 8)**: `SvgDiff` is confirmed as its own distinct struct (not a type
  alias or reuse of `XmlDiff`), while genuinely reusing `XmlAttr`/`XmlDeclaration`/`XmlNode` as the
  underlying node model — matching Phase 1's documented design ("svg declares its own diff types,
  reusing xml's node model"), not a divergence.

## Cross-cutting checks (all 5 standards together)

- **Grep for `Ref(...)` in any `.protocol.semio` file across all 4 artifacts** → zero hits. No
  standard attempted the known-broken `Prim::Ref` recursion.
- **Grep for bare-paren macro-call-shaped productions, hand-rolled `{INT|IDENT}*` hex, and
  reserved-keyword-named productions across all 30 grammar files** → zero real hits (only
  in-comment mentions explaining why these pitfalls were avoided).
- **Full crate suite** (`cargo test -p semio-s-plugin-stdio --lib`), run once by me at the end →
  **1806 passed, 0 failed, 1 ignored, finished in 8.71s**. This exactly matches gltf's own
  concurrent-clean run and is strictly better than pdf's/ply's/svg's own individual runs, which
  each hit transient concurrent-churn failures from sibling FG3 sessions still in flight at the
  time — all now resolved.
- **Policy check** (`bun run ./📜️script.ts policy`) → ran the full repo-wide policy sweep. It
  reported 21,509 high-priority breaches repo-wide across 25 rule categories (all pre-existing,
  large-scale, unrelated churn from other concurrent artifact-type work per the ticket's own
  standing warning). Grepped the entire breach output for any mention of `🧊️gltf`, `📄️pdf`,
  `☁️ply`, or `🎨️svg` paths — **zero hits across all 4 artifacts**, confirming none of this wave's
  own files appear in any breach category, matching every self-report's own "zero new breaches"
  claim.

## Mechanism gaps reported by the 4 agents — spot-checked, all consistent with the recipe's own
consolidated table

`protocol-prim-ref-recursion`, `protocol-array-of-records`, `protocol-repeat-length-not-named`
(gltf), `register-schema-spec-needs-recordspec`, `protocol-framing-magic-fixed-8-bytes`,
`protocol-backward-magic-fixed-8-bytes`, `protocol-cos-text-not-byte-fixed-width` (pdf 1.7),
`ply-ascii-header-schema-external` (pre-approved M2 exclusion, ply), `svg-path-data-implicit-and-
flag-squeeze`, `svg-style-value-untyped` (svg) — all either already present in the recipe's
consolidated table or are the SAME root cause under a new artifact-specific name (e.g.
`ply-ascii-header-schema-external` is the same family as PDF 1.7's `/W`-array gap, both correctly
cross-referenced by their own agents). No agent silently worked around a wall; every fallback was
documented and independently verified against the real `Prim`/`walk_protocol` mechanism.

## Conclusion

No dispatch of a targeted fix wave is needed. All 5 standards genuinely landed real
binary-frame-upgraded `DiffCodec`/`OpBinary`, real dialect grammar/protocol files on the correct
side of each artifact's hybrid classification, real fixtures, full 5-role (×2 for pdf)
registration, and 0-failure test suites both individually and in the full crate. The pdf agent's
restraint around the M2 xref/object-graph exclusion, and the svg agent's structural fidelity to
xml's own FG1 grammar, were both independently confirmed rather than assumed.
