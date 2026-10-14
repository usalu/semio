# Semio Text Paid Receiving Map

Read-only actual current owner, 2026-10-13. No builds/edits/runtime claims.

Base `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text`. Current schema model has exactly Snapshot{schema,runs}, Run{language,content,marks}, Mark{kind,href}, enum kind Bold/Italic/Code/Link. No paragraph/style/line tree exists in this owner. Empty language inherits context but is preserved exact; nonlink href is a String and must not be dropped based on prose unless schema forbids it.

## Retirement Already Exists

Subset root `🦀️.rs:190–193` installs actual artifact_retire_leaf!(MarkKind) and artifact_retire_struct! for Mark, Run and Snapshot with every field. Lack of derive on struct is not missing RetireOwned. Do not add duplicate implementations. Actual value retirement macro builds deferred/sequence cursors, with declared sequence_birth_bytes; original close must fund those real scaffolding births. A new receiving Frame can derive RetireOwned and reuse actual model retirement; no new domain retirement model is necessary.

## Exact Current SQL And Hooks

`🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql` has three complete relational tables: document id/schema; run id/document_id/ordinal/language/content; mark id/run_id/ordinal/kind/href. INTEGER PRIMARY KEYs permit signed ids; ordinals nonnegative and kind has four-tag CHECK. Current cold helper `visit_rows` writes all fields; reconstruction/reconstruction billing helpers restore ordered groups. Lower trait `pack::snapshot::ArtifactSqliteSnapshot` owns semantic SQL/P/R, Host `store::os_store::serialization::ArtifactSqliteSnapshotHost` owns receiving/native/subset hooks.

Current root has no four explicit receiving overrides; shared Host defaults forward cold P/R allocation but do not retain arbitrary local scratch. Cold subset success borrows dialect/document identity/schema only and checkpoints. Native decode still uses native_decoding helper receiving underlying control, distinct from four semantic receiving hooks. This plan concerns complete SQL P/R/validation, not raw codec replacement.

## Minimum Paid Body

Add original Frame with paid id/order vectors for document/run/mark, Projection database, candidate Snapshot, and result. Borrowed schema validator validates three declarations; shape/id uniqueness, actual parent ownership and per-parent contiguous ordinals before reconstruction. Snapshot schema and language/content/href text copy into retained fields through Port; runs/marks vectors preallocated and header moves admitted before push; all four enum variants mapped explicitly. Signed surrogate renumbering, duplicate semantic contents/marks and empty groups remain legal.

Projection borrows complete source, checks aggregate counts/limits before births, creates paid table metadata/row/cell vectors and text directly. Reconstruction retains current Run/Mark inside candidate before first text copy. Both directional validators can share retained indexes and complete borrowed source/SQL equality with 256-byte text work/callbacks, avoiding comparison candidate. Settle inside binder before output publication; drop RefCell/Port before post-body close observer. Release0 must retain full original prefixes and all indexes until separately funded original close.

## Existing Law Mount And Selection

Existing `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` has prefix `sqlite_snapshot_semio_text_`, including native_fields_and_ordinals_roundtrip and rejects_invalid_owners_order_kinds_and_budgets. Existing Source sibling `🟦️.ts` and fixture subtree support independent SQLite rows. Actual own Cargo package `📦️packages/🦀️rust/Cargo.toml` name semio-s-artifact-stdio-semio-text. Use its real owning Native target/selector rather than only parent aggregate Semio tests. This audit did not identify a distinct existing GUI/seed row naming this package/prefix; new executable Source/Native rows must be registered in both GUI/seed through canonical script before running. No inferred command or runtime status.

Schema-first fixture: all tags, arbitrary schema, empty language/content/href, empty runs/marks, true NUL/Unicode >256bytes, signed ids, per-parent order, malformed duplicate/orphan/gap/kind/arity, exact rows/metadata. Independent Bun SQLite implements edits and field witnesses; current original body/five-axis/SQL-native cancellation/zero+funded cleanup laws use finite authored policies. No duplicate producer found at current inspected root; preserve concurrent work and recheck before implementation.
