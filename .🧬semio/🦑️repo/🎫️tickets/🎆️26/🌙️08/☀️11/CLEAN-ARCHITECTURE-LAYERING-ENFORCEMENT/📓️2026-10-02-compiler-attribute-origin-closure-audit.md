# Compiler Attribute Origin Closure Audit

Bounded source-only review, 2026-10-02. No compiler, Cargo, Nx, test execution, or production edits. Native behavior described by Root was not independently rerun. No concrete unsound admission was established in this review; this is not full correctness or whole-census GREEN evidence.

## Observed Admission Boundary

Discovery now records compilerAttributes on syntax-admissible local-block scopes. Actual source execution checks the origin closure helper inside candidates.every after requiring a valid manifest, coherent expansion offsets, a source chain from crate root to the inspected path, and an exclusive matching sourceScope. Include-mounted subjects remain refused. Thus the new facts are not merely decorative metadata or an unconnected helper.

## Captured Namespace Closure

Binding helper rustCompilerAttributeOriginsClosed rejects explicit imports or extern aliases whose effective name is test or doc, regardless of conditions. It inspects all same-crate/same-manifest accepted source contexts before checking the target module. Global macro_use externs, cfg_attr-bearing extern routes, and macro_use module contexts fail closed. Denied macro_use module participations fail globally as well.

Local wildcard routes beginning self/super/crate recurse into captured contexts. External or unqualified wildcard providers fail closed. Traversal uses crate-root/module keys and refuses ambiguity, unresolved targets, missing modules, denied matching participation, and cycles. Alias-mediated local wildcard paths are conservatively refused when they do not identify a captured actual module; no optimistic alias fallback is present.

Include-generated module bindings are inspected through all accepted contexts for the logical target module, retaining each context's sourceScope. Module-level include producers require a captured include mount corresponding to the actual authored path; missing contributions do not silently prove absence. Other module-level macro invocations fail closed. Opaque attribute producers fail closed except exact bare test; derive is not whitelisted. Module-level macro_rules definitions are allowed, while their invocation producers remain refused. Function-local producer rows have blockScope and cannot accidentally masquerade as module imports.

The module scope proof checks root and inline ancestor scopes, including uniqueness and unresolved metadata. Unsupported use trees and malformed extern declarations refuse the containing captured file. These are deliberate conservative refusals rather than bypasses.

## Remaining Verification Obligations

The ten new attribute-origin vectors and their native/portable tests were being authored concurrently and were not executed here. Verify hostile explicit/renamed aliases, external wildcard ambiguity, literal-doc aliases, local self/super/crate wildcard traversal, aliases contributed by includes, opaque producers, macro_use imports, denied participation, and cycles through the actual executor. Positive PDF/Home parsing alone does not prove admission under their full captured manifests and namespace graph.

The helper currently treats both test and doc as protected names even when only one appears on the function. This is conservative. Safe wildcard cycles may also be refused; no permissive cycle memoization should be introduced without separate absence-of-provider evidence. Captured graph/source completeness remains an input authority requirement, not something this bounded review reconstructed independently.

No production fix is recommended from this bounded audit beyond completing the authored executable witnesses and checking their actual admission/refusal results.
