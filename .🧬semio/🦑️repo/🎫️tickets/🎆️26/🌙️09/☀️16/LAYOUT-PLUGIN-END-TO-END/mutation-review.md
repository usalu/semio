# Mutation Decisions Extraction Review

2026-10-28. Independent read-only source review, comparing current files with git show HEAD through repo tools exec. No builds, tests or source changes. native-inventory.md was read; mutations.md did not yet exist during inspection.

## Result

No actionable semantic defect found in the extraction as inspected. This is a source-equivalence review, not a compiler or runtime pass.

All 58 LayoutMutation variants have an identically ordered LayoutMutationView variant, each containing only a borrowed schema payload. The source impl has exactly 58 matching Self::Variant(p) => V::Variant(p) arms and no wildcard; future additions cannot silently fall through. The only mutable projection returns the existing CreateParentPage payload, exactly matching the former last_mut enum pattern.

Compared all 15 source bodies mounted by the new decisions namespace with their HEAD equivalents: simple, target, document decision/equality, data plan/decision, overrides plan/decision, story plan/text/source, frame decision, batch plan/message and parent creation. After removing only extraction edits (imports/module paths, visibility, generic source parameters, .view calls, associated source Mutation type, test mounts and the parent mutable projection), executable bodies are identical. The remaining story source difference is its generic SpliceTextSource parameter/associated type. All string literals are unchanged, except seven removed child test-mount paths whose tests are now explicitly mounted under root retained. Thus refusal IDs, diagnostic strings, terminal phase decisions, budget constants, allocation/copy/release demands and cursor transitions remain unchanged.

## Lifetime and Ownership

LayoutMutationView is Copy/Clone over references; forming it copies only its tag and payload reference. It does not clone payload vectors, strings, snapshot owners or retained bindings. Generic RetainedCloneRef<M> keeps the original source owner rather than creating a temporary owned wrapper. .project closures return fields borrowed through view references with the original source lifetime, and projection discriminators remain unchanged (including authored document8500 and story8810/8811). Bindings and close order in the state machines remain exactly as before.

OriginalTextSource and DynamicMessageSource now carry an associated Mutation type; their source methods still lend RetainedCloneRef for the same caller-owned root. SpliceTextSource stores the same two borrowed roots and numeric span coordinates. ParentCreationOutput still installs into the caller-owned prepared last mutation and then moves that page into sparse insertion with mem::replace; the new parent_creation_mut method does not add copying or change custody.

## Source Ownership

The new decisions lib mounts the 15 existing taxonomy files directly. Root retained modules reexport the decisions implementations rather than compiling second copies of those same decision modules. Output/inverse/prepare ownership remains with the root mutations crate. The decisions manifest depends on model/diff/content/frames, not root mutations, so the extraction does not introduce a dependency cycle. Existing seven child test modules are explicitly remounted in root retained; their imports now address public decision modules and the concrete root LayoutMutation. No stale super references were found in those remounted tests.

The public borrowed trait expands the callable API surface but all involved schema payloads belong to this codebase; no third-party type is newly exposed by it. There is an extra bounded enum dispatch at view access, with no extra per-step payload copying or allocation visible in source.

## Validation Boundary

The native source closure limits and Cargo registration still require the main inventory/compiler checks. This review cannot establish source-count admission or compiler correctness. No mutation-specific correction is requested on the inspected extraction.
