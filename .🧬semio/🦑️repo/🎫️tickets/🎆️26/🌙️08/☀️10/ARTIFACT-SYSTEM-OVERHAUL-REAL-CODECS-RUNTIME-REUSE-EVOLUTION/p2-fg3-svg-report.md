# P2-FG3 — svg (standard 1.1) — Report

Scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/**`, per the ticket's FG3 svg brief.
Reference docs read in full before drafting: `📖️grammar-recipe.md`, the relevant rows of
`p2-w0-recon-report.md`, and `📰xml`'s own just-landed FG1 grammar/protocol/diff/mutations files
(svg's snapshot is literally an `XmlDocument`, and svg's diff type mirrors `XmlDiff`'s shape
field-for-field per Phase 1's design, so xml's files are the structural template throughout).
`🖼️tiff`'s FG2 diff file was read as the worked reference for a real, non-shortcut recursive binary
frame (§ "binary-frame lesson").

## What changed

### 1. JSON-transfer violation fixed (W0 census row, "svg", "Yes — in scope")
`SvgSnapshot::ArtifactPack::encode_pack_with`/`decode_pack_with`
(`🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`) routed the BINARY pack payload through
`serde_json::to_vec`/`from_slice` wrapped in a real SEMIO envelope — a literal-JSON-disguised-as-
binary violation of `POLICY_STDIO_JSON_TRANSFER_BAN`. Replaced with `write_svg_xml(&self.doc)`/
`parse_svg_xml(text)` (svg's own real wire-text codec, itself `xml_document_to_text`/
`xml_document_from_text`), matching `📰xml`'s own FG1 fix to the identical pattern in its sibling
file. Verified clean by re-grepping `serde_json` across the whole `🎨️svg` tree post-fix (only
doc-comment mentions remain, describing what the code no longer does).

### 2. Grammar files — rewritten from the pre-Phase-2 pseudo-ABNF placeholder to the real dialect
All three (`📸️snapshot/📝️text`, `🔺️diff/📝️text`, `🧬️mutations/📝️text`) previously used the wrong
header dialect (`dialect grammar stdio.svg.snapshot` — no `grammar`/`extension`/`start` lines),
`'...'` single-quote literals, `*OCTET` — none of which are real dialect syntax; two of the three
(`diff`/`mutations`) additionally just said "the text representation IS its JSON serialization,
not reproduced here" — a placeholder, not a real grammar of what the codec actually emits.

- **Snapshot** (`📸️snapshot/📝️text/📖️component.grammar.semio`): real SVG 1.1 (= XML) document
  grammar, restated production-for-production from `📰xml`'s own just-landed FG1 grammar (element/
  attribute/content-node recursion, declaration, doctype, CDATA, comment, PI, entity/char
  references, `:`-qualified names) — cross-artifact `use` doesn't resolve at recognize time yet, so
  restating (not `use`-ing) is the established convention (zip's OPC family, xml/svg's own node
  model precedent). Per this wave's brief, ADDED a documented, deliberately-unreachable-from-`start`
  trailer section modeling the SVG-specific attribute-VALUE micro-grammars this artifact's own Rust
  codec (`NumCursor`/`parse_view_box`/`parse_points`/`parse_transform_list`/`parse_path_data`)
  genuinely structures: `viewBox`'s 4-number list, `points`'s coordinate-pair run, `transform`'s
  function-call list, and `d`'s per-explicit-command-letter arity (all real, structurally accurate
  productions) — with `style`'s free-CSS value left an honest opaque leaf (the real codec never
  types it beyond a naive split) and `d`'s implicit-command-repetition + arc-flag digit-squeeze
  filed as a genuine, pre-existing mechanism gap (the shared lexer has already atomized `108` into
  one `INT` token before any grammar production could ever see it). These trailer productions are
  UNREACHABLE from `start=document` by construction — the outer `attribute = name "=" TEXT`
  production already consumes a whole quoted value as one opaque token, and there is no
  attribute-NAME-conditioned dispatch primitive in this dialect — so they exist purely to document
  the real internal structure, per the brief's own framing.
- **Diff** (`🔺️diff/📝️text/📖️component.grammar.semio`): real one-line `print_diff`/`parse_diff`
  shape (`declaration=`/`doctype=`/`root=` tokens, recursive `E[...]`/`T[...]`/`R[...]`
  `SvgNodeDiff` tree, name-keyed attrs triple, index-keyed children triple), restated from
  `📰xml`'s own diff grammar (identical shape, since `SvgDiff` mirrors `XmlDiff` field-for-field per
  Phase 1's explicit "svg declares its own diff types, reusing xml's node model" decision).
- **Mutations** (`🧬️mutations/📝️text/📖️component.grammar.semio`): real one-line `print_op`/
  `parse_op` shape for all 11 `SvgMutation` variants (3 more than xml's 8: `SetElementName`/
  `SetViewBox`/`SetTransform`, svg's own flagship-vocabulary additions) — incl. the typed
  `ViewBox`/`TransformOp` op-payload micro-grammars (`enc_view_box`/`enc_transform_list`, which
  print PLAIN decimal literals via `f64::Display`, genuinely different from the attribute-string
  micro-grammars in the snapshot grammar's trailer, and genuinely reachable/real since they ARE the
  op wire format, not an opaque attribute value).

### 3. Protocol files — rewritten from the placeholder to the real binary-frame shape
All three (`📸️snapshot/💾️binary`, `🔺️diff/💾️binary`, `🧬️mutations/💾️binary`) previously used the
same wrong header dialect and a fake `magic = %x00` / `payload = *OCTET` (snapshot) or
`payload = json-utf8` (diff/mutations) placeholder.

- **Snapshot**: `framing record` + `chain payload utf8` (text-native, envelope-unwrapped SVG wire
  text) — same honest boundary xml's/json's own snapshot protocol give their text payload.
- **Diff**: `header fixed 2 { field format u8; field flags u8 }` + `chain payload bytes`, matching
  the REAL upgraded `DiffCodec::encode_diff`/`decode_diff` binary frame (below).
- **Mutations**: same `header fixed 2 { format u8; tag u8 }` + `chain payload bytes` shape, matching
  the REAL upgraded `OpBinary::encode_op`/`decode_op` binary frame (below).

### 4. `DiffCodec` binary upgrade (was F6's `print_diff().into_bytes()` text-as-binary shortcut)
`🔺️diff/🦀️component.rs`: added a full real recursive binary codec mirroring `📰xml`'s own FG1
upgrade to `XmlDiff` (svg's `SvgNodeDiff`/`SvgElementDiff`/`SvgAttributesDiff`/`SvgChildrenDiff`
are structurally identical to xml's counterparts) — `write_bytes_lp`/`read_bytes_lp`/
`write_str_lp`/`read_str_lp` LEB128-varint-framed primitives, `enc_xml_node_bin`/`dec_xml_node_bin`
+ `enc_declaration_bin`/`dec_declaration_bin` (svg's own `pub(crate)` copies, per the established
duplication convention — cross-artifact reuse of xml's `pub(crate)` items isn't available), and the
recursive `enc_node_diff_bin`/`dec_node_diff_bin` + `enc_attrs_diff_bin`/`dec_attrs_diff_bin` +
`enc_children_diff_bin`/`dec_children_diff_bin` tree. `encode_diff`/`decode_diff` now emit/parse a
real `format u8 | flags u8 | [declaration][doctype][root]` frame instead of the printed-text bytes.
Also added `demo_diff_cases()` (the single source of truth now reused by both the local round-trip
test and the engine's new conformance laws), refactoring the pre-existing inline test fixture.

### 5. `OpBinary` binary upgrade (was F6's `print_op().into_bytes()` text-as-binary shortcut)
`🧬️mutations/🦀️component.rs`: added real binary primitives for every `SvgMutation`-specific payload
shape not already covered by the diff module's reused primitives — `enc_node_path_bin`/
`dec_node_path_bin` (`NodePath = Vec<usize>`), `enc_view_box_bin`/`dec_view_box_bin` (4 fixed LE
`f64`), `enc_transform_op_bin`/`dec_transform_op_bin` (6-variant tagged enum) +
`enc_transform_list_bin`/`dec_transform_list_bin`, and `enc_svg_snapshot_bin`/`dec_svg_snapshot_bin`
(mirrors xml's own `enc_xml_snapshot_bin` shape). `encode_op`/`decode_op` now emit/parse a real
`format u8 | tag u8 (0-10) | variant payload` frame. Added `demo_mutation_cases()` (single source
of truth, reused by the local round-trip test and the engine's new conformance laws).

### 6. Real fixtures generated
`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (previously a fake `{"hello":"stdio.svg","n":1}`)
and the previously-nonexistent `🎒️example.pack.semio` were generated by a temporary `[DEBUG]`/
`#[ignore]`d test in the engine's own test module that called `store::ArtifactDsl::print_dsl`/
`store::ArtifactPack::encode_pack` directly on a new `demo_svg_snapshot()` (declaration, simple
doctype, `:`-qualified `xmlns:xlink` attribute name, entity decode, a self-closing element, CDATA,
comment, and a PI — every real-syntax construct the W0 census row names, mirroring xml's own
`demo_xml_snapshot` construct-for-construct). Bytes were copied from the real test run's stdout
(378 bytes for the pack fixture, verified byte-length-exact), and the temporary test was deleted
before finishing. `📚️examples/🎬️demo/🖼️assets/example.svg` (previously an unrelated
`<note>...</note>` XML placeholder, unused by any Rust code but dishonest) was also replaced with
the same real document body for consistency.

### 7. 5-role `LanguageSpec` registration + 6 conformance-law tests
`⚙️engine/🦀️component.rs`'s `register_pilot_languages()` previously registered only the Document
role. Added the remaining 4 (`stdio.svg.op`/`.diff`/`.pack`/`.spr`, `diff`'s `protocol` slot `None`
per the exemplar's own shape), all `dsl::passthrough_hooks`. Added `demo_svg_snapshot()` and a new
`conformance_laws` submodule with all 6 checklist laws (`committed_facet_files_parse`,
`grammar_conformance_law`, `ops_grammar_conformance_law`, `diff_grammar_conformance_law`,
`protocol_walk_law`, `fixture_honesty_law`), copied structurally from xml's own FG1
`conformance_laws` module. `register_schema_spec` deliberately NOT called (no derivable
`RecordSpec` exists — every codec here is hand-rolled because the persisted `doc: XmlDocument`
field is a data-carrying recursive enum with no `DslField` impl, same root cause documented on
every hand-rolled impl's own doc comment) — filed as a `mechanism_gaps` entry, not worked around.

## Verification

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::svg::"` — **69 passed, 0 failed, 0 ignored**
  (run twice, both clean; includes all 6 new conformance-law tests plus every pre-existing F6-era
  op/diff/mutation-diff/absorb/inverse/field-sweep test, all still green after the binary-frame
  upgrades).
- Whole-crate `cargo test -p semio-s-plugin-stdio --lib`: blocked, both on first attempt and a
  retry, by a compile error **outside this artifact's ownership boundary** —
  `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/…/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/⚙️engine/…`
  references a `📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` fixture file/directory that does
  not yet exist on disk (confirmed by direct `ls`) — a different concurrent FG3-sibling session's
  in-progress `ply` work, matching this ticket's own documented "expect transient compile breaks
  mentioning paths you don't recognize as yours" guidance. Not touched (outside `🎨️svg`'s ownership
  boundary). The isolated `artifacts::svg::` run above was captured cleanly both before AND after
  this `ply` breakage appeared in the shared tree, confirming it is unrelated to any change in this
  report.
- `grep -rn serde_json` across the whole `🎨️svg` tree: zero live-code hits (two remaining hits are
  doc-comment prose describing the fix, not code).
- `grep -rn "print_op().into_bytes()\|print_diff().into_bytes()"` across the whole `🎨️svg` tree:
  zero live-code hits (two remaining hits are doc-comment prose).

## Deviations from the checklist

- `register_schema_spec` not called (see §7) — filed as `mechanism_gaps`, not a deviation from
  intended behavior, matching the checklist's own explicit exception for hand-rolled artifacts.
- The SVG-specific attribute-value micro-grammars (viewBox/points/transform/d) are real,
  structurally accurate productions but are UNREACHABLE from the snapshot grammar's own `start`
  production, for the structural reason documented in §2 and in the grammar file's own trailer
  comment (no attribute-NAME-conditioned dispatch primitive exists in this token-based dialect).
  This is the honest, brief-mandated treatment, not a shortcut — flagged here for visibility.
- `style`'s attribute value and `d`'s implicit-command-repetition/arc-flag-digit-squeeze are
  genuinely NOT representable at the token-grammar layer — filed as `mechanism_gaps`, matching the
  W0 census's own pre-existing characterization of this exact row ("inner mini-languages ... stay
  hand-written regardless").

## mechanism_gaps

| id | engine_area | symptom | blocking |
|---|---|---|---|
| `svg-path-data-implicit-and-flag-squeeze` | grammar dialect / shared lexer | `d`'s implicit command-letter repetition (a bare number run reuses the PREVIOUS command's letter — stateful, not context-free) and the arc command's flag-digit squeeze (`"108 8"` must decompose into flags `1`,`0` then `8` — but `108` is already one atomic `INT` token by the time any grammar production runs) are both genuinely unrepresentable at this token-based dialect layer. | No — the real Rust `parse_path_data` handles both correctly and is independently round-trip tested; only the grammar-level DESCRIPTION of this attribute-value micro-language is bounded. |
| `svg-style-value-untyped` | grammar dialect | `style="..."` attribute values are a naive `;`/`:`-split key/value pair in the real Rust codec (`parse_style_decls`) with the VALUE side left a plain, never-further-typed `String` — there is no fixed grammar to restate without fabricating structure the codec itself doesn't impose. | No — same honest-boundary treatment as the consolidated `csv-quoted-field-embedded-newline`/xml `text-run` gaps. |
| `register-schema-spec-needs-recordspec` | `dsl::registry::register_schema_spec` | Requires `fn() -> RecordSpec`; `SvgSnapshot`/`SvgDiff`/`SvgMutation` have none (all hand-rolled, same root cause as every xml-node-embedding artifact). | No — matches the consolidated gap table's own row for json/csv/zip/png/xml; `register_schema_spec` simply isn't called. |
| `protocol-prim-ref-recursion` (consolidated, re-hit) | `walk_protocol`, `Prim::Ref` | svg's diff/mutations protocol frames hit the same pre-existing gap every other stdio pilot's recursive payload hits — `declaration`/`doctype`/`root` (diff) and the variant payload (mutations) are one opaque trailing `bytes` chain past the real fixed `format`/`flags`-or-`tag` header, per §2.5's worked pattern. | No — the Rust side stays genuinely, fully structured and round-trip tested independently; this is the documented, plan-permitted honest boundary, not a local workaround. |

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` (ArtifactPack JSON fix)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` (rewritten)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio` (rewritten)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio` (rewritten)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio` (rewritten)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` (real binary `DiffCodec` + `demo_diff_cases()`)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` (rewritten)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio` (rewritten)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (real binary `OpBinary` + `demo_mutation_cases()`)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/⚙️engine/🦀️component.rs` (5-role registration, `demo_svg_snapshot()`, `conformance_laws` module)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (real fixture)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` (new, real fixture)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/📚️examples/🎬️demo/🖼️assets/example.svg` (replaced placeholder with real document)

Temporary scratch/log files (this ticket folder, `.txt`): `p2-fg3-svg-check1.txt`,
`p2-fg3-svg-fixture-gen.txt`, `p2-fg3-svg-fixture-gen2.txt`, `p2-fg3-svg-dsl-extract.txt`,
`p2-fg3-svg-testrun1.txt`, `p2-fg3-svg-fullcrate.txt`, `p2-fg3-svg-fullcrate2.txt`,
`p2-fg3-svg-testrun-final.txt` — left in place per the ticket's own "MUST NOT delete temporary
files" rule.
