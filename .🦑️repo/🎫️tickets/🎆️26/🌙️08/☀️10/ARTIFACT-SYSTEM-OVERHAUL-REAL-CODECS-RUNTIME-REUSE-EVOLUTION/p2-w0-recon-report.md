# Phase 2 W0 Recon Report

Design doc reference: `📖️phase2-design.md` (this ticket folder) — verbatim "PHASE 2 PROGRAM" section
of `~/.claude/plans/the-current-schemas-are-scalable-journal.md`. This report is the input to M1
dispatch.

All findings below are read-only; no `.rs`/`.semio` source file was modified. Citations are
`file:line` against the cited Rust file unless stated otherwise.

---

## 0. Current dialect ground truth (read directly, not delegated)

Read in full: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/🦀️component.rs` (2070 lines) and
the first ~260 lines of `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🔍️lexer/🦀️component.rs` directly;
the remainder of the lexer file was read by the text-native census agent (§1) and its findings are
folded in below.

**Grammar dialect**: header `dialect grammar`? / `grammar <id>` / `extension <ext>`? / `use <fragment>`*
/ `start <production>`, then `name = alt | alt` production lines. A `Symbol` is
`Literal("text")` / `Terminal(ALLCAPS)` / `Ref(lowername)` / `Macro(name(args))` / `Group{alt|alt}` /
postfix `? * +` (grammar/component.rs:56-66). **Structurally token-based, not character-based**: the
whole input is first run through the shared lexer (`crate::os_dsl::lex`) into a fixed token stream,
*then* productions match against tokens (grammar/component.rs:1261-1277) — there is no character
class/range/regex primitive at any level.

**Shared lexer alphabet** (🔍️lexer/🦀️component.rs, confirmed full-file by the text-native agent):
Newline, Whitespace (trivia), `#`-to-EOL Comment (trivia, **the only comment form** — no `//`/`;`/`%`),
`"..."` Text (backslash-escapes copied **raw/uninterpreted**, no generic escape-decoding), one
triple-backtick Fence token, `-inf`, Int/Float (leading `-`, digit run, optional `.`+digit, optional
`e`/`E` exponent — **requires at least one digit after `.`**, so `0.`/`10.` do NOT lex as floats;
leading `.` with no preceding digit is unhandled outside `..`), Ident (`is_ident_continue` allows
`_-./`), and single-char tokens `{ } [ ] ( ) , : = @ ^ + * /` plus `..`. **Not tokenized at all**:
`<`, `>` (only fused into arrow/edge forms), `&`, `;`, `$`, `'` (single quote), `%`, `~` — any bare
occurrence is a lex error/`Error` token.

**Protocol dialect**: `protocol <id>/version/schema/start/framing magic 0x…|record|chunked/use…`
header, then `header fixed N{...}` / `segment name? kind=N? {...}` / `record name tag=N? {...}` /
`struct name{...}` (def-only, never walked) / `enum name{...}` (def-only, never walked) /
`footer fixed N` / `chain <prim>`. `Prim`: u8/u16/u32/u64/i32/i64/f32/f64 (**all little-endian only,
`from_le_bytes` hardcoded**, grammar/component.rs:1633-1651 — no big-endian variant exists),
varint/zigzag (zigzag reads identically to varint — no real zigzag decode despite the name),
bytes/utf8 (greedy "rest of buffer minus reserved trailing bytes", not length-prefixed unless
`varint bytes`), `fixed(n)`, `Array(inner, Fixed(n)|Varint|Field(name))` (`Field(name)` looks up an
EARLIER field from a HashMap **local to one `walk_fields` call**, never threaded across blocks),
`Ref(name)` (**`walk_prim`'s `Prim::Ref` arm unconditionally returns `Err(...)` — grammar/
component.rs:1610** — struct/enum blocks are parsed into the AST but never actually walked against
bytes, not even for a non-recursive struct). `walk_protocol` (grammar/component.rs:1664-1718) visits
`spec.blocks` in file order **exactly once**, single forward pass, `pos` only increases, must land on
exactly `bytes.len()` — **no block-level repeat directive, no tag-dispatched conditional block
selection, no backward seek**. `kind=`/`tag=` on segment/record blocks are parsed into the AST but
**`walk_protocol` destructures them away with `..`** — they are inert metadata, never runtime-checked
against a byte (confirmed independently by the hybrid census agent, grammar/component.rs:1687-1701).

**Cross-artifact `use` mechanism — confirmed non-functional on both sides** (hybrid census agent,
Part B, verified against my own read of `FragmentRegistry`/`Recognizer::compile_with`,
grammar/component.rs:1180-1254): the grammar-side `FragmentRegistry::builtin()` only ever populates 7
built-in `👪️family/*` kit fragments via `include_str!`; `compile_with`'s merge loop silently no-ops
on any `use` name not among those 7 (no error). The protocol side has **no `FragmentRegistry`
equivalent at all** — `ProtocolFile.uses` is parsed and round-tripped by `print_protocol` but
`walk_protocol` never reads `spec.uses` anywhere in its body. **`use zip` in a docx protocol file
today parses, round-trips, and does nothing at walk time** — pure fiction until M1/M2/M3 build real
cross-artifact resolution. This is load-bearing for the FG4 OPC-tail wave's design.

---

## 1. Per-format dialect-requirements census (32 standards)

Compiled from three parallel deep-read agents (one per native-side class), each citing the artifact's
own real `⚙️engine`/`📸️snapshot` codec. All citations below are `file:line` against the named file.

### 1a. Text-native (12) — grammar models the real syntax

