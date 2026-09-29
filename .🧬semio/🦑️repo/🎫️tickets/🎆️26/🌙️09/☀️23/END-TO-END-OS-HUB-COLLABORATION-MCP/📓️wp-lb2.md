# WP-LB2 — stdio csv Redo Arena Budget, Verb-Arg Law, LD Leftovers

Session 14 slice LB2 (continues [📓️wp-lb.md](📓️wp-lb.md) and [📓️wp-ld.md](📓️wp-ld.md)). Coordinator = main chat. Ports 8130–8139 / 6630–6639.
Scripts + prepared patches `wp-lb2/`, captures `wp-lb2/generated/` (expendable), private target `wp-lb2/target`, durable data
`.🧬semio/🌐hub/s14-lb2-*`. Native cargo only via `wp-lb2/cargo-lane.sh` (native lane, build-fleet-b, nice 15); overlay builds via the
overlay lane with a private build-dir. Landing rows: `📓️landing.md` § Session 14.

### Session 14c

Successor agent (2026-09-28 16:5x, after the 14:37 usage cut + app restart). Guest freeze ON (chain relaunched 16:55:46) → non-test
guest code = prepared patches (dry-run clean) for window 3; rule 22 test-only edits may land compile-atomic.

| # | Item | Status |
|---|---|---|
| 0 | Reconcile the docx/xlsx test port (predecessor's last step) | **done 17:01**: no docx/xlsx file changed after 14:36 except the regenerated set-snapshot quintet (generator run of 14:36, `t8-docx-xlsx-2.txt` GENERATE rc=0); the tree is committed by the 16:29 auto-commit → no half-applied hunk. Native `check --lib --tests` docx + xlsx **EXIT 0** 16:59 (`c1-check-docx-xlsx.txt`); lib tests docx 68/5 red, xlsx 49/19 red (`c1-test-docx-xlsx.txt`) = exactly the 14:36 set, all in NON-test code (item 1) |
| 0b | lb-p4 proof: `cargo test -p semio-s-plugin-stdio --test shipped_fleet` (native lane) | **EXIT 0** 17:14 (1 passed, `c2-test-shipped-fleet.txt`) — stdio `plugin()` assembles with the landed structural-row classification |
| 1 | 24 docx/xlsx lib reds after the peer's OPC `xml_parts` migration (coordinator counted 25) | **test half LANDED 17:34** (rule 22, 7 test files, landing row): 13 of the 24 were test code — xlsx Strict/Transitional analyzer/construction/composition tests built their packages in the BINARY lane (workbook.xml via `opc.set_part`, no root relationship) or relied on the old "encode regenerates Transitional XML" scope cut; docx `canonical_xml_authority…` pStyle red was an assertion bug (the edit is applied; the serializer wraps attributes, `contains("w:pStyle w:val=…")` missed it → quick-xml oracle now). **11 non-test reds → prepared p5** `lb2-p5-docx-xlsx-opc-reds.py` (12 files, dry run 0 problems; patched files rustfmt-clean with the repo config) + `lb2-p5-land.sh` (window 3: apply → check, auto-revert on red → both fixture writers → lib tests): xlsx demo `xl/styles.xml` → XML lane (fixed point proven); 4 grammar + 2 protocol facets rewritten to the real print forms (JSON-hex `set-snapshot`/cell `address`, `R[..]` error cells, JSON diff text `{}`, `OP_BINARY_FORMAT`+UTF-8 JSON frame); xlsx worksheet content-type check by ROLE (`XlsxSnapshot::worksheet_part_paths`, new `REL_TYPE_WORKSHEET_STRICT`) — the peer's `content_type.contains("worksheet")` filter could never see the violation; Strict VML ban over both lanes + new law; one `workbook_part_path`; dead imports/consts; docx assets regenerate through `zzz_write_native_docx_fixture` (demo already a fixed point, only relationships differ). All payload facets proven by temporary `lb2_probe` tests (removed): docx ops 18/18, xlsx ops 9/9, typed JSON diff grammars 4/4 each, utf8 diff protocols 4/4 each, worksheet-role prototype flags the retyped sheet and not the clean one (`c3-probe-1.txt`) |
| 2 | SDK table-kit half of the Home row-action fix (lands with WG11's wgpu TableRow painter) | **prepared, re-diffed 17:3x**: `lb2-p3-row-actions.py` dry run 5 files / 0 problems (`table_row_action` already always sets `label`; the law asserts it). WG11's painter `wp-wg11/wg11-table-row-painter-patch.py` is disjoint (10 files + 2 new, none of p3's). Joint window-3 runbook = `lb2-p3-wg11-joint-land.sh` (ONE native hold: p3 `--write` + WG11 `--apply` → check SDK + ui `wgpu-engine` `--lib --tests`, both halves revert on red → SDK table laws → WG11 ui laws → renderer-react typecheck + Interpreter vitest; rule-20 Home boot after) |
| 3 | ui table conformance fixture pair | **done 17:47**: Rust corpus 6/6 (14:33); the remaining TS red was a harness divergence (TS: deduped sorted set; Rust: ordered multiset) → TS aligned to the Rust definition + table TS test reads absent `children`; Interpreter vitest 144/144, typecheck 0 (landing row) |
| 4 | csv details-panel admission fix (p1) + verb-arg census law (p2 + p2b) | **window-3 list, re-diffed 17:3x**: p1 7 files / 0 problems, p2 1 file 3 hunks / 0, p2b 38 files / 0 |
| 5 | NEW (coordinator 17:5x, S18 served matrix) csv/tsv undo refused `stdio.table.cell-arguments: cell argument map capacity`, redo `ui.snapshot-details.arguments …` | redo = p1 (unchanged cause). Undo = same arena class in the TABLE path: the three stdio table argument builders reported arena refusals under their own codes (the SDK row window only ends early on `ui.fixed-capacity`), and `render_structural_table` built add-row/add-column AFTER the windowed body, so a drained arena took the controls (= the render) down. **Prepared p7** `lb2-p7-table-arena.py` (7 files, dry run 0; write/revert round trip exact in scratch): `table_arena(stage)` = `ui.fixed-capacity`, body passed as a closure and admitted after the controls (csv + tsv callers), law `🧪️tests/📊️table-arena-headroom` + fixture/schema (python-jsonschema valid; serde_json RFC 6901 oracle on every cell address; controls / third / half / complete headroom, en + de). Independent of p1 |
| 6 | NEW json/i-json `set-node` refused `snapshot-edit.schema-unregistered 's.stdio.json'` | **root-caused natively** (`c10-probe-1.txt`): after `semio_s_plugin_stdio::plugin()` NONE of `s.stdio.{json,xml,csv,tsv,txt,md,html}` is in the OS-wide schema catalog — `PluginBuilder::artifact(…)` (stdio, trinity, remodel, dag, writer, forms, mathematical, puzzle, fem, draw, playbook) keeps schemas/inferences/languages in `PluginRuntimeRegistry` marked "captured but unwired" and never publishes them; only `declare_artifact` does. Every stdio editor's snapshot-edit route (json `set-node`, every details-panel edit) resolves its schema there. **Prepared p6** `lb2-p6-declared-catalogs.py` (SDK `publish_declared_catalogs` after `commit_artifact_declarations` — schema + inference descriptors, same semantics; languages stay unwired on purpose: 51 single `register_language` overwrite sites + fn-address comparison would risk false conflicts at every describe; describe OUTPUTS unchanged, `ContributionSet` carries no schema catalogs; law in `shipped_fleet`: every shipped editor's document schema registered after `plugin()` + a revision-bound json `set-node` through the shipped editor lands, serde_json oracle — **witnessed failing on the current tree** `c12-p6-law-witness.txt`: `s.stdio.html … must publish`). Chain note: a conflicting intra-plugin descriptor in any `.artifact(…)` plugin would now fail its describe. **Test half LANDED 18:16**: json editor unit tests register the schema their fixture needs + 3 stale asserts → json 137/137 (was 7 red) |
| 7 | NEW xml file-source draft "every apply refused (DSL parser on XML text)" | **not reproducible on current source** (`c10-probe-1.txt`, `c13-xml-catalog-probe-2.txt`): `parse_dsl` falls back to XML import without a semio preamble; the curated catalog's rendered source applies (unchanged → no-op, text change → 1 mutation). The real current-source defect: the demo asset `🗣️.dsl.semio`/`🎒️.pack.semio` is the old 5-field wire → `xml_any_example_snapshot` silently opens an EMPTY document (`unwrap_or_default`), i.e. the served editor had no text nodes and an empty draft. **Landed 18:31** (test-only): law `the_natural_source_draft_applies_what_it_shows` (quick-xml oracle) + 2 stale xml tests; the 4 remaining xml reds = the asset → regenerated in window 3 by `lb2-w3-stdio-sets.sh` through `zzz_write_dsl_and_pack_fixtures`. Follow-up (not mine, 63 sites): every `PRIMARY_TEXT).unwrap_or_default()` example loader is a silent fallback |
| 8 | NEW duplicate DOM ids (tree `edit`, table `cell-N`) | the SDK keys are contract-correct (the ui contract: a key is "unique only among this node's own siblings (not surface-wide)"); the HOST's `uiNodeDomId = surface/key` assumes surface-wide keys. Keys stay (p3/WG11 `cell-<col>` contract untouched). **LANDED 19:01** p8 `lb2-p8-row-scoped-dom-ids.py` via `lb2-p8-land.sh` (typecheck 0, Interpreter vitest 146/146, Home boot 12.6 s / 0 pageerrors on serve 6630, stopped; `c16-p8-land.txt`): `UiInterpreterContext.domScope`, `uiNodeDomId(…, scope?)`, every `nodeDomId` call forwards the scope, tree rows hand their DOM id to their inline controls, table rows to their cells; laws in `🧪️tests/🪟️tree-windows` + `🧪️tests/📊️table` (Testing Library DOM oracle). Landed after S18's de run finished 18:59 (coordinator relay) |

#### Log 14c

- 16:58 start. Load 49 (chain), 8 rustc, 84 GiB free, native lane empty (2 slots).
- 17:0x reconcile done (item 0); shipped_fleet queued detached (pid 79643) → EXIT 0 17:14.
- 17:1x item 1 analysis: of the coordinator's list, the Strict/Transitional analyzers already READ `xml_part` — their tests built
  the workbook in the binary lane (test code); the composer tests encoded `snapshot.opc` directly (drops every XML part). Real
  non-test defects: xlsx demo styles part in the binary lane, stale grammar/protocol facets (both artifacts), the worksheet
  content-type filter (regression: it keys on the content type it is supposed to validate). Found in passing (not touched, report
  only): the peer's migration left ~312 dead-code items in docx/xlsx `🔺️diff` (old semantic diff engine + old text/binary codecs,
  185 docx / 127 xlsx `never used` warnings, `c1-check-docx-xlsx.txt`) → separate cleanup set for window 3+ (owner: peer/ST2).
- 17:2x temporary `lb2_probe` tests (test-only, env-gated `LB2_PROBE_DIR`) in both base unit files → all candidates proven;
  probes removed before the landing run.
- 17:34 item 1 test half landed (24 → 11 red, `c4-land-tests-1.txt`); p5 prepared + `lb2-p5-land.sh`.
- 17:3x coordinator relay (WG11 painter ready, one-step landing) → joint runbook `lb2-p3-wg11-joint-land.sh` written; p1/p2/p2b/p3 dry runs clean.
- 17:4x item 3: Interpreter vitest (`test long`) showed 2 reds on the edited table pair (TS action-id set semantics; plain row without
  `children`) → TS test-only fixes, 144/144 + typecheck 0. Found in passing: a stray empty directory literally named `$PWD` (27th 21:50,
  not mine) in `📺️renderer/…/⚛️react/📦️packages/🟦️typescript/` — reported, not touched.
- 17:5x coordinator: 4 new stdio items from S18's served matrix (rows 5–8 above). 18:0x native reproduction runs (`c7`, `c9`, `c10`
  probes; temporary `lb2_probe_*` tests, all removed — `git diff` of those files empty afterwards). 18:16 json test half landed; 18:24 p6 law
  witnessed failing; 18:31 xml test half landed; p6/p7/p8 prepared; window-3 runbook `lb2-w3-stdio-sets.sh` (xml regen → p6 → p7, each
  reverting on its own red) + `lb2-p5-land.sh` + `lb2-p3-wg11-joint-land.sh`; p8 = host TS lands with tsc + vitest + boot.
