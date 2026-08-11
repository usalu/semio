# Wave G — Final Gate Report

Ran every item in the plan's "Verification (end-to-end definition of done)" checklist for real,
fresh, on-disk, from this session. Raw command outputs saved in this ticket folder:
`g-full-crate-test.txt`, `g-framework-check.txt`, `g-policy-run.txt`, `g-nonstdio-check.txt`.

## Headline

**1075 passed, 0 failed, 0 ignored** — `cargo test -p semio-s-plugin-stdio --lib` (fresh run, no
filter, ~8.5s). Matches F6d's own closer-confirmed number exactly; no regression since F6 closed.

All 7 checklist items below: **PASS**, with two precisely-quantified, previously-flagged, correctly
out-of-scope gaps (grammar-leaf honesty, facet-mirror drift) and one real bug found+fixed in a
non-stdio downstream consumer (trinity plugin) during the item-2 spot-check.

---

## 1. `cargo test -p semio-s-plugin-stdio --lib` — PASS

```
test result: ok. 1075 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 8.49s
```

Zero `FAILED` lines anywhere in the output (`grep -n "FAILED\|^error"` → no matches).

### 31-row law-test coverage table

Built by extracting all 1076 `test ...` lines, grouping by `artifacts::<name>::standards::<version>`
prefix (32 prefixes found — the 31 official standards + `ifc::v2x3`, confirmed out of scope, excluded
below), then grepping each prefix's own test names for the 8 law-kinds. Where the literal grep is 0,
the actual test list was inspected for an equivalent-but-differently-named test (permitted by the
brief: "some artifacts may use slightly different but equivalent test names").

| Standard | field_sweep | mutation_diff_law | inverse_law | absorb_law | between_roundtrip_law | codec_retention_law | op_text_binary_roundtrip_law | diff_codec_text_binary_roundtrip_law |
|---|---|---|---|---|---|---|---|---|
| bcf/2.1 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| binary/raw | ✅ | ✅ | ✅ | ✅ | ✅¹ | ✅ | ✅ | ✅ |
| bmp/v3 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| csv/rfc4180 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| deflate/rfc1950 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| docx/ecma-376 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| dwg/ac1018 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| dwg/ac1024 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| dxf/r12 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| gif/87a | ✅ | ✅ | ✅ | ✅ | ✅ | ✅² | ✅ | ✅ |
| gif/89a | ✅ | ✅ | ✅ | ✅ | ✅ | ✅² | ✅ | ✅ |
| gltf/2.0 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| ifc/4 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| jpg/jfif-1.01 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| json/rfc8259 | ✅ | ✅ | ✅ | ✅³ | ✅ | ✅⁴ | ✅ | ✅ |
| las/1.0 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| md/commonmark | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| obj/3.0 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| pdf/1.4 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| pdf/1.7 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅⁵ | ✅ | ✅ |
| ply/1.0 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| png/1.2 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| pptx/ecma-376 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| step/ap214 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| stl/ascii | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| svg/1.1 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| tiff/6.0 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| txt/utf-8 | ✅ | ✅ | ✅ | ✅ | ✅¹ | ✅ | ✅ | ✅ |
| xlsx/ecma-376 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| xml/1.0 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| zip/2.0 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

**31/31 standards have all 8 law-kinds present.** No literal-name gap survived judgment review.
Footnotes (equivalent-name resolutions, verified by reading the actual test):

1. `binary`/`txt`: literal `between_roundtrip_law` is named `between_roundtrip_synthetic` in these
   two artifacts' diff test modules — same law, same assertion (`between(a,b).apply(a)==b`).
2. `gif/87a`, `gif/89a`: no test literally named `codec_retention_law`; the law is covered by each
   standard's own `engine::tests::encode_decode_round_trip_*` / `encode_decode_encode_decode_is_stable`
   suite (7+ tests each) — decode→encode byte-preserving round trips, the law's actual content.
3. `json`: no single test named `absorb_law`; covered by 11 separately-named
   `absorb_array_*`/`absorb_object_*` tests including `absorb_array_associativity` and
   `absorb_object_associativity` — the full canonical-case list from the plan (insert+remove-before,
   insert+insert-same-index, add+setfield, modify+remove) plus associativity, per key kind.
