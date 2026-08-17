# Grammar & Protocol Recipe — the copy-pasteable reference for every FG-wave

Written by Phase 2's PC (pilot-closer) wave, after M1 (grammar/lexer), M2 (protocol/walker), M3
(harness/registry/envelope), and the P1-P3 pilot ladder (json, csv, zip, png, txt, binary) all
landed. Every syntax fragment below is copied **verbatim** from a real, committed pilot file (or,
where noted, a framework file) — never paraphrased — with an exact repo-relative citation. Read
this once before starting your standard's grammar/protocol files; it is the thing every future
FG-wave brief points at instead of re-deriving the dialect from scratch.

Source material this recipe consolidates: `p2-w0-recon-report.md`, `p2-m1-report.md`,
`p2-m2-report.md`, `p2-m3-report.md`, `p2-p1-json-report.md`, `p2-p1-csv-report.md`,
`p2-p1-fix-report.md`, `p2-p2-zip-report.md`, `p2-p2-png-report.md`, `p2-p3-txt-report.md`,
`p2-p3-binary-report.md` (all in this ticket folder), plus direct reads of the real dialect parser
(`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/🦀️component.rs`) and the 36 real
`.grammar.semio`/`.protocol.semio` files the 6 pilots landed on disk.

---

## 1. Grammar-file syntax (the `.grammar.semio` dialect)

### 1.1 Header

