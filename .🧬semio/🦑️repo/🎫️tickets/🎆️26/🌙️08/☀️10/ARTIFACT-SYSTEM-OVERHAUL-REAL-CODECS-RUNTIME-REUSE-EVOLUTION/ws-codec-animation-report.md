# W-S Codec Wave — `stdio.semio.animation` (`✳️animation` subset)

Real-codec wave for **semio**'s `animation` subset, following the proven, fully-verified `✳️workflow`
pilot (`ws-codec-workflow-report.md`) and `✳️drawing` wave (`ws-codec-drawing-report.md`, the closest
precedent per the brief — animation has multiple data-carrying tagged enums, `AnimTargetProperty`/
`AnimValue`, like drawing's `PathSegment`/`DrawNode`). Skimmed `ws-codec-brep-report.md`/
`ws-codec-cad-report.md` for the tagged-union codec pattern. Scope: `✳️animation`'s three facets
(snapshot, diff, mutations), plus a new example fixture slug.

**Status: fully verified green, in this session, synchronously — no deferred/unverified claims.**

---

## 1. Derive path vs hand-rolled — what actually happened

Per the brief, the `#[derive(dsl::DslArtifact)]` path was reconsidered now that the 6 shared
`⚙️engine/🧮️geometry` value types (`SemioPoint3`, `SemioQuaternion` among them) all derive
`dsl::DslRecord`. It remains blocked here, for the SAME shape brep's/drawing's own reports
identified (`semio-tagged-enum-heterogeneous-variants-no-dslenum-text-path`): `AnimTargetProperty`
(`Translation`/`Rotation`/`Scale`/`Weights`/`Custom{name}`) and `AnimValue`
(`Scalar{value:f64}`/`Vec3{value:SemioPoint3}`/`Quat{value:SemioQuaternion}`/`Weights{values:Vec<f64>}`)
are data-carrying tagged enums whose variants hold different field sets — even though their own
payload types (`SemioPoint3`/`SemioQuaternion`) are individually derive-ready, no `DslEnum`-over-
heterogeneous-payload mechanism is proven to emit a matching TEXT grammar production set.

**Decision**: hand-rolled `ArtifactDsl`/`ArtifactPack` for the snapshot (replacing the old
hex-of-`serde_json` passthrough entirely), duplicating the value-codec primitives
(`enc_str`/`enc_property`/`enc_target`/`enc_interpolation`/`enc_value`/`enc_keyframe`/`enc_channel`/
`enc_timeline`, all module-private) in `📸️snapshot/🦀️component.rs` — same "duplicate in snapshot,
don't reverse-depend on diff" convention brep's own wave established (diff/mutations depend ON
snapshot's plain types, so snapshot can't import diff's codec functions without a cycle). `🔺️diff`'s
own value codecs (`pub(crate)`) were ALREADY real hand-rolled text pre-wave (confirmed by reading —
not hex-of-JSON), and `🧬️mutations` already imports and reuses them for everything except
`SetSnapshot`.

**One real, non-obvious bug found and fixed while writing the grammar** (not present in any prior
semio wave's report): `AnimTargetProperty::Custom{name}` encoded as `format!("c{}", enc_str(name))`
— a bare `c` glued directly to hex digits with NO separator. The shared lexer's `is_ident_continue`
includes alphanumerics, so `c68656c6c6f` lexes as ONE fused identifier token, not `c` followed by a
separate hex run — a grammar production `"c" hex` could never match a glued token. This is a NEW
variant of the grammar recipe's own pitfall #2 (hex-macro backtracking), hit here at the
lexer-fusion level instead. **Fixed** by changing the wire format to `c:<hex>` (colon separator,
matching the `S:`/`V:`/`Q:`/`W:`-tag convention `AnimValue` already used) in BOTH
`🔺️diff/🦀️component.rs`'s pre-existing `enc_property`/`dec_property` and the newly-duplicated copy
in `📸️snapshot/🦀️component.rs` — confirmed round-trip-safe by `op_text_binary_roundtrip_law` (which
exercises a `Custom` property) and `grammar_conformance_law` (which recognizes the reconstructed
demo body containing `c:6f706163697479`).

---

## 2. Per-facet checklist (recipe §4)

### Snapshot (`📸️snapshot/`)

- [x] **Real text DSL** — `parse_dsl`/`print_dsl` in `📸️snapshot/🦀️component.rs` now produce/consume
  a genuine 2-line structured body: `schema=<hex>`, `timelines=[<timeline>,...]`. Every
  `timeline`/`channel`/`keyframe`/`target`/`value` is real hex/bracket-encoded value text
  (tag-prefixed for the two tagged enums) — never a hex dump of a JSON blob.
- [x] **Real binary pack** — `encode_animation_snapshot_binary`/`decode_animation_snapshot_binary`:
  `format u8` + varint-length-prefixed `schema` UTF-8, then a varint `timelines` count and, per
  timeline, a presence byte + varint-length-prefixed name, a varint `channels` count and per-channel
  real fields (target node string, a real per-variant tag byte for `AnimTargetProperty`, an
  interpolation tag byte, a varint `keyframes` count, and per-keyframe real 8-byte LE `f64` fields
  plus a real per-variant tag byte for `AnimValue`). Replaces the old `serde_json::to_vec`-in-
  envelope shortcut entirely (`store::pack_rt`/`store::ByteReader`, no external crate, no hand-rolled
  varint).
- [x] **Grammar file** (`📸️snapshot/📝️text/📖️component.grammar.semio`) — real dialect syntax
  (`{ }` grouping — n/a here, `[ ]` value grouping, bare `hex` macro, one production per line, tagged
  alternation for `property`/`value`), matching `print_animation_snapshot_body` field-for-field.
- [x] **Protocol file** (`📸️snapshot/💾️binary/📡️component.protocol.semio`) — real `header fixed 1
  {field format u8}` + real bare `segment schema_len varint` / `segment schema_bytes Array(u8,
  Field(schema_len))` (proven bare form, not the braced form), then one honest opaque `chain payload
  bytes` tail for `timelines` (`protocol-array-of-records` gap). The real Rust encode/decode stays
  fully structured past that point.
- [x] `🅰️component.g4`/`🔤️component.ebnf` (text mirrors), `🥋️component.ksy`/`🌶️component.spicy`/
  `🔠️component.abnf` (binary mirrors) — descriptive, same production names, not test-parsed.
- [x] **Fixtures** — `📚️examples/🚶️walk/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio`:
  genuine `print_dsl()`/`encode_pack()` output of `demo_animation_snapshot()` (promoted from the
  pre-existing `full_snapshot()` test fixture — one timeline exercising every `AnimValue` variant
  incl. `Weights`, and every `AnimTargetProperty` kind incl. `Custom`). Generated via a temporary
  `#[test] #[ignore] fn animation_temp_print_real_fixtures()` in `🎹️composer/🦀️component.rs` that
  `eprintln!`'d both outputs, run once, bytes extracted with a small Python script (never
  hand-transcribed), temp test then deleted (confirmed absent in the final file).

### Diff (`🔺️diff/`)

- [x] **Text codec already real** — confirmed pre-wave by direct read: `print_semio_animation_diff`/
  `parse_semio_animation_diff` and every entity/value codec (`enc_timeline`/`enc_channel`/
  `enc_keyframe`/`enc_target`/`enc_property`/`enc_interpolation`/`enc_value`) already emitted genuine
  hex/bracket text via the shared `engine::triples::enc_indexed_triple`/`dec_indexed_triple` — ZERO
  `serde_json` anywhere in this file, confirmed by grep (unlike drawing's own diff facet, whose
  STRUCTURE was real but whose LEAF values were still hex-of-JSON — animation's diff had neither
  problem). Only the `enc_property`/`dec_property` `Custom` separator bug (§1 above) needed fixing.
- [x] **Binary upgrade** — was on the F6 `print_diff().into_bytes()` text-as-binary shortcut
  (confirmed pre-wave). Now: `format u8` + `presence u8` (bit0=`timelines` — the only collection this
  facet has, since `SemioAnimationDiff` has exactly one top-level field) as two real fixed header
  fields, then one opaque `payload` chain (the same `enc_indexed_triple`-produced text this facet's
  own `print_diff` already emits) present only when the bit is set — no length prefix needed since it
  is the last, and only, field in the frame.
- [x] Grammar (`🔺️diff/📝️text/📖️component.grammar.semio`) — real dialect syntax. UNLIKE workflow's/
  brep's/drawing's own diff grammars (flat one-field-per-line, each field itself tri-state), this
  facet's per-item diffs are SPARSE tag-lists (`enc_timeline_diff`/`enc_channel_diff`/
  `enc_keyframe_diff` print only the fields that actually changed, as `TAG:value` entries inside a
  bracket-wrapped comma-joined list) — modeled as an alternation of tagged-field productions repeated
  0-or-more times inside brackets (`timeline-diff-field = "N" ":" option-name | "C" ":" "[" ... "]"`,
  same shape at the channel/keyframe nesting levels). This is a genuinely NEW grammar shape relative
  to every prior semio wave's diff grammar (none of workflow/brep/drawing had a sparse-tag-list diff
  encoding) — double-bracket nesting (`"C" ":" "[" triple-body "]"` where `triple-body` is itself
  `enc_indexed_triple`'s unwrapped `[r];[m];[a]`) was worked out by tracing `enc_channel_diff`'s exact
  `format!("K:[{}]", enc_indexed_triple(...))` call site character-by-character, not guessed.
- [x] Protocol (`🔺️diff/💾️binary/📡️component.protocol.semio`) — `header fixed 2 {format, presence}`
  + `chain payload bytes`.
- [x] g4/ebnf/ksy/spicy/abnf mirrors.
- [x] `demo_diff_cases()` (`#[cfg(test)] pub(crate) fn`, module scope) added — `kf`/`channel`/
  `timeline` helpers (pre-existing, `mod tests`-local) promoted to module-scope `#[cfg(test)]
  pub(crate) fn` (a private item of a child `mod tests` isn't visible to the sibling `composer`
  module), reused by both this facet's own `diff_codec_text_binary_roundtrip_law` test (refactored to
  call `demo_diff_cases()` instead of duplicating its own `a`/`b` construction) AND the composer's
  `diff_grammar_conformance_law`/`protocol_walk_law`.

