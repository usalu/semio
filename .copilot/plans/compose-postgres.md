## Plan: Fully Normalized Kit VCS Schema

Add full kit version-control persistence to the shared PostgreSQL schema by mirroring the Rust KitStore model for checkpoints, alternatives, sessions, drafts, transactions, and releases, while also normalizing the metabolism kit snapshot shape into relational snapshot tables. Recommended implementation: keep command and change streams as raw JSONB where the payload is polymorphic, but fully normalize durable kit snapshots and snapshot-derived entities.

**Steps**

1. Phase 1: Confirm domain invariants from the existing Rust source before editing SQL. Reuse KitCheckpoint, MaterializedKit, KitAlternative, Draft, Transaction, Session, KitStore, and KitFullDto as the canonical model. Preserve these invariants in the schema: drafts may only target the latest main checkpoint or latest alternative tip; alternatives are linear ordered checkpoint lists that may share checkpoints; releases are checkpoints with an attached materialized kit; sessions are stateful and own drafts; transactions support undo and redo inside drafts.
2. Phase 1: Define the durable root tables for version control in c:\git\compose\repo\postgres\schema.sql. Add tables for kits, kit_checkpoints, kit_checkpoint_authors, kit_alternatives, kit_alternative_checkpoints, kit_sessions, kit_drafts, kit_transactions, kit_transaction_changes, and kit_releases or equivalent release columns on checkpoints. Use text ids throughout to match the existing schema and Rust ids. Add explicit foreign keys and unique constraints for ordered alternative membership, one open transaction per draft, and one release record per released checkpoint.
3. Phase 1: Model checkpoint and transaction payloads intentionally. Store checkpoint changes and transaction changes as JSONB command or change payloads rather than trying to relationally explode the full command algebra. This keeps the schema aligned with Rust KitChange and ChangeKitCommand while still allowing relational querying of lineage, ownership, timestamps, authors, and release markers.
4. Phase 2: Add normalized snapshot storage for initial kits and materialized kits in c:\git\compose\repo\postgres\schema.sql. Introduce a kit_snapshots header table keyed by snapshot id and snapshot kind, with links back to the owning kit and optional source checkpoint. Reuse that snapshot id as the partition key for normalized entity tables rather than duplicating separate table families for initial versus materialized data.
5. Phase 2: Normalize the metabolism JSON top-level kit graph under the snapshot boundary. Add relational tables for the fields represented in KitFullDto and the metabolism fixture: snapshot-level kit metadata, types, designs, files, folders, authors, concepts, tags, qualities, props, and attributes. Preserve original external ids from the JSON fixture. Add ordered child tables for nested collections that need stable identity or ordering.
6. Phase 2: Normalize the nested entities that make the metabolism fixture materially queryable. At minimum include type families, type connectors, type representations, design pieces, design connections, design props, design layers, and file or folder metadata. Represent flexible substructures that do not yet have stable domain contracts as JSONB columns hanging off normalized parent rows instead of blocking the rollout on exhaustive decomposition.
7. Phase 2: Decide and document the minimum normalization boundary for ambiguous nested payloads. Recommended boundary: normalize entities and join relationships that are addressed by id in the DTOs, and keep opaque geometry or polymorphic representation payloads in JSONB columns on normalized rows until their cross-language schema is stabilized.
8. Phase 3: Add indexes and integrity rules needed by the VCS flows. Include indexes for checkpoint parent traversal, main-head lookup, session-by-kit lookup, draft-by-session lookup, open transaction lookup, alternative ordered membership, and snapshot entity foreign keys. Add check constraints where possible for transaction state and snapshot kind. Prefer partial indexes for open-session and open-transaction lookups if PostgreSQL semantics are helpful.
9. Phase 3: Extend supporting documentation so the schema is self-describing. Update c:\git\compose\repo\postgres\README.md to explain the new table groups, the normalized snapshot boundary, and why changes remain JSONB while snapshots are relational. If there are schema comments or specs nearby, update those in the same pass.
10. Phase 4: Add verification coverage in existing test locations rather than new test files. Reuse c:\git\compose\compose\rs\tests\metabolism_kit.rs as the authoritative fixture proof that the metabolism kit deserializes to KitFullDto. Extend an existing test file in the repo package, most likely c:\git\compose\repo\cli\main_test.go or another existing repo test location that already validates schema-oriented behavior, to assert the emitted or embedded schema contains the new tables and critical constraints. If there is no existing Postgres schema test harness, add string-level schema assertions in an existing repo test file rather than inventing a new dedicated test file.
11. Phase 4: Validate the end-to-end mapping manually after implementation. Materialize a metabolism kit into the normalized snapshot tables, create at least one session, draft, transaction, checkpoint, alternative, and release, and confirm the stored graph can reconstruct the same logical state the Rust KitStore expects.

**Relevant files**

- c:\git\compose\repo\postgres\schema.sql — primary schema file to extend with all VCS tables, snapshot tables, indexes, and constraints.
- c:\git\compose\repo\postgres\README.md — document the new normalized kit persistence model and intended usage.
- c:\git\compose\compose\rs\lib.rs — canonical source for KitCheckpoint, MaterializedKit, KitAlternative, Draft, Transaction, Session, KitStore, finalize_draft logic, and draft-base invariants.
- c:\git\compose\compose\rs\tests\metabolism_kit.rs — fixture-backed proof that metabolism.kit.compose.json maps to KitFullDto and hydrates KitStore.
- c:\git\compose\compose\assets\compose\metabolism.kit.compose.json — target fixture whose top-level and nested shape the normalized snapshot schema must cover.
- c:\git\compose\compose\js\index.ts — client-side SessionKitStore and UndoableKitStore contracts that rely on persistent sessions, snapshots, and history.
- c:\git\compose\repo\cli\main_test.go — likely existing test file to extend for schema assertions if no dedicated Postgres schema tests already exist.

**Verification**

1. Run the existing Rust metabolism fixture test path to confirm the DTO shape used for normalization remains valid.
2. Run the repo test target that covers the extended existing test file and assert the schema text includes all new kit VCS and snapshot tables plus the most important unique constraints and indexes.
3. Validate a representative lifecycle manually against the implemented schema: insert initial kit snapshot, open session, create draft, append transactions, finalize to checkpoint, branch an alternative, mark a release, and materialize the release snapshot.
4. Compare the normalized snapshot coverage against the metabolism fixture fields and confirm there are no silently dropped top-level collections.

**Decisions**

- Chosen direction: fully normalized storage for durable kit snapshots that match the metabolism fixture, not JSONB-only snapshot persistence.
- Recommended compromise: keep raw command and change payloads in JSONB because they are an action log, not the primary query surface.
- Included scope: PostgreSQL schema design, integrity rules, documentation, and tests for the new VCS and normalized snapshot model.
- Excluded scope: implementing the actual async backbone service logic, runtime migration tooling, or server endpoints unless existing work in the same task already requires schema consumers to change.
- Constraint: there is no separate compose/postgres package in the workspace; the shared target is the schema under c:\git\compose\repo\postgres.

**Further Considerations**

1. Snapshot granularity: prefer one reusable snapshot table family keyed by snapshot id over duplicating separate initial-kit and materialized-kit entity tables.
2. Geometry and representation payloads: if some nested metabolism fields are too polymorphic to normalize cleanly now, store them as JSONB on normalized parent rows and document that boundary explicitly.
3. If execution later reveals an existing repo-side SQL generation or embedding path for PostgreSQL similar to the SQLite export path, update that path in the same implementation ticket so the schema does not diverge from runtime consumers.
