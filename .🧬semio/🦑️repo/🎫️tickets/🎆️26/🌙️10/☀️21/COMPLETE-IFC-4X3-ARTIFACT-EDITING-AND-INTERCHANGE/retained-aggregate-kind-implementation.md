# Retained Aggregate Operand Classification Fix

The confirmed aggregate defects from the targeted audit are corrected in native `machine-operation.rs`. Intersection now produces SET whenever either actual input is SET; union deduplication follows the selected result kind rather than a synthetic left LIST used to hold a scalar. No additional evaluator, recursive operation, allocation owner or grant is introduced.

The original native source predicted four names for `'INTEGER' + TYPEOF(1)` because INTEGER was inserted twice, and selected BAG for `BAGVALUES() * SETVALUES()`. A nested `+ ['x']` exposed that wrong intermediate category even when a later return coercion could have masked it. These are certain-value counterexamples, distinct from unresolved aggregate membership semantics.

## Tests And Custody

The closed language-neutral `aggregate-kind.schema.json` and `aggregate-kind-cases.json` preceded the production change. Ten cases cover scalar/SET union in both orders, BAG/SET intersection in both orders, SET/SET intersection, BAG/BAG multiplicity, asymmetric SET/BAG union and nested unions before final coercion. The registered `aggregate-kind.test.ts` parses four fixture function declarations and checks actual TS values and aggregate categories.

New `machine-aggregate-tests.rs` is mounted as a child of the retained machine. It authors the same ten expressions and fixture functions as immutable native AST, checks their intermediate result categories and exact sorted multiplicities, denies each insufficient original item/copy/capacity/release/depth loan before an actual transition, measures requested/released heap bytes against the same original receipt, and retires through controlled close after deliberately denied release loans. A second test cancels every completed prefix of the scalar/SET and mixed-intersection/nested-union paths and requires the same original owner to close completely. These two native tests are authored but unexecuted.

The production change only alters the existing kind/deduplication decisions. Existing Contains/Match/TextCompare/frame and collection-growth continuations still own the work. Indeterminate output membership continues to return `express.aggregate-indeterminate`. Static production AST arrays, owner-index tables, source identity and semantic admission gates are unchanged.

## Executed Evidence

- The aggregate and registered native-program reference/schema suites pass **13 tests / 55 assertions**, zero failures. These execute TypeScript/Ajv, not Rust.
- Focused strict TypeScript checking passes for the new aggregate suite.
- `retained-program-index.ts --check` passes the exact unchanged production digest `10f5e2e69fd863a7e26e85f1a24d5f4915e41a3987c99afb449e8729fc8e0b84`, 584 direct owners and 876 inherited entity index rows.
- `retained-aggregate-kind-oracle.py` executes the actual installed IfcOpenShell `express_set` through a test interface, using **IfcOpenShell 0.8.3 / Python 3.12.14**. All **seven** supported SET-returning matrix cases match exact category/values, including both confirmed counterexamples and nested union. Its installed helper is not a general BAG implementation; three general BAG-result cases are explicitly unverified by that third-party helper rather than misreported as independent matches.
- Rustfmt parses/formats the changed native operation and new native tests. This is syntax evidence only.
- Authentic `cargo test --manifest-path <4x3 rust package>/Cargo.toml --lib retained_aggregate` still refuses before rustc during compiler version probing: **Original compiler ingress requires the repository launcher and authentic enforcing store**. The retained log is `🗑️generated/retained-aggregate-cargo.txt`. No compiler route/guard was changed.

The earlier Low audit used a separate Python environment resolving IfcOpenShell 0.8.5; this new oracle records its actual bundled environment separately. Neither environment proves native output. The translated ISO mirror/preview is not treated as authenticated full normative clause text.

## SELECT Closure And Next Work

No TYPEOF closure implementation was changed by this aggregate fix. The root worker subsequently reproduced an actual pinned IFC rule counterexample: two distinct IfcColourRgb entries in IfcFillAreaStyle wrongly pass MAXONECOLOUR when containing SELECT names are absent. The official ADD2 proposition and released WHERE provide a concrete IFC conformance obligation beyond the earlier unauthenticated general ISO wording. Root owns the TS evaluator/population fix and shared neutral closure vectors; this worker will implement the retained native counterpart after that contract arrives. No TS evaluator/population edit overlaps this checkpoint.

`staged-template-form-plan.md` records the chosen future producer boundary. Guided-form source/framework implementation remains paused during these semantic fixes. Template semantic admission and native execution flags remain false; no complete native admission is claimed.

Changed production: `machine-operation.rs`; test mount: `machine.rs`; new neutral/native/TS tests: `aggregate-kind-{cases.json,schema.json,test.ts}`, `machine-aggregate-tests.rs`; package test registration adds the aggregate suite. Reusable oracle input and this plan/report remain at ticket root, with results under `🗑️generated/retained-express/aggregate-kind-oracle.json`. No source file was copied to the ticket, no AGENTS/Git/build file was modified, and no ticket/goal lifecycle action was taken.