Every real grammar file starts with these lines, each on its **own physical line** (the parser's
main loop is unified for header directives AND productions — see §3.1's reserved-word pitfall):

```
dialect grammar
grammar <id>
extension <ext>          # optional
start <production>
```

Verbatim, `🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`:

```
dialect grammar
grammar json.snapshot
extension json
start document
string double backslash
```

Optional header directives, declared right after `start` (any order among themselves):

- `string <double|single> <raw|backslash|doubled>` — configures a quote-delimited string mode for
  the shared lexer (M1 item 1). Declaring **any** `string` directive replaces the default
  `"`-only/raw quote set entirely; a grammar wanting both delimiters must declare both.
- `comment none` / `comment line none` / `comment line "<marker>"` / `comment block "<open>" "<close>"`
  — per-grammar comment dialect (M1 item 4). Declaring neither leaves the default `#`-to-EOL
  comment untouched.

Verbatim, `📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`:

```
dialect grammar
grammar stdio.csv.snapshot
extension csv
start document

# RFC 4180 §2: quote-state-aware record/field grammar, matching this artifact's real
# `parse_csv_records` tokenizer (⚙️engine/component.rs:20-69) exactly — not a "hex dump"
# shortcut, not the pre-Phase-2 ABNF placeholder this file used to contain.
comment none
string double doubled
```

`comment none` disables both comment forms (RFC 4180 field data may legally contain a literal `#`).
`string double doubled` turns on `""`-doubling decode for every `"..."` token (csv's own escape
convention — NOT backslash).

### 1.2 Productions, alternation, grouping, terminals

`name = alt | alt | ...` — one production per **one physical line** (see §3.1 pitfall #4).  A
`Symbol` is one of:

- `Literal("text")` — a quoted terminal, matched against the shared lexer's `Text`/keyword tokens.
- `Terminal(ALLCAPS)` — a lexer token class: `IDENT`, `INT`, `FLOAT`, `TEXT`, `BOOL`, plus M1's new
  `LINE`, `REST`, `DOTENUM`, and the promoted single-char tokens `LT`/`GT`/`AMP`/`DOLLAR`/`SEMICOLON`.
- `Ref(lowername)` — a reference to another production (or, if no production of that name exists,
  a framework **macro** — see `hex` below).
- `Group{alt | alt}` — grouping, **always** `{ }`, never bare `( )` (see §3.1 pitfall #1).
- Postfix `?` `*` `+` on any symbol.

Grouping example, verbatim json snapshot grammar:

```
value = object | array | string | number | "true" | "false" | "null"
object = "{" "}" | "{" member {"," member}* "}"
```

### 1.3 M1's new capabilities, with real excerpts

**(a) String escape modes** (`StringEscape::{Raw,Backslash,Doubled}`, `🔍️lexer/🦀️component.rs`
new `//#region 🔖️Dialect`) — `string double backslash` (json, RFC8259 `\" \\ \/ \b \f \n \r \t
\uXXXX` incl. surrogate pairs) vs. `string double doubled` (csv, `""` → one literal `"`). STEP/IFC
(not yet a FG-wave, cited from M1's own report as the intended future use) would declare
`string single doubled` for Part-21's `''`-doubling.

**(b) Raw-span terminals `LINE`/`REST`** — `Symbol::Terminal("LINE")` captures rest-of-physical-line
verbatim from the ORIGINAL source text (past whatever the shared lexer already fragmented into
tokens); `Symbol::Terminal("REST")` captures rest-of-EOF. Verbatim,
`📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`:

```
dialect grammar
grammar stdio.txt.snapshot
extension txt
start document
comment none

document = envelope-mark body
envelope-mark = "stdio.txt"
body = REST
```
The whole remaining document, verbatim, is the one primitive built for exactly this whole-body
capture case; `obj`'s `o`/`g` names and `stl`'s `solid <name>` are the FG-wave-future analogues
M1's own report names for `LINE`.

**(c) Promoted single-char tokens `< > & $ ;`** — unconditional lexer-alphabet extension (every
grammar gets these; no header directive needed), reachable as ordinary terminal names `LT`/`GT`/
`AMP`/`DOLLAR`/`SEMICOLON`. None of the 6 P1-P3 pilots needed these (they serve xml/svg's `<>&`,
step/dxf's `$`/`;`) — cited from `p2-m1-report.md`'s own worked test
(`promoted_tokens_are_real_terminals_the_recognizer_can_require_positionally`,
`🗣️dsl/📖️grammar/🦀️component.rs`'s test module):
```
tag = LT IDENT GT AMP IDENT SEMICOLON DOLLAR IDENT
```

**(d) Per-grammar comment dialect** — `comment none` (csv, txt — see §1.1) is real and pilot-proven;
`comment block "/*" "*/"` + `comment line none` (STEP/IFC's `#`-as-entity-ref-vs-comment fix) is
not yet exercised by any pilot — cited from `p2-m1-report.md`'s own worked STEP-shaped example:
```
comment line none
comment block "/*" "*/"
string single doubled
```

**(e) Trailing-dot floats / leading-dot enum literals** — `0.`/`10.` now lex as `FLOAT`; `.T.`/
`.UNSPECIFIED.` lex as one `DOTENUM` token. Not yet exercised by a pilot (STEP/IFC-future); cited
from `p2-m1-report.md`'s own worked example:
```
step-value = FLOAT | DOTENUM | INT
```

**(f) The `hex` macro** — the mandatory way to model an open-ended hex-digit-run payload (see §3.1
pitfall #2 for why a hand-rolled `{INT|IDENT}*` production is wrong). Bare `hex` with **no**
matching production — `Symbol::Ref`'s existing production→macro fallback resolves it via
`Recognizer::match_macro_span`, which tries the largest span first and backtracks correctly
(unlike `Symbol::Star`). Verbatim, json's mutations grammar,
`🔣️json/…/🧬️mutations/📝️text/📖️component.grammar.semio`:
```
set-member = "set-member" "path" "=" path "key" "=" hex "value" "=" value
remove-member = "remove-member" "path" "=" path "key" "=" hex
path-segment = "K" "[" hex "]" | "I" "[" INT "]"
snapshot-lit = "[" hex "," value "]"
value = "Z" | "B" "[" INT "]" | "N" "[" hex "]" | "S" "[" hex "]" | "A" "[" arr-item* "]" | "O" "[" obj-item* "]"
```

**(g) `Ref` self-recursion** — already worked, zero Recognizer changes needed (confirmed by a real
3-level-nested test, `p2-m1-report.md` item 6). json's own `value`/`object`/`array` mutual
recursion and the diff grammars' `value-diff`/`array-diff-body`/`object-diff-body` recursion (both
verbatim above/below) are real, pilot-proven exercises of this.

### 1.4 Collection-triple grammars (the copy-pasteable shape for a diff's `removed`/`modified`/`added`)

First landed by csv (`p2-p1-csv-report.md` §2), reused verbatim-in-spirit by every later pilot.
General shape:

```
<collection>-clause = "<collection>" "{" "[" removed-list? "]" ";" "[" modified-list? "]" ";" "[" added-list? "]" "}"
removed-list = <key> {"," <key>}*
modified-list = <collection>-modified {"," <collection>-modified}*
<collection>-modified = <key> ":" <item>-diff
added-list = <collection>-added {"," <collection>-added}*
<collection>-added = <key> ":" <item>-value
```

`<key>` is `INT` for index-keyed collections (csv records, png text-chunks/chunk-order — real
verbatim example below), or hex-encoded `TEXT`/`IDENT` for name-keyed collections (zip entries —
real verbatim example below).

**Index-keyed**, verbatim `📷️png/…/🔺️diff/📝️text/📖️component.grammar.semio`:
```
removed-list = INT {"," INT}*
text-chunks-clause = "text-chunks" "{" "[" removed-list? "]" ";" "[" text-chunk-modified-list? "]" ";" "[" text-chunk-added-list? "]" "}"
text-chunk-modified-list = text-chunk-modified {"," text-chunk-modified}*
text-chunk-modified = INT ":" text-chunk-diff-value
text-chunk-added-list = text-chunk-added {"," text-chunk-added}*
text-chunk-added = INT ":" text-chunk-value
```

**Name-keyed**, verbatim `🎒️zip/…/🔺️diff/📝️text/📖️component.grammar.semio`:
```
entries-part = "entries" "{" "[" removed-list "]" ";" "[" modified-list "]" ";" "[" added-list "]" "}"
removed-list = {hex {"," hex}*}?
modified-list = {modified-item {"," modified-item}*}?
added-list = {added-item {"," added-item}*}?
modified-item = hex ":" entry-diff-body
added-item = INT ":" entry-body
```
(zip's `removed`/`modified` are keyed by the entry's hex-encoded NAME; `added` is keyed by the
FINAL-position `usize` index — matches the S-1 spine's own normative `apply` semantics: "`added`
indices refer to FINAL state.")

**Tri-state (`Option<Option<T>>`) fields** — a uniform `[0]` (unchanged/None) / `[1,<value>]`
(Some) tag, verbatim `📷️png/…/🔺️diff/📝️text/📖️component.grammar.semio`:
```
plte-clause = "plte" "=" plte-diff-opt
plte-diff-opt = "[" "0" "]" | "[" "1" "," "[" removed-list? "]" ";" "[" plte-modified-list? "]" ";" "[" plte-added-list? "]" "]"
trns-clause = "trns" "=" transparency-opt
transparency-opt = "[" "0" "]" | "[" "1" "," transparency-value "]"
```

---

## 2. Protocol-file syntax (the `.protocol.semio` dialect)

### 2.1 Header + framing

```
dialect protocol
protocol <id>
version <n>
schema <schema-id>
start <block-name>
framing magic 0x<hex> | framing record | framing chunked
```

Verbatim, `🔣️json/…/📸️snapshot/💾️binary/📡️component.protocol.semio` (text-native artifact — no
binary structure of its own, payload IS the SEMIO-envelope-unwrapped UTF-8 text):
```
dialect protocol
protocol json.snapshot
version 1
schema stdio.json
start body

framing record
chain payload utf8
```

Verbatim, `📷️png/…/📸️snapshot/💾️binary/📡️component.protocol.semio` (binary-native — real magic,
genuinely byte-checked at walk time, `Framing::Magic` arm):
```
dialect protocol
protocol stdio.png.snapshot
version 1
schema stdio.png
start chunks
framing magic 0x89504E470D0A1A0A
```

**The SEMIO envelope is described ONCE, framework-side — never re-describe it in your own
artifact's protocol file.** Verbatim,
`🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/📡️protocol/📡️component.protocol.semio` (P2-M3
deliverable 3, the framework's own canonical envelope description):
```
dialect protocol
protocol semio.envelope
version 1
schema semio.envelope
start envelope
framing magic 0x8953454D0D0A1A0A
header fixed 4
field token_len u32
segment token Array(u8, Field(token_len))
chain bytes
```
Your own `.protocol.semio` should describe ONLY the post-unwrap payload — model it as if the bytes
you're walking already start right after the envelope's token, matching how
`m5_handcrafted_protocol_conformance`'s own `inner_payload_from_semio_example` `unwrap_binary`s
the `.pack.semio`/`.spr.semio` fixture before handing bytes to your protocol's `walk_protocol`.
Cross-artifact `use` to point at this file directly does **not** work at walk time yet (`use` is
parsed/round-tripped on both sides but never consulted by `walk_protocol` — confirmed unchanged
through M1/M2/M3/every pilot) — inline your own header instead of `use`-ing this one (see binary's
`.spk`-container protocol file in §2.3 for a worked example of "inline honestly" over "pretend
`use` works").

### 2.2 Fields, `Prim` types, blocks

`field <name> <prim>` inside `header fixed N { ... }` / `segment <name>? { ... }` / `record <name>
{ ... }` / an `arm` body. `Prim`:

- `u8 u16 u32 u64 i32 i64 f32 f64` — little-endian (this dialect's original, still-default form).
- `u16be u32be u64be i32be i64be f32be f64be` — **M2 item 2**, always-big-endian siblings, a
  static author-time choice. Verbatim, png's protocol (every IHDR/gAMA/cHRM/pHYs field):
  ```
  arm "IHDR" { width u32be height u32be bit_depth u8 color_type u8 compression u8 filter u8 interlace u8 }
  arm "gAMA" { gamma u32be }
  ```
- `varint` / `zigzag` — LEB128 varint (zigzag decodes identically to varint despite the name —
  no real zigzag decode exists).
- `bytes` / `utf8` — greedy "rest of buffer minus reserved trailing bytes" unless length-prefixed
  via `Array`.
- `fixed(n)` — an opaque n-byte span.
- `Array(inner, Fixed(n) | Varint | Field(name))` — a length-prefixed repeat of one scalar `Prim`
  (see §5's `protocol-array-of-records` gap for what this can't do).
- `Ref(name)` — **unconditionally errors during `walk_protocol`** (parsed into the AST, never
  walked) — see §3.2 pitfall #5 and §5's `protocol-prim-ref-recursion` gap.
- `marker(0xNN)` — **M2 item 1c**, JPG-style forward scan for the next occurrence of a marker
  byte, past an unbounded run of fill bytes; distinct from a fixed-position read.
- `endian { "II"=le "MM"=be }` — **M2 item 6**, TIFF-style runtime-selected endianness: reads
  `key.len()` bytes, matches the table, and mutates `WalkState.big_endian` for every subsequent
  PLAIN (non-`Be`-suffixed) `Prim` read for the rest of the walk. Not yet exercised by a pilot;
  cited from `p2-m2-report.md`'s own worked test:
  ```
  field byte_order endian { "II"=le "MM"=be }
  ```

`Cond` (**M2 item 4**), word-keyword operators (`eq ne lt le gt ge` — not symbolic `==`/`<=`,
deliberately, to avoid a lexer-alphabet change — see `p2-m2-report.md` §4 item 2), guards one
field OR a whole segment:
```
field mask u32 if compression eq 3
segment palette if bits_per_pixel le 8 { colors u8 }
```
Verbatim, `💾️binary/…/🔺️diff/💾️binary/📡️component.protocol.semio` (real `.spk` container, every
segment arm):
```
arm 3 { flags u8 seg_len varint raw_len varint if flags eq 3 payload Array(u8, Field(seg_len)) }
```

Cross-block field-env threading (**M2 item 3**) — `Field(name)`/`Cond{field}` now resolve against
ANY earlier-decoded field, not just the current block. This is what makes zip's `jump central_dir_start
from cd_offset` (below) and the `.spk` container's `if flags eq 3` (guarding a field decoded in a
LATER arm than `flags` itself, within the same arm) legal.

### 2.3 `repeat`/`backward`/`jump` — M2's headline additions, real worked excerpts

**`repeat`** (tag-dispatched, repeats until EOF or a declared sentinel `until` value):
```
repeat <name> {
  tag <prim>                      # discriminator: Fixed(n) ascii tag, u8/u32be numeric tag, marker(0xNN) scan
  length <prim>?                  # optional length/count read
  order tag-first|length-first?   # default tag-first; PNG/GLB need length-first
  trailer <prim>?                 # optional trailer read after each iteration's body
  until <tag>?                    # sentinel — TEXT ("IEND") or int/hex (0x3B) literal
  arm <tag> { field1 ty1 ... }
  arm <tag> { nested <name> <prim> { arm <tag> {...} ... } }   # one further dispatch level, GIF89a-style
}
```
An unrecognized discriminator (no matching `arm`) is auto-skipped exactly `length` declared bytes
— the SAME mechanism PLTE's own honest opaque arm (below) relies on.

Verbatim, PNG's chunk loop (length-first, ASCII 4-byte tag, sentinel `"IEND"`, BE trailer):
```
repeat chunks {
tag fixed 4
length u32be
order length-first
trailer u32be
until "IEND"
arm "IHDR" { width u32be height u32be bit_depth u8 color_type u8 compression u8 filter u8 interlace u8 }
arm "PLTE" { }
arm "gAMA" { gamma u32be }
... (cHRM/sRGB/pHYs/tIME/bKGD/tEXt/zTXt/iTXt/IDAT/IEND arms, then the closing "}", omitted here for length)
```
(`arm "PLTE" { }` is deliberately empty — a genuine content gap, `protocol-repeat-length-not-named`,
§5 — `walk_repeat`'s own length-based auto-skip covers it honestly, same treatment PNG's real
decoder gives a wholly unrecognized ancillary chunk.)

Verbatim, ZIP's local-header loop (tag-first, 4-byte BE-encoded-as-hex-literal magic, sentinel is
the central-directory tag, empty sentinel arm):
```
repeat entries {
  tag fixed 4
  until 0x504B0102
  arm 0x504B0304 {
    version_needed u16
    flags u16
    method u16
    dos_time u16
    dos_date u16
    crc32 u32
    comp_size u32
    uncomp_size u32
    name_len u16
    extra_len u16
    name Array(u8, Field(name_len))
    extra Array(u8, Field(extra_len))
    payload Array(u8, Field(comp_size))
  }
  arm 0x504B0102 { }
}
```

**`backward`** (**M2 item 5a**) — genuinely locates a structure whose start is unknowable except by
finding its end first (ZIP's EOCD, whose preceding comment field is 0-65535 bytes):
```
backward <name> magic 0x<hex> { field1 ty1 ... }
```
Verbatim, ZIP's EOCD block:
```
backward eocd magic 0x504B0506 {
  disk_number u16
  disk_cd u16
  count u16
  total_count u16
  cd_size u32
  cd_offset u32
  comment_len u16
  comment Array(u8, Field(comment_len))
}
```

**`jump`** (**M2 item 5b**) — repositions `pos` to an ABSOLUTE value looked up in the walk-wide
field env (must have been decoded by an earlier block — needs M2 item 3):
```
jump <name> from <field-name> { field1 ty1 ... }
```
Verbatim, ZIP's central-directory entry (jump to `cd_offset`, decoded by the `backward` block
above, then `repeat` the real per-entry metadata):
```
jump central_dir_start from cd_offset { }

repeat central_directory {
  tag fixed 4
  until 0x504B0506
  arm 0x504B0102 {
    version_made_by u16
    version_needed u16
    flags u16
    method u16
    dos_time u16
    dos_date u16
    crc32 u32
    comp_size u32
    uncomp_size u32
    name_len u16
    extra_len u16
    comment_len u16
    disk_start u16
    internal_attrs u16
    external_attrs u32
    local_off u32
    name Array(u8, Field(name_len))
    extra Array(u8, Field(extra_len))
    comment Array(u8, Field(comment_len))
  }
  arm 0x504B0506 { }
}
```
**Once a protocol file declares `backward`/`jump`, `walk_protocol`'s final `pos == bytes.len()`
check is (correctly, per M2's own documented exception) skipped** — assert a sane in-range
`consumed`, not `== len`, in your `protocol_walk_law` (zip's own test does this).

### 2.4 A real `.spk`-container worked example (framework-generic wire, inlined honestly)

Every `#[derive(dsl::DslDiff)]`-derived diff type routes through the FULL `.spk` binary document
container (`store::pack_rt::encode_document`), not a lightweight per-message frame. Cross-artifact
`use` to a shared framework description doesn't work at walk time (§2.1), so every such artifact's
diff protocol file **inlines** the same real magic/header/repeat/footer shape. Verbatim,
`💾️binary/…/🔺️diff/💾️binary/📡️component.protocol.semio` (the framework's own canonical worked
example for this exact container, confirmed against `📖️grammar/🦀️component.rs`'s own
`protocol_parse_print_round_trip_retains_body`/`protocol_parses_rich_struct_enum_segment_forms`
tests, which use the identical magic/header-size/footer-size triple):
```
framing magic 0x8953504B0D0A1A0A
header fixed 24
field version_major u16
field version_minor u16
field required_flags u32
field optional_flags u32
field header_crc32 u32
field reserved fixed 8

repeat segments {
  tag u8
  trailer u32
  until 0
  arm 3 { flags u8 seg_len varint raw_len varint if flags eq 3 payload Array(u8, Field(seg_len)) }
  arm 4 { flags u8 seg_len varint raw_len varint if flags eq 3 payload Array(u8, Field(seg_len)) }
  arm 1 { flags u8 seg_len varint raw_len varint if flags eq 3 payload Array(u8, Field(seg_len)) }
  arm 0 { flags u8 seg_len varint raw_len varint if flags eq 3 payload Array(u8, Field(seg_len)) }
}

footer fixed 84
```
The 84-byte footer stays one opaque fixed trailer — `Block::Footer` has no field-level detail in
this dialect at all (same honest-boundary treatment as PDF's xref / DWG's encrypted sections).
**Any future FG-wave standard whose diff type derives `DslDiff`** (i.e. is a flat struct, no
tri-state/enum fields) will hit this exact same shape — copy this block verbatim, only the magic
inside a real `.pack.semio`/framing header may legitimately differ if the artifact's own document
container differs (it won't, for `.spk`-routed diffs — the container is framework-generic).

### 2.5 The recursive/opaque-tail pattern for op & diff binary frames

When a mutation/diff type is a genuinely recursive, data-carrying enum (json's `JsonMutation`
embedding `JsonValue`), `Prim::Ref` cannot describe the recursive payload — model the real FIXED
header fields precisely, then let the payload be one opaque trailing `chain ... bytes`. This is
the honest boundary, not a shortcut: the Rust `encode_op`/`decode_op` side IS genuinely, fully
recursive and round-trip tested independently. Verbatim,
`🔣️json/…/🧬️mutations/💾️binary/📡️component.protocol.semio`:
```
framing record
header fixed 2
field format u8
field tag u8
chain payload bytes
```
`format u8` + `tag u8` are the two REAL fixed leading fields (`store::OP_BINARY_FORMAT` convention
+ the mutation variant's ordinal), individually, genuinely protocol-walkable — only the recursive
`JsonValue`/`JsonPath` payload is opaque. **Always put the opaque payload LAST** in its containing
arm/field-list, so it can honestly consume "rest of buffer" via bare `bytes` with no length prefix
(csv's/png's diff frames use a length-prefixed `Array(u8, Field(<name>_len))` instead when there
are MULTIPLE opaque payloads in the same frame and only the true last one could safely use bare
`bytes`).

---

## 3. Five documented authoring pitfalls (every one bit a real pilot — read before you draft)

1. **Grouping is always `{ }`, never bare `( )`.** `(` is reserved exclusively for macro-call
   argument lists (`table("rows", row)`-shaped). `member ( "," member )*` parses as a macro call
   `member(...)` to an undefined macro `member` — `expected a symbol, found LParen` or worse, a
   silently-wrong parse. Use `member {"," member}*`. (`p2-p1-fix-report.md` Bug 2; found in json's
   original draft, csv's sibling files were already correct.)

2. **Never model an open-ended hex/opaque payload as a hand-rolled `{INT | IDENT}*` PRODUCTION.**
   `Symbol::Star` is a single greedy pass with **no backtracking** — it silently swallows the next
   literal keyword if that literal happens to tokenize as the same kind (`IDENT`) as the content
   it's matching (e.g. `key=<hex> value=<value>` — `hex`'s own greedy Star eats the literal
   `value` keyword, desyncing everything after it; no parse error, just silently wrong
   recognition). A `<digits>e<digits>` hex run (e.g. hex-encoding `"2.5e10"` → `322e35653130`)
   ALSO lexes as one `FLOAT` token, not alternating `INT`/`IDENT`, breaking `{INT|IDENT}*` a
   second, independent way. **Use the framework's built-in `hex` macro instead** (a bare `hex`
   ident with NO matching production — `Symbol::Ref` already falls back production→macro,
   `Recognizer::match_macro_span` tries the largest span first and backtracks correctly). See
   §1.3(f) for the real syntax. (`p2-p1-fix-report.md` Bug 3 + M-fix; a genuine framework gap
   found and fixed by this same wave, plus two artifact-side authoring mistakes.)

3. **Production names can never collide with the five reserved header keywords**: `extension`,
   `use`, `start`, `comment`, `string` — not just in the header, ANYWHERE in the file.
   `parse_grammar` runs one unified loop for header directives AND productions; a leading ident
   matching one of those five words is ALWAYS parsed as a header directive. json's snapshot
   grammar originally defined `string = TEXT` and failed with a confusing `expected Ident, found
   Equals` — renamed to `json-string`. (`p2-p1-fix-report.md`, "Extra bug found.")

4. **Keep every production on ONE physical line.** `parse_sequence` stops at the first `Newline`
   token — there is no line-continuation syntax. A production wrapped across two source lines for
   readability silently truncates; the next line is then mis-parsed as a new (invalid) production
   (`"expected Ident, found Pipe"` / `"expected Equals, found Question"`). Bit BOTH csv (2
   productions, caught before commit) and png (2 productions, caught by
   `ops_grammar_conformance_law`/`diff_grammar_conformance_law` — png's 17-alternative mutations
   line and 17-optional-clause diff line were long enough to tempt a wrap where json/csv's own
   shorter productions weren't). (`p2-p2-png-report.md` §7's "real bug caught and fixed.")

5. **`Prim::Ref` still cannot recurse into nested/enum-valued protocol fields.** `walk_protocol`'s
   `Prim::Ref` arm unconditionally errors — confirmed unchanged through every pilot (json, csv,
   zip, png, binary all independently re-confirmed this by direct read, not assumption). Model the
   nested/recursive payload as an opaque tail past the real fixed header (§2.5's worked example),
   never attempt to describe it field-by-field. This is a structural dialect gap, not a mistake to
   fix locally — see §5's `protocol-prim-ref-recursion` row.

---

## 4. Per-standard deliverable checklist (copy this list into your own FG-wave report)

For **each** standard you own:

- [ ] **Grammar file**, real syntax of the real format (text-native) or an honest hex-dump grammar
  (binary-native — png's own precedent, §1.1's snapshot example) — `📸️snapshot/📝️text/📖️component.grammar.semio`.
- [ ] **Protocol file**, real byte layout (binary-native) OR the pack-container-of-a-text-format
  shape (text-native — `framing record` + `chain ... utf8`, §2.1's json example) — per your
  standard's native-side classification (text-native / binary-native / hybrid, per
  `📖️phase2-design.md`'s own table — confirm which before drafting).
- [ ] **Mutations grammar+protocol**: grammar = the real op-text form (`keyword key=value ...`)
  ALREADY emitted by `print_op`/`parse_op` since F6 — trace it from the real function, never
  guess. Protocol = binary-frame-upgraded `OpBinary` if it was still on F6's
  `print_op().into_bytes()` text-as-binary shortcut (check first — some standards, like zip/binary,
  were ALREADY real; upgrading an already-real codec is a no-op, not a mistake).
- [ ] **Diff grammar+protocol**: same, for `print_diff`/`parse_diff`/`DiffCodec`. Per the P2-W0
  census, **100% of stdio's `DiffCodec` impls were still on the text-as-binary shortcut before
  this pilot ladder** — expect to do a real upgrade here for almost every standard, not just check.
- [ ] **Real fixtures**: `🗣️example.dsl.semio` (genuine `print_dsl` output WITH the mandatory
  `semio stdio.<artifact>.dsl v1` preamble line — never a bare fake like `{"hello":"stdio.xml"}`),
  `🎒️example.pack.semio` (genuine `encode_pack` bytes), optionally `📡️example.spr.semio` (genuine
  `encode_op` bytes — bonus, not blocking, but every future graduation of `ConformanceFacet::ProtocolSpr`
  needs one on disk). Generate via a temporary `[DEBUG]`-prefixed or `#[ignore]`d test that calls
  the REAL Rust encoder directly (never hand-derive independently of the real code path) — run it
  once, copy the bytes, delete the temp test before finishing.
- [ ] **The 6 conformance-law tests**, in the artifact's OWN test region (never a framework file):
  `committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
  `diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`. Copy the shape from
  any pilot's `⚙️engine/🦀️component.rs`'s `conformance_laws` module — every pilot's is
  near-identical in structure, only the demo-case helpers differ.
- [ ] **5-role `LanguageSpec` registration** (`register_pilot_languages()`, §2.1's json exemplar):
  `stdio.<art>` (Document), `stdio.<art>.op` (Ops), `stdio.<art>.diff` (Diff, `protocol: None` —
  the 5-role scheme has no dedicated "diff binary" role even when a real diff protocol file
  exists), `stdio.<art>.pack` (Pack), `stdio.<art>.spr` (Spr). All `dsl::passthrough_hooks`.
- [ ] **`register_schema_spec` calls** — call it for both your snapshot's schema id and its
  `"<schema>#diff"` id **whenever a real `fn() -> RecordSpec` genuinely exists** (i.e. your type
  derives `dsl::DslRecord`/`DslArtifact`/`DslDiff` for real — txt's pilot is the first and so-far
  only real caller). **Do NOT skip registration just because your mutations facet has multiple
  per-variant specs** (binary's own fix: `DslVariants` gives one spec per `BinaryMutation`
  variant, no single canonical id — but the SNAPSHOT and DIFF specs, if real, still each get
  registered individually; only the mutations facet itself has no single id to register under).
  If your types are fully hand-rolled (no derivable `RecordSpec` at all — json/csv/zip/png's own
  situation), skip the call and file it as a `mechanism_gaps` entry rather than fabricate a
  `RecordSpec` that would diverge from what your real hand-rolled codec actually does.
- [ ] **JSON-transfer elimination**: grep your own artifact's `.rs` files for
  `serde_json::to_vec`/`from_slice`/`to_string`/`from_str`/`Value` inside `ArtifactPack`/
  `OpBinary`/`DiffCodec` impl blocks. Every pilot came back clean (none of the 6 were on W0's
  4-violator list) — if yours ISN'T clean (check `POLICY_STDIO_JSON_TRANSFER_BAN`'s current seed —
  it is NOT limited to W0's original 4, see §6), fix it as part of your own wave's scope.
- [ ] **`STDIO_CONFORMANCE_GRADUATED` graduation**: **you do NOT touch this** — it lives in
  `🧪️fixture-sweep/🦀️component.rs`, a framework file no FG-wave agent may edit (see the ticket's
  own repo-rules digest). Land your files + tests genuinely passing; your wave's closer or the
  program's own periodic graduation pass appends your tuple(s).

---

## 5. Known mechanism gaps (consolidated from all 6 pilot reports — recognize these immediately, don't rediscover them)

| gap id | engine area | symptom | hit by | honest workaround |
|---|---|---|---|---|
| `protocol-prim-ref-recursion` | `walk_protocol`, `Prim::Ref` arm | Unconditionally errors — a nested struct/enum-valued protocol field (recursive or not) can't be described field-by-field. | json (JsonValue/JsonPath), csv (nested CsvRecord/CsvSnapshot), png (nested value/tri-state payloads), zip (per-variant record bodies), binary (nested BinarySnapshot inside SetSnapshot) | Model the real FIXED leading header fields precisely; the nested/recursive payload is one opaque trailing `bytes` (last field in its arm) or length-prefixed `Array(u8, Field(<name>_len))` blob when multiple opaque payloads share one frame. Rust encode/decode side stays genuinely, fully structured — round-trip tested independently. |
| `protocol-array-of-records` | `Prim::Array`, `Block::Repeat`'s arm-body grammar | `Array(inner, Count)` repeats one FIXED-WIDTH scalar `Prim`, never a repeated multi-field RECORD of per-item-varying shape; `repeat`'s own arms are tag-dispatched, not a plain "repeat N times, N from a count field" for HOMOGENEOUS untagged records. | zip (per-variant `OpBinary` record bodies; `ZipDiff`'s `entries` collection triple), csv (`csv-nested-record-array-unwalkable` — same root cause, independently named) | Same as `protocol-prim-ref-recursion`'s workaround — opaque trailing tail past the real fixed header. Arguably the general form of the Ref-recursion gap (repeated heterogeneous records vs. self-recursive values). |
| `repeat-cannot-embed-jump` | `Block::Repeat`'s `RepeatArm.fields: Vec<Field>` | ZIP's real per-entry `local_off` backward-jump-to-local-header cross-validation can't be composed with a `repeat` over central-directory entries — an arm body has no `Block::JumpTo` sub-directive. | zip (explicitly latitude-permitted by M2's own report — "the central-directory entries alone already carry the real per-entry metadata") | Model the central directory's own real per-entry fields (incl. `local_off` itself, decoded but not dereferenced); skip the per-entry cross-validation jump. Non-blocking, explicitly plan-permitted. |
| `protocol-cond-cannot-chain` | `eval_cond`/`walk_fields` | `Cond` can gate at most one UNCONDITIONALLY-decoded field; chaining a second `if`-guard onto a field that was ITSELF only conditionally decoded makes `eval_cond` hard-error ("condition references unknown field"). | png (`tRNS`/`bKGD`, whose real shape depends on `color_type` — a genuinely new gap json/csv's simpler diff shapes never had reason to hit) | Model as one honest opaque arm (real Rust decoder still fully types it — protocol-DESCRIPTION depth limit only). A diff facet with the SAME tri-state-inside-tag-dispatch shape should use a single flat 3-way flag (§1.4's tri-state pattern) instead of chained `Cond`s, specifically to avoid this wall. |
| `protocol-repeat-length-not-named` | `walk_repeat`'s `DispatchOrder::LengthFirst` read | A `repeat` block's own `length` directive value is used internally for auto-skip/overrun but never bound to a named field in `WalkState.env` — an arm's fields can't `Array(_, Field(name))` against "the declared length of THIS chunk." | png (`PLTE`'s `length / 3` RGB-entry count) | One honest opaque arm, auto-skipped via the SAME length-based mechanism a genuinely unrecognized chunk type gets — just explicitly named for self-documentation. |
| `register-schema-spec-needs-recordspec` / `-one-spec-per-artifact` | `dsl::registry::register_schema_spec`/`FullResolver` | Requires `fn() -> RecordSpec`; hand-rolled types (no `DslField`-derivable shape) have none at all (json/csv/zip/png). Conversely, binary's types DO have real specs but THREE independent ones (snapshot/per-mutation-variant/diff) with no single-id API shape to register a mutations facet under. | json, csv, zip, png (no spec exists); binary (too many specs, API expects exactly one) | Skip the call, file as `mechanism_gaps` — never fabricate an unrelated `RecordSpec` just to satisfy the API. txt is the one pilot where a real, single spec genuinely exists per facet — call it there. |
| `csv-newline-trivia` | grammar/lexer | `Newline` is always lexer trivia — no `NEWLINE` terminal exists to structurally delimit line/record-oriented formats. | csv (record boundaries) | `record = field {"," field}*` naturally stops at a record boundary because no `COMMA` token bridges one record's last field to the next record's first — proven against real fixtures. Would NOT generalize to a format needing an EXPLICIT line-boundary token for disambiguation. |
| `csv-quoted-field-embedded-newline` | lexer, `StringEscape::Doubled`/`Backslash` scanners | A raw `\n` mid-string is treated as unterminated even in forgiving mode — a quoted field containing a LITERAL embedded newline (legal per RFC 4180 §2 rule 5) is not correctly tokenized as one `TEXT` token. | csv | This artifact's own grammar fixture deliberately avoids that specific input shape (documented, not silently papered over); a genuine multi-line-aware string mode needs a lexer change outside any single artifact's ownership boundary. |
| `txt-opbinary-record-body-wire-is-framework-generic` | protocol/pack | Past `OpBinary`'s own `format`/`ordinal` header, the remaining bytes are `os_pack::encode_record_body`'s own wire (varint-prefixed symbol table + self-describing per-field-ID tag+value encoding) — not expressible by `Array`/`Ref`/`repeat`. Shared verbatim by EVERY `#[derive(dsl::DslOps)]` type in the repo, not artifact-specific. | txt (and any future derive-path-`OpBinary` standard) | `format`/`ordinal` genuinely, individually byte-walked; the record-body tail is one opaque `bytes` segment. |
| `txt-diffcodec-spk-container-is-framework-level` | protocol/pack | `#[derive(dsl::DslDiff)]` routes through the FULL `.spk` document container (§2.4) — affects EVERY future `DslDiff`-derived diff type, not just one standard. | txt, binary (both confirmed) | §2.4's worked example is the copy-paste answer — a framework-level `pack_format.protocol.semio` file (M3's own `semio.envelope` precedent) is the RIGHT long-term fix (a future `use`-able shared description once cross-artifact `use` resolution is real) but is out of any single artifact wave's ownership; inline it per-standard until then. |

---

## 6. The M2 exclusions carve-out (restated for FG2's dwg, FG3's pdf — already decided, don't reopen)

Per M2's own explicit scope decision (`p2-m2-report.md`, and the plan's original "opaque segments
only where honestly irreducible" carve-out), these are **out of scope for the protocol dialect
itself**, not bugs to fix locally:

- **DWG ac1024's decrypt+decompress+two-level-compressed-indirection pipeline** (LCG-XOR decrypt
  with position-dependent keys, a bespoke LZ77 decompress, double indirect lookup across two
  independently-decompressed side-tables) — an imperative transform pipeline, not a declarative
  layout description. FG2's dwg agent: model the version sentinel + declare the rest one opaque
  encrypted/compressed segment, documented.
- **PDF/1.7's full random-access indirect-object graph** (xref `/Prev` chains, hybrid streams,
  arbitrary backward jumps to resolve any object) — same category. FG3's pdf agent: model the
  container framing (header, honestly-bounded xref/trailer region located via the SAME
  backward-scan capability M2 built for zip) but do NOT attempt full object-graph resolution in
  the protocol walker — that stays Rust-side, as it already is.
- **Cross-dialect grammar-parsed-value → protocol-field-width parameterization** (PDF 1.7's `/W`
  array selecting xref-row byte width from a text-parsed COS dict; PLY's per-file schema
  selecting its own binary field list from an ASCII header) — deferred. Document these fields as
  bounded-but-schema-external; real semantic resolution stays in Rust, same as every other pilot's
  own opaque-tail treatment.

---

## 7. Verification commands (per-standard, before closing your own FG-wave)

```
cargo test -p semio-s-plugin-stdio --lib "artifacts::<your-artifact>"     # 0 failures, own scope
cargo test -p semio-s-plugin-stdio --lib                                  # whole-crate, classify unrelated churn by file path, don't chase
bun run ./📜️script.ts policy                                              # zero NEW breaches on POLICY_GRAMMAR_PARSEABILITY / POLICY_PROTOCOL_PARSEABILITY / POLICY_FIXTURE_HONESTY / POLICY_LANGUAGE_REGISTRATION / POLICY_STDIO_JSON_TRANSFER_BAN for your standard once you land real files — you do NOT edit these policy rules' allowlists yourself; the ticket's periodic policy-shrink pass removes your standard's entries once it re-runs the census and confirms
```

Real per-artifact conformance (§4's 6 laws) is your OWN early-warning, independent of the
eventual `STDIO_CONFORMANCE_GRADUATED` framework-level gate — trust your own tests, they run
every time regardless of graduation status.
