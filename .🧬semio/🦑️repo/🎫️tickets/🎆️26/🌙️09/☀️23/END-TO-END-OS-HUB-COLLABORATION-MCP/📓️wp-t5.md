# WP-T5: Binary Protocol Tags, Runtime Inventories, Oracles, Editor Catalogs, Mutation Vectors

Slice: T5 (session 10). Captures: `.tmp-ticket/wp-t5/generated/`. Private cargo: `.tmp-ticket/wp-t5/target`.
Inherits: T2 §2.6/§3 (contract-3), R3 (base bridge + oracle), R5, T1 (contract-2: 573 high rows, test-platform 111/2).
Siblings: t4 (parity infra, asset schema scopes, gltf: not touched), p5 (pack schema hashing), t3 (serializers).

## Status

| Item | State | Evidence |
|------|-------|----------|
| 0. Baseline contract | 564 rows in the full set (incl. medium): runtime-inventory-missing 175, binary-protocol-drift 97 (medium), unregistered-mutation-vocabulary 48, missing-external-oracle 27, no-oracle-covers-mutation 13, mutation-without-fixture 9 | `contract-0.txt`, `breaches-0.json` |
| 1. Binary protocol drift (97): one schema-first tag source | **Records authored for all 97 + the 21 vocabularies that already had some; every codec now derives its tags from them.** Replay of the drift rule: 97 → 0 (`drift.py`). Compile: waves 1–3 `cargo check --tests` clean except 3 peer errors (§1.6). **Lib tests of the 77 touched crates: 24,414 pass / 18 fail** (`test-1-summary.txt`). One failure was mine: html's `Some(None)` round trip, fixed via `tagged_text_binary`, and html now passes **42/42** (`html-test-2.txt`). The other 17 are missing fixture files (mp4, pptx, zip, deflate) plus peer flow/remodeling tests; none of them touches a codec | `wave{1,2,3}-check.txt`, `test-1-summary.txt`, `dsl-test-1.txt` |
| 2. Runtime inventories (175) | **175 → 0 `runtime-inventory-missing`.** 35 generated per-plugin bridges all build; `test inventory` produced **173/175** inventories, and **137 agree exactly** with their manifests. The other 2 are peers' pre-existing semio mesh/brep subset bridges, which do not compile (E0432/E0433). The inventories surface real drift: outcome-mismatch 254, variant-mismatch 119 (all gltf, t4), runtime-only 81, manifest-only 55 (§2) | `inventory-2.txt`, `bridges/` |
| 3. Unmet oracle requirements (27) | **27 → 4 high + 23 `justified-reference-gap` (medium)**, measured in `contract-2`. The 4 left are dwg ×2 ×2 (§5). 9 new rows come from a peer's new `os.config@1/ui-preferences` manifest | `contract-2.txt`, `delta-2.txt` |
| 4. Editor-layer vocabularies without catalogs (48) | **Not done** (§4): each needs a catalog, a claiming case and a production report surface, and ~30 of the 48 aggregates are not publicly reachable. The plan is in §4 | — |
| 5. 13 no-oracle decisions (no-oracle-covers-mutation) | **13 → 5 measured. 8 justified, schema-first** (§5): equation, sequence, drawing, gisterrain config, raw buffer, os.config ×3. **5 open**: jpg, tiff and txt already have registered readers that no case runs; dwg ×2 has an installed LibreDWG CLI (`dwgread 0.13.3`) that can read the preamble | test-platform `surveyed decision` 1/1 |
| 6. mutation-without-fixture | **Live rule 9 → 0** (`nofixture.ts`). The "87" in T1's run had already fallen to 9 (all gif 89a) before T5 started. Root cause: gif `🧱️base` re-declared the 9 kinds its `🎛️graphic-control`/`💬️comment`/`🧩️application` subsets own. Fix: the schema's own `mutation.subset` override (§6) | `nofixture.ts` output |
| 7. Contract rerun + delta | **564 → 668 rows**, but the rise is the new measurement: 509 rows (outcome/variant/runtime-only/manifest-only) exist only because inventories now exist. Removed: runtime-inventory-missing −175, binary-protocol-drift −97, missing-external-oracle −14, no-oracle-covers-mutation −8, gif fixture −9 (+15 new peer rows: os.config ui-preferences, cad object). Peers removed stub-serializer/deserializer and depth rows in the same window (§7) | `delta-2.txt` |

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

## 2. Runtime inventories

