# 📓️ stdio-b — native test debt of the second half of `🗄️stdio`

Session 5 (fleet v4), 2026-09-21. Crates: `semio-s-artifact-stdio-{mp3,mp4,obj,pdf,ply,png,pptx,semio,step,stl,svg,tiff,tsv,txt,wav,xlsx,xml,zip}`
plus `semio-s-plugin-stdio` (native `--lib --tests` only). Logs under `🗑️generated/stdio-b/`.

Command used for every run (private target dir, shared build dir):

```
CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-stdio-b CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 \
RUST_MIN_STACK=33554432 DEVELOPER_DIR=/Library/Developer/CommandLineTools \
cargo test -p … --lib --tests --no-fail-fast --features component-app-assembly -- --test-threads=4
```

## Per crate: first run → final

First run = `run3.txt` (the first run that compiled; `run1`/`run2` died on two compile errors, see below).
Final = `run6.txt` (18 artifact crates in one invocation) + `run6-plugin.txt` (`semio-s-plugin-stdio`).

| crate | first run | final | note |
|---|---|---|---|
| stdio-mp3 | ok 40 | **ok 40** | |
| stdio-mp4 | ok 54 | **ok 54** | |
| stdio-obj | ok 53 | **ok 53** | |
| stdio-pdf | compile error | **ok 558** | `LocalizedLabel` `{}` formatting |
| stdio-ply | ok 55 | **ok 55** | |
| stdio-png | ok 150 | **ok 150** | |
| stdio-pptx | FAILED 109/6 | **ok 115** (+1 ignored) | 3 root causes |
| stdio-semio | ok 2755 lib, FAILED 6/1 `brep_analytic_blend` | **ok 2755 + 7** | the 1 red was a wall-clock budget law under fleet load; green in `run6` |
| stdio-step | ok 224 | **ok 224** | |
| stdio-stl | ok 53 | **ok 53** | |
| stdio-svg | FAILED 106/7 | **FAILED 111/2** | 5 root causes fixed, 1 left (below) |
| stdio-tiff | ok 112 | **ok 112** | |
| stdio-tsv | ok 41 | **ok 41** | |
| stdio-txt | ok 70 | **ok 70** | |
| stdio-wav | ok 39 | **ok 39** | |
| stdio-xlsx | FAILED 103/2 | **ok 105** (+1 ignored) | 2 root causes |
| stdio-xml | ok 96 | **ok 96** | compile-broken mid-run by a peer, fixed by that peer |
| stdio-zip | FAILED 82/1 | **ok 83** | |
| semio-s-plugin-stdio | — | **FAILED 5/1** | `descriptor_is_fresh`, owned by `play-stdio` (below) |

17 of 18 artifact crates green; 4 828 artifact tests pass, 2 fail.

## Root causes, grouped

### 1. Peer `LocalizedLabel` sweep left one `{}` format (pdf)
`Mutation::label()` now returns `protocol::LocalizedLabel`, which has no `Display`. One assertion message still
formatted it with `{}`. Fixed with the same resolution siblings use:
`.resolve(protocol::Terminology::Native, protocol::Locale::En)`.

### 2. A decoder's canonical order vs. an index-based test (zip)
`decode_zip` deliberately canonicalizes members to the SAME `name`-ascending order `encode_zip` writes (its own
doc comment: keeping the physical order would put un-round-trippable information into the logical snapshot, and
`deterministic_logical_round_trip` / `fixture_honesty_law` measure exactly that). `decode_rich_synthetic_archive`
still asserted the ARCHIVE's physical order by index. Updated to the current contract: it now pins the canonical
member order explicitly and looks every member up BY NAME for its payload assertions — so it still proves what it
proved (stored+deflate, UTF-8 name, CP437 fallback, data descriptor, ZIP64 sentinel) *and* now also proves the
canonicalization.

### 3. Tri-state `Option<Option<T>>` collapses on decode (pptx)
`PptxRunDiff.font_size` is a tri-state (`None` unchanged / `Some(None)` cleared / `Some(Some(n))` set).
`print_diff` emits `fontSize=null` for `Some(None)` correctly, but the value derive's blanket
`impl<T: FromValue> FromValue for Option<T>` reads `Null` as absence at ANY nesting depth (the framework pins that
collapse deliberately: `🧰️framework/🔨️modules/🌱️value/🔁️codec/🧪️tests/🔬️unit/🦀️.rs::option_collapses_nested_none_like_naive_serde`),
so every `print_diff`/`parse_diff` round trip silently dropped the "clear the font size" edit. Fixed the way the
value derive's own docs prescribe and the way `🧿️semio`'s `🏛️model` mutations already do it — a local
`deserialize_double_option` helper wired with `#[value(deserialize_with = …)]`. Wire format unchanged.

### 4. Fixtures that predate a deliberate encoder/normalizer change (pptx, xlsx, svg)
- **pptx `fixture_honesty_law`**: `parse_dsl(fixture) == demo()` PASSED, only `print_dsl(demo()) == fixture` failed
  — i.e. the fixture is a faithful encoding of the same logical snapshot in an older container form (our zip writer
  now emits the MS OPC growth-hint extra field `0xA220`, the fixture is from 2026-08-12).
- **xlsx `fixture_honesty_law`**: the shared XML writer now wraps attributes at 120 columns
  (`📰️xml/…/📸️snapshot/🦀️.rs::xml_node_to_text`); the 08-12 fixture has the pre-wrap bytes.
- **svg `fixture_honesty_law` / `inference_determinism_law` / `grammar_conformance_law`**: the `.dsl.semio` body is
  still raw `.svg` markup, while `print_dsl` has since become `encode_snapshot`'s structured hex frame.
Each fixture pair was regenerated with **the crate's own printer**, never by hand: xlsx already had
`zzz_write_demo_fixtures`; I added the same `#[ignore]`d writer to pptx and to svg (same shape as docx's
`zzz_write_native_docx_fixture`), and ran all three.

### 5. Hand-transcribed mutation fixtures not in the applied normal form (pptx)
`PptxDiff::apply` ends in `normalize_logical_keys()`, which sorts `content_types.defaults` by key. The committed
`set-snapshot/retitles-and-lowers-the-title-placeholder` quintet carried `[xml, rels]`, so `applies_to_committed_after`,
`committed_diff_applies_to_after` and `inverse_restores_before` all failed on that order alone. The three JSONs
(`⬅️before`, `➡️after`, `🦠️mutation`) now carry the normalized `[rels, xml]`. Nothing else in them changed.

### 6. A JSON pointer missing one segment (pptx)
`committed_diff_is_canonical` asserted `…/shapes/modified/0/kind`; a `NamedModified`/`IndexModified` row nests its
payload under `diff`, so the real path is `…/shapes/modified/0/diff/kind`. The sibling `produces_committed_diff`
(green) already proved the produced diff equals the committed one, so this was purely the pointer. Fixed the pointer.

