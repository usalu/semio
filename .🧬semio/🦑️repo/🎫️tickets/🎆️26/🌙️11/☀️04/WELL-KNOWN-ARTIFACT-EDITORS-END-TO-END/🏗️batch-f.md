# Batch F (T12): ifc, bcf, idf, epw, inventory

Phase 1 inspection. Root: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts`. No source edited yet.

## Per-artifact status

| Artifact | Surface | Edit capability found | Status |
| --- | --- | --- | --- |
| ifc | `Ifc4AnyEditor`, `Ifc2x3{Any,Cobie,Cv20,Sav}Editor`, 4x3 any (composition `🏗️ifc/🧩️composition/🔣️.json`, family `🏙️bim`) | `SetActiveExample` + generic `EditSnapshot` (SetValue/Insert/Remove/Move/Rename/ReplaceSource) with entity mutation laws in `🏗️ifc/🧪️tests/🏗️mutate-ifc-4` | surfaces exist; runtime result unverified (build queue) |
| bcf | `BcfAnyEditor`/`BcfAnyViewer` (`💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup`) | `SetCell` (guid, title, status, priority, author; revision guarded, stale-row/column faults) plus snapshot edits; mutations insert/remove topic, set-topic-markup, comment insert/set/remove, set-version; derived zip fixtures in `🧪️tests/🧫️fixtures` with a jszip oracle | surface exists; unverified |
| epw | `EpwEditor`/`EpwViewer` | hourly `set-cell` by column (row-revision guarded); header lines (location, design conditions, ground temps, comments, data periods) via snapshot details pane (`🧭️edit-rules`); insert/remove record; fixtures per mutation | surface exists; unverified |
| idf | none | `idf` is deliberately definition-only (`opaque` profile/dialect, `codecs: []`, composition `apps: []`; `assembly()` = `definition_only_assembly`). IDF is edited through `🔋️energy` model: `🔋️energy/🗿️artifacts/🔋️model/.../🚪️io/📥️import/⚡️idf` (decode to model) and `📤️export/⚡️idf` (encode), with a round-trip unit test | needs coordinator decision, see below |
| inventory | none | confirmed non-editor | out of scope |

## inventory (confirmed)

`📇️inventory` has no `📜️artifact-definition.json`, no composition, no standards. It holds only `🟦️.ts` plus a TypeScript package that inventories the stdio package records (`STDIO_RELATIVE_ROOT`, `canonicalStdioArtifactNames`, admission checks). It is a library. No editor was invented.

## idf: finding and decision needed

The ask was to add `#editor`/`#viewer` to `idf`. A standalone `idf` surface is a full per-subset scaffold (the txt analogue is about 256 files: snapshot schema, mutations, proto/graphql/ts/rust twins, io, viewer, oracles, family component enum). It would also contradict the artifact's own declaration ("without claiming a generic document editor"; opaque, no codec, no snapshot schema) and duplicate the energy model's IDF decoder/encoder. I did not start it. The consistent design that gives end-to-end IDF editing is:

1. Register `.idf` as an import/export format of `s.stdio.energy-model` (energy model already has `import_bytes`/`idf_from_model_json`). Opening `x.idf` then resolves to the energy model `#editor` via import; save exports IDF. Owners: T01 (registry/extension precedence), T04 (import/export routing).
2. Typed field edits (IDF object fields) are then the energy model editor's `SetStructureField` edits, with history.

Caveat: `deserialize_bytes` refuses IDF with import diagnostics, so objects the model cannot represent are refused with an explicit error rather than silently dropped (accurate, but means partial IDFs do not open).

If the coordinator still wants a standalone text-kit IDF surface, say so and name the generator/scaffold to use; I will then build it (records of class + positional fields, comments preserved, round-trip fixture cross-checked against a third-party IDF/epJSON parse).

## BCF fixture-honesty drift (checked)

