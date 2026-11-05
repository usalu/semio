# Advertised Puzzle IO Route Coverage

Read-only current-source audit. No native execution or source change performed.

The public `import_stdio_kinds`, `export_stdio_kinds`, and composer descriptors in each artifact `🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs` advertise these actual runtime routes:

| Artifact | Import | Export | Additional acceptance beyond Pack/DSL/JSON/SQLite |
|---|---|---|---|
| 2d | JSON, TXT | DWG, DXF, JSON, PDF, PNG, SVG, TXT | Actual SVG/PDF/PNG/DXF/DWG export execution and readable output |
| 3d | JSON, TXT | JSON, TXT | No further format advertised in this registry |
| 5d | JSON, TXT, ZIP | JSON, PNG, TXT, ZIP | Actual ZIP export/import, independent archive witness, PNG output |

Canonical artifact prefix is `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/{◻️2d,🧊️3d,🖐️5d}`. These are real serializers, not advertised stubs: 2d drawing leaves call the common first-party Semio drawing encoder; 5d ZIP leaf calls the document archive codec, and PNG calls drawing serialization. Output-only drawing formats do not imply a lossless inverse or an unadvertised import requirement.

## Actual Existing Test Reachability

Correction after full canonical-root inspection: the generic format laws ARE mounted. The previous audit inspected the root tail and searched a prefixed path, missing relative test mounts in the middle of the roots. 2d canonical IO lines204–206 cfg(test)-mount its relative unit file; 5d lines192–194 do likewise. Do not add duplicate mounts.

True chain2d: `semio-s-artifact-puzzle-2d-text-inverse` Cargo lib `🦀️.rs` → `🪞️mirror/🦀️.rs` lines228–230 → canonical IO component → relative tests. True chain5d: `semio-s-artifact-puzzle-5d-tests` Cargo lib → mirror lines37–39 → canonical IO component → relative tests. Dependency compilation does not enable these owners own cfg(test).

Consequently planned unfiltered5d tests-owner --lib includes the existing three format laws. The2d aggregate mutation owner and artifact-root physical prefix do not execute the text-inverse owners tests: add/select its existing --lib tests for format acceptance rather than inventing a new owner. A `--lib standards::v1::subsets::any::io::component::tests::` filter at either true owner selects the three existing format laws; nonzero actual execution still required.

2d existing laws: `svg_draws_the_visible_board` checks real SVG visible paths/text; `page_formats_are_real_files` checks PDF header, decoded PNG dimensions and DXF circle count; `txt_is_the_exact_dsl_carrier` roundtrips actual TXT leaves. There is no DWG assertion in this authored file. Its fixture construction still uses typed pack-json decoding, so mounting may expose current API drift; no compile result is inferred.

5d existing laws: `zip_is_a_real_archive_of_the_exact_document` opens actual output with third-party test-only `zip::ZipArchive`, verifies exact DSL member and snapshot.json presence, then actual ZIP import equality; `png_draws_the_board` decodes actual PNG; `txt_is_the_exact_dsl_carrier` roundtrips actual TXT leaves. These are source evidence, not runtime evidence.

Existing actual IO composition mirrors to inspect for a coherent test mount (not a claimed new executable selector): 2d `🧬️schema/🧬️mutations/🔤️text/↩️inverse/📦️packages/🦀️rust/🪞️mirror/🦀️.rs`, package `semio-s-artifact-puzzle-2d-text-inverse`; 5d `🧬️schema/🧬️mutations/🧪️tests/🔬️unit/📦️packages/🦀️rust/🪞️mirror/🦀️.rs`, package `semio-s-artifact-puzzle-5d-tests`. Proper owner choice must preserve the current split source and custody boundaries.

## Required Evidence Still Missing

Keep these advertised output/archive routes in final completion acceptance. Use the existing mounted format laws at their actual IO owner, qualify their current API/custody, execute nonzero intended laws, and add a real DWG output witness if no other mounted Puzzle-specific law supplies it. Existing JS/core54 checks, serializer source presence, shared generic codec tests, physical SQLite laws and app boot do not prove these Puzzle format routes. No requirement is added for other artifacts or unsupported imports.

Bounded root log observation: sqlite2d-3.log showed ongoing compilation through 1140 seconds and late infinite/world/UI dependencies, warnings only; no error or intended test count observed. Parent14996 remains the authoritative process owner.