| standard | native side | dialect capability | specific gap(s) | recommended extension |
|---|---|---|---|---|
| txt/utf-8 | Raw UTF-8 body, `\n`/`\r\n` line split (own grammar doc: `line = *(OCTET-except-CR-LF)`) | **Insufficient** | Arbitrary prose routinely contains characters outside the fixed token alphabet → `Error` tokens (🔍️lexer/component.rs:381-388) | **ORCHESTRATOR DESIGN QUESTION** — fundamentally "raw character span"; only the built-in opaque Fence token comes close, and it can't represent literal ``` in body |
| json/rfc8259 | `{ } [ ] : ,` structural, `"..."` strings w/ `\" \\ \/ \b \f \n \r \t \uXXXX` + surrogate pairs (🔣️json/…/📸️snapshot/🦀️component.rs `parse_string`/`parse_unicode_escape`:193-267), numbers w/ exponent as preserved lexeme (`parse_number`:271-310) | **Partially sufficient** | `{}[]:,` ARE real tokens (LBrace/RBrace/LBracket/RBracket/Colon/Comma) and Int/Float already support exponents — outer shape expressible. Gap: shared lexer's `"..."` scanner copies `\`+next raw/uninterpreted (🔍️lexer:132-138) — `\uXXXX`/surrogate decoding has no grammar-level equivalent | Escape decoding stays a hand-written post-process hook (not fixable at grammar level); otherwise sufficient |
| csv/rfc4180 | Quote-state-aware scan, `""`-doubling escape inside quoted fields, literal commas/CRLF-as-data while quoted (📊️csv/…/⚙️engine/🦀️component.rs `parse_csv_records`:20-69) | **Insufficient** | (1) shared `"..."` scanner closes at first `"`, no doubling awareness (🔍️lexer:140-146) — `"a""b"` mis-tokenizes; (2) whole-file context-free pre-lex cannot resolve "structural comma" vs "quoted-data comma" by quote-depth state | Needs a lexer-level quote-doubling string mode, or CSV stays lexed by its own bespoke (non-shared-token) scanner — structural gap |
| md/commonmark | Leading-space-count-gated block classifiers: `leading_spaces()`, `>3`-space lazy continuation (📝️md/…/⚙️engine/🦀️component.rs:22-125), 4-space indented code (:61-63), list dedent = `indent+marker_len` per item (:87-104,138+), fence open/close length rules incl. tilde fences (:26-40) | **Insufficient (inherent)** | No fixed-token, whitespace-as-trivia lexer can express leading-whitespace-COUNT nesting even with grouping/quantifiers; also the lexer's own built-in Fence token requires exactly 3 backticks/no indent/no tilde — mismatched from CommonMark's own fence rule (🔍️lexer:165-222) | **ORCHESTRATOR DESIGN QUESTION** — impossible in principle for this token model; do not guess a fix |
| xml/1.0 | `<name attr="v">`/`</name>`/`/>`, entity decode `&amp; &lt; &gt; &quot; &apos; &#NNN; &#xHHHH;` (📰xml/…/📸️snapshot/🦀️component.rs `xml_unescape_text`:146-187), CDATA/comment/PI, single-OR-double-quoted attr values (`parse_attr_value`:387-405), `:` as name-start char | **Insufficient** | `<`/`>` not standalone tokens (only fused into arrow forms, 🔍️lexer:310-346); `&` not tokenized at all (→Error); single-quoted `'...'` strings unsupported (only `"`, 🔍️lexer:124) | Needs `<`/`>`/`&` promoted to real single-char tokens + single-quote string support added to the shared lexer alphabet |
| svg/1.1 | Reuses `xml_document_from_text`/`_to_text` directly (🎨️svg/…/📸️snapshot/🦀️component.rs:39-52) for outer doc, plus bespoke mini-grammars for `d`/`transform`/`points`/inline `style` attr values (`parse_path_data`:358-463, implicit-command/no-whitespace-required number runs) | **Insufficient (XML's gaps) + extra** | Same `<`/`>`/`&`/single-quote gaps as xml; inner mini-languages need char-level non-whitespace-delimited number scanning (`"M10-20"`) with no token-grammar equivalent | Outer: same XML lexer fix. Inner value mini-languages: arguably out of scope for a document-level grammar, stay hand-written regardless |
| obj/3.0 | `#` line comments (exact match to shared `#`-comment!), whitespace-split `v/vt/vn/f/o/g/usemtl/mtllib/s` keyword statements (🧊️obj/…/⚙️engine/🦀️component.rs `decode_obj`:81-218), `/`-separated face-vertex triplets w/ empty fields e.g. `v//vn` (`parse_face_vertex`:57-74) | **Mostly sufficient** | `#`, Ident/Int/Float, `/` all match real tokens; `v//vn` expressible as adjacent Slash Slash. Gap: `o`/`g` names are unquoted "rest of line" (can contain spaces) — no such capture primitive | Minor — "rest-of-line as one opaque token" terminal, or accept lossy re-join. Otherwise ready |
| stl/ascii | Keyword-line grammar: `solid <name>` (name=rest-of-line), `facet normal x y z`/`outer loop`/`vertex x y z`×3/`endloop`/`endfacet`/`endsolid <name>` (🟪️stl/…/⚙️engine/🦀️component.rs `decode_stl_ascii`:20-71) | **Mostly sufficient** | Multi-word keywords = adjacent Literal tokens, fine. Same "rest of line, arbitrary text" gap as obj's `o`/`g` for `solid_name`/`endsolid` | Same minor "rest-of-line" primitive request as obj |
| step/ap214 | Real Part-21 lexer: single-quoted strings w/ `''`-doubling + `\X\HH\`/`\X2\HHHH…\X0\` escapes (📐️step/…/⚙️engine/📐️part21/🦀️component.rs `read_string`:253-276, `read_escape`:278-336), trailing-dot-no-digit reals `0.`/`10.` (`read_number`:338-388), dot-delimited enums `.T.`/`.UNSPECIFIED.` (`read_enum`:390-409), `$` unset/`*` derived (`read_value`:414-421), `#N` entity refs, `/* */` block comments (`skip_ws_and_comments`:190-207), `;` statement terminators | **Insufficient** | (1) no single-quote handling at all; (2) shared Float rule requires digit after `.` → `0.` mis-lexes; (3) leading-`.` enum literals unhandled outside `..`; (4) `$`/`;` not tokenized at all; (5) shared lexer's ONLY comment form is `#`-to-EOL, but STEP's `#` is the entity-ref sigil `#123=...` — **direct conflict**, shared lexer would misparse every entity line as a comment | Multiple hard lexer-alphabet gaps — needs real alphabet extension (quoted-string+doubling, trailing-dot floats, leading-dot literals, `$`/`;` tokens, block comments) AND resolution of the `#`-comment-vs-entity-ref collision before STEP/IFC can be lexed at all |
| ifc/4 | Reuses `step::engine::part21::{parse_part21,write_part21}` directly (🏗️ifc/🏅️standards/🔖️4/⚙️engine/🦀️component.rs:7,252,258,271,291) — identical wire syntax to STEP | **Insufficient** | Identical gaps to step row (same tokenizer, same file) | Same as step |
| ifc/2x3 | Reuses `step::engine::part21::{parse_part21,write_part21}` directly (🏗️ifc/🏅️standards/🔖️2x3/⚙️engine/🦀️component.rs:4-6,12,27,42, explicit doc comment) | **Insufficient** | Identical gaps to step row | Same as step |
| dxf/r12 | Group-code/value line-pair tokenizer: every tag = int-code line + arbitrary-text value line, semantically typed by the code number (🖊️dxf/…/📸️snapshot/🦀️component.rs `tokenize_dxf`:322-338, `classify_group_code_value`:68); `$`-prefixed header var names | **Insufficient** | (1) value line is fundamentally "rest of physical line as one opaque string," not a token sequence — text like `SOME TEXT, with punctuation!` fragments across Ident/Comma/Error; (2) `$ACADVER`-style names need `$`, not tokenized; (3) per-code semantic typing (string vs float vs int chosen by an EARLIER code number) isn't a lexical-shape decision the terminal-predicate model handles | Needs same "rest-of-line opaque value" primitive as stl/obj PLUS `$` token support; code-driven typing dispatch stays a post-parse hook |

**Additional lexer alphabet findings** (text-native agent, full-file read of 🔍️lexer/🦀️component.rs,
797 lines): tokenized as single-char tokens: `{ } [ ] ( ) , : = @ ^ + * /` plus `..`. **Never**
tokenized: `<`, `>` (only fused into arrow/edge forms — no path to a bare `<`/`>` token), `&` (zero
special-casing anywhere), `;` (zero special-casing — directly blocks STEP/IFC statement terminators),
`'` (zero special-casing — blocks STEP/IFC strings and XML/SVG single-quoted attrs), `$` (zero
special-casing — blocks STEP/IFC unset sigil and DXF header var names), `%`, `~`. Comment syntax is
`#`-to-EOL only, with no block-comment form anywhere, actively conflicting with STEP/DXF's non-comment
uses of `#`/leading digits. String tokens preserve backslash escapes raw — no generic escape-decoding
capability exists in the lexer itself for ANY format.

### 1b. Binary-native (12) — protocol models the real byte layout

| standard | native side | dialect capability | specific gap(s) | recommended extension |
|---|---|---|---|---|
| binary/raw | No structure — `bytes: Vec<u8>` kept fully verbatim (💾️binary/🏅️standards/🔖️raw/⚙️engine/🦀️component.rs:1-56) | **Sufficient** | None | None — a single greedy `bytes`/`chain bytes` field covers this exactly |
| zip/2.0 | `PK\x03\x04` local header+payload/entry, `PK\x01\x02` central-dir record/entry (duplicate metadata), trailing `PK\x05\x06` EOCD located by **backward magic scan** from EOF (`find_eocd`, 🎒️zip/🏅️standards/🔖️2.0/⚙️engine/🦀️component.rs:229-241) because the preceding comment field is 0-65535 bytes variable; `decode_zip` jumps to `cd_offset` (a pointer read FROM the EOCD, `resolve_central_directory`:252-281) then for each entry jumps to `local_off` (a per-entry backward pointer, :350-403) to cross-validate. All LE, CRC32 custom table (:9-33) | **Insufficient** | (a) no backward/magic-scan discovery of a variable-length trailing footer whose start is unknown until its end is found; (b) no offset/pointer-field resolution (jumps to arbitrary earlier positions, impossible under strictly-forward monotonic `pos`); (c) no block repeat sourced from a count field in a LATER-positioned structure | **ORCHESTRATOR DESIGN QUESTION — see dedicated ZIP paragraph below (load-bearing, decide before M2)** |
| png/1.2 | 8-byte signature, chunks repeated **until `IEND`** (not count-prefixed): `len(u32 BE)+type(4 ASCII)+data+crc32(u32 BE)` (`read_chunks`, 📷️png/🏅️standards/🔖️1.2/⚙️engine/🦀️component.rs:42-77); dispatch on 4-byte type tag into wholly different field shapes (:540-726+); **all big-endian** | **Insufficient** | (a) no tag-dispatch construct; (b) repeat-until-sentinel-tag (not count-prefixed) doesn't exist; (c) all fields BE, dialect's Prim ints are LE-only | Big-endian `Prim` variants; tag-dispatch block construct keyed off a decoded value; "repeat until sentinel tag" block-repeat modifier |
| gif/87a | `GIF87a` magic + LE screen descriptor, then `loop{match introducer_byte{0x2C=>image desc, 0x21=>error(89a-only), 0x3B=>break}}` — classic tag dispatch + repeat-until-trailer (🎞️gif/🏅️standards/🔖️87a/⚙️engine/🦀️component.rs:487-529); LZW payload opaque (honest boundary) | **Insufficient** | (a) no block-introducer tag dispatch; (b) no repeat-until-sentinel-byte | Tag-dispatch construct; repeat-until-byte-value modifier |
| gif/89a | Same shell, **two-level** dispatch `loop{match b{0x21=>match label{0xF9 GCE,0xFE comment,0x01 plain-text,0xFF app-ext}, 0x2C=>image, 0x3B=>break}}` (🎞️gif/🏅️standards/🔖️89a/⚙️engine/🦀️component.rs:225-338); a GCE's decoded fields (`pending_gce`, :220,236) are consumed by the NEXT, different block (:256-268,319-334) — cross-block state carry | **Insufficient** | (a) nested/two-level tag dispatch; (b) cross-block state carry — a later block's shape depends on an earlier block's decoded values, beyond the per-block-local field env `walk_fields` provides | Tag-dispatch (nested); explicit "carry named decoded values forward into next N blocks" mechanism distinct from `Array(Field(name))`'s single-block scope |
| jpg/jfif-1.01 | `0xFFD8` SOI + marker-scan loop dispatching on marker byte into heterogeneous segment shapes (SOF0/DHT/DQT/DRI/SOS→hand off to bit-level entropy decode) (📷️jpg/🏅️standards/🔖️jfif-1.01/⚙️engine/🦀️component.rs:727-846); all BE (`read_u16`:919-923); entropy scan = opaque bit-packed Huffman (honest boundary) | **Insufficient** | (a) marker-byte tag dispatch across a large heterogeneous segment-shape set; (b) all-BE; (c) byte-scan for a marker prefix, not a fixed-position walk | Big-endian Prim variants; tag-dispatch construct; possibly a "scan for 2-byte marker prefix" primitive distinct from PNG/GIF's single-byte tag read |
| bmp/v3 | 14-byte file header + variable-size info header (branches at fixed offsets on `header_size`, 🖼️bmp/🏅️standards/🔖️v3/⚙️engine/🦀️component.rs:142-161) + **conditional** BITFIELDS masks only if `compression==3` (:138-161) + **conditional** palette only if `bpp<=8`, sized by an earlier field (:168-176) + pixel rows repeated `height` times, row-padded to 4-byte boundary (:188-241); all LE | **Insufficient** | (a) conditional field/segment PRESENCE gated on an earlier field's VALUE — no such construct exists at all (only unconditional sequences); (b) palette/row counts sourced from the header block but consumed in separate later blocks — crosses per-block-local env boundary | Conditional-presence field/segment guard (`if <earlier field>==<value>{...}`); thread `Field(name)` resolution across blocks |
| tiff/6.0 | `II`/`MM` byte-order mark at offset 0-2 selects LE/BE for the **entire rest of the file, at runtime** (🖼️tiff/🏅️standards/🔖️6.0/⚙️engine/🦀️component.rs:344-348); IFD chain = pointer-chased linked list until `next==0`, cycle-guarded (`read_ifd_chain`:122-138); tag values inline or out-of-line offset pointers (`read_tag_values`:143-154); pixel strips via `StripOffsets` (more pointers, :283-306) | **Insufficient** | (a) runtime-selected endianness for the WHOLE file from one leading marker — see dedicated TIFF paragraph below; (b) IFD-chain pointer-chasing repeat, not a single forward pass; (c) out-of-line tag-value offset resolution; (d) strip-offset pointer resolution | **See TIFF paragraph below** — needs "select Prim endianness from an earlier field, apply to all subsequent reads," plus the offset/pointer-resolution primitive ZIP needs, plus "repeat via next-pointer until sentinel 0" |
| deflate/rfc1950 | zlib container: 2-byte CMF/FLG + optional 4-byte DICTID + raw DEFLATE bitstream + 4-byte Adler32 trailer; Adler32/DICTID **BE** (🗜️deflate/🏅️standards/🔖️rfc1950/⚙️engine/🦀️component.rs:522,543,578,581,618,629); inner stream is bit-level (not byte-level) LSB-first Huffman/LZ77 | **Insufficient for container fields; compressed payload is an honest, intentional boundary** | (a) Adler32/DICTID BE; (b) the DEFLATE bitstream cannot be modeled at Prim/byte granularity at all — not merely "opaque bytes" but opaque because Huffman codes are variable-bit-length and cross byte boundaries | Big-endian Prim variants for the container; DEFLATE bitstream itself stays opaque `bytes` — no dialect extension reaches inside it |
| las/1.0 | Fixed 227-byte header at named offsets, LE (☁️las/🏅️standards/🔖️1.0/⚙️engine/🦀️component.rs:33-48,63-91); VLRs repeat `number_of_vlrs` times (header field), each a 54-byte sub-header + length-in-itself data (`decode_vlrs`:158-175); point records repeat `number_of_point_records` times with a conditional legacy/extended-count fallback (:200-206); point field-SET chosen once by header's `point_data_format_id`, applied to every record (`decode_point`:107-154) | **Insufficient** | (a) block-level repeat sourced from an earlier header field (no such construct — only within-block `Array(Field(name))`); (b) a header field selecting which of several field-set shapes applies to EVERY following record (single upfront selection, not per-instance dispatch) — nearest concept is `Ref`/`Enum` resolution, unconditionally errors; (c) two-condition conditional field lookup | Block-level "repeat N times, N=earlier-block field" modifier; real `Ref`/`Enum` resolution usable as a repeated-record's shape selector |
| dwg/ac1018 | **No real byte-level codec exists** — 🖊️dwg/🏅️standards/🔖️ac1018/⚙️engine/🦀️component.rs is an 82-line stub (register/struct/tests only); its own test calls `decode_dwg` from a DIFFERENT module against a synthetic `b"AC1018\0…"` stub, asserting only version string + verbatim bytes round-trip (:70-79) | **N/A — cannot assess** | No real decode to compare against the dialect | N/A until a real byte-level pipeline lands (likely faces gaps similar to ac1024's by analogy, not confirmed) |
| dwg/ac1024 (R2004+) | Real D1-D2 pipeline (🖊️dwg/🏅️standards/🔖️ac1024/⚙️engine/🦀️component.rs): (1) file header decrypted via symmetric LCG XOR cipher before any field readable (`decrypt_r2004_header`:20-28, applied :346-348); (2) header fields include pointers into a SEPARATELY-compressed page-directory blob (:353-367); (3) blob decompressed (bespoke LZ77 variant) into a `page_number→file_address` table (`parse_page_directory`:243-260); (4) `section_info_id` looked up in THAT table (:380-384), its own page decompressed+parsed into name-dispatched sections (:295-335); (5) each section's pages resolved AGAIN via the same page table (:401,409-411) — two-level join across two independently-decompressed side-structures; (6) each page has its own 32-byte encrypted header, XOR-keyed by ITS OWN file address, i.e. position-dependent key (`decrypt_page_header`:213-231); (7) page payload optionally decompressed, bounded by a field from a DIFFERENT block (:452) | **Insufficient — strictly harder than ZIP/TIFF's single-level pointer gap** | (a) no decrypt-before-parse transform stage exists at all; (b) position-dependent (self-referential) decryption keys; (c) two-level indirect lookups across separately-decompressed side-tables, not a single pointer hop; (d) name-string dispatch rather than numeric tag dispatch; (e) a repeated block's decompression bound sourced from an unrelated block | **ORCHESTRATOR DESIGN QUESTION** — likely needs a dedicated non-linear/imperative decode path entirely outside the `.protocol.semio` block-list model, not an incremental extension |

**ZIP central-directory — forward-walk vs. backward-seek (load-bearing decision, binary-native census
agent)**: `decode_zip` (🎒️zip/🏅️standards/🔖️2.0/⚙️engine/🦀️component.rs:286-435) is **not** a linear
forward pass even in principle — this is structural, not an implementation choice that could be
refactored away. `find_eocd` scans BACKWARD from EOF for `PK\x05\x06` (:229-241) because the EOCD's
preceding comment field is 0-65535 bytes: there is no way to know where the footer starts without
first finding its end. A "forward-walk + trailing directory as opaque bounded tail" reformulation
still cannot avoid this: `cd_size`/`cd_offset` are themselves fields stored INSIDE the EOCD, which
sits at the very end of the file — nothing about the central directory's span is knowable until the
backward scan happens first ("trailer describes the middle," same family as MP4's moov-at-end).
Additionally `decode_zip` performs a genuine per-entry backward jump via each central-directory
record's `local_off` pointer (:316,341), dereferenced for real cross-validation (:350-403), not
redundant/ignorable bytes. **ZIP genuinely requires backward-seek/offset-pointer resolution the
current strictly-forward `walk_protocol` cannot express** — record this before M2 starts.

**TIFF byte-order marker — a structurally distinct kind of gap**: TIFF's `II`/`MM` marker
(🖼️tiff/🏅️standards/🔖️6.0/⚙️engine/🦀️component.rs:344-348) does not mean "sometimes big-endian
instead of little" — it means the endianness of EVERY subsequent multi-byte field, including fields
declared LATER in the same `.protocol.semio` text, is chosen AT DECODE TIME by a value read from the
data itself. This differs qualitatively from adding `U16Be`/`U32Be` Prim variants (sufficient for
PNG/JPEG, where endianness is a static format-wide constant known from the grammar alone). A
`.protocol.semio` file's `Prim` declarations are static text — `field width u32` picks byte order once
at authoring time — but TIFF needs the SAME declared field read as LE for one file and BE for another,
decided by a byte the walker itself just consumed. Honest support needs a new dialect-level construct
("read this field, let its value select the endianness applied to all Prim reads for the remainder of
the walk") — a runtime-conditioned interpretation mode, not a new leaf type. Scope and name this
separately from a generic "add big-endian primitives" item.

### 1c. Hybrid/both-real (8) — grammar + protocol both real

| standard | native side(s) | dialect capability | specific gap(s) | recommended extension |
|---|---|---|---|---|
| gltf/2.0 | JSON document (base64 `data:` URIs via hand-rolled table, 🧊️gltf/🏅️standards/🔖️2.0/⚙️engine/🦀️component.rs:16-57; enum-shaped `componentType`/`type` fields — semantic constraints not new tokenization needs) + GLB binary container | JSON side: sufficient (per §1a json row). GLB side: **insufficient** | `decode_glb` (:416-444) is a `while pos+8<=len` loop reading `(u32 LE length, 4-byte type tag)` and dispatching on tag (`JSON`/`BIN\0`), tolerating any order/count, skipping unknown types. `walk_protocol` visits blocks in file order exactly once — no repeat-until-EOF, no tag dispatch; confirmed the `kind=`/`tag=` syntax that LOOKS like dispatch support is inert AST metadata (destructured away with `..`, grammar/component.rs:1687-1701) | **ORCHESTRATOR DESIGN QUESTION** — general "repeated tag-dispatched chunk" block type (would also fix PNG/GIF/ZIP-central-dir/PDF-1.7-xref-stream at once) vs. a narrower fixed-2-chunk positional GLB stopgap that undersells the real decoder's tolerance |
| pdf/1.4 | COS text (objects/dict/name/string/array/xref/trailer) + `stream`/`endstream` FlateDecode payload | **Insufficient, but current codec is a documented frozen stub that doesn't exercise it either** | 📄️pdf/🏅️standards/🔖️1.4/⚙️engine/🦀️component.rs is 112 lines; `encode_pdf` hand-writes a fixed 5-object template w/ plaintext xref (:8-35, confirming xref/trailer are TEXT); `decode_pdf` does NO COS parsing, just a raw substring search for `stream`/`endstream` + zlib-inflate (:50-75); test comment "1.4 stays a frozen stub" (:101-104). Separately: shared grammar lexer's `GKind` has no Slash/`<`/`>`/`[`/`]` tokens — 1.7's real parser uses its own hand-rolled byte-level Lexer entirely independent of `crate::os_dsl::lex` (1.7 ⚙️engine/component.rs:58) — COS syntax can't be tokenized by today's shared lexer at all | No urgent grammar work for 1.4 itself (stub doesn't exercise COS grammar); inherits 1.7's gaps if later enriched |
| pdf/1.7 | Real COS parser (1780 lines, own hand-rolled Lexer) + classic-text xref + **binary xref STREAMS** | **Insufficient** — same lexer-token gap as 1.4, plus a genuine binary-protocol gap | Xref stream (`parse_xref_stream`:625-666): row width = `w[0]+w[1]+w[2]` bytes where `/W=[...]` is discovered AT RUNTIME from a text-parsed COS dict PRECEDING the binary blob (:633-640) — record width isn't a compile-time constant, and nothing lets a grammar-parsed value parameterize a protocol field width (`Count::Field` only reaches an earlier field in the SAME local env, never cross-dialect). Each field decoded big-endian by manual shift (`decode_xref_row`:574-585) — Prim has zero BE variants (only call sites are `from_le_bytes`, grammar/component.rs:1635,1640,1645). Row type byte does NOT change row width (only semantics, :658-662) — tag-dispatch framing is a red herring here | **ORCHESTRATOR DESIGN QUESTION** — needs a BigEndian Prim family PLUS a cross-dialect "this field's width/count came from a value the grammar side already parsed" mechanism, shared with ply below |
| ply/1.0 | ASCII header+data, AND both `binary_little_endian` and `binary_big_endian` — all 3 real and tested | **Insufficient — confirmed real gap** | ☁️ply/🏅️standards/🔖️1.0/⚙️engine/🦀️component.rs implements all 3 `PlyFormat` variants (:133-134,172-177,301-303,354-373,392-393) w/ round-trip tests for both LE (:515-522) and BE (:524-531) — no BigEndian Prim exists (same gap as pdf/1.7). Additionally the per-element property SCHEMA (names/types/order, incl. list-count-kind) comes from a preceding ASCII header (`parse_header_text`:386) that then dictates the exact binary body layout PER FILE (:354-373) — the set of fields/blocks itself is file-defined, not just one width | **ORCHESTRATOR DESIGN QUESTION** — (1) BigEndian Prim family (shared fix with pdf/1.7); (2) decide whether PLY's dynamic per-file binary schema is `.protocol.semio`-shaped at all, since the dialect assumes a spec fixed at authoring time |
| docx (ecma-376 OPC) | Real OPC container (delegates to zip's Rust module) + XML parts | **Insufficient — no dialect files exist yet** (stub `*OCTET` placeholders) | `use crate::artifacts::zip::opc::{...}` 📜️docx/🏅️standards/🔖️ecma-376/⚙️engine/🦀️component.rs:8, calls `opc::decode_opc`/`encode_opc` (:389,393,436) — confirmed real (not aspirational). Parts: `word/document.xml`(:51), `word/styles.xml`(:53), `word/numbering.xml`(:552) + generic OPC `[Content_Types].xml`/`*.rels` (🎒️zip/📦️opc/🦀️component.rs:50-57,199-219) | Rust-level delegation is real and solid; dialect files should mirror it — grammar imports XML productions (needs cross-artifact `use` to actually work, see §0) + docx-specific part-name productions; protocol needs zip's real record shapes, which don't exist yet either (zip's own `.protocol.semio` is still boilerplate stub — see zip note below) |
| xlsx (ecma-376 OPC) | Same OPC pattern | **Insufficient — no dialect files exist yet** | `use crate::artifacts::zip::opc::{...}` 📕️xlsx/…/⚙️engine/🦀️component.rs:11, calls :519,523,585. Parts: `xl/workbook.xml`(:48), `xl/sharedStrings.xml`(:49), `xl/worksheets/sheetN.xml`(:498), `xl/styles.xml`(:770) | Same as docx |
| pptx (ecma-376 OPC) | Same OPC pattern + recursively-nested shape-tree XML | **Insufficient — no dialect files exist yet** | `use crate::artifacts::zip::opc::{...}` 🎞️pptx/…/⚙️engine/🦀️component.rs:17, calls :559,563,608. Parts: `ppt/presentation.xml`(:55), `ppt/slideMasters/1.xml`(:56), `ppt/slideLayouts/1.xml`(:57), `ppt/theme/1.xml`(:58), `ppt/slides/N.xml`(:526). Doc comment (:7-12): slide body is recursive (`p:spTree` containing `p:sp`/`p:grpSp`/`p:graphicFrame`/`p:cxnSp`, groups nest groups) | Same OPC-reuse pattern as docx/xlsx, PLUS confirm the grammar dialect's `Ref` genuinely supports self-recursion before assuming the shape-tree grammar is free (recursion is claimed to already work per the plan's mechanism findings — worth a direct conformance test, not just an assumption, in FG3) |
| bcf 2.1 (bcfzip) | Real zip container, deliberately NOT OPC (no content-types/relationships) | **Insufficient — no dialect files exist yet** | Docstring states explicitly (💬️bcf/🏅️standards/🔖️2.1/⚙️engine/🦀️component.rs:3-5): "NOT an OPC package... builds its own simple wrapper directly on `zip::ZipEntry`" — imports `zip::schema::snapshot::ZipEntry`(:12), calls `zip::engine::encode_zip`/`decode_zip` directly (:413,417), one layer below OPC. Parts: `bcf.version`(:399/421), `<guid>/markup.bcf`(:401), `<guid>/<guid>.bcfv`(:403), catch-all `BcfRawPart`(:562,599) | Cannot reuse OPC-specific grammar built for docx/xlsx/pptx (no content-types/rels layer) — needs its own non-OPC part-naming grammar sitting directly on zip's raw-entry productions |

**Zip's own dialect files are still stub-only** (hybrid census agent, context note): real signatures
at 🎒️zip/🏅️standards/🔖️2.0/⚙️engine/🦀️component.rs:222-224 (`SIG_LOCAL=0x04034b50`,
`SIG_CENTRAL=0x02014b50`, `SIG_EOCD=0x06054b50`) but zip's `.protocol.semio`/`.grammar.semio` are
still `magic = %x00`/`payload = *OCTET` boilerplate — no local-header/central-dir/EOCD structure
modeled at the dialect level yet. So the OPC-reuse story is solid at the Rust-engine level but there
is currently ZERO real dialect coverage for zip to delegate from, let alone docx/xlsx/pptx/bcf —
**zip's own protocol file must land (FG2, before or alongside FG4) before the OPC-tail wave (FG4) can
meaningfully delegate to it**, independent of whether cross-artifact `use` gets built.

**GLB/PNG/ZIP/PDF-1.7 chunk-tag-dispatch commonality** (hybrid census agent): GLB's decode loop
(read length+type, branch on type, advance, repeat until EOF, skip unknown) is structurally identical
to PNG/GIF chunk dispatch, close to ZIP's local/central-directory walk, and in spirit to PDF/1.7's
xref-stream row loop. All four want the same protocol primitive: "repeat: read a discriminator +
length/count, branch or consume, until EOF/N." A single well-designed "repeated tag-dispatched block"
addition in M2 would likely unlock GLB, PNG, GIF, ZIP's central directory, and partially PDF/1.7's
xref stream at once — flagged as the single highest-leverage protocol-dialect extension across this
whole census.

---

## 2. JSON-transfer census

Grepped every `serde_json::to_vec`/`to_string`/`from_slice`/`from_str`/`Value` occurrence in
`✏️s/🔌️plugins/🗄️stdio/` (26 raw hits, all read in context below) plus the F6 `print().into_bytes()`
text-as-binary shortcut pattern (grepped separately, 43 hits across `OpBinary`/`DiffCodec` impls) plus
the framework io wire-compose layer named in the plan.

### 2a. Literal JSON transfer (real violations against "no JSON on any transfer path")

| file:line | what's transferred | mechanism | belongs to | in-scope for this program? |
|---|---|---|---|---|
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:83-99` | `Ifc2x3Mutation` op text AND binary — `OpText::print_op`/`parse_op` = `serde_json::to_string`/`from_str`; `OpBinary::encode_op`/`decode_op` = `serde_json::to_vec`/`from_slice` | **Literal JSON transfer** — never migrated to F6's `keyword key=value` text shape or `dsl::variants_binary` real binary; matches STATUS.md's "ifc/2x3 diff-completeness breach, the only remaining one" | ifc/2x3 (32nd standard, explicitly out of Phase 1's F6 scope) | **Yes — in scope for this program** (stdio's own op/diff facet, FG4 per the execution plan already lists ifc/2x3 there) |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs:1208-1230` | `SvgSnapshot`'s `ArtifactPack::encode_pack_with`/`decode_pack_with` (the BINARY pack transfer path) — payload is `serde_json::to_vec(&self.doc)`/`from_slice`, wrapped in a real binary SEMIO envelope (`wrap_binary`/`unwrap_binary`) | **Literal JSON payload disguised as binary** — outer framing is real (magic/envelope), inner content is raw JSON bytes, not pack_rt/DSL-spec-driven | svg (own snapshot pack facet) | **Yes — in scope** (FG3, svg row) |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs:533-556` | `XmlSnapshot`'s `ArtifactPack::encode_pack_with`/`decode_pack_with` — identical pattern to svg | **Literal JSON payload disguised as binary** | xml (own snapshot pack facet) | **Yes — in scope** (FG1, xml row; note svg embeds xml's node model per Phase 1 design, so fixing xml's pack path likely also touches/motivates svg's) |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs:5-7` | `deserialize(from: &JsonSnapshot) -> GltfSnapshot` — re-serializes json artifact's typed `JsonValue` to bytes via `serde_json::to_vec(&from.value)`, then re-parses as `GltfDocument` via `serde_json::from_str` inside `parse_gltf_document` | **Literal JSON round-trip used as cross-artifact glue** — a real internal transfer surface bypassing DSL/pack entirely, not native-format serde | gltf (own io/deserializers bridge from json) | **Yes — in scope** (FG3, gltf row; this cross-artifact bridge is stdio-internal, not framework) |

### 2b. Native-format JSON (NOT a transfer-path violation — informational only)

| file:line | why it's fine |
|---|---|
| `🧊️gltf/🏅️standards/🔖️2.0/⚙️engine/🦀️component.rs:333,353,387,449` (`parse_gltf_document`/`serialize_gltf_document`/`encode_glb`/`decode_glb`) | Uses `serde_json` purely to parse/emit the FORMAT'S OWN native JSON text — a `.gltf` file and a GLB's JSON chunk literally ARE JSON per spec. This is legitimate native-format codec implementation (analogous to using a library to implement a text-native format's own grammar), not an internal wire/pack shortcut. The M1 grammar-dialect work for gltf's JSON side (§1c) is the correct venue for eventually modeling this at the dialect level — the `serde_json` crate usage in the Rust engine itself is not something to eliminate. |
| `🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs:34` (`looks_like_gltf_json`) | `serde_json::from_str::<serde_json::Value>` used only for format-sniffing/classification (does this input look like a `.gltf` JSON document), not for moving data across a boundary. Uses untyped `Value` which would matter for a separate "no bare Value in public types" grep gate, but not the transfer-path ban. |
| `🔣️json/…/📸️snapshot/🦀️component.rs:4`, `🔣️json/…/✳️i-json/🧐️analyzer/🦀️component.rs:6`, `🔣️json/…/✳️i-json/🧬️schema/🦀️component.rs:8` | Doc-comment text mentioning `serde_json::Value` to describe what the code deliberately does NOT do — no actual usage |
| `🎞️gif/📚️examples/💃️dancing/🦀️component.rs:26`, `🧊️gltf/📚️examples/🌱️metabolism/🦀️component.rs:33`, and `🧊️gltf/📚️examples/🌱️metabolism/🧪️tests/🦀️test.rs:6` (doc comment) | `serde_json::to_string(&decoded_snapshot())` feeds `ExampleSource::new(id, label, document_json, icon)` — see §2c, this is a FRAMEWORK-level API surface (field literally named `document_json: String`), not stdio's own transfer design. Only 2 of 32 `ExampleSource::new` call sites in stdio actually pass real JSON (the "real fixture" dancing.gif/metabolism.gltf examples); the other 30 pass `.dsl.semio`/stub text through the same misleadingly-named parameter. |

### 2c. Framework-level JSON-transfer surfaces (out of scope for stdio F-waves — note only, per the plan's own M3 item)

| file:line | what's transferred | status |
|---|---|---|
| `🧰️framework/🔨️modules/🚪️io/🦀️component.rs:494` (`wire_decode_composed_artifact`) | Cross-plugin `WireComposedArtifact` — decoded from literal JSON bytes | Documented simplification |
| `🧰️framework/🔨️modules/🚪️io/🦀️component.rs:500-505` (`wire_list_composer_entries`) | This process's composer roster, encoded as `serde_json::to_vec` | Doc comment (:500-503) explicitly states: "JSON (not `pack_rt::encode_wire_value`) is a deliberate simplification for this first cut: the WIT signature is an opaque `list<u8>` either way... this module has no existing dependency on `store`/`dsl`'s pack machinery worth introducing just for this," and cites THIS ticket by name |
| `🧰️framework/🔨️modules/🚪️io/🦀️component.rs:517-518` (`wire_artifact_compose`, decode) | `IoKey` + `Vec<WireComposeSource>` request, decoded from literal JSON bytes | Same simplification |
| `🧰️framework/🔨️modules/🚪️io/🦀️component.rs:533` (`wire_artifact_compose`, encode) | Compose result, encoded as literal JSON bytes | Same simplification |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:3171-3176` (`ExampleSource.document_json: String`) | Every plugin's example-gallery payload — field name implies JSON but in practice carries either real JSON (2/32 stdio call sites) or plain `.dsl.semio`/stub text (30/32) through the same `String` parameter | Structural framework API, not stdio-specific; misnamed rather than uniformly JSON |

This confirms the plan's own citation exactly: the `🚪️io` wire-compose layer's JSON choice is a
**real, currently-live, host↔guest WIT-ABI boundary** (guest's `io-dispatch` fallback hook,
`list-artifact-dialects`/`artifact-compose` exports) — a genuine transfer path, not internal
bookkeeping — but it is framework-owned (`🧰️framework/🔨️modules/🚪️io`, not under `✏️s/🔌️plugins/🗄️stdio`)
and per the plan's own M3 scoping ("io wire flip lands as its own M3 item with both ends updated
atomically") should NOT be touched by any stdio FG-wave. Flag and defer to M3/orchestrator.

### 2d. Text-as-binary shortcut (F6's `print().into_bytes()` pattern — separate category, NOT literal JSON but NOT real binary either)

Grepped `Ok(self.print_op().into_bytes())` and `Ok(self.print_diff().into_bytes())` across all of
`✏️s/🔌️plugins/🗄️stdio/`:

- **`OpBinary::encode_op` via `print_op().into_bytes()`**: 18 standards (bcf, json, dxf, ifc/4, ply,
  svg, md, tiff, xml, jpg, png, pdf/1.7, xlsx, docx, pptx, step, csv, gltf — one `component.rs` each
  under `🧬️mutations/`).
- **`OpBinary::encode_op` via real binary** (`dsl::variants_binary::encode_op`/`decode_op` — confirmed
  by direct read of zip's and bmp's `🧬️mutations/🦀️component.rs`, matching the F6 "derive path"): the
  13 standards NOT in the shortcut list above — binary, txt, zip, bmp, obj, deflate, dwg/ac1018,
  dwg/ac1024, pdf/1.4, las, gif/87a, gif/89a, stl. (ifc/2x3, also absent from the shortcut list, uses
  literal JSON instead — see §2a, not real binary.)
- **`DiffCodec::encode` (all 25 standards with a `🔺️diff/🦀️component.rs` implementing the trait) via
  `print_diff().into_bytes()`**: **100% — every single one** (`grep -rL` against the full
  `impl protocol::DiffCodec` file list returned zero exceptions). This matches the Phase 1 F6 execution
  log's own finding that hand-rolled diffs (mandated by the "no `Option<Option<T>>`/enum" derive
  limitation) are "roughly half the program by count, more by effort" — in practice, for the DIFF
  facet specifically, it is not half but essentially the WHOLE facet still on the text-as-binary
  shortcut. (7 standards lack a `DiffCodec` impl file matching this grep entirely — gif/87a and
  gif/89a's diff files DO match via a separate check; the discrepancy between "25 files" and "31+
  standards" reflects standards sharing one `🔺️diff` module across sub-standards or not yet having a
  separate diff facet wired — worth a closer look in FG-wave dispatch, not resolved here.)

**Classification**: per the task's own instruction, this counts as "text-as-binary shortcut," a
distinct, softer category from literal JSON — it IS already the real DSL text shape F6 landed
(`keyword key=value`), just not yet framed as a true binary record. This is exactly what Phase 2's
target architecture (§ "Diff/mutations facets" in `📖️phase2-design.md`) mandates upgrading via
`pack_rt::encode_record_body` (where spec-expressible) or handcrafted per-artifact binary layouts —
**belongs to each artifact's own F6-successor op/diff facet, in scope for FG1-FG4's per-standard
"binary-frame upgrade of DiffCodec/OpBinary" deliverable**, not a separate JSON-transfer bug to fix
first.

---

## 3. Baselines

### 3a. `cargo test -p semio-s-plugin-stdio --lib`

**Result: 1075 passed, 0 failed, 0 ignored** (finished in 7.80s) — exact match to Phase 1's G-gate
number recorded in STATUS.md, confirming zero regression since Phase 1 closed.

Getting to this clean result required **3 retries**: the first two attempts failed to compile with
three DIFFERENT error signatures in successive runs (crate-name resolution error citing
`semio_framework`/`semio_framework_hash`; a transient "file not found" read error; then a stable
"cannot find `store`/`semio_framework` in scope" error, 42 errors, in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs`, reproduced identically twice with
an unchanged file checksum). Root-caused via `git status --porcelain`: **this repo-wide, currently
uncommitted working tree has 166+ modified/added files spanning dozens of unrelated plugins**,
including `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` itself (modified, uncommitted)
plus untracked scratch files literally named
`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT/w1-framework-check-with-workflow{,-v2,-v3}.txt`
— a live concurrent session on a DIFFERENT ticket is mid-refactor across `store`/`workflow` framework
wiring right now. This matches the documented "Concurrent Cargo Workspace Churn" pattern exactly
(target-dir/incremental-cache races producing different transient error sets across nearby builds of
an actively-edited dependency). The third attempt, run after this diagnosis, succeeded clean. Neither
`🗣️dsl` nor `🎒️pack` (this program's own choke files) were touched by that churn — see §3c.
**Baseline stands at 1075/0**, but note for M1/M2 dispatch: expect possible transient compile flakiness
from this unrelated concurrent session until it settles; retry once before concluding a real
regression.

### 3b. `cargo test -p semio-framework-os-kernel` (package name confirmed via `Cargo.toml`)

**Result: 736 passed, 5 failed, 0 ignored** (reproduced identically on a second run — file checksums
unchanged, stable, NOT transient). Failures, all in the non-stdio pilot regression gate the Phase 2
plan explicitly depends on:

```
os_dsl::fixture_sweep::m5_handcrafted_grammar_conformance::dag_dsl_grammar_recognizes_shipped_fixture_tokens
os_dsl::fixture_sweep::m5_handcrafted_grammar_conformance::en1992_dsl_grammar_recognizes_shipped_fixture_tokens
os_dsl::fixture_sweep::m5_handcrafted_grammar_conformance::fem2d_dsl_grammar_recognizes_shipped_fixture_tokens
os_dsl::fixture_sweep::m5_production_coverage::dag_reports_uncovered_productions_for_shipped_fixture
os_dsl::fixture_sweep::m5_production_coverage::en1992_reports_uncovered_productions_for_shipped_fixture
```

**Critical finding for M1/M2 dispatch**: 3 of the plan's 6 non-stdio pilot grammars (`dag`, `en1992`,
`fem2d`) are **currently failing `m5_handcrafted_grammar_conformance` at HEAD**, before Phase 2 even
starts (`lowpoly`, `cad`, `note` — the other 3 pilots — still pass; failure count is 736 pass / 5 fail,
stable across reruns). Since `git status --porcelain` on `🗣️dsl` (which contains
`🧪️fixture-sweep/🦀️component.rs` and the pilot fixture files) is CLEAN (no uncommitted changes — see
§3c), this is baked into the current committed HEAD, not live churn. **The plan's stated regression
gate — "the 6 existing non-stdio pilot grammars... must keep parsing and passing m5" — is already
violated for half of them before M1 begins.** The orchestrator needs to decide: (a) M1 is only
responsible for not making these 3 WORSE (can't guarantee "keeps passing" for tests already red), or
(b) fixing these 3 pilots is itself a prerequisite item, possibly folded into M1's own scope since it
touches the same `🗣️dsl/📖️grammar` mechanism area. This report does not diagnose the root cause of the
3 failures (out of W0's read-only recon scope) — flagging the fact and the exact test names for the
orchestrator's triage.

### 3c. Concurrent-edit check on choke files

```
git status --porcelain -- "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl"   → (empty, clean)
git status --porcelain -- "🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack"  → (empty, clean)
```

**Both of this program's own choke files (`🗣️dsl`, `🎒️pack`) are clean — no concurrent session has
uncommitted changes there right now. M1/M2 can start immediately without a coordination wait**, per
the plan's own stated gate. (Repo-wide `git status --porcelain` shows 166+ files modified elsewhere —
see §3a — none inside `🗣️dsl`/`🎒️pack`; that churn is a different, already-identified concurrent
session on ticket `26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT`, touching
`store`/`workflow`, unrelated to this program's own files. Classify-don't-chase per the repo's standing
guidance; not investigated further.)

### 3d. Design doc placement

`📖️phase2-design.md` placed in this ticket folder — verbatim copy of the "PHASE 2 PROGRAM — Real-Format
Grammars & Protocols (FINAL PLAN)" section of `~/.claude/plans/the-current-schemas-are-scalable-journal.md`
(from the `## Context` heading through the final `## Verification (end-to-end definition of done)`
section, including the Phase-2-specific survey/mechanism findings blocks), with a header noting it was
placed by P2-W0 and pointing back at the full journal path. Confirmed not gitignored
(`git check-ignore -v` exit 1 = not ignored).

### 3e. Ticket folder sanity check

`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/` contains
282 entries (Phase 1's full history: `STATUS.md`, `🧬️schema-design.md`, and per-wave
`f1`-`f6`/`s1`-`s2`/`g`-prefixed `*-report.md`/`*-scratch`/`*-artifacts` dirs, plus this report and
`📖️phase2-design.md` newly added). Nothing existing was touched, moved, or deleted — folder confirmed
in a sane state to keep working in.

---

## Summary for the orchestrator

- **32/32 standards censused** across all three native-side classes; every "insufficient" row carries
  a `file:line` citation to the real codec proving the gap. 4 rows are flagged
  **ORCHESTRATOR DESIGN QUESTION** requiring judgment before M1/M2 can proceed on them specifically:
  txt (raw-character-span grammar), md (leading-whitespace-count nesting — impossible in principle for
  a token grammar), zip (backward-seek — load-bearing for M2, answered above: genuinely required, not
  avoidable via forward-walk reformulation), dwg/ac1024 (decrypt+two-level-indirection — likely needs
  a decode path entirely outside the block-list model), pdf/1.7 + ply (cross-dialect grammar→protocol
  field-width parameterization, shared design question), gltf/GLB (tag-dispatched repeated chunks —
  high-leverage, shared by PNG/GIF/ZIP-central-dir/PDF-1.7 too).
- **Highest-leverage single M2 addition**: a general "repeated tag-dispatched block" construct — would
  meaningfully unlock GLB, PNG, GIF (both standards), ZIP's central directory, and partially PDF/1.7's
  xref stream at once.
- **Cross-artifact `use` is non-functional on both grammar and protocol sides today** — the FG4
  OPC-tail wave's "delegate to zip's dialect productions" design depends on M1/M2/M3 building real
  resolution; the Rust-engine-level delegation (ordinary Rust `use` of `zip::opc`) is real and already
  proven by docx/xlsx/pptx/bcf, but the dialect-file layer has zero coverage to delegate to yet (zip's
  own `.protocol.semio` is still stub).
- **JSON-transfer census**: 4 real literal-JSON-transfer violations in stdio's own scope (ifc/2x3
  op+diff, svg pack, xml pack, gltf's json-deserializer bridge) — all naturally fall into their
  standard's existing FG-wave slot, no separate wave needed. 1 framework-level surface (`🚪️io` wire
  layer, 4 call sites) correctly out of scope per the plan's own M3 note. The F6 "text-as-binary"
  shortcut is far more widespread than literal JSON: **100% of `DiffCodec::encode` impls** and 18/31
  `OpBinary::encode_op` impls still use `print().into_bytes()` — this is the real bulk of the "binary
  whenever possible" mandate's remaining work, correctly scoped to each FG-wave's per-standard
  deliverable rather than a standalone cleanup pass.
- **Baselines**: stdio 1075/0 (clean, matches Phase 1 exactly, after riding out unrelated transient
  concurrent-churn build flakiness). framework-os-kernel 736/5 — **3 of 6 non-stdio pilot grammars
  (dag, en1992, fem2d) already fail `m5` conformance at current HEAD, reproducibly, not transient** —
  needs an explicit orchestrator decision on whether M1 owns fixing this or only not-worsening it.
- **`🗣️dsl`/`🎒️pack` are clean of concurrent edits — M1 can start immediately**, no coordination wait
  needed.
- Design doc and this report both placed; ticket folder confirmed sane, nothing else touched.
