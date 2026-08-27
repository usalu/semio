# Aggregate Source Validation Contract

## Frozen Shared Boundary

An aggregate must compare every wrapped leaf's descriptor and provenance with the aggregate's own compiler-resolved workspace and mutation root. A well-formed owner string or matching semantic name alone is insufficient. The public leaf derive remains metadata-only; behavior still belongs to the leaf's handwritten mutation-kind implementation.

The lower contract will provide a const validator using a `MutationLeafSourceScope` value with exactly `workspace_token: [u8; 32]`, `mutation_root: &'static str`, `taxonomy_path: &'static str`, `source_filename: &'static str`, and `descriptor_filename: &'static str`. These are the aggregate authority's independently resolved facts. The scope has no default and no wire serialization.

`validate_mutation_leaf_source(descriptor, provenance, scope)` returns `Result<(), MutationLeafSourceValidationError>`. The error has static `field` and `requirement` strings. The function must work in const evaluation, allocate nothing, and use only repository-owned/system facilities.

Validation requires the full descriptor to pass its existing validator; both root and taxonomy locators and both filenames to be safe normalized portable paths; descriptor owner to be an immediate child of the scope's mutation root; all 32 workspace-token bytes to match; provenance root, owner, and taxonomy locator to match the independently expected facts; source and descriptor paths to equal the descriptor owner plus exactly one slash and their respective canonical taxonomy filenames. Prefix-only, foreign workspace, foreign root, nested paths, historical filenames, parent/empty components, backslashes, NUL, newlines, drive syntax, and excluded compose components must fail. Filenames are single path components. Do not introduce Unicode normalization or case guessing.

The aggregate derive must additionally enforce variant name/descriptor identity, semantic-kind agreement with `SEMANTICS`, and full roster uniqueness. Those checks remain distinct from this per-leaf source validator. Manual provenance can technically reproduce the constants; the independent source-policy gate must reject hand-written production metadata implementations and unapproved provider aliases.

## Implementation Packet

FND-SOURCE-ROSTER-13 owns the lower source validator/types, explicit OS command/SPR reexports, and dedicated schema-first neutral fixtures and tests. No base `Mutation` requirement, kind supertrait, existing aggregate derive, production leaf, or root policy changes are included in this narrow write set. Tests must use the actual unchanged production validator, include compile-time rejection probes, and compare outcomes against a separately implemented Ajv-backed exact-value/path reference. All compiler sources and logs stay in the ticket. The subsequent aggregate cutover consumes this validator directly; no fallback or opt-in path is permitted in the final architecture.