### Mutations (`🧬️mutations/`)

- [x] **Real text codec — NOT already fully real, confirmed a genuine policy violation.** Pre-wave,
  `OpText::print_op`'s `SetSnapshot` arm was `format!("S:{}", enc_str(&serde_json::to_string(snapshot)...))`
  — hex-encoded `serde_json`, exactly the pattern the brief's item 4 warned to check for (every OTHER
  variant was already real, keyword-tag text reusing the diff facet's value codecs). Fixed: added
  `enc_animation_snapshot`/`dec_animation_snapshot` (`[hex(schema),[timeline,...]]`, reusing diff's
  `pub(crate)` `enc_timeline`/`dec_timeline`/`enc_list`/`dec_list`/`enc_str`/`dec_str`), `SetSnapshot`
  now calls these — `serde_json` fully removed from this file.
- [x] **Binary upgrade** — was `<Self as OpText>::print_op(self).into_bytes()` (F6 text-as-binary
  shortcut, confirmed pre-wave). Now: `format u8` + `tag u8` (variant ordinal, `OP_KEYWORDS`/
  `variant_ordinal`, 0-12 matching `parse_op`'s own keyword match) as two real fixed fields, then the
  variant's own argument text (i.e. `print_op`'s output with its `TAG:` prefix stripped) as one
  opaque trailing `bytes` chain — reuses the real, tested `print_op`/`parse_op` text codec (one source
  of truth).
