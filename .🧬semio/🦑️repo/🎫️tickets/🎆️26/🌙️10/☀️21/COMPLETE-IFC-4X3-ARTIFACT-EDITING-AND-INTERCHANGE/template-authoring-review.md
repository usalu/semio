# Selected Template Authoring Review

## Review Point

The TypeScript selected-template implementation is substantially integrated with the existing graph mutation infrastructure. One reproducible P2 input admission defect requires repair before resuming native implementation. No P1 issue was found in this focused review. Native mutation/UI execution and the explicitly documented complex-target/custom-derived-unit limits remain open; this review does not mark the IFC goal complete.

## P2: Inspect Every Own Key Before Capturing Commands

Location: new IFC4x3 `🧬️schema/🔺️diff/🟦️.ts`, `copyIfcOwnedSteps`, lines 83–86.

`keys()` traverses ordinary objects with `for ... in` and arrays only by numeric indices. Consequently, the closed input admission accepts and silently removes non-enumerable own accessors, symbol properties, and arbitrary own array properties (including enumerable accessors). This contradicts the source-owned command contract and implementation report's accessor/unknown-field refusal claim. Because admission sees the sanitized copy, native shape validation cannot discover these input defects later.

Retained reproduction: `template-audit-shape.ts`. Run `bun <ticket>/template-audit-shape.ts`. Each of the following independently publishes the valid four-entity template graph:

- `Object.defineProperty(app, 'description', {get(){...}})` (non-enumerable optional accessor).
- `app.entries.unknown = true` (own enumerable unknown array field).
- `Object.defineProperty(app.entries, 'unknown', {enumerable:true,get(){...}})` (own enumerable array accessor).
- `app[Symbol('unknown')] = true` (own unknown symbol field).

Actual output for each is `ACCEPTED 4 invoked false`. The getter refusal is incomplete, although ignored getters are correctly never invoked. This issue affects the common capture boundary, including template commands and generic diff commands, rather than carrier authoring itself.

Repair should inspect all own property descriptors, reject accessors and unknown/symbol fields before copying, and define canonical array admission (numeric own indices plus its intrinsic length only). Preserve cooperative cursor/cancellation and bounded work: an eager unbounded `Reflect.ownKeys` allocation would weaken the retained control contract. Add rejection tests at the actual history/command publication boundary, asserting no revision/event/snapshot publication and no getter invocation.

## Verification and Scope

Re-executed the complete `template-apply.test.ts` suite with Bun 1.4.2: **30 pass, 0 fail, 106 assertions**. Existing tests cover all seven carriers, all twelve forms, explicit named units, typed exact enum reuse/conflicts, selected table/bounded domains, deterministic identities, exact generic inverse and undo/redo, actual cancellation phases, and a real revision race during semantic execution.

Reviewed selected-template construction plus central command capture, inverse, async semantic admission, and final history publication. Async template execution evaluates the prepared full snapshot and then checks cancellation/revision at publication. The generic inverse remains routed through the central diff admission/applier. No new parallel state was found.

Ran retained `template-audit-domains.ts` against every released entry: every published `*Measure` data type resolves through the implementation's unit mapping/alias strategy; no unmapped published measure was found. This is a focused consistency check, not a claim of arbitrary derived unit dimensional equivalence.

The implementation's independent IfcOpenShell 15/15 population equivalence and 30 complete EXPRESS validations were read from the existing report; they were not rerun in this audit. The existing shape suite was not rerun here. No production sources were changed. The two audit scripts and this review are retained ticket input/report assets. No generated output files were created, and shared generated assets were left untouched.

## Neutral Regression Assertions

For each malformed native command shape above, the language-neutral expected outcome is: command rejected as invalid input; getter invocation count remains zero; source snapshot unchanged; history revision remains 0; history event count remains 0. Repeat these expectations for `applyTemplate`, `applyDiff`, and snapshot/diff capture because the faulty key iterator is shared. A valid dense array without additional fields must still be admitted. Own non-enumerable data fields that are legal command fields require an explicit canonical policy; they must never be silently lost before validation. Own non-enumerable unknown fields must be refused.

Further carrier/domain review found no additional actionable P1/P2: seven category branches select schema carrier/relationship entities; occurrence/type distinctions and material/profile direct references agree with the documented release contract; quantity values use declared simple measure domains; property values preserve exact typed tags; enum definitions retain source literal ordering and require exact full values/unit reference for reuse. Those conclusions are bounded by existing fixture/independent-oracle coverage and explicitly declared unsupported cases.
