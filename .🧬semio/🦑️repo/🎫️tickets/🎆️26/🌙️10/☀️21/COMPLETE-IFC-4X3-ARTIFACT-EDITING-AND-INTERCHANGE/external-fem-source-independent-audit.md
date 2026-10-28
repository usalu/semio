# Independent External FEM Source Capture Audit

## Result

No new confirmed P1/P2 in the stable source checkpoint. Read-only review covers the optional central copier policy, external-source.ts, femSourceRecords and FEM entry integration, neutral external-source cases/schema/tests, and external-fem-source-plan.md. The worker paused these production changes for review. No production edits, full suite duplication, compiler bypass or native execution occurred.

## Central Default Preservation

copyIfcOwnedSteps retains its existing first three parameters and typed IFC admission branches. The fourth IfcOwnedCopyPolicy defaults to no effective property ceiling, the existing transformed container-depth bound, and the previous ordinary assignment behavior. External plainDataOnly changes are selected explicitly. The released IFC schema/value/reference validators and their source capture callers remain the same. This is source-level default preservation; no independently executed full central regression sweep is claimed.

The external policy supplies exact maxProperties and actual maxContainerDepth (root container depth zero), rejects function and symbol leaves, and defines target own properties with Object.defineProperty. That last choice preserves an own __proto__ field as data and prevents invoking Object.prototype's setter. The policy does not invoke source constructors. Source containers require ordinary Object/Array prototypes; own fields must have enumerable data descriptors. Symbol keys, accessors, nonenumerable data, sparse/additional array fields and cyclic containers refuse.

Immutable scalar values are directly retained without JSON conversion, preserving bigint beyond the Number safe domain, signed zero and strings. Mutable object/array aliases are expanded independently; active recursion detects cycles but allows repeated noncyclic references, so completed private data contains no mutable alias into the caller. Numbers are preserved rather than claimed semantically validated by this copier; actual FEM numeric/engineering admission remains a separate responsibility.

## FEM Source Boundary

fem3dToIfc4x3 captures original controls and awaits captureIfcExternalSource before reading any FEM model property or invoking its semantic/export/topology callbacks. It rebinds model to the admitted artifact. Source getters cannot execute through pre-capture model.nodes/model.elements reads because those reads moved after capture.

The generic interface reuses the central generator copier rather than adding another cloning implementation. Original signal, callback, finite bounds and batch controls are privately captured. runIfcEditAsync checks cancellation around generator advancement and callbacks; source errors map to exchange.source, cancellation remains io.cancelled, and arbitrary callback exceptions are not misreported as invalid source. Partial owned capture is never returned as an artifact.

Optional domain-record indexing begins only after complete private copy and invokes FEM's internal generator on owned data. femSourceRecords validates root/analysis objects, all eight collection arrays and their record elements, plus nested load arrays/records. maxEntities counts identity-bearing rows in those collections and nested loads, rather than every object field. Analysis is structural metadata, not an identity-bearing row. maxValues counts copied own properties/array indices, excludes intrinsic array length, and maxDepth bounds actual object/array nesting. This meaning is explicit and tested at exact boundaries.

Indexing checkpoints before the initial row, at admitted bounded batches, and after the final row. Limit failure yields no source artifact to the FEM exporter; original cancellation prevents semantic work/publication. All later topology/material/node/member/load processing reads private model containers, so callback mutations of caller arrays, engineering words or fixed-DOF aliases do not alter admitted output.

## Tests And Remaining Boundaries

Authored tests cover property/depth limits; function/symbol/prototype/cycle refusal; own __proto__; mutable aliases and exact bigint/signed-zero/string leaves; model/record/word/constructor accessors without invocation; semantic/index callback mutation; exact record/property/container-depth boundaries; and capture/index original signal replacement cancellation. The worker initially reported 24 passing tests/81 assertions, with expanded central/FEM regression suites still running at audit handoff. These are worker-reported TS checks, not independently rerun results or native evidence.

Cooperative capture can observe caller mutations before capture completes, as documented by the shared contract. It is not an atomic initial-state snapshot. Reflect.ownKeys still allocates a complete key roster before yielding per property; a huge source container can therefore cause an eager host allocation before the property ceiling refuses. This known physical frontier remains open and is not fixed by the new finite logical property limit. Proxy traps and arbitrary hostile runtime mutation are not proven absent by ordinary own-data descriptor inspection; this report does not extend getter safety into a general JavaScript sandbox claim.

The generic copier and record traversal are structural/source-ownership admission, not full FEM schema/solver correctness or complete IFC EXPRESS validation. Existing downstream domain validation and explicit loss/refusal policy remain necessary. Metadata/semantic mapping and native grant/copy/retirement/publication gaps remain separate. No native-produced output or execution marker was created during this audit.

## BIM Adoption Follow-Up

Read stable bim.ts bimSourceRecords and entry adoption plus bim-source.test.ts after root's test-first checkpoint. No new confirmed P1/P2. The exporter captures controls and source before reading project/family fields or invoking semantic progress, then rebinds its local model to owned data. The domain generator counts the project plus records in each populated family and checks object containers; this keeps BIM record accounting separate from FEM's eight collections/nested loads. The same exact property/container-depth/plain-data policy applies through the shared helper.

New tests cover semantic callback mutation of project/site/nested origin fields, project/name/latitude/origin/constructor accessors, all configured finite limits before semantic work, and original cancellation during both capture/index phases. Root reports 13 new tests and a combined 59 passing tests/240 assertions with one old gated skip. This reviewer inspected source/tests but did not duplicate their run. The adoption establishes private source custody, not complete BIM domain schema admission. Cooperative capture's caller-stability contract and Reflect.ownKeys physical roster frontier remain unchanged.