- 18:4x coordinator: p6 FIRST in window 3 with p1/p7 + xml regen → `lb2-w3-stdio-sets.sh` = xml regen → p6 → p1 → p7, each
  `--write` backed up and `--revert`ed on its own red (revert round trip verified exact on a scratch copy); all six sets (p6, p1, p7,
  p3, p5, p8) applied in that order on one scratch copy with 0 problems and every patched Rust file parsing. `.artifact(…)` census for
  the describe watch (failure mode = `plugin-assembly.declaration-schema|inference`): trinity, remodel, raster, flow, process, norm, cad,
  dag, stdio, sequence, procedural, vcs, imperative, sourcing, note, forms, architect, shooting, mathematical, layout, puzzle, fem, draw,
  playbook, lowpoly, energy (+ gis, space, writer, block, demonstrator sub-builders). p8 lands after S18's de matrix (`lb2-p8-land.sh`:
  apply → typecheck + Interpreter vitest → Home boot on serve 6630 → stop; revert on red). S18: its pins select by key/label, unaffected.
- 19:01 p8 LANDED (landing row): typecheck 0, Interpreter 146/146, Home boot 12.6 s / 0 pageerrors, serve 6630 stopped, backup dropped.

### Session 14c — phase 2 (2026-09-29): hosted kinds, runtime roots, editor catalogue

