# Jack Native Parent Snapshot SQLite Readiness

Jack owns eight persisted fields: schema, name, optional manifest_id, inline Manifest, Camera, literal content ArtifactChild, optional root_node_id, and query. Its graph instance lives in a separately addressed SemioGraphSnapshot child. The local JackWorkingScene attached to that handle is runtime materialization, not a ninth persisted parent field. Parent SQL preserves the exact handle and never re-mints coordinates from that local scene.

## Individually Authored Semantic Contract

The shared snapshot SQL has thirteen explicitly authored domain tables:

| Table | Width | Native domain |
| --- | ---: | --- |
| jack_document | 6 | schema/name/nullable manifest_id/root_node_id/query |
| jack_camera | 11 | x/y/zoom query REAL + signed word + class |
| jack_content_child | 7 | five independent literal handle fields |
| jack_node_kind | 4 | ordered node-kind names |
| jack_edge_kind | 4 | ordered edge-kind names |
| jack_port_kind | 5 | ordered names and In/Out direction |
| jack_node_kind_port | 4 | ordered literal port-kind references |
| jack_value_type | 2 | Boolean/Integer/Decimal/Text/List/Schema/Any tag |
| jack_value_type_list | 3 | a List's one exact child type |
| jack_value_type_schema | 3 | a Schema's exact literal reference |
| jack_node_property | 7 | ordered name/Data-or-Derived/optional expr/type root |
| jack_edge_property | 7 | same actual property type under an edge kind |
| jack_port_property | 7 | same actual property type under a port kind |

The fixed root/parent fields, map-free ordered collections, variant relations and ordinal semantics are domain-authored. Structural surrogate keys/FKs cannot constrain duplicate/empty literal kind/property names, unresolved schema/port strings, or child alias/target equality. A reconstructed Manifest must not call the explicit resolve_manifest registry helper: that overwrites the persisted Manifest when manifest_id is present, and refuses arbitrary registry IDs. Null versus empty optional strings remains distinct.

Camera fields use the existing first-party IEEE scalar companion convention, not scalar blobs: readable REAL (NULL for NaN), signed INTEGER exact word, and numeric class. Nine neutral words cover both zeros, subnormal, ordinary finite, maximum finite, both infinities, quiet and signaling NaN. Exact Native f64 remains the model authority; Source uses the actual first-party Binary64.

The neutral fixture contains a full literal nativeCase, not a schema-derived DTO. It repeats legal names and port references, retains all seven ValueType variants plus a nested List<List<Schema>>, both property kinds and directions, expr absent/empty/nonempty, arbitrary independent manifest_id, and literal empty/reserved/NUL/Unicode child addresses. Its explicit full-case row counts total 153 rows across the thirteen tables. Source executor owns its runtime/Bun/Ajv and publication proof using this shared DDL and fixture.

## Authentic Native Baselines — Not Executed

Nine baseline laws are mounted at the actual Snapshot owner, and a tenth is mounted inside the actual subset declaration scope so it calls the real private io_declaration rather than a guessed external API. The registered Native selector explicitly enables the existing component-app-assembly feature because this declaration is gated by that actual owner.

The ten laws cover bare capability; complete inline Manifest/literal child preservation in ordinary Binary and Text; every exact camera word with complete parent equality; separately attached and unresolved child handles; genuine controlled typed Value construction/output; real declared JSON leaf camera closed-word output/reconstruction; independent Bun SQL width/word/FK interpretation; actual erased Binary/Text projection/reconstruction; and the actual subset Native declaration's optional capability.

Every successfully constructed Native fixture is wrapped in a scope that drives the existing JackSnapshotRetirementFactory to its terminal-empty state on success and assertions/refusals. This respects the actual owner lifecycle and does not invent a generic ordinary-drop fallback. Rustfmt parsed/formatted the two new Native test files. No Cargo invocation was launched by this executor; parsing is not compiler/runtime evidence.

Current inspected ordinary Native gap: JackPackRecord has no inline Manifest or literal content coordinates. It reads local scene nodes/edges, then decode re-mints a handle and calls resolve_manifest. The existing declared JSON serializer uses numeric Camera values. These are staged assertions, not yet executed feature REDs. Rust SQL projection/reconstruction, controlled Jack native methods and capability hooks remain unmounted pending Root's sole Native lane.

## Owned Routes and Source Coordination

Exact owning project: @semio-tech/trinity-jack-rs. The existing Rust script preserves graph-generate, graph-wire-check and verify, and adds snapshotSqliteTests pointing to the Source executor's physically present suite. Combined/native/Source routes are registered through that script, Nx and both launch catalogs at 408.724–726. Explicit Native feature: component-app-assembly. Source-public and strict consumer routes belong to the Source executor (408.727+).