- `📚️examples/🎬️demo/🖼️assets/🧪️example/💬️.bcf` is a 0-byte placeholder; nothing references it (grep). Documented earlier in the mutate-bcf feature; still unresolved.
- The demo DSL (`🗣️.dsl.semio`) carries `snapshot=[ 137 80 78 71 13 10 26 10 ]`, an 8-byte PNG signature only, not an image. Viewer snapshot preview of the example is therefore not a valid PNG.
- The real fixture `🧪️tests/🧫️fixtures/🏥️wellness-center-coordination-review.bcf` is a valid bcfzip (bcf.version, 3 topics, bcfv, a 249 KB png, project.bcfp); two topics share identical 1468-byte markup sizes (derived content).
- Planned fix: replace the 0-byte placeholder by a copy of a derived valid bcfzip (or delete it) and give the demo viewpoint a real tiny PNG; add an invalid fixture (truncated zip, missing bcf.version).

## Checks performed

- Static reading of editors, composition JSON, edit rules, oracles, fixtures, host contract.
- `repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-stdio-bcf --lib` started in the background (log `🗑️generated/f-bcf-lib.log`). At last look the shared capacity gate was still waiting (load above 21, swap ceiling, 7 jobs ahead). No test result exists yet.

## Needs

- T06: queue slots for bcf, epw, ifc (`semio-s-artifact-stdio-{bcf,epw,ifc}` and the `mutate-*` laws) and the family bim `✏️editor-catalog` and `🚢️shipped-fleet` laws.
- Coordinator: IDF decision above. T01/T04: `.idf` routing to energy model. Batch H: bcf is a zip container; its codec depends on `s.stdio.zip` and `s.stdio.xml`, no change requested yet.


## Phase 2 update (T12)

### Matrix labels
- idf: **opens in energy model editor** (definition-only artifact; no standalone surface, per coordinator decision).
- inventory: non-editor library.
- epw, bcf, ifc 4, ifc 2x3 (base subset): natural-file codec added (open/save real files).

### idf contract
`.idf` resolves to `s.energy.model@1/*#editor`. Composer `s.energy.model.composer.idf` already reads `s.stdio.idf@25.2/*`, and `import_stdio_kinds`/`export_stdio_kinds` list `stdio.idf`. New: the energy editor now declares `natural_file_codec` with `format_kind "s.stdio.idf@25.2"`, `.idf`, `encode_natural_file` = `export::idf::serialize_bytes`, `decode_natural_file` = `import::idf::deserialize_bytes`. Refusal: IDF with import diagnostics returns `MediaError::Payload("artifact:native", "IDF import requires resolving diagnostics: <message> (<subject>), ...")`, non-UTF-8 returns "IDF: invalid UTF-8", so the open path ends in the localized refusal notice with the named reason and the open document is untouched (recoverable). Files: `🔋️energy/.../✳️any/✏️editor/🖥️editor-impl/🦀️.rs`, `✏️editor/📦️packages/🦀️rust/Cargo.toml` (import/export crates promoted to runtime deps; import removed from dev-deps; no cycle, neither crate depends on the editor), tests appended to `✏️editor/🧪️tests/🔬️unit/🦀️.rs`.
Open risk: a Cargo.lock refresh of the energy workspace; `Ifc`-style generated-catalog regeneration is not needed for idf because no surface was added.

