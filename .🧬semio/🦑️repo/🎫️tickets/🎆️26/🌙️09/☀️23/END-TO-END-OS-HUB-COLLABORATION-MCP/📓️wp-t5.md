# WP-T5: Binary Protocol Tags, Runtime Inventories, Oracles, Editor Catalogs, Mutation Vectors

Slice: T5 (session 10). Captures: `.tmp-ticket/wp-t5/generated/`. Private cargo: `.tmp-ticket/wp-t5/target`.
Inherits: T2 §2.6/§3 (contract-3), R3 (base bridge + oracle), R5, T1 (contract-2: 573 high rows, test-platform 111/2).
Siblings: t4 (parity infra, asset schema scopes, gltf: not touched), p5 (pack schema hashing), t3 (serializers).

## Status

| Item | State | Evidence |
|------|-------|----------|
| 0. Baseline contract | 564 rows in the full set (incl. medium): runtime-inventory-missing 175, binary-protocol-drift 97 (medium), unregistered-mutation-vocabulary 48, missing-external-oracle 27, no-oracle-covers-mutation 13, mutation-without-fixture 9 | `contract-0.txt`, `breaches-0.json` |
| 1. Binary protocol drift (97): one schema-first tag source | **Records authored for all 97 + the 21 vocabularies that already had some; every codec now derives its tags from them.** Replay of the drift rule: 97 → 0 (`drift.py`). Compile: waves 1–3 `cargo check --tests` clean except 3 peer errors (§1.6). Tests: running (`test-1.txt`) | `wave{1,2,3}-check.txt`, `test-1.txt`, `dsl-test-1.txt` |
| 2. Runtime inventories (175) | pending | |
| 3. Unmet oracle requirements (27) | pending | |
| 4. Editor-layer vocabularies without catalogs (48) | pending | |
| 5. 13 no-oracle decisions (no-oracle-covers-mutation) | **8 justified, schema-first** (§5): equation, sequence, drawing, gisterrain config, raw buffer, os.config ×3. **5 open**: jpg, tiff and txt already have registered readers that no case runs; dwg ×2 has an installed LibreDWG CLI (`dwgread 0.13.3`) that can read the preamble | test-platform `surveyed decision` 1/1 |
| 6. mutation-without-fixture | **Live rule 9 → 0** (`nofixture.ts`). The "87" in T1's run had already fallen to 9 (all gif 89a) before T5 started. Root cause: gif `🧱️base` re-declared the 9 kinds its `🎛️graphic-control`/`💬️comment`/`🧩️application` subsets own. Fix: the schema's own `mutation.subset` override (§6) | `nofixture.ts` output |
| 7. Contract rerun + delta | pending | |

## 1. One tag source: the `📡️.protocol.semio` records

### 1.1 Mechanism (schema-first, compile-time)
- `dsl::protocol_record` (`🗣️dsl/🦀️.rs`, region `🏷️ProtocolRecord`): a `const fn` scanner over the included protocol text.
  - `tag`, `tag_u8`, `tag_u32`: the tag of `record <kind> tag=<n>`. A missing or duplicated record, or a tag too wide for the codec's field, is a **const-evaluation error**, so drift fails the build.
  - `records`, `kind`: runtime iteration and tag → kind.
  - Unit test `🧪️tests/🏷️protocol-record`: the scanner agrees with the dialect parser `parse_protocol` (`Block::Record`), and tagged ops carry the declared tag, including a 2-byte varint (300). **2/2 pass** (`dsl-test-1.txt`).
- `dsl::variants_binary::{encode_tagged_op, decode_tagged_op}(protocol, …)`: the `DslVariants` frame with the tag taken from the record of the variant keyword. It replaces the ordinal for every vocabulary with a protocol. The ordinal `encode_op/decode_op` remain only for the ~90 ephemeral editor/presence layers, which have no `💾️binary` facet (open item §1.6).
- `dsl::tagged_value_binary::{encode_op, decode_op}(protocol, VariantTag, …)`: the real binary frame for the aggregates whose "binary" was JSON or text bytes with no tag. The frame is `format u8 | tag varint | pack wire value of the ToValue tree without the variant name`. It handles both internally/adjacently tagged (`Field("mutation")`) and externally tagged (`Key`) aggregates.
- `impl_serde_op_codec!(T, what, protocol = …)` (stdio contract): a new arm that gives the same text codec plus the tagged binary.

### 1.2 How each record set was handcrafted from the real wire (never guessed)

