# Process3d Independent Capability Wiring Audit

## Current Readback

No Cargo, runtime, Git or source edits were performed. Fresh scans still find no Process3d ArtifactSqliteSnapshot implementation or sqlite_snapshot_codec override. The existing Native SQL file is present but **unmounted**: Snapshot declares only test modules, and no production path declaration includes its SQLite file. Therefore complete Source projection/reconstruction is mounted, while the Native 32-table producer remains an authored candidate. No concurrent fix was visible during this audit.

Let `snapshot` denote `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot` below.

- `snapshot/🦀️.rs:24` owns the actual nine-field Process3dSnapshot; ArtifactDsl starts at589, ordinary ArtifactPack at609. The latter supplies ordinary packing, decoding and record_spec only. Native SQLite tests are mounted at659.
- `snapshot/🪶️sqlite/🦀️.rs:9` owns SQL include; forecast86, project201, reconstruct427. This is the existing handwritten model to repair and mount, not a reason to create another DTO.
- Artifact identity is `s.process.process3d@1/*`, declared in artifact root `🦀️.rs:51`; document registration consumes the same schema at1105 and document_codec at1108.
- No process2d artifact, standard or subset appeared in the repository filename scan or Process-plugin text scan. BREP and Flow are separate child capabilities, not alternate Process3d subsets. Do not invent a process2d conversion or lower the parent into either child.

## Concrete Before-Mount Defect

The Native SQL candidate owns unpaid reconstruction collections: `Rows.used` BTreeSet at261, duplicate ID BTreeSet at270/273, used insertion at301, child BTreeMap at307/314, and Vec::with_capacity result at318. Neither forecast nor check_database admits these actual index nodes/backing. Adding the capability hooks alone would leave full cumulative ownership false. Replace these with concrete admitted indexes/frontiers and controlled backing before publication; a generic byte forecast is insufficient. Existing String-returning provider APIs are terminal projections, but internal allocation refusals must retain ValueError categories until those boundaries.

## Minimum Owning Facet and Wiring

1. Mount and repair the existing handwritten SQL facet from Snapshot. Bind SQLITE_SCHEMA, to_sqlite_database and from_sqlite_database to this actual 32-table authority. Keep all actual projection and reconstruction backing paid by the same caller control, including duplicate/order/used indexes and partial typed collections.
2. Implement the actual canonical domain types' controlled schema/input/output producers. The Snapshot derives DslRecord, but handwritten enum wrappers in artifact root provide only controlled shape at174; this does not establish controlled typed conversion. Cover Workshop/Machine/Capability/Parameter/Rule, recipes, Stock/WorkingSolid, ProcessStep/Measure/Origin, Pose and literal Child fields through their actual variants.
3. Implement ArtifactSqliteSnapshot's decode_sqlite_snapshot_native, encode_sqlite_snapshot_native and preflight_sqlite_snapshot_encoding for both existing envelope Pack and Text carriers. Store trait authority is `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:10886`; strict defaults refuse these hooks. Use actual controlled record/schema and physical bridge producers, not ordinary decode/encode wrapped in before/after checks. Bind retire_sqlite_snapshot to the existing retained owner/cold retirement authority instead of default drop.
4. Override ordinary ArtifactPack::sqlite_snapshot_codec with Some(Self's ArtifactSqliteSnapshot codec). Existing bare codec and Plugin document registration then expose the same parent capability; no duplicate declaration or erased JSON carrier is needed.
5. Add actual public registry/io_route/io_run_with_snapshot_control Binary and Text physical transfer coverage if full public endpoint success is the objective. Existing erased law calls capability export/import directly and cannot prove those public routing stages.

## Schema Fidelity Obligations

Keep nine independent root fields, root stock presentation independent of stock payload identity/pose/solid; workshop machines/capabilities/parameters/rules; six recipes, six solid variants, three measures, five quantities; ordered step payloads and optional origins; three literal child-slot families. Preserve empty/repeated/unresolved/NUL/Unicode semantic IDs without SQL surrogate constraints. Positive surrogate IDs and contiguous relationship ordinals remain structural only. Every scalar retains named query REAL/null, signed exact binary64 INTEGER word and class cells; signed zero/NaN payload/infinity must reconstruct from exact words and companion validation. No serialized native, JSON, snapshot payload or normalized DTO column belongs in the 32 tables.

## Exact Existing Gates

Owning project `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/📋️project.json` defines test-snapshot-sqlite52, Native61, Source70 and consumer check79. Commands:

- `SEMIO_TEST_LEVEL=quick bun nx run @semio-tech/process-process3d-rs:test-snapshot-sqlite-native --skip-nx-cache`
- `SEMIO_TEST_LEVEL=quick bun nx run @semio-tech/process-process3d-rs:test-snapshot-sqlite-source --skip-nx-cache`
- `bun nx run @semio-tech/process-process3d-rs:check-snapshot-sqlite-source --skip-nx-cache`

Native selector `sqlite_snapshot_process3d_` has eight authored laws: actual parent capability135, complete native formats139, independent child literals149, controlled construction/schema158, erased Binary/Text174, independent SQLite200, Plugin document212, interior controls218. These are staged source facts only; this audit executed none. Neutral fixture is `snapshot/🧫️fixtures/🪶️sqlite/🔣️.json`, SQL `snapshot/🪶️sqlite/🗄️.sql`, Source laws `snapshot/🧪️tests/🪶️sqlite/🟦️.ts`. Existing report's Source16/161 receipt is prior evidence, not a fresh run here.