### Natural-file codecs (all hand-written per `📗️recipe`)
| Editor | format_kind | Codec source | Tests appended |
| --- | --- | --- | --- |
| epw `EpwEditor` | `s.stdio.epw@energyplus`, `.epw`, text | `decode_epw`/`encode_epw`, MOVED to snapshot crate `🌦️epw/.../🚪️io/📄️natural/🦀️.rs` (editor cannot depend on the io-registry crate, which depends on it); io re-exports them | open real fixture, comments_1/comments_2/design conditions preserved, one hourly field edit, save, reopen, only the one data line differs; truncated and non-UTF-8 refused |
| bcf `BcfAnyEditor` | `s.stdio.bcf@2.1`, `.bcf`, binary | `encode_bcf`/`decode_bcf` (same crate); decode now refuses an archive without `bcf.version` | open minimal bcfzip, SetCell title edit via `bcf_emit`, save, reopen, comments, viewpoints and PNG retained; truncated and missing-version fixtures refused |
| ifc 4 `Ifc4AnyEditor` | `s.stdio.ifc@4`, `.ifc` | `decode_ifc4_document` + `from_part21_document`, `encode_ifc4_document` + `to_part21_document` | official column fixture: ids and order equal the file text, `SetEntityArg` on IFCPROJECT name, every other entity equal after save/reopen; dangling `#99999` and IFC2X3 schema refused |
| ifc 2x3 `Ifc2x3AnyEditor` (base) | `s.stdio.ifc@2x3`, `.ifc` | `decode_ifc2x3`/`encode_ifc2x3` | real Revit export fixture: `UpsertInstance` on IFCPROJECT name, ids, reference attribute and all other instances preserved; IFC4 declaration refused |
| energy model | `s.stdio.idf@25.2` (see above) | export/import idf | round trip BESTEST 600 IDF byte-identical; diagnostics refused with name |

Lifecycle fixture `🧰️framework/.../🔌️plugin/🧪️tests/🧫️fixtures/📄️natural-file-lifecycle/🔣️.json` has rows for epw, bcf, ifc@4, ifc@2x3 and the energy model (count asserted at 48 in `📄️natural-file-lifecycle/🦀️.rs`; other workers add rows concurrently, so recount before merging).
ifc 2x3 subsets cobie, cv20, sav and the 4x3 orphan have no natural codec (their `format_kind` would equal the base one; the default `*` subset editor is the natural-file owner, same as jpg baseline).

### BCF fixture repair
- New `🧪️tests/🧫️fixtures/📂️natural-open/{minimal,truncated,missing-version}.bcf` under `💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup`, generated by `<ticket>/🏗️batch-f-make-bcf-fixtures.py` (python zipfile writer, a real 2x2 PNG). Cross-checks: python `zipfile.testzip()` is clean and lists the four parts; the Rust test reads it through our codec. Not produced by our encoder (no executable build); `🧪️tests/🔀️mutate-bcf-2-1` still holds the jszip oracle harness (bun missing from PATH, not run).
- The 0-byte `📚️examples/🎬️demo/🖼️assets/🧪️example/💬️.bcf` is replaced by a copy of `minimal.bcf`. The demo DSL's 8-byte PNG signature stays: its `🎒️.pack.semio` is derived, regenerating it needs the generator.

### Not done (honest)
- No test has finished. Queued background runs (logs in `🗑️generated`): bcf lib, `semio-s-artifact-stdio-epw-editor` + `epw-io-registry`, `bcf-io-registry natural_file`, `semio-s-artifact-stdio-ifc-4 natural_file`, `ifc-2x3-base natural_file`. Not compiled: the energy editor tests and `natural-file-lifecycle`.
- Native receipts for ifc/epw (needs `composition-prepare` and `native-codec-projection` generators through T06).
- `Ifc4x3AnyEditor` orphan (939-file subset: viewer, `builder.editor/viewer` in `🏙️bim`, playground row, fleet counts).
- ifc 4 edit window RO: the reachable edit today is the Details pane snapshot edits (`SetEntityArg` laws in the unit tests); no editable window added.
- BCF third-party jszip run and EPW third-party cross-check beyond std line comparison.

## Phase 2b update (T12)