## DWG Readable Output Witness Preflight

The actual Puzzle2d DWG leaf calls `encode_drawing(puzzle2d_board_drawing(snapshot), SemioDrawingFormat::Dwg)`. Common drawing export `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/📤️export/🦀️.rs:108` serializes SemioDrawingToDwg, materializes native drawing, then calls actual DWG AC1024 writer. No Puzzle inverse is required to validate output.

Existing first-party independent read path is `semio_s_artifact_stdio_dwg_read::standards::v_ac1024::subsets::any::io::import::dwg_from_bytes(&bytes) -> Result<DwgDrawing,String>`; its entity geometry supports exact visible circle center/radius, path vertices, text content and layers. A useful witness must check known visible board geometry/text and exclude the hidden node, rather than only successful parse/header.

An existing third-party test interface is already demonstrated in DWG canonical export `🧪️tests/🔮️acadrust-oracle/🦀️.rs`: `acadrust::DwgReader::from_stream(std::io::Cursor::new(bytes.to_vec())).read()` yields CadDocument; `entities()` gives EntityType::{Circle,LwPolyline,Text,...} with actual geometry. The existing oracle independently compares kind/layer/color/coordinates and asserts document.version. Reuse this test-only reader behind a local owned witness helper, with no exported external API or runtime dependency.

Concrete declaration discrepancy: Puzzle composer advertises DWG standard ac1018 (IO root139), and leaf taxonomy is ac1018, while the actual shared writer used here produces AC1024 (its existing independent oracle asserts DxfVersion::AC1024). A new witness must not label output AC1018 merely from the Puzzle route name. Birth/root informed; choose a coherent advertised standard or genuine version-specific writer, preserving the DWG export route. No runtime test executed by this audit.

## AC1024 Target Declaration Reachability

Current Stdio DWG `📜️artifact-definition.json` explicitly declares standard ac1024, its AC1024-sniffed owner representation and composer runtime capability `dialect:s.stdio.dwg@ac1024/*`. Actual DWG root declaration binds ac1024 document codec (line63), and combines ac1024 composer entries (line74) with the other existing standard. Canonical ac1024 IO root derives a composer whose WRITES dialect is exactly `s.stdio.dwg@ac1024/*`. Thus aligning the Puzzle output descriptor/leaf with its actual AC1024 bytes has a genuine existing receiving target; no Stdio compatibility alias, restored export or declaration addition is needed. This is static registration evidence; actual producer/runtime gates remain pending.

## DWG Witness Preparation

The actual Puzzle leaf and composer advertise AC1018, while the shared drawing encoder writes real AC1024. The existing Stdio AC1024 target is registered in its schema-owned definition, composer and document codec, so the Puzzle route can align directly with the real writer without an alias or upstream change. Exactly five Puzzle source occurrences and the leaf directory require a coordinated handwritten correction. A Puzzle-specific law will decode actual output with the existing first-party DWG reader and test-only acadrust 0.5.5 oracle, asserting geometry and labels rather than only the file marker. Existing SVG/PDF/PNG/DXF/TXT laws stay mounted.

Before edits, the exact text-inverse owner Source reader reports ordinary 119850 bytes / 11 files / 1750 active lines and test 204889 / 22 / 2548, with identical physical-probe totals. These fit unchanged controls. Temporary inventory is generated/puzzle2d-io-before-extents.json; final measurement and native execution are pending. No native compiler was requested while root14996 holds the lane.

## Staged Puzzle DWG Route and Independent Witness

The Puzzle DWG leaf directory is now ac1024, with the standard composer declaration, actual IO dialect/call and text-inverse mirror module/path aligned. The old ac1018 leaf path is removed, without an alias. The registered Stdio AC1024 composer/document codec already exists; no Stdio owner was changed.

The existing mounted IO law now has four tests, preserving the original SVG, PDF/PNG/DXF and TXT assertions and adding dwg_draws_the_visible_board_for_both_independent_readers. Its new closed portable JSON Schema and case corpus retain the original real board input, including a visible circle, visible rectangle, link, goal region, labels and hidden ghost. Expected circle center [72,72,0], radius 20, root label [60,76,0] and goal label [140,48,0] are authored from the documented diagram translation and text positioning, not copied from serialized output. Both actual readers compare complete circle/text witness arrays to that same independent case. Thus the hidden ghost must add no circle and both labels must survive; no guessed total path/entity expansion is asserted.

