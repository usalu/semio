# Borrowed Text Corpus Progress

The DSL test corpus records canonical scalar text for bare identifiers, whitespace, empty text, Unicode, and control-character escapes. The matching test compares the existing owned canonical printer with the recorded document and the borrowed prospective measurement, and uses SQLite to independently verify UTF-8 byte lengths. It does not test a retained writer.

The modified Rust test file passed an AST-only parse with Rust 2024; no semantic compilation or test execution has been performed. The writer still needs explicit resumable traversal, bounded token emission, actual encoder work admission, physical receipt and retirement integration, plus tests for all authored compound shapes and failure frontiers.

A first internal retained text-token component now incrementally scans bare identifiers, escapes scalar characters, emits at most one byte per turn, and uses the original encoder action loan for transitions and close. The token test checks the corpus with one-byte emission and work denial before and after a prefix. This is groundwork only: output physical admission belongs to its enclosing cursor and the record traversal, owner integration, cancellation-frontier tests and semantic validation are still outstanding.

A private flat scalar record component now retains source and schema identity, scans positional order and missing placeholders, emits named scalar fields and composes the text-token cursor under the original work loan. Language-neutral fixtures cover scalar text and positional ordering. Both new Rust files pass AST-only parsing; semantic compilation and test execution remain pending. The component does not implement call layouts, compound shapes, caller-owned output publication, physical receipt integration, or an adapter. It must not be presented as the requested complete record writer.

Scalar record source projection is now reserved against the actual work loan before access, with its action committed even when projection fails. The internal failure carries both primary refusal and a secondary settlement error. The token reborrow and token transition are separately accounted. The complete writer, publication custody and native checks remain outstanding.

The flat cursor quotes its next output byte and covers positional order, identity changes, source refusal with settlement refusal, denied grants and cancellation after a prefix in finite test cases. Inline staging copy receipts, nested frame allocations, complete typed output construction and publication, and the coordinated native build remain outstanding. The finite test loans are interface fixtures, not original Job authority proof.

The scalar ordering fixture now has an independent SQLite exact-document oracle as well as the existing owned-printer comparison. A direct SQLite run produced the expected ordering and document. Field-ID validation and enum selection each inspect one ordinal per action. The Rust selectors remain unexecuted.

The private cursor now supports one inline list frame whose elements are scalar values, and named-field scans honor the existing canonical shape rank. The list fixture compares the cursor with the owned emitter and SQLite and exercises cancellation at a retained list prefix. These tests have not been run natively. Nested lists and records, tables, maps, statements, blocks, physical staging receipts and publication remain unfinished.

The Unicode text-list case now exercises per-element reborrow and one-byte continuation. Pending output drains before a later field is projected, verified by a projection-counter test. IO integration cannot wrap this cursor with the current typed encoder effect quote because both reserve encoder work; a shared transaction or a physical-only quote is still required.

The private single sequence frame now also handles scalar tuples and coordinate/direction sigils, with language-neutral fixtures and SQLite/owned-print comparisons. Nested sequences and records still require a genuinely funded stack; source and output physical custody still need integration. Native selectors remain queued.

Borrowed byte slices and byte-source views now use bounded base64 groups with retained source offsets and one-byte output. The neutral corpus covers empty, one-, two-, three-, and four-byte inputs; SQLite hex and an independent base64 encoder verify the four-byte example. The native tests and actual physical staging receipts remain outstanding.

The private scalar path now handles intrinsic null, booleans, signed/unsigned/floating numbers, text, and both byte-source variants, with neutral fixtures and independent formatting oracles. It does not handle intrinsic arrays, objects or paged text. Inline staging receipts, an unwind-safe action transaction, nested frame allocation, original output-slot publication and native tests are still outstanding.

The token cursor now exposes an unmetered one-action transition and close that accept caller-owned performed/physical markers for the staged IO transaction. It calculates escape and UTF-8 output bytes directly, so the exact next output span is zero or one byte and no retained escape-byte staging is allocated. Its original private self-metered wrapper remains in use by the unfinished scalar cursor; no owner adapter has migrated. The expanded eight-case text corpus passes the SQLite UTF-8 length oracle, and the Rust effect test is queued, not executed.

Text-token effects no longer retain escape-byte staging; UTF-8 and escapes are generated one output byte at a time and the raw effect reports an exact byte copy to a caller-owned marker. The byte path likewise computes each base64 character from bounded source reads without retaining an encoded word. Tests for partial base64 groups and cancellation mid-group are present but await native execution. The enclosing record still lacks physical transaction integration, fully guarded source callbacks, and recursive frames.

Numeric output now reborrows and formats into transient local scratch for each emitted byte, so the cursor no longer retains numeric byte staging. This trades repeated formatting for bounded retained custody and has not been performance-tested. Transient formatting work and complete physical receipt accounting still need the shared transaction design; the cursor remains private and self-metered.

The language-neutral text corpus now includes bare and quoted Unicode, a three-byte scalar, a two-digit control escape, and lone-surrogate JSON examples. The queued Rust test checks that serde_json rejects those surrogate examples before a Rust string can be supplied to the token cursor.

A separate JavaScript arithmetic model of the token UTF-8 byte calculation agreed with Bun Buffer on sampled Unicode scalar values; this checks the formula only and is not execution of the Rust cursor.