- cobie, cv20, sav editors: natural-file codec with `format_kind` `s.stdio.ifc@2x3/<subset>` (shares base `decode_ifc2x3`/`encode_ifc2x3`), one round-trip test each, three lifecycle rows (fixture now 51 rows, count asserted in `📄️natural-file-lifecycle/🦀️.rs`). Plain `.ifc` stays with `s.stdio.ifc@2x3` and `@4`/`@4x3`; the `OpeningResolver` must pick the owner subset (T01).
- ifc 4x3: natural-file codec on `Ifc4x3AnyEditor` (`s.stdio.ifc@4x3`, `parse_ifc4x3_step`/`write_ifc4x3_step`), test on the official `wall-extruded-solid.ifc`. New viewer `🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/👁️viewer/🦀️.rs` (`Ifc4x3AnyViewer`, mesh window plus read-only inspector, mounted as `viewer` in the 4x3 crate). Registered: composition JSON viewer row (order 11) and playground row `stdio-ifc-4x3` (react 6301, wgpu 6401); hub `🏠️bim/🦀️.rs` enum and builder, hub workspace/stdio/bridge Cargo dependency rows. NOT regenerated: `🏠️bim/🧩️component/🔣️.json` manifest, `🤖️generated` roster/editor-laws, `🔌️plugins.json`/`🚀️playgrounds.json`, shipped-fleet/editor-catalog counts, hub Cargo.lock. These need `composition-prepare` via T06, then count updates (+2 apps, +1 playground).
- EPW independent cross-check: `<ticket>/🏗️batch-f-epw-csv-oracle.py` (Python stdlib `csv`) wrote `🌦️epw/.../✏️editor/🧪️tests/🧫️fixtures/📖️natural-open/🔣️.json`; the Rust test `natural_file_matches_the_independent_csv_oracle` compares the 35 columns of every record and the 8 header lines.
- Cobie/sav/cv20 domain kinds NOT wired (finding): these editors declare `type Mutation = Ifc2x3Mutation` (base vocabulary), while `set-facility-name`, `set-floor-elevation`, `set-space`, `set-type-assignment`, `set-view-definition` belong to `Ifc2x3CobieMutation` (and cv20/sav enums). A Details edit rule can only emit the editor's own mutation type, so emitting the domain kinds means changing each subset's editor, viewer, history, sqlite-snapshot law and composition `mutationDescriptors` to the domain enum, not adding a rule row. Until then the same edits (building/storey/space name, elevation) are reachable as generic `/document/instances/<n>/entities/0/1/<arg>` Details edits mapped to `upsert-instance`. Needs a decision before it is attempted.
- Build queue: one invocation chain (bcf, ifc 4/2x3 base/cobie/cv20/sav/4x3 natural_file, epw) logs to `🗑️generated/f-batch.log` (EXIT-bcf, EXIT-ifc, EXIT-epw markers).

## Domain-enum switch for ifc 2x3 cobie/cv20/sav: design and why it is not landed yet

Decision received: switch each subset's editor, viewer, history, sqlite-snapshot law and composition descriptors to the domain enum, adding the base per-instance operations schema-first.

Current coupling (cobie, same shape for cv20/sav): `✏️editor/🦀️.rs:173` and `👁️viewer/🦀️.rs:63` use `type Mutation = Ifc2x3Mutation` (base enum re-exported by `🧬️schema/🧬️mutations/🦀️.rs:39`); `🚪️io/🦀️.rs:108-175` (analyzer/composer) applies `Ifc2x3Mutation::UpsertInstance`; the editor's Details path is the base `inspector::edit_special` returning `Vec<Ifc2x3Mutation>`; composition `mutationDescriptors` and `🤖️generated/🧪️editor-laws` pin the base enum with `Ifc2x3Snapshot`.

Plan, in dependency order:
1. Schema: add `UpsertInstance`, `RemoveInstance`, `SetHeader` as variants of `Ifc2x3CobieMutation` (and the cv20/sav enums) in each `🧬️schema/🧬️mutations/🔣️.json`, regenerate the rs/ts/proto/graphql twins with the schema generator (T06). The base leaf payload structs implement `MutationKind<Ifc2x3Snapshot, Ifc2x3Mutation>`; the domain enum needs the same impls for its own enum parameter (allowed by the orphan rules, the enum is local). Shared logic stays in the base crate as free functions on `Ifc2x3Diff` (no adapter layer): the three leaves call one `diff`/`inverse` function with the enum as a parameter.
2. Editor and viewer: `type Mutation = Ifc2x3CobieMutation`; `inspector::edit_special` becomes generic over the target enum with a `From`-free constructor trait implemented by both enums (or each subset passes its three variant constructors).
3. Domain Details rules in `special`: `/document/instances/<n>/entities/0/1/2` on `IFCBUILDING` to `set-facility-name`; same arg on `IFCSPACE` to `set-space` name; `IFCBUILDINGSTOREY` arg 9 to `set-floor-elevation`; `IFCRELDEFINESBYTYPE` to `set-type-assignment`; `FILE_DESCRIPTION` implementation view to `set-view-definition`; all other pointers fall through to the base per-instance variants.
4. io, laws, sqlite snapshot law and composition `mutationDescriptors` to the domain enum; `composition-prepare` regenerates the rosters.