The real leaf output is decoded by the first-party dwg_from_bytes reader and independently by acadrust::DwgReader. The latter must report actual AC1024. acadrust 0.5.5 with default features disabled is reused as a test-only dependency; first-party dwg-read and dwg-geometry are added as direct dev dependencies of the existing text-inverse owner. No external type escapes a test-local interface or enters production dependencies.

Fixture snapshots are now constructed once from the same authored neutral input and held by a test-local guard. Both original boards and the TXT-restored board retire through the existing schema testkit admit_source/close_source protocol, including the real allocation/progress/release receipt checks. The existing owner test root now installs the genuine Trace HeapWitness allocator that protocol observes. No law, receipt check or old output assertion was removed; the old redundant default/extra snapshot construction is replaced by direct construction of the same neutral snapshot.

Before native execution, the schema/corpus passed existing third-party Ajv strict validation through the Repo Bun tool interface. This is a portable input validation result only. Actual DWG writer/decoder/oracle execution remains pending.

Final exact Source-reader census: ordinary 119970 bytes / 11 inputs / 1752 active lines; test 208897 / 23 / 2605. Every physical probe is included. The additional captured input is the real neutral case JSON; its schema is used by the separate portable Ajv validation. The unchanged Source caps are preserved. Full temporary inventory: generated/puzzle2d-io-final-extents.json.

Exact manifest:
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/↩️inverse/📦️packages/🦀️rust/Cargo.toml

Exact actual native package/selection:
-p semio-s-artifact-puzzle-2d-text-inverse --lib standards::v1::subsets::any::io::component::tests::

That selector must execute all four IO laws, not only the new DWG case. Source review, final same-reader census and portable schema validation are complete; native compilation, real output readback and all four-law runtime outcome are still required. Integration currently owns Value request12092; no compiler was launched by this lane.

## Staged DWG Witness Independent Review

Current source has four existing-owner IO tests, with all former SVG/PDF/PNG/DXF/TXT assertions retained. New closed schema/case supplies original board and exact authored visible circle/text arrays; both firstpartyDWG and independentacadrust inspect actual serialized bytes, enforce full extracted circle/text lists and actualAC1024. BoardOwner explicitly admits/closes genuine snapshot through existingrecords-testkit; actualtext-inverse mirror exposes testkit undercfgtest, its devdependency enables feature, and realcfgtestHeapWitness is installed. Externalacadrust is dev-only and no external type leaves test helpers. Source-only review; nativefour-law execution required. Geometry source independently explains circle offset72,72 from framebounds−40 plus32padding, radius20; label positions follow existing6unitadvance and+4baseline, matching root60,76 andgoal140,48. No concrete staged source defect identified in bounded review.

The final DWG law also inspects the actual registered Puzzle composer entries: exactly one DWG output row must exist and its real StandardId must be ac1024 before serialization/readback. This adds no test or runtime owner. The final actual reader measurement after these three assertions is ordinary 119970 / 11 / 1752 and test 209206 / 23 / 2608; physical probes remain identical. The same four-law selector applies. A scoped path/whitespace check found no remaining ac1018 reference anywhere in Puzzle sources, no over-256-character touched path (maximum166), and no trailing whitespace or extra EOF blank lines in the new case/schema/law and relocated leaf files. Independent source review is green; actual native proof remains pending.

Final staged DWGlaw also inspects its actual io_registry entries, requires exactlyoneDWG output row and standardac1024 before serializing. This closes the route-metadata/bytes mismatch at the owning law without duplicate tests. Source-reviewed only; nativepending. Birth same-reader currentextent209206/23/2608 test,119970/11/1752 production.

## Material PDF and SVG Parsing

The same existing page_formats_are_real_files law now passes actual Puzzle PDF output through a private test adapter backed by the existing Stdio reference library lopdf 0.44: Document::load_mem, real page-tree enumeration and actual page text extraction. The portable case requires one real page and both visible labels goal/root. The original PDF 1.4 marker assertion remains in addition to parsing. Original decoded-PNG dimensions and DXF circle assertions are unchanged.

