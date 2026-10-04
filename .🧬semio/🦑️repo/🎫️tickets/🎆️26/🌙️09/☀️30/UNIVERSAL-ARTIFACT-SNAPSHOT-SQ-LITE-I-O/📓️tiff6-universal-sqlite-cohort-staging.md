# TIFF6 Universal SQLite Cohort Staging

## Authority and boundaries

The persisted owner is `TiffSnapshot` at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🦀️.rs`. Its complete fields are literal `schema`, `TiffByteOrder`, ordered `Vec<TiffIfd>`, and document RGBA bytes. Each IFD owns ordered tags and a separate raw strip buffer; each tag independently owns its u16 tag number, declared field kind, and one of twelve actual value variants. The existing hand-authored seventeen-table SQL remains the sole relational authority. No new payload table, inferred identifier, serialized snapshot blob, or parallel snapshot DTO was introduced.

The semantic snapshot admits literal empty/NUL/Unicode schemas, duplicate tag numbers, empty sequences, a declared kind distinct from its value variant, IFD0 strip bytes, separate directory bytes, and every floating word. Its typed SQLite export/import must retain all these fields.

The existing ArtifactDsl/ArtifactPack implements a distinct documented external TIFF carrier: Text is hexadecimal TIFF bytes inside the real semio text envelope; Binary is TIFF bytes inside the real pack envelope. The actual codec canonicalizes primary raster tags, sorts entries, emits RGB from document RGBA, ignores primary IFD strip bytes, reconstructs pointer tags, strips secondary offsets, and restores the fixed schema. The new native laws compare controlled decoding/encoding at this external boundary against that real behavior. They do not mistake that boundary for full owned-state preservation, or require fictional TIFF headers for otherwise arbitrary owned snapshots.

## Schema-first neutral cohort

Added:

- `snapshot/🧫️fixtures/🪶️sqlite/🚦️cohort/🔣️.json`
- `snapshot/🧬️schema/🪶️sqlite/🚦️cohort/🔣️.json`

The corpus specifies all seventeen table row counts (52 domain entities), literal schemas, nine binary32/binary64 words including signed zero, subnormal/minimum normal/maximum finite/infinities/quiet and signaling NaNs, 100,000 pixel bytes, long Unicode metadata, actual TIFF dialect and envelope identity, SQLite's physical signature, and independently handcrafted real 131-byte little/big endian TIFF raster files. Their nine entries, offsets, counts, and RGB bytes are checked with DataView rather than a first-party decoder.

Six new Source laws reuse the complete canonical TIFF model and existing provider. Independent Ajv validates the closed cohort schema; Bun SQLite checks the entire authored DDL, every table count, real file integrity, independent field/value kinds, and serialized-file restoration. SQL `printf('%016x',value_bits)` interprets all exact signed INTEGER bits without JavaScript precision loss. DataView independently supplies IEEE classes and numeric query values.

## Executed Source RED and narrow fix

First registered Source baseline ran ten laws: eight passed and two failed.

- The exact 52-row ceiling genuinely failed because the Source provider added an implicit metadata reservation. Its three checks now use domain `count` rather than `count+1`. The actual I/O metadata attachment owns its separate admission; the provider contributes exactly its declared domain entities.
- The IEEE oracle incorrectly expected +0 when SQLite retained -0. Numeric equivalence is now checked independently of zero sign; the raw companion still verifies the exact sign bit. This was a harness mistake, not a semantic RED.

The first public owning package check also caught the new oracle's use of a Bun runtime Statement method absent from installed TypeScript declarations. The oracle now queries exact SQL hexadecimal words instead. No type shim, ambient patch, bigint-to-number conversion, or production dependency was introduced.

Current executed receipts:

| Registered command | Actual result |
|---|---|
| `bun nx run @semio-tech/stdio-tiff-rs:test-snapshot-sqlite-source --skip-nx-cache` | 10/10 laws, 134 assertions, 1.83s Bun / 8.7s uncached Nx |
| `bun nx run @semio-tech/stdio-tiff:test --skip-nx-cache` | owning build/export/independent declaration consumer/suite typecheck and 10/10 laws, 134 assertions, 10 exports, 12.4s uncached |
| Same owning package command without explicit artifact-output environment | 10/10 laws, 134 assertions, 10 exports, 10.9s uncached |

The file test honors either caller artifact-directory variable and otherwise resolves this active ticket's generated directory from its physical owner path. The thirteen-level Source and seven-level Cargo-manifest root paths were physically checked. Both test-path calculations are portable and require no command-line setup.

Logs under ticket generated: `tiff6-cohort-source-baseline.log`, `tiff6-cohort-source-current.log`, `tiff6-cohort-source-complete.log`, `tiff6-public-package-current.log`, `tiff6-public-package-harness-fixed.log`, `tiff6-public-zero-touch.log`. Actual SQLite file: `tiff6-semantic-cohort.sqlite`.

## Staged Native laws, no Native execution

Mounted tests only at `snapshot/🧪️tests/🪶️sqlite/🚦️cohort/🦀️.rs`, included by the existing owning native unit facet. Eight existing laws remain intact; the registered no-args native route now has fourteen authored laws.

The six new native laws are:

1. `sqlite_snapshot_tiff_cohort_public_real_file_preserves_every_owned_field`: real declaration-owned typed export/import, literal schemas, physical files, independent SQLite queries and reserialization.
2. `sqlite_snapshot_tiff_cohort_domain_row_limit_counts_only_owned_entities`: exact 52 domain rows and one-under refusal.
3. `sqlite_snapshot_tiff_cohort_controlled_input_matches_documented_external_boundary`: both byte orders and both native encodings, plus independently handcrafted raw TIFF fixtures.
4. `sqlite_snapshot_tiff_cohort_controlled_input_cancels_inside_real_metadata_and_admits_file`: exact file ceiling and real interior long-field cancellation.
5. `sqlite_snapshot_tiff_cohort_controlled_output_uses_actual_external_carrier_scope`: actual external codec decoding of the controlled output; arbitrary semantic state remains separately preserved in SQLite even when external raster encoding refuses it.
6. `sqlite_snapshot_tiff_cohort_controlled_output_cancels_inside_real_metadata_and_bounds_ownership`: low cumulative ownership refusal and known-byte-stage interior encoding cancellation.

Exact Root-only native route: `@semio-tech/stdio-tiff-rs:test-snapshot-sqlite-native`, no extra selector needed. Existing command remains `📜️script.ts test-snapshot-sqlite native`. No Cargo invocation, native compilation, or native assertion execution occurred in this worker lane. Current Rust hidden +1 row forecast is intentionally untouched until this owning law executes its authentic RED. Controlled hooks remain strict defaults until Root records the Native baseline.

## Unmounted controlled external codec drafts

Added adjacent drafts:

- `document/🚪️io/🛬️decoding/🦀️.rs`
- `document/🚪️io/🛫️encoding/🦀️.rs`

Neither module is mounted, and neither ArtifactSqliteSnapshot hook is overridden.

The reader borrows real TIFF headers and IFD fields; performs an allocation-free, cancellable cycle/row census; admits actual IFD/tag/typed-vector storage; reads all twelve scalar widths and exact float bits; copies ASCII using bounded replacement semantics; concatenates secondary strips under cumulative admission; writes PackBits directly into an admitted raster; fills admitted RGBA storage with interior checks; and adapts existing borrowed semio envelope parsing plus paid hexadecimal conversion. It preserves the actual external decoder's documented normalization.

The writer holds borrowed references to actual tag values with explicitly authored nine primary raster fields. It admits layout views, scalar-width/count/offset arithmetic and the final physical buffer; writes all twelve actual value variants directly without temporary full value buffers; emits the same real sorted IFD chain and strip layout; writes RGB directly from borrowed RGBA; and adapts genuine controlled envelope/hex emission. It uses the existing default uncompressed native carrier, not an alternate stored representation or an ordinary encoder fallback.

Private control refusals retain ValueError until their declared String terminal. Native budget counts are cumulative and known output work is physical bytes. Input and output file ceilings include the actual semio prefix before owning the body or envelope.

Only Rust syntax parsing was performed for these drafts and the new native law file. This does not establish type checking, linkage, runtime equivalence, limits or cancellation. Root's Native baseline and subsequent owned implementation validation remain mandatory.

## Registration and remaining work

The artifact-owned Rust script and Nx targets already existed and remain canonical. Added exact combined/native/Source launches to both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`, verified unused orders 408.783–408.785. No extra script, runtime dependency, Cargo lane, worktree, or Git mutation was created.

