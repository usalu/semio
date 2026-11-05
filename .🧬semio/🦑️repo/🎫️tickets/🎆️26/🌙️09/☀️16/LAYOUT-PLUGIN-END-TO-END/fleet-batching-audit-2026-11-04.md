# Independent Cargo Resolver Batching Audit — 2026-11-04

Read-only audit of current custody helper, caller, schema and neutral batch tests. No builds/tests/source edits.

No actionable regression established. The actual failure was an oversized single owned input, so retaining the65,536byte ingress and partitioning the ordered edge list is a legitimate transport fix. No decoder, Source cap or input limit was weakened.

The generator initializes bytes to2 for JSON brackets, counts Buffer UTF8 bytes of each JSON-encoded edge and one comma only after the first member. It yields the existing complete batch before inserting the current edge when the next edge would exceed bytes or128members. It then resets bracket accounting and inserts that same held edge exactly once. Final partial batch is emitted; an empty edge list emits no child input. A singleton whose encoded member plus brackets exceeds65,536 refuses rather than splitting an edge or raising a limit. Input order and duplicate occurrences are preserved.

The caller consumes the generator lazily, checks the same issued lease and abort signal before each child, decodes the exact submitted batch, requires an output array of equal length, and compares every selected result by index. The accumulated resolved count must equal original edge count. Empty resolution input retains the subsequent original input/output byte checks. This does not manufacture an empty authority subprocess.

Every batch calls the existing controlled capture helper with the same snapshotted parent authority. Its maximum is still min(schema child maximum, actual child budget, same phase remaining time), and withRequestedIssuedResourceControlV1 refines that existing phase. Same Original capabilities, phase environment and cancellation lineage remain. Existing before/after progress operations and post-await cancellation checks run per child; no fresh top-level deadline or per-batch renewal is introduced.

The6 neutral cases cover empty, ASCII, multibyte UTF8, exact byte boundary, overlong singleton and259ordered edges crossing128members. Tests flatten decoded batches and compare complete original arrays, compare YAML roundtrip output, compare Buffer bytes against independent TextEncoder, validate case schemas with Ajv, and invoke unchanged readOwnedCommandInputV1 for every generated input. These source checks support the reported6case/49assertion result but no execution was independently launched in this audit.

No missing, reordered or duplicated edge mechanism found. Actual native owner completion remains separate evidence; progress and budget preservation are source-reviewed here.