- **Generator** `bridges.py`. It emits one bridge per plugin (and one for `os/🎚️config`) at `<plugin>/🏭️bridge/{📜️script.ts,🦀️.rs,Cargo.toml}`. The inventory orchestration finds a bridge at the owner or any ancestor, so one bridge per plugin serves every subset manifest below it.
  - The Rust binary links the plugin's artifact crates (own `[workspace]`, like R3's base bridge). It lists every public `#[derive(dsl::Mutations)]` aggregate under a `🧬️mutations/🦀️.rs` outside the editor/viewer/presence layers, typed with the snapshot its `#[mutations(snapshot = …)]` names. It maps each manifest coordinate to its owner directory.
  - A subset's inventory is the set of `DESCRIPTORS` whose leaf `owner` lies inside that subset's owner. Nothing is read from the manifest, so the gate compares production with the claim.
  - The module tree is resolved by walking inline `mod x {}` blocks, `#[path]` mounts and `mod top_level; pub use top_level::*` re-exports, with public visibility tracked. 86 stdio aggregates across 36 crates resolved.
- **Launch**: `⚖️gate🧪️test🏭️inventory` (`bun nx run @semio-tech/repo-test-domain:test-inventory`) is registered in `.vscode/launch.json` and `🧩️launch.seed.jsonc`.
- **Builds**: sequential, one cargo at a time, private `CARGO_TARGET_DIR=wp-t5/target-bridge` (`build-bridges.sh`).
  - The first pass lost 17 builds to the 12:43 external wipe of the shared build-dir (exit 137 and missing `.rmeta`/`.d` files).
  - First builds before the wipe: 28 of 35 built. Failures: puzzle (ambiguous `Mutation<_>`, fixed with explicit snapshot types) and stdio (wav's `standards::riff_pcm` module path, fixed in the module walker).
  - The rebuild after the wipe is running.
- **Subsets without their own leaves**: note ×8, draw ×4, las points/vlr, gif sub-subsets, bcf, avi, dxf, obj material, equation ×3, sequence ×2. Their leaves live in the sibling `✳️any`/base vocabulary. The descriptors' `owner` points there, so the owner-prefix filter yields an **empty** inventory for such a subset, and the gate will report its rows as manifest-only. That is the true state: production descriptors attribute these kinds to the base vocabulary. The fix is the gif pattern (§6): the base manifest owns the kind, with a `subset` override.

### 2.1 What the inventories measured (`inventory-2.txt`)

- Build: all 35 bridges compile in the private target `wp-t5/target-bridge` (`bridges/summary.txt`).
- Generator fixes made after the first build:
  - The snapshot type is now picked by the module closest to the aggregate: gif 87a/89a and pdf 1.4/1.7 each carry a same-named snapshot.
  - Where several manifests share one owner directory (os.config opening, merge-policy and identity), the bridge restricts to the aggregate whose type name starts with the subset (`Opening…`, `MergePolicy…`, `Identity…`).
- **Manifest spelling fixed** (`productionDispatch.variant`): 117 rows in 12 stdio manifests named the enum-qualified form (`LasMutation::SetSnapshot`) or the payload type (`ChangeHeaderMutation`) instead of the aggregate variant. 2,488 of 2,576 manifest rows already use the aggregate variant. Fixed in las, gif 87a/89a, svg, xml, png, jpg, dxf, tiff, obj, json and bmp.
  - The remaining 119 are all gltf (t4's scope, untouched).
- Findings handed to owners (real drift, not fixed):
  - **mutation-outcome-mismatch 254**: manifests declare `rejected`, but the leaf descriptors' `outcomeClasses` hold only `applied` (wfc, norm, stdio). One of the two declarations is wrong for each kind.
  - **runtime-only 81 / manifest-only 55**, grouped by where the kinds live:
    - fem 2d/3d: 0 runtime, because the leaves' descriptor owners are not under the manifest owner.
    - las points/vlr, dxf tables/entities/blocks, bcf viewpoint/snapshot, avi movi/idx1, obj material and tiff baseline: kinds dispatched from the base vocabulary, which needs the gif §6 pattern.
    - pdf 1.7: 60 dispatched against 16 declared.
    - cad: 24 against 19.
    - os.config: `UiPreferences` aggregate.

## 4. Editor-layer vocabularies (not done)

- Each of the 48 needs the following, per the only precedent (`🏔️gisterrain` window config):
  - an oracles contribution with a catalog, a decision or survey, and a manifest;
  - a case whose feature claims the catalog, with mutate and inverse per kind;
  - a public production report function the adapter can reach.
- 13 of them already carry serde-cross-checked contract vectors (`🧫️fixtures/🔁️mutation-contracts.json` + `🧪️tests/🔬️contract-vectors`). Those are the natural vector source.
- Blockers:
  - ~30 aggregates are in private modules. The bridge walker reported them as "not publicly reachable".
  - Every case needs its own plugin test-host build, and all builds are cold since 12:43.
- **Plan**: one generic `mutation_report_json::<S, M>(base, mutation)` in the plugin framework that returns `{base, snapshot, inverseSnapshot}`, re-exported by each editor facet. One adapter template, and a generator that emits catalog, feature and adapter from the leaf descriptors plus the existing contract vectors.

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

## 7. Contract delta (`contract-0` → `contract-2`, `delta-2.txt`)

| Class | Before | After |
|---|---|---|
| runtime-inventory-missing | 175 | 0 |
| binary-protocol-drift (medium) | 97 | 0 |
| missing-external-oracle | 27 | 13 (dwg 4 + a peer's new os.config ui-preferences 9) |
| justified-reference-gap (medium, new) | 0 | 23 |
| no-oracle-covers-mutation | 13 | 5 (jpg, tiff, txt, dwg ×2: §5) |
| mutation-without-fixture | 9 | 15 (gif 9 → 0; the 15 are a peer's new os.config ui-preferences 9, cad object 5 and layout 1) |
| mutation-outcome-mismatch (new, measured) | 0 | 254 |
| mutation-variant-mismatch (new, measured) | 0 | 119 (all gltf) |
| runtime-only / manifest-only (new, measured) | 0 | 81 / 55 |
| unregistered-mutation-vocabulary | 48 | 48 (§4) |
| total | 564 | 668 |

- The total rose because runtime completeness is now measured for 173 manifests. The 509 measured rows are the gate working, not regressions.
- Peers' concurrent work removed stub-serializer/deserializer (−102) and test-depth (−18) in the same window.

## Processes (pids)
- contract-0: 90978; contract-1/2 run in the foreground; inventory 91152 plus a second foreground run; bridge loops 62160, 97495, 13371, 90348 (all exited; the loops of 15558/26482 died in the 12:43 wipe).
- contract-0: 90978 (exited). dsl test: 98789 (exited). Checks: 3977, 6226, 16301 (exited). Tests: 24583 (`test-1-summary.txt`).

## Files changed (T5)

## Files changed (T5)

- **dsl** (`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs`): new `protocol_record`, `variants_binary::{encode,decode}_tagged_op`, `tagged_value_binary`, `tagged_text_binary`; test `🧪️tests/🏷️protocol-record/🦀️.rs` (3/3).
- **Codecs**: ~118 `🧬️mutations/💾️binary/📡️.protocol.semio` rewritten with per-kind records.
  - Codec sources in: norm (15), stdio (~50 artifacts/subsets), procedural, process3d, cad, block ×3, puzzle ×3, wfc ×4, gis ×2, raster, remodeling, playbook, procedure, energy, trinity ×2, space ×2, flow, shooting, layout, architect program, demonstrator playground.
  - Per-leaf `💾️binary/🦀️.rs` tag consts: png, jpg, tiff, bmp, pdf, txt, svg, xml, json, jack, rewriting, space, home, procedure, gisterrain.
  - The `.ksy`/`.abnf`/`.spicy` tag docs of 16 semio subsets.
  - 1908 leaf descriptors' `binaryTag`.
  - stdio contract `impl_serde_op_codec!` (protocol arm).
- **Vocabulary rename**: generation2d `🎛️set-camera` → `🎛️update-camera` (leaf, fixtures, module, oracle, feature).
- **Test platform**:
  - `🧪️test/🟦️.ts`: drift gate from descriptors, `binary-protocol-tag-reuse`, `isJustifiedMutationDecision`, `justified-reference-gap`.
  - `🧪️test/🧬️schema/🔣️.json`: `NoOracleDecision.coversMutations/referenceSurvey`, `ReferenceSurvey`.
  - `🧪️tests/🧪️test-platform/🟦️.ts`: 2 new tests (surveyed decision; live protocol records), 2/2 pass.
- **Oracle contributions**:
  - 8 decisions (equation graph, sequence, drawing, gisterrain config, binary raw, os.config).
  - gif 89a base plus its three narrow subsets.
  - `productionDispatch.variant` in 12 stdio manifests.
  - generation2d oracle note.
- **Bridges (new)**: `<plugin>/🏭️bridge/{📜️script.ts,🦀️.rs,Cargo.toml,Cargo.lock}` for 34 plugins and `🧰️framework/🛍️products/💻️os/🎚️config/🏭️bridge/`.
- **Launch**: `⚖️gate🧪️test🏭️inventory` in `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`.
- **Ticket inputs (kept)**: `.tmp-ticket/wp-t5/*.py|*.ts|*.sh`. The private targets are deleted (rule 14).

## Notes

- The coordinator freeze on stdio/gis/kernel edits took effect after all codec edits had landed and compiled. Nothing was edited in those crates afterwards; only bridge crates, which nothing links, were rebuilt.
- Every codec crate touched passed `cargo check --tests` right after its edit wave (rule 13). The dsl kernel change is pure `const fn` with no cfg(wasm) code, so no wasm32 check was needed.
