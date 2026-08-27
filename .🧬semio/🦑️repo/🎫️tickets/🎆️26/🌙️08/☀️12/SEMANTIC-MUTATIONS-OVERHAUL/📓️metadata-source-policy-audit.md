# Metadata Source Policy Audit

## Current Parsed Boundaries

The bounded Luna audit identified `policyMutationStructuralBreaches`, `policyMutationRootReachability`, `inspectRustStructure`, and `inspectRustModuleGraphFacts` as the existing authorities to extend. Public struct/enum facts currently lose derive/metadata attributes. Impl facts already expose a trait path. The module graph does not yet preserve sufficient `extern crate` aliases or renamed Cargo package identity to prove a metadata contract path.

The metadata check must follow the actual wrapped payload declaration, whatever its type name. A hardcoded public type named `Mutation` is incorrect for existing semantic names such as `RenameWidget`. Reuse the proven source/reexport path from reachability; do not introduce a second filename or type-name guessing mechanism.

## Required Inspector Facts

Add exact source facts for struct/enum declarations: item name, kind, visibility, module scope, derive paths, and `mutation_leaf` contract arguments. Record malformed or ambiguous attributes explicitly rather than inventing defaults. Comments, strings, nested functions, lookalike names, conditional attributes, aliases, and inline scopes need adversarial fixtures.

Add exact crate-alias facts for `extern crate ... as ...`, `extern crate self as ...`, and root import/reexport aliases where resolvable. Existing `RustImplFact.traitPath` can identify visible manual `MutationLeaf` implementations, but terminal-name checks alone cannot prove aliases. All facts must remain source syntax evidence, not pretend to be rustc semantic resolution.

## Policy and Provider Identity

Every public wrapped payload must have the approved metadata derive on its resolved declaration. Explicit manual implementations are rejected, including imported trait aliases once resolved. The genuine provider is the lower replication package (`semio-framework-replication`, lib `protocol`) or its deliberate OS facade (`semio-framework-os-kernel`, lib `semio_framework_os_kernel`). Package dependency key, package name, lib name, and root alias are distinct facts. Resolve declared path/workspace dependencies and public exports, with no name-only fallback.

A fake dependency called `protocol`, an unrelated `MutationLeaf` derive, ambiguous conditional providers, and unresolved aliases cannot satisfy ownership. Existing public reexports and semantic aliases are accepted only with an exact resolved declaration and genuine trait boundary. Unknown macro output is not source proof. Cross-workspace identity additionally requires the actual compiler-generated workspace token; relative lexical paths alone cannot establish it.

The source policy must reject manual provenance even though a handwritten Rust trait implementation can type-check. A fake trait path should fail the aggregate's genuine lower trait requirement. This is a complementary boundary, not a claim that Rust syntax parsing defeats arbitrary malicious macro expansion.

## Test-First Sequence

First implement neutral/compiler-backed inspector facts without changing global policy activation or unrelated structural fixtures. Then integrate the exact facts with reachability and Cargo identity, updating every affected positive fixture and adding negative manual/fake/ambiguous-provider cases. Finally enable the metadata requirement at high severity with the mandatory aggregate/registry transaction. A facts-only packet is not policy acceptance or an optional metadata exemption.

No production source was changed by this audit; real `compose/**` was not accessed.
