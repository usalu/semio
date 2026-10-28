# IFC Retained Reduction Custody Corrections

This follow-up addresses the two actionable findings in `host-derived-reaudit.md`. Source changes and regression tests are complete; native type checking, cancellation execution and heap receipts remain blocked at authentic compiler ingress. The broader retained physical accounting work remains open.

## Ownership At Fallible Frontiers

`retype_arguments` installs the displaced argument vector as an `Aggregate` in the worker's existing garbage owner before calling a fallible tick. Reused slots move from that custodied vector into the copied entity. Cancellation can no longer unwind a local vector containing the old large argument payloads.

`owned_entity_edit` receives the worker's pending mutation vector by mutable borrow and returns only success or failure. The worker installs that vector before native reduction begins. A value moved into a sparse argument or complex mutation therefore remains in worker custody when any later tick fails. Argument reduction likewise appends directly to the custodied vector. Existing partial entity, value, source and garbage fields remain part of controlled close.

The source-owned reducer no longer calls the fallible synchronous `entity_changes` / complex normalization fallback. Raw whole-entity replacement now installs the decoded replacement in the retained entity field, moves displaced fields into garbage before subsequent checks, and appends sparse changes directly to pending mutations. It preserves the existing entity identity. Constituent payloads are installed in the retained entity before controlled GlobalId population and contextual normalization. This preserves raw entity and constituent editing while avoiding the identified large local-owner fallback shape.

Direct synchronous reference helpers retain their separate cold execution behavior and local temporary owners. They are not a retained cancellation implementation. Newly supplied DSL-to-native payload decoding, shallow Vec storage moves and final mutation allocation remain synchronous and have not been heap-receipt proven.

## Empty GlobalId

Root compact retyping marks a missing or empty GlobalId slot changed before identity population. The generated value is therefore included in sparse mutations even when an empty existing string stayed at the same declaration index. A neutral fixture and native regression compare direct and retained results, require a 22-character generated GlobalId, and restore the original empty string through inverse mutations.

## Regression Tests And Independent Oracle

`retained.json` now includes a cancellation custody case that changes `IfcWall` to `IfcOrganization`, retaining a 1 MB `Description`. IfcOpenShell independently confirms that this attribute has the same declared domain and moves from index 3 to index 2, ensuring the native fixture actually transfers a large retained value into a sparse mutation.

The written IFC4x3 regression uses the actual `IfcDetailsWork` with `OriginalFixtureCaller` and a captured source snapshot. Test-only control injects cancellation on the original token at every reduction tick. After each refused prefix it requires the 1 MB copied payload to remain in the worker's entity, value, displaced-value or pending-mutation owners, then drains controlled close using the worker's exact declared copy, capacity, release and depth demands. It checks that no emission occurs. The baseline requires the large `Description` to appear in a sparse argument mutation, so late cancellation cases cover the partial mutation custody defect as well as displaced argument custody. The earlier cancellation-during-clone test remains present.

These are written tests, not executed receipts. In particular, the custody assertion inspects retained fields; it is not a heap accounting measurement or a complete Store publication test. The existing unavailable fixture Store read boundary remains explicit.

The IFC4x3 editor reference tests and shared retained work reference tests now use `ColdMutation`, `apply_diff_cold` and `ColdDiffAlgebra` for borrowed arithmetic. Production retained mutation row declarations continue to use the native `Mutation::inverse_rows` metadata API. Existing IFC4 production trait migrations are outside this focused update.

Executed the ticket's `details-oracle.py` using Python 3.13 and retained IfcOpenShell 0.9.0.post1. It passed the new `[3, 2]` attribute positions, 1,000,000-byte retained description, independent GlobalId assignment/undo, all seven contextual cases, the 2000-value typed list, previous GlobalId vectors and the 301-entity / 257-reference population. Populations validated by the oracle report zero EXPRESS errors. The oracle explicitly reports native cancellation prefixes and native cursor execution as not executed and physical complex mapping conformance as not evaluated.

Rustfmt accepts the changed provider, cursor and IFC4x3 test source. The focused `cargo test ... --lib every_reduction_cancellation_prefix` again stopped on the compiler `-vV` probe with: **“Original compiler ingress requires the repository launcher and authentic enforcing store.”** No native Rust test or type check ran, and compiler enforcement was not changed.

## Remaining Boundary

Concurrent changes added `validate_work_publication` inside the worker. Its row capacity checks are retained. Its `check_publication_limits` still constructs exact inverse operations and encodes them synchronously, outside demonstrated retained copy/allocation receipts. The earlier report's proposal to leave this advisory check solely to the central retained Store does not describe the latest code. Removing or replacing that check requires coordination with its author and an authentic bounded publication implementation, not a fabricated token or scan.

The final native reducer still performs synchronous catalog/component scans, Vec shifts/rotations, path metadata allocation and newly supplied payload conversion. Fuel exhaustion inside final metadata reduction is a refusal rather than a serialized resumable reduction position. These limitations are separate from the fixed dangling custody gaps. Source copying is incrementally cursor-owned; full end-to-end physical grant closure, checkpoint restore, retained Store publication and scheduler cancellation execution remain unverified and unfinished.

Changed production scope: shared IFC details provider and work cursor, plus a test-only IFC4x3 wrapper reexport. Changed verification scope: IFC4x3 editor tests and neutral retained fixture, shared work cold-reference tests, ticket oracle and reports. Parent MVD production/profile lines were preserved. No AGENTS, IDE, git history, worktree, global build or compiler enforcement files were edited.