Coordinator items (06:10, widened 06:3x): 7 stdio roots register no document schema; a family package must be self-sufficient for
the kinds it hosts (ONE SDK mechanism: schemas + inferences + codecs + composers; hosting ≠ owning; law (d) per package in its own
process); `editor_catalog` other reds. Rules 25/26: captures/backups/scratch only under `.🧬semio/🌐hub/s14-lb2-*`.

| # | Item | Status |
|---|---|---|
| 9 | **p9 `lb2-p9-hosted-artifacts.py`** (window-3 set: SDK + stdio; 49 files) | see below |
| 10 | **p10 `lb2-p10-editor-catalog.py`** (rule 22, test-only: law window roster + fixture rows) | see below |
| 11 | **p11 `lb2-p11-editor-documents.py`** (window-3 set, stdio schema content) | see below |

#### p9 — design (hosting is not owning)

- **Root cause (native probe `c20-isolated-probe.txt`)**: after ST2's split each family is its own wasm guest (own process), but
  family packages declare no artifacts — the owner declarations run only in the `stdio` guest. In a process that assembled only
  `stdio-image`, png/jpg/bmp/svg schemas were absent (snapshot edits refused `schema-unregistered`), and so were the kinds'
  document codecs (`store::document_codec`: emit-op application, replay), composers (Import/Export), formats and subset validators.
  Independently, 7 roots (bmp, wav, epw, binary, ifc, gif, semio) were `definition_only_assembly` with `runtime_capabilities: []`
  — no guest published anything for them (their imperative `register()` is called by no plugin).
- **SDK `PluginBuilder::host_artifact(ArtifactDeclaration)`** (builder + `ArtifactDeclaration::preflight_hosted`): the owner's
  declaration (built by the owner crate's own `declaration(definition()?)`) commits in the hosting guest through the SAME
  transactional plan as owned declarations — schemas + inference descriptors (`publish_declared_catalogs`), composers, formats,
  subset validators, document codecs, dialect migrations (`commit_artifact_registration_plan`: identical rows tolerated, conflicting
  rows fatal). Preflight: the kind is owned by a DIRECT dependency (`plugin-assembly.hosted-artifact-dependency`), never by the host
  (`…-owner`: declare it instead), once per kind (`…-repeated`), every channel inside the kind (shared `preflight_channels`).
- **Hosting ≠ owning (W4 gate / hub catalog stay truthful)**: (1) the hosted definition is never registered by the host → the kind's
  identity, capability claims and the native artifact-catalog topic stay `stdio`'s; (2) inference SERVICES are not hosted — they
  stay listed/executed by the owner alone (only the inference DESCRIPTORS are published in-guest); (3) `describe` lists composers
  only for kinds the plugin id owns (`owns_artifact_kind`: `s.stdio-image` never matches `s.stdio.png`) → no io/composer rows move;
  (4) **codec rows** (W4 zero-codec gate, hub trusted catalog, `world actor` codec interface) are answered by `artifact_codec_owner`
  over the bundle's APPS — a family already owns the codec rows of the kinds its editors/viewers open (unchanged by p9); a HOSTED
  codec is a store-registry row (`store::document_codec`) the guest uses internally, never a descriptor/catalog row; (5) the
  declaration's own capability requirements union into the host manifest (none declared in stdio today).
- **Families**: each of the 9 families hosts exactly the kinds it activates on (29 `host_artifact` lines, generated from the
  `.activation(OnArtifactKind …)` rows).
- **7 roots → runtime declarations**: `declaration()` next to `definition()`; schema-first `runtime_capabilities` rows generated
  from the declarations' OWN `runtime_capability_requirements()` (scratch-only dump, `lb2-p9-dump-probe.py`) with the id/descriptor
  grammar all 29 existing runtime definitions follow (checker: 0 deviations) — bmp 5, wav 5, epw 5, binary 5, ifc 15 (4 + 2x3, 3 MVD
  validators), gif 9 (87a + 89a), semio 90 (19 subsets); representation rows publish the kinds' formats. semio's 19 subsets and ifc's
  3 MVD subsets gain `declare()`, the declarative twin of their `register()` (kept: native callers register without a plugin).
- **Ownership finding (fixed in p9)**: semio's bridge tables mix deserializers (write semio) and serializers (write step, png, … — 28
  foreign dialects). A composer capability claims the dialect it WRITES and a claim belongs to exactly one definition
  (`artifact-definition.conflicting-claim`), so semio declares only the rows writing semio (`semio_written`); semio → X exports stay
  X's to declare (they were never registered in the product: semio was definition-only).
- **Law (d)** `every_package_hosts_the_runtime_of_every_kind_it_opens_in_its_own_process` (`🚢️shipped-fleet`): the test re-executes
  its own binary once per package (`package_runtime_probe`, `#[ignore]`d child, env-selected); the child assembles ONLY that package,
  asserts it assembled (`package_id`), and checks every requirement row of the owner declaration of every kind the package activates
  on against the live registries (kernel schema/inference catalogs, store document codecs, io composers/formats/subset validators;
  grammar rows are captured-never-published by design).