- [x] Grammar/protocol/mirrors, same treatment as the sibling facets — grammar traced verbatim from
  `print_op`'s real `format!(...)` call sites (named per-variant productions, e.g. `insert-timeline =
  "IT" ":" index "," timeline`, mirroring brep's own named-production style rather than one giant
  inline alternation); a new `snapshot-lit`/`enc_animation_snapshot`/`dec_animation_snapshot`
  production+function pair for `SetSnapshot`'s whole-snapshot payload.
- [x] Promoted `base_snapshot()`/`all_variants(&base)` (pre-existing test-local helpers) into
  module-scope `#[cfg(test)] fn fixture()` + `pub(crate) fn demo_mutation_cases()` (renamed to match
  brep's/workflow's own convention exactly — `demo_mutation_cases` takes no args, bakes its own
  `fixture()` internally), reused by this facet's own 3 tests AND the composer's
  `ops_grammar_conformance_law`/`protocol_walk_law`.

### Conformance-law tests

All 6 (`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
`diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`) added to animation's
`🎹️composer/🦀️component.rs`'s existing `#[cfg(test)] mod tests`, in a new nested `mod
conformance_laws` — same home every prior semio wave's report identifies as correct (animation has
no per-standard `⚙️engine/` dir; the shared 14-subset `🏅️standards/🔖️v1/⚙️engine/🦀️component.rs`
aggregator has no test module of its own and is out of this wave's `✳️animation/`-only edit scope).

### Not done (explicit, per brief item 9 / recipe's own instruction)

`LanguageSpec`/`register_schema_spec` registration — skipped, same reasoning as every prior semio
wave: no derivable `RecordSpec` exists for animation's hand-rolled tagged-enum types
(`AnimTargetProperty`/`AnimValue`), and no clear per-subset registration site was found beyond
`🎹️composer::register()` itself. Filed as a follow-up rather than guessed at.

### JSON-transfer ban (checklist item 8)

```
$ grep -n "serde_json" 📸️snapshot/🦀️component.rs 🔺️diff/🦀️component.rs 🧬️mutations/🦀️component.rs
📸️snapshot/🦀️component.rs:151:/// hex-of-`serde_json` passthrough. Duplicated here (not imported from `schema::diff`, which
📸️snapshot/🦀️component.rs:345:/// backing the real `ArtifactPack` below — replaces the old `serde_json::to_vec`-in-envelope
📸️snapshot/🦀️component.rs:559:/// 🎁 Real structured text/binary codecs (animation wave — off the old hex-dump-of-`serde_json`
🧬️mutations/🦀️component.rs:178:/// the old whole-enum `serde_json::to_string`/`from_str` passthrough — a real JSON-transfer-ban
🧬️mutations/🦀️component.rs:199:/// snapshot codec above (W2c closer fix — was `serde_json`, see that region's doc comment).
```
All 5 hits are doc-comment prose describing the OLD, now-replaced shortcuts — zero `serde_json::`
calls remain inside any `ArtifactPack`/`OpBinary`/`DiffCodec` impl body (`🔺️diff/🦀️component.rs` has
ZERO hits at all — its `DiffCodec` was already, and remains, `serde_json`-free).

---

## 3. Exact files touched

All paths relative to repo root, base
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/`.

**Snapshot**: `📸️snapshot/🦀️component.rs`, `📸️snapshot/📝️text/📖️component.grammar.semio`,
`📸️snapshot/📝️text/🅰️component.g4`, `📸️snapshot/📝️text/🔤️component.ebnf`,
`📸️snapshot/💾️binary/📡️component.protocol.semio`, `📸️snapshot/💾️binary/🥋️component.ksy`,
`📸️snapshot/💾️binary/🌶️component.spicy`, `📸️snapshot/💾️binary/🔠️component.abnf`.

**Diff**: `🔺️diff/🦀️component.rs`, `🔺️diff/📝️text/📖️component.grammar.semio`,
`🔺️diff/📝️text/🅰️component.g4`, `🔺️diff/📝️text/🔤️component.ebnf`,
`🔺️diff/💾️binary/📡️component.protocol.semio`, `🔺️diff/💾️binary/🥋️component.ksy`,
`🔺️diff/💾️binary/🌶️component.spicy`, `🔺️diff/💾️binary/🔠️component.abnf`.

**Mutations**: `🧬️mutations/🦀️component.rs`, `🧬️mutations/📝️text/📖️component.grammar.semio`,
`🧬️mutations/📝️text/🅰️component.g4`, `🧬️mutations/📝️text/🔤️component.ebnf`,
`🧬️mutations/💾️binary/📡️component.protocol.semio`, `🧬️mutations/💾️binary/🥋️component.ksy`,
`🧬️mutations/💾️binary/🌶️component.spicy`, `🧬️mutations/💾️binary/🔠️component.abnf`.

**Tests**: `…/✳️animation/🎹️composer/🦀️component.rs` (new `mod conformance_laws` inside its existing
`#[cfg(test)] mod tests`; the fixture-generating temp test was added then removed in the same
session — confirmed absent in the final file).

**New example slug** (outside `✳️animation/`, explicitly permitted by the brief, mirroring workflow's/
brep's/drawing's own precedent — note the pre-existing `📚️examples/🎬️demo` slug is shared/generic
across `🧿️semio` and was confirmed untouched, and other prior waves' slugs `🧊️solid`/`🖍️sketch`/
`🌊️pipeline`/`🕸️graph`/`🏢️building`/`📄️memo`/`📐️drawing`/`🖼️swatch`/`🧊️cube` belong to different
subsets and were confirmed untouched):
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🚶️walk/🦀️component.rs`,
`…/🚶️walk/🟦️component.ts`, `…/🚶️walk/🖼️assets/🗣️example.dsl.semio` (real, byte-verified),
`…/🚶️walk/🖼️assets/🎒️example.pack.semio` (real, byte-verified).

**Explicitly NOT touched, per the brief**: the 4-language schema mirrors (`🔗️component.graphql`,
`🔣️component.json`, `🛰️component.proto`, `🟦️component.ts`) at every level of `🧬️schema/` — these show
as `M` in `git status` but were ALREADY modified before this session started (handwritten earlier in
this ticket, per the brief's own note) and this session made zero edits to any of them, confirmed by
never invoking Write/Edit against any such path. `🧪️fixture-sweep/🦀️component.rs`, `📜️script.ts`,
`📦️glue.rs`, `launch.json`, `catalog.json`, the shared `⚙️engine/🧮️geometry`/`⚙️engine/🧰️triples`
modules, and every other subset were left untouched — confirmed via `git status --porcelain` scoped
to `✳️animation/`/`📚️examples/🚶️walk/` (exactly the files listed above, plus the pre-existing schema
mirror `M`s) and to the rest of `🧿️semio/`/`🗄️stdio/` (all other `M`/`??` entries pre-date or are
concurrent with this session — other agents' in-progress subsets, e.g. `✳️video`, `✳️cad`, per this
repo's own heavy-concurrency ground rules, confirmed by their own `M` status observed mid-session).

**One authoring mistake, caught and cleaned up in-session**: an early `Write` call had a copy-paste
typo in the repo-root path (`🏅️标准` instead of `🏅️standards`), creating a stray one-file directory
tree outside any real module. Caught immediately, the entire stray tree was `rm -rf`'d before any
further work, confirmed via `git status --porcelain` that no trace of it remains.

---

## 4. Mechanism gaps hit

| gap id | recipe row? | what happened here |
|---|---|---|
| `protocol-array-of-records` | yes, §5 | snapshot pack's `timelines` (embedding `channels`/`keyframes`, which further embed two tagged unions). Opaque trailing `chain payload bytes` after the real `format`+`schema` header. |
| `protocol-cond-cannot-chain` | yes, §5 | diff binary's `timelines` — only ONE collection here (unlike brep's 6/workflow's 2), so this gap is somewhat moot in practice, but the same reasoning (a `presence` bitmask + opaque payload, not a chained `Cond`) was applied for consistency with every other semio wave's diff protocol shape. |
| `register-schema-spec-needs-recordspec` | yes, §5 | skipped `register_schema_spec` — no derivable `RecordSpec` (hand-rolled tagged-enum types). |
| `semio-tagged-enum-heterogeneous-variants-no-dslenum-text-path` | no (brep's own, confirmed re-hit) | `AnimTargetProperty`/`AnimValue` are data-carrying tagged enums with heterogeneous per-variant field sets — no derive-path route to a matching TEXT grammar production set. Hand-rolled per brep's/drawing's own established convention. |
| **`lexer-fusion-glued-tag-prefix`** (NEW — not in recipe's table, not in any prior semio wave's report) | no | `AnimTargetProperty::Custom{name}`'s original wire format (`c<hex>`, no separator) glues into ONE lexer token (`is_ident_continue` includes alphanumerics), so no 2-symbol grammar production (`"c" hex`) could ever match it — a real bug in the PRE-EXISTING diff-facet text codec, not merely a grammar-authoring gap. **Fixed** at the codec level (changed the wire format to `c:<hex>`), not worked around in the grammar, since no legal grammar could describe the glued shape. **Recommend**: any future semio subset with a `Custom{name}`/tag-plus-inline-string variant should use an explicit separator (`:`, or a bracket) between the tag letter and any hex/ident-shaped payload from the start, rather than relying on a bracket that happens to follow in every OTHER variant of the same enum. |

---

## 5. Verification — real, not claimed

All commands below were run directly, synchronously, in the foreground in this session, and their
real output was read before writing this report.

1. **`cargo check -p semio-s-plugin-stdio --lib`** → **0 errors**, clean (491 pre-existing warnings,
   none attributable to animation's own files).

2. **`cargo check -p semio-s-plugin-stdio`** (non-`--lib`, per the brief's exact verification
   command) → **0 errors**, `Finished `dev` profile [unoptimized] target(s) in 31.55s`.

3. **`cargo test -p semio-s-plugin-stdio --lib "artifacts::semio::standards::v1::subsets::animation"`**
   → **37 passed, 0 failed, 0 ignored**, including all 6 conformance-law tests individually confirmed
   `ok`: `committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
   `diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`. Two real, in-session
   bugs hit and fixed along the way (first full grammar-conformance run failed):
   - `grammar_conformance_law` failed once on a genuine `artifact-mark` mismatch — the grammar
     originally declared `"stdio.semio.animation"` but this subset's real `envelope_id()` (and
     `#[artifact_schema(id = ...)]`) is `"s.stdio.semio.animation"` (WITH the `s.` prefix — different
     from brep's `"stdio.semio.brep"`, confirmed by direct read of the const). Fixed, re-run green.
   - `fixture_honesty_law` failed on the placeholder `PLACEHOLDER_WILL_BE_REGENERATED_FROM_REAL_
     print_dsl_OUTPUT`/`PLACEHOLDER` fixture text, exactly as expected before fixture generation —
     fixed by the temp-test-then-delete method (§2's snapshot section), re-run green.

4. **`cargo test -p semio-s-plugin-stdio --lib`** (whole crate) → **1922 passed, 1 failed, 3
   ignored** (two consecutive runs, ~20s apart, gave 1921/1 then 1922/1 — the passing count grew
   between runs, confirming a concurrent session actively landing new tests). The 1 failure is
   `artifacts::semio::standards::v1::subsets::video::composer::tests::conformance_laws::
   fixture_honesty_law` — **not this wave's code**: `video`'s own shipped fixture still contains the
   literal `"PLACEHOLDER-REGENERATE-VIA-TEMP-TEST"` string, and `git status --porcelain` confirms
   `…/🪆️subsets/✳️video/…` is `M`-modified (7+ files) by a different, concurrent session mid-way
   through its OWN real-codec wave — the same "concurrent cargo workspace churn" pattern this
   ticket's own environment notes and every prior semio wave's report (workflow/brep/drawing/mesh/
   model/object) has independently hit and correctly attributed. Re-checked once per the brief's own
   instruction ("often self-resolve") — still failing on the second run, but for the SAME reason
   (still mid-wave), not a new/different error, so not chased further. **Zero failures attributable
   to anything in `artifacts::…::animation`.**

**Status: this wave is genuinely proven, fully green for `✳️animation`'s own scope**, with one honest,
explicitly-flagged, pre-existing, out-of-scope failure in a sibling subset (`video`) this session did
not touch and is not responsible for.

---

## 6. Notes for the next semio-subset wave

1. **A sparse tag-list diff shape (this facet's own `AnimTimelineDiff`/`AnimChannelDiff`/
   `AnimKeyframeDiff`) is a genuinely different grammar shape from the flat "every slot present, each
   itself tri-state" shape workflow/brep/drawing's own diff grammars use** — check the ACTUAL
   `enc_*_diff` function bodies for `if let Some(v) = ...  parts.push(format!("TAG:{}", ...))`-style
   conditional pushing before assuming the recipe's §1.4 tri-state-per-field pattern applies; if the
   diff type only prints CHANGED fields (not every field with a `[0]`/`[1,...]` marker), model it as
   an alternation of tagged-field productions repeated inside brackets instead.
2. **A tag-letter immediately followed by a hex/ident-shaped payload with NO separator is a REAL bug,
   not just a grammar-authoring inconvenience** — `is_ident_continue` includes alphanumerics, so
   `c<hex>` fuses into one lexer token. Any `Custom{name}`-shaped enum variant (or similar
   tag-plus-inline-string convention) needs an explicit separator (`:` matches this subset's own
   `AnimValue` tag convention) between the tag and the payload from the start.
3. **Always double-check the real `envelope_id()`/`STDIO_*_DOCUMENT_SCHEMA` constant's exact string**
   before writing the grammar's `artifact-mark` production — it is NOT always the bare
   `"stdio.semio.<subset>"` shape brep/workflow use; this subset's own is `"s.stdio.semio.animation"`
   (leading `s.`), confirmed only by grepping the real const, not assumed from a sibling subset's
   convention.
4. **Checking the mutations facet for a "half-real" `SetSnapshot`/whole-value-payload variant is
   worth doing even when every OTHER variant is already real keyword-tag text** — this subset's
   `print_op` was real for 12 of 13 variants pre-wave; only `SetSnapshot`'s payload (the one variant
   carrying a whole nested aggregate rather than a handful of scalar fields) was still hex-of-JSON.
   Grep the WHOLE match arm list, not just the top-level dispatcher shape.
