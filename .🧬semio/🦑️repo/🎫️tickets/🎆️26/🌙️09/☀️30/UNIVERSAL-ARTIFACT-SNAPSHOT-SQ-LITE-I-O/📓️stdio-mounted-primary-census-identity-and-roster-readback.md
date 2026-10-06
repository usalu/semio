# Mounted Primary Census Identity and Roster Readback

Read-only source audit; no tests executed. The current shipped-fleet SQLite corpus contains 36 unique kind rows and 89 unique coordinate/schema rows. A direct comparison against the retained authored literal document-codec roster has no differences in either direction, including GeoJSON. This comparison is a source/corpus consistency check, not a runtime enumeration receipt.

Current owner law at shipped-fleet Rust lines 341–381 compares actual selected definitions and assemblies against exact expected kinds and checks cardinality to reject duplicate identities. It preserves the Definition-only enum channel. Runtime declarations require nonempty hosted kinds; every hosted schema must resolve through the actual registry to a codec with the same schema, concrete Snapshot TypeId and nonempty SQL.

The binding law at lines 385–423 obtains actual declaration bindings independently of installed IO entries or editor membership. It compares the complete coordinate/schema set and total cardinality against the 89-row fixture. For each declared provider it checks concrete TypeId, registered codec `identical_to` (SQL/TypeId/export/import hooks), successful actual native SQL lookup with exact SQL equality, and exact-fidelity routes in both directions. Preflight is supplementary conflict evidence and no longer substitutes for actual registration lookup.

The binding law relies on the companion owner law for the nonempty SQL requirement. Both must be selected together when reporting the complete census. Route lookup is registration/path evidence; it does not execute payload conversion. The route depth 1 requires a direct native/SQLite route. Native provider preflight compares actual provider identity but does not independently compare the codec schema string; the declared coordinate/schema census and registry lookup key provide that separate association.

The selected full catalog currently has Runtime assemblies in the independently read source roster. Preserving Definition-only handling is appropriate; accepting such assemblies does not prove a payload owner where none is declared. This gate is scoped to the actual selected Stdio composition, not a universal repo owner denominator. Its DEBUG rows and actual selected Native assertions remain required for execution credit.

## Fresh Six-Law Payload Extension

The four additional owning laws inspect Binary/Txt public declaration trees and execute ordinary public payload routes for Binary, Txt and the non-editor GeoJSON binding. The shared helper independently queries every domain table, exact metadata/integrity/FK, preserves complete Text/Pack wire, and checks complete typed Snapshot equality. GeoJSON additionally uses an independent Serde parse, actual conformance validator and public route rejection of valid JSON outside GeoJSON. This is materially stronger than table presence alone.

Two bounded source findings sent to Physical: the helper borrows bytes from &file, moves file into import io_run, then uses bytes.len in DEBUG, predicting a borrow-across-move compiler prerequisite. Retaining byte count before the move preserves assertions. Also the public declaration-tree loop admits empty standards/subsets vacuously; pinning the actual Binary/Txt coordinate membership avoids that independent-tree gap even though the separate Runtime roster law retains its own denominator.

These are source findings before the actual six-law receipt, not runtime failures or provider defects. SQL helper ORDER BY id is suitable for the three specifically authored witnesses; it is not an assumption generalized to every future domain table.