| Old tag source | Vocabularies | How the records were read | Codec change |
|---|---|---|---|
| Hand-written literals (`=> N`, `write_u8(N)`, `push(N)`, trailing `N`, `fn mutation_tag`) | 21: norm en1990–en1994, din4108, iso16757, vdi3805; stdio bcf, stl, dxf, ifc 4/2x3, csv, step, md, xlsx, ply, docx, semio base and value | `literal.py` reads the **encode** side and the **decode** side of each codec independently. It accepts a codec only when both agree for every variant and the variant set equals the leaf kinds | Literals → `const TAG_<KIND>: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "<kind>")`, used in both encode and decode (match patterns are the consts) |
| `OP_KEYWORDS`/`KINDS` index + `variant_ordinal` | 16 semio subsets | `keyword_table.py` cross-checks `variant_ordinal` against the keyword table | `variant_ordinal` → `wire_tag` over the consts. Decode maps tag → record kind via `protocol_record::kind`. `OP_KEYWORDS` is deleted. Where the text grammar uses other keywords (drawing `rotate`, animation `IT`, …) a kind → text-keyword table `TEXT_KEYWORDS` replaces it: that is grammar, not a tag source. Tests: `KINDS == records` |
| Per-leaf `BINARY_TAG`/`TAG` const (registry) | png, jpg, tiff, bmp, pdf 1.4/1.7, txt (u32), playground (u32); declared-only in jack, rewriting, space, home, procedure, gisterrain, svg, xml, json | `registry.py` reads each leaf's literal. For the declared-only ones it verifies them against the wire records first (`--consts-only`) | Leaf const → `tag_u8/tag_u32(include_str!("../../💾️binary/📡️.protocol.semio"), "<kind>")` |
| Derive ordinal (`variants_binary::encode_op`) | 39 aggregates: norm ×7, procedural ×2, process3d, cad, block ×3, puzzle ×3, wfc ×4, gis ×2, raster, remodeling, playbook, procedure, energy (287 kinds), rewriting, jack, space, home; stdio mp4, deflate, zip, gif ×2, obj, binary | `variants.py` parses the `DslOps`/`DslEnum` enum and reproduces `dsl_variants_codegen`'s rule: declaration order, keyword = `#[dsl(key)]` or kebab(variant). It accepts only when keywords == leaf kinds | `encode_tagged_op/decode_tagged_op(COMPONENT_PROTOCOL_SEMIO or include_str!, …)` |
| None: JSON/text bytes, no tag | avi, epw, wav, mp3, svg, html, pptx, xml, json, tsv, program (266 nested kinds), shooting, layout (externally tagged), flow plugin (was framework-flow bytes + a `0xD0` tag for duplicate-widget), dwg ac1024/ac1018 | No tag existed on the wire, so these are **new** records, one per kind in enum declaration order. svg/xml/json keep their leaves' already-declared tags (1-based) | `tagged_value_binary` (flow's `DUPLICATE_WIDGET_OP_BINARY_TAG` is deleted; dwg via the new macro arm) |

