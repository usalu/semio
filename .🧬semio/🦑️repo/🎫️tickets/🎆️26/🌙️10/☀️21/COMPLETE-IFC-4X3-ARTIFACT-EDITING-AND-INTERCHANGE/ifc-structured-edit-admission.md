# Catalog-Backed Structured Edit Admission

Implemented in IFC shared catalog `📚️catalog/⚖️domain` (schema, Rust, TypeScript and shared neutral vectors), with definitive native integration in shared Details `domain_edit.rs` and `work.rs`. IFC4 and IFC4x3 retain their own catalog nominal types and use the same owned graph and primitive mutation authorities. IFC2x3 remains on its own Part21 path; this report does not claim its structured predicate integration.

## Behavior

The former IFC4x3 private structural matcher now delegates to the shared declaration core. Its diagnostic wrapper retains separate population/reference reporting, but shares scalar/alias/enum/SELECT matching, static aggregate bounds and EXPRESS-style value equality. SET is implicitly unique. Native REAL must be finite; TypeScript checks exact binary64 words. Mixed INTEGER/REAL equality compares an exactly integral real with the exact integer, rather than coercing the integer to binary64. Nested BAG/SET equality is unordered with multiplicity; LIST/ARRAY equality preserves order. Unicode STRING width counts scalar values, BINARY obeys the existing Part21 lexical predicate and declared bit width, and typed SELECT constructors require one admitted payload.

Typed changed-value checks validate the new subtree and affected aggregate ancestors, preserving unrelated invalid siblings. Cardinality deficit/excess may improve incrementally but cannot worsen. A duplicate semantic value group cannot increase beyond its original multiplicity (or one for a new group); repairing an existing duplicate is permitted. Partial ARRAY length changes are refused. Attribute-derived slots refuse changes. Optional ARRAY elements do not make the required aggregate optional. Explicit entity/constituent type variants remain an intentional incomplete draft operation; required unset placeholders from those variants are not represented as valid completed EXPRESS instances.

The native retained work checks borrowed prepared mutations before the existing publication validator. Original captured snapshot, original cancellation/fuel callback, pending-owner custody, granted copy/receive work, row capacity and Store publication authority remain in place. Failure emits no mutations and retained close still owns pending values. The synchronous Details special route also checks declarations. TypeScript `declaredValueMutation` emits the existing stable argument/complex primitive and referenceMutation now delegates to the same changed-ancestor check.

## Executed Evidence

- Neutral fixture currently contains 26 strict-domain vectors and 8 changed-aggregate repair vectors, run against both IFC4 and IFC4x3 catalogs.
- Five actual TypeScript domain test groups pass with zero strict diagnostics. Existing AJV independently checks Unicode width, closed enum and typed-payload contracts. Existing Decimal independently checks exact mixed numeric comparisons and negative zero. Every exercised domain/equality callback cancellation boundary is tested.
- Fifteen full Details TypeScript groups pass with zero strict diagnostics, including real primitive apply/inverse and typed enum/derived/aggregate edits. Nine own IFC2x3 Details/selection groups remain passing with zero strict diagnostics.
- Native tests consume the same neutral JSON vectors under each own version profile. Actual retained branch regressions were authored for raw enum/derived refusal and cancellation during final declaration admission using the same OriginalWorkFixture. All changed Rust sources parse with the independent tree-sitter grammar.

Harness: ticket `ifc-domain-check.mjs`, exported `runDomainChecks(root,ticket)`; full Details `ifc4-details-check.mjs` / `runDetailsChecks`. All generated output stays under this ticket's generated folder.

## Remaining Gates

Native compilation and retained host execution remain unexecuted because the repo build cache has the already reported Windows case-sensitivity AccessDenied gate; source grammar checks are not compilation. IfcOpenShell is unavailable. AJV/Decimal checks cover these field-domain/equality contracts, not full EXPRESS conformance. Population WHERE/UNIQUE/inverse/global rules and contextual bounds are separate existing validation work, not evaluated for each input edit.

The declaration scan is controlled and bounded by original fuel/cancellation, but it is currently one-shot: exhausting a turn during a large domain/uniqueness scan refuses that edit and retains its owners for close. It does not claim a resumable domain-validation cursor. Existing copy/receive/publication phases remain resumable. Unordered nested aggregate equality uses controlled repeated group comparisons without an unbudgeted retained matching frontier.

Main-agent independent rerun: all5domain groups and15fullDetails groups pass with zero diagnostics at the delivered source state. Actual source helper checks do not establish native execution or resumable domain-validation completion.