Next Root-owned Native command:

`SEMIO_TEST_LEVEL=quick bun nx run @semio-tech/trinity-jack-rs:test-snapshot-sqlite-native --skip-nx-cache`

The existing owner runner includes no-fail-fast and a strict selected-test admission contract. Root owns the sole Cargo lane; this worker has run no Native verification. No universal-complete, Native-control, native-json, declaration or erased-I/O claim is made.

## File Ownership

Under `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack`:

- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🗄️.sql` — complete thirteen-table contract.
- `…/📸️snapshot/🧫️fixtures/🪶️sqlite/🔣️.json` — explicit neutral case/types/words/table widths and row counts.
- `…/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs` — nine genuine Native baselines.
- `…/📸️snapshot/🧪️tests/🪶️sqlite/🚪️io/🦀️.rs` — actual private subset declaration law.
- `…/📸️snapshot/🦀️.rs` and `…/🪆️subsets/✳️any/🦀️.rs` — only test mounts, production behavior unchanged.
- `📦️packages/🦀️rust/{📜️script.ts,📋️project.json}` and root `.vscode/{launch.json,🧩️launch.seed.jsonc}` — narrow owned registrations.

Source models/provider/JSON facets/public assets belong to the parallel Source executor and are not reported as Native work here. Process3d sixteen Source laws and its consumer green are reported separately. Block22 and Puzzle3d six Native providers remain unmounted pending authentic assertion baselines.

## First actual Native baseline and corpus portability repair

The coordinator reached ten selected Native laws: all ten failed in 19ms, Nextest `8376adc3-1d08-4af3-b2d5-e996255a3fb8`. Exactly three independently reached genuine missing-capability assertions: bare parent, actual subset declaration, and erased Binary/Text capability. Seven fixture-dependent laws instead failed during serde_json input admission: Source-added `invalidUnicode` unpaired UTF-16 escape strings were not portable to the scalar-valid Native JSON parser. Those seven failures establish no ordinary Native field/codec fidelity RED.

The negative vectors now persist their exact code units as `invalidUtf16CodeUnits: [[55296], [57343]]`, and only the Source negative loop reconstructs them with `String.fromCharCode`. The exact invalid JS strings and Buffer replacement oracle are preserved; every other fixture property is unchanged. This is a neutral corpus portability repair, not a production semantic correction. The Source owner explicitly confirmed no concurrent fixture/test edit. Fresh registered Source validation and a valid-corpus Native baseline are required before further claims. Ordinary Pack/Text/manifest/camera Native fixes remain unmounted until that fresh baseline executes. Capability/projection opt-in is authorized by the three genuine assertions but has not yet been mounted.

## Portable Neutral Unicode Corpus Verification

Replaced the two negative lone-surrogate JSON strings with explicit UTF-16 code-unit arrays `[55296]` and `[57343]`; the Source negative loop constructs those exact units with `String.fromCharCode`. No nativeCase literal changed. Independent Python JSON parsing plus strict UTF-8 encoding validated all 661 decoded string values/keys without surrogates. The registered uncached Jack Source target passed all 38 laws and 248 assertions (1.301 s assertions, 16.2 s Nx). Native serde corpus admission remains to be verified by the main native lane. Its preceding result remains three genuine capability refusals and seven fixture-harness failures; this repair establishes no ordinary Native fidelity result.

## Fresh Portable Native Corpus Classification

After the malformed surrogate fixture was replaced with explicit UTF-16 units, the valid corpus genuinely executed ten laws: one passed/nine failed, Nextest `8c706ec0-b54a-41c7-b41a-041360071c96`, `root-parent-valid-corpus-native-baselines.log` lines 6376–6584. All nine now reach actual owner assertions: three absent capabilities, one declared JSON missing closed camera bits, four Binary/Text/camera/child laws rejecting the independently persisted inline manifest ID, and one strict missing controlled ToValue implementation. The independent SQLite oracle passed. No Native production repair or post-repair result is claimed at this point.

## Mounted Native repair, runtime pending

Following the valid ten-law baseline (one pass/nine owner failures), thirteen authored SQLite tables now have an explicit Rust projection/reconstruction. Scalar camera columns retain numeric queries, signed exact words and numeric classes. Ordered kinds, property definitions, node ports, independently owned ValueType chains and literal Child coordinates have named entities. Names may repeat or be empty; a manifest identifier remains independent of the retained inline manifest. Reconstruction rejects dangling/cyclic/multiply-owned/orphan entities and malformed ordinals. Partial type and manifest retirement is iterative; completed snapshots use the actual Jack retirement factory.

The previous native scene/registry carrier is replaced with literal Jack native records: schema/name/optional identifiers/query, camera, all three typed manifest collections with explicit property type paths, and all five Child address fields. Ordinary native decode no longer resolves a registry or remints a handle. Genuine controlled copies and physical record bindings are mounted for erased Binary/Text. Declared JSON camera output/input and the fixture observability helper now use the closed exact word boundary. Rustfmt parsing succeeds; no fresh Native test result is claimed.

The direct erased thunk returns thirteen domain tables. The prior test expected fourteen before its missing capability refusal; this unexecuted table-count expectation is corrected separately as a harness correction. Central metadata is attached by the public I/O boundary.

Canonical controlled typed Value remains unfinished: direct ValueType input/output laws are staged, with production unchanged until Root measures their primitive failure. Jack’s demo native asset and its separate scene-oriented to_json/from_json consumer surface still need coherent owner/child asset alignment before whole-package success.

The selected Jack Native suite is now thirteen laws, retaining all original ten. New laws inspect all thirteen tables and all 153 neutral fixture rows using independent Bun SQLite, edit manifest/query/independent Child aliases and reconstruct all camera words, apply all fifteen authored hostile SQL edits, and measure exact row limits plus interior Project/Reconstruct/Decode/Encode Unicode-copy cancellation. These new laws are mounted but unexecuted.

Jack Snapshot’s typed controlled Value methods are now explicitly field-by-field. Output preserves the original omitted optional IDs; input preserves existing admitted defaults and requires content/query. Manifest and Child intermediates and completed snapshots are guarded across checkpoints. Output never calls ordinary ToValue, input never calls ordinary FromValue, and completed snapshot retirement uses the actual Jack cursor. The Graph PropertyDef custom input conversion remains unmounted pending its direct owning refusal; that is a genuine remaining control dependency.

The authored native records flatten only the domain’s singly nested ValueType chain into explicit kind tokens and optional schema text. They carry typed kind/property/port records and literal Child address fields, retain empty/duplicate names and independent manifest references, and introduce no whole native/JSON/Value payload field. The canonical ValueType owner now provides the chain retirement primitive, avoiding a second recursive lifecycle implementation. Entity lookup and relationship scans also expose interior 256-unit cancellation.

## Canonical Typed Construction Dependency Mounted

Canonical ValueType's four controlled laws now executed in the owning registered 43-law selection (43/43, Nextest `6e160516-35ef-4998-8eb3-c0e48efa891c`). Graph PropertyDef's direct custom-field law subsequently reached a genuine missing controlled-retirement refusal (`40c1c65c-4da6-4d8c-a4e0-16384f7f0277`). Its explicit borrowed custom constructor and canonical iterative retirement are now mounted, preserving optional expression presence and admitted copies. Jack's own thirteen-law post-repair run remains pending; these lower owner results do not establish Jack Native success.

The explicit flat-record pre-materialization row scan now uses the same cumulative NativeDecodeControl, checks all row-count arithmetic, and publishes interior checkpoints while visiting kinds/properties. It no longer contains an uncheckpointed full manifest scan. The borrowed typed forecast and final schema/projection checks retain their separate authority. Rustfmt parsing passed; this change is runtime-pending with the selected thirteen-law suite.

## Public Snapshot JSON Boundary Staged Separately

The selected suite now has fourteen laws. A new direct `JackSnapshot::to_json/from_json` law consumes the same neutral complete parent/IEEE states and uses independent serde to inspect the literal content address and closed camera words. It refuses any ephemeral nodes/edges payload and checks complete owned equality after reconstruction. This public API still emits the old scene-oriented shape and remints/resolves upon import; its production is intentionally unchanged until the direct owning law executes. The already repaired declared JSON import/export leaves are a distinct boundary and do not prove this public helper.

The existing demo asset still names scene `nodes`/`edges`, while the canonical literal native record contains the inline typed manifest and five-field child address. The existing demo/codec fixpoint and shell/editor materialization consumers need an actual whole-owner run and hand-authored child closure asset before whole-package completion can be claimed. No ordinary JSON/native carrier or legacy parser branch has been added to the corrected record codec.

Graph's exact controlled property dependency is now genuinely GREEN after its own measured stage-accounting repair: all three laws passed, Nextest `2f5d1726-627b-4b9a-a210-55bd95c1fb29`, 12 ms, 185 outside. This is lower-owner verification only; Jack's fourteen laws are queued for the sole Root native lane, and its direct public JSON method remains deliberately unmodified until the new literal-parent law executes.
