# Office Native Facet Authority

## Observed implementation

Office SQLite maps remain individually authored relational entities and relationships. The newly controlled native representations encode typed schema, OPC package metadata, intrinsic binary part octets, typed XML documents and PPTX presentation entities. They do not encode ZIP archives as snapshot bodies. The external OOXML converters still own real ZIP/OPC transport.

The existing XLSX snapshot grammar describes external SpreadsheetML markup. Its binary protocol describes external ZIP entries. DOCX/PPTX carry analogous external facets. These must be hand-authored for their actual native snapshot representation after the new facet laws produce genuine runtime RED. The root native factory currently overrides `pack_schema_hash` with SHA256 of the old protocol asset. This must not be described as a structural typed schema hash.

## Existing protocol walker boundary

The canonical DSL grammar implementation is `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/🦀️.rs`. In `walk_protocol`, named records under `Framing::Record` set position to input length without traversing their declared fields. `walk_prim` explicitly rejects unresolved `Prim::Ref`. Consequently, `trace.consumed == body.len()` alone is not evidence that a named nested record matched its described layout. Pack-backed owner protocols often describe logical records using `use semio.pack`; that is a different physical layout from the newly literal Office streams. No permissive consumed-rest behavior can be counted as a complete deep validation result.

## Recommended explicit authority

Declare literal native framing explicitly, interpret the declared start record, resolve borrowed record definitions, and traverse nested arrays/conditional fields with an iterative admitted cursor. Preserve current scalar wire primitives, validate enum/discriminator domains, reject missing/cyclic non-consuming definitions, truncated bodies and trailing bytes. Do not reinterpret framed Pack records as linear literal records. Give actual Pack framing its owned physical/schema admission path. A controlled walk should share byte progress and cumulative frontier admission. Its neutral fixture must distinguish identical-length malformed bodies from accepted bodies and exercise nested/repeated empty records, discriminator guards and truncation.

This is an unfinished prerequisite for honest Office native facet/protocol identity. The current Office native controls and relational provider tests are separate runtime evidence. No whole-goal completion is inferred.

## Literal cursor staged

An explicit framing literal declaration is now part of the canonical protocol model/parser/printer. Its controlled named-record walker is still a strict refusal stub pending genuine runtime RED. The neutral fixture declares Root and recursive Entry boundaries, including a variable-count array, conditional child, UTF-8 label and final byte. The independent source oracle reconstructs exact bytes with Buffer and records parent identities in Bun SQLite, with strict Ajv admission. Two public kernel native laws cover all truncated prefixes/trailing data and iterative depth8192 / 100KB interior cancellation / before-allocation budget rejection. The existing public admission route includes these laws and the newly authored Child input laws.

The first public retry reached test compilation but stopped at ten ambiguous FromValue/DslField calls in the new Child test facet. That fixture owner has received the exact diagnostic. This is a pre-assertion prerequisite, not literal-cursor runtime RED or direct Child runtime proof.

## Literal cursor actual proof

The qualified public fixture run reached genuine strict missing-cursor RED: 58 selected, 33 executed, 32 passed and one failure; 25 were not run after fail-fast (4efef5a7-373b-4f8e-a616-532688eeee59). The implemented iterative cursor then ran the complete public selector with no fail-fast: 58/58 passed, zero skipped, 5833377c-c8fd-497f-b3b7-282436ed292d, 1.246 seconds assertions / 27.0 seconds uncached. All three Child input laws and five Child output laws actually executed in this green run.

The cursor owns only admitted borrowed scopes/frontiers. It walks the declared root, resolves named records and enums, iterates variable/fixed/field-count arrays, evaluates conditions within each record scope, validates scalar widths/minimal ULEB/UTF-8, and rejects trailing bytes and unresolved definitions. It handles depth8192 on a 256 KiB stack, interior cancellation during 100KB primitives, and ownership refusal before the first scope allocation. Pack Record and Chunked behavior remain their distinct declarations.

The XLSX binary facet is now individually handwritten with explicit revision, OPC package/member/content-type/relationship fields and the actual multi-document XML node/attribute/declaration/doctype/entity records. Its owning runtime retry is pending; earlier protocol failure is not yet claimed repaired.
## Current Literal Protocol Verification