4. `json`: `codec_retention_law` equivalent is `engine::tests::codec_round_trip`.
5. `pdf/1.7`: `codec_retention_law` equivalent is `engine::tests::encode_then_decode_recovers_pages_and_text_via_identity_tounicode`
   plus the sibling engine-level round-trip suite (16 engine tests).

---

## 2. `cargo check -p semio-framework` — PASS, plus a real non-stdio bug found+fixed

```
Checking semio-framework-os-kernel v0.1.0 ...
Checking semio-framework-ui v0.1.0 ...
Checking semio-framework v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/📦️packages/🦀️rust)
Finished `dev` profile [unoptimized] target(s) in 46.34s
```
0 `error` lines, 7 pre-existing `warning` lines (dead-code style, unrelated). Clean.

**Non-stdio `impl ArtifactBuilder` spot-check**: grepped all `impl.*ArtifactBuilder` sites outside
stdio (122 in stdio; largest non-stdio implementors: `norm` 45, `block`/`puzzle` 9 each, `trinity`/
`fem`/`gis`/`procedural` 6 each). Ran `cargo check` on the 4 largest (`semio-s-plugin-norm`,
`semio-s-plugin-block`, `semio-s-plugin-puzzle`, `semio-s-plugin-trinity`):

- **`norm`, `block`, `puzzle`**: each fails with `error: couldn't read
  ".../📄️document/🦀️component.rs": No such file or directory` — a dangling `#[path]` in each
  plugin's OWN `glue.rs` pointing at a `📄️document` directory that doesn't exist (only `📄️artifact`
  exists under `norm`). **Confirmed unrelated to this program**: `git log` on `norm`'s `glue.rs` and
  directory shows its last touching commits are from 2026-06 (`🚩️478-485`), two months before this
  ticket opened (2026-08-10) and completely outside the `🗄️stdio` plugin this program's 31 standards
  live in. Pre-existing, out-of-scope breakage — not fixed, flagged here only for completeness per the
  plan's own "external churn misattribution" risk mitigation (classify, don't chase).