Native14 is staged, not green. Native controlled hook/default refusal and exact row admission await Root's authentic receipts before any production mount. The universal artifact goal remains incomplete.

### Authentic TIFF14 Compiler Gate (2026-10-03 01:24 UTC)

The fresh uncached public Native route stopped before discovery with56compilerdiagnostics:52retired dsl::json references plus4diagnostics at2old positioned constructors. Root narrowly ported13callerfiles:32readers now use semio_framework_pack_json::from_json_str with explicit Reject member policy;20writers use the current first-party to_json_string API. Existing fixture inputs/expected outputs were preserved, with no compatibility alias or runtime dependency added. Two malformed-input TextError creation sites now explicitly supply InvalidValue. The same14candidate route is running again. No artifact feature RED or acceptance claim follows from this compile-only gate; Native owner hooks/drafts remain unmounted. Receipt: generated/root-authentic-tiff14-owned-native-authentic-red.log, Nx31.9s.

### Authentic TIFF14 Behavioral RED (2026-10-03 01:26 UTC)

After the measured caller repairs, the same registered public Native route reached Nextestec40f3f4-9408-42d6-a7fb-9d58e5fe1dc0:14selected laws,7passed,7failed,101outside selector,120ms execution,Nx26.4s. Genuine failures confirm missing controlled native input/output hooks, metadata interior cancellation and the hidden +1domain-row limit. This authentic assertion RED now authorizes mounting the owned controlled integration and fixing row counting; the High IO worker owns that implementation next. External wire behavior remains distinct from complete owned relational SQLite state; no full-feature or green claim yet. Receipt: generated/root-authentic-tiff14-current-json-positioned-callers.log.


