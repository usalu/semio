# IFC4X3_ADD2 EXPRESS Semantic Evaluation Plan

## Scope and Ownership

Implement an owned TypeScript semantic frontend and evaluator under `🧬️schema/⚖️validation`, retaining a canonical language-neutral AST JSON contract for a subsequent Rust implementation. The existing catalog and structural APIs remain unchanged. The parent owns MVD/catalog changes and the other worker owns geometry interchange; this work does not modify those surfaces.

The task covers all retained IFC4X3_ADD2 WHERE/type rules, derived expressions, inverse declarations, UNIQUE constraints, context-dependent bounds, 48 function bodies and two global RULE declarations. A construct is supported only when compiled and executed, with explicit incomplete diagnostics for unsupported expressions/statements, indeterminate results and exhausted budgets. Full-conformance metadata remains false until the complete retained corpus and independent differential corpus pass.

## Schema First

Define owned expression, statement, declaration, type-expression and program AST variants in a JSON Schema before implementation. Canonical AST scalars use decimal strings for integer/real literals, source spans and names; no external runtime types escape the interface. Implement a bounded lexer and Pratt expression parser, plus function/global-rule statement parsing, over the retained official catalog source. Compile and index every retained source expression and statement, including dynamic aggregate bounds and inverse relationships.

## Evaluation Model

Use a tagged semantic domain with exact bigint integers, finite reals, three-valued logical/indeterminate values, typed/enumerated values, identity-bearing entity references and kind-aware LIST/SET/BAG/ARRAY aggregates. Keep value equality separate from entity identity. Resolve inherited attributes, SELF type qualifiers, derived dependencies, inverse membership and named-type constraints from the same catalog. Implement 1-based and explicit ARRAY indexing, QUERY, aggregate arithmetic and the EXPRESS builtins actually required by the full schema.

Implement function execution with scoped parameters/LOCAL variables, assignments, IF/CASE, REPEAT bounds and WHILE/UNTIL, RETURN and entity/type constructors. Global rules iterate the actual schema population; UNIQUE uses declared constraint tuples. Cycle detection, source diagnostics and conservative indeterminate behavior prevent false semantic success.

## Progress and Cancellation

Compilation and evaluation take caller-owned cancellation/progress options and explicit token/node/work/depth/iteration budgets. Public expensive APIs yield at bounded batches, check cancellation before publication, and return a complete/incomplete status. No external runtime parser or evaluator dependency is introduced.

## Verification Sequence

1. Language-neutral parser/evaluation cases and canonical AST schema validation through existing Ajv.
2. Compile every retained expression/function/global rule and report exact construct coverage; unsupported constructs remain visible.
3. Evaluate focused logical, numeric, identity, aggregate, QUERY and control-flow adversarial cases.
4. Evaluate official triangulated/alignment/structural fixtures and authored invalid models with independent IfcOpenShell `express_rules=True`, comparing named rule failures and absence/presence of incomplete status.
5. Iterate every corpus mismatch; retain full semantic flags as false until all required constructs and normative cases execute.

Native Rust source fixes are recorded separately in `native-io-fixes.md`; blocked authentic compiler ingress does not prevent TypeScript semantic work and is not bypassed.