The same existing SVG law retains its original path/string assertions and additionally passes the real output through a private quick-xml 0.42 test adapter, the version already used by Stdio’s SVG oracle. It validates a balanced, single SVG root, counts actual parsed path elements and compares the parsed visible text array against the same authored goal/root labels. The portable case requires four paths. This parses the material output rather than matching markup substrings alone.

Both parser libraries are added only to the existing text-inverse owner’s dev dependencies, behind private test-local interfaces returning ordinary owned primitives. No production dependency, exported external type, new owner, additional test or format advertisement is introduced. The closed portable case schema was extended first with page/path counts and visible labels, and the complete case again passed strict third-party Ajv validation through Repo’s Bun tool interface.

Final actual Source-reader census after these assertions: ordinary 119970 bytes / 11 inputs / 1752 active lines; test 211603 / 23 / 2662. Physical probe paths and bytes exactly match admitted inputs. All controls remain unchanged, and the same four-law native selector still applies.

These are staged assertions and a portable schema-validation result, not a PDF/XML native runtime pass. Integration request12092 still owns the serialized lane; actual material parsing, both DWG readbacks and all four IO-law outcomes remain required.

## Material SVG/PDF Witness Review

Same four-law owner now independently parses SVG with existingquickxml0.42 and PDF with existinglopdf0.44. SVG requires one balancedsvgroot,4parsedpaths,exactparsedASCIIroot/goallabels. PDF load_mem/get_pages/extract_text requires1realpage andbothlabels; oldheader/rawSVG/PNG/DXF/TXT assertions preserved. Text API agrees with existingStdioSVG oracle. Parsers remain private primitive-return testhelpers/dev-onlydependencies; BoardOwnergenuineclose unchanged. No concrete bounded source mismatch found; actualnative4-lawproof remains pending. Birth reports latesttestextent211603/23/2662,production119970/11/1752,physicalprobesequal.

## Root7993 Lock Frontier and Dev-Edge Repair

Root7993 authoritatively stopped beforetests atlockrefresh: newtext-inverseDWGgeometryworkspace inheritance absentactual2dworkspace. Independentcurrentmanifestreview verifiesbothnewDWGread/geometrydevdependencies nowuse explicitrelativepaths toactualAC1024 IOimport/schemaGeometryownerCargo manifests, exactpackage.names match. Recordsedge resolvesactual2drecords withtestkitfeaturedeclared; existingTraceworkspaceentry resolvesrealTraceowner. acadrust0.5.5/default-featuresfalse,lopdf0.44/false,quickxml0.42 remain dev-only. No runtimeedge/externalAPI/cap/sourcecontrolchange; actualnativeIO4laws remainpending. No immutableoldcaptureclaim.

## Actual Workspace Dependency Refusal and Correction

Root request7993 ended before physical2d tests during the owned Cargo.lock refresh: the new text-inverse dev dependency on semio-s-artifact-stdio-dwg-geometry used workspace=true, but its actual Puzzle2d workspace declares neither that entry nor the new dwg-read entry. The temporary exact refusal remains in generated/sqlite2d-4.log. This is an incorrect test manifest assumption, not a compiler/runtime result or a reason to change policy.

Both first-party dev edges now use direct relative paths from the text-inverse manifest to the actual registered Stdio AC1024 read and geometry package manifests. Each path was resolved and its exact package name checked before the edit. No root workspace table/alias or production dependency was added. The allocator dependency was checked against the actual Puzzle2d workspace: semio-framework-trace resolves to the real framework/trace package. Existing records testkit is a direct path with testkit enabled. External acadrust0.5.5/default-features=false, lopdf0.44/default-features=false and quick-xml0.42 remain explicit test-only declarations.

The Rust tests, neutral case/schema, actual AC1024 route, four-law selector, allocator and source contents are unchanged by this manifest correction. Their latest exact source census remains ordinary119970/11/1752 and test211603/23/2662. Actual dependency refresh, compilation and four-law execution remain required. Low is reviewing the corrected resolved endpoints; no native command was launched by this lane while the root coordinates its next handoff.

##3d Advertised TXT Carrier Execution Gap