The current public kernel route executed 58 native laws successfully (Nextest 5833377c-c8fd-497f-b3b7-282436ed292d), including the iterative literal protocol cursor, all three direct ArtifactChild controlled input laws and existing output/lifecycle controls. The source run independently checked the literal fixture through Buffer, Ajv and Bun SQLite.

The XLSX source profile suite was repeated after its periodic yield refinement: eight laws passed, zero failed. Exact XLSX native profile and facet verification is currently compiling on the private I/O Cargo lane. DOCX and PPTX now have explicit whole-body protocol truncation/trailing laws staged against their stale facets; no successful execution or correction is claimed yet.

DOCX's new neutral profile corpus distinguishes actual namespace declarations from ordinary text that happens to contain the VML URI. Its exact named declaration and typed diagnostic laws are authored before implementation. Their native execution is pending the single current XLSX lane.
## Literal Text Admission and Measured Factory Identity

The current XLSX native owner run executed all 17 laws: 14 passed and three failed (Nextest 0599323e-a5eb-433b-908a-a28ec7b18b64). The complete literal binary protocol and typed profile/control laws passed. The failures established that the handwritten Text facet used unsupported raw regular expression syntax and the manifest still declared the old binary protocol hash.

A neutral intrinsic lexical corpus then executed in the public kernel route: 59 laws ran, 58 passed and the new lexical law failed (Nextest bf253a41-a8e4-4591-ab2b-2606c78a3766). It checks canonical lowercase octet pairs and the entire unsigned 64-bit decimal domain, including malformed pairs, leading zeros and overflow. Independent RegExp and Buffer oracles are mounted in the existing source facet. HEX_BYTES and U64 terminal matching are now implemented, and the XLSX Text facet uses those exact intrinsic terminals without unsupported regex productions.

The independent XLSX source hash law observed eight passes and one failure. Node crypto and Bun CryptoHasher measured the handwritten literal binary protocol SHA-256 as b81858b598a8d0bdcbcc36b48291e9d3a3f638adf4cac479e5ebce714c9a1a17. The owning manifest was hand-corrected to that measured identity; the factory continues to calculate the protocol digest itself.

Both current green retries stopped in fresh Nx graph construction because the renderer WGPU frame-worker generated JavaScript has no declared producer. No successful execution is claimed for these current lexical/hash corrections.

DOCX source subsequently exposed a separate real incidental-child namespace failure: five passed, one failed. Requiring the actual document root's resolved namespace corrected that failure; the registered source suite then passed six laws and 60 assertions, and its public package check passed. The corresponding Rust profile implementation is authored but deliberately not mounted until the pending native exact-profile baseline can execute. Whole-document text is not used as profile evidence.

### Current lexical boundary and Office admission

The changed renderer producer admitted the current public native gate. Nextest `31a7b08d-33dd-4ce6-9063-b24c61f3bdd0` executed all 59 laws: 58 passed, and the new lexical `[00ff,0]` case failed. The core numeric lexer splits contiguous hexadecimal input into numeric and identifier tokens; a single-token HEX_BYTES predicate was therefore insufficient. The replacement matches one maximal contiguous source atom, validates its exact lowercase paired-octet alphabet, and consumes only complete token boundaries. The neutral corpus also covers exponent-looking hex, internal whitespace, decimal punctuation, underscore and trailing invalid alphabet. Its current retry is not yet evidence of success.

The XLSX independent source gate is genuinely green at nine laws/83 assertions after the definition's factory hash was aligned with the exact authored literal protocol SHA measured separately by Node crypto and Bun CryptoHasher. The native declaration and grammar corrections still await a fresh owning run.

### Verified lexical and XLSX correction

The contiguous lexical replacement is genuinely green: public native Nextest `533b2379-dbfc-468c-a793-b9b9b55a7129` ran all 59 selected laws successfully, zero skipped, 3.321 seconds assertions/40.3 seconds Nx. Independent source ran six laws successfully. XLSX then ran all 17 selected SQLite laws successfully (`a3e91136-7c19-4e10-bc07-81ec4a6cc167`, 0.559 seconds assertions/1 minute 44 seconds Nx), with 77 unrelated package laws outside the selector. Source ran nine laws successfully. Its actual declarations exercise wildcard plus exact strict/transitional policies, warning preservation and foreign-profile import refusal; its physical body facet now walks the complete authored logical binary fields and rejects truncation/trailing bytes.

