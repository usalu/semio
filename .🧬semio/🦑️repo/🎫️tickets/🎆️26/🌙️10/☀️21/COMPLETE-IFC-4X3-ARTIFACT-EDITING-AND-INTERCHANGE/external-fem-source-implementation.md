# Original External FEM Source Capture

The actual `fem3dToIfc4x3` endpoint now captures external FEM data before reading any model property or invoking semantic progress. Original mutable nodes, members, materials, section definitions, supports and load actions cannot be rewritten by later caller callbacks. Source getters and constructors are not invoked during ordinary plain-object capture.

## Shared Interface And Central Policy

New `🚪️io/external-source.ts` exports:

`captureIfcExternalSource<T>(source: T, control: IfcIoControl, records?: (owned: T) => Iterable<unknown>): Promise<IfcExchangeOutcome<T>>`

It captures the original operation controls and uses the existing `copyIfcOwnedSteps` and `runIfcEditAsync` as the only copier/driver. Copy progress is `capture-external`. An optional domain-supplied record iterator runs only over the private completed copy; its progress is `index-external`. Unsupported source/limit results use `exchange.source` with no artifact. Cancellation retains the initiating signal and throws `io.cancelled`, including cancellation from a callback which replaces the original caller-owned options.

With root approval, the central copier adds an optional fourth `IfcOwnedCopyPolicy` argument:

- `maxProperties` bounds copied enumerable own fields and array indices. The next field is refused before its descriptor value is copied when the budget is exhausted.
- `maxContainerDepth` bounds actual object/array nesting, with the root container at depth zero.
- `plainDataOnly` rejects function and symbol leaves. It defines copied properties directly, so an own `__proto__` data field remains ordinary owned data and cannot alter the copied object's prototype.

Existing graph callers omit the policy and retain their original typed admission, depth scaling, descriptors/prototype checks and scalar/alias behavior. No IFC graph capture behavior, schemas or native admission gate were weakened. All capture still requires ordinary Object/Array prototypes, dense arrays and enumerable own data descriptors; symbol keys, cyclic objects and custom prototypes refuse. Acyclic source aliases are expanded according to the existing copier semantics. Bigint, binary64 word bits, signed zero, strings and primitive values remain exact.

## FEM Entry And Honest Limit Meaning

At entry, `fem3dToIfc4x3` captures controls, then calls the generic source capture. Its first `model.nodes`/other model access follows successful private capture. The endpoint's private record iterator verifies root collection/record containers and counts identity-bearing FEM nodes, elements, materials, sections, solids, supports, load cases and combinations plus every case-scoped load. Analysis display/solver settings are not counted as another identity-bearing record.

For this external source route:

| Control | Meaning |
| --- | --- |
| maxValues | Total own fields and array indices actually copied, including native word wrappers and extra own data. It does not mean IFC graph values or UTF-8 bytes. |
| maxDepth | Actual source object/array container depth, root zero. It does not use the typed IFC envelope multiplier. |
| maxEntities | Number of the listed FEM domain records after capture. It does not bound arbitrary source objects, generated IFC entity expansion or copy allocation before record counting. |
| batchSize | Cooperative central copy yield frequency and private record-index checkpoint frequency, using the same original controls. |

The supported two-node, one-member, one-material, one-section, one-support, one-load-case/one-load fixture contains **eight** counted records; source container depth is **five**. Exact-boundary tests admit those limits and refuse one less. Property-budget tests admit exactly the measured own-field count and refuse one less.

The existing cooperative contract remains explicit: capture can observe caller mutations while it is still copying. Exact start-state capture requires the caller to keep source data stable until capture completes. Once copying finishes, indexing, FEM topology preflight and every later semantic callback use only the private source.

## Test-First And Executed Evidence

Closed neutral `external-source.schema.json` and `external-source-cases.json`, plus the registered `external-source.test.ts`, preceded implementation. The initial new suite reproduced **4 passes / 20 failures**. It now covers policy property/depth limits, function/symbol refusal, safe own `__proto__` data, exact scalars and alias expansion; every represented FEM source getter/constructor location; custom prototypes/cycles; finite source limits; and original cancellation during both capture and private indexing.

The expanded tests additionally mutate original arrays during `index-external`, `fem-export`, `fem-topology` and `fem-loads` callbacks and require unchanged captured output. A separate alias case shares native word objects and restraint arrays, then changes the original words, member endpoints, material values, section name, restraints and actions after capture; the private result remains equal to its baseline.

The required focused run executes **146 passing tests / 6,120 assertions across eight suites**, with one pre-existing genuine-output-gated skip and zero failures (16.32 seconds):

- new external source and FEM isolated-node suites;
- existing FEM and original-control/source exchange suites;
- template capture, central native shape, and the explicitly registered central editing law suite.

This executes the actual changed copier and endpoint. It does not execute native Rust. Focused strict TypeScript checking reports only the existing missing `@semio-tech/framework-interaction` transitive shared module; no new type diagnostic was reported. No stub, altered checking mode or build bypass was introduced.

Actual mapper outputs were regenerated after source capture was enabled. The retained `fem-isolated-oracle.py` independently validates all **three** admitted connected graphs with IfcOpenShell **0.8.3**, full `express_rules=True`, and **zero diagnostics**. Its physical STEP reserialization is reparsed and projected through the actual importer by `fem-isolated-verify.ts`: **3/3 supported semantic matches**, including topology, engineering values, supports and nodal loads. Identity canonicalization and equivalent signed-zero roll normalization remain as documented in the preceding isolated-node report; no new bit-exact angular claim is added.

Focused test/type logs are retained under `🗑️generated/external-fem-source-*.txt`; independent output remains under `🗑️generated/fem-isolated`. Existing reusable oracle scripts were reused rather than copied or replaced.

## Review And Remaining Physical Boundaries

Production source and tests are paused for independent read-only review. The existing `Reflect.ownKeys` intrinsic still allocates a complete host key roster for each source container before per-field copying. Property limits do not turn that intrinsic into a physically budgeted allocator cursor. Ordinary JavaScript Proxy reflection traps are also outside the plain-object descriptor noninvocation guarantee. No native heap/receipt/scheduler proof is claimed by this TypeScript work.

This change adds source ownership and real capture/index cancellation. It does not recast existing FEM semantic collection constructions/material loops as fully bounded native work or complete source-schema conformance. Their broader physical/semantic boundaries remain explicit follow-on work. Unsupported native Map-bearing combination sources still refuse; supported exported models use plain data and the existing explicit no-combination policy.

Changed source: new `🚪️io/external-source.ts`, `🚪️io/fem.ts`, and the approved optional policy in `🧬️schema/🔺️diff/🟦️.ts`. New neutral inputs/tests: `🚪️io/🧪️tests/external-source-{cases.json,schema.json,test.ts}`. The new suite is explicitly registered in the IFC TypeScript package router. This report and `external-fem-source-plan.md` are retained ticket inputs.

Root separately owns BIM adoption and concurrent test refinements; this worker did not edit BIM. Native FEM/EXPRESS, build enforcement, template semantic gates, AGENTS, Git state and ticket/goal lifecycle were preserved. Shared generated dependencies remain available to the fleet.