### 7. A name-keyed collection with no ordering lane (xlsx)
`inverse_law`'s two-hop `SetSnapshot` round trip landed the same MEMBERS in a different sequence, and
`OpcPackage`'s derived `PartialEq` is order-sensitive on `parts: Vec<OpcPart>`. `regenerate_workbook_parts`
already declares path-ascending as this format's part NORMAL FORM (it ends with exactly that sort, added
2026-09-20 for the same class of bug on the codec side) — the apply path just never adopted it. `XlsxDiff::apply`
now lands every applied package in that normal form (parts only; `content_types.overrides` position stays fixed
writer policy, per `sweep_b`'s own note), and the two `sweep_a`/`sweep_b` test fixtures build their parts in it.
Same shape pptx's `normalize_logical_keys` already had.

### 8. A facet law that a JSON-Schema file can never satisfy (svg)
`schema_facets_reject_source_and_raw_doctype_shadow_state` asserts no diff/mutation persistence facet contains
"json". Two of the 20 listed facets ARE JSON-Schema files, whose `$schema` (`json-schema.org`) and `$id`
(`…/text.json`) necessarily say so — while their bodies correctly describe `"Structured tagged/hex text emitted by
SvgDiff::print_diff"`. The law now reads the described codec, skipping those two meta keys.

### 9. A law contradicting its own function's documented contract (svg)
`artifact_dsl_rejects_native_svg_without_a_semio_envelope` required `parse_dsl("<svg …/>")` to be `Err`, but
`parse_dsl`'s own two-branch contract (and its doc comment: refusing it "made every such caller fail on the preamble
check alone") routes envelope-less text to the lossless `import_utf8` branch. Rewritten as
`artifact_dsl_routes_native_svg_by_its_semio_envelope`, which proves the SAME thing the old one was about — the
envelope is what selects the branch: envelope-less markup equals `import_utf8` of the same bytes, and raw markup
UNDER a preamble is still refused.

### 10. A grammar describing the other branch's wire (svg)
`📸️snapshot/📝️text/📖️.grammar.semio` described raw `.svg` markup (what `export_utf8` writes), but
`grammar_conformance_law` recognizes `print_dsl`'s preamble body, which is `encode_snapshot`'s structured hex frame.
Rewritten to describe that frame, restating the same primitives the sibling `🔺️diff` grammar already uses
(`opt-hex`/`opt-bool`/`opt-declaration`/`opt-doctype`/`external-id`/`xml-node`/`attr-item`/`node-item`). The
documentary SVG attribute-value micro-grammars (viewBox/points/transform/path-d) are kept, with their two stale
cross-references to the deleted `attribute = name "=" TEXT` production corrected to `attr-item`.

## Left failing

### `semio-s-artifact-stdio-svg` — `exact_native_analyzer_text_and_pack_roundtrip` + `exact_native_composer_text_and_pack_roundtrip`
Both assert a byte-exact round trip of `temp/artifacts.svg` (an untracked 423 KB real-world file). The only
divergence they reach is the XML declaration's QUOTE STYLE: the source is `<?xml version='1.0' encoding='UTF-8'?>`,
`xml_document_to_text` always writes `"`. `XmlDeclaration { version, encoding, standalone }`
(`🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:102`) has no slot for it, so the
information is lost at import, not at export.

**Why I did not land it**: the honest fix is a `quote` field on `XmlDeclaration` (genuine document state, not the
"source blob" shadow state svg's own facet law forbids). That is a schema-first change across the whole OOXML/XML
family: the xml crate's struct + `xml_document_to_text`/`xml_document_from_text` + its `🔣️.json`/`🟦️.ts`/`🔗️.graphql`/
`🛰️.proto`/grammar/protocol facets and fixtures, svg's `enc_declaration`/`dec_declaration` + grammar + protocol +
fixtures, and every artifact that persists an `XmlDocument` (pptx's `xml_parts`, and docx — owned by `stdio-a`,
which is editing those crates right now). Landing it while two peers sweep that exact tree would break more than it
fixes.

**Next step**: one owner for the whole `XmlDeclaration` quote lane, after the `LocalizedLabel` sweep and
`stdio-a`'s docx work settle. Add `quote: XmlQuote { Double, Single }` with
`#[value(default, skip_serializing_if = …)]` so the wire stays additive, set it in `xml_document_from_text`, honour
it in `xml_document_to_text`, then re-run the `zzz_write_demo_fixtures` writers in svg/pptx/xlsx/docx.
Also worth noting: these two tests read `<repo>/temp/artifacts.svg`, which is **not tracked by git** — a sweep of
`temp/` makes them fail with "read temp/artifacts.svg" instead. That fixture belongs beside the crate.

### `semio-s-plugin-stdio` — `descriptor_is_fresh`
The committed `✏️s/🔌️plugins/🗄️stdio/🛂️.descriptor.semio` (written 11:19 today) no longer byte-matches the native
`describe_plugin()` output. Several agents changed descriptor inputs after it was written: `play-stdio` added eight
playground variants to `📦️packages/🦀️rust/Cargo.toml` and staged a new `🔣️.json` + `🛂️.descriptor.semio` (15:02),
peers rewrote the html/csv/tsv/md/txt/json/xml editors and their manifests (15:02), and my own schema-facet edits
(svg grammar, pptx/xlsx diff schemas) feed the same hashes.

**Why I did not land it**: refreshing it rewrites `🛂️.descriptor.semio` AND `🔣️.json`, and the stdio descriptor
`🔣️.json` is explicitly `play-stdio`'s file in my brief.

**Next step**: `play-stdio` runs `describe` for `🗄️stdio` once and commits both files — after the stdio editor and
schema edits settle. It is a NATIVE `describe_plugin()` comparison, so this does not need the wasm mutex.

### `semio-s-artifact-stdio-semio` — `fillet_of_all_twelve_box_edges_stays_inside_the_interactive_budget` (flaky, green in the final run)
A wall-clock law: three attempts, each held to a 200 ms budget. In `run3` (load average ~50, 15 concurrent cargos)
attempt 0 took 83 ms and attempt 1 took 331 ms — with the fillet itself at 1.0 ms, so the swing is tessellation
losing its core, not the algorithm. It passed in `run6`. Not a defect of this crate; it is the
"timing laws jitter under peer load" class. If the fleet keeps it red, the law needs a CPU-time (not wall-clock)
measurement rather than a looser budget.

## Files changed (absolute)

Production code:
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`

Tests / test fixtures:
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🧪️tests/🔬️codec/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧪️tests/🔬️unit/🦀️.rs` (added `zzz_write_demo_fixtures`)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📸️set-snapshot/🧪️tests/🏷️retitles-and-lowers-the-title-placeholder/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs` (`#[cfg(test)]` sweep fixtures only)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🚪️io/🧪️tests/🔬️unit/🦀️.rs` (added `zzz_write_demo_fixtures`)

Committed fixtures (hand-edited only where stated; the six `.dsl.semio`/`.pack.semio` were written by the crates' own printers):
- `…/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧫️fixtures/🧬️mutations/📸️set-snapshot/🏷️retitles-and-lowers-the-title-placeholder/{📸️snapshot/⬅️before,📸️snapshot/➡️after,🦠️mutation}/🔣️.json` (defaults order only)
- `…/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/📚️examples/🎬️demo/🖼️assets/{🗣️.dsl.semio,🎒️.pack.semio}`
- `…/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/📚️examples/🎬️demo/🖼️assets/{🗣️.dsl.semio,🎒️.pack.semio}`
- `…/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/📚️examples/🎬️demo/🖼️assets/{🗣️.dsl.semio,🎒️.pack.semio}`

(all under `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`)

## Fleet notes worth keeping

- **The shared artifact lock starved the whole fleet.** Between ~14:10 and ~14:50 ten cargos, including the
  coordinator's baseline, sat on `⚡️cache/cargo/target/debug/.cargo-artifact-lock` with zero progress; my first run
  waited 40+ min without compiling a line. A private `CARGO_TARGET_DIR` (build dir still shared) took the same
  18-crate batch to ~7 min. The repo's own `.cargo/config.toml` documents this as supported: "A private
  `CARGO_TARGET_DIR` only diverts the small uplifted deliverables; intermediates stay shared." Mine lives at
  `.🧬semio/🦑️repo/⚡️cache/cargo/target-stdio-b` rather than inside `🗑️generated/`, so a sweep of the ticket's
  generated folder cannot delete it and it stays warm for the next session.
- **Peers broke two of my crates mid-run and fixed them themselves**: `stdio-xml` lost its `semio-framework-job`
  dependency for ~4 minutes (re-added by the peer at 15:02), and `semio-framework-plugin` did not compile at all
  for ~10 minutes around 16:05 (`register_bounded_job_kind` arity, in the no-touch
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`). Both cleared on their own; brief v4's "wait a few
  minutes and re-run" was right both times.
- **Half-applied predecessor edits**: only `🧿️semio` carried any (`🗿️artifacts/🧿️semio/🦀️.rs` close-cursor phase
  reorder to ReturnedLeases → DisplacedOwners + `close_phase_witness`, and a +52-line test
  `returned_read_leases_retire_before_the_displaced_owners_that_alias_them`). They were COMPLETE and correct — the
  framework counterpart `ArtifactStore::close_owned_phase_witness` already exists, and the crate's 2 755 lib tests
  pass with them. Nothing to finish. `🎒️zip` had no pending edit at all, contrary to the handover note.

---

## 2026-09-21 17:30 — `XmlDeclaration` quote lane (coordinator follow-up)

Landed the schema-first `quote` slot the previous section named as the next step. Logs: `run7.txt` (first run
after the code change), `regen2.txt` (fixture writers), `run8.txt`, `run9.txt` (23 crates), `run10.txt` (XML family).

### What landed
- **`XmlQuote { Double, Single }`** in `📰️xml`'s snapshot module, with `as_char`/`from_char`/`is_double`, plus
  `XmlDeclaration.quote` carrying `#[value(default, skip_serializing_if = "XmlQuote::is_double")]` — so the default
  spelling writes **no `quote` key at all** and every pre-existing Value/JSON wire value stays byte-identical.
- **Parser**: `parse_attr_value_quoted` reports the delimiter it consumed; `parse_xml_declaration_prolog` records
  the one `version` used (mandatory and always first). A declaration whose pseudo-attributes disagree with each
  other normalizes to `version`'s — documented in `XmlQuote`'s own doc comment.
- **Writer**: `xml_document_to_text` writes all three pseudo-attributes with that delimiter.
- **Structured codecs**: `enc_declaration`/`dec_declaration` gained a 4th `bool-lit` slot and
  `enc_declaration_bin`/`dec_declaration_bin` a trailing flag byte, in BOTH `📰️xml` and `🎨️svg` (each owns its
  own copy, per their existing convention), with `enc_quote`/`dec_quote` helpers.
- **Facets** (hand-written, no generator exists for these): `declaration-lit` extended in all four
  `📖️.grammar.semio` files (xml snapshot + diff, svg snapshot + diff); `quote` added to 7 JSON schemas across the
  xml and svg trees, to xml's `📸️snapshot/{🔗️.graphql,🛰️.proto,🟦️.ts}` (new `XmlQuote` enum/type + parser guard)
  and to svg's `🔺️diff/{🔗️.graphql,🛰️.proto,🟦️.ts}`. The two `💾️binary/📡️.protocol.semio` files needed no
  change — they already carry the declaration as an opaque tail.
- **New law** `declaration_quote_style_survives_a_parse_write_cycle` in xml's schema unit tests: both spellings
  round-trip byte for byte, and the default still writes `"`.
- **Fixtures** regenerated with each crate's own `zzz_write_*` writer, never by hand: xml and svg
  `📚️examples/🎬️demo/🖼️assets/{🗣️.dsl.semio,🎒️.pack.semio}`.

### Numbers (run9: 23 crates in one invocation; run10: the XML family re-run)
`semio-s-artifact-stdio-xml` **97 passed / 0 failed** (was 94/3 mid-change, 96/0 before it — the net +1 is the new
law). `svg` **111 passed / 2 failed**. `html` **29/0**, `docx` **90/0**, `pptx` **115/0**, `xlsx` **105/0**,
`bcf` **48/0**, `zip` **83/0**, `semio` **2755/0** lib. Across run9, **28 of 29 test targets green**; the only red
target is `svg --lib`. `semio`'s `brep_analytic_blend` flaked red again in run10 under load (green in run6/run9) —
same wall-clock budget law as before, not a defect of the change.

### Still red, and why the quote slot alone cannot close it
`exact_native_analyzer_text_and_pack_roundtrip` / `exact_native_composer_text_and_pack_roundtrip` assert byte
equality against `<repo>/temp/artifacts.svg`, a 423 KB dvisvgm file. I measured the divergences rather than
guessing:

| state of the writer | first differing byte | what differs there |
|---|---|---|
| before this change | 102 | `version="1.0"` vs the file's `version='1.0'` |
| with `quote` landed | 102 | `<svg version="1.1" …>` — ELEMENT attributes, not the declaration |
| probe: element attributes forced to `'` | **37 215** | the file writes `stroke-miterlimit='10' />`, we write `…'10'/>` |

(The probe was a one-line temporary edit to `xml_node_to_text`, measured in `probe.txt` and reverted immediately;
`git diff` of that file now shows only the declaration change.)

So the declaration was one of at least three divergence classes. The second — per-ATTRIBUTE quoting — is arguably
document state too, but it is a 132-literal change across 40 files and it would still not make these two laws pass.
The third — a space before `/>` — is pure serialization formatting, not state, and the same goes for whatever
follows it in the remaining 386 KB. Byte equality with an arbitrary third-party file is achievable only by
retaining the source text, which is exactly the shadow state this artifact's own
`schema_facets_reject_source_and_raw_doctype_shadow_state` law forbids (`source-field`, `artifact-source`).

**This is now a decision about the two laws, not a missing feature.** The three coherent options:
1. **Narrow them to what "lossless" means for a logical model**: assert `import → export → import` is a fixpoint
   (the SNAPSHOT round-trips, which is the real guarantee downstream code needs) instead of byte equality with the
   source. Keeps every bug they have ever caught, including the declaration one this session fixed.
2. **Keep byte equality and give the fixture a writer**: replace `temp/artifacts.svg` with a file this crate's own
   `export_utf8` produced, committed beside the crate. Then byte equality is a real, meetable law.
3. **Model the remaining syntax** (per-attribute quote + empty-element spacing + …) until this one file matches —
   which is retained formatting under another name, and the next third-party file breaks it again.

I recommend 1 combined with 2: the fixture also needs a home either way, because `temp/` is **not tracked by git**
and a sweep of it makes both tests fail with "read temp/artifacts.svg" rather than a real verdict.

### Not verified
`cargo check --target wasm32-wasip2` for the changed crates is **unverified** — my mutex ticket sat behind a
multi-hour peer release build with 9 tickets queued, and the coordinator told me to cancel it rather than block
(my ticket `20260921162137-59304-stdio-b` was removed from `/tmp/semio-wasm-build.queue`; no other ticket touched).
The coordinator's own activation compiles these crates for wasm32. The change adds no `cfg`-gated code and no new
dependency, so the native `--lib --tests` build exercises every line of it.

---

## 2026-09-21 17:55 — svg's native-routing laws restated (coordinator decision)

`semio-s-artifact-stdio-svg` is now **GREEN: 113 passed / 0 failed / 1 ignored** (`run11.txt`, one invocation of
`cargo test -p semio-s-artifact-stdio-svg --lib --tests --no-fail-fast --features component-app-assembly --
--test-threads=4`). Previous run: 111 passed / 2 failed. The +2 are the two restated laws; nothing was removed.

### (1) The fixture has a tracked home
New: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧫️fixtures/🎨️native-third-party-shape.svg`
(~1.5 KB, beside the crate's other `.svg` fixtures). It replaces `<repo>/temp/artifacts.svg`, which git does not
track — a sweep of `temp/` turned both laws into a `read temp/artifacts.svg` panic rather than a verdict.

It is hand-written in a real generator's shape (dvisvgm 3.0.4) rather than an excerpt of that file, so nothing of
unknown provenance enters the tree, and it deliberately keeps every construct that made the old laws disagree with
our writer: single-quoted declaration, single-quoted element attributes, a space before `/>`, an attribute run long
enough to hit the 120-column wrap, entity references (`&amp;`, `&#x2014;`, `&lt;`) in content, a comment, a
processing instruction, and a CDATA body that must survive verbatim.

### (2) The laws now measure what a logical model can promise
`exact_native_analyzer_text_and_pack_roundtrip` / `exact_native_composer_text_and_pack_roundtrip` →
`native_analyzer_text_and_pack_routing_is_a_fixpoint` / `native_composer_text_and_pack_routing_is_a_fixpoint`,
sharing one `assert_native_routing_is_a_fixpoint(route, label)` helper so both routes are held to the same four
assertions:
- **snapshot fixpoint** — `parse(print(parse(x))) == parse(x)`;
- **byte-exact canonical form** — `print(parse(print(x))) == print(parse(x))`;
- **the declaration keeps the source's delimiter** — the printed form still starts with
  `<?xml version='1.0' encoding='UTF-8'?>`, so the one piece of the source's own spelling that IS document state
  is pinned here too, on a real-shaped file rather than only in the unit law;
- **the pack lane is the text lane** — `encode_pack` → re-analyze yields the same snapshot AND the same bytes.

What the old pair proved that this keeps: both routes accept real third-party SVG with zero diagnostics, produce a
snapshot, and survive the pack lane identically. What it drops is only `export_utf8(parse(x)) == x` — a claim about
the SOURCE's formatting, unsatisfiable without the retained source text that this artifact's own
`schema_facets_reject_source_and_raw_doctype_shadow_state` law forbids (evidence and measurements in the previous
section).

### (3) The declaration-quote law stays
`declaration_quote_style_survives_a_parse_write_cycle` in `📰️xml`'s schema unit tests is untouched, and the
`XmlQuote` lane it covers is now also exercised end-to-end by the two restated svg laws above.

### Files changed in this step (absolute)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧫️fixtures/🎨️native-third-party-shape.svg` (new)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🚪️io/🧪️tests/🔬️unit/🦀️.rs`

### Standing state of my scope
All 18 artifact crates green. The only reds left anywhere in my scope are `semio-s-plugin-stdio`'s
`descriptor_is_fresh` (owned by stdio-examples, queued on the mutex) and `stdio-semio`'s wall-clock
`fillet_of_all_twelve_box_edges_stays_inside_the_interactive_budget`, which flakes with fleet load and is green
when the machine is not saturated. `temp/artifacts.svg` is now referenced by nothing in this crate.

---

## 2026-09-21 21:20 — feature-gated `XmlDeclaration` literals (urgent follow-up)

The play activation's wasm-dev build hit `error[E0063]: missing field 'quote'` in `semio-s-artifact-stdio-semio`.
Cause: my earlier sweep fixed every site the fleet's standard command compiles (`--features
component-app-assembly`), but `🧿️semio`'s value⇄xml conversion lane is behind `conversion-*` features that neither
the baseline script nor any fleet command passes, so those literals were never type-checked natively.

### The recurrence guard
`XmlDeclaration::new(version, encoding, standalone)` now exists in `📰️xml`'s snapshot module and sets
`quote: XmlQuote::Double`. Every call site that MINTS a declaration goes through it; only a reader that recovered a
real delimiter builds the struct directly. Adding a further modeled facet of the declaration can no longer silently
miss a literal.

### Sites fixed (5; the coordinator's list was partly stale — `🎒️zip/📦️opc/🦀️.rs:229,362` and svg
`🧬️schema/🦀️.rs:681` were already covered in the earlier pass)
- `…/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📰️xml/🔖️1.0/✳️any/🦀️.rs:150` ← **the wasm-dev break**
- `…/🔢️value/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📰️xml/🔖️1.0/✳️any/🧪️tests/🔬️unit/🦀️.rs:29`
- `…/🔢️value/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📰️xml/🔖️1.0/✳️any/🧪️tests/🔬️unit/🦀️.rs:9`
- `…/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧪️tests/🎨️mutate-svg-1-1/🦀️.rs:194`
- `…/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧪️tests/📰️mutate-xml-1-0/🦀️.rs:261`

A repo-wide re-grep of `XmlDeclaration {` over `✏️s` + `🧰️framework` now reports **zero** initializers without
`quote` or `..Default::default()`.

### Numbers
| command | result |
|---|---|
| `cargo test -p semio-s-artifact-stdio-semio --lib --tests --no-fail-fast --all-features` (`run12-semio.txt`) | lib **3032 passed / 2 failed / 1 ignored** (415 s); `brep_analytic_blend` 6/1; `extrude-orientation` 12/0, `procedural-example-booleans` 2/0, `shell-orientation` 2/0, `tessellation-jobs` 10/0, flow binary 3/0 |
| `cargo test -p …-zip -p …-svg -p …-xml --lib --tests --no-fail-fast --features component-app-assembly` (`run12-family.txt`) | **zip 83/0, svg 113/0 (+1 ignored), xml 97/0 (+1 ignored) — all green** |
| `cargo check -p semio-s-artifact-stdio-semio --all-features` (`check-semio-all.txt`) | **clean, 0 errors** |
| `cargo check -p semio-s-plugin-stdio --lib --all-features` (`check-plugin-all.txt`) | **clean, 0 errors** |

`--all-features` raises stdio-semio's lib suite from 2 755 to **3 032** tests: 277 tests in the `conversion-*`
lanes had never been compiled, let alone run, by any fleet command.

### The 2 lib failures are NOT from the quote lane
Both are pre-existing debt in `conversion-*` lanes that `--all-features` surfaced for the first time. Neither
touches `XmlDeclaration`, and neither is in a file I changed:
- `standards::v1::subsets::brep::io::…::fixture_honesty_law` — `parse shipped .dsl.semio fixture: "vertex:
  expected 3 fields, got 2"`. A brep VERTEX codec/fixture arity drift; the brep lane has no XML in it.
- `standards::v1::subsets::presentation::io::…::ops_grammar_conformance_law` — the presentation mutations grammar
  does not recognize its own real `print_op` output for `SetTextBoxBlocks` (`blocks=[P[[0],[…]],H[1,[1,73],[]]]`).
  These are semio's own `Paragraph`/`DocRun`/`RunStyle` types, not pptx.
Evidence that they are newly surfaced rather than newly broken: `🧪️baseline-plugin-tests.py` passes only
`--features component-app-assembly`, so no baseline or fleet run has ever compiled these two lanes. They want the
same treatment as this session's other two classes — regenerate the brep fixture with the crate's own printer once
the vertex arity is settled, and bring the presentation mutations grammar up to the codec it describes. I did not
touch them: they are outside the follow-up's scope and outside the quote lane.

### Peer interference during this step
`cargo check -p semio-s-artifact-stdio-semio --all-features` first failed on a peer's in-flight edit —
`semio-framework-ui` missing `TreePresentation` in `ui_contract` (3 errors in
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs`), nothing of mine. Re-ran on a loop per brief v4;
it cleared and the check is now clean. The numbers above are from the clean run.

---

## 2026-09-22 04:10 — the two `--all-features` reds closed (successor session)

Successor after a coordinator restart. Both reds the previous section left open are GREEN; the run is
`🗑️generated/stdio-b/run13.txt` (`cargo test -p semio-s-artifact-stdio-semio --lib --tests
--no-fail-fast --all-features -- --test-threads=4`, one hold of the fleet's new
`📜️native-test-mutex.sh`, private `CARGO_TARGET_DIR` under `🗑️generated/stdio-b/target`).

### Numbers

| target | before (`run12-semio.txt`, 09-21 21:20) | now (`run13.txt`) |
|---|---|---|
| `--lib` | 3032 passed / **2 failed** / 1 ignored | **3033 passed / 1 failed / 2 ignored** |
| `brep_analytic_blend` | 6/1 (wall-clock flake) | **7 / 0** |
| `brep_extrude_orientation` | 12/0 | 12 / 0 |
| `brep_procedural_example_booleans` | 2/0 | 2 / 0 |
| `brep_shell_orientation` | 2/0 | 2 / 0 |
| `brep_tessellation_jobs` | 10/0 | 10 / 0 |
| `flow_retained_decode` | 3/0 | 3 / 0 |

`…brep::io::…::fixture_honesty_law` **ok**, `…presentation::io::…::ops_grammar_conformance_law` **ok**
(both named explicitly in the log). The +1 ignored is the new brep fixture writer.

### Half-applied predecessor edits (checked first, as instructed)
The worktree was clean; the auto-committer had STAGED five files nobody had run yet — the presentation
keyword rename in 4 facet files and the brep `zzz_write_demo_fixtures`. Both were correct in substance
and are what this session verified; the rename was also INCOMPLETE (below).

### 1. brep `fixture_honesty_law` — the fixture was the stale side
`parse shipped .dsl.semio fixture: "vertex: expected 3 fields, got 2"`. The shipped `🧊️solid`
`.dsl.semio`/`.pack.semio` are dated 2026-08-12 and predate two waves of real schema:
`BrepVertex.tol`/`BrepEdge.tol`/`BrepFace.tol` and the `coedges=`/`nextLabel=` lines that
`BrepCoedge`/`next_label` added (ticket 26/09/03 BREP-KERNEL wave W3-A).

The CODE is right, decided from this subset's own printer/parser convention rather than from the
error: the hand-written `📸️snapshot/📝️text/📖️.grammar.semio` — the authoritative spec, and the file
the independent Python oracle was written from — already declares
`vertex = "[" hex "," point3 "," number "]"`, `coedge`, `coedges-line` and `next-label-line`, and its
own `grammar_conformance_law` (the grammar recognising real `print_dsl` output) was already GREEN.
Only the committed bytes lagged. So the fixtures were REGENERATED with the crate's own printer —
never hand-edited — through the `#[ignore]`d `zzz_write_demo_fixtures`
(`cargo test … --lib --all-features -- --ignored zzz_write`, log `regen5.txt`, 1 passed).

Both mounts were written: the artifact ships twice, as `✉️base/📚️examples/🧊️solid/🖼️assets` (what the
law reads, and what `🧊️mutate-semio-brep`'s `asset://🧊️solid/` resolves against) and as this subset's
own `🧊️brep/🖼️assets/🧊️solid` (what `🧊️brep/📚️examples/🧊️solid` includes). They were byte-identical
and both stale; leaving one behind would leave a copy the current parser rejects. The writer now
emits both from one printer run, and says why in its doc comment.

New bytes (443 → 640 text, 537 → 736 pack), e.g.
`vertices=[[7631,[0,0,0],0.0000001],…]` and the two new lines
`coedges=[[636f31,6531,1,~L[[0,0],[1,0]],[0,4],6c31,636f32,636f33],…]`, `nextLabel=100`.

### 2. presentation `ops_grammar_conformance_law` — the grammar was the stale side
`print_presentation_mutation` emits **`set-text-box-blocks`** (the spelling the mutation leaf
`✍️set-text-box-blocks` and its `SemanticDescriptor.kind` carry, which the `dsl::Mutations` derive
pins to `to_kebab("SetTextBoxBlocks")`); four text-facet files still said `set-textbox-blocks`, so
the recognizer rejected that one demo case at its first token. The printer is right — the law and the
printer were left alone and the grammar was corrected.

The predecessor's staged rename covered `📝️text/{🅰️.g4,📖️.grammar.semio,🔤️.ebnf,🟦️.ts}`. A re-grep of
the wrapper-independent primitive (the literal keyword, not the files it had already touched) found
**three more** stale facets, now finished in the same spelling:
`🧬️mutations/🔗️.graphql` and `🧬️mutations/📝️text/🔗️.graphql` (`SET_TEXTBOX_BLOCKS` →
`SET_TEXT_BOX_BLOCKS`) and `🧬️mutations/🛰️.proto` (oneof field `set_textbox_blocks` →
`set_text_box_blocks`; every sibling field is the snake_case of its own message name). Repo-wide, no
production file carries the old spelling any more. `🔣️.json`'s `"const": "setTextBoxBlocks"` is the
camelCase VALUE tag and is correct as it stands.

### The one remaining `--lib` red is a peer's in-flight framework edit, not this crate
`tests::returned_read_leases_retire_before_the_displaced_owners_that_alias_them`
(`🧿️semio/🧪️tests/🔬️unit/🦀️.rs:128`) now fails its SETUP assertion:
`returned_snapshot_read_count()` is 0 where the test expects 2. Cause, located and not touched:
`SnapshotReadLeaseRegistry::try_release_aliased`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:236`) does **not exist in HEAD** (`git show
HEAD:…` finds zero occurrences) — it is an uncommitted peer edit, and the file is `MM` (staged *and*
worktree-modified) right now. It adds a fast path that releases a returned lease immediately whenever
`Arc::strong_count(&slot.owner) != 1`, i.e. whenever the store still holds the snapshot the lease
aliases, so nothing is ever parked in the `returned` queue in the test's ordering (leases dropped
BEFORE the displacing commits). The law itself — returned reads retire before displaced owners — is
unaffected; only its way of manufacturing a parked lease is. It passed for the predecessor at 15:25
and 21:20, so this is new since 21:41.

Per brief v4 ("never revert or fix a peer's in-flight edit") this was left alone. **Next step for
whoever owns that store change**: either the peer keeps the fast path, in which case this test must
take its two leases, run the two `SetSnapshot` commits and the undo FIRST and drop the leases after —
so the slot really is the last alias — or the fast path is wrong and the accounting stays. It cannot
be decided while the file is mid-edit.

### Files changed
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/🧪️tests/🔬️derived-composition-unit/🦀️.rs` (writer now covers both example mounts)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/🧬️mutations/🔗️.graphql`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/🧬️mutations/📝️text/🔗️.graphql`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/🧬️mutations/🛰️.proto`
- Fixtures written by the crate's own printer (4 files):
  `…/🪆️subsets/✉️base/📚️examples/🧊️solid/🖼️assets/{🗣️.dsl.semio,🎒️.pack.semio}` and
  `…/🪆️subsets/🧊️brep/🖼️assets/🧊️solid/{🗣️.dsl.semio,🎒️.pack.semio}`

### Knock-on to flag
`🧪️tests/🧊️mutate-semio-brep`'s Python oracle re-encodes the committed `🧊️solid` pack byte for byte.
It was written from the committed grammar/protocol, which already carry `tol`/`coedges`/`nextLabel`,
but the fixture it reproduces only carries them from now on — that cross-language scenario (a repo
test-host feature, not a cargo `[[test]]`, so it is outside every run above) needs re-running by
whoever owns it. Likewise `semio-s-plugin-stdio`'s `descriptor_is_fresh` stays red and stays owned by
the descriptor lane.

---

## 2026-09-22 16:40 — describe cost, the export catalog, the tsv hex example (session 7)

Successor session 7. `🗑️generated/` was SWEPT at ~16:30 (only `knowledge-children` survived), taking this
topic's STATUS.md and every run log of the day with it — everything below that is still provable is
provable from tracked files, from `🗑️generated/activation/steps/` (rewritten after the sweep) or from
the source tree. Every source edit survived and is listed with its absolute path.

### 0. What the predecessor had already landed (verified on disk before continuing)
`regen4`/`regen5` DID complete (brep `zzz_write_demo_fixtures`, EXIT=0) and `run13` was 3033/1 — the brep
`fixture_honesty_law` and the presentation `ops_grammar_conformance_law` were both green. An entire
**export-catalog lane** had also landed unannounced at 04:41–04:44: `representation_short_id` in
`📇️registry/🧬️contract/🦀️.rs`, gltf's `.glb` representation + runtime capability, and a registry law.
Its cross-plugin effect was already measured in that run: process3d's
`export_brep_out_returns_step_text_structured_payload` **ok**.

### 1. The export catalog stdio publishes (for the engineering agent)

`FormatDescriptor.short_id` is now the representation's FIRST extension without its dot, while `kind_id`
keeps the fully qualified representation identity; `format_descriptor` resolves a row by `kind_id`,
`short_id` or any alias. **28 rows, every short id unique:**

```
avi bcf csv docx dwg dxf glb gltf jpg json las md mp3 mp4 obj pdf ply png pptx step stl svg tiff txt xlsx xml zip zz
```

The mesh lane `MeshExporter::format_kind` needs is complete — `step`, `obj`, `stl`, `ply`, `gltf`, `glb`:

| short id | kind id | mime | extension |
|---|---|---|---|
| `step` | `s.stdio.step.standard.ap214.representation.document` | `model/step` | `.step` |
| `obj` | `s.stdio.obj.standard.3-0.representation.document` | `model/obj` | `.obj` |
| `stl` | `s.stdio.stl.standard.ascii.representation.document` | `model/stl` | `.stl` |
| `ply` | `s.stdio.ply.standard.1-0.representation.document` | `model/ply` | `.ply` |
| `gltf` | `s.stdio.gltf.standard.2-0.representation.document` | `model/gltf+json` | `.gltf` |
| `glb` | `s.stdio.gltf.standard.2-0.representation.binary` | `model/gltf-binary` | `.glb` |

`glb` is the representation gltf's definition gained (it declared NO binary representation and no
representation runtime capability at all). Artifacts that publish NO format row, because they declare no
`category: "representation"` runtime capability: **binary, bmp, epw, gif, html, ifc, tsv**.
Law: `every_published_format_owns_its_short_id_and_the_mesh_lane_is_complete`
(`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧪️tests/🔬️unit/🦀️.rs`).

### 2. `describe` cost — root cause and the fix

**Root cause (source, not machine).** `plugin()` walked the 36 artifact definitions about EIGHT times over:

- `artifact_assemblies()` = `validate_catalog()` (36 × parse+validate through `schema_summary`) +
  36 × `(contribution.assembly)()`, each of which parses and validates again inside
  `definition_from_schema_with_executables` and once more inside `native_codec_executables`;
- and it ran **TWICE**, because `native_codec_factory_receipts()` rebuilt the entire 36-assembly set from
  scratch — only to learn which artifacts carry a runtime declaration — although `plugin()` had just
  handed that exact slice to `artifact_catalog_contribution`;
- plus `validate_catalog()` and `artifact_native_codec_factory_receipts()` in the receipt pass, and three
  loops that re-collected `native_codec_factories()` once per receipt.

≈290 parse+validate rounds over 231 KiB of JSON, each one running `validate`'s `format!`s, `BTreeSet`s and
`ArtifactIdentity::parse` per identity. Free natively. The guest's `describe()` runs in the **owned
interpreter**, where it was the 1 764 s against the 1 800 s epoch.

**Fix (schema-first, byte-identical descriptor output):**
1. `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs` — `validated_source(schema)`
   memoizes parse+validate per `&'static str` ADDRESS (a compiled-in schema's address is its identity);
   all six `source()+validate()` call sites go through it. Public counters
   `artifact_definition_parse_count()` / `artifact_definition_lookup_count()` make the cost measurable.
2. `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs` — `native_codec_factory_receipts_for(assemblies)`
   reuses the caller's assemblies (the second full `artifact_assemblies()` is gone); `native_codec_factories()`
   hoisted out of `validate_native_openable_projection`, `preflight_native_catalog_projection` and
   `native_artifact_catalog`.
3. `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧪️tests/🔬️unit/🦀️.rs` — law
   `assembling_the_component_parses_every_artifact_definition_at_most_once` (a complete `plugin()` after a
   warm `artifact_assemblies()` must re-parse ZERO definitions, and the lookup counter must still show the
   multiplier), plus an `#[ignore]`d `zzz_describe_assembly_cost_report` measurement.
4. `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧪️tests/🔬️catalog-projection-budget/🦀️.rs` —
   the one call site that still passed the old 2-argument `preflight_native_catalog_projection`.

**Result, measured by the coordinator's own chain.** The lib compiled fine throughout (only a `lib test`
target was briefly stale, item 4 above), so the 14:21 `wasm-dev` component carries this fix, and
`🗑️generated/activation/steps/describe-stdio-153446.txt` shows:

```
[describe] owned phase=compile bytes=251648202 elapsed_ms=0
[describe] owned phase=execute fuel=0 elapsed_ms=4736
…
[describe] owned phase=execute fuel=1910888918 elapsed_ms=441040
described stdio (plugin semio:stdio@0.1.0) -> ✏️s/🔌️plugins/🗄️stdio
  (wasm=b0d39c81bc0105cab54c3a3b88c9a7dc3056051a12322dd9dae3d07e809722de descriptor=50e0ce89e8f3ea17…)
```

**441 s of guest execution (1.91e9 fuel) against the 1 800 s epoch — a 4.0× improvement on the 1 764 s
that lost the describe pass three times, and it succeeded on a machine at load ≈100.** Caveat stated
honestly: the 1 764 s figure was measured by peer PZ1 on a quiet machine and there is no pre-fix FUEL
number for stdio to compare against, so the machine-independent half of the evidence is the parse-count
law, not the wall clock.

### 3. `returned_read_leases_retire_before_the_displaced_owners_that_alias_them` — restated

`/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/🔬️unit/🦀️.rs`

The peer's `SnapshotReadLeaseRegistry::try_release_aliased` is settled (engineering agent, kernel 1118/0).
Under it a read handed back while its root is STILL aliased frees its slot on the returning thread — and
the displaced-owner queue owns exactly that root — so nothing ever parks and
`returned_snapshot_read_count() == 2` was asserting the OLD return path, not this law. The law is now
pinned directly and more strongly: the close cursor OPENS in `ReturnedLeases`, a close taken while a lease
is still in the registry must answer `Blocked` and STAY in that phase instead of walking into the
displaced-owner queue, and the same close then converges to `Complete` with its terminal-empty witness.
Verified: `semio-s-artifact-stdio-semio --all-features` **3034 passed / 0 failed / 2 ignored**, plus
`brep_analytic_blend` 7/0, `brep_extrude_orientation` 12/0, `brep_procedural_example_booleans` 2/0,
`brep_shell_orientation` 2/0, `brep_tessellation_jobs` 10/0, `flow_retained_decode` 3/0 — EXIT=0.

### 4. The `stdio-tsv` pane rendered HEX — found live, fixed at the source

Live probe of the 9 stdio panes on :6033 (03:04 activation) at 12:05: 9/9 `data-shell-ready`, 0 console
errors, 0 page errors, 0 refusals, and 8 of the 9 rendering their curated demo. The ninth, `stdio-tsv`,
rendered a single `Column 1` cell containing `6964096e616d650971747909756e69745f7072696365…`.

Root cause: the demo example's `🗣️.dsl.semio` was the W1b scaffold — the HEX TRANSCRIPTION of its own
`🖼️assets/📊️.tsv` ("a trivial hex-encoded instance, matching gif's own demo convention"). `TsvSnapshot::parse_dsl`
finds no `semio stdio.tsv.dsl v1` preamble on such a file, so `decode_tsv` takes the whole hex string as ONE
record with ONE field. No round-trip law could see it: a byte-exact split/rejoin codec round-trips hex
exactly as well as it round-trips real TSV.

Fixed by regenerating the fixture with the crate's OWN printer, never by hand:
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🔬️unit/🦀️.rs`
  — new `zzz_write_demo_fixtures` (writes `🗣️.dsl.semio` + `🎒️.pack.semio` from `📊️.tsv` via `print_dsl`/`encode_pack`)
  and new law `demo_dsl_is_this_subsets_own_printed_table` (body == this crate's printed form, grid == the
  authored file, 6 records × 5 fields, header `id name qty unit_price note`).
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs`
  — `RAW_TSV` (the authored file is the authority) + the module doc rewritten to name the defect.
- Regenerated: `…/📚️examples/🎬️demo/🖼️assets/{🗣️.dsl.semio,🎒️.pack.semio}` — the DSL is now
  `semio stdio.tsv.dsl v1` + the five-row price list. Writer run: `zzz_write_demo_fixtures ... ok`, EXIT=0.

The staged descriptor the browser loads already carries it: the tsv demo's `artifactJson` is 310 bytes of
real DSL, and the staged `🔣️.json` declares **9 examples and 9 `setActiveExample` actions** — one per pane.

### 5. Live verdict on the 15:47 activation (:6033, recycled 15:57)

Headless Chromium, one page at a time, logs + screenshots in
`/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/play-fleet/stdio-b/probe-s7-final/`.
**9/9 `data-shell-ready` in 7–8 s, 0 console errors, 0 page errors, 0 refusals**, every pane on its
curated `Example Demo`:

| pane | visible content | verdict |
|---|---|---|
| `stdio` (md) | Text window: `# Title`, block quote, `**markdown**`, a ```` ```rust ```` fence | correct |
| `stdio-txt` | Text window: `Hello, stdio.txt!` | correct |
| `stdio-csv` | Table: `name / note`, `alpha / plain`, `Doe, John / He said "hi"` | correct |
| `stdio-tsv` | Table, 5 columns: `id name qty unit_price note`, `1 Oak Panel 12 18.50 in stock`, `2 Steel Bracket L-90 48 2.05 backordered`, `3 Glass Pane 4mm 6 44.99 fragile;handle with care`, `4 Cable Tie 200mm 500 0.03 …` | **FIXED** (was one `Column 1` cell of `6964096e616d65…`) |
| `stdio-json` | Tree `{7}`: name/count/ratio/active/missing/tags[3]/nested… | correct |
| `stdio-json-i` | Tree `{5}`: `semio.stdio.i-json.demo`, members[2] | correct |
| `stdio-xml` | Tree: `<catalog>` 2 attrs/5 children, comment, PI, CDATA `raw markup`, `Tom & Jerry` | correct |
| `stdio-xml-valid` | Tree: `<catalog>` → `<item><title>Concrete Forest`, `Hexagonal Cut` | correct |
| `stdio-html` | Text window: `<!DOCTYPE html> … Hello from the <b>semio</b> html demo.` | correct |

The coordinator's strict acceptance on the same activation is 70/70 including all nine stdio panes, and
play's unit suite is 71/71.

### 6. Full batch, green — `s7d`, 2026-09-22 18:18–18:26

ONE `cargo test --no-fail-fast --features component-app-assembly` invocation through
`📜️native-test-mutex.sh stdio-b`, `CARGO_BUILD_JOBS=2`, `CARGO_INCREMENTAL=0`, private
`CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-stdio-b`. Logs (durable, outside `🗑️generated`):
`/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/play-fleet/stdio-b/s7d-family.txt` — **EXIT=0**.

| crate | passed | failed | | crate | passed | failed |
|---|---|---|---|---|---|---|
| stdio-bcf | 48 | 0 | | stdio-png | 150 | 0 |
| stdio-docx | 111 | 0 | | stdio-pptx | 115 | 0 |
| stdio-gltf | 269 | 0 | | stdio-step | 224 | 0 |
| stdio-html | 54 | 0 | | stdio-stl | 53 | 0 |
| stdio-mp3 | 40 | 0 | | stdio-svg | 113 | 0 |
| stdio-mp4 | 54 | 0 | | stdio-tiff | 112 | 0 |
| stdio-obj | 53 | 0 | | stdio-tsv | 42 | 0 |
| stdio-pdf | 558 | 0 | | stdio-txt | 70 | 0 |
| stdio-ply | 55 | 0 | | stdio-wav | 39 | 0 |
| stdio-xlsx | 105 | 0 | | stdio-xml | 97 | 0 |
| stdio-zip | 83 | 0 | | **semio-s-plugin-stdio** | **14** | **0** |
| | | | | **TOTAL (22 crates)** | **2 459** | **0** |

Separately, in the same session and the same private target dir:
`semio-s-artifact-stdio-semio --all-features` **3 034 / 0 / 2 ignored** plus `brep_analytic_blend` 7/0,
`brep_extrude_orientation` 12/0, `brep_procedural_example_booleans` 2/0, `brep_shell_orientation` 2/0,
`brep_tessellation_jobs` 10/0, `flow_retained_decode` 3/0 (EXIT=0); and
`semio-s-artifact-stdio-contract` 2/0 (EXIT=0). **Every stdio-b crate is green.**
`semio-s-plugin-stdio::descriptor_is_fresh` now PASSES — the 15:42 describe closed it.

### 7. Did the source change make describe cheap? — the numbers, and what they do and do not prove

Measured by the new `zzz_describe_assembly_cost_report` (native, `--test-threads=1`, two independent runs):

```
describe assembly: apps=18 lookups=196 parses=36 multiplier=5.4x cold=84.4ms warm=36.0ms
```

- **196 → 36.** One complete `plugin()` — exactly what `describe()` runs — asks for an artifact
  definition **196** times. Before the memo every one of those was a full `pack::from_json_str` + the
  whole of `validate` (a `format!`, a `BTreeSet` and an `ArtifactIdentity::parse` per identity, per
  document). It is now **36**: the parse+validate workload is **5.4× smaller**, and the second complete
  36-definition BUILD that `native_codec_factory_receipts()` used to perform is gone entirely.
- **Natively that work is 57 % of the assembly**: a cold `plugin()` is 84 ms, a fully memoized one 36 ms.
  In the owned interpreter — allocation- and string-heavy code, no JIT — its share is higher, not lower.
- **End to end**: `describe` was 1 764 s on a QUIET machine (peer PZ1, 2026-09-21 22:45) and lost the
  coordinator's describe pass three times. Today it was **441 s of guest execution / 1.91e9 fuel**
  (`🗑️generated/activation/steps/describe-stdio-153446.txt`, 15:34–15:42) — **4.0× faster**, against the
  1 800 s epoch.

**Attribution, stated honestly.** The 15:42 describe ran on the 14:21 `wasm-dev` component, which was
built after every edit of this session (11:15–13:17), so the fix IS in the measured build; and it got
4.0× faster *while the descriptor grew* (the `stdio-examples` topic had meanwhile added 9 curated example
bodies and 9 `setActiveExample` actions, i.e. more describe work, not less). What cannot be claimed is
100 % attribution: there is no pre-fix FUEL number for stdio to compare against — only a wall clock on a
differently-loaded machine — so "the source change removed 5.4× of the dominant workload, and the
end-to-end time fell 4.0×" is the exact claim, not "the source change accounts for all of it".

**The bound law** (`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧪️tests/🔬️unit/🦀️.rs`):
`assembling_the_component_parses_every_artifact_definition_at_most_once` — the process may never have
parsed more than the 36 definitions this crate compiles in, and a complete second assembly (196 lookups)
may not re-parse a single one. It immediately earned its keep: on its first run it read **126** parses of
36 definitions, because `validated_source` released the memo lock before parsing and four test threads
first-touching the same schema each parsed it. `validated_source` now parses WITH the memo locked (the
guest is single-threaded and the documents are tiny, so serializing costs nothing), and the law is an
exact `== 36`.
