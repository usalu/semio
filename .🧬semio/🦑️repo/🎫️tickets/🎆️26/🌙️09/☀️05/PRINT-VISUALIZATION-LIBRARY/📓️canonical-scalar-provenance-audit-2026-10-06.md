# Canonical Scalar Provenance Admission Audit

Actual Bun 1.3.14 + Nx exec session 26814 exited 0. Strict neutral fixture was retained before execution as `📥️authored-inputs/canonical-scalar-provenance-fixture-2026-10-06.json`; its 12 cases were independently validated by AJV. Every case ran public changeVizChartValue on preset data, guarded applyVizChartDiff, and public inferVizChart. Every replay had no mutation/replay messages, complete inference and independent ChartSpecification AJV admission. Exact emitted sources and diagnostics are retained in `📥️authored-inputs/canonical-scalar-provenance-observation-2026-10-06.json`.

Canonical serialization preserves provenance correctly:

| Authored row scalar | Emitted canonical carrier |
|---|---|
| string null | SemioVizString{null} |
| string undefined | SemioVizString{undefined} |
| string true | SemioVizString{true} |
| actual null | SemioVizNull{} |
| absent row property | SemioVizUndefined{} |
| actual boolean true | SemioVizBoolean{true} |

The chart table schema admits string/number/boolean/null cells and does not require every declared column property in every row. Missing properties therefore reach undefined carriers even though JSON has no undefined scalar. This is proven through public mutation/replay/inference, not a manually invented native carrier. Literal null root/parent IDs and actual null ID/parent/duration all remain physically distinct in emitted rows. No string-null conflation is established at this boundary.

Current read source owner is catalogue rowScalar, `🧬️schema/💡️inferences/📚️catalogue/🟦️.ts` line 17. Current native data owner defines table_scalar:N at line 18, which decodes exactly one SemioVizString payload without trimming. The requested table_scalar:nN/genericScalarString names were not present in this exact source observation; the owning functionality was audited directly instead. Source hashes are retained in `📥️authored-inputs/canonical-scalar-provenance-source-capsule-2026-10-06.json`.

Native policy candidates remain distinctly bounded:

- GraphStore semio_viz_network_id_require:N decodes one string, then explicitly rejects empty, SemioVizNull and SemioVizUndefined identities. This preserves literal string null/undefined while rejecting absent carriers. No new native execution was performed here; the existing graph-reference gate is parent-owned.
- Domain tree_row decodes the ID and checks blank/duplicate, without the same explicit Null/Undefined carrier checks in the inspected definition. It likewise decodes parent strings but retains actual null/undefined parent carriers; tree_depth treats only empty parent tokens as roots. These are reachable carrier/policy differences warranting existing native intake tests, not established native failures from this source audit alone.
- Domain tree_value validates a literal finite nonnegative numeric string after string decoding. Null/Undefined/Boolean carrier spellings fail that regex rather than becoming numeric null/boolean coercions; native execution remains required to prove the actual rejection receipt. Literal string null duration remains distinguishable from actual null duration until that role gate.

Canonical inference completing for these cases does not prove native rendering accepts them: the schema is generic and native family role gates are separate. Accordingly this audit claims no product RED merely from complete=true or the source candidates. A minimal genuine next regression should use these exact canonical sources in the existing native critical/forest helper, retain actual rejection diagnostics for null/missing IDs, and verify literal string null/undefined nodes with string-null parent edges preserve depth/timing. Parent null/missing semantics need an explicit existing contract: decide whether they mean an absent parent/root or invalid typed parent, without conflating them with authored strings. Do not weaken rejection contracts to match an invented D3 coercion.

Graphlib commonly stringifies node IDs and D3 stratify has its own missing-ID/root policy, so either library alone is not a strict typed-identity admission oracle for this product. AJV already independently proves the generic public table admission. A role-specific schema must state the desired identity/parent policy before using Graphlib/D3 only for admitted forest depth/timing; this audit deliberately avoids inventing such a policy.

Generated console is `🗑️generated/canonical-scalar-provenance.log`. No product files, helper modules or native compiler outputs were changed.
