# EXPRESS Population Traversal Independent Audit

Before source SHA256 for validation/population.ts is `4ce24941bd34541c19ece4d4a482879a79ef59b77a1df2dd7fe3cd9d8de928dd`, recorded in `generated/population-traversal-before-hash.json`. This report describes that immutable checkpoint; subsequent root edits must be reviewed separately. No production files were edited by this audit.

## Model-dependent cold work

At before line75 each global population executes `[...context.entities.values()].filter(...)` before its next evaluator operation. The released issued program has two globals, each one population (IFCGEOMETRICREPRESENTATIONCONTEXT and IFCPROJECT). The number of cohorts is fixed, but the array birth, scan and selected result size depend on the caller model. Fixed catalog/program size does not bound this cold model-sized turn.

Before line48 inverse attribute resolution ticks each incoming edge, filters exact role plus source type ancestry, and deduplicates with a growing `matching.includes(edge.entity)`. Each tick can therefore perform a linear search of prior matches: distinct matching edges yield quadratic comparison work even though edge iteration is cooperative. Before line72 scalar inverse cardinality calls that attribute resolver and then repeats a synchronous incoming filter/map/Set materialization to count all distinct matches. Scalar selection of the first entity must not imply cardinality1; the recount is semantically necessary until its actual matching result is reused.

The static inverse declaration find, source entity ancestry checks and compiler rule rosters are bounded by the released catalog/program, distinct from model-dependent incoming edges and cohort output. Existing context initialization converts/indexes actual values under evaluator ticks; a private ancestry cohort index built there can avoid repeated whole-map scans. A progressive projection must still tick each selected member and completion rather than expose a live mutable index Set to global scope.

## Genuine before evidence

`population-traversal-before.ts` compiles the actual complete777-rule program, constructs declaration-owned snapshots, uses the actual STEP writer and parser, then initializes the real IfcExpressContext. No fabricated issued program, cloned authority or substituted context is used. `generated/population-traversal-before.json` records material-layer inverse0/1/2 incoming owner populations and actual inherited/exact/nonmatching cohorts. Physical input files are separately generated.

Zero incoming produces unknown scalar inverse; one selects10; two selects first10 while incoming IDs remain10,11. This is existing scalar selection behavior, not a valid two-owner inverse claim. The genuine compiler roster contains seven scalar inverse declarations. Work observations24/35/46 describe these small actual before contexts and are not minimum-grant or CPU bounds.

`population-traversal-oracle.py` uses a test-tool-independent interface over existing IfcOpenShell0.9.0.post1. Exact original headers are IFC4X3_ADD2. It independently finds inverse IDs[],[10],[10,11] and the same cohort IDs for all three actual STEP inputs. Layer and layer-set both belong to IfcMaterialDefinition; unrelated CartesianPoint9 and empty IfcWall cohort remain distinct. External by_type ordering is normalized only for membership comparison; source insertion order is retained independently in the TS observations.

These inputs establish physical parsing/object identity, not full EXPRESS validity. The zero/two scalar inverse populations intentionally violate the singular cardinality domain; the minimal optional fields also do not establish complete model validity. No native execution is claimed. No wall-time/per-tick fairness failure was manufactured; the uncheckpointed scans and quadratic search are direct source findings.

## Concise neutral next laws

- Population exact type, inherited type, nonmatch and empty cohort; mixed source order retained, each source entity once even if ancestry repeats; lowercase query names follow the agreed normal identifier contract.
- Private initialized-index population projection produces a SET owned by the consumer; mutating returned aggregate storage cannot mutate later projections. Noninitialized context behavior must be explicit rather than publish a partial cohort.
- Scalar inverse zero/one/two distinct owners: unknown/first/first selection remains, cardinality0/1/2 independently preserved. Lower/upper singular constraints must reject0 and2.
- Duplicate edges from the same owner produce one inverse owner; a different attribute or unrelated source type must not match. USEDIN BAG multiplicity remains a separate contract and must not be changed accidentally.
- Original small work denial, initial/interior/completion cancellation, exact callback throws including undefined, and observer/signal replacement cannot skip index or projection work. Denied or cancelled projection cannot return a completed partial cohort.

