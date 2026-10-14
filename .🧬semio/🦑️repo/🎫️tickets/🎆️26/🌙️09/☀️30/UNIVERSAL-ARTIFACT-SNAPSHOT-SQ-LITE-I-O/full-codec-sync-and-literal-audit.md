# Full Codec Sync And Literal Audit

Read-only source audit, 2026-10-13. No compiler execution or production edits.

## Generic Bound Risk

Actual generic call is `🏪️store/🦀️.rs:2245`, `artifact_codec<P,Mutation>` calling P::sqlite_codec. Its existing P bound already includes RetireOwned + Sync, and Mutation similarly includes Sync. RetireOwned itself requires Send+'static (`🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:13`). Adding Self:Sync to sqlite_codec therefore introduces no missing bound in this real generic caller. ArtifactSqliteSnapshot/ArtifactPack alone are only Sized and do not imply Sync; specialized full export helper generic bounds must explicitly include Sync.

Other inspected calls are concrete Self implementations or named Snapshot providers, including actual Norm helper functions/macro injection. Repository generic-call scan found no additional unconstrained single-type-parameter caller. This is source evidence, not compiled universal Sync qualification. Reviewed snapshot models are immutable owned domain data (String, Vec, scalar, ordered entities); stdio snapshot-schema scan found no stored Cell/RefCell/Rc/UnsafeCell field. CommonMark Cell counters found elsewhere are thread-local retirement instrumentation, not Snapshot fields. Nested model auto-trait qualification still belongs to the owning compiler, not grep.

## Actual Struct Literals To Handcraft

1. `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/🦀️.rs:193`: generic host constructor; add Some specialized full export-read/import-slot pointers under Self:Sync.
2. `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:3290`: actual guest literal, owned SQL schema, snapshot_type None, guest raw callbacks; both new full host pointers must None.
3. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/⚠️refusal/🦀️.rs:58`: refusal fixture replaces a typed provider's raw pointers while retaining original schema and TypeId. Add explicit None fields to preserve its intentionally refusal-only raw capability. Presence mismatch with original full host provider is then a real identity difference, not accidental pointer inheritance.

Other grep matches are return-type function bodies (`fn codec()->ArtifactSqliteSnapshotCodec{...}`), struct definition or impl, not literals. Host component SQLite test codec delegates generic artifact_codec; no literal requiring fields there.

## Identity Rules

Compare optional full pointer presence and addresses explicitly in identical_to, along with existing schema/TypeId/raw pointer identity. Guest validator must reject any full pointer Some with snapshot_type None. A typed provider may intentionally expose None full pointers (refusal fixture); native identity can still be valid while complete-snapshot capability is unavailable. Public full APIs should issue exact UnsupportedOwner before effect for missing direction, and exact type mismatch for wrong borrowed source/caller Option slot. Do not weaken get<P>, invent a readable wrapper or assign guest host TypeId.