DOCX's fresh baseline reached genuine assertions: `e83cdd44-7e1f-4d3f-b593-3c547c26aafe`, twelve executed, nine passed and three failed, 77 outside the selector. Failures were the missing typed exact-profile guard, missing exact profile declaration and stale ZIP-shaped binary facet. The explicit guard and strict/transitional declarations are now mounted, and the binary facet is hand-authored against the existing literal schema/OPC/XML wire. These repairs remain unverified until the current owning gate executes. A separate native Text grammar law and independent factory-hash source law were added before correcting those remaining assets.

PPTX now has an independent typed namespace fixture and three additional source laws. The first attempted nonexistent source target is an infrastructure invocation error, not feature RED; the actual registered `test` router is running next. Its typed profile implementation remains absent until actual missing-owner policy failures are measured.

### DOCX complete selected proof and PPTX measured gaps

DOCX current selected native proof is green: Nextest `39ddc22d-0c93-49a0-b5c8-9617ce7ccf73`, 13 executed/13 passed, 77 outside the selector, 0.500 seconds assertions/24.1 seconds Nx. The independent source gate ran seven laws successfully and its public check also passed. Prior protocol/profile baseline was nine pass/three fail; the separate first profile implementation correctly passed direct policy and complete binary-walk laws, leaving three measured stale grammar/factory failures. The current literal facets are explicit schema/OPC/XML fields and independently measured SHA is `8ab98ce506ac2e5d6b432dc20a85507f723dafe79071b08bdb0a74e18ebd070c`.

PPTX source policy baseline genuinely executed nine laws: seven pass/two missing-owner failures, then nine passed/73 assertions after its typed policy implementation. Its current native baseline executed 15 laws: 11 passed and four failed on missing owned exact guard/declaration and both stale facets. This proves the existing typed/native cursor laws actually run, separately from new profile/facet repairs. An explicit strict/transitional owner guard and registrations are now mounted, and complete literal presentation/shape/run/transform/OPC/XML facets are hand-authored.

The PPTX Text facet also requires canonical signed64 transform and unsigned32 font-size terminals. Independent Node Buffer vectors exercise extrema, values beyond safe JS integer precision, negative zero, leading zeros and overflow. Public native missing-terminal RED is authentic: `db6ff3c3-004a-48c9-a1e1-c4e315e75d9e`, 59 executed, 58 passed and one lexical scalar failure. The implementation retry remains pending. Binary font-size fields are physically ULEB64 with the actual typed reader narrowing to u32; the facet names every field but does not independently assert that narrower semantic ceiling. No blanket facet-value or unrestricted-depth claim is made.

### Signed scalar runtime and current native lane

The canonical I64/U32 Text terminal repair is genuinely green against the expanded neutral extrema/refusal corpus: public native `868e23f7-de05-45de-8aa9-6c613c6fc20d` executed all 59 laws successfully, zero skipped, 1.619 seconds assertions/31.8 seconds Nx. Independent source executed six laws successfully. Canonical negative zero, leading zeros and both integer-domain overflows are refused.

PPTX's independent new protocol hash source law produced actual stale-receipt RED (nine pass/one fail). Both Node crypto and Bun CryptoHasher measured `a2d087ee150879f8e6aa463f1f307babc5edc3b51948d0227f80196295adf668`; the owning definition is aligned. Current public source then passed all ten laws. Current PPTX native selection is running against the complete owner guard, declarations and explicit facets; no native success is inferred.

The existing quick full-package verification commands for XLSX, DOCX and PPTX are now explicit launch entries in group 4_gate, orders 408.6901–408.6903. They call only the existing owning test routers, use quick budgets and no-fail-fast, and preserve all concurrent launch additions. Full-package results are still pending separately from the selected snapshot proofs.

### Current PPTX proof and complete XLSX owner RED

PPTX current selected native proof executed all 15 laws successfully (`796ff8c9-1bba-4472-a33a-bdcbedebe2ae`, 3.088 seconds assertions/1 minute 32 seconds Nx), 83 package laws outside the selector. Independent source executed ten laws successfully; the strict public package check also passed. This includes exact wildcard/strict/transitional declarations, semantic namespace policy, diagnostics, complete logical native facets, signed transform and unsigned font-size Text domains.

