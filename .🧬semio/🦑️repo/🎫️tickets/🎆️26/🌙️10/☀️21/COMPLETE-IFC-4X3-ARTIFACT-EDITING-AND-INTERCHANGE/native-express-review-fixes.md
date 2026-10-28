# Native EXPRESS Review Fixes

## P1 Effective Derived Attribute Bindings

Native and TypeScript entity scopes now bind only SELF. Explicit locals, formal parameters and QUERY variables remain normal lexical bindings. Unqualified entity attributes resolve lazily through SELF after checking lexical bindings, so an inherited positional derived marker cannot mask its effective derived value. This also removes the eager copy of every entity field into each evaluation scope.

The shared regression population contains a concrete IfcGeometricRepresentationSubContext whose derived TrueNorth comes from its parent context. Both populations have zero structural errors. With a three-component north direction, full validation must fail inherited North2D for source entities #4 and #5. With a two-component direction, both pass. Explicit SELF, inherited/leaf qualified group access and parent access return the same north entity. Local known/unknown values, formal parameters, local declarations and QUERY variables retain precedence over that unqualified attribute name.

Tests and neutral fixtures were written before the production fix. Initial focused execution showed two failures: full validation missed #5 and unqualified TrueNorth returned Unknown. After the fix, all four new TypeScript tests pass (25 assertions).

## Independent Full Oracle

The retained ticket script derived-binding-oracle.py executes actual complete IfcOpenShell 0.9.0.post1 validation, with EXPRESS rules enabled, against both shared source bodies. It separately checks structural validation. Actual oracle results:

| Case | Structural Errors | North2D Failures |
| --- | --- | --- |
| inherited-three-dimensional-north | 0 | #4, #5 |
| inherited-two-dimensional-north | 0 | none |

The new full TypeScript validation tests produce exactly those source identities and completeness/validity outcomes. This evidence comes from full model validation, not an isolated evaluator expression. The generated oracle output is under the ticket generated folder for shared ticket cleanup; the input script and permanent neutral fixtures remain reproducible.

## P2 Foreign Native Handles

Identifier now validates bound values and SELF ownership before any arena declaration lookup. Missing and coincident foreign arena indexes both produce the owned `ExpressEvaluationError` code entity-domain, with no arena indexing first. Qualified attribute reads, scalar/aggregate equality, binary operations and callable inputs also check their originating value context.

Aggregate values containing entity handles retain that context dependency. Construction validates child ownership incrementally before creating the aggregate; callable/constructor copies cannot erase it or import foreign nested handles. Primitive-only aggregates remain context-independent and portable. Public make(Entity(index)) is refused: raw arena data cannot be copied into another evaluator and rebound to an unrelated local entity. Only the internal owned arena factory, entity-ID queries and constructors issue entity values. Source IDs and context-bound handles remain separate.

Native regressions cover an empty receiving arena, coincident indexes, direct SELF reads, attributes, missing-value equality, TYPEOF, constructor arguments, nested aggregate creation and raw-data reconstruction, then verify the receiving context's own source attributes still work. They also verify that a primitive aggregate can be used by another evaluator. These are native test sources; execution remains unverified under the authentic compiler restriction.

## Verification

- Full relevant TypeScript validation/parity/binding suites: **89 tests, 1,107 assertions pass**.
- All unchanged official examples: **47 corpus tests, 462 assertions pass**.
- Actual full IfcOpenShell 0.9.0.post1 oracle matches both new populations, with zero structural errors.
- Strict TypeScript check passes using target ES2024, matching the current STEP boundary's standard string API requirements.
- rustfmt parses/formats and check passes for eight native source/test/fixture/module files.
- The canonical AST/program source remains unchanged, with digest `10f5e2e69fd863a7e26e85f1a24d5f4915e41a3987c99afb449e8729fc8e0b84`; the existing native program parity test passes.
- Native execution tests now total 17. All remain unrun: authentic compiler ingress still refuses before project compilation. No bypass or build changes were made. Limits and physical NotAssessed metadata are unchanged.

## Changed Sources And Fixtures

All production/test changes are within the IFC4x3 validation domain: evaluator.rs, population.rs, population.ts, execution-tests.rs; new derived-binding-cases.json, binding-expression-cases.json, derived-binding.test.ts, derived-binding.feature and binding-cases.rs. The retained native-expression-cases.ts generator emits the new shared native binding AST. Native implementation/integration reports are updated with current counts and ownership semantics. No shared IFC details, FEM receiver, host framework, AGENTS or Git state was edited.