Every rewritten protocol now describes the real frame: `framing record`, `header` = `format u8` (when written) plus `tag u8|varint`, then one `record <kind> tag=<n>` per kind. Existing per-kind field lists are kept (csv's `repeat`/`arm` fields moved into its records; base/wfc/cad record fields kept). The walker treats named records under `framing record` as the rest of the body, so the existing `protocol_walk_law` tests still walk real op bytes.

### 1.3 Gate
- `binaryProtocolDriftBreaches` (`🧪️test/🟦️.ts`) now takes kinds from the leaf descriptors (`readLeafDescriptors`/`mutationLeafDirectories`). Before, it took top-level directory names, which is wrong for the nested DomainOperations layout (architect program: 47 entity directories, 266 real kinds).
- New high breach `binary-protocol-tag-reuse`: a record that reuses a tag or a kind.
- The leaf descriptors' `binaryTag` is re-projected from the records, the scaffolder's own rule (`descriptors.py`, 1908 descriptors; most were `null`). One numeric correction: cad rename-node, 10 → 12, which is its real ordinal.

### 1.4 Vocabulary fixes found on the way
- generation2d: leaf directory `🎛️set-camera` held semantic kind `update-camera` (`UpdateCamera`). The directory, fixture directory, module (`set_camera` → `update_camera`), descriptor owner, oracle `mutationId`, feature paths and test messages are renamed; the stale oracle note about the naming bug is rewritten.
- gismap's old records (`record CreatePosition tag 1`, PascalCase, not parseable as tags) are replaced by the real ordinal records (0-based).
- cad's old records were 1-based. The wire (ordinal) is 0-based; the records now match the wire.

### 1.5 Still open (item 1)
- dwg ac1018 re-exports ac1024's `DwgMutation`. Its own leaf directories and protocol are declared but not dispatched: the records mirror ac1024's.
- ~90 editor/presence/config vocabularies use the ordinal `variants_binary::encode_op` and have no `💾️binary` facet. They are outside the drift gate. Moving them onto records means authoring a `💾️binary/📡️.protocol.semio` for each.
- svg basic/tiny, jpg baseline, xml valid, tiff baseline (`impl_serde_op_codec!` without protocol) still use JSON binary. They have no `💾️binary` facet, so there are no tags to derive.

### 1.6 Peer compile errors seen (not T5, not fixed)
- `semio-s-artifact-norm-en1994` lib test: `mutation.label()` is compared with `&str` (LocalizedLabel).
- `semio-s-artifact-raster-raster` lib test: `crate::io::…::artifacts::{pdf,dwg}` are missing (t3's io work).
- `semio-s-artifact-stdio-dwg` lib test: `include_bytes!` of the missing `temp/architectural_example.dwg`.

## 5. Justified mutation reference gaps (schema-first)

- **Schema** (`🧪️test/🧬️schema/🔣️.json`): `NoOracleDecision` gains `coversMutations: boolean` and `referenceSurvey` (a new `ReferenceSurvey` $def).
  - `ecosystemsSearched`.
  - `candidatesConsidered[]`: `{package, ecosystem, verdict, reason ≥ 20 chars}`, where `verdict` is one of `cannot-express-the-mutation` / `format-defined-by-this-repository` / `no-open-implementation` / `shares-the-production-engine`.
  - `whyNoneQualifies`: at least 40 chars.
- **Gate** (`🧪️test/🟦️.ts`):
  - `isJustifiedMutationDecision` requires the explicit claim plus a non-empty survey.
  - `oracleRequirementBreaches` then reports **`justified-reference-gap` at medium**: visible and listed with the surveyed verdicts, but not blocking. It does not report `missing-external-oracle`.
  - `noOracleMisuseBreaches` stays high for any decision that claims mutations without a survey.
- **Test**: `🧪️tests/🧪️test-platform` "a surveyed decision keeps a mutation's reference gap visible as medium, and an unsurveyed one still blocks": **1/1 pass**.
- **Applied** (`justify.py`). Each decision got a truthful survey, and capabilities that already had a qualifying oracle were dropped from it: `equation-1-mutate` (CSV), `sequence-1-mutate` (CSV), `drawing-1-mutate` (quick-xml).
  - equation (sympy, networkx, shapely, petgraph)
  - sequence (networkx, bpmn-js, graphviz)
  - drawing (quick-xml, svgelements, paper)
  - gisterrain config camera (pydeck, cesium)
  - raw buffer (coreutils dd/truncate, numpy)
  - os.config opening, merge-policy and identity (xdg-mime, automerge, keyring)
  - For the three semio-native graphs, the owners' own "this decision is a debt" text is kept: a verified native second implementation is what would discharge it.
- **Not converted** (a third-party reference exists, so a decision would be dishonest):
  - jpg baseline (`pillow-…-reader`), tiff baseline (`tiff-…-reader`), txt (`bstr-…-reader`): these readers are registered, but no case runs them. The decisions contradict them. The fix is to wire each reader into the case's oracle role, then retarget the feature from `@no-oracle-…` to `@oracle-…`.
  - dwg ac1024/ac1018: `dwgread` (LibreDWG 0.13.3, GPL, used as a CLI, so nothing is linked) is installed and reads the preamble version and codepage these kinds edit. It should be registered as a `third-party-cli`.

## 6. gif 89a fixture debt

- The three narrow subsets' own contributions said the four graphic-control kinds, the two comment kinds and the three application kinds are theirs. `🧱️base` still declared all nine as its own, with no fixtures, while the siblings' fixtures target their own subsets.
- **Fix: one owner per mutation.**
  - The base manifest keeps the dispatch (`GifMutation` and all 21 leaves live in base). Each of the nine carries the schema's `"subset": "<owner>"` override and the owner's capability, requirement and invariants.
  - The three sibling `mutationManifests` are removed, so ownership is not duplicated.
  - The base catalog drops the nine kinds, which the siblings' catalogs and cases cover.
- Result: `mutationFixtureBreaches` 9 → 0. The base runtime inventory (21 leaves under base) now equals its manifest (21).

## Processes (pids)
- contract-0: 90978 (exited). dsl test: 98789 (exited). Checks: 3977, 6226, 16301 (exited). Tests: 24583 (`test-1.txt`).

## Files changed (T5)
