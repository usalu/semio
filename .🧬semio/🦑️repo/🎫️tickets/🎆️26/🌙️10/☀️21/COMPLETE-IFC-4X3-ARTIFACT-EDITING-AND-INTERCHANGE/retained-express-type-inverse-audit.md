# Targeted Retained EXPRESS Type And Inverse Audit

## Decision And Evidence Boundary

Keep semantic publication refused. This targeted source review confirms an aggregate operand/result classification defect with independently executed reference counterexamples. It also identifies a concrete missing TYPEOF SELECT-closure branch, whose full normative expectation remains distinguished from the available unauthenticated translated clause. No native output/type-check/runtime/receipt proof exists. The implementation worker has received exact counterexamples and is preparing separate fixes; this report describes source at discovery, not post-fix approval.

Reviewed native machine-population.rs, machine-operation.rs, machine-domain.rs and machine-rules.rs type/inverse branches; actual pinned IFC4X3_ADD2.exp uses; and TS evaluator.ts/population.ts. Small independent ticket interfaces retained: retained-type-inverse-probe.ts and retained-type-inverse-oracle.py. Their outputs are under 🗑️generated/retained-type-inverse-probe.json and retained-type-inverse-oracle.json. The Python environment actually resolved IfcOpenShell 0.8.5; this differs from other workers' separate environments and is recorded exactly.

## P2 — Classify Aggregate Results And Deduplication From Both Actual Operands

machine-operation.rs at discovery line 524 chooses the result kind from the left aggregate (or right only when left was scalar). Lines 552-558 choose union deduplication from left_kind alone and intersection consumption from whether either input is BAG.

Concrete scalar-only counterexample: SIZEOF('INTEGER' + TYPEOF(1)). The native source promotes the left scalar to a temporary LIST, builds a SET result from the right input, but disables deduplication because left_kind is LIST. It inserts INTEGER twice and predicts 4. Executed TS reference returns 3. The independent installed IfcOpenShell express_set implementation also returns the three distinct names. This is not an uncertain-membership case and needs no unsupported source population.

Concrete mixed-kind counterexample: B is BAG ['x','x']; S is SET ['x']. TYPEOF(B*S) should expose SET, but the source chooses BAG. Consequently SIZEOF((B*S)+['x']) predicts 2 in native source while executed TS returns 1. Independent IfcOpenShell intersection returns one x. Return coerce-to-SET at a surrounding function boundary could mask the bug; tests must inspect the intermediate result or the enclosing union as above.

Fix both result-kind selection and deduplication/multiplicity rules using actual operand categories before scalar promotion. Add neutral/native cases in both operand orders, duplicate BAG/SET/scalar elements, and an observable post-intersection union. Do not merely coerce the final output to the expected type. Worker owns edits; this reviewer made none.

Pinned canonical uses include list-initializer/TYPEOF intersections in IfcDirectrixCurveSweptAreaSolid DirectrixBounded (EXP line 5737), IfcStructuralLinearAction SuitableLoadType (10857), and point/surface action counterparts. Their current SIZEOF checks may hide result-kind defects when constants are distinct. No false outcome on those specific pinned populations is claimed without native execution.

## TYPEOF SELECT Closure — Concrete Missing Branch, Normative Authentication Still Open

machine-population.rs population_operation TypeOf stage 1 (approximately 246-276 at discovery) walks entity supertypes or a defined alias's named base and then primitive names. It never adds SELECT declarations containing any discovered type.

Counterexpression: 'IFC4X3_ADD2.IFCAXIS2PLACEMENT' IN TYPEOF(IFCAXIS2PLACEMENT3D(IFCCARTESIANPOINT([0.,0.,0.]),?,?)). The native source predicts false. Pinned EXP lines 3306-3309 and the official buildingSMART normative Annex A explicitly put IfcAxis2Placement3D in IfcAxis2Placement. Independent IfcOpenShell schema select_list confirms exactly the 2D/3D roster. [Official Annex A](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/annex-a-express.html).

The translated ISO mirror's §15.25 b3 describes containing-SELECT closure; this mirror is not independently authenticated normative English text. Its §12.6.2 table 16 likewise describes SET results for mixed BAG/SET intersections. [Translated mirror](https://normadocs.ru/gost_r_iso_10303-11-2009). The available [ISO 10303-11:2004 preview](https://cdn.standards.iteh.ai/samples/38047/aebaf1a0bcc94d968632742366476f49/ISO-10303-11-2004.pdf) confirms the section/table locations but does not include those full clauses. Do not report the preview as proof of clause content.

Executed TS TYPEOF returns false for this expression too. Installed IfcOpenShell TYPEOF also omits SELECT names and only traverses supertypes/base declarations. Therefore neither implementation independently proves the SELECT-closure expectation. A schema roster comparison confirms membership, not complete TYPEOF semantics. This is a source conformance candidate requiring authenticated clause review; shared cold-reference behavior must not silently become the standard.

## Inverse And USEDIN Review

The native inverse branch scans actual received original reference nodes, restricts the referencing entity by declared inverse target type/subtype and attribute name, and deduplicates inverse results through instance identity. Ordinary USEDIN preserves duplicate uses in a BAG. Constructed values with no original incoming population links return empty. The released schema's inverse declarations inspected here use the supported SET/scalar shapes; no BAG inverse declaration counterexample was established.

USEDIN role parsing drops the schema prefix and accepts suffix owner/attribute matching. A role such as WRONG_SCHEMA.IFCRELASSOCIATES.RELATEDOBJECTS is therefore treated like the corresponding IFC4X3_ADD2 role. This is a concrete permissive input behavior; it should be explicitly admitted/refused according to an authenticated role contract. Current TS and installed IfcOpenShell helpers are permissive too, so cross-implementation agreement is not proof. The only direct canonical USEDIN call found is IfcWallStandardCase.HasMaterialLayerSetUsage (EXP line 12135), which supplies the correct fully qualified schema/owner/attribute; no new pinned-population failure is demonstrated by this malformed-role case.

## SELECT/Alias Rules, Qualified/Constructed Types And Uncertain Matching

Domain lanes traverse declared selects/aliases and apply nested aggregate item types. Type-WHERE traversal skips entity domains, selects the tagged actual domain for typed values, follows aliases and schedules rules under SELF scopes. Original source domain metadata and constructed named tags remain distinct. No additional confirmed complete/valid false-positive counterexample was established in those branches during this limited review; this is not approval of every qualified/derived/constructed case.

Matching builds a retained comparison matrix, first attempts certain identity matches, then admits unknown edges for an Unknown result; impossible match remains False. Aggregate output membership uncertainty explicitly refuses with express.aggregate-indeterminate. These refusals and bounded matching-depth/work limits are documented incompleteness, not success. Full uncertain BAG normative behavior, derived return domain/type-WHERE propagation and qualified partial-entity semantics remain open independent review obligations.

## Required Follow-Up

Review worker corrections against the exact source counterexamples, retain neutral/native tests without changing canonical arrays/order, execute reference/independent oracle checks with honest environment metadata, and keep native flags false until authentic ingress executes. Obtain authenticated normative clauses for TYPEOF and role semantics before claiming full ISO behavior. Full original-wallet receipts, controlled retirement and registered revision/history publication remain separately unproved.