## Mounted Native Owner After Authentic Fourteen-Law RED

Root obtained Nextest `ec40f3f4-9408-42d6-a7fb-9d58e5fe1dc0`: fourteen executed, seven passed and seven failed, 101 outside selection, 120 ms assertions / 26.4 s Nx. Failures measured absent controlled input/output, metadata interior control and the hidden extra domain row. After that evidence, document I/O now mounts the concrete controlled reader and writer modules; the snapshot semantic provider binds its controlled native input/output hooks to them. Its two per-IFD/per-tag row checks count actual domain rows without adding envelope metadata. External Binary/Text remain actual TIFF/enveloped hex TIFF; arbitrary owned semantic state remains the typed SQLite authority.

Changed mount/behavior files: document `🚪️io/🦀️.rs`, `🚪️io/🛬️decoding/🦀️.rs`, `🚪️io/🛫️encoding/🦀️.rs`, snapshot `🪶️sqlite/🦀️.rs`. The existing first-party Value dependency was already declared. The mounted reader, writer and provider syntax parse successfully through rustfmt `--emit stdout`; no Cargo/typecheck/native run was performed by this agent. Root owns the native rerun.

## Independent Audit Follow-Up Staging

The independent audit correctly separates new performance and normalization gaps from the measured fourteen-law failures. Three additional native laws are staged, bringing candidates to **seventeen**: reverse-ordered duplicate tags must preserve stable exact external bytes within an explicit n-log-n work frontier, for both writer and reordered physical IFD reader; a secondary-strip carrier must admit exactly the row count of its returned normalized semantic snapshot and reject one less. The neutral fixture/schema now declare 1,024 reverse tags, duplicate groups, work factor, and secondary strip values. The audit laws syntax-parse; they have not executed. Sorting and normalization production behavior remain unchanged pending authentic new-law RED.

The existing Source facet adds independent BunSQLite numeric-tag/original-ordinal ordering and exact pointer-free secondary strip domain proofs. These validate neutral semantic expectations, not native sort complexity or TIFF normalization execution. Their registered Source receipt is recorded below after execution.

The reconstruction audit also found unadmitted BTreeMap/BTreeSet/reference-vector/typed-vector backing allocations. Current `max_value_bytes` is a semantic scalar/text/blob contract; it does not represent aggregate container backing storage. A separate explicit allocation authority/limit must be settled before claiming container admission or silently changing the existing exact semantic ceiling. No such allocation-limit fix or runtime proof is claimed here.

Additional changed inputs: snapshot `🧫️fixtures/🪶️sqlite/🚦️cohort/🔣️.json`, `🧬️schema/🪶️sqlite/🚦️cohort/🔣️.json`, `🧪️tests/🪶️sqlite/🚦️cohort/🦀️.rs`, and `🧪️tests/🪶️sqlite/🟦️.ts`. Parser readback: `🗑️generated/tiff6-audit-native-cohort-parser.rs`. No new commands, launches or dependencies are required; existing owning routes select the extended facets.


### Current Source Audit Oracle Receipt

