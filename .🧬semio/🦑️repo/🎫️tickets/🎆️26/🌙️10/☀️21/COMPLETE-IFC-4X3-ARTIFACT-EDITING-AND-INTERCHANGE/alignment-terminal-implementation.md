# Alignment Terminal Business Predicate Implementation

The actual IFC4x3 API now exports validateIfcAlignmentTerminal and its closed request/report types. It evaluates the explicitly selected business-terminal-zero clause from the released Horizontal, Vertical and Cant Alignment child. It does not report complete child, geometry, schema or full-view conformance.

## Contract And Implementation

Definition, closed JSON schema,38 neutral graph cases, Gherkin and test fixture were authored before the endpoint. The first run failed on the missing endpoint. The immutable definition pins the official buildingSMART release commit610ee3f95db9d0380b1eb8de4dc6e17833531518 and source SHA25646f10e76e2646fbfd5f3c8e6967c59fb3b522169086dfd9a1af6e0ccfefe7158. The prior downloaded original source and hash manifest remain authoritative inputs; a fresh raw web fetch returned cache-miss.

Request targets contain canonical positive u64 layout ID, explicit horizontal/vertical/cant kind and applicability true/false/null. They are closed and unique by original layout ID. False is not-applicable; unknown applicability is incomplete. A known mismatching selected layout kind fails, while unknown/mixed layout roles remain incomplete. Reports always keep geometryConformance, schemaConformance and viewConformance not-evaluated.

Original source and request are privately captured before semantic callbacks. A duplicate-aware source ID index and one cooperative source relationship pass find actual IfcRelNests.RelatingObject owners. Multiple lists are ambiguous; no concatenation or ID sort invents a business order. RelatedObjects order supplies the terminal member. Every member must resolve to an authenticated AlignmentSegment and corresponding H/V/C DesignParameters declaration. Direct complex fields use their own declaration ordinals; simple records use inherited positions. Unknown/duplicate/missing attributes, sources, members and parameters remain incomplete, which outranks known terminal failure.

The declared NonNegativeLengthMeasure→LengthMeasure→REAL chain accepts exact matching single-item wrappers or raw numeric values. Integer zero is compared as bigint; it is never converted to binary64. Finite real zero includes signedzero. Tiny and large finite nonzero values fail at the terminal position; malformed or nonfinite evidence remains incomplete. Earlier zero never satisfies a nonzero terminal. Underlying alias WHERE constraints are outside this explicit partial clause.

Shared concepts/TS and types now own original capture, cancellation, duplicate-aware index, cooperative declaration membership/attribute/primitive access and shared status/provenance/diagnostic types. Existing Body Tessellation delegates these source operations and retains its three released predicates. No runtime external library or catalog parser was added.

## Executed Evidence

The final combined Alignment and Body Tessellation suite passes **92 tests /507 assertions /zero failures**. Alignment contributes61 tests across38 neutral cases, closed schemas, exact editable pointers, caller replacement, original signal/callback authority, capture/index/relation/member/result cancellation, unavailable-source cancellation and actual official-model editing. Narrow strict TypeScript checking passes for both concept suites after the official integration law; the preceding combined owned/control/body/Field+concept graph also passes.

Independent IfcOpenShell0.9.0.post1 recreates the existing11 physical H/V/C cases. The actual STEP importer reads its92-entity graph, and the new endpoint agrees with all11 independent statuses and actual layout IDs/length-attribute choices. The retained inputs are alignment-terminal-oracle.py, alignment-terminal-cases.json and alignment-terminal-endpoint.ts; exact physical outputs/comparison are under generated/alignment-terminal. PATH Python3.14 cannot load the existing cp313 oracle extension; the explicit installed Python3.13 passes. This is a dependency ABI distinction, not a conformance failure.

An expanded cancellation harness with batch1 hit the existing5-second per-test timeout once while other work ran. Its final five-target cases admit batch4, reaching genuine interior and completion boundaries for every selected phase while retaining original signal replacement/refusal checks. Production defaults, global test timeout and previous batch1 laws are unchanged. Dedicated original capture checks still use batch1.

Independent alignment-endpoint-independent-audit.md found no new confirmed P1/P2. Earlier fixture-only audit findings were corrected: variant rows require at least two lengths, and wrong-type fixtures use actual alternate declarations. They are not presented as complete schema-valid complex populations.

## Integration And Remaining Scope

The actual IFC package script registers the new concept suite, and the IFC4x3 public module exports the endpoint and owned types. The refreshed58-file combined registration passes941/18309assertions/12gatedskips/0fail and includes the official-model integration law. Focused concept92/507 and narrow strict also pass. The preceding56-file872 snapshot is historical. See registered-test-scope.md for exact scope.

The original pinned official sectioned-solid-horizontal.ifc already has exact IFC4X3_ADD2 header and169 entities. Its selected H34/V45 terminal lists pass. A real central setEntityArg on parameter1102 yields a sparse diff whose output agrees with independent fast-json-patch; only H changes to fail. Actual STEP export/import preserves the edited graph, and both central diff inverse and inverse mutation restore it exactly. The original graph still passes afterward. The independently observed geometry links remain separate from this business verdict. IfcOpenShell f.schema is a family label; exact header/schema_identifier establish ADD2, with no header rewriting.

The actual endpoint also parses both original official cant populations from existing corpus files: segmented-reference-curve layout61 and linear-placement-of-signal layout2388. Both match independent IfcOpenShell ordered business length observations,2/2. Original hashes are checked against the independent corpus oracle before parsing; no input is rewritten. Exact comparisons are generated/alignment-terminal/official-cant-endpoint.json, reproduced by alignment-terminal-endpoint.ts. This adds official cant business evidence and keeps geometryConformance not-evaluated, including the model whose terminal has no direct Representation.

Native Alignment parity is not implemented or executed. Native compiler ingress still refuses before rustc; no native semantic/runtime/capability flags are enabled. Full released child correspondence, geometry, authenticated complete-view applicability/constraints, actual host auxiliary mount and all original goal requirements remain open. Official geometry counterpart exploration is recorded separately in alignment-geometry-counterpart-source-plan.md.
