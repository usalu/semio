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