Current3dIOregistry advertisesTXTimport/export. Actualleaves under io/{export/serializers,import/deserializers}/artifacts/txt/v_utf_8/any call print_dsl.into_bytes and UTF8+parse_dsl. Existing3dphysicaltestsource callsbothdeclaredJSONleaves at116–117 but scopedIOtests containnoactualTXTleafinvocation; hostPack/DSL callsArtifactDsl directly. Therefore theseproofs donotexecuteadvertisedTXTcarrierboundary. Noimplementationbugclaimed(thinrealwrappers); missingactualruntimeevidence shouldbequalifiedinsideexisting3dphysicalowner alongsideJSON, with exactbytes/typedstate andUTF8refusal ifrequired. No newowner/compiler/testwrittenbyaudit. 2d/5dmappedformatlawsalreadyexercisetheiractualTXTleaves.

## Original Board Identity Preservation

The old fixture helper retained Puzzle2dSnapshot::default().schema (board.ports.directed.v1) and copied only nodes/edges/target regions from its temporary JSON specimen, discarding that specimen’s puzzle.2d schema. The new directly constructed portable board now explicitly authors board.ports.directed.v1 in both fixture and case schema, preserving the original actual snapshot and TXT identity. Geometry/labels and all four output laws are unchanged. Strict Ajv validation passed after the correction. Latest exact text-inverse Source is ordinary119550/11/1752 and test211047/23/2662; physical probes exactly match. The final IO inventory was refreshed, and native material parsing remains unexecuted.

## Current Canonical Foreign Route Staging Review Frontiers

IntegrationstagedtypedSerializer/Deserializerentriesforalladvertised2d9/3d4/5d7foreigndirectionsandpublishesactualsubsetIOentries, distinctfromoldComposerregistry. Boundedinitialreviewfound2dDWG/DXFmoduletyposvac1024/vr12 versusactualv_ac1024/v_r12; Integrationnotified. CurrententryhelpersrequireArtifactNativeSnapshotaswellasPack; directPuzzlesourcehasnoexplicitimpl, soactualderive/macro/nativebindingmustbeidentifiedbeforecompile-readinessclaim. FirstcheckpointisgenuineIoRunControl butdoesnotbyitselfproveinteriorlargeconversionprogress/cancellation. No tests/compilerlaunchedbyreview; finalrouteAPI/mount/dependencyreviewpendingconfirmednativebinding andactualrouteacceptancelaws.

Typedroutefollowthroughreview: canonicalIOrootsallpubliclymountactualroutes, subsetsactuallypublishentries;2d9/3d4/5d7directions,fidelityExactJSON/TXT/ZIPandLossydrawingsagreeadvertisedcontracts. ExistingIoRunControlcheckpointgenuinelyadmittedbeforeleaf,errorconvertedtoIoError,no fakecontrols. Hubfixturearraylengthcheckedbeforezipavoidsomittedtail. Dwg/DxfmoduletyposcorrectedbyIntegration. ActualArtifactNativeSnapshotimplmissingconfirmed; Rootcoordinatinggenuinecontrolledcarrierhelper+explicittypedimpls,nohelperboundweakening. Finalcompile-readinessandruntimepublicrouteproofpendingthatwork; singlecheckpointnotinteriorconversionprogressproof.

## Current Explicit Native Binding and Physical Route Acceptance Review

Actual3typedbinaryfacetsnowimplementArtifactNativeSnapshotbydelegatingexistingstoredecode/encode_native_snapshot_recordwithrealenvelope/specproducer/controlledbindprojectandcallercontrols; no blanket/helperboundweakening. NativeDecodeControlaliasesresolveonetype. PhysicalregisteredJSONhelpersusesamegenuineoriginal_owners decode/encode directionsfornativecarrier/export/import/restore,independentSerdewire/reencodeandfullrelationalexactcomparison. Restoredtypedrootsguarded;3dguardHostimportnowcorrectedatactualroot. Existingdirectleafwordlawsremain. Hubnow9laws(old7+completeindependentroster+7genesisroutepairs),notfullfieldsubstituteforphysical3owners. Finalreportedcensuses2d75571/3/888,3d32242/2/381,5d95158/5/1244,Hub18186/3/236;probesidenticalunchangedcaps. No furtherconcreteboundedtype/mount/controlmismatchidentifiedafterreportedcorrections; allactualnative/workerinteriorprogressproofpending. Sharedhelperpostconstructcheckpointdropisreviewfrontiernotprovenpanic; avoidunprovedruntimeclaim.