All before Bun and independent Python probes have exited. Root was notified that frozen population.ts could be edited after before hashes/projections were captured. Inputs and before outputs remain preserved for a later independent after comparison.

## Test-first traversal contract review

Read only new traversal.test.ts/cases/schema while population.ts was actively edited; production was not read or mapped. The neutral zero/one/two MaterialLayer inputs match the independently emitted/parsed before graphs and IfcOpenShell inverse IDs. No confirmed test defect was found. Inherited membership, distinct unrelated types, unknown/empty cohort, source-order `[30,20]` and mixed-case IfcRoot `[7,1]` appropriately distinguish source order from numeric sorting. Scalar tests separately inspect full matched IDs, scalar first/unknown output and actual issued-program cardinality results; they do not infer count from scalar output.

The projection alias test mutates only the returned array and reprojects to detect leaked index storage. Work denial derives the actual initialized baseline before applying a four-step remaining allowance. Cancellation/undefined/opaque-object callback refusal and genuine host-timer cancellation inspect the actual initialized context, avoiding mock programs or synthetic host continuation. Empty projection cancellation ensures absence is not an unchecked success path. Full global tests use actual genuine compiler and explicitly retain no physical-validity claim for minimal Project inputs.

Coverage recommendations remain duplicate matching edges from one owner, role/type filtering, inverse projection initial/interior/completion controls and returned inverse-array index independence. These are additional test laws, not confirmed defects in the reviewed tests. A complete singleton inverse cardinality assertion can inspect both lower/upper results when useful; the existing `some` predicate detects the intended false bound in zero/two cases. New API behavior before initialize should be explicitly refused or otherwise specified. Await frozen source and after probes before declaring this refactor closed.

### Supplemental invalid-edge external observation

The separate `population-inverse-edges-oracle.py` physically parses a duplicated MaterialLayers owner10 and a wrong-domain MaterialProfiles owner11. IfcOpenShell returns raw scalar inverse occurrences `[10,10]` and generic incoming owner IDs `[10,11]`. The initial probe expected deduplication and failed its assertion after writing the raw observation; that output is preserved. The corrected observed-output run records the same actual result and exits successfully. This intentionally invalid UNIQUE-list/wrong-domain model does **not** independently validate deduplication as normative cardinality. The existing TS matching code deduplicates same owners, and preservation of that behavior is a separate internal contract; wrong-role filtering excludes owner11. Do not turn this external invalid-input discrepancy into a production semantics change without authoritative inverse-domain evidence.

## Frozen traversal implementation closure

Fresh actual after input `population-traversal-after.ts` uses the same genuine777-rule compiler and writer/parser source construction. Distinct `generated/population-traversal-after.json` retains exactly the same inherited/exact/empty/nonmatch cohort IDs and scalar0/1/2 projections as before and the independent IfcOpenShell oracle. Additional indexed/projection work is intentionally charged: observed total work38/53/68 replaces before24/35/46; this is not a minimum budget guarantee. Before files remain unchanged (an initial exclusive write correctly refused an old physical filename before the ticket script adopted distinct after filenames).

`population-controls-independent.ts` separately executes actual population and inverse projections, with original cancellation, thrown undefined and exact opaque object at the second original callback. All six reject exactly; no partial output returns. Selected entity values are the genuine initialized context identities. Clearing a returned population array does not change the next `[1,10,11]` projection. Output is `generated/population-controls-independent-after.json`; after source hash is `generated/population-traversal-after-hash.json`.

Source review confirms private cohort index construction during actual initialization, per-ancestry-member tick, private single-use readiness state, final initialized tick, and failed initialization refusal. Population copying is source-ordered and ticked per selected value, with initial tick covering empty results. Global scopes call that operation. Shared inverseValues ticks each incoming edge, filters exact role/source type and deduplicates through Set. Both attribute selection and cardinality use those matches; the synchronous scalar filter/map/Set recount and global spread/filter are removed. Fixed catalog ancestry and declaration search remain distinguished from model-dependent work.

No new confirmed P1/P2 in this bounded source/actual after closure. Existing source/value custody and complete EXPRESS semantics remain separate boundaries, as do native execution and physical accounting. Invalid duplicate inverse observations remain documented rather than treated as normative external proof. No production files were edited. All probes exited normally, with no live Bun mappings.