Not landed: the enum change touches compiled crates in three workspaces plus generated twins in the generator-owned set, and nothing in this batch has compiled yet (queue). A blind change risks breaking cobie, cv20, sav, the hub and the editor-laws for every worker. Proposal: land step 1 and 2 for cobie first after the current chained run reports (so the baseline is known), use it as the template for cv20 and sav, one chained invocation per step.

## Cobie schema step (done) and rule table (ready to paste)

Done (source JSON only): `🏢️cobie/🧬️schema/🧬️mutations/🔣️.json` union `Ifc2x3CobieMutation` now has `UpsertInstance`, `RemoveInstance`, `SetHeader` entries (externally tagged like the five domain entries), each `$ref`-ing the BASE leaf schema (`…/ifc/2x3/base/mutation/{upsert-instance,remove-instance,set-header}/schema.json`, referenced, never restated). cv20 and sav get the same three entries when their turn comes.

Generator targets: none. The cobie mutations folder has no rs/ts/proto/graphql twins (only the union `🔣️.json`, five leaf `🔣️.json` + `🧬️schema/🔣️.json`); the Rust enum `Ifc2x3CobieMutation` and its `KINDS`/`kind()` tables are hand-written, and the only consumer is the `ifc-2x3-cobie` catalog check in `🔣️oracle.json` (`KINDS` verbatim) plus the `dsl::Mutations` derive. The step-2 Rust change must therefore add the three variants, extend `KINDS` and `kind()`, and update `🔣️oracle.json` in one commit. The only generated artefacts downstream are the hub rosters and editor-laws (`composition-prepare` via T06, one batch). Caveat: until step 2 the union JSON lists three kinds the Rust enum lacks, so any schema-vs-enum parity test on cobie fails in between; run step 2 straight after the baseline.