A private source-view slot now provides an unmetered marked projection and paid-release transition, with identity/occupancy checks and a test for a panicking source callback retaining its work marker. It borrows the original string and does not issue source-byte spans. It has not been connected to the record frame, IO transaction or original Job, and its Rust tests have not run.

Emission tests now have a firstparty IO-control dev dependency and exercise the staged work quote around the source-view slot, including panic with a refused post, and the staged physical quote around token byte effects with a cumulative finite fixture recipient. These tests are source-authored and syntax-parsed only; they do not use or prove original Job/Source authority.

The emission Cargo manifest parses as TOML with IO-control listed only as a dev dependency. A read-only sum of the known emission source files was 192,222 bytes; this is not the repository source-boundary capture and does not replace it.

Latest lightweight checks: all three private Rust files parse as AST, all ten corpus files parse as JSON, and the emission manifest parses as TOML. These checks do not type-check or execute the Rust code.

A new neutral list-of-records fixture establishes the FEM-relevant nested shape before implementation. Its baseline test compares the owned canonical printer with SQLite and checks that node text views borrow original storage; the retained cursor does not yet consume this fixture. SQLite produced the expected document directly, and the Rust file passed AST parsing only.

A private caller-slot tree now stages recursive records and lists of records without internal frame allocation. It binds source identity and frame-slice address, returns required slot count without work on shortage, and pays frame transitions/close through the supplied work loan. Its FEM-shaped list-of-records fixture checks shortage, cancellation before child birth, one-byte output and slot close against the previously established owned/SQLite document. The slice capacity and prefix scratch have not been admitted or receipted through the original allocator, growing/relocating backing is not implemented, and the Rust selector has not run. Blocks, statements, tables, maps, original source spans, guarded unwind and output-slot publication remain unfinished.

The caller-slot tree now has a second neutral fixture for three nested record frames. Its staged test compares the complete document to SQLite and pays frame closure, while the list-of-records test checks same-backing slot shortage, frame-storage identity and cancellation before child birth. A direct SQLite run matched the deep fixture; neither Rust test has executed. The caller must provide one stable, already-admitted typed slice; moving/growing its backing, allocator receipts, source spans, guarded unwind, output publication, nested lists and compound grammar beyond records remain unimplemented.

The nested cursor now constructs source-path scratch only inside an admitted projection action and binds both original source identity and the caller slice base. The list-of-records test checks changed source and changed frame storage without mutating the frontier. Prefix scratch accounting, caller slice allocation/relocation, output custody and panic-safe transactions remain open.

A private FEM-shaped named block and statement traversal is staged, including per-byte variant-key matching, indentation and child frame paths. Its neutral fixture is compared in a queued test against owned canonical output and SQLite, with cancellation after a retained statement prefix. SQLite produced the expected document and the Rust files passed AST parsing; semantic compilation and native execution have not occurred. Positional/nested blocks, tables, maps, source-span authority, guarded action unwind, slot-allocation receipts, final output publication and original Job proof remain open. The coordinator and other agents reported workspace-routing timeouts during this work.

The private tree now stages named columnar scalar tables alongside named blocks and statements. The table fixture has owned-print and SQLite comparisons plus cancellation at a row prefix; SQLite matched directly and the source passed AST parsing. These are not native test passes. Source-span admission, typed frame-slot allocation and relocation receipts, guarded action unwind, output publication, tables with all compound cells, maps, nested collections, positional blocks, and original Job proof remain open. Root coordination is currently unavailable after a workspace-routing timeout.

A later read-only sum of the known emission source files reached 244,278 bytes before the latest mixed-fixture additions, against the existing 262,144-byte source boundary. This is not an authoritative capture; a legitimate companion/package carve may be needed. No ceiling was changed. Root coordination remains unavailable after its workspace-routing timeout, so no independent native compiler was launched.

Additional staged fixtures cover a block/statement table cell, mixed block-before-table ordering, and a record-valued table cell with glued braces. SQLite generated the expected documents and the files parse as Rust AST; owned canonical and retained cursor assertions are present but unexecuted. The source boundary may now require a package carve; no quota was changed.

The latest known emission source-file sum is 249,447 bytes of the unchanged 262,144-byte ceiling, before any other files the repository capture may include. A real test/package carve is likely needed before extending the grammar. The table record-cell fixture has a SQLite oracle and staged owned/cursor assertions; SQLite and AST parsing passed, but no native compilation has occurred.

## Durable encoder fixture loans

The private token, text-source, scalar-record, and tree fixtures now lend work through `NativeEncodeWorkIssuer`, and the physical quote fixture lends construction through `NativeEncodeConstructionIssuer`. Issuers remain in their fixture scope across turns. These are finite fixture authorities, not original Job or Source proof. All four modified Rust files pass syntax-only AST parsing; no native semantic compilation or test result is available. The eight Rust files under the emission printing and encoding directories currently total 252,684 bytes as a non-authoritative source-cap estimate.

The independent OS post-turn relay encoder must migrate its scoped Source pause/resume calls to the original retirement recipient and caller-owned allocation-progress slot; this is outside the borrowed writer source ownership.

The physical-quote fixture also follows the current `run_native` callback signature, without taking or spending a nested native work reservation. This remains source inspection only until the shared native driver runs.