- **`trinity`**: **14 real compile errors, genuinely caused by this program.** `trinity`'s `🔌️jack`
  and `♻️rewrite` artifacts each have a hand-written cross-plugin bridge
  (`🚪️io/📤️export/🧵️serializers` / `📥️import/🧩️deserializers`) that constructs/reads stdio's
  `JsonSnapshot`/`MdSnapshot`/`CsvSnapshot` directly by field name. F1 (json), F3 (md), and F1 (csv)
  changed those snapshots from flat placeholder shapes (`JsonSnapshot{value: serde_json::Value}`,
  `MdSnapshot{body: String}`, `CsvSnapshot{headers, rows}`) to their real typed models
  (`JsonSnapshot{value: JsonValue}` — json's own key-order/lexeme-preserving enum, not
  `serde_json::Value`; `MdSnapshot{blocks: Vec<MdBlock>}`; `CsvSnapshot{has_header, records}`) —
  outside every F-wave agent's `🗿️artifacts/<own-artifact>/**` ownership boundary, so nothing in
  F1-F6 ever saw or fixed trinity's bridge files. **Fixed** (8 files, all in `trinity`, none in
  `stdio`): rewired each bridge to use the real artifacts' own reuse-first primitives instead of
  hand-rolling a converter —
  `semio_s_plugin_stdio::artifacts::json::schema::snapshot::{parse_json_text, write_json_text,
  write_json_pretty}` for the json bridges (text round-trip through json's own RFC8259 codec,
  since `JsonValue`'s `#[serde(tag="kind")]` shape isn't a drop-in `serde_json::Value` substitute),
  `semio_s_plugin_stdio::artifacts::md::engine::{parse_markdown_blocks, render_markdown_blocks}` for
  the md bridges, and typed `CsvRecord`/`CsvField` construction (header record + one data record) for
  jack's csv export. Re-ran `cargo check -p semio-s-plugin-trinity` after the fix: **0 errors**,
  `Finished` cleanly (55 pre-existing warnings, all unrelated dead-code/lifetime style, unchanged).
  Files touched (all under `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/`): `🔌️jack/.../🚪️io/📤️export/.../📝️md/.../🦀️component.rs`,
  `🔌️jack/.../🚪️io/📥️import/.../📝️md/.../🦀️component.rs`,
  `🔌️jack/.../🚪️io/📤️export/.../📊️csv/.../🦀️component.rs`,
  `🔌️jack/.../🚪️io/📤️export/.../🔣️json/.../🦀️component.rs`,
  `🔌️jack/.../🚪️io/📥️import/.../🔣️json/.../🦀️component.rs`,
  `♻️rewrite/.../🚪️io/📤️export/.../📝️md/.../🦀️component.rs`,
  `♻️rewrite/.../🚪️io/📥️import/.../📝️md/.../🦀️component.rs`,
  `♻️rewrite/.../🚪️io/📤️export/.../🔣️json/.../🦀️component.rs`,
  `♻️rewrite/.../🚪️io/📥️import/.../🔣️json/.../🦀️component.rs` (9 files touched, all real fixes;
  `jack`'s csv *import* — a 10th file in the same bridge family — was already a safe stub returning
  `JackSnapshot::default()` and referenced no removed field, so it was left alone, not touched).

  This is judged in-scope for this gate wave ("a stray import... something any closer in this program
  would have fixed in passing") since the fix is a pure reuse of each artifact's own already-existing
  public codec functions — no new business logic, no shape redesign, no stdio-side edit.

**Post-fix note (transient, unrelated churn observed)**: a later `cargo test -p semio-s-plugin-trinity
--lib` run in this same session (attempting extra thoroughness beyond the `cargo check` already
reported above) failed on 2 `E0063` errors in `semio-framework-plugin` — `missing field
topic_contributions in initializer of PluginManifest`. Investigated: `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`
and `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` both show as currently modified
(uncommitted) in `git status` — a live concurrent session mid-way through adding a
`topic_contributions` field to `PluginManifest`, not yet finished updating every initializer site.
Completely unrelated subsystem (plugin manifest/topics, not artifacts/schemas), landed strictly
*after* this wave's own `cargo check -p semio-s-plugin-trinity` (0 errors) had already run and been
recorded above. Classified as transient external churn per the plan's own risk-mitigation guidance
("external churn misattribution → own-filter verification + verify-stage classification") — not
chased, not fixed, not counted against this wave's fix.

**Full workspace check**: not run (intractable in this session's time budget given the workspace's
size — dozens of crates). The `semio-framework` clean check + the 4-crate non-stdio spot-check above
(which surfaced one real, program-caused bug and fixed it, plus correctly classified unrelated
pre-existing breakage) is judged sufficient spot-check coverage per the brief's "or a full `cargo
check --workspace` if tractable" wording.

---

## 3. `bun run ./📜️script.ts policy` — PASS, S-8 rules at expected state

Full run: 21592-line output (exit code 1 — expected, ~21.6k pre-existing unrelated repo-wide
breaches, not this program's concern per the brief).

| Rule | Live breach count (stdio) | Allowlist size (stdio) | State |
|---|---|---|---|
| `stdio-artifacts/diff-algebra` | 0 | 0 (`POLICY_DIFF_ALGEBRA_ALLOWLIST = new Set([])`) | **Fully real** — seeded 31/31 at S2 (zero implementors then), shrunk to 0/0 through F1-F6. No shortcuts. |
| `stdio-artifacts/field-sweep-presence` | 0 | 0 (`POLICY_FIELD_SWEEP_ALLOWLIST = new Set([])`) | **Fully real** — same pattern, seeded 31/31, shrunk to 0/0. |
| `stdio-artifacts/vcs-machinery-ban` | 0 | (seeded empty at S2, stayed empty) | **Fully real**, unchanged since S2. |
| `stdio-artifacts/grammar-honesty` | 0 (allowlist absorbs it) | **353/651** (down from 645/651 seeded at S2) | **Real, quantified gap** — see §4 below. Zero *live* breaches only because the shrink-only allowlist still lists 353 leaves as documented-placeholder. |
| `stdio-artifacts/facet-mirror-drift` | 0 (allowlist absorbs it) | **93/93** (unchanged since S2's seed) | **Real, quantified gap, zero progress** — see §6 below. |
| `dsl-migration/diff-completeness` | **1** (`ifc/2x3` only) | 0 stdio entries (`POLICY_DIFF_COMPLETENESS_ALLOWLIST`, grepped directly — zero `stdio` matches) | Matches F6's own final number exactly. |

`POLICY_DIFF_COMPLETENESS_ALLOWLIST` (📜️script.ts:2304) confirmed by direct read: 0 stdio entries,
same as every F6 sub-wave closer reported — never used as a shortcut.

---

## 4. Grep gates — PASS (3 of 4 clean), grammar-leaf gap precisely quantified

**a) `serde_json::Value` in any of the 384 snapshot/diff/mutations `component.rs` files' PUBLIC type
definitions**: **zero real hits.** 3 raw grep hits found (json's and gltf's own snapshot files), all
inside doc comments explicitly stating its ABSENCE ("No `serde_json::Value` anywhere in this file"),
not actual usage. Clean.

**b) `snapshot: Option<` in diff files**: **zero real hits of the banned full-replace slot.** ~40 raw
grep hits, 39 are doc-comment citations of the OLD banned shape (explaining why it's gone), and 1 real
struct field: `BcfViewpointDiff::snapshot: Option<Option<Vec<u8>>>` — inspected directly, this is
BCF's real per-viewpoint PNG-image tri-state field (`camera`/`components`/`snapshot` are BCF's own
weak, whole-value-replaced viewpoint fields per its recipe doc comment), unrelated to the banned
`Option<XSnapshot>` artifact-level full-replace pattern. Clean.

**c) Apply-and-capture pattern** (`apply_x_mutation` called from inside a `diff()` body): scanned all
64 mutations `component.rs` files with an awk window over every `fn diff(` body for
`apply_.*_mutation(` calls — **zero matches anywhere**, not just the ~5-artifact spot-check the brief
asked for. Additionally hand-verified svg, gif, zip, xml, png directly: every `apply_x_mutation(...)`
call found is inside a `#[cfg(test)]` test body (constructing fixtures / round-trip assertions), never
inside the real `diff()`/`apply_x_mutation` production body. The canonical body
(`let d = mutation.diff(&*snapshot); *snapshot = d.apply(snapshot); d`) is present verbatim in the doc
comments of svg, xml, png, and others. Clean.

**d) Grammar-leaf honesty — real, material, precisely quantified gap** (per the brief, NOT to be
fixed this wave):

Extracted `POLICY_GRAMMAR_HONESTY_ALLOWLIST` (📜️script.ts:8649-9003, 353 stdio entries — this is the
ground truth for "still placeholder": the live policy breach count is 0 only because every remaining
placeholder is allowlisted; the rule's own reverse-check, "allowlisted but no longer a placeholder,"
is also 0, confirming no stale entries). Universe = 31 standards × 3 facets
(snapshot/diff/mutations) × 7 leaf kinds (text: `.g4`/`.ebnf`/`.grammar.semio`; binary:
`.ksy`/`.spicy`/`.abnf`/`.protocol.semio`) = 651 leaves.

**353 of 651 (54%) grammar leaves remain the literal S2-scaffolded placeholder** (`payload = *OCTET` /
`size-eos: true` / `payload: bytes &eod;` / the fixed `DOCUMENT: 'schema'...` templates), down from
645/651 (99%) seeded at S2 — i.e. real, substantial progress happened during F1-F6, but the user's
explicit "handcraft ALL formats, no placeholders left" decision is **not** satisfied.

| Standard | Placeholder / 21 possible | Standard | Placeholder / 21 possible |
|---|---|---|---|
| binary/raw | **0** (fully handcrafted) | md/commonmark | **0** (fully handcrafted) |
| png/1.2 | **0** (fully handcrafted) | tiff/6.0 | **0** (fully handcrafted) |
| txt/utf-8 | **0** (fully handcrafted) | | |
| deflate/rfc1950 | 2 | gltf/2.0 | 2 |
| pdf/1.7 | 4 | json/rfc8259 | 6 |
| ply/1.0 | 12 | stl/ascii | 10 |
| bmp/v3 | 9 | obj/3.0 | 9 |
| csv/rfc4180 | 11 | dxf/r12 | 9 |
| xml/1.0 | 9 | ifc/4 | 15 |
| step/ap214 | 15 | las/1.0 | 14 |
| svg/1.1 | 13 | zip/2.0 | 17 |
| bcf/2.1 | **21** (untouched) | docx/ecma-376 | **21** (untouched) |
| dwg/ac1018 | **21** (untouched) | dwg/ac1024 | **21** (untouched) |
| gif/87a | **21** (untouched) | gif/89a | **21** (untouched) |
| pdf/1.4 | **21** (untouched) | pptx/ecma-376 | **21** (untouched) |
| xlsx/ecma-376 | **21** (untouched) | | |

**Summary**: 5/31 standards fully handcrafted (0 placeholder leaves), 17/31 partially handcrafted
(2-17 of 21 remaining), **9/31 standards have literally zero grammar-leaf work done** (still 21/21
placeholder — bcf, docx, dwg×2, gif×2, pdf/1.4, pptx, xlsx). Grand total: **298 of 651 leaves (46%)
handcrafted, 353 (54%) still placeholder.** Cross-referencing STATUS.md/F-wave reports: none of the
F1-F6 reports read this session claim any dedicated grammar-leaf-authoring pass ever ran as its own
tracked deliverable — the 298 handcrafted leaves appear to be incidental byproducts of F6's op-codec
work touching `.protocol.semio`/`.grammar.semio` files that double as OpText/OpBinary grammars for
some standards (matching the plan's own note that json's `.grammar.semio`/`.protocol.semio` "got real
content from an earlier wave"), not a systematic sweep. **Confirmed: this is real, unaddressed
engineering debt against the user's explicit "handcraft ALL formats" decision — correctly not
attempted in this wave** (would require authoring up to 353 grammar files, an unbounded scope for a
single gate).

---

## 5. Fixture suites — PASS, all green

| Fixture | Test count | Result |
|---|---|---|
| `artifacts::gif::examples::dancing` | 4 | all `ok` |
| `artifacts::pdf::examples::bachelor_thesis` | 6 | all `ok` |
| `artifacts::dwg::examples::architectural` | 4 | all `ok` |
| `artifacts::gltf::examples::metabolism` | 5 | all `ok` |

(`architectural` and `metabolism` initially looked absent from a truncated `tail`-only view of the
background test run; re-checked against the full saved `g-full-crate-test.txt` — all 19 tests present
and passing, including `real_decode_reaches_d2_with_every_named_section` /
`real_decode_stays_lossless_on_reencode` for the 145KB real `architectural.dwg`, and
`base_glb_decode_encode_decode_is_semantically_equal` for the real `base.glb`.)

`codec_retention_law` equivalents on real fixtures: confirmed present for all 4 (dancing/bachelor_thesis/
architectural/metabolism tests are themselves the fixture-level retention checks the plan's law #5
describes — "decode→encode byte-preserving... on fixtures").

---

## 6. Facet-mirror drift — real, material gap, ZERO progress since S2

`POLICY_FACET_MIRROR_DRIFT_ALLOWLIST` (📜️script.ts:9082-9176): **93 stdio entries**, live breach
count 0 (same allowlist-absorption mechanism as grammar-honesty). 93 = 31 standards × 3 facets
exactly — **every single facet of every single one of the 31 official standards is still flagged as
drifted**, unchanged from S2's own seed ("93/93 seeded"). No F-wave rewrote a single TS/GraphQL/
JSON-schema/proto facet mirror to match its real Rust shape. This matches what several F-wave reports
already said explicitly ("facet mirrors left as pre-existing stale content... tracked by the
shrink-only allowlist") — confirmed here with the precise number: **100% of stdio facet mirrors
(93/93) are stale**, not a subset. Correctly not attempted this wave (same unbounded-scope reasoning
as §4).

---

## 7. Known cross-cutting cleanup items — confirmed accurately captured, not touched

**a) OPC-diff-type duplication** (docx/xlsx/pptx each define their own `XxxOpcDiff` instead of a
shared `zip::opc` hoist — F5's finding): re-confirmed live via direct grep — `docx`, `xlsx`, and
`pptx` each still have their own `OpcDiff`-family type in their `🔺️diff/🦀️component.rs`. `bcf` does
not (it references `opc::` directly rather than duplicating a diff type — the STATUS.md F5 mention of
"×4" appears to slightly overstate bcf's involvement, but the core dedup opportunity for
docx/xlsx/pptx is real and unfixed). Accurately captured in STATUS.md's F5 section and the "Final
summary table" `glue_edits` paragraph. Not touched (real work, not this wave's scope).

**b) The 4 `dsl`-crate framework gaps from F6** (derive-macro `record` field-name hygiene collision
[csv]; nested fixed-arity array `[[T;N];M]` print/parse bug [stl]; missing blanket `impl DslField` for
tuples of any arity [las, jpg — 2 independent confirmations]; zero-generics support in
`DslDiff`/`DslOps` derive for generic collection-diff wrappers [gltf/pptx/docx/bcf/xlsx — 5 independent
confirmations, the most-hit gap]): all 4 fully documented in `f6-final-summary.md` §4 with doc-comment
citations at each point of use in the affected artifacts' own source. STATUS.md's "F6 program —
CLOSED" capstone section correctly points to `f6-final-summary.md` for the full detail. Confirmed
still real and unfixed (out of every wave's ownership boundary by design — fixing any of them means
editing the shared `dsl` crate). Not touched.

---

## Final consolidated list of known remaining work (for follow-up ticket)

1. **Grammar-leaf authoring**: 353 of 651 stdio grammar leaves (54%) still the S2 placeholder
   skeleton, against the user's explicit "handcraft ALL formats" decision. 9 standards have zero
   leaves done (bcf, docx, dwg×2, gif×2, pdf/1.4, pptx, xlsx); 17 partially done; 5 fully done. Precise
   per-standard counts in §4's table. Largest remaining effort in the whole program by file count.
2. **Facet-mirror rewrite**: 93/93 stdio facet mirrors (100%, every facet of every standard) still
   stale TS/GraphQL/JSON-schema/proto content, zero progress since S2. `POLICY_FACET_MIRROR_DRIFT`
   shrink-only allowlist is the tracking mechanism.
3. **OPC-diff-type hoist**: docx/xlsx/pptx's independently-duplicated `XxxOpcDiff` families should be
   hoisted into a shared `zip::opc`-adjacent module. Not a correctness bug, pure duplication cleanup.
4. **4 `dsl`-crate framework gaps** (§7b above): derive-macro `record` hygiene collision, nested
   fixed-arity array print/parse bug, missing blanket `DslField` for tuples, zero-generics support for
   collection-diff wrappers in `DslDiff`/`DslOps`. Prioritize the generics gap (5 artifacts hit it) and
   the tuple gap (2 independent confirmations) if a framework-focused wave picks this up.
5. **`ifc/2x3`**: the one remaining `dsl-migration/diff-completeness` stdio breach, genuinely the
   out-of-scope 32nd standard (never part of the official 31). Shares its model shape with the
   already-hand-rolled `ifc/4`, so a follow-up would likely be a fast template-reuse job, not fresh
   design, if anyone wants literal zero.
6. **`norm`/`block`/`puzzle` plugins' dangling `📄️document` path** (discovered incidentally during
   this wave's item-2 spot-check): pre-existing, unrelated to this program (last touched 2026-06,
   outside `🗄️stdio`), breaks `cargo check` on those 3 non-stdio plugins. Not this ticket's scope;
   flagged for whoever owns those plugins.

---

## Final verdict

**The program's core mandate is satisfied for all 31 official standards**: every snapshot is a
complete semantic model (no `serde_json::Value`, no untyped bare bytes outside documented
raw-retention cases), every diff is handcrafted and sparse (zero `snapshot: Option<XSnapshot>`
full-replace slots), every mutation returns a handcrafted diff via
`apply_x_mutation`/`Mutation::diff` (zero apply-and-capture), and every artifact's generic
op-slot/stub code has been replaced with format-specific typed code, including the wire-codec layer
(`DiffCodec`/`OpText`/`OpBinary`, F6's scope) — confirmed by a fresh, from-scratch
1075/0 test run and direct file inspection, not by trusting prior waves' self-reports.

**Two items explicitly named in the user's original decisions are NOT satisfied and must not be
mistaken for complete**: grammar-leaf handcrafting (54% still placeholder) and facet-mirror parity
(100% still stale). Both were correctly deferred by every F-wave as out of their per-artifact
ownership scope, are now precisely quantified above (not just gestured at), and are appropriately
sized as their own dedicated follow-up ticket(s) — neither is a "few strays," both are hundreds of
files' worth of real authoring work.

**One real bug was found and fixed in this wave**: trinity plugin's jack/rewrite json/md/csv
cross-plugin bridges, broken by F1/F3's snapshot-shape changes reaching outside their ownership
boundary into an undocumented downstream consumer. Fixed via reuse of each artifact's own existing
codec primitives (8 files, `✏️s/🔌️plugins/🔱️trinity/...`), verified with a clean
`cargo check -p semio-s-plugin-trinity` (0 errors). This is the only source-code change this wave
made; `stdio`, `framework`, and `script.ts` were not touched.