The registered quick `@semio-tech/stdio-tiff-rs:test-snapshot-sqlite-source --skip-nx-cache` executed **12/12 passing, zero failing, 1,676 assertions**, 3.42 s Bun / 8.4 s Nx, uncached. Log: `🗑️generated/tiff6-audit-neutral-source-current.log`. Ajv validates the extended shared fixture/schema; independent BunSQLite establishes numeric-tag/original-ordinal stable duplicate order for all 1,024 metadata entries and the exact secondary strip relational domain without wire pointer entities. This does not establish native n-log-n complexity, native normalization admission, or backing-allocation budgets. All seventeen native candidates await Root execution of the changed cohort.

The reverse-writer and reverse-reader work laws are separate tests, so a writer refusal cannot hide the reader baseline. Existing fourteen plus these two and the normalization row law produce seventeen candidates.


### Authentic Seventeen-Law Audit RED and Authorized Repair

Root measured Nextest `7c2303cd-b27a-4aa6-a897-c739079d4381`:17executed,14passed/3failed,101outside,139ms assertions/1m8s Nx. All original fourteen controlled fidelity/public owner laws passed. The three new audit laws independently failed writer stable-work frontier, reader stable-work frontier and exact normalized secondary-row ceiling. Log `root-authentic-tiff17-owned-hooks-sorting-frontiers-current.log`.

After this receipt, TIFF uses a shared owner-paid stable merge sort of admitted original-position indices plus scratch. It permutes the actual tag values without cloning them and preserves equal-tag input order; known initialization and n-log-n merge/permutation work are cancellable. Decoder secondary pointer removal is stable linear compaction. Borrowed census excludes pointer entities only when the secondary directory really has nonempty matching strips, while the actual transient wire values still validate and admit their own native storage. Changed owner: `🚪️io/🗂️ordering/🦀️.rs`, its I/O mount and the controlled reader/writer. Syntax parse passed; Root owns fresh seventeen-law runtime verification.

## Actual Producer Allocation and Reconstruction Backing Staging

Current Native candidate count19: original17 including three authenticated sort/normalized-row audit failures, one repeated native producer cumulative ledger law and one relational reconstruction backing law. Each real Binary/Text owner input/output must transfer its admitted backing to the caller, preserve it through cancellation and refuse the same producer at an exhausted exact first-run limit. Production bridge integration is deliberately absent before these new authentic failures.

The existing constructionAudit neutral model has oneIFD, oneLong tag, fouru32 values, nine domain rows, empty schema/pixels. Independent BunSQLite counts the entity/value relations and exact scalar payload. New closed fields establish a16-byte minimum typed u32-vector backing and1-byte refusal. The native law retains the exact semantic payload ceiling while requiring actual from_sqlite_database to refuse insufficient construction backing. Maps, borrowed reference vectors, IFD/tag vectors and twelve typed collection constructors remain unconverted. Concrete unmounted admission helpers are retained in ticket 🧮️tiff-relational-backing-draft.rs; integrating them requires replacing unobservable BTree node allocations with explicit paid row/index vectors and preserving ordinal/ownership checks. This draft is not a complete provider or runtime evidence.

Registered Source latest neutral fixture run before final backing-field addition passed13/13,1,683 assertions,25.3s Nx in tiff-allocation-neutral-source-current.log; the complete current fixture rerun also passed13/13,1,683 assertions,13.8s Nx in tiff-construction-backing-neutral-source-current.log.

## Owning Native19 Authentic Failures and Authorized Allocation Repair

Root executed19/19:17passed,2failed,101outside,114ms,Nextest94468a0e-ef11-436c-9127-a1a1629b1319,Nx23.6s. The prior three sorting/normalized-row audit laws now passed. New native producer transfer failed positive used-backing observation; relational construction incorrectly admitted the1-byte frontier. Log root-authentic-tiff18-owned-stable-sort-allocation-authentic-red.log.

After authorization real TIFF decode/encode now use the common allocation_stage, supplying only remaining backing and settling actual child ownership even on error/cancellation. A separate explicit literal SQL cell census preserves exact semantic max_value_bytes, including typed numeric/value-class columns and all intrinsic pixels; no backing bytes are folded into that field.

Relational reconstruction replaces BTreeSet/BTreeMap node ownership and uncontrolled collect with typed paid row indexes, controlled in-place heapsort, contiguous owner/ordinal validation, and before-allocation vectors for directories, tags and every twelve value variant. Schema/pixel/ASCII ownership copies admit their own backing. Index storage uses concrete Rust size_of tuple slots; no hidden node-size approximation. All allocated index frontiers remain cumulative even when their temporary scope ends. The adjacent 🧮️construction owner module and native input/output/provider parsed successfully; fresh Native19 verification remains Root-owned and unexecuted at this report update. Shared SQL schema-validator/physical engine internal allocations are separate unresolved core coverage, not erased by this owner repair.