The first complete XLSX quick owner suite executed 92 laws: 90 passed and two failed (`1c2bd2ab-124d-4f82-820e-9935d1813644`), with two pre-existing ignored asset-writing helpers skipped. Failures were an obsolete test feeding contained XML into the snapshot's logical native Text grammar, and obsolete shipped ZIP-hex Text/Pack fixtures. The conformance law now checks the actual complete owned native Text body, while the protocol law requires consuming every logical snapshot byte; existing independent XML/OPC laws remain unchanged. Canonical native Text (5047 bytes) and Pack (2509 bytes) were measured into the ticket through the existing ignored fixture helper, then the two shipped demo inputs were updated and that temporary asset writer was removed. No migration script or fallback codec remains. The fresh complete quick suite is running; no success is inferred.

Both actual launch configuration and its canonical `.vscode/🧩️launch.seed.jsonc` now declare the same three full Office quick commands at 408.6901–408.6903. Concurrent FEM entries are preserved.

### Full XLSX owning runtime

The fresh complete XLSX quick suite executed all 92 selected laws successfully (`239c455c-aa1b-4894-9475-6664eabc0412`, 0.638 seconds assertions/19.9 seconds uncached Nx). One unrelated pre-existing mutation-quintet asset-writing helper is ignored. The snapshot demo writer was removed after measurement, and native Text/Pack fixture equality now passes along with complete literal grammar/protocol admission. This is complete package evidence for the current XLSX source, separately from the 17-law SQLite/profile selector.

Complete DOCX quick then executed 89 laws: 87 passed, two failed (`aca03505-a9e0-470f-a237-44106a89f941`, 1.412 seconds assertions/38.7 seconds Nx), one pre-existing demo-writing helper ignored. Both failures are the same obsolete contained-XML grammar assumption and obsolete native snapshot fixtures, independently observed in this actual owner. The grammar law and complete protocol byte-consumption assertion are corrected, and current literal native demo measurements are pending. No DOCX complete-green claim is made until the changed assets run.

### Full DOCX owning runtime and PPTX independent gaps

Complete DOCX quick is genuinely green: `604dc4ac-fefa-4216-9beb-4ba2279c91e3`, 89 executed/89 passed, zero skipped, 0.969 seconds assertions/32.2 seconds uncached Nx. The ordinary native `.docx` demo measurement is unchanged; only obsolete owned Text (3535 bytes) and Pack (1753 bytes) twins changed. The asset-writing helper was removed. Whole-package evidence includes the actual exact profiles, declarations, current literal codecs and semantic SQLite controls.

PPTX complete quick then executed 97 laws: 93 passed and four failed (`7287e5b8-2ebd-4e02-a7c6-ba4f1b3afc1d`, 5.195 seconds assertions), one pre-existing demo writer ignored. Besides obsolete grammar/demo inputs, two clean-profile composer laws still passed a bare hex-encoded ZIP to the native Text analyzer. Those now construct the owned snapshot from the actual hand-built OPC input and emit its canonical native Text, retaining the original independent Strict/Transitional package evidence. The rejection and wire-recheck laws additionally require real profile diagnostics and prohibit skipped-decode false positives. Full repaired runtime is pending.

### Full PPTX owning runtime

Complete PPTX quick is genuinely green: `8e2d2b95-08fb-4736-87af-60a732ec0e45`, 97 executed/97 passed, zero skipped, 2.984 seconds assertions. Both repaired profile composers run from their actual independent OPC inputs through canonical owned Text; negative cases require genuine profile diagnostic codes and clean rechecks cannot pass on skipped decode warnings. Literal demo Text is 13424 bytes and Pack is 6589 bytes. The temporary demo writer is removed. Complete XLSX, DOCX and PPTX owner suites therefore passed on current source, with ignored counts distinguished above; this does not imply remaining ZIP, DWG, PDF admission or Layout work is complete.

### Full-Suite Repair File Ownership

This final full-suite pass changed the base schema unit test facets under each of XLSX, DOCX and PPTX (`🧬️schema/🧪️tests/🔬️unit/🦀️.rs`), and each owning base demo's `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio` and `🎒️.pack.semio`. PPTX additionally changed the exact Strict and Transitional `🚪️io/🧪️tests/🔬️derived-composition-unit/🦀️.rs` inputs/diagnostic assertions. Existing independent ZIP/XML model and ordinary office-export tests are preserved. `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` contain the same three explicitly registered full-owner commands. No runtime dependencies, migration scripts, legacy codecs or guessed dialect aliases were added.