- **Schema documents (added 08:1x, coordinator: "target 91/91")** — same set, same commit path: framework schema
  `register_artifact_inference_descriptor` also registers the inference document as the `inference` export of its scope;
  SDK `ArtifactDeclarationBuilder::schema_documents(ScopeSchemaExports)` (one `schema` row `<scope>.<export>` each, scope must be
  the declaring kind — `plugin-assembly.schema-documents-owner`; hosted like every other channel); the kernel's store and io
  vocabulary register their own documents (`os.store`: child/link/blob, `framework.io`: schema — kernel gains the
  dependency-free `semio-framework-schema-registry`) from `publish_declared_catalogs`; semio declares
  `SEMIO_SHARED_SCHEMA_DOCUMENTS` (geometry, child). LAW (e) `every_registered_snapshot_contract_resolves_in_each_package_process`
  (child process per package: every registered artifact's snapshot contract compiles). Cargo.lock gains the two dependency
  edges (kernel → schema-registry, stdio dev → schema). p9 = 56 files.
- **Not in p9 (reported)**: conversion features — the stdio-semio family does not enable semio's `conversion-*` features, so its
  guest hosts the semio composers compiled into ITS build (the deserializing bridges only exist where the feature is on); enabling
  them is a packaging/wasm-size decision. `definition_only_assembly`/`ArtifactAssembly::Definition` lose their last production
  callers (all 36 are runtime now) → a separate cleanup set (contract + registry + 36 `assembly()` fns).

#### p10 / p11 / p12 — `editor_catalog` toward 91/91

Baseline after p9 (scratch 07:3x, `p9-s7-all.out`): 42/91 (was 1/89 on ST2's T2 run): p9 cleared `schema-unregistered` ×28 and
p10's window roster cleared `window-config.window-context` ×25. Remaining classes and their owner set:

| Class (editors) | Root cause | Set |
|---|---|---|
| `window-config.window-context` (25) | law rendered `window_id` with an EMPTY `window_instances` roster | p10 (test) |
| fixture rows (gltf, obj, pdf 1.4 ×3, xlsx ×3, json ×2, txt, csv, mp3, docx ×3, png, xml, gif) | rows written for older snapshot shapes, or edits a native file cannot carry on an empty document (csv header flag, 3-byte ID3v1, `trailingNewline` with no line, docx namespace rewrite) | p10 (test) |
| identity law (new) | `E::DOCUMENT_SCHEMA` == initial snapshot `schema` | p10 (test) |
| `snapshot-edit.schema-identity` (pdf ×10, gif 89a ×1, avi ×1) | pdf/gif-89a apps edit the canonical model but declared the legacy model's schema; avi `Default` had an empty schema | p12 (pdf, gif), p11 (avi) |
| `invalid-schema-contract` (las, dwg ×2, ifc 4 once published) | broken `$ref`s (`#/definitions` vs `$defs`, camelCase vs `Dwg…` names) | p11 |
| `constraint-invalid` (dxf, dwg ac1018) | schema requires what the model omits (`DxfTables`), `dwf_3dPrecision` vs projected `dwf3dPrecision` | p11 |
| `schema.fragment.invalid` (png, xml ×2, ifc 2x3 ×3, binary) | Option fields projected `null` against optional non-null contracts; binary `bytes` typed string | p11 |
| default encode panics (jpg ×2, tiff, gif ×2) + epw/wav/pptx new documents not reopening | a new document of those kinds was not a valid native file / not a codec fixed point | p11 (`blank_*_snapshot`, wav default data, pptx `build_minimal_pptx`) |
| semio brep / kit / object (3) | their snapshot schemas `$ref` semio's shared schema documents (`base/geometry.json`, `base/child.json`, `brep/inference.json`) that no descriptor facet registers — the validator's cross-document scope is the schema export registry | **open**: needs a declaration channel for an artifact's shared schema documents (SDK) or moving the shared `$defs` into the registered `base/artifact.json` facet — reported, not in these sets |

Landing order (shared files): p9 → p10 → p11 → p12 (p9/p12 both edit the gif root; p11/p12 both edit the gif 89a editor/viewer).
p10 is test-only (rule 22), disjoint from the guest sets. p11/p12 are guest sets for T6 with p9 (L1 manifest: p9 → p11 → p12).

- **p12 descriptor consequence**: the pdf (20 apps) and gif 89a (2 apps) `io.artifactSchema` change → `stdio-pdf` and `stdio-image`
  descriptors (and their hub codec rows: `stdio.pdf` → `stdio.pdf.1.7`, 89a `stdio.gif` → `stdio.gif.89a`) must be regenerated
  by describe in T6; p9 alone leaves every family descriptor unchanged. Follow-up (not in p12): the host-native pdf codec
  (`stdio.native.pdf.v1`, receipts → hub bootstrap generation) still materializes the 1.4 `PageDoc` stub under `stdio.pdf`, which
  no editor opens — mapping native open onto the canonical 1.7 model is a hub-generation change for H14/L1.
- **semio shared schema documents (open)**: `brep` refs `brep/inference.json` (inference documents are not registered as schema
  exports), `object` refs `base/geometry.json`, `kit` refs `base/child.json` → `os/store/child.json` (neither is a registered
  export: artifact descriptors register only their 4 facets). Proposal: (1) `register_artifact_inference_descriptor` also
  registers the inference document as an export of its scope; (2) an `ArtifactDeclarationBuilder` channel for an artifact's
  shared schema documents (schema-first capability row, committed with the other catalogs); (3) the store registers its own
  schema documents (`os/store/child.json`, `link.json`) as framework exports. Until then these 3 editors stay red.
- **Oracle** `lb2-initial-snapshot-oracle.py` (python-jsonschema + referencing, third-party): validates every editor's NEW
  document (scratch dump) against its own snapshot contract; on the 17 early dumps it reproduced the Rust findings exactly (dxf
  required tables, las ref, png nulls, xml root, avi identity). `--refs` mode (third-party twin of law e, one global registry of
  every `$id` document): live tree 3 of 57 snapshot contracts carry unresolvable `$ref`s (dwg, ifc, las) → scratch with p11: 0.

### Session 14c — phase 2b (2026-09-29 08:3x–): s9 proof red → p9 grows, p13/p14 new, p10–p12 extended

**s9** (scratch, p9+p10+p11+p12, 08:33, `s14-lb2-captures/p9-s9-catalog.out`): editor_catalog 59/90, shipped_fleet 4/6, stdio lib 7/8.
L1 holds p9/p11/p12 out of T6 until a green proof (coordinator 08:5x: all T6 sets tracked in `📓️t6-queue.md`; rows 3–5c are LB2's).

| s9 class | Root cause | Fix (set) |
|---|---|---|
| law e: `stdio-office` alone cannot compile docx/xlsx snapshot contracts | their `DocxXmlPart`/`XlsxXmlPart` `$ref` the xml snapshot document; office hosted only the kinds it opens | p9: a family hosts the kinds it opens **plus their schema closure** (`$ref`s of their snapshot contracts, transitively, computed from the tree: office → xml); office Cargo.toml + lock edge `semio-s-plugin-stdio-office → semio-s-artifact-stdio-xml` |
| law d: `stdio-semio` alone: `schema s.stdio.semio.geometry/child` unmet | the schema-document claim `<scope>.<export>` has the exact shape of a subset schema id (`s.stdio.semio.brep`) — ambiguous, and it would collide with a subset named `child` | p9: own claim namespace `schema-export`, value `<scope>#<export>` (`ArtifactIdentityClaim::schema_export`, `ArtifactIdentityNamespace::schema_export`); stdio contract allow-list; semio rows `…schema.schema-export-s-stdio-semio-{geometry,child}.v1`; law d resolves them through `resolve_schema_export` |
| laws d/e stopped at the first red package | loop asserted per package | p9: laws collect every package's failure |
| stdio `descriptor_is_fresh` | the 7 runtime roots now publish formats (`.bin`, `.bmp`, `.epw`, `.gif`, …) | expected: T6 describe regenerates stdio (+ stdio-pdf, stdio-image after p12) |
| 22 new `unused_qualifications` warnings | p9 generated qualified paths where the names are in scope (semio `ComposerEntry` ×17, binary `ArtifactDeclaration` under the feature-gated prelude, gif 89a codec types); p12's root re-export made the pdf codec line's path redundant | p9 emits the in-scope names (binary imports `ArtifactDeclaration`; p9 re-exports `STDIO_GIF89A_DOCUMENT_SCHEMA` beside the 89a codec it declares, p12 adds it only if absent); p12 shortens the pdf root codec line |
| pdf ×10 `path-invalid … no child metadata`; semio kit `/types`, object `/transform` | `FromValue::edit_value_at_path`'s default edits only the root; these three codecs are hand-written | **p13**: value crate `edit_through_value` (edit the emitted value tree, decode back; re-exported by `pack::value`, `os_dsl::schema`, kernel root; serde_json-oracle unit law); the three codecs route path edits through it |
| xlsx `*` text replay: bytes differ | `OpcPackage::relationships: HashMap` — per-instance order, one document had two encodings (strict/transitional passed by chance) | **p13**: `BTreeMap` (JSON/contract unchanged), docx/xlsx/pptx relationship diff helpers follow |
| xml_any `publication-codec … no record for 'set-snapshot'` | the `set-snapshot` leaf (tag 7) joined `XmlMutation` but not its aggregate wire surfaces; every xml editor maps every edit to `SetSnapshot` | **p14**: protocol record, binary/text rosters, grammar roster, aggregate schema arm — all from the leaf descriptor |
| xml_valid `publication-mismatch` | the family row writes a root without DOCTYPE; the valid subset (correctly) refuses it (§5.1), the refused edit is a no-op | p10: valid row = `<!DOCTYPE root><root/>` |
| gif89a `interactive-job.catalog-authority` (schema_eq=false) | tool proofs declared `artifact_schema: "stdio.gif"` | p12: `stdio.gif.89a` |
| pdf pack round trip; png `IHDR: zero dimension`; avi `reserved` `[]` vs `[0,0,0,0]` | new documents were not codec fixed points: pdf `Default` has no retained COS graph (decode always carries one), png 0×0, avi's writer emits `dwReserved[4]` | p12 `blank_pdf_snapshot` = `decode(encode(default))` for all 20 pdf apps; p11 `blank_png_snapshot` (1×1 white), avi `Default` reserved `[0; 4]` |
| dwg ×2 `constraint-invalid … paperUcsName expected integer` | 16 optional handles projected `null`; contract + TS: optional non-null | p11: `skip_serializing_if` (region-guarded to `DwgHeaderRelations`) |
| docx ×3 `publication-invalid … expected WordprocessingML document root` | per-subset fixture rows (precedence over the family row) rewrote the `w:` namespace | p10: rows retired → family row (empty `w:p`) |
| ifc2x3 ×4 `no-op fixture path`; pptx save/reopen; semio brep `publication-mismatch` | rows addressed a field a new document omits (`edmPreamble`, p11), a typed slide without its parts, the `nextLabel` allocator (not in the brep diff) | p10: `/document/header/file_name/0`, `/opc/comment`, brep `/vertices` |
| pdf rows | a typed-lane edit re-lowers the retained graph → the reopened `objects` differ | p10: `/declaredVersion` (keeps the graph canonical) |

Order: p9 → p11 → p12 → p13 (shares the pdf 1.7 snapshot file with p12) · p14 independent · p10 last (rows address the fixed documents).
Scratch reset 09:00 (resync + all six sets written, 0 problems); proof job **s11** = `lb2-scratch-proof.sh p9-s11` (catalog + dump,
families descriptors, SDK builder `--test-threads 1`, schema/registry/contract libs, value `edit_through_value`, 20 roots' libs).
- **s11** (09:28–09:33): SDK `builder::` **20/20** serially (the s8 reds `strict_artifact_identity_{mixed_channels_publish_nothing,
  all_builder_channels_reject_before_publication}` are a PRE-EXISTING parallel race: their witness differences the registries
  around a rejected build while the sibling `…_owned_tree_and_definition_channels_publish` publishes the same tree on another test
  thread — both flip all-false → all-true within 0.02 s; not a p9 defect; follow-up: serialize the three laws); schema 29/29,
  schema-registry 6/6, stdio contract 46/46 (the `schema-export` allow-list), value `edit_through_value` 1/1. catalog / families /
  roots did not build: docx's edit-preparation memory bound called `relationships.capacity()` (a B-tree has none) → p13 counts
  entries (13 files). Coordinator 09:1x: scratch build outputs deleted for disk (32 GiB) → s12 builds cold.
- **s12** (10:25–10:36): **shipped_fleet 6/6 — laws d + e green** (every package alone hosts the runtime of what it opens, every
  registered snapshot contract compiles in every package process); editor_catalog **74/90**; families `descriptor_is_fresh` 7/9 —
  stdio-image and stdio-pdf stale = p12's intended `io.artifactSchema` change (describe in T6), office unchanged with xml hosted;
  stdio lib descriptor stale (describe). Remaining 16, all understood (python-jsonschema oracle agrees on the contract ones):
  - pdf ×10 `constraint-invalid … objects[0].value`: the 1.7 contract (+ TS twin + TS-embedded copy) describes `PdfObject`'s
    `real`/`ref` arms as `{kind, value}` while `#[value(tag = "kind")]` flattens an object payload beside the tag (the derive's
    documented rule) → p12 describes the declared encoding (`allOf` payload + tag; oracle: 0 violations on the new document).
  - png: the writer emits interlace method 0 only → an `interlace` edit never saves → p10 edits `/pixels`.
  - xml valid `inverse-mismatch`: the undo lands on the empty (invalid) document, which the subset refuses → p11
    `blank_valid_xml_snapshot` (`<!DOCTYPE root><root/>`), p10 row edits text inside it.
  - ifc 2x3 ×4 `$.schema: expected object`: the contract swapped `schema`/`document` (twins correct) → p11 swaps back; its
    `Part21*` definitions (kind-tagged, camelCase — twins agree) describe an encoding the SHARED `step::part21` types (external
    tagging, snake_case, also used by step) never produce → **open, not in these sets**: the step owner decides whether the shared
    type follows the contract or ifc 2x3 `$ref`s a step-owned contract of the real projection.
  - roots step broke mid-build: my 10:37 scratch resync removed sources under the running job (mine; s13 reruns it).

- **s13/s14 + live baseline** (native lane `p9-live-baseline-roots-2`, the 12 affected root crates on the live tree): s14 editor_catalog
  76/90 (pdf ×10 = U6 row 6b wire gap; ifc2x3 ×4 → p16), shipped_fleet 6/6; root libs 82 → 12 reds. Against the live baseline
  (bmp 1, epw 2, pdf 1, png 1, pptx 1, zip 4 red today): mine were jpg/png `committed_json_is_canonical` ×24 + dwg ×2 + ifc ×1
  (p11 now converts the 59 committed fixtures), pptx strict/transitional analyzer laws ×7 (p11 no longer changes pptx `Default` —
  `blank_pptx_snapshot` for the six apps), ifc shadow-state keys, jpg/png optional-field laws (`InsertValue` for absent members),
  png typed-source (replaced itself); U6's relayed items (png, epw set-cell, pptx honesty after the demo writer) are covered.
  Pre-existing and NOT in my sets: bmp `ops_grammar` (set-snapshot grammar drift, p14's class), epw render, pdf kind roster, zip ×4.
- **p15** (coordinator 11:4x, publish-all refused the 9 families: "component codec probe requires at least one declared artifact
  kind"): hosted kinds end to end — `PluginManifest.hostedArtifactKinds` (framework manifest + TS projection, typegen 198),
  `ArtifactDeclaration::hosted_kinds`, hub trusted-catalog pairing + `open_target_codec_package` (owner present, declared
  dependency, exact codec; named refusals; bundle validation, load, manifest check, creation selection), publish script
  (`trustedBootstrapHostedKindsV1`: a hosting package skips the probe, binds the owner's rows; a hosted pair without an owner
  row opens nothing — the owner publishes ONE codec row per kind). Laws: shipped_fleet hosted-descriptor law; hub descriptor
  open-target fixture case `hosted` + `hosted_open_targets_bind_the_owner_codec_or_are_refused_by_name`. Proof in scratch2
  (scratch + 🌎️hub/gis/vcs, p15 applied; job `p15-s1`).
- **p16** (coordinator: "you own the Part21 decision"): see the script docstring; IfcOpenShell 0.8.4 is installed → oracle over
  the 43 committed IFC 2x3 fixtures (ids, types, argument counts, instance count, FILE_SCHEMA). Proof in scratch1 (job `p9-s16`).

### Session 14b

Successor agent (2026-09-28 12:0x, after the usage cut + app restart). Guest freeze ON since 12:02:46 → every guest-linked item is a
prepared patch (dry-run clean on the live tree) + an overlay proof; landing in window 3, compile-atomic.

| # | Item | Status |
|---|---|---|
| 0 | Live tree carries no half-applied LB2 guest edit | **verified 12:1x**: p1/p2/p2b `--dry-run` report no "already applied" hunk; `/usr/bin/grep` for every LB2 marker (`take_arena_unbuilt_rows`, `ArenaStarvedSurfaces`, `details_arena_headroom`, `command_bridge`, `UNDECLARED_ARGUMENT_VOCABULARY`, …) over `🧰️framework` + `✏️s`: 0 hits; the 5 payload files do not exist |
| 1 | stdio csv redo `snapshot details UI admission failed` (p1) | patch re-anchored on the overnight tree (SDK export block reformatted), dry run **7 files / 0 problems**; overlay synced + applied 12:28; law + details unit tests queued (overlay lane, hold A) |
| 2 | stdio html/json/md/txt/xml "no snapshot schema is registered" (5 MCP mutations) | root cause gone from the tree: the Codex peer's `5bcb2da23da` resolves the edit schema from the editor's DOCUMENT schema (`s.stdio.<kind>`, registered by every one of the 5) instead of `{kind}.{standard}.{base\|any}` (never registered); native proof queued (`editor_catalog` laws of the text family, native lane) |
| 3 | Verb-arg law (p2 + p2b) | p2 re-anchored (the live bridge law now also skips `CANCEL_TYPED_OPERATION_ACTION_ID` → one `framework_owned_verb` predicate shared by both laws), dry runs 0 problems; overlay census queued (hold B) |
| 4 | LD leftover: conflict probe de 4/5 | probe criterion = committed DOMAIN ops on a census socket (C13's `HubProbeDocument.relayedEnvelopes`); the WAL reader is superseded; live run after the chain (item 6) |
| 5 | lb-p1 / lb-p4 still green | check default `--lib --tests` stdio plugin + semio artifact **EXIT 0** 13:43 (`i5-check-default-2.txt`); **`shipped_fleet` RED 14:25** (`i5-test-shipped-2.txt`): stdio `plugin()` panics — the peer's `structural_table_window_kind` rows (add/remove row/column, set-header) are `Unclassified` → csv/tsv (shipped) + wav fail assembly → CHAIN RISK (descriptors). Fix prepared **p4** `lb2-p4-structural-classification.py` (stamp `Migrated`; dry run 0 problems), overlay proof queued, escalated to the coordinator for W4 / priority. semio brep lib + `editor_catalog` + brep parity still queued |
| 6 | Item-3 live run orchestration | `live/run-conflict.sh up\|probe\|down` complete (own vigilant hub 8130 + link proxy 8131/8132 + serves 6630/6631); run scheduled after the chain publish (keeps the machine idle for the critical path) |
| 8 | docx/xlsx lib tests after the peer's OPC + `xml_parts` snapshot move (coordinator 13:38, rule 22: test-only, may land now) | **edited in the tree (test-only)**: docx — quick-xml 0.42 names are `&str` (7 sites), `🧹️clear-main-declaration` include path one level too deep, `encode_docx` via `crate::engine`; xlsx — mutations unit tests (projection via `project_workbook`, cell edits through lineage-bound `xlsx_cell_address`, absorb law re-expressed on XML-part deltas, field sweep restored over BOTH lanes incl. binary parts/content types/part-owned relationships in the `#[cfg(test)]` sweeps), schema unit tests (SST/unmodeled parts as XML authority, shrinking via `RemoveSheet`), outline + result-apply tests, set-snapshot quintet test (new XML-part diff assertions + `#[ignore] zzz_write_committed_quintet` generator + drift law); native check → generate → lib tests queued (`t8-docx-xlsx.sh`) |
| 9 | S18 matrix: 6 stdio editors (csv, tsv, json, json/i-json, xml, xml/valid) newly red, edits `[0,0,0,0]` (coordinator 14:1x) | **root-caused, no guest patch**: not S20's initializer (all four families override `build_document_store_initialization_job` with the bounded job; the example loads — `renderAfterRedo` shows the demo rows; "No data" is `render0`, captured before the pre-verb exactly as in S16's green run). The peer made `set-cell` (stdio `structural_table_window_kind`) and the SDK `set-node` revision-bound overnight — `revision` is DECLARED required (schema-first, consistent; every rendered cell/node binding carries it) — while the matrix pins `stdio.set-cell {row,column,value}` / `set-node {nodeId,value}` carry no revision → `command_from_action` refuses → 0 edits. Fix = S18's harness (resolve the live revision from the rendered binding, or drive the cell input); relayed |
| 10 | ui conformance corpus `🧩️component/📊️table` (peer 00:21 snapshot vs 09-25 expect, coordinator 14:1x, rule 22) | **edited in the tree (fixture-only)** by `lb2-t10-table-corpus.py`: snapshot brought to the agreed TableRow contract (the two `row-action-0` child buttons dropped — `rowActions` props stay the one representation; the editable row keeps its `cell-0`/`cell-1` Commit-bound, revision-guarded inputs), expectation DERIVED from the snapshot alone (nodeCount 5, shape, accessibility, actionIds openSpace + 2× set-cell; no generator existed — the script is it); Rust corpus harness queued (native lane, `t10-ui-conformance-1`) |
| 7 | Home rows carry each row action twice → item capacity 27 < 32-row viewport (S18 via coordinator; SDK half mine, wgpu TableRow painter WG11, land together) | prepared `lb2-p3-row-actions.py` (5 files, dry run 0 problems live + overlay): SDK drops `table_row_action_buttons` (both call sites), React Interpreter drops its `row-action-` key filter, law `home_shaped_rows_carry_actions_as_props_and_fill_the_default_window` + fixture/schema `🪟️window-kits/📊️table/🧫️fixtures/🏠️row-capacity` (python-jsonschema valid); contract agreed with WG11 (unchanged `TableRowProps`); overlay law queued (hold B) |

#### Log 14b

- 12:09 start. Load 37, swap 4.5/6 GiB, 100 GiB free; native lane 5 deep (g12 holding), overlay lane free, wasm = chain.
- 12:1x last night's overlay proofs never compiled: the overlay was cloned 18:44 while the kernel was red (Codex, E0282/E0308 ×74) —
  `o-p1-proof-1`, `o-p2-proof-1`, `o-probe-2` all stop in `semio-framework-os-kernel`. `i4-test-laws-1` (shipped_fleet) **EXIT 0**
  21:04; `i4-test-semio-lib-1` cut at the restart (no EXIT).
- 12:2x overlay synced onto the live tree (`overlay-sync-2.sh`: clone changed files, top-up now also refreshes stale gitignored sources
  such as `🎨️styling/🔤️tokens/🦀️.rs`, NEW `lb2-overlay-prune.py` removes files the live tree no longer has); p1 + p2 + p2b applied
  (0 problems). Found: `📜️fleet-mutex.sh` waiters survive SIGTERM (the TERM trap only removes the ticket, the loop keeps waiting and
  would take a slot once the queue drains) — my two stray waiters were SIGKILLed (pids 63829, 64661, both mine). Coordinator fixed the
  mutex (exit on INT/TERM).
- 13:21 overlay hold A (p1 + p2 + p2b applied, private build-dir, 1 m 51 s): the SDK (incl. the reactor-turn re-dirty) and the stdio
  contract COMPILE; details unit tests **21/21** (`o2-p1-unit.txt`, EXIT 0); p1 law **FAILED on case 4** (`o2-p1-law.txt`): "3 rows under
  40 free collections, 3 complete" — cases 1–3 passed en + de, but the fixture's absolute `freeCollections` were calibrated on last
  night's details costs; the peer's rewrite shows untyped add controls only where `allows_collection_insert` holds (default false).
  Law made self-calibrating: each case measures what its complete render holds (`complete_cost`) and leaves a share free (`none`,
  `third`, `half`, `complete-plus-one-row` — the last must render WHOLE, i.e. no over-reservation); the test provider mirrors the DSL
  provider's `allows_collection_insert`. Fixture v2 validated with python-jsonschema 4.25.1. Re-run queued (hold B).
- 13:1x item 2 root cause (read, not yet proven): at G11's run (7800 B3, built 09-27 13:56) `snapshot_schema_descriptor_for_dialect`
  looked up `{kind}.{standard}.{base|any}` (e.g. `s.stdio.html.5.any`) — never registered; the text kinds register `s.stdio.<kind>`.
  Codex `5bcb2da23da` (27 21:54) resolves from the editor's `DOCUMENT_SCHEMA` (`stdio.html` → `s.stdio.html`, owner-checked).
  The stdio `editor_catalog` gate carries a peer `println!("[DEBUG] editor=…")` (not mine; rule 19: nobody edits that gate now).
- 13:3x item 7 (coordinator): the duplicate exists because the wgpu reconcile never painted `TableRowProps.row_actions` (TableRow with
  children → bare horizontal Stack, cells dropped; childless → one Button, no actions). Split agreed: SDK half = p3 (mine), wgpu
  TableRow painter = WG11 (Table → retained Tree section, TableRow → tree item with column cells + trailing actions); contract deltas
  relayed (editable rows keep exactly one child per materialised cell `cell-<col>` / draft / read-only surface; `table_row_action`
  always sets `label`). Also: the live SDK test `table_kit_render_rows_builds_one_table_node_with_one_record_per_row` asserts
  `row.children.is_empty()` — red on the live tree since the duplicate landed; p3 makes it true again.

## Session 14

| # | Item | Status |
|---|---|---|
| 1 | stdio csv redo `snapshot details UI admission failed` (details panel vs `UiValue` arena / `ui_value_headroom`) | prepared `wp-lb2/lb2-p1-arena-budget.py` (7 files incl. the reactor follow-up render; dry run 0 problems on tree + overlay); overlay probe + law run queued (overlay lane; load 86–97) |
| 2 | Verb-arg law (LB item 4): sound bridge-law census + per-app argument declarations | prepared: SDK law + type-erased `command_bridge` (`lb2-p2-declared-arguments.py`, 3 hunks) + one call per plugin (`lb2-p2-plugin-calls.py`, 38 files / 31 plugins); dry runs 0 problems; overlay census (note, block, puzzle, space, wfc) queued |
| 3 | LD leftovers: conflict probe de 4/5 criterion (winner's check-in transition), other open LD items | probe criterion rewritten (`wp-lb2/live/probe-s12-conflict.mjs`: committed DOMAIN ops from the hub WAL, transitions excluded); reader `wp-lb2/wal-ops` build queued (overlay); live re-run after that |
| 4 | lb-p1 (brep casing) + lb-p4 (stdio shipped guard, editor_catalog required-features) still green | **check green** 18:41 (`i4-check-default-1.txt`, EXIT 0, 7 m 20 s); law/lib/parity queued (native lane 6 deep) |

### Log

- 18:27 start. Read AGENTS.md, preambles 14/13/12, `📓️fleet-14-agents.md` (no "CHAIN LAUNCHED" yet), `📓️wp-lb.md`, `📓️wp-ld.md`,
  `📓️fleet-13-agents.md` from 14:00. Load 5.2, 0 rustc, 141 GiB free, swap 7.6/9.2 GB, native lane empty.
- 18:3x item 4: lb-p1 and lb-p4 are both still in the tree (brep enums carry `rename_all_fields = "camelCase"` ×3; stdio
  `editor_catalog` `required-features = ["full-app-catalog"]`, `shipped_fleet` test target, `artifact-app-testing` dev-dependency).
  Native re-verification chain launched detached (`wp-lb2/item4-verify.sh`, pid 40311): check default `--lib --tests` of stdio plugin +
  semio artifact, `shipped_fleet` law, semio lib brep tests, brep parity (`i4-*.txt`).
- 18:41 item 4 `cargo check --keep-going -p semio-s-plugin-stdio -p semio-s-artifact-stdio-semio --lib --tests` (native lane, build-fleet-b)
  **EXIT 0** (`i4-check-default-1.txt`): lb-p1 + lb-p4 still compile after the later peers' edits.
- 18:4x item 1 evidence (S16 capture `🧑‍💻dev/🤖️generated/🧮️program-matrix/s16-r1en/matrix.json`, row stdio/csv): en FAIL, de PASS (same
  build). The redo applied (`edits=[0,1,0,1]`) but its dispatch was refused: `input #52 redo refused: dispatch-failed … —
  ui.snapshot-details.arguments: snapshot details UI admission failed`; after the redo the details window shows only
  `Source Details` (14 chars). The code is `ui_args` (`🪟️details/🦀️.rs`): `UiMapBuilder::try_new()` → `None` = the process-wide
  `UiValue` arena has no free collection. Arena (`🖱️ui/🧬️contract/🎬️action`): ONE page = `UI_VALUE_PAGE_ROWS` (128) ×
  `UI_VALUE_ROW_COLLECTIONS` (5) = 640 collections / 1 792 pages. The SDK windowing (`tree_window_indexed_rows`) already ends a
  window early on `ui.fixed-capacity`, but the details panel reports arena refusals under its own code, so the refusal escapes as a
  fatal assembly error; nothing in the details panel reads `ui_value_headroom()`. Cost from source (demo 3×2 csv, schema not
  resolved in the guest — the capture shows the 7 untyped add buttons on the root): collection rows 9–11 maps (7 add buttons + template +
  up/down/remove), scalar rows 3–4 → ≈175 maps / 28 rows ≈ 6.3 per row (priced 5). Retirement of a previous generation is
  asynchronous (exact owners paged by the reactor, 256 units/turn), so a quick set-cell → undo → redo keeps several generations live;
  en (29 s) failed, de (41 s, more idle turns) passed — consistent with retirement lag. To be measured in the overlay probe.
- 18:44 overlay `.🧬semio/🌐hub/s14-lb2-overlay` = APFS clonefile of 77 770 tracked + untracked files (`wp-lb2/lb2-overlay.py`, 5 m 26 s,
  no data copied); 18:55 first build refused (`🎨️styling/🔤️tokens/🦀️.rs` is gitignored + generated) → `wp-lb2/lb2-overlay-missing.py`
  cloned the 2 587 missing source-like files (1 m 40 s). Probe test (`lb2_probe`, overlay-only, appended to the details unit tests):
  per-render collections/items/rows for csv 3×2 and 40×5, typed and untyped, generations admitted until refusal, drain back to base.
  Runner `wp-lb2/overlay-cargo.sh` (overlay lane, private build/target dirs `.🧬semio/🌐hub/s14-lb2-{build,target}`).
- 18:5x coordinator: a Codex peer edits `🪟️details` (in-flight 18:49: `ValueShape` provider, schema lookup) → my change stays narrowly in
  the admission path, re-diff before landing. Found + reported: the Codex stdio `📜️script.ts` TestScript gate asserts 36/36 shipped
  formats + 88 playground rows (fails on the lb-p3 tree, 7 formats / 9 rows) → coordinator: ST2 owns that conflict; not touched.
- 19:2x item 1 design (patch p1, `wp-lb2/lb2-p1-arena-budget.py`, `--dry-run` 6 files / 0 problems on the live tree and on the overlay):
  (a) SDK `tree_window_indexed_rows` starts a row only while `ui_value_headroom().rows() > 0` (the arena is ONE page shared by
  every panel and by the generations still being retired; a stop there stamps the full extent like every short run);
  (b) details `ui_args`/`pointer_argument` report the 4 arena refusals as the framework's `ui.fixed-capacity` (new
  `fn arena(stage)`; the other `ui.snapshot-details.*` codes stay), so the existing early-end path of the windowing catches them;
  (c) law `🧬️contract/🧪️tests/🎟️details-arena-headroom` (own test binary = own arena, no interference from parallel unit tests) +
  fixture/schema `✏️editing/{🧫️fixtures,🧬️schema}/🎟️details-arena-headroom`: csv-shaped documents, the arena held down to N free
  collections, render en + de → Ok, the details section stamps its full extent, fewer rows than the idle render, every bound
  pointer resolves in the document by serde_json's RFC 6901 `pointer` (third-party oracle; inserts: their parent), then the
  credit returns and the complete window comes back. No UX change, no arena growth.
- 19:2x finding (for the Codex details owner, not changed): the S16 capture's first details render shows the 7 untyped add
  buttons on the csv ROOT, although the csv snapshot schema (`additionalProperties: false`, typed `items`) would give none —
  i.e. `snapshot_schema` did not resolve `s.stdio.csv` in that guest render (HEAD code). Typed, the demo costs ≈ 81 maps
  (≈ 2.9 / row, within the 5 priced); untyped ≈ 175 (≈ 6.3 / row). The peer is rewriting `snapshot_schema` right now.
- 19:3x item 2 design (patch p2, `wp-lb2/lb2-p2-declared-arguments.py`, dry run 1 file / 0 problems): the SDK bridge law that 21
  plugins already call (`assert_declared_actions_bridge_to_commands`) also asserts `declared_verbs_reading_undeclared_arguments`
  — base = staged declared defaults; probe = each key of `UNDECLARED_ARGUMENT_VOCABULARY` ∪ every key the app declares anywhere,
  not declared by this verb, under 5 value shapes; any different answer (other encoded command / refusal ↔ command) is an observed
  read. The framework-owned verb list becomes one const shared by both laws (its rationale leaves the loop body). Heuristic re-run on
  the current tree: 508 candidates (`heuristic-census-1.txt`); e.g. block2d's 9 include `addHandleKind`, whose command is the empty
  `AddHandleKind {}` (a heuristic false positive).
- 19:3x item 3: harness copied to `wp-lb2/live/` (captures → `wp-lb2/generated`, data roots `s14-lb2-*`). Criterion rewritten: the
  head also moves for every history transition (auto check-in, undo), so "the hub committed only the winner's edit" now counts the
  document's committed DOMAIN operations (after − before, read after the head is quiet for 8 s) with `wp-lb2/wal-ops` (ticket-local
  reader: db `replay_document` + `decode_wal_command`, commands inside committed transactions only, classified by the replication
  crate's `is_history_transition`); the normal-policy row requires 2 domain ops from 2 actors. `bun build --no-bundle` parses.
- 20:0x item 2 coverage: only 15 hand-written bridge-law calls exist (10 plugins; `bridge-callers-census.py` → `bridge-callers-census-1.txt`),
  so a per-app law alone would leave most apps unchecked. p2 therefore adds `command_bridge` to `ArtifactCodecTableV1` (the per-app
  table of type-erased answers every registration already records — one construction site, `artifact_codec_table::<A>()`) and
  `assert_every_registered_app_reads_only_declared_arguments(plugin)`, which probes every app a plugin registers (editors + viewers of
  every declared subset, every `PluginBuilder` app). p2b (`lb2-p2-plugin-calls.py`) adds ONE call per plugin: appended to the 28
  mounted `🧪️tests/🔬️surface` files; process + forms get the surface mount, the file and the SDK `artifact-app-testing` dev feature;
  stdio gets `🧪️tests/🔑️declared-arguments` + `[[test]] declared_arguments`. The typed bridge law keeps calling the same check.
- 20:2x p1 extended (same script, now 7 files): a short window would stay short — the React host's tree-window request is a function
  of geometry only (`treeWindowBodyRequestsV1`), so after an arena-cut render it never asks again. The SDK windowing now counts the
  rows it leaves unbuilt for want of arena credit (`take_arena_unbuilt_rows`, thread-local, re-exported; the capacity-refusal branch
  counts only when the arena is the cause, `ui_value_headroom().rows() == 0`), and the reactor turn notes each short surface after its
  render (`ArenaStarvedSurfaces`, fixed `DIRTY_RENDER_CAPACITY` slots) and re-dirties it next to the deferred-surface re-dirty as soon as
  `ui_value_headroom().rows()` can price the missing rows — each follow-up builds more, a surface the arena cannot help is not re-rendered.
  The law asserts `take_arena_unbuilt_rows()` > 0 for every arena-shortened window and 0 for whole ones.