Pointer → kind table (Details pointers per the base tests, `entities/0/arguments/<i>/value`; `<n>` is the instance position; match by the instance's first entity type name; `id` is the instance id):

| Subset | Pointer | Entity guard | Emits | Payload |
| --- | --- | --- | --- | --- |
| cobie | `/document/instances/<n>/entities/0/arguments/2/value` | `IFCBUILDING` | `SetFacilityName` | `{building: id, name: Some(text)}`, `None` clears |
| cobie | `/document/instances/<n>/entities/0/arguments/2/value` | `IFCSPACE` | `SetSpace` | row with `name` from the edit, `global_id` and `placement` read from the instance |
| cobie | `/document/instances/<n>/entities/0/arguments/9/value` | `IFCBUILDINGSTOREY` | `SetFloorElevation` | `{storey: id, elevation: decimal}` |
| cobie | `/document/instances/<n>/entities/0/arguments/{4,5}` | `IFCRELDEFINESBYTYPE` | `SetTypeAssignment` | assignment row from the instance with the edited related objects or relating type |
| cobie, cv20, sav | `/document/header/fileDescription/...` implementation level / description text | none | `SetViewDefinition` | the view name string |
| cv20 | `/document/instances/<n>/entities/0/arguments/5/reference` | `IFCPRODUCT` subtypes | `SetProductPlacement` | `{product: id, placement: ref}` |
| cv20 | units entity arguments | `IFCUNITASSIGNMENT` | `SetProjectUnits` | units payload |
| sav | analysis-model, load-group and group-assignment entities | `IFCSTRUCTURALANALYSISMODEL`, `IFCSTRUCTURALLOADGROUP`, `IFCGROUP`/`IFCRELASSIGNSTOGROUP` | `SetAnalysisModel`, `SetLoadGroup`, `SetGroupAssignment` | per leaf payload |
| all | every other `/document/instances/**` pointer | none | `UpsertInstance` (rename: `UpsertInstance` + `RemoveInstance` via base `renamed_instance`), header edits `SetHeader`, removals `RemoveInstance` | base payloads unchanged |

Rule: `special` first tries the guarded domain rows (guard failure falls through, never errors), then the base `special`. Test each row test-first on the existing wellness-center fixture (building/space/storey names present in the export) and assert undo restores the exact instance.

## Transcribed evidence (T20, before the `🗑️generated` deletion)

Full text in `✅️validation.md` E6.
- `f-batch.log`: capacity-wait lines only (bcf, ifc 4, 2x3 base, cobie, cv20, sav, 4x3, epw). No result, no error.
- `f-bcf-lib.log`, `f-bcf-natural.log`, `f-epw.log`, `f-ifc4-natural.log`, `f-ifc2x3-natural.log`: waiting or Cargo.lock refresh; no result.
- Contract (offline) for idf and ifc, bcf, epw: passing in E1 (`✅️validation.md`; idf passes per Update 3).

## Review fixes (T12)

1. 4x3 playground ports moved into the IFC band: react 6222, wgpu 6322 (both unused in every stdio composition JSON; the earlier 6301/6401 broke the 62xx/63xx convention).
2. The 4x3 editor row now carries `laws: { editing: "ifc4x3_any_editor", sqliteSnapshot: "sqlite_snapshot_ifc4x3_any_editor" }`. It persists through the same `IfcSnapshot` sqlite snapshot as ifc 4, so the generic `editor_catalog_laws!` row applies unchanged; T06's `composition-prepare` adds the tuple to `🌎️hub/🧩️compositions/🗄️stdio/🤖️generated/🧪️editor-laws/🦀️.rs`. The viewer row keeps `laws: null` like every other viewer. The 4x3 `factory` keys are flat (`…::editor::create_ifc4x3_any_editor`, `…::viewer::create_ifc4x3_any_viewer`) because the 4x3 crate mounts its editor and viewer as single files, not per-subset nested modules; both rows share that shape.
3. Typed bilingual refusals (code plus English and German, English first):
   - IDF: `IdfImportError` in `🔋️energy/…/🚪️io/📥️import/⚡️idf/🦀️.rs` (`NotUtf8`, `Malformed`, `Diagnostics`, `Unrepresentable`; `code()`, `message_en()`, `message_de()`, Display `[code] en / de`). `deserialize_bytes` returns it; the txt carrier uses `message_en()`, the composer uses `to_string()`. The diagnostics stay verbatim inside the localized frame. The editor puts the Display text into `MediaError::Payload`.
   - EPW: `EpwDecodeError` in `🌦️epw/…/🚪️io/📄️natural/🦀️.rs` (header-lines, record, no-records, malformed, not-utf8) with `From<EpwDecodeError> for String` so `Error::fatal` callers still compile; the mutate-epw test converts with `String::from`.
   - Limit: `MediaError::Payload(port, String)` is the framework carrier, so the shell shows `[code] en / de` as one text. A locale-selecting render needs `MediaError` to carry a typed localized payload (framework change, owner T02/T03); not done here.
4. Cobie Rust step: pending the build baseline. It will rewrite the module doc at `🧬️mutations/🦀️.rs:4-8` (kinds are the domain vocabulary plus the three generic per-instance variants), add the three variants, extend `KINDS`/`kind()`, update `🔣️oracle.json`, and add a test asserting that the union JSON's `oneOf` property names, the Rust `KINDS`, and the oracle catalog are equal.
5. IDF codec media type: the energy editor declares `media_type: "text/plain"` while the idf definition has `mimes: []`. This is intentional: `.idf` has no registered MIME, resolution is extension-first (`.idf` to `s.stdio.idf@25.2` to the energy model editor, T01), and the `NaturalFileCodec.media_type` is only the transfer label. The energy `AGENTS.md` "no IDF" line concerns the engine API; IDF interop already lived in the model artifact's io.